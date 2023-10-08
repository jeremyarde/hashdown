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

    use crate::{db::database::CreateAnswersModel, mware::ctext::Ctext, ServerError, ServerState};

    #[derive(Deserialize, Debug)]
    pub struct SubmitResponseRequest {
        survey_id: String,
        responses: HashMap<String, Vec<String>>,
    }

    #[derive(Deserialize, Serialize)]
    pub struct SubmitResponseResponse {
        survey_id: String,
        responses: HashMap<String, Vec<String>>,
    }

    #[tracing::instrument]
    #[axum::debug_handler]
    pub async fn submit_response(
        State(state): State<ServerState>,
        ctx: Option<Ctext>,
        Json(payload): extract::Json<SubmitResponseRequest>, // for urlencoded
    ) -> Result<Json<SubmitResponseResponse>, ServerError> {
        info!("->> submit_response");
        debug!("    ->> request: {:#?}", payload);

        // check survey exists
        let survey = match state
            .db
            .get_survey(&payload.survey_id)
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

        // let answer_result = state
        //     .db
        //     .create_answer(create_answer_model)
        //     .await
        //     .expect("Should create answer in database");

        info!("completed submit_response");

        return Ok(Json(SubmitResponseResponse {
            survey_id: payload.survey_id,
            responses: HashMap::new(),
        }));
    }

    #[tracing::instrument]
    #[axum::debug_handler]
    pub async fn list_response(
        State(state): State<ServerState>,
        Path(survey_id): Path<String>,
        ctx: Option<Ctext>,
        Json(payload): extract::Json<Value>, // for urlencoded
    ) -> Result<Json<Value>, ServerError> {
        info!("->> submit_survey");
        debug!("    ->> survey: {:#?}", payload);

        // json version
        let survey = match state
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
            id: None,
            answer_id: nanoid_gen(12),
            survey_id: survey_id.clone(),
            answers: json!(payload),
            submitted_at: chrono::Utc::now().to_string(),
            // external_id: "".to_string(),
            // survey_version: "".to_string(),
            // start_time: chrono::Local::now().to_string(),
            // end_time: "".to_string(),
            // created_at: "".to_string(),
        };

        let answer_result = state
            .db
            .create_answer(create_answer_model)
            .await
            .expect("Should create answer in database");

        info!("completed survey submit");

        return Ok(Json(json!({ "survey_id": survey_id })));
    }
}
