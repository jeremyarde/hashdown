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
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
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

#[derive(Deserialize, Serialize, sqlx::FromRow, Debug, PartialEq, Eq)]
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

struct ServerApplication {
    pub base_url: SocketAddr,
    server: JoinHandle<()>,
}

impl ServerApplication {
    async fn get_router(test: bool) -> Router {
        let db = Database::new(true).await.unwrap();
        // let ormdb = SqliteConnection::connect(":memory:").await?;
        // let state = Arc::new(ServerState { db: db });
        let state = ServerState { db: db };

        let corslayer = if !test {
            println!("Not testing, adding CORS headers.");
            CorsLayer::new()
                .allow_methods([Method::POST, Method::GET])
                .allow_headers([http::header::CONTENT_TYPE, http::header::ACCEPT])
                .allow_origin("http://127.0.0.1:8080/".parse::<HeaderValue>().unwrap())
                .allow_origin("http://127.0.0.1:8080".parse::<HeaderValue>().unwrap())
                .allow_origin("http://127.0.0.1:3001".parse::<HeaderValue>().unwrap())
        } else {
            println!("Testing, adding wildcard CORS headers.");
            CorsLayer::new()
                .allow_methods([Method::POST, Method::GET])
                .allow_headers([http::header::CONTENT_TYPE, http::header::ACCEPT])
                .allow_origin("*".parse::<HeaderValue>().unwrap())
        };

        let corslayer = CorsLayer::new();

        // build our application with a route
        let app: Router = Router::new()
            .route(&format!("/surveys"), post(create_survey).get(list_survey))
            .route("/surveys/:id", get(get_survey).post(answer_survey))
            // .layer(Extension(state))
            .with_state(state)
            .layer(corslayer)
            .layer(TraceLayer::new_for_http());

        return app;
    }

    async fn new(test: bool) -> ServerApplication {
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
            .init();

        let app = ServerApplication::get_router(test).await;

        // let app = configure_app().await;
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        tracing::debug!("listening on {}", addr);

        let server = tokio::spawn(async move {
            println!("before axum.");
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
            println!("after axum.");
        });

        // println!("before join");
        // tokio::try_join!(server);
        // print!("after join");
        // let server = tokio::spawn(async move {});
        // axum::Server::bind(&addr)
        //     .serve(app.into_make_service())
        //     .await
        //     .unwrap();

        return ServerApplication {
            base_url: addr,
            server: server,
        };
    }

    // async fn run(&self) {
    //     let _ = tokio::try_join!(self.server);
    // }
}

// impl Drop for ServerApplication {
//     fn drop(&mut self) {
//         tracing::debug!("Dropping test server at {:?}", self.base_url);
//         self.server.abort()
//     }
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // curl -X GET 127.0.0.1:3000/surveys
    // curl -X GET https://127.0.0.1:3000/surveys
    /*

    curl -X POST http://localhost:3000/surveys \
       -H 'Content-Type: application/json' \
       -d '{"id": "test", "plaintext": "content"}'



        */

    // cargo watch -- cargo run
    println!("Spinning up the server.");
    ServerApplication::new(false).await;
    println!("Server is running...");
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use axum::{
        body::Body,
        http::{self, Request},
    };
    use mime::Mime;
    use serde_json::json;

    use crate::{CreateSurvey, ServerApplication};

    #[tokio::test]
    async fn list_survey_test() {
        let app = ServerApplication::new(true).await;

        tokio::time::sleep(Duration::from_secs(2)).await;
        // let get = Request::builder()
        //     .method(http::Method::GET)
        //     .uri("/surveys")
        //     // .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        //     .body(Body::from(serde_json::to_string("").unwrap()))
        //     .unwrap();

        // headers.insert(
        //     // "Content-Type",
        //     header::CONTENT_TYPE,
        //     header::HeaderValue::from_static("application/json"),
        // );
        // headers.insert(header::CONTENT_ENCODING,
        // header::)

        let client = reqwest::Client::builder()
            // .default_headers(headers)
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
        let results: CreateSurvey = response.json().await.unwrap();
        println!("results: {results:?}");

        // let create_resp = serde_json::from_slice(response.into_body());
        // assert_eq!(response.status(), StatusCode::OK);

        // let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        // let body: Value = serde_json::from_slice(&body).unwrap();
        // serde_json::from_value(results);
        assert_eq!(
            serde_json::to_value(results).unwrap(),
            json!({"test": "yo"})
        );
    }

    #[tokio::test]
    async fn create_survey_test() {
        // let app = configure_app().await;
        let app = ServerApplication::new(true).await;
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
