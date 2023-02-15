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
use tokio::task::JoinHandle;
// use uuid::Uuid;
// use sqlx::{Sqlite, SqlitePool};
use std::{net::SocketAddr, sync::Arc};
// use tower_http::http::cors::CorsLayer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
// use tower_http::trace::TraceLayer;
// use tower::http

use crate::{
    db::Database,
    survey::{create_survey, get_survey, list_survey},
};
mod survey;

use anyhow;
mod db;

#[derive(Debug, Clone)]
pub struct ServerState {
    db: Database,
}

#[derive(Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct CreateSurvey {
    id: String,
    plaintext: String,
}

#[axum::debug_handler]
async fn answer_survey(
    State(state): State<ServerState>,
    extract::Json(payload): extract::Json<AnswerSurveyRequest>,
) -> impl IntoResponse {
    /*
    1. check for survey in database, with same version
    2. check that questions are the same as expected
     */

    (StatusCode::ACCEPTED, Json("fakeid".to_string()))
}

#[derive(Debug, Serialize, Clone, FromRow, Deserialize)]
struct AnswerSurveyRequest {
    id: String,
}
#[derive(Debug, Serialize, Clone, FromRow, Deserialize)]
struct AnswerSurveyResponse {
    id: String,
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
    ServerApplication::new(false).await;
    Ok(())
}

struct ServerApplication {
    pub base_url: SocketAddr,
    server: JoinHandle<()>,
}

impl ServerApplication {
    async fn new(test: bool) -> ServerApplication {
        const V1: &str = "v1";

        dotenvy::from_filename("dev.env").ok();
        // initialize tracing
        tracing_subscriber::fmt::init();

        let db = Database::new(true).await.unwrap();
        // let ormdb = SqliteConnection::connect(":memory:").await?;
        // let state = Arc::new(ServerState { db: db });
        let state = ServerState { db: db };

        // build our application with a route
        let app: Router = Router::new()
            .route(&format!("/surveys"), post(create_survey).get(list_survey))
            .route("/surveys/:id", get(get_survey).post(answer_survey))
            // .layer(Extension(state))
            .with_state(state)
            .layer(
                CorsLayer::new()
                    .allow_methods([Method::POST, Method::GET])
                    .allow_headers([http::header::CONTENT_TYPE, http::header::ACCEPT])
                    .allow_origin("http://localhost:8080/".parse::<HeaderValue>().unwrap())
                    .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
                    .allow_origin("http://localhost:3001".parse::<HeaderValue>().unwrap()),
            )
            .layer(TraceLayer::new_for_http());

        // let app = configure_app().await;
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        tracing::debug!("listening on {}", addr);

        let server = tokio::spawn(async move {
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        });

        return ServerApplication {
            base_url: addr,
            server: server,
        };
    }
}

impl Drop for ServerApplication {
    fn drop(&mut self) {
        tracing::debug!("Dropping test server at {:?}", self.base_url);
        self.server.abort()
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{self, Request},
    };
    use mime::Mime;

    use crate::{CreateSurvey, ServerApplication};

    #[tokio::test]
    async fn list_survey_test() {
        let app = ServerApplication::new().await;

        // let get = Request::builder()
        //     .method(http::Method::GET)
        //     .uri("/surveys")
        //     // .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        //     .body(Body::from(serde_json::to_string("").unwrap()))
        //     .unwrap();

        let mut headers = http::header::HeaderMap::new();
        // headers.insert(
        //     // "Content-Type",
        //     header::CONTENT_TYPE,
        //     header::HeaderValue::from_static("application/json"),
        // );
        // headers.insert(header::CONTENT_ENCODING,
        // header::)
        headers.insert("", val);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let client_url = format!("http://{}{}", app.base_url.to_string(), "/surveys");
        println!("Client sending to: {client_url}");

        let response = client
            .post(client_url)
            .json(&CreateSurvey {
                id: "test".to_string(),
                plaintext: "- another\n - this one".to_string(),
            })
            .send()
            .await
            .unwrap();

        // client
        // .post("http://localhost:3000/survey")
        // .json(&CreateSurvey {
        //     id,
        //     plaintext: content,
        // })
        // .send()
        // .await

        // .body(json!({ "id": "test", "plaintext": "- header here\n  - this is a question" }));
        // client.execute(Request::builder().body(body));

        println!("Response: {response:?}");
        // let create_resp = serde_json::from_slice(response.into_body());
        // assert_eq!(response.status(), StatusCode::OK);

        // let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        // let body: Value = serde_json::from_slice(&body).unwrap();

        // assert_eq!(body, json!({ "data": [1, 2, 3, 4] }));
    }

    #[tokio::test]
    async fn create_survey_test() {
        // let app = configure_app().await;
        let app = ServerApplication::new().await;
        // let response = app.oneshot(get_create_survey_request()).await.unwrap();

        // println!("response: {response:?}");
        // let create_resp = serde_json::from_slice(response.into_body());
        // assert_eq!(response.status(), StatusCode::OK);

        // let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        // let body: Survey = serde_json::from_slice(&body).unwrap();

        // println!("real response: {body:?}");
        // assert_eq!(body, json!({ "data": [1, 2, 3, 4] }));
    }

    fn get_create_survey_request() -> Request<Body> {
        let create_request = CreateSurvey {
            id: "test".to_string(),
            plaintext: "- this is the titles\n  - option 1".to_string(),
        };
        Request::builder()
            .method(http::Method::POST)
            .uri("/surveys")
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(serde_json::to_string(&create_request).unwrap()))
            .unwrap()
    }
}
