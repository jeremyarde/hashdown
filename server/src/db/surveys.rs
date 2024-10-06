use entity::surveys::Model as SurveyModel;
use markdownparser::{nanoid_gen, ParsedSurvey};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::server::Metadata;

use super::database::{MdpSession, MdpSurvey};

use axum::{
    extract::{self, Path, State},
    http::HeaderMap,
    Extension, Json,
};
// use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};

use tracing::{debug, info};

use crate::{
    mware::ctext::SessionContext, survey_responses::SubmitResponseRequest, ServerError, ServerState,
};

// #[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
// pub struct SurveyModel {
//     pub id: i32,
//     pub name: Option<String>,
//     pub survey_id: String,
//     pub user_id: String,
//     pub created_at: DateTime<Utc>,
//     pub modified_at: DateTime<Utc>,
//     pub plaintext: String,
//     // pub questions: Option<Vec<Question>>,
//     pub version: Option<String>,
//     pub parse_version: Option<String>,
//     pub blocks: Value,
//     pub workspace_id: String,
//     // pub parsed_json: Option<Value>,
// }
impl MdpSurvey {
    pub(crate) fn new(payload: CreateSurveyRequest, session: &MdpSession) -> MdpSurvey {
        // let parsed_survey =
        //     (payload.plaintext.clone()).expect("Could not parse the survey");
        // let survey = markdown_to_form_wasm_v2(payload.plaintext);
        let survey = ParsedSurvey::from(payload.plaintext.clone()).unwrap();
        let metadata = Metadata::new();

        MdpSurvey(SurveyModel {
            // id: 0,
            survey_id: nanoid_gen(12),
            plaintext: payload.plaintext.clone(),
            user_id: session.0.user_id.to_owned(),
            created_at: metadata.created_at.into(),
            modified_at: metadata.modified_at.into(),
            // version: Some(survey),
            parse_version: Some(survey.parse_version.clone()),
            name: Some("name - todo".to_string()),
            version: Some("version - todo".to_string()),
            blocks: json!(&survey.blocks),
            workspace_id: session.0.workspace_id.clone(),
        })
    }
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn create_survey(
    headers: HeaderMap,
    State(state): State<ServerState>,
    extract::Json(payload): extract::Json<CreateSurveyRequest>,
) -> anyhow::Result<Json<Value>, ServerError> {
    info!("->> create_survey");
    info!("Creating new survey for user={:?}", ctx.user_id);

    let survey = MdpSurvey::new(payload, &ctx.session);

    let insert_result = state
        .db
        .create_survey(survey, &ctx.session.0.workspace_id)
        .await
        .map_err(|x| {
            ServerError::Database(format!("Could not create new survey: {x}").to_string())
        })?;

    info!("     ->> Inserted survey");

    return Ok(Json(json!({ "survey": insert_result })));
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn submit_survey(
    State(state): State<ServerState>,
    Path(survey_id): Path<String>,
    Json(payload): extract::Json<SubmitResponseRequest>,
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
