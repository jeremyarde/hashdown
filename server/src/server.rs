use askama::Template;
use axum::{
    http::{self, HeaderValue, Method},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use db::db::Database;
use oauth2::basic::BasicClient;
// use ormlite::FromRow;
// use ormlite::{model::ModelBuilder, Model};

use tokio::task::JoinHandle;
use tracing::log::info;
// use uuid::Uuid;
// use sqlx::{Sqlite, SqlitePool};
use std::net::SocketAddr;
// use tower_http::http::cors::CorsLayer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

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
    return "Yo this is great";
}

impl ServerApplication {
    pub async fn get_router() -> Router {
        let db = Database::new(true).await.unwrap();
        // let ormdb = SqliteConnection::connect(":memory:").await?;
        // let state = Arc::new(ServerState { db: db });
        let state = ServerState { db: db };

        let corslayer = CorsLayer::new()
            .allow_methods([Method::POST, Method::GET])
            .allow_headers([http::header::CONTENT_TYPE, http::header::ACCEPT])
            .allow_origin("http://127.0.0.1:8080/".parse::<HeaderValue>().unwrap())
            .allow_origin("http://127.0.0.1:8080".parse::<HeaderValue>().unwrap())
            .allow_origin("http://127.0.0.1:3000".parse::<HeaderValue>().unwrap());

        // build our application with a route
        let app: Router = Router::new()
            // .merge(setup_routes())
            .route(&format!("/surveys"), post(create_survey).get(list_survey))
            // .route("/surveys/new", get(create_survey_form))
            // .route(&format!("/surveys"), post(create_survey).get(list_survey))
            // .route("/surveys/:id", get(get_survey).post(post_answers))
            // .route("/surveys/:id/answers", post(post_answers))
            // .layer(Extension(state))
            // .route("/template", get(post_answers))
            .route("/", get(hello))
            .with_state(state)
            .layer(corslayer)
            .layer(TraceLayer::new_for_http());

        return app;
    }

    pub async fn new() -> ServerApplication {
        // const V1: &str = "v1";

        // dotenvy::from_filename("dev.env").ok();
        // initialize tracing
        // tracing_subscriber::fmt::init();

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
            info!("Server address: {addr}");
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

// fn oauth_client() -> BasicClient {
//     // Environment variables (* = required):
//     // *"CLIENT_ID"     "REPLACE_ME";
//     // *"CLIENT_SECRET" "REPLACE_ME";
//     //  "REDIRECT_URL"  "http://127.0.0.1:3000/auth/authorized";
//     //  "AUTH_URL"      "https://discord.com/api/oauth2/authorize?response_type=code";
//     //  "TOKEN_URL"     "https://discord.com/api/oauth2/token";

//     // client id: 662612831867-q8ppdr4tc2gti8qgcmdbaff4b394774j.apps.googleusercontent.com

//     let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID!");
//     let client_secret = env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET!");
//     let redirect_url = env::var("REDIRECT_URL")
//         .unwrap_or_else(|_| "http://127.0.0.1:3000/auth/authorized".to_string());

//     let auth_url = env::var("AUTH_URL").unwrap_or_else(|_| {
//         "https://discord.com/api/oauth2/authorize?response_type=code".to_string()
//     });

//     let token_url = env::var("TOKEN_URL")
//         .unwrap_or_else(|_| "https://discord.com/api/oauth2/token".to_string());

//     BasicClient::new(
//         ClientId::new(client_id),
//         Some(ClientSecret::new(client_secret)),
//         AuthUrl::new(auth_url).unwrap(),
//         Some(TokenUrl::new(token_url).unwrap()),
//     )
//     .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
// }

#[axum::debug_handler]
pub async fn create_survey(
    State(_state): State<ServerState>,
    extract::Json(payload): extract::Json<CreateSurveyRequest>,
) -> impl IntoResponse {
    let survey = parse_markdown_v3(payload.plaintext.clone());
    // let survey = Survey::from(payload.plaintext.clone());
    let response_survey = survey.clone();
    let now = chrono::offset::Utc::now();
    let nowstr = now.to_string();
    let _res = sqlx::query!(
        r#"insert into surveys (id, plaintext, user_id, created_at, modified_at, version, parse_version)
        values 
        ($1, $2, $3, $4, $5, $6, $7)
        "#,
        response_survey.id,
        payload.plaintext,
        survey.user_id,
        survey.created_at,
        survey.modified_at,
        "1",
        nowstr
    );

    let response = CreateSurveyResponse {
        survey: Survey::from(response_survey),
        // metadata: res,
    };

    (StatusCode::CREATED, Json(response))
}

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
        // metadata: res,
    };

    let template = FormTemplate {
        survey_id: response.survey.id,
    };

    return (StatusCode::OK, template);
}

// #[derive(Debug, Serialize, Clone, FromRow, Deserialize)]
// pub struct Survey {
//     pub id: String,
//     nanoid: String,
//     pub plaintext: String,
//     user_id: String,
//     created_at: String,
//     modified_at: String,
//     version: String,
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSurveyRequest {
    pub plaintext: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSurveyResponse {
    pub survey: Survey,
}

use std::collections::HashMap;

use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    Json,
};
use markdownparser::{markdown_to_form, parse_markdown_v3, Survey};
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

#[derive(Template)]
#[template(path = "form.html")]
struct FormTemplate {
    survey_id: String,
}

pub async fn get_form(
    State(_state): State<ServerState>,
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
pub async fn create_survey_form(State(_state): State<ServerState>) -> impl IntoResponse {
    CreateSurveyTemplate {}
}
