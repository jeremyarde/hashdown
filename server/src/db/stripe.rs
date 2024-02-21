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
}

#[cfg(test)]
mod tests {}
