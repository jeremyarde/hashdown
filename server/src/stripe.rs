use axum::{
    extract::State,
    http::{response, status},
    response::Redirect,
    Extension, Form, Json,
};
use reqwest::redirect;
use sea_orm::ActiveModelBehavior;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use sqlx::{self, FromRow};
use tracing::info;
use tracing_subscriber::filter::FromEnvError;

use crate::{mware::ctext::SessionContext, ServerError, ServerState};

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
    let secret_key = dotenvy::var("STRIPE_SECRETKEY").unwrap();

    // Construct the request body parameters
    // let mut params = HashMap::new();

    let params = [
        ("success_url", frontend_success_url),
        ("cancel_url", frontend_cancel_url),
        ("line_items[0][price]", price_id),
        ("line_items[0][quantity]", "1"),
        ("mode", "subscription"),
    ];

    let encoded = serde_urlencoded::to_string(params).unwrap();

    // Construct the reqwest client
    let client = reqwest::Client::new();

    // Make the POST request to the Stripe API
    let response = client
        .post("https://api.stripe.com/v1/checkout/sessions")
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

    // Check if the request was successful
    if response.status().is_success() {
        let json: Value = response.json().await.unwrap();

        println!("Response: {:#?}", json);
        return Ok(json);
    } else {
        // If not successful, print the error status code and message
        let status = response.status();
        let response_text = response.text().await.unwrap();
        info!("Error: {} - {}", status, response_text);
        return Err(ServerError::Stripe(format!(
            "Issue creating new checkout session: {}",
            response_text
        )));
    }
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn checkout_session(
    state: State<ServerState>,
    // Extension(ctx): Extension<SessionContext>, // need to pay to login?
    // payload: Json<Value>,
    Form(input): Form<Value>,
) -> Redirect {
    let price_id = "price_1PowrGH1WJxpjVSWQ48Fz7Vn".to_string();

    let form_input_obj = input.as_object().unwrap();
    let success_url = form_input_obj.get("success_url").unwrap().as_str().unwrap();
    let cancel_url = form_input_obj.get("cancel_url").unwrap().as_str().unwrap();

    let checkout_session =
        match create_checkout_session(price_id.as_str(), success_url, cancel_url).await {
            Ok(x) => x,
            Err(err) => {
                info!("Error creating checkout session: {err}");
                return Redirect::to(&state.config.frontend_url);
            }
        };

    let redirect_url = checkout_session.get("url").unwrap().as_str().unwrap();
    return Redirect::to(redirect_url);
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn list_survey(
    state: State<ServerState>,
    Extension(ctx): Extension<SessionContext>,
    payload: Json<Value>,
) -> anyhow::Result<Redirect, ServerError> {
    return Ok(Redirect::to(&state.config.frontend_url));
}

// async fn log_event(request: Value) -> Result<MdpStripeEvent, ServerError> {
//     println!("->> log_event");

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
