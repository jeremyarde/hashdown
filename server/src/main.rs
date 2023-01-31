use axum::{
    extract::{self, State},
    http::{self, HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
// use ormlite::FromRow;
// use ormlite::{model::ModelBuilder, Model};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
// use uuid::Uuid;
// use sqlx::{Sqlite, SqlitePool};
use std::{net::SocketAddr, sync::Arc};
// use tower_http::http::cors::CorsLayer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
// use tower_http::trace::TraceLayer;
// use tower::http

use crate::db::Database;

use anyhow;
mod db;

#[derive(Debug, Clone)]
struct ServerState {
    db: Database,
}

#[derive(Deserialize, Serialize, sqlx::FromRow, Debug)]
struct CreateSurvey {
    id: String,
    plaintext: String,
}

#[derive(Debug, Serialize, Clone, FromRow, Deserialize)]
struct Survey {
    id: String,
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

    // let res = Survey::builder()
    //     .nanoid(payload.id)
    //     .plaintext(payload.plaintext)
    //     .insert(&state.db.pool)
    //     .await
    //     .unwrap();

    let res = sqlx::query("insert into surveys (id, plaintext) values ($1, $2)")
        .bind(payload.id.clone())
        .bind(payload.plaintext)
        .execute(&state.db.pool)
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
    println!("Survey count: {count:#?}");

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(payload.id))
}

#[axum::debug_handler]
async fn list_survey(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(state): State<ServerState>,
    // extract::Json(payload): extract::Json<CreateSurvey>,
) -> impl IntoResponse {
    let pool = state.db.pool;

    let count: i64 = sqlx::query_scalar("select count(id) from surveys")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
        .unwrap();
    println!("Survey count: {count:#?}");

    // let res = Survey::select()
    //     .fetch_all(&pool)
    //     .await
    //     .map_err(internal_error)
    //     .expect("Could not select all surveys");

    let res = sqlx::query_as::<_, Survey>("select * from surveys")
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("Survey: {res:#?}");

    // for item in res.into_iter() {
    //     println!("Survey: {res:#?}")
    // }
    // let mapped: Vec<String> = res.iter().map(|x| x["plaintext"]).collect();

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::OK, Json(res))
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
    // cargo watch -- cargo run
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
        .route(&format!("/survey"), post(create_survey).get(list_survey))
        // .layer(Extension(state))
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_methods([Method::POST, Method::GET])
                .allow_headers([http::header::CONTENT_TYPE, http::header::ACCEPT])
                .allow_origin("http://localhost:8080/".parse::<HeaderValue>().unwrap())
                .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap()),
        )
        .layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
