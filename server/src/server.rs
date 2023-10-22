use axum::{
    http::{self, HeaderName, Method},
    middleware::{self},
    Router,
};
use chrono::{DateTime, Utc};
use db::database::Database;

use hyper::header::CONTENT_ENCODING;
use markdownparser::Survey;

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
    db::{self, database::ConnectionDetails},
    mail::mail::Mailer,
    routes::routes::get_routes,
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

        let mut origins = vec![];
        info!("Starting app in stage={:?}", state.config.stage);
        if state.config.is_dev() {
            origins.append(&mut vec![
                "http://localhost:3000".parse().unwrap(),
                "http://localhost:3001".parse().unwrap(),
                "http://localhost:8080".parse().unwrap(),
                "http://localhost:5173".parse().unwrap(),
                // "http://api.example.com".parse().unwrap(),
            ]);
        }
        let corslayer = CorsLayer::new()
            .allow_methods([Method::POST, Method::GET])
            .allow_headers([
                http::header::CONTENT_TYPE,
                http::header::ACCEPT,
                http::header::AUTHORIZATION,
                http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                http::header::ACCESS_CONTROL_REQUEST_METHOD,
                HeaderName::from_static("x-auth-token"),
                HeaderName::from_static("x-sid"),
                HeaderName::from_static("session_id"),
                HeaderName::from_static("credentials"),
            ])
            // .allow_headers(Any)
            .allow_credentials(true)
            .allow_origin(origins)
            .expose_headers([CONTENT_ENCODING, HeaderName::from_static("session_id")]);
        // .expose_headers([HeaderName::from_static("session_id")]);

        // build our application with a route

        // let session_store = MemoryStore::default();
        // let session_store = PostgresStore::new(state.db.pool.clone());
        // session_store.migrate().await.unwrap();
        // let deletion_task

        // let session_service = ServiceBuilder::new()
        //     .layer(HandleErrorLayer::new(|_: BoxError| async {
        //         StatusCode::BAD_REQUEST
        //     }))
        //     .layer(
        //         SessionManagerLayer::new(session_store)
        //             .with_secure(true)
        //             .with_max_age(Duration::minutes(10)),
        //     );

        let auth_session_service = ServiceBuilder::new().layer(middleware::from_fn_with_state(
            state.clone(),
            validate_session_middleware,
        ));

        Router::new()
            .merge(get_routes(state).unwrap())
            .layer(corslayer)
            .layer(TraceLayer::new_for_http())
            .layer(auth_session_service)
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

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct SurveyModel {
    pub id: i32,
    pub survey_id: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub plaintext: String,
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

mod test {

    // #[tokio::test]
    // async fn test_create_answer() {
    //     let db = Database::new(true).await.unwrap();
    //     let state = ServerState { db: db };
    //     let mp = Multipart;
    //     let res = submit_survey(state, mp);
    // }
}
