use axum::{extract::State, Json};
use hyper::Server;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, QueryFilter};
use serde_json::Value;
use tracing::{debug, info};

use crate::ServerState;

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

fn map_stripe_event_to_enum(event: &str) -> StripeEvent {
    return match &event {
        &"checkout.session.completed" => StripeEvent::CheckoutSessionCompleted,
        &"stripe.checkout.session.async_payment_succeeded" => {
            StripeEvent::StripeCheckoutSessionAsyncPaymentSucceeded
        }
        &"checkout.session.expired" => StripeEvent::CheckoutSessionExpired,
        &"customer.source.expiring" => StripeEvent::CustomerSourceExpiring,
        &"customer.subscription.deleted" => StripeEvent::CustomerSubscriptionDeleted,
        &"customer.subscription.updated" => StripeEvent::CustomerSubscriptionUpdated,
        &"radar.early_fraud_warning.created" => StripeEvent::RadarEarlyFraudWarningCreated,
        &"invoice.payment_action_required" => StripeEvent::InvoicePaymentActionRequired,
        &"customer.subscription.trial_will_end" => StripeEvent::CustomerSubscriptionTrialWillEnd,
        &"invoice.payment_failed" => StripeEvent::InvoicePaymentFailed,
        &"invoice.paid" => StripeEvent::InvoicePaymentSuccess,
        &"setup_intent.succeeded" => StripeEvent::SetupIntentSuccess,
        &"payment_intent.succeeded" => StripeEvent::PaymentIntentSuccess,
        &"charge.dispute.created" => StripeEvent::ChargeDisputeCreated,
        _ => StripeEvent::UntrackedEvent,
    };
}

#[axum::debug_handler]
pub async fn echo(State(state): State<ServerState>, payload: Json<Value>) {
    info!("->> payments/echo");
    debug!("payload: {:#?}", payload);

    let event_enum = map_stripe_event_to_enum(payload.get("type").unwrap().as_str().unwrap());

    if event_enum == StripeEvent::UntrackedEvent {
        info!("Untracked event");
        return;
    }

    match event_enum {
        StripeEvent::CheckoutSessionCompleted | StripeEvent::InvoicePaymentSuccess => {
            handle_subscription_success(state, &payload).await;
        }
        StripeEvent::CheckoutSessionExpired => todo!(),
        StripeEvent::StripeCheckoutSessionAsyncPaymentSucceeded => todo!(),
        StripeEvent::CustomerSourceExpiring => todo!(),
        StripeEvent::CustomerSubscriptionDeleted => todo!(),
        StripeEvent::CustomerSubscriptionUpdated => todo!(),
        StripeEvent::RadarEarlyFraudWarningCreated => todo!(),
        StripeEvent::InvoicePaymentActionRequired => todo!(),
        StripeEvent::CustomerSubscriptionTrialWillEnd => todo!(),
        StripeEvent::InvoicePaymentFailed => todo!(),
        StripeEvent::UntrackedEvent => todo!(),
        StripeEvent::SetupIntentSuccess => todo!(),
        StripeEvent::PaymentIntentSuccess => todo!(),
        StripeEvent::ChargeDisputeCreated => todo!(),
        _ => {
            info!("Untracked event");
        }
    }
}

use entity::users::Entity as User;

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
