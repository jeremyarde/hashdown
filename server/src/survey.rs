use std::collections::HashMap;

use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use markdownparser::nanoid_gen;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::{internal_error, CreateSurvey, ServerState};

#[derive(Debug, Serialize, Clone, FromRow, Deserialize)]
pub struct Survey {
    id: String,
    // nanoid: String,
    plaintext: String,
    // user_id: String,
    // created_at: String,
    // modified_at: String,
    // version: String,
}

impl Survey {
    pub fn from(text: String) -> Survey {
        return Survey {
            id: nanoid_gen(10),
            plaintext: text,
        };
    }
}

struct Form {
    id: String,
    views: i32,
    starts: i32,
    submissions: i32,
    completions: i32,
    created_on: String,
    modified_on: String,
}

struct CreateForm {
    text: String,
}

struct Question {
    qtype: QuestionType,
    text: String,
}

enum QuestionType {
    Multi,
    Radio,
    Text,
}

#[derive(Debug, Deserialize, Serialize)]
struct Answers {
    id: String,
    used_id: String,
    survey_id: String,
    submitted_on: String,
    answers: HashMap<String, String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAnswers {
    form_id: String,
    start_time: String,
    form: HashMap<String, String>,
}

#[axum::debug_handler]
pub async fn create_answers(
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

#[axum::debug_handler]
pub async fn create_survey(
    State(state): State<ServerState>,
    extract::Json(payload): extract::Json<CreateSurvey>,
) -> impl IntoResponse {
    let survey = Survey::from(payload.plaintext.clone());

    let res = sqlx::query("insert into surveys (id, plaintext) values ($1, $2)")
        .bind(payload.id.clone())
        .bind(payload.plaintext)
        .execute(&state.db.pool)
        .await
        .unwrap();

    (StatusCode::CREATED, Json(survey))
}

#[axum::debug_handler]
pub async fn list_survey(State(state): State<ServerState>) -> impl IntoResponse {
    let pool = state.db.pool;

    let count: i64 = sqlx::query_scalar("select count(id) from surveys")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
        .unwrap();
    println!("Survey count: {count:#?}");

    let res = sqlx::query_as::<_, Survey>("select * from surveys")
        .fetch_all(&pool)
        .await
        .unwrap();

    // json!({ "surveys": res });

    println!("Survey: {res:#?}");

    (StatusCode::OK, Json(json!({ "surveys": res })))
}

#[axum::debug_handler]
pub async fn get_survey(
    State(state): State<ServerState>,
    Path(params): Path<HashMap<String, String>>,
) -> impl IntoResponse {
    let pool = state.db.pool;

    let count: i64 = sqlx::query_scalar("select count(id) from surveys")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
        .unwrap();
    println!("Survey count: {count:#?}");

    let res = sqlx::query_as::<_, Survey>("select * from surveys as s where s.id = $1")
        .bind(params.get("id"))
        .fetch_one(&pool)
        .await
        .unwrap();

    println!("Survey: {res:#?}");

    (StatusCode::OK, Json(res))
}
