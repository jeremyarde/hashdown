use hyper::Server;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{debug, info};

use sqlx::{self, FromRow};

use chrono::{self, Utc};

use crate::{Database, ServerError};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct StripeEvent {
    id: String,
    stripe_event_id: String,
    attributes: Value,
    event_type: String,
}

pub trait StripeCrud {
    async fn log_event(&self, request: Value) -> Result<StripeEvent, ServerError>;
    // async fn create_user(&self, request: CreateUserRequest) -> Result<UserModel, ServerError>;
    // async fn get_user_by_email(&self, email: String) -> Result<UserModel, ServerError>;
    // async fn get_user_by_confirmation_code(
    //     &self,
    //     confirmation_token: String,
    // ) -> Result<UserModel, ServerError>;
    // async fn verify_user(&self, new_user: UserModel) -> Result<UserModel, ServerError>;
}

impl StripeCrud for Database {
    async fn log_event(&self, request: Value) -> Result<StripeEvent, ServerError> {
        println!("->> log_event");

        let _time = chrono::Utc::now();
        let stripeevent = sqlx::query_as::<_, StripeEvent>(
            "insert into mdp.stripe_events (
                    stripe_event_id, 
                    attributes, 
                    event_type
                ) values ($1, $2, $3) returning *",
        )
        .bind(request.get("id"))
        .bind(json!({"test": "test event attribute"}))
        .bind(request.get("type"))
        .fetch_one(&self.pool)
        .await
        .map_err(|err| ServerError::Database(format!("Could not create log_event: {err}")))?;

        Ok(stripeevent)
    }
}

#[cfg(test)]
mod tests {}
