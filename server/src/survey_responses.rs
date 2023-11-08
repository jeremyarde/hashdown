pub mod survey_responses {
    use std::collections::HashMap;

    use axum::{
        extract::{self, Path, Query, State},
        Json,
    };
    use markdownparser::nanoid_gen;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use tracing::{debug, info};

    use crate::{
        db::database::{AnswerModel, CreateAnswersModel, Session},
        mware::ctext::Ctext,
        ServerError, ServerState,
    };

    // #[derive(Deserialize, Debug)]
    // pub struct SubmitResponseRequest {
    //     survey_id: String,
    //     responses: Value,
    // }

    #[tracing::instrument]
    #[axum::debug_handler]
    pub async fn submit_response(
        State(state): State<ServerState>,
        // extract(session): Extract<Session>,
        Json(payload): extract::Json<CreateAnswersModel>,
    ) -> Result<Json<Value>, ServerError> {
        info!("->> submit_response");
        debug!("    ->> request: {:#?}", payload);

        if payload.survey_id.is_empty() {
            return Err(ServerError::BadRequest("No survey_id found".to_string()));
        }

        let answer_result = state
            .db
            .create_answer(payload)
            .await
            .map_err(|_| ServerError::Database("Not able to insert response".to_string()))?;

        info!("completed submit_response");

        return Ok(Json(json!({"accepted": "true"})));
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

        info!("completed survey submit");
        // let test = serde_json::to_value(responses).unwrap();
        return Ok(Json(json!({ "responses": responses })));
    }
}
