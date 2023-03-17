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
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
// use uuid::Uuid;
// use sqlx::{Sqlite, SqlitePool};
use std::net::SocketAddr;
// use tower_http::http::cors::CorsLayer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

// use crate::answer::post_answer;
use crate::ServerState;

use self::handlers::{create_survey, get_survey, list_survey};
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

mod handlers;
// use handlers;

impl ServerApplication {
    pub async fn get_router(_test: bool) -> Router {
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
            .route("/surveys/new", get(create_survey_form))
            .route(&format!("/surveys"), post(create_survey).get(list_survey))
            .route("/surveys/:id", get(get_survey).post(post_answers))
            .route("/surveys/:id/answers", post(post_answers))
            // .layer(Extension(state))
            .route("/template", get(post_answers))
            .route("/", get(hello))
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
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "tower_http=debug".into()),
            )
            .with(tracing_subscriber::fmt::layer());

        let app = ServerApplication::get_router(test).await;

        // let app = configure_app().await;
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        tracing::debug!("listening on {}", addr);

        let server = tokio::spawn(async move {
            println!("Server address: {addr}");
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await;
            println!("after axum.");
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
