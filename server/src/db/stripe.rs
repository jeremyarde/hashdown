use hyper::Server;
use sea_orm::TryIntoModel;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{debug, info};

use sqlx::{self, FromRow};

use chrono::{self, Utc};

use crate::{MdpDatabase, ServerError};

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

impl MdpDatabase {
    async fn log_event(&self, request: Value) -> Result<MdpStripeEvent, ServerError> {
        println!("->> log_event");

        let _time = chrono::Utc::now();
        // let stripeevent = sqlx::query_as::<_, StripeEvent>(
        //     "insert into mdp.stripe_events (
        //             stripe_event_id,
        //             attributes,
        //             event_type
        //         ) values ($1, $2, $3) returning *",
        // )
        // .bind(request.get("id"))
        // .bind(json!({"test": "test event attribute"}))
        // .bind(request.get("type"))
        // .fetch_one(&self.pool)
        // .await
        // .map_err(|err| ServerError::Database(format!("Could not create log_event: {err}")))?;

        // Ok(stripeevent)
        Ok(MdpStripeEvent(
            entity::stripe_events::ActiveModel {
                ..Default::default()
            }
            .try_into_model()
            .unwrap(),
        ))
    }

    pub async fn create_customer(&self, name: &str, email: &str) -> Result<Value, ServerError> {
        let secret_key = dotenvy::var("TEST_STRIPE_SECRETKEY").unwrap();

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

        return Ok(json);
    }
}

#[cfg(test)]
mod tests {}
