use std::time::Duration;

use axum::{
    extract::State,
    http::{HeaderMap},
    response::Redirect, Json,
};
use sea_orm::ActiveModelBehavior;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use sqlx::{self, FromRow};
use tracing::{debug, info};

use crate::{
    auth::get_session_context,
    ServerError, ServerState,
};

struct StripeProducts {
    price: String,
    quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct StripeEvent {
    id: String,
    stripe_event_id: String,
    attributes: Value,
    event_type: String,
}

#[derive(Serialize)]
struct StripeCustomer {
    name: String,
    email: String,
}

use entity::stripe_events::Model as StripeEventModel;

pub struct MdpStripeEvent(pub StripeEventModel);

async fn create_checkout_session(
    price_id: &str,
    frontend_success_url: &str,
    frontend_cancel_url: &str,
) -> Result<Value, ServerError> {
    info!("Creating checkout session...");

    // check if stripe customer exists

    let secret_key = dotenvy::var("STRIPE_SECRETKEY")
        .map_err(|err| ServerError::ConfigError("Issue getting stripe secret key".to_string()))?;

    // Construct the request body parameters
    // let mut params = HashMap::new();

    let params: [(&str, &str); 5] = [
        ("success_url", frontend_success_url),
        ("cancel_url", frontend_cancel_url),
        ("line_items[0][price]", price_id),
        ("line_items[0][quantity]", "1"),
        ("mode", "subscription"),
    ];
    debug!("Checkout params: {params:?}");

    let encoded = serde_urlencoded::to_string(params).unwrap();
    debug!("Encoded params: {encoded:?}");

    // Construct the reqwest client
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
        .map_err(|err| {
            ServerError::ConfigError(format!("Could not create reqwest client: {err}"))
        })?;

    // Make the POST request to the Stripe API
    let response = client
        .post("https://api.stripe.com/v1/checkout/sessions")
        .basic_auth("sk_test_51Q6fmUHCinVRF92pMaieBLsJGfSiQWDiJ8HLtgCSlNLzPTyKGLdOt2KovXeQvxD8ectrQYctU1mXc8hYNVJLElkm00PV79CCV4", None::<&str>)
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .body(encoded)
        .send()
        .await
        .map_err(|err| ServerError::Stripe(format!("Failure creating checkout session: {err}")))?;

    info!("Stripe response: {:#?}", response);

    // Check if the request was successful
    if response.status().is_success() {
        info!("Checkout successful: {:#?}", response);
        let json: Value = response.json().await.unwrap();
        Ok(json)
    } else {
        // If not successful, print the error status code and message
        let status = response.status();
        let response_text = response.text().await.unwrap();
        info!("Error: {} - {}", status, response_text);
        Err(ServerError::Stripe(format!(
            "Issue creating new checkout session: {}",
            response_text
        )))
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct CheckoutSession {
    cancel_url: String,
    success_url: String,
    price_id: String,
}

#[axum::debug_handler]
pub async fn checkout_session(
    state: State<ServerState>,
    headers: HeaderMap,
    payload: Json<CheckoutSession>,
    // Form(input): Form<Value>,
) -> anyhow::Result<Redirect, ServerError> {
    info!("Recieved checkout session request");
    let ctx = get_session_context(&state, headers).await?;
    debug!("User details: {:?}", ctx);

    // let price_id = "price_1PowrGH1WJxpjVSWQ48Fz7Vn".to_string();

    // let form_input_obj = input.as_object().unwrap();
    // let success_url = form_input_obj.get("success_url").unwrap().as_str().unwrap();
    // let cancel_url = form_input_obj.get("cancel_url").unwrap().as_str().unwrap();
    // let success_url = payload.success_url;
    // let cancel_url = payload.cancel_url;

    let price_id = payload.price_id.clone();
    let checkout_session =
        match create_checkout_session(price_id.as_str(), &payload.success_url, &payload.cancel_url)
            .await
        {
            Ok(x) => x,
            Err(err) => {
                info!("Error creating checkout session: {err}");
                return Ok(Redirect::to(&state.config.frontend_url));
            }
        };

    info!("Checkout session: {checkout_session:?}");

    let redirect_url = checkout_session.get("url").unwrap().as_str().unwrap();
    Ok(Redirect::to(redirect_url))
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn list_survey(
    state: State<ServerState>,
    payload: Json<Value>,
) -> anyhow::Result<Redirect, ServerError> {
    return Ok(Redirect::to(&state.config.frontend_url));
}

// async fn log_event(request: Value) -> Result<MdpStripeEvent, ServerError> {
//     info!("->> log_event");

//     let _time = chrono::Utc::now();
//     // let stripeevent = sqlx::query_as::<_, StripeEvent>(
//     //     "insert into mdp.stripe_events (
//     //             stripe_event_id,
//     //             attributes,
//     //             event_type
//     //         ) values ($1, $2, $3) returning *",
//     // )
//     // .bind(request.get("id"))
//     // .bind(json!({"test": "test event attribute"}))
//     // .bind(request.get("type"))
//     // .fetch_one(&self.pool)
//     // .await
//     // .map_err(|err| ServerError::Database(format!("Could not create log_event: {err}")))?;

//     // Ok(stripeevent)
//     Ok(MdpStripeEvent(
//         entity::stripe_events::ActiveModel {
//             stripe_event_id: NanoId::new().to_string(),
//             from_stripe_event_id: request.get("id"),
//             attributes: request,
//             event_type: todo!(),
//             created_at: todo!(),
//             workspace_id: todo!(),
//         }
//         .try_into_model()
//         .unwrap(),
//     ))
// }

pub async fn create_customer(name: &str, email: &str) -> Result<Value, ServerError> {
    let secret_key = dotenvy::var("STRIPE_SECRETKEY").unwrap();

    let newcust = StripeCustomer {
        name: name.to_string(),
        email: email.to_string(),
    };

    let encoded = serde_urlencoded::to_string(newcust).unwrap();

    // Construct the reqwest client
    let client = reqwest::Client::new();

    let response = client
        .post("https://api.stripe.com/v1/customers")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", secret_key),
        )
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .body(encoded)
        .send()
        .await
        .unwrap();

    let json: Value = response.json().await.unwrap();
    // Object {
    //     "address": Null,
    //     "balance": Number(0),
    //     "created": Number(1707365872),
    //     "currency": Null,
    //     "default_currency": Null,
    //     "default_source": Null,
    //     "delinquent": Bool(false),
    //     "description": Null,
    //     "discount": Null,
    //     "email": String("test@jeremyarde.com"),
    //     "id": String("cus_PWRxPIJ5odGVnQ"),
    //     "invoice_prefix": String("A754E100"),
    //     "invoice_settings": Object {
    //         "custom_fields": Null,
    //         "default_payment_method": Null,
    //         "footer": Null,
    //         "rendering_options": Null,
    //     },
    //     "livemode": Bool(false),
    //     "metadata": Object {},
    //     "name": String("Jenny Ross"),
    //     "next_invoice_sequence": Number(1),
    //     "object": String("customer"),
    //     "phone": Null,
    //     "preferred_locales": Array [],
    //     "shipping": Null,
    //     "tax_exempt": String("none"),
    //     "test_clock": Null,
    // }

    Ok(json)
}

#[cfg(test)]
mod tests {}
