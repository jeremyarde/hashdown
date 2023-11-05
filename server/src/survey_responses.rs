pub mod survey_responses {
    use std::collections::HashMap;

    use axum::{
        extract::{self, Path, State},
        Json,
    };
    use markdownparser::nanoid_gen;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use tracing::{debug, info};

    use crate::{
        db::database::{CreateAnswersModel, Session},
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

    #[tracing::instrument]
    #[axum::debug_handler]
    pub async fn list_response(
        State(state): State<ServerState>,
        Path(survey_id): Path<String>,
        // ctx: Option<Ctext>,
        Json(payload): extract::Json<Value>, // for urlencoded
    ) -> Result<Json<Value>, ServerError> {
        info!("->> submit_survey");
        debug!("    ->> survey: {:#?}", payload);

        // json version
        let _survey = match state
            .db
            .get_survey(&survey_id)
            .await
            .expect("Could not get survey from db")
        {
            Some(x) => x,
            None => {
                return Err(ServerError::BadRequest(
                    "Resource does not exist".to_string(),
                ))
            }
        };
        // info!("Found survey_id in database");
        // let answer_id = nanoid_gen(12);
        // let response = CreateAnswersResponse {
        //     answer_id: answer_id.clone(),
        // };
        let create_answer_model = CreateAnswersModel {
            survey_id: survey_id.clone(),
            responses: json!({"accepted": "true"}),
        };

        let _answer_result = state
            .db
            .create_answer(create_answer_model)
            .await
            .expect("Should create answer in database");

        info!("completed survey submit");

        return Ok(Json(json!({ "survey_id": survey_id })));
    }
}
