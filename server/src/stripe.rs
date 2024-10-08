use std::time::Duration;

use axum::{extract::State, http::HeaderMap, response::Redirect, Json, RequestExt};
use sea_orm::ActiveModelBehavior;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use sqlx::{self, FromRow};
use tracing::{debug, info};
use tracing_subscriber::field::debug;

use crate::{auth::get_session_header, constants::SESSION_ID_KEY, ServerError, ServerState};

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
    stripe_customer_id: &str,
    price_id: &str,
    frontend_success_url: &str,
    frontend_cancel_url: &str,
) -> Result<Value, ServerError> {
    info!("Creating checkout session...");
    let secret_key = dotenvy::var("STRIPE_SECRETKEY")
        .map_err(|err| ServerError::ConfigError("Issue getting stripe secret key".to_string()))?;

    let params: [(&str, &str); 6] = [
        ("success_url", frontend_success_url),
        ("cancel_url", frontend_cancel_url),
        ("line_items[0][price]", price_id),
        ("line_items[0][quantity]", "1"),
        ("mode", "subscription"),
        ("customer", stripe_customer_id),
    ];
    let encoded = serde_urlencoded::to_string(params).expect("Not able to encode params");
    println!("Encoded params: {encoded:?}");

    let response = reqwest::Client::new()
        .post("https://api.stripe.com/v1/checkout/sessions")
        .basic_auth(secret_key, None::<&str>)
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .body(encoded)
        .send()
        .await;

    let result: Value = response.unwrap().json::<Value>().await.unwrap();
    info!("Stripe response json: {:#?}", result);

    return Ok(result);
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct CheckoutSession {
    cancel_url: String,
    success_url: String,
    price_id: String,
}

#[axum::debug_handler]
pub async fn checkout_session(
    headers: HeaderMap,
    state: State<ServerState>,
    // State(state): State<ServerState>,
    payload: Json<CheckoutSession>,
    // Form(input): Form<Value>,
) -> anyhow::Result<Redirect, ServerError> {
    info!("Recieved checkout session request");

    let session_id = get_session_header(&headers).unwrap();

    let ctx = get_session_context(&state, headers)
        .await
        .map_err(|err| ServerError::AuthFailNoSession)?;

    if ctx.user_id.is_empty() {
        info!("No session found, direct customer to create an account");
        return Ok(Redirect::to(&state.config.frontend_url));
    }
    debug!("User details: {:?}", ctx);

    let user = &state
        .db
        .get_user_by_id(ctx.user_id)
        .await
        .expect("Database failed")
        .expect("Did not find user");

    debug!("User details: {:?}", user);

    let stripeid = user
        .0
        .stripe_customer_id
        .clone()
        .expect("User has no stripe customer id");

    info!("Stripe customer id: {stripeid}");

    let price_id = "price_1Q7Na3HCinVRF92pWfMSH0wI"; // sandbox price

    let checkout_session = match create_checkout_session(
        &stripeid,
        price_id,
        &payload.success_url,
        &payload.cancel_url,
    )
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

pub async fn create_customer(name: &str, email: &str) -> Result<Value, ServerError> {
    info!("Creating customer in Stripe");
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

    Ok(json)
}

#[cfg(test)]
mod tests {
    use argon2::password_hash::SaltString;
    use rand::rngs::OsRng;
    use serde_json::{json, Value};

    use crate::{
        db::database::{CreateUserRequest, MdpDatabase},
        stripe::create_customer,
    };

    #[tokio::test]
    // #[serial]
    //     async fn test_signup() {
    async fn test_stripe_checkout() {
        let name = "test user";
        let email = "test@jeremyarde.com";

        let stripe_customer = create_customer(name.as_ref(), email.as_ref())
            .await
            .expect("Stripe customer should be created");

        let stripe_customer_id = stripe_customer.get("id").unwrap().as_str().unwrap();

        let frontend_success_url = "http://localhost:5173/v1/payment/success";
        let frontend_cancel_url = "http://localhost:5173/v1/payment/cancel";
        // let price_id = "price_1I1w6lI5j0q7u0x7x0";
        let price_id = "price_1Q7Na3HCinVRF92pWfMSH0wI"; // sandbox price

        let params: [(&str, &str); 6] = [
            ("success_url", frontend_success_url),
            ("cancel_url", frontend_cancel_url),
            ("line_items[0][price]", price_id),
            ("line_items[0][quantity]", "1"),
            ("mode", "subscription"),
            ("customer", stripe_customer_id),
            // ("currency", "cad"),
        ];

        let encoded = serde_urlencoded::to_string(params).expect("Not able to encode params");
        println!("Encoded params: {encoded:?}");

        let response = reqwest::Client::new()
            .post("https://api.stripe.com/v1/checkout/sessions")
            .basic_auth("sk_test_51Q6fmUHCinVRF92pMaieBLsJGfSiQWDiJ8HLtgCSlNLzPTyKGLdOt2KovXeQvxD8ectrQYctU1mXc8hYNVJLElkm00PV79CCV4", None::<&str>)
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(encoded)
            .send()
            .await;
        let result: Value = response.unwrap().json::<Value>().await.unwrap();

        // assert!(response.is_ok());

        println!("Stripe checkout response: {result:?}");

        assert!(result.get("url").is_some());
    }
}
