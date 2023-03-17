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

use ts_rs::TS;

use crate::{internal_error, ServerState};



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

#[axum::debug_handler]
pub async fn create_survey(
    State(state): State<ServerState>,
    extract::Json(payload): extract::Json<CreateSurveyRequest>,
) -> impl IntoResponse {
    let survey = parse_markdown_v3(payload.plaintext.clone());
    // let survey = Survey::from(payload.plaintext.clone());
    let response_survey = survey.clone();

    let res: SurveyModel = sqlx::query_as::<_, SurveyModel>(
        "insert into surveys (id, plaintext, user_id, created_at, modified_at, version, parse_version) 
        values 
        ($1, $2, $3, $4, $5, $6, $7)
        returning *",
    )
    .bind(response_survey.id.clone())
    .bind(payload.plaintext)
    .bind(survey.user_id)
    .bind(survey.created_at)
    .bind(survey.modified_at)
    // .bind(json!({"questions": survey.questions}))
    .bind(survey.version)
    .bind(survey.parse_version).fetch_one(&state.db.pool)
    .await
    .unwrap();

    let response = CreateSurveyResponse {
        survey: Survey::from(response_survey),
        metadata: res,
    };

    (StatusCode::CREATED, Json(response))
}

#[axum::debug_handler]
pub async fn list_survey(State(state): State<ServerState>) -> impl IntoResponse {
    let pool = state.db.pool;

    let count: i64 = sqlx::query_scalar("select count(id) from surveys")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
        .unwrap();
    println!("Survey count: {count:#?}");

    let res: Vec<SurveyModel> = sqlx::query_as::<_, SurveyModel>("select * from surveys")
        .fetch_all(&pool)
        .await
        .unwrap();

    let surveys = res.iter().map(|x| SurveyModel::to_survey(x)).collect();

    // json!({ "surveys": res });

    println!("Survey: {res:#?}");
    let listresp = ListSurveyResponse { surveys: surveys };

    // (StatusCode::OK, Json(json!({ "surveys": res })))
    (StatusCode::OK, Json(listresp))
}


#[axum::debug_handler]
pub async fn get_survey(
    State(state): State<ServerState>,
    Path(survey_id): Path<String>,
) -> impl IntoResponse {
    let pool = state.db.pool;

    let count: i64 = sqlx::query_scalar("select count(id) from surveys")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
        .unwrap();
    println!("Survey count: {count:#?}");

    let res = sqlx::query_as::<_, SurveyModel>("select * from surveys as s where s.id = $1")
        .bind(survey_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    println!("Survey: {res:#?}");
    let resp_survey = parse_markdown_v3(res.plaintext.clone());
    let response = CreateSurveyResponse {
        survey: resp_survey,
        metadata: res,
    };

    let template = FormTemplate {
        survey_id: response.survey.id,
    };

    return (StatusCode::OK, template);
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
