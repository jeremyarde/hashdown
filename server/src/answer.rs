use std::collections::HashMap;

use axum::{
    extract::{self, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::ServerState;

// #[axum::debug_handler]
// pub async fn post_answer(
//     State(state): State<ServerState>,
//     extract::Json(payload): extract::Json<CreateAnswers>,
// ) -> impl IntoResponse {
//     (StatusCode::CREATED, ())
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAnswers {
    form_id: String,
    start_time: String,
    answers: HashMap<String, String>,
}

#[axum::debug_handler]
pub async fn post_answers(
    State(state): State<ServerState>,
    extract::Json(payload): extract::Json<CreateAnswers>,
) -> impl IntoResponse {
    // Check for survey existence
    let exists = sqlx::query("select (id) from surveys as s where s.id = $1")
        .bind(payload.form_id)
        .execute(&state.db.pool)
        .await
        .unwrap();

    println!("exists: {exists:?}");

    (StatusCode::CREATED, Json(true))
}
