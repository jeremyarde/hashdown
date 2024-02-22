use argon2::password_hash::rand_core::block;
use axum::{
    extract::{self, Query, State},
    Extension, Json,
};

use entity::responses::Model;
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::{debug, info};

use crate::{
    db::database::{AnswerModel, MdpSurvey},
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

struct Answer {
    id: String,
    value: String,
    question_text: String,
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
    let responses: Vec<Model> = state
        .db
        .list_responses(&response_query.survey_id, &ctx.session.0.workspace_id)
        .await
        .expect("Could not get responses from db");

    let survey = state
        .db
        .get_survey(&response_query.survey_id)
        .await
        .expect("Could not find survey");

    let block_ids = survey
        .inner()
        .blocks
        .as_array()
        .map(|blocks| println!("jere/ {:?}", blocks));

    println!("jere/ after: {:?}", block_ids);

    info!("completed survey submit");
    Ok(Json(json!({ "responses": responses, "survey": survey })))
}

// #[derive(Debug)]
// struct BlockIdName(String, String);

fn get_block_details(survey: MdpSurvey) -> () {
    let block_ids = survey.inner().blocks.as_array().map(|blocks| {
        // println!("jere/ {:#?}", blocks);
        blocks.into_iter().map(|block| block.as_object())
    });
    println!("jere/ {:#?}", block_ids);
    // return block_ids;
    return ();
}

fn combine_survey_with_response(survey: MdpSurvey, response: Value) -> Value {
    //     let block_ids: Vec<BlockIdName> = survey.blocks.as_array().map(|blocks| {
    //         println!("jere/ {:#?}", blocks);
    //         blocks.iter().map(|block| {
    //             BlockIdName(
    //                 block.get("id"),
    //                 block.get("properties").unwrap().get("question"),
    //             )
    //         })
    //     });
    //     println!("jere/ blocks: {:#?}", block_ids);
    return json!({"test": "another"});
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use entity::surveys;
    use sea_orm::{Set, TryIntoModel};
    use serde_json::json;

    // use crate::db::surveys::SurveyModel;

    use super::{combine_survey_with_response, get_block_details};

    #[test]
    fn test_combine() {
        let response = json!({
          "4wgpbx5nqiav": "test",
          "93241ezrlet1": "test"
        });

        let answers = json!([
          {
            "id": "4cxmez99swdf",
            "index": 0,
            "block_type": "Title",
            "properties": {
              "type": "Title",
              "title": "Get emailed when hashdown is available"
            }
          },
          {
            "id": "4wgpbx5nqiav",
            "index": 0,
            "block_type": "TextInput",
            "properties": {
              "type": "TextInput",
              "default": "",
              "question": "Email"
            }
          },
          {
            "id": "93241ezrlet1",
            "index": 0,
            "block_type": "Textarea",
            "properties": {
              "type": "Textarea",
              "default": "",
              "question": "What do you want to use Hashdown for?"
            }
          },
          {
            "id": "svjimprwun33",
            "index": 0,
            "block_type": "Submit",
            "properties": {
              "type": "Submit",
              "default": "",
              "question": "Put me on waitlist"
            }
          },
          {
            "id": "3gvtzvmsz1ip",
            "index": 0,
            "block_type": "Empty",
            "properties": {
              "type": "Nothing"
            }
          }
        ]);

        let survey = surveys::ActiveModel {
            // id: Set(0),
            name: Set(Some(String::new())),
            survey_id: Set(String::new()),
            user_id: Set(String::new()),
            created_at: Set(Utc::now().into()),
            modified_at: Set(Utc::now().into()),
            plaintext: Set(String::new()),
            version: Set(Some(String::new())),
            parse_version: Set(Some(String::new())),
            blocks: Set(answers),
            workspace_id: Set(String::new()),
        };

        // combine_survey_with_response(survey, response);
        get_block_details(crate::db::database::MdpSurvey(
            survey.try_into_model().unwrap(),
        ));
    }
}
