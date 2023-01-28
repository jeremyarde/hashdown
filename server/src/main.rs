use axum::{
    extract::{self, State},
    http::{self, HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use ormlite::{model::ModelBuilder, Model};
use serde::{Deserialize, Serialize};
// use uuid::Uuid;
// use sqlx::{Sqlite, SqlitePool};
use std::{net::SocketAddr, sync::Arc};
// use tower_http::http::cors::CorsLayer;
use tower_http::cors::CorsLayer;
// use tower_http::trace::TraceLayer;
// use tower::http

use crate::db::Database;

use anyhow;
mod db;

#[derive(Debug, Clone)]
struct ServerState {
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

#[derive(Debug, Serialize, Model, Clone)]
struct Survey {
    id: i32,
    nanoid: String,
    plaintext: String,
    user_id: String,
    created_at: String,
    modified_at: String,
    version: String,
}

#[axum::debug_handler]
async fn create_survey(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(state): State<ServerState>,
    extract::Json(payload): extract::Json<CreateSurvey>,
) -> impl IntoResponse {
    // insert your application logic here
    // let survey = "yo";
    // let survey = CreateSurvey {
    //     id: payload.id,
    //     plaintext: payload.plaintext,
    // };

    // let insert = InsertSurvey {
    //     id: payload.id,
    //     plaintext: payload.plaintext,
    // };

    let res = Survey::builder()
        .nanoid(payload.id)
        .plaintext(payload.plaintext)
        .insert(&state.db.pool)
        .await
        .unwrap();

    let pool = state.db.pool;
    // let res = sqlx::query_as::<_, Survey>(
    //     "insert into surveys (id, plaintext) values ($1, $2) returning *;",
    // )
    // .bind(survey.id)
    // .bind(survey.plaintext)
    // .fetch_one(&pool)
    // .await
    // .map_err(internal_error)
    // .unwrap();
    // let res: Survey = insert.insert(&pool).await.into();

    let count: i64 = sqlx::query_scalar("select count(id) from surveys")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
        .unwrap();
    // let countval: i32 = count.into();

    println!("Survey count: {count:#?}");

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(res))
}

#[axum::debug_handler]
async fn list_survey(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(state): State<ServerState>,
    // extract::Json(payload): extract::Json<CreateSurvey>,
) -> impl IntoResponse {
    let pool = state.db.pool;
    // let res = sqlx::query_as::<_, Survey>(
    //     "select id, plaintext, user_id, created_at, modified_at from surveys",
    // )
    // .fetch_all(&pool)
    // .await
    // .map_err(internal_error)
    // .unwrap();
    let res = Survey::select()
        .fetch_all(&pool)
        .await
        .map_err(internal_error)
        .unwrap();

    println!("Survey: {res:#?}");

    // for item in res.into_iter() {
    //     println!("Survey: {res:#?}")
    // }
    // let mapped: Vec<String> = res.iter().map(|x| x["plaintext"]).collect();

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::FOUND, Json(res))
}

// #[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
// struct Survey {
//     id: String,
//     plaintext: String,
//     user_id: String,
//     created_at: String,
//     modified_at: String,
// }

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const V1: &str = "v1";

    dotenvy::from_filename("dev.env").ok();
    // initialize tracing
    tracing_subscriber::fmt::init();

    let db = Database::new(true).await?;
    // let ormdb = SqliteConnection::connect(":memory:").await?;
    // let state = Arc::new(ServerState { db: db });
    let state = ServerState { db: db };

    // build our application with a route
    let app = Router::new()
        .route(
            &format!("/{V1}/survey"),
            post(create_survey).get(list_survey),
        )
        // .layer(Extension(state))
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_methods([Method::POST])
                .allow_headers([http::header::CONTENT_TYPE])
                // .header("Access-Control-Allow-Origin", "http://localhost:8080/")
                // .header("Access-Control-Allow-Origin", "http://localhost:3000/")
                // .header(reqwest::header::CONTENT_TYPE, "application/json")
                .allow_origin("http://localhost:8080/".parse::<HeaderValue>().unwrap()),
        );
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
