use std::str::FromStr;

// use askama::Template;
use axum::{
    extract::{DefaultBodyLimit, Multipart, Query},
    http::{self, HeaderMap, HeaderName, HeaderValue, Method, Response},
    response::IntoResponse,
    routing::{get, post},
    RequestPartsExt, Router,
};
use db::{
    database::Database,
    models::{CreateAnswersModel, CreateAnswersResponse, CreateSurveyRequest, SurveyModelBuilder},
};
use markdownparser::{nanoid_gen, parse_markdown_v3, MetadataBuilder, Survey, SurveyBuilder};
use oauth2::basic::BasicClient;
// use reqwest::Response;
use serde_json::json;
// use reqwest::header;
use sqlx::FromRow;
use tower::ServiceExt;
// use ui::mainapp::{self, dioxusapp};
// use ormlite::FromRow;
// use ormlite::{model::ModelBuilder, Model};

use tokio::{fs, task::JoinHandle};
use tracing::log::info;
// use uuid::Uuid;
// use sqlx::{Sqlite, SqlitePool};
// use tower_http::http::cors::CorsLayer;
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    services::ServeDir,
    trace::TraceLayer,
};
// use yewui::runapp;

// use crate::answer::post_answer;
use crate::{internal_error, ServerState};

// use tower_http::trace::TraceLayer;
// use tower::http
pub struct ServerApplication {
    pub base_url: SocketAddr,
    pub server: JoinHandle<()>,
    pub oauth_client: Option<BasicClient>,
}

async fn hello() -> impl IntoResponse {
    "hello from server!"
}

// async fn uiapp(State(_state): State<ServerState>) -> impl IntoResponse {
//     println!("Rendering ui app, curr state: {_state:?}");

//     Html(mainapp::dioxusapp().await)
//     // Html("This is great")
// }
// use include_dir::{include_dir, Dir};

// static STATIC_DIR: Dir<'_> = include_dir!("./ui/public");

// async fn runyew() -> impl IntoResponse {
//     Html(runapp().await)
// }

impl ServerApplication {
    pub async fn get_router() -> Router {
        let db = Database::new(true).await.unwrap();
        let state = ServerState { db: db };

        let corslayer = CorsLayer::new()
            .allow_methods([Method::POST, Method::GET])
            // .allow_headers([
            //     http::header::CONTENT_TYPE,
            //     http::header::ACCEPT,
            //     HeaderName::from_str("x-user-id").unwrap(),
            // ])
            .allow_headers(Any)
            .allow_origin(Any);

        // let corslayer = CorsLayer::new().allow_headers(Any);

        // let static_dir = "./dist";

        // build our application with a route
        let app: Router = Router::new()
            // .merge(setup_routes())
            .route(&format!("/surveys"), post(create_survey).get(list_survey))
            .route(&format!("/surveys/test"), post(test_survey))
            .route(&format!("/surveys/:id"), get(get_survey))
            .route(&format!("/submit"), post(submit_survey))
            .with_state(state)
            .layer(corslayer)
            .layer(DefaultBodyLimit::disable())
            .layer(RequestBodyLimitLayer::new(1 * 1024 * 1024 /* 250mb */))
            .layer(TraceLayer::new_for_http());

        return app;
    }

