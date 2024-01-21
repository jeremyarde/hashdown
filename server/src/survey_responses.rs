use axum::{
    extract::{self, Query, State},
    Extension, Json,
};

use serde::Deserialize;
use serde_json::{json, Value};
use tracing::{debug, info};

use crate::{
    db::database::{AnswerModel, SurveyCrud},
    mware::ctext::SessionContext,
    ServerError, ServerState,
};

#[derive(Deserialize, Debug)]
pub struct SubmitResponseRequest {
    pub survey_id: String,
    pub answers: Value,
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
    Extension(ctx): Extension<SessionContext>,
    // Json(payload): extract::Json<Value>, // for urlencoded
) -> Result<Json<Value>, ServerError> {
    info!("->> submit_survey");
    debug!("    ->> survey: {:#?}", response_query);

    // json version
    let responses: Vec<AnswerModel> = state
        .db
        .list_responses(&response_query.survey_id, &ctx.session.workspace_id)
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
