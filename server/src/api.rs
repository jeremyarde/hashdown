use axum::{extract::State, Extension, Json};

use crate::{mware::ctext::SessionContext, survey_responses::SubmitResponseRequest, ServerState};

use axum::extract::{self};

use serde_json::{json, Value};
use tracing::{debug, info};

use crate::ServerError;

#[tracing::instrument]
#[axum::debug_handler]
pub async fn list_survey(
    state: State<ServerState>,
    Extension(ctx): Extension<SessionContext>,
    // headers: HeaderMap,
) -> anyhow::Result<Json<Value>, ServerError> {
    info!("->> list_survey");

    info!("Getting surveys for user={}", ctx.user_id);
    let res = &state
        .db
        .list_survey(ctx)
        .await
        .map_err(|err| ServerError::Database(err.to_string()))
        .unwrap();

    // let resp = ListSurveyResponse { surveys: *res };
    let json = json!({"surveys": res});

    Ok(Json(json))
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
        return Err(ServerError::RequestParams(
            "Missing required parameters: survey_id".to_string(),
        ));
    }

    state.db.create_answer(payload).await?;
    // .map_err(|_| ServerError::Database("Not able to insert response".to_string()))?;

    info!("completed submit_response");

    Ok(Json(json!({"accepted": "true"})))
}
