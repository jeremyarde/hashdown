use axum::Router;
use chrono::{DateTime, Utc};
use db::database::Database;

// use hyper::Server;
// use hyper::Server;
use markdownparser::{nanoid_gen, ParsedSurvey, Survey};

use serde_json::{json, Value};
use sqlx::FromRow;
use tokio::task::JoinHandle;

// use tower_sessions::PostgresStore;
use tracing::log::info;

use crate::{
    config::EnvConfig,
    db::{self, database::ConnectionDetails},
    mail::mailer::Mailer,
    routes::get_router,
    ServerState,
};

use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, str::FromStr};

pub struct ServerApplication {
    pub base_url: SocketAddr,
    pub server: JoinHandle<()>,
}

impl ServerApplication {
    pub async fn get_router() -> Router {
        let uri = dotenvy::var("DATABASE_URL").expect("Could not get connection string from env");
        let db_url = ConnectionDetails(uri);
        let db = Database::new(db_url)
            .await
            .expect("Error connecting to database");
        let state = ServerState {
            db,
            mail: Mailer::new(),
            config: EnvConfig::new(),
        };

        get_router(state).unwrap()
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
        let addr =
            SocketAddr::from_str(&dotenvy::var("SERVER_URL").expect("server url should be set"))
                .expect("Should parse server url properly");
        tracing::debug!("listening on {}", addr);

        let server = tokio::spawn(async move {
            info!("Server address: http://{addr}");
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            axum::serve(listener, app.into_make_service())
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

// impl CreateSurveyResponse {
//     fn from(survey: Survey) -> Self {
//         CreateSurveyResponse { survey }
//     }
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub metadata_id: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub version: String,
}

impl Metadata {
    pub fn new() -> Self {
        Self {
            metadata_id: nanoid_gen(24),
            created_at: Utc::now(),
            modified_at: Utc::now(),
            version: "0".to_string(),
        }
    }
}

// struct Form {
//     id: String,
//     views: i32,
//     starts: i32,
//     submissions: i32,
//     completions: i32,
//     created_on: String,
//     modified_on: String,
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct Answers {
//     id: String,
//     used_id: String,
//     survey_id: String,
//     submitted_on: String,
//     answers: HashMap<String, String>,
// }

// struct Answer {
//     form_id: String,
//     value: String,
// }

mod test {

    // #[tokio::test]
    // async fn test_create_answer() {
    //     let db = Database::new(true).await.unwrap();
    //     let state = ServerState { db: db };
    //     let mp = Multipart;
    //     let res = submit_survey(state, mp);
    // }
}