    pub async fn new() -> ServerApplication {
        // const V1: &str = "v1";

        // dotenvy::from_filename("dev.env").ok();
        // initialize tracing
        // tracing_subscriber::fmt::init();
        tracing_subscriber::fmt::try_init();

        // tracing_subscriber::registry()
        //     .with(
        //         tracing_subscriber::EnvFilter::try_from_default_env()
        //             .unwrap_or_else(|_| "tower_http=debug".into()),
        //     )
        //     .with(tracing_subscriber::fmt::layer());

        let app = ServerApplication::get_router().await;

        // let app = configure_app().await;
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        tracing::debug!("listening on {}", addr);

        let server = tokio::spawn(async move {
            info!("Server address: http://{addr}");
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
            info!("after axum.");
        });
        // let oauth_client = oauth_client();

        return ServerApplication {
            base_url: addr,
            server: server,
            oauth_client: None,
        };
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSurveyResponse {
    pub survey: Survey,
}

impl CreateSurveyResponse {
    fn from(survey: Survey) -> Self {
        CreateSurveyResponse { survey: survey }
    }
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn create_survey(
    headers: HeaderMap,
    State(state): State<ServerState>,
    extract::Json(payload): extract::Json<CreateSurveyRequest>,
) -> impl IntoResponse {
    info!("Called create survey");

    // Check user + type, can they make surveys?
    let testuser = HeaderValue::from_str("").unwrap();
    let user_id = headers
        .get("x-user-id")
        .unwrap_or(&testuser)
        .to_str()
        .unwrap();

    println!("Creating new survey for user={user_id:?}");
    // Check that the survey is Ok
    let parsed_survey = parse_markdown_v3(payload.plaintext);

    let metadata = MetadataBuilder::default().build().unwrap();

    let survey_model = SurveyModelBuilder::default()
        .id(metadata.id.clone())
        .plaintext(parsed_survey.plaintext.clone())
        .parse_version(parsed_survey.parse_version.clone())
        .user_id(user_id.to_string())
        .created_at(metadata.created_at.clone())
        .modified_at(metadata.modified_at.clone())
        .version(metadata.version.clone())
        .build()
        .unwrap();

    let new_survey = SurveyBuilder::default()
        .metadata(metadata)
        .survey(parsed_survey)
        .user_id(user_id.to_string())
        .build()
        .unwrap();

    let insert_result = state.db.create_survey(survey_model).await.unwrap();
    let response = CreateSurveyResponse::from(new_survey);
    (StatusCode::CREATED, Json(response))
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn submit_survey(
    State(state): State<ServerState>,
    mut multipart: Multipart,
) -> Result<Json<CreateAnswersResponse>, CustomError> {
    // let content_type_header = req.headers().get(CONTENT_TYPE);
    // let content_type => content_type_header.and_then(|value| value.to_str().ok());

    info!("Called submit survey");
    // let insert_result = _state.db.create_survey(payload).await.unwrap();
    // let response = CreateSurveyResponse::from(insert_result);
    let mut dict: HashMap<String, String> = HashMap::new();
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.text().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
        dict.insert(name, data);
    }

    info!("Turned survey into dict");

    /*
    check survey id exists in database
     */
    let submitted_survey_id = match dict.get("survey_id") {
        Some(x) => x,
        None => {
            return Err(CustomError::BadRequest("Issue found".to_string()));
        }
    };

    info!("Found survey_id in request");

    let survey = match state.db.get_survey(submitted_survey_id).await.unwrap() {
        Some(x) => x,
        None => return Err(CustomError::BadRequest("another issue".to_string())),
    };
    info!("Found survey_id in database");
    let answer_id = nanoid_gen();
    let response = CreateAnswersResponse {
        answer_id: answer_id.clone(),
    };
    let create_answer_model: CreateAnswersModel = CreateAnswersModel {
        id: None,
        answer_id: answer_id,
        external_id: "".to_string(),
        survey_id: submitted_survey_id.to_owned(),
        survey_version: "".to_string(),
        start_time: chrono::Local::now().to_string(),
        end_time: "".to_string(),
        answers: json!(dict).to_string(),
        created_at: "".to_string(),
    };

    let answer_result = state.db.create_answer(create_answer_model).await.unwrap();

    info!("completed survey submit");

    return Ok(Json(response));
}

fn try_thing() -> Result<(), anyhow::Error> {
    anyhow::bail!("it failed!")
}
#[derive(Debug, Deserialize)]
pub enum CustomError {
    BadRequest(String),
    Database(String),
}

// So that errors get printed to the browser?
impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            CustomError::Database(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            CustomError::BadRequest(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
        };

        format!("status = {}, message = {}", status, error_message).into_response()
    }
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

// #[tracing::instrument]
#[axum::debug_handler]
pub async fn get_survey(
    State(_state): State<ServerState>,
    Path(survey_id): Path<String>,
    Query(query): Query<GetSurveyQuery>,
) -> impl IntoResponse {
    let db_response = _state.db.get_survey(&survey_id).await.unwrap();
    // let response = CreateSurveyResponse::from(insert_result);
    info!("query: {query:#?}");
    println!("query: {query:#?}");

    // let results = transform_response(db_response, query);
    (StatusCode::OK, Json(db_response))
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn test_survey(
    // State(_state): State<ServerState>,
    extract::Json(payload): extract::Json<CreateSurveyRequest>,
) -> impl IntoResponse {
    let survey = parse_markdown_v3(payload.plaintext.clone());
    (StatusCode::OK, Json(survey))
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn list_survey(
    State(state): State<ServerState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    println!("Recieved headers={headers:#?}");
    let testuser = HeaderValue::from_str("").unwrap();
    let user_id = headers
        .get("x-user-id")
        .unwrap_or(&testuser)
        .to_str()
        .unwrap();

    println!("Getting surveys for user={user_id}");
    let pool = state.db.pool;

    // let count: i64 = sqlx::query_scalar("select count(*) from surveys where surveys.id = $1")
    //     .bind(user_id)
    //     .fetch_one(&pool)
    //     .await
    //     .map_err(internal_error)
    //     .unwrap();
    // println!("Survey count: {count:#?}");

    let res: Vec<SurveyModel> =
        sqlx::query_as::<_, SurveyModel>("select * from surveys where surveys.user_id = $1")
            .bind(user_id)
            .fetch_all(&pool)
            .await
            .unwrap();

    let resp = ListSurveyResponse { surveys: res };

    (StatusCode::OK, Json(resp))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListSurveyResponse {
    pub surveys: Vec<SurveyModel>,
}

// #[axum::debug_handler]
// pub async fn get_survey(
//     State(state): State<ServerState>,
//     Path(survey_id): Path<String>,
// ) -> impl IntoResponse {
//     let pool = state.db.pool;

//     let count: i64 = sqlx::query_scalar("select count(id) from surveys")
//         .fetch_one(&pool)
//         .await
//         .map_err(internal_error)
//         .unwrap();
//     println!("Survey count: {count:#?}");

//     let res = sqlx::query_as::<_, SurveyModel>("select * from surveys as s where s.id = $1")
//         .bind(survey_id)
//         .fetch_one(&pool)
//         .await
//         .unwrap();

//     println!("Survey: {res:#?}");
//     let resp_survey = parse_markdown_v3(res.plaintext.clone());
//     let response = CreateSurveyResponse {
//         survey: resp_survey,
//         // metadata: res,
//     };

//     let template = FormTemplate {
//         survey_id: response.survey.id,
//     };

//     return (StatusCode::OK, template);
// }

// #[derive(Debug, Serialize, Clone, FromRow, Deserialize)]
// pub struct Survey {
//     pub id: String,
//     nanoid: String,
//     pub plaintext: String,
//     // questions: Question,
//     user_id: String,
//     created_at: String,
//     modified_at: String,
//     version: String,
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct CreateSurveyRequest {
//     pub plaintext: String,
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct CreateSurveyResponse {
//     pub survey: Survey,
// }

use std::{collections::HashMap, net::SocketAddr};

use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    Json,
};
// use markdownparser::{markdown_to_form, parse_markdown_v3, Survey};
use serde::{Deserialize, Serialize};

// use ts_rs::TS;

// use crate::{internal_error, ServerState};

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

// impl SurveyModel {
//     fn to_survey(survey: &SurveyModel) -> Survey {
//         let survey = survey.clone();
//         let questions = markdown_to_form(survey.plaintext.clone()).questions;
//         return Survey {
//             survey: todo!(),
//             metadata: todo!(),
//             UserInfo: todo!(),
//             // nanoid: markdownparser::nanoid_gen(),
//             // parse_version: survey.parse_version,
//         };
//     }
// }

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

struct Answer {
    form_id: String,
    value: String,
}

// #[derive(Template)]
// #[template(path = "form.html")]
// struct FormTemplate {
//     survey_id: String,
// }

// pub async fn get_form(
//     State(_state): State<ServerState>,
//     Path(survey_id): Path<String>,
// ) -> FormTemplate {
//     FormTemplate {
//         survey_id: survey_id,
//     }
// }

// #[derive(Template)]
// #[template(path = "create_survey.html")]
// struct CreateSurveyTemplate {
//     // survey_value: String,
// }

// #[axum::debug_handler]
// pub async fn create_survey_form(State(_state): State<ServerState>) -> impl IntoResponse {
//     CreateSurveyTemplate {}
// }

mod test {
    use std::collections::HashMap;

    use axum::extract::{multipart, Multipart};
    use db::database::Database;

    use crate::ServerState;

    // #[tokio::test]
    // async fn test_create_answer() {
    //     let db = Database::new(true).await.unwrap();
    //     let state = ServerState { db: db };
    //     let mp = Multipart;
    //     let res = submit_survey(state, mp);
    // }
}
