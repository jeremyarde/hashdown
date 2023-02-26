use axum::{
    extract::{self, State},
    http::{self, HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
// use ormlite::FromRow;
// use ormlite::{model::ModelBuilder, Model};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tokio::{task::JoinHandle, try_join};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
// use uuid::Uuid;
// use sqlx::{Sqlite, SqlitePool};
use std::{env, net::SocketAddr, sync::Arc};
// use tower_http::http::cors::CorsLayer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

// use crate::answer::post_answer;
use crate::{
    answer::post_answers,
    db::Database,
    survey::{create_survey, get_survey, list_survey},
    ServerState,
};
// use tower_http::trace::TraceLayer;
// use tower::http
pub struct ServerApplication {
    pub base_url: SocketAddr,
    pub server: JoinHandle<()>,
    pub oauth_client: BasicClient,
}

impl ServerApplication {
    pub async fn get_router(test: bool) -> Router {
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
            .route(&format!("/surveys"), post(create_survey).get(list_survey))
            .route("/surveys/:id", get(get_survey))
            .route("/surveys/:id/answers", post(post_answers))
            // .layer(Extension(state))
            .with_state(state)
            .layer(corslayer)
            .layer(TraceLayer::new_for_http());

        return app;
    }

    pub async fn new(test: bool) -> ServerApplication {
        // const V1: &str = "v1";

        dotenvy::from_filename("dev.env").ok();
        // initialize tracing
        // tracing_subscriber::fmt::init();

        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                    "example_parse_body_based_on_content_type=debug,tower_http=debug".into()
                }),
            )
            .with(tracing_subscriber::fmt::layer())
            .try_init()
            .unwrap();

        let app = ServerApplication::get_router(test).await;

        // let app = configure_app().await;
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        tracing::debug!("listening on {}", addr);

        let server = tokio::spawn(async move {
            println!("before axum.");
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
            println!("after axum.");
        });
        let oauth_client = oauth_client();

        return ServerApplication {
            base_url: addr,
            server: server,
            oauth_client: oauth_client,
        };
    }
}

fn oauth_client() -> BasicClient {
    // Environment variables (* = required):
    // *"CLIENT_ID"     "REPLACE_ME";
    // *"CLIENT_SECRET" "REPLACE_ME";
    //  "REDIRECT_URL"  "http://127.0.0.1:3000/auth/authorized";
    //  "AUTH_URL"      "https://discord.com/api/oauth2/authorize?response_type=code";
    //  "TOKEN_URL"     "https://discord.com/api/oauth2/token";

    // client id: 662612831867-q8ppdr4tc2gti8qgcmdbaff4b394774j.apps.googleusercontent.com

    let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID!");
    let client_secret = env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET!");
    let redirect_url = env::var("REDIRECT_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:3000/auth/authorized".to_string());

    let auth_url = env::var("AUTH_URL").unwrap_or_else(|_| {
        "https://discord.com/api/oauth2/authorize?response_type=code".to_string()
    });

    let token_url = env::var("TOKEN_URL")
        .unwrap_or_else(|_| "https://discord.com/api/oauth2/token".to_string());

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url).unwrap(),
        Some(TokenUrl::new(token_url).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}
