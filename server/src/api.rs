use axum::{extract::State, http::HeaderMap, Json};

use crate::{auth::get_session_header, survey_responses::SubmitResponseRequest, ServerState};

use axum::extract::{self};

use serde_json::{json, Value};
use tracing::{debug, info};

use crate::ServerError;

// #[tracing::instrument]
#[axum::debug_handler]
pub async fn list_survey(
    State(state): State<ServerState>,
    headers: HeaderMap,
) -> anyhow::Result<Json<Value>, ServerError> {
    info!("->> list_survey");

    // let ctx = get_session_context(&state, headers)
    //     .await
    //     .map_err(|err| ServerError::AuthFailNoSession)?;
    let sessionid = get_session_header(&headers);
    let ctx = state.db.get_session(sessionid.unwrap()).await?;

    info!("Getting surveys for user={}", ctx.0.user_id);
    let res = &state
        .db
        .list_survey(&ctx.0.user_id)
        .await
        .map_err(|err| ServerError::Database(err.to_string()))
        .unwrap();

    // let resp = ListSurveyResponse { surveys: *res };
    let json = json!({"surveys": res});

    Ok(Json(json))
}

// #[tracing::instrument]
#[axum::debug_handler]
pub async fn submit_response(
    State(state): State<ServerState>,
    // extract(session): Extract<Session>,
    Json(payload): extract::Json<SubmitResponseRequest>,
) -> anyhow::Result<Json<Value>, ServerError> {
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
