use axum::{
    extract,
    http::{self, HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{Sqlite, SqlitePool};
use std::{net::SocketAddr, sync::Arc};
// use tower_http::http::cors::CorsLayer;
use tower_http::cors::CorsLayer;
// use tower_http::trace::TraceLayer;

// use tower::http

use crate::db::Database;

use anyhow;
mod db;

struct State {
    db: Database,
}

// Cross-Origin Request Blocked: The Same Origin Policy disallows reading the remote resource at http://localhost:3000/survey. (Reason: CORS header “Access-Control-Allow-Origin” does not match “http://localhost:8080/”).

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(Deserialize, Serialize, sqlx::FromRow, Debug)]
struct CreateSurvey {
    id: String,
    plaintext: String,
}

async fn create_survey(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    extract::Json(payload): extract::Json<CreateSurvey>,
) -> impl IntoResponse {
    // insert your application logic here
    let survey = CreateSurvey {
        id: payload.id,
        plaintext: payload.plaintext,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(survey))
}

// the input to our `create_user` handler
// #[derive(Deserialize)]
// struct CreateUser {
//     username: String,
// }

// // the output to our `create_user` handler
// #[derive(Serialize)]
// struct User {
//     id: u64,
//     username: String,
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let db = Database::new(true).await?;
    let state = Arc::new(State { db: db });

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/survey", post(create_survey))
        .layer(Extension(state))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::POST])
                .allow_headers([http::header::CONTENT_TYPE])
                .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap()),
            // .allow_origin(tower_http::cors::Any),
        );
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
