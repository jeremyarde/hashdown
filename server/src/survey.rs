use std::collections::HashMap;

use askama::Template;
use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use markdownparser::{markdown_to_form, parse_markdown_v3, Survey};
use serde::{Deserialize, Serialize};

// use ts_rs::TS;

use crate::answer;
use crate::survey;
use crate::{internal_error, ServerState};

// #[derive(Debug, Serialize, Clone, FromRow, Deserialize)]
// pub struct Survey {
//     pub id: String,
//     // nanoid: String,
//     pub plaintext: String,
//     // user_id: String,
//     // created_at: String,
//     // modified_at: String,
//     // version: String,
// }

// impl Survey {
//     pub fn from(text: String) -> Survey {
//         return Survey {
//             id: nanoid_gen(10),
//             plaintext: text,
//         };
//     }
// }

impl SurveyModel {
    fn to_survey(survey: &SurveyModel) -> Survey {
        let survey = survey.clone();
        let questions = markdown_to_form(survey.plaintext.clone()).questions;
        return Survey {
            id: survey.id,
            plaintext: survey.plaintext,
            user_id: survey.user_id,
            created_at: survey.created_at,
            modified_at: survey.modified_at,
            questions: questions,
            version: survey.version,
            parse_version: survey.parse_version,
        };
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SurveyModel {
    pub id: String,
    pub plaintext: String,
    pub user_id: String,
    pub created_at: String,
    pub modified_at: String,
    // pub questions: Option<Vec<Question>>,
    pub version: String,
    pub parse_version: String,
}

struct Form {
    id: String,
    views: i32,
    starts: i32,
    submissions: i32,
    completions: i32,
    created_on: String,
    modified_on: String,
}

struct CreateForm {
    text: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Answers {
    id: String,
    used_id: String,
    survey_id: String,
    submitted_on: String,
    answers: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnswerRequest {
    pub form_id: String,
    pub start_time: String,
    pub answers: HashMap<String, String>,
}

struct Answer {
    form_id: String,
    value: String,
}

#[derive(Deserialize, Serialize, sqlx::FromRow, Debug, PartialEq, Eq, TS)]
#[ts(export)]
pub struct CreateSurveyRequest {
    pub plaintext: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateSurveyResponse {
    pub survey: Survey,
    pub metadata: SurveyModel,
}

#[derive(Template)]
#[template(path = "form.html")]
struct FormTemplate {
    survey_id: String,
}

pub async fn get_form(
    State(state): State<ServerState>,
    Path(survey_id): Path<String>,
) -> FormTemplate {
    FormTemplate {
        survey_id: survey_id,
    }
}

#[derive(Template)]
#[template(path = "create_survey.html")]
struct CreateSurveyTemplate {
    // survey_value: String,
}

#[axum::debug_handler]
pub async fn create_survey_form(State(state): State<ServerState>) -> impl IntoResponse {
    CreateSurveyTemplate {}
}
