
use axum::{extract::State, Json};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde_json::Value;
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

// what should I do in response to a stripe event
enum StripeAction {}

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
pub async fn echo(State(state): State<ServerState>, payload: Json<Value>) {
    info!("->> payments/echo");
    info!("payload: {:#?}", payload);
    let event_type = payload["type"].as_str().unwrap();

    // let event_enum = map_stripe_event_to_enum(payload.get("type").unwrap().as_str().unwrap());

    match event_type {
        "stripe.checkout.session.async_payment_succeeded" => {}
        "checkout.session.completed" => {}
        "checkout.session.expired" => {}
        "customer.source.expiring" => {}
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
        }
        "customer.subscription.updated" => {}
        "customer.subscription.created" => {}
        "radar.early_fraud_warning.created" => {}
        "invoice.payment_action_required" => {}
        "customer.subscription.trial_will_end" => {}
        "invoice.payment_failed" => {}
        "invoice.paid" => {}
        "setup_intent.succeeded" => {}
        "payment_intent.succeeded" => {}
        "charge.dispute.created" => {}
        _ => {}
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
    //     .one(&state.db.sea_pool)
    //     .await
    //     .unwrap();

    // let active_user = user.unwrap().into_active_model();

    // active_user.role = Set("basic");
    // active_user.update(&state.db.sea_pool).await.unwrap();
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
