use axum::Router;
use chrono::{DateTime, Utc};
use db::database::MdpDatabase;

// use hyper::Server;
// use hyper::Server;
use markdownparser::{nanoid_gen, Survey};

use tokio::task::JoinHandle;

// use tower_sessions::PostgresStore;
use tracing::log::info;

use crate::{
    config::{EnvConfig},
    db::{self},
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
    pub async fn get_router(config: &EnvConfig) -> Router {
        let db = MdpDatabase::new(config.database_url.clone())
            .await
            .expect("Error connecting to database");
        let state = ServerState {
            db,
            mail: Mailer::new(config.smtp_username.clone(), config.smtp_password.clone()),
            config: config.to_owned(),
        };

        get_router(state).unwrap()
    }

    pub async fn new(config: EnvConfig) -> ServerApplication {
        info!("Spinning up the server.");

        // const V1: &str = "v1";
        let _ = tracing_subscriber::fmt::try_init();

        // tracing_subscriber::registry()
        //     .with(
        //         tracing_subscriber::EnvFilter::try_from_default_env()
        //             .unwrap_or_else(|_| "tower_http=debug".into()),
        //     )
        //     .with(tracing_subscriber::fmt::layer());

        let app = ServerApplication::get_router(&config).await;

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
