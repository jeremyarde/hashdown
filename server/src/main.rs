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
use tokio::{task::JoinHandle, try_join};
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
    server::ServerApplication,
    survey::{create_survey, get_survey, list_survey},
};
mod answer;
mod db;
mod server;
mod survey;
use anyhow;

#[derive(Debug, Clone)]
pub struct ServerState {
    db: Database,
}

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
    // curl -X GET 127.0.0.1:3000/surveys
    // curl -X GET https://127.0.0.1:3000/surveys
    /*

    curl -X POST http://localhost:3000/surveys \
       -H 'Content-Type: application/json' \
       -d '{"id": "test", "plaintext": "content"}'
        */

    // cargo watch -- cargo run
    println!("Spinning up the server.");
    let server_app = ServerApplication::new(false).await;
    println!("Server is running...");

    try_join!(server_app.server).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, time::Duration};

    use axum::{
        body::Body,
        http::{self, Request},
    };
    use mime::Mime;
    use reqwest::StatusCode;
    use serde_json::json;
    use serial_test::serial;
    use tower::{Service, ServiceExt};

    use crate::{
        answer::{AnswerDetails, AnswerType, CreateAnswersRequest, CreateAnswersResponse},
        survey::{AnswerRequest, CreateSurveyRequest, CreateSurveyResponse, ListSurveyResponse},
        ServerApplication,
    };

    #[serial]
    #[tokio::test]
    async fn list_survey_test() {
        let app = ServerApplication::new(true).await;
        let mut router = ServerApplication::get_router(true).await;
        router.ready().await.unwrap();

        let client = reqwest::Client::builder()
            // .default_headers(headers)
            .build()
            .unwrap();

        let client_url = format!("http://{}{}", app.base_url.to_string(), "/surveys");
        // let client_url = format!("/surveys");

        println!("Client sending to: {client_url}");

        let response = client
            .post(&client_url)
            .json(&CreateSurveyRequest {
                plaintext: "- another\n - this one".to_string(),
            })
            .send()
            .await
            .unwrap();

        let results: CreateSurveyResponse = response.json().await.unwrap();

        assert_eq!(results.survey.plaintext, "- another\n - this one");

        //call list
        let listresponse = client.get(&client_url).send().await.unwrap();
        let listresults: ListSurveyResponse = listresponse.json().await.unwrap();

        assert_eq!(listresults.surveys.len(), 1);
        assert_eq!(listresults.surveys[0].plaintext, "- another\n - this one");
    }

    #[tokio::test]
    #[serial]
    async fn create_survey_test() {
        let app = ServerApplication::new(true).await;
        let mut router = ServerApplication::get_router(true).await;
        router.ready().await.unwrap();

        let client = reqwest::Client::builder()
            // .default_headers(headers)
            .build()
            .unwrap();

        let client_url = format!("http://{}{}", "localhost:8080", "/surveys");
        // let client_url = format!("/surveys");

        println!("Client sending to: {client_url}");

        let response = client
            .post(&client_url)
            .json(&CreateSurveyRequest {
                plaintext: "- create\n - this one".to_string(),
            })
            .send()
            .await
            .unwrap();

        let results: CreateSurveyResponse = response.json().await.unwrap();

        assert_eq!(results.survey.plaintext, "- create\n - this one");
    }

    #[tokio::test]
    #[serial]
    async fn answer_survey_test() {
        let app = ServerApplication::new(true).await;
        let mut router = ServerApplication::get_router(true).await;
        router.ready().await.unwrap();

        let client = reqwest::Client::builder().build().unwrap();

        let client_url = format!("http://{}{}", "localhost:8080", "/surveys");
        // let client_url = format!("/surveys");
        println!("Client sending to: {client_url}");

        let response = client
            .post(&client_url)
            .json(&CreateSurveyRequest {
                plaintext: "- another\n - this one".to_string(),
            })
            .send()
            .await
            .unwrap();

        let results: CreateSurveyResponse = response.json().await.unwrap();

        assert_eq!(results.survey.plaintext, "- another\n - this one");

        let listresponse = client.get(&client_url).send().await.unwrap();
        let listresults: ListSurveyResponse = listresponse.json().await.unwrap();

        assert_eq!(listresults.surveys.len(), 1);
        assert_eq!(listresults.surveys[0].plaintext, "- another\n - this one");

        let mut answers = HashMap::new();
        answers.insert(
            listresults.surveys[0].questions[0].id.clone(),
            AnswerDetails {
                r#type: AnswerType::String,
                values: listresults.surveys[0].questions[0]
                    .options
                    .iter()
                    .map(|x| x.text.clone())
                    .collect(),
            },
        );
        let answers_request = CreateAnswersRequest {
            survey_id: listresults.surveys[0].id.clone(),
            start_time: "".to_string(),
            answers: answers,
            survey_version: listresults.surveys[0].version.clone(),
        };

        let response = client
            .post(format!(
                "{client_url}/{}/answers",
                listresults.surveys[0].questions[0].id.clone()
            ))
            .json(&answers_request)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
        let answer_response: CreateAnswersResponse = response.json().await.unwrap();

        println!("Create answer response: {answer_response:?}");
    }
}
