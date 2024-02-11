use std::ops::Add;

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, Debug, Clone, FromRow)]
pub struct Session {
    pub id: i32,
    pub user_id: String,
    pub session_id: String,
    pub active_period_expires_at: DateTime<Utc>,
    pub idle_period_expires_at: DateTime<Utc>,
    pub workspace_id: String,
}

impl Session {
    pub fn new() -> Self {
        Session {
            id: 0,
            user_id: String::from(""),
            session_id: String::from(""),
            active_period_expires_at: chrono::Utc::now().add(chrono::Days::new(1)),
            idle_period_expires_at: chrono::Utc::now().add(chrono::Days::new(1)),
            workspace_id: String::from(""),
        }
    }
}
