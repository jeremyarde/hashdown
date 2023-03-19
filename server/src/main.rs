use axum::http::StatusCode;
// use ormlite::FromRow;
// use ormlite::{model::ModelBuilder, Model};

use tokio::try_join;

// use uuid::Uuid;
// use sqlx::{Sqlite, SqlitePool};

// use tower_http::http::cors::CorsLayer;

// use tower_http::trace::TraceLayer;
// use tower::http

use crate::server::ServerApplication;
// mod answer;
// mod db;
mod server;
// mod survey;
use anyhow;
use db::db::Database;
use markdownparser;

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
    use std::collections::HashMap;

    use db::models::{AnswerDetails, AnswerType, CreateAnswersRequest, CreateAnswersResponse};
    use markdownparser::{markdown_to_form, markdown_to_form_wasm};
    use reqwest::StatusCode;

    use serial_test::serial;
    use tower::ServiceExt;

    use crate::ServerApplication;

    #[serial]
    #[tokio::test]
    async fn test_supabase() {
        let _url = "https://uhfivxaprdgdcahfqrzx.supabase.co";
    }

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
        let _app = ServerApplication::new(true).await;
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
        let _app = ServerApplication::new(true).await;
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

        let actualsurvey = markdown_to_form(results.survey.plaintext);

        answers.insert(
            actualsurvey.questions[0].id.clone(),
            AnswerDetails {
                r#type: AnswerType::String,
                values: actualsurvey.questions[0]
                    .options
                    .iter()
                    .map(|x| x.text.clone())
                    .collect(),
            },
        );
        let answers_request = db::models::CreateAnswersRequest {
            id: "test".to_string(),
            survey_id: listresults.surveys[0].id.clone(),
            start_time: "".to_string(),
            answers: answers,
            survey_version: actualsurvey.version.clone(),
        };

        let response = client
            .post(format!(
                "{client_url}/{}/answers",
                actualsurvey.questions[0].id.clone()
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
