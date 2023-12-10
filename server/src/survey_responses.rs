use axum::{
    extract::{self, Query, State},
    Json,
};

use serde::Deserialize;
use serde_json::{json, Value};
use tracing::{debug, info};

use crate::{db::database::AnswerModel, ServerError, ServerState};

#[derive(Deserialize, Debug)]
pub struct SubmitResponseRequest {
    pub survey_id: String,
    pub answers: Value,
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn submit_response(
    State(state): State<ServerState>,
    // extract(session): Extract<Session>,
    Json(payload): extract::Json<SubmitResponseRequest>,
) -> Result<Json<Value>, ServerError> {
    info!("->> submit_response");
    debug!("    ->> request: {:#?}", payload);

    if payload.survey_id.is_empty() {
        return Err(ServerError::BadRequest("No survey_id found".to_string()));
    }

    state
        .db
        .create_answer(payload)
        .await
        .map_err(|_| ServerError::Database("Not able to insert response".to_string()))?;

    info!("completed submit_response");

    Ok(Json(json!({"accepted": "true"})))
}

#[derive(Deserialize, Debug)]
pub struct ResponseQuery {
    survey_id: String,
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn list_response(
    State(state): State<ServerState>,
    // Path(survey_id): Path<String>,
    response_query: Query<ResponseQuery>,
    // ctx: Option<Ctext>,
    // Json(payload): extract::Json<Value>, // for urlencoded
) -> Result<Json<Value>, ServerError> {
    info!("->> submit_survey");
    debug!("    ->> survey: {:#?}", response_query);

    // json version
    let responses: Vec<AnswerModel> = state
        .db
        .list_responses(&response_query.survey_id)
        .await
        .expect("Could not get responses from db");

    let _survey = state
        .db
        .get_survey(&response_query.survey_id)
        .await
        .expect("Could not find survey");

    info!("completed survey submit");
    // let test = serde_json::to_value(responses).unwrap();
    Ok(Json(json!({ "responses": responses })))
}

// struct AnswerData {
//     answer_id: String,
//     survey_id: String,
//     answer_text: HashMap<String, String> //
// }
