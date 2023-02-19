use std::collections::HashMap;

use axum::{
    extract::{self, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use markdownparser::nanoid_gen;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::ServerState;

// #[axum::debug_handler]
// pub async fn post_answer(
//     State(state): State<ServerState>,
//     extract::Json(payload): extract::Json<CreateAnswers>,
// ) -> impl IntoResponse {
//     (StatusCode::CREATED, ())
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAnswersRequest {
    // id: String,
    pub survey_id: String,
    pub survey_version: String,
    pub start_time: String,
    pub answers: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAnswersResponse {
    id: String,
    survey_id: String,
    survey_version: String,
    start_time: String,
    answers: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct CreateAnswersModel {
    id: String,
    survey_id: String,
    survey_version: String,
    start_time: String,
    end_time: String,
    answers: String,
}

impl CreateAnswersModel {
    fn from(create_answer: CreateAnswersRequest) -> CreateAnswersModel {
        CreateAnswersModel {
            id: nanoid_gen(12),
            survey_id: create_answer.survey_id,
            survey_version: create_answer.survey_version,
            start_time: create_answer.start_time,
            end_time: "now".to_string(),
            answers: json!(create_answer.answers).to_string(),
        }
    }
}

#[axum::debug_handler]
pub async fn post_answers(
    State(state): State<ServerState>,
    extract::Json(payload): extract::Json<CreateAnswersRequest>,
) -> impl IntoResponse {
    // Check for survey existence
    let exists = sqlx::query("select (id) from surveys as s where s.id = $1")
        .bind(payload.survey_id.clone())
        .execute(&state.db.pool)
        .await
        .unwrap();
    println!("exists: {exists:?}");

    let create_answer_model = CreateAnswersModel::from(payload);

    // Create a survey
    let res = sqlx::query_as::<_, CreateAnswersModel>(
        "insert into answers (id, survey_id, survey_version, start_time, end_time, answers) 
            values 
            ($1, $2, $3, $4, $5, $6) 
            returning *",
    )
    .bind(create_answer_model.id)
    .bind(create_answer_model.survey_id)
    .bind(create_answer_model.survey_version)
    .bind(create_answer_model.start_time)
    .bind(create_answer_model.end_time)
    .bind(json!(create_answer_model.answers).as_str())
    .fetch_one(&state.db.pool)
    .await
    .unwrap();

    let answers: HashMap<String, String> = serde_json::from_str(&res.answers).unwrap();
    let response = CreateAnswersResponse {
        id: res.id,
        survey_id: res.survey_id,
        survey_version: res.survey_version,
        start_time: res.start_time,
        answers: answers,
    };

    (StatusCode::CREATED, Json(response))
}
