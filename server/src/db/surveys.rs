use chrono::{DateTime, Utc};
use markdownparser::{nanoid_gen, ParsedSurvey};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::FromRow;

use crate::{db::database::SurveyCrud, routes::ListSurveyResponse, server::Metadata};

use super::database::MdpSession;

use axum::{
    extract::{self, Path, State},
    http::HeaderMap,
    Extension, Json,
};
use std::time::Duration;
// use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};

use tower_http::cors::CorsLayer;
use tracing::{debug, info};

use crate::{
    mware::ctext::SessionContext, survey_responses::SubmitResponseRequest, ServerError, ServerState,
};

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct SurveyModel {
    pub id: i32,
    pub name: Option<String>,
    pub survey_id: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub plaintext: String,
    // pub questions: Option<Vec<Question>>,
    pub version: Option<String>,
    pub parse_version: Option<String>,
    pub blocks: Value,
    pub workspace_id: String,
    // pub parsed_json: Option<Value>,
}
impl SurveyModel {
    pub(crate) fn new(payload: CreateSurveyRequest, session: &MdpSession) -> SurveyModel {
        // let parsed_survey =
        //     (payload.plaintext.clone()).expect("Could not parse the survey");
        // let survey = markdown_to_form_wasm_v2(payload.plaintext);
        let survey = ParsedSurvey::from(payload.plaintext.clone()).unwrap();
        let metadata = Metadata::new();
        SurveyModel {
            id: 0,
            survey_id: nanoid_gen(12),
            plaintext: payload.plaintext.clone(),
            user_id: session.0.user_id.to_owned(),
            created_at: metadata.created_at,
            modified_at: metadata.modified_at,
            // version: Some(survey),
            parse_version: Some(survey.parse_version.clone()),
            name: Some("name - todo".to_string()),
            version: Some("version - todo".to_string()),
            blocks: json!(&survey.blocks),
            workspace_id: session.0.workspace_id.clone(),
        }
    }
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn create_survey(
    headers: HeaderMap,
    State(state): State<ServerState>,
    Extension(ctx): Extension<MdpSession>,
    extract::Json(payload): extract::Json<CreateSurveyRequest>,
) -> anyhow::Result<Json<Value>, ServerError> {
    info!("->> create_survey");
    info!("Creating new survey for user={:?}", ctx.0.user_id);

    let survey = SurveyModel::new(payload, &ctx);

    let insert_result: SurveyModel = state
        .db
        .create_survey(survey, &ctx.0.workspace_id)
        .await
        .map_err(|x| ServerError::Database(format!("Could not create new survey: {x}").to_string()))
        .unwrap();

    info!("     ->> Inserted survey");

    return Ok(Json(json!({ "survey": insert_result })));
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn submit_survey(
    State(state): State<ServerState>,
    Path(survey_id): Path<String>,
    // Extension(ctx): Extension<Option<Ctext>>,
    Json(payload): extract::Json<SubmitResponseRequest>, // for urlencoded
) -> Result<Json<Value>, ServerError> {
    info!("->> submit_survey");
    debug!("    ->> survey: {:#?}", payload);

    state
        .db
        .create_answer(payload)
        .await
        .expect("Should create answer in database");

    info!("completed survey submit");

    return Ok(Json(json!({ "survey_id": survey_id })));
}

#[derive(Deserialize, Debug)]
pub struct GetSurveyQuery {
    pub format: SurveyFormat,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SurveyFormat {
    Html,
    Json,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct CreateSurveyRequest {
    pub plaintext: String,
    pub organization: Option<String>,
}

// #[tracing::instrument]
#[axum::debug_handler]
pub async fn get_survey(
    State(_state): State<ServerState>,
    // Extension(ctx): Extension<Ctext>,
    // authorization: TypedHeader<Authorization<Bearer>>,
    Path(survey_id): Path<String>,
) -> anyhow::Result<Json<Value>, ServerError> {
    let db_response = match _state.db.get_survey(&survey_id).await {
        Ok(x) => x,
        Err(_err) => return Err(ServerError::Database("Could not get survey".to_string())),
    };

    Ok(Json(json!(db_response)))
}

// #[tracing::instrument]
// #[axum::debug_handler]
// pub async fn list_survey(
//     state: State<ServerState>,
//     Extension(ctx): Extension<SessionContext>,
//     // headers: HeaderMap,
// ) -> anyhow::Result<Json<Value>, ServerError> {
//     info!("->> list_survey");

//     println!("Getting surveys for user={}", ctx.user_id);
//     let pool = &state.db.pool;

//     let res = sqlx::query_as::<_, SurveyModel>(
//         "select * from mdp.surveys where mdp.surveys.user_id = $1 and mdp.surveys.workspace_id = $2",
//     )
//     .bind(ctx.user_id.clone())
//     .bind(ctx.session.workspace_id.clone())
//     .fetch_all(pool)
//     .await
//     .map_err(|err| ServerError::Database(err.to_string()))
//     .unwrap();

//     let resp = ListSurveyResponse { surveys: res };

//     Ok(Json(json!(resp)))
// }
