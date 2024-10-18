use axum::{extract::State, Json};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde_json::{json, Value};
use tracing::info;

use crate::{ServerError, ServerState};

#[derive(PartialEq, PartialOrd)]
enum StripeEvent {
    CheckoutSessionCompleted, // give the user the subscription
    CheckoutSessionExpired,   // abandoned cart, email with checkout recovery link
    StripeCheckoutSessionAsyncPaymentSucceeded,
    CustomerSourceExpiring, // email to billing portal to update payment method
    CustomerSubscriptionDeleted, // revoke access, send "win back" email
    CustomerSubscriptionUpdated, // called when initiating cancel_at_period_end cancellation
    RadarEarlyFraudWarningCreated, // proactively cancel subscription and refund
    InvoicePaymentActionRequired,
    CustomerSubscriptionTrialWillEnd,
    InvoicePaymentFailed, // send email to update payment information
    InvoicePaymentSuccess,
    UntrackedEvent,
    SetupIntentSuccess,
    PaymentIntentSuccess,
    ChargeDisputeCreated,
}

// fn map_stripe_event_to_enum(event: &str) -> StripeEvent {
//     return match &event {
//         &"checkout.session.completed" => StripeEvent::CheckoutSessionCompleted,
//         &"stripe.checkout.session.async_payment_succeeded" => {
//             StripeEvent::StripeCheckoutSessionAsyncPaymentSucceeded
//         }
//         &"checkout.session.expired" => StripeEvent::CheckoutSessionExpired,
//         &"customer.source.expiring" => StripeEvent::CustomerSourceExpiring,
//         &"customer.subscription.deleted" => StripeEvent::CustomerSubscriptionDeleted,
//         &"customer.subscription.updated" => StripeEvent::CustomerSubscriptionUpdated,
//         &"radar.early_fraud_warning.created" => StripeEvent::RadarEarlyFraudWarningCreated,
//         &"invoice.payment_action_required" => StripeEvent::InvoicePaymentActionRequired,
//         &"customer.subscription.trial_will_end" => StripeEvent::CustomerSubscriptionTrialWillEnd,
//         &"invoice.payment_failed" => StripeEvent::InvoicePaymentFailed,
//         &"invoice.paid" => StripeEvent::InvoicePaymentSuccess,
//         &"setup_intent.succeeded" => StripeEvent::SetupIntentSuccess,
//         &"payment_intent.succeeded" => StripeEvent::PaymentIntentSuccess,
//         &"charge.dispute.created" => StripeEvent::ChargeDisputeCreated,
//         _ => StripeEvent::UntrackedEvent,
//     };
// }

use entity::users::Entity as User;

