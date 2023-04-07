use std::{hash::Hash, path::PathBuf};

use askama::Template;
use axum::{
    body::{self, boxed, Body, Empty, Full},
    extract::Multipart,
    http::{self, HeaderValue, Method, Response},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use db::{database::Database, models::CreateSurveyRequest};
use markdownparser::{markdown_to_form, parse_markdown_v3, Survey};
use oauth2::basic::BasicClient;
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
            .allow_headers([http::header::CONTENT_TYPE, http::header::ACCEPT])
            .allow_origin(Any);

        // let static_dir = "./dist";

        // build our application with a route
        let app: Router = Router::new()
            // .merge(setup_routes())
            .route(&format!("/surveys"), post(create_survey).get(list_survey))
            .route(&format!("/surveys/test"), post(test_survey))
            .route(&format!("/surveys/:id"), get(get_survey))
            .route(&format!("/surveys/:id/submit"), post(submit_survey))
            .with_state(state)
            .layer(corslayer)
            .layer(TraceLayer::new_for_http());

        return app;
    }

    pub async fn new() -> ServerApplication {
        // const V1: &str = "v1";

        // dotenvy::from_filename("dev.env").ok();
        // initialize tracing
        tracing_subscriber::fmt::init();

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
    State(_state): State<ServerState>,
    extract::Json(payload): extract::Json<CreateSurveyRequest>,
) -> impl IntoResponse {
    let insert_result = _state.db.create_survey(payload).await.unwrap();
    let response = CreateSurveyResponse::from(insert_result);
    (StatusCode::CREATED, Json(response))
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn submit_survey(
    State(_state): State<ServerState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    // let insert_result = _state.db.create_survey(payload).await.unwrap();
    // let response = CreateSurveyResponse::from(insert_result);
    let mut dict: HashMap<String, String> = HashMap::new();
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.text().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
        dict.insert(name, data);
    }
    (StatusCode::CREATED, Json(dict))
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn get_survey(
    State(_state): State<ServerState>,
    Path(survey_id): Path<String>,
) -> impl IntoResponse {
    let (db_response) = _state.db.get_survey(survey_id).await.unwrap();
    // let response = CreateSurveyResponse::from(insert_result);
    (StatusCode::OK, Json(db_response))
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn test_survey(
    State(_state): State<ServerState>,
    extract::Json(payload): extract::Json<CreateSurveyRequest>,
) -> impl IntoResponse {
    let (insert_result, expected_answers) = _state.db.test_survey(payload).await.unwrap();
    let response = CreateSurveyResponse::from(insert_result);

    (
        StatusCode::OK,
        Json(json!({"survey": response, "expect": expected_answers})),
    )
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn list_survey(State(state): State<ServerState>) -> impl IntoResponse {
    let pool = state.db.pool;

    let count: i64 = sqlx::query_scalar("select count(*) from surveys")
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

#[derive(Deserialize, Serialize, Debug)]
pub struct ListSurveyResponse {
    pub surveys: Vec<Survey>,
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
            // questions: questions,
            version: survey.version,
            questions,
            parse_version: "0".to_string(),
            // nanoid: markdownparser::nanoid_gen(),
            // parse_version: survey.parse_version,
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
