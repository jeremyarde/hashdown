use axum::{
    http::{self, HeaderName, Method},
    middleware::{self},
    Router,
};
use chrono::{DateTime, Utc};
use db::database::Database;

use hyper::header::CONTENT_ENCODING;
use markdownparser::{nanoid_gen, parse_markdown_v3, Survey};

use serde_json::Value;
use sqlx::FromRow;
use tokio::task::JoinHandle;

use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

// use tower_sessions::PostgresStore;
use tracing::log::info;

use crate::{
    auth::validate_session_middleware,
    config::EnvConfig,
    db::{
        self,
        database::{ConnectionDetails, Session},
    },
    mail::mail::Mailer,
    routes::routes::{get_router, CreateSurveyRequest},
    ServerState,
};

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr};

pub struct ServerApplication {
    pub base_url: SocketAddr,
    pub server: JoinHandle<()>,
}

impl ServerApplication {
    pub async fn get_router() -> Router {
        let in_memory = dotenvy::var("DATABASE_IN_MEMORY")
            .expect("Could not find `DATABASE_IN_MEMORY` env variable")
            .trim()
            == "true";

        let uri = dotenvy::var("DATABASE_URL").expect("Could not get connection string from env");
        let db_url = ConnectionDetails(uri);
        let db = Database::new(in_memory, db_url)
            .await
            .expect("Error connecting to database");
        let state = ServerState {
            db,
            mail: Mailer::new(),
            config: EnvConfig::new(),
        };

        let router = get_router(state).unwrap();
        return router;
    }

    pub async fn new() -> ServerApplication {
        info!("Spinning up the server.");

        // const V1: &str = "v1";
        let _ = tracing_subscriber::fmt::try_init();

        // tracing_subscriber::registry()
        //     .with(
        //         tracing_subscriber::EnvFilter::try_from_default_env()
        //             .unwrap_or_else(|_| "tower_http=debug".into()),
        //     )
        //     .with(tracing_subscriber::fmt::layer());

        let app = ServerApplication::get_router().await;

        // let app = configure_app().await;
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
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

        info!("Server is running...");
        ServerApplication {
            base_url: addr,
            server,
            // oauth_client: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSurveyResponse {
    pub survey: Survey,
}

impl CreateSurveyResponse {
    fn from(survey: Survey) -> Self {
        CreateSurveyResponse { survey }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub metadata_id: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub version: String,
}

impl Metadata {
    fn new() -> Self {
        Self {
            metadata_id: nanoid_gen(24),
            created_at: Utc::now(),
            modified_at: Utc::now(),
            version: "0".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct SurveyModel {
    pub id: i32,
    pub survey_id: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub plaintext: String,
    // pub questions: Option<Vec<Question>>,
    pub version: Option<String>,
    pub parse_version: Option<String>,
    pub parsed_json: Option<Value>,
}
impl SurveyModel {
    pub(crate) fn new(payload: CreateSurveyRequest, session: &Session) -> SurveyModel {
        let parsed_survey =
            parse_markdown_v3(payload.plaintext.clone()).expect("Could not parse the survey");
        let metadata = Metadata::new();
        return SurveyModel {
            id: 0,
            survey_id: nanoid_gen(12),
            plaintext: payload.plaintext,
            user_id: session.user_id.to_owned(),
            created_at: metadata.created_at,
            modified_at: metadata.modified_at,
            version: Some("fixme".to_string()),
            parse_version: Some(parsed_survey.parse_version.clone()),
            parsed_json: Some(serde_json::to_value(parsed_survey.clone()).unwrap()),
        };
    }
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

mod test {

    // #[tokio::test]
    // async fn test_create_answer() {
    //     let db = Database::new(true).await.unwrap();
    //     let state = ServerState { db: db };
    //     let mp = Multipart;
    //     let res = submit_survey(state, mp);
    // }
}