#[axum::debug_handler]
pub async fn handle_stripe_webhook(
    State(state): State<ServerState>,
    payload: Json<Value>,
) -> anyhow::Result<Json<Value>, ServerError> {
    info!("->> payments/handle_stripe_webhook");
    info!("payload: {:#?}", payload);
    let event_type = payload["type"].as_str().unwrap();

    // let event_enum = map_stripe_event_to_enum(payload.get("type").unwrap().as_str().unwrap());

    match event_type {
        // "stripe.checkout.session.async_payment_succeeded" => {}
        // "checkout.session.completed" => {
        "customer.subscription.updated" => {
            // successful payment
            info!("Stripe subscription successful, setting up subscription");
            let checkout_session_id: &str = payload["data"]["object"]["id"].as_str().unwrap();

            // get the checkout session
            // let secret_key = dotenvy::var("STRIPE_SECRETKEY").unwrap();
            let secret_key = state.config.stripe_secret_key.clone();

            // let session_id = "cs_test_a17ltYjf9B9OcUkXY7rNLyEhZpXs2Mum3u3NeLBbe2rSXBsFvSvdU7neRV";
            let params = [("expand[]", "line_items")];
            let encoded = serde_urlencoded::to_string(params).unwrap();

            let client = reqwest::Client::new();

            // Make the POST request to the Stripe API
            let response = client
                .post(format!(
                    "https://api.stripe.com/v1/checkout/sessions/{}",
                    checkout_session_id
                ))
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
            if !response.status().is_success() {
                return Err(ServerError::Stripe(
                    "Could not complete checkout session for product purchased verification"
                        .to_string(),
                ));
            }

            // this is the stripe checkout session: https://docs.stripe.com/api/checkout/sessions/object?lang=curl
            let json_session: Value = response.json().await.unwrap();
            let customer_id = json_session["customer"].to_string();

            let customer_json = get_stripe_customer(&customer_id).await;
            let price_id = json_session["line_items"]["data"][0]["price"]["id"].to_string();

            let customer_email = customer_json["email"].to_string();
            if customer_email.is_empty() {
                info!("handle_stripe_webhook: customer email is empty");
                return Err(ServerError::Stripe("Customer was not found".to_string()));
            }
            // look for user in our database
            let user = User::find()
                .filter(entity::users::Column::Email.eq(customer_email))
                .one(&state.db.pool)
                .await
                .map_err(|err| ServerError::Database(err.to_string()))
                .unwrap();

            if user.is_none() {
                info!("handle_stripe_webhook: user not found in database: {customer_json} - this should not be possible...");
            }

            let mut user = user.unwrap().into_active_model();
            // user.stripe_subscription_id = Set(Some()); // TODO: this might need to be set
            user.stripe_subscription_price_id = Set(Some(price_id));
            Ok(Json(json!({"success": true})))
        }
        // "checkout.session.expired" => {}
        // "customer.source.expiring" => {}
        "customer.subscription.deleted" => {
            let stripe_customer = payload["data"]["object"]["customer"].as_str().unwrap();
            let mut user = User::find()
                .filter(entity::users::Column::StripeCustomerId.eq(stripe_customer))
                .one(&state.db.pool)
                .await
                .map_err(|err| ServerError::Database(err.to_string()))
                .unwrap()
                .unwrap()
                .into_active_model();

            user.stripe_subscription_id = Set(None);
            user.stripe_subscription_modified_at = Set(Some(Utc::now().fixed_offset()));
            let res = user
                .save(&state.db.pool)
                .await
                .map_err(|err| ServerError::Database(err.to_string()));

            Ok(Json(json!("NOT IMPLEMENTED")))
        }
        // "customer.subscription.updated" => {}
        // "customer.subscription.created" => {}
        // "radar.early_fraud_warning.created" => {}
        // "invoice.payment_action_required" => {}
        // "customer.subscription.trial_will_end" => {}
        // "invoice.payment_failed" => {}
        // "invoice.paid" => {}
        // "setup_intent.succeeded" => {}
        // "payment_intent.succeeded" => {}
        // "charge.dispute.created" => {}
        _ => Err(ServerError::Stripe("Unhandled event".to_string())),
    }

    // match event_enum {
    //     StripeEvent::CheckoutSessionCompleted | StripeEvent::InvoicePaymentSuccess => {
    //         handle_subscription_success(state, &payload).await;
    //     }
    //     StripeEvent::CheckoutSessionExpired => todo!(),
    //     StripeEvent::StripeCheckoutSessionAsyncPaymentSucceeded => todo!(),
    //     StripeEvent::CustomerSourceExpiring => todo!(),
    //     StripeEvent::CustomerSubscriptionDeleted => todo!(),
    //     StripeEvent::CustomerSubscriptionUpdated => todo!(),
    //     StripeEvent::RadarEarlyFraudWarningCreated => todo!(),
    //     StripeEvent::InvoicePaymentActionRequired => todo!(),
    //     StripeEvent::CustomerSubscriptionTrialWillEnd => todo!(),
    //     StripeEvent::InvoicePaymentFailed => todo!(),
    //     StripeEvent::UntrackedEvent => todo!(),
    //     StripeEvent::SetupIntentSuccess => todo!(),
    //     StripeEvent::PaymentIntentSuccess => todo!(),
    //     StripeEvent::ChargeDisputeCreated => todo!(),
    //     _ => {
    //         info!("Untracked event");
    //     }
    // }
}

async fn handle_subscription_success(state: ServerState, payload: &Value) {
    let stripe_obj = payload.as_object().unwrap();
    let stripe_id = stripe_obj.get("id").unwrap().as_str().unwrap();
    info!("handling stripe subscription for stripe id: {stripe_id}");
    // let mut user = User::find_by_id((stripe_id))
    //     .filter(entity::users::Column::StripeId.eq(stripe_id))
    //     .one(state.db.sea_pool)
    //     .await
    //     .unwrap();

    // let active_user = user.unwrap().into_active_model();

    // active_user.role = Set("basic");
    // active_user.update(state.db.sea_pool).await.unwrap();
}

// checkout.session.completed
// stripe.checkout.session.async_payment_succeeded
// checkout.session.expired
// customer.source.expiring
// customer.subscription.deleted
// customer.subscription.updated
// radar.early_fraud_warning.created
// invoice.payment_action_required
// customer.subscription.trial_will_end
// invoice.payment_failed

async fn get_stripe_customer(customer_id: &str) -> Value {
    let secret_key = dotenvy::var("STRIPE_SECRETKEY").unwrap();
    let customer_id = "cus_QgJkE2C3r5oQdc";

    // Construct the reqwest client
    let client = reqwest::Client::new();

    let response = client
        .post(format!(
            "https://api.stripe.com/v1/customers/{}",
            customer_id
        ))
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", secret_key),
        )
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        // .body(encoded)
        .send()
        .await
        .unwrap();

    let json: Value = response.json().await.unwrap();
    json
}
