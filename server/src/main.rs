use axum::http::StatusCode;
// use ormlite::FromRow;
// use ormlite::{model::ModelBuilder, Model};

use dotenvy::dotenv;
use tokio::try_join;
use tracing::{instrument, log::info};

// use uuid::Uuid;
// use sqlx::{Sqlite, SqlitePool};

// use tower_http::http::cors::CorsLayer;

// use tower_http::trace::TraceLayer;
// use tower::http

use crate::server::ServerApplication;
// mod answer;
// mod db;
mod auth;
mod mware;
mod server;
// mod survey;
use anyhow;
use db::database::Database;

mod error;
mod routes;

pub use self::{error::ServerError, routes::*};

#[derive(Debug, Clone)]
pub struct ServerState {
    db: Database,
}

#[tokio::main]
#[instrument]
async fn main() -> anyhow::Result<()> {
    // cargo watch -d 1.5 -- cargo run

    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .init();
    // env::set_current_dir("./server").unwrap();
    info!("{:?}", std::env::current_dir());

    // dotenvy::from_filename().unwrap();
    // dotenvy::dotenv().unwrap();
    dotenvy::from_filename("./server/.env")?;
    // curl -X GET 127.0.0.1:3000/surveys
    // curl -X GET https://127.0.0.1:3000/surveys
    /*

    curl -X POST http://localhost:3000/surveys \
       -H 'Content-Type: application/json' \
       -d '{"id": "test", "plaintext": "content"}'
       */

    info!("Spinning up the server.");
    let server_app = ServerApplication::new().await;
    info!("Server is running...");
    try_join!(server_app.server).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use axum::http::{HeaderMap, HeaderValue};
    use db::models::{
        AnswerDetails, AnswerType, CreateAnswersRequest, CreateAnswersResponse, CreateSurveyRequest,
    };
    use dotenvy::dotenv;
    use markdownparser::ParsedSurvey;
    // use markdownparser::{markdown_to_form, markdown_to_form_wasm};
    use reqwest::{header::CONTENT_TYPE, Client, StatusCode};

    use serde_json::{json, Value};
    use serial_test::serial;
    use tower::ServiceExt;
    use tracing::info;

    use crate::{
        routes::routes::{ListSurveyResponse, LoginPayload},
        server::CreateSurveyResponse,
        ServerApplication,
    };

    async fn get_client() -> Client {
        let mut headers = HeaderMap::new();
        headers.insert("x-user-id", HeaderValue::from_static("testuser"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        return client;
    }

    #[serial]
    #[tokio::test]
    async fn list_survey_test() {
        let app = ServerApplication::new().await;
        let mut router = ServerApplication::get_router().await;
        router.ready().await.unwrap();

        // let mut test_headers = HeaderMap::new();
        // test_headers.insert("test", HeaderValue::from_str("yo").unwrap());

        let client = get_client().await;

        let client_url = format!("http://{}{}", app.base_url.to_string(), "/surveys");

        println!("Client sending to: {client_url}");

        // TODO! Send issues to request for headers???
        let request = client
            .post(&client_url)
            // .headers(headers)
            // .header("x-user-id", "testuser")
            .header("duringbuildnotworking", "custom")
            .json(&CreateSurveyRequest {
                plaintext: "- another\n - this one".to_string(),
            })
            .build()
            .unwrap();
        println!("Sending request={request:#?}");

        let response = client.execute(request).await.unwrap();

        let results: CreateSurveyResponse = response.json().await.unwrap();

        assert_eq!(results.survey.survey.plaintext, "- another\n - this one");

        //call list
        let listresponse = client.get(&client_url).send().await.unwrap();
        let listresults: ListSurveyResponse = listresponse.json().await.unwrap();

        assert_eq!(listresults.surveys.len(), 1);
        assert_eq!(listresults.surveys[0].plaintext, "- another\n - this one");
    }

    #[tokio::test]
    #[serial]
    async fn create_survey_test() {
        dotenvy::from_filename("./server/.env").unwrap();

        let _app = ServerApplication::new().await;
        let mut router = ServerApplication::get_router().await;
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

        println!("Results: {results:#?}");

        assert_eq!(results.survey.survey.plaintext, "- create\n - this one");
    }

    #[tokio::test]
    #[serial]
    async fn test_survey_test() {
        let _app = ServerApplication::new().await;
        let mut router = ServerApplication::get_router().await;
        router.ready().await.unwrap();

        let client = get_client().await;
        let client_url = format!("http://{}{}", "localhost:8080", "/surveys/test");

        println!("Client sending to: {client_url}");

        let request_test = "- test question\n - this one";
        let response = client
            .post(&client_url)
            .json(&CreateSurveyRequest {
                plaintext: request_test.to_string(),
            })
            .send()
            .await
            .unwrap();

        let results: ParsedSurvey = response.json().await.unwrap();

        assert_eq!(results.plaintext, request_test);
        assert_eq!(results.questions[0].value, "test question");
    }

    #[tokio::test]
    #[serial]
    async fn create_answer_test() {
        let _app = ServerApplication::new().await;
        let mut router = ServerApplication::get_router().await;
        router.ready().await.unwrap();

        let client = get_client().await;
        let client_url = format!("http://{}{}", "localhost:8080", "/submit");

        println!("Client sending to: {client_url}");

        let request = CreateAnswersRequest {
            id: "1".to_string(),
            survey_id: "testid".to_string(),
            survey_version: "0".to_string(),
            start_time: "now()".to_string(),
            answers: HashMap::new(),
        };
        let exjson = json!({"first": "answer"});
        // let request_test = "- test question\n - this one";
        let response = client
            .post(&client_url)
            // .json(&request)
            .json(&exjson)
            .send()
            .await
            .expect("Should recieve repsonse from app");

        // let json: Value = response.json().await.unwrap();
        println!("{:?}", response.text().await);
        //         .json::<Result<CreateAnswersResponse, CustomError>>()
        //         .await
        // );
        // println!("{:?}", response.json::<CreateAnswersResponse>().await);
        // let results = response.json().await.unwrap();
        // assert!(json.get("answer_id") != None);
        // assert!(json.get("answer_id").unwrap() != "");

        // assert!();
    }
    #[tokio::test]
    #[serial]
    async fn login_test() {
        let _app = ServerApplication::new().await;
        let mut router = ServerApplication::get_router().await;
        router.ready().await.unwrap();

        let url = "/login";
        let client = get_client().await;
        let client_url = format!("http://{}{}", "localhost:8080", url);

        println!("Sending req to: {client_url}");

        let request = LoginPayload {
            email: "jere".to_string(),
            password: "mypassword".to_string(),
        };
        // let exjson = json!({"first": "answer"});
        // let request_test = "- test question\n - this one";
        let response = client
            .post(&client_url)
            // .json(&request)
            .json(&request)
            .send()
            .await
            .expect("Should recieve repsonse from app");

        // dbg!(response.headers());

        let results = response.text().await;
        // let results = response.json::<Value>().await;
        dbg!(results);
        // assert_eq!(
        //     &response.json::<Value>().await.unwrap(),
        //     json!({"result": false}).get("result").unwrap()
        // )
    }

    #[tokio::test]
    #[serial]
    async fn signup_test() {
        println!("=== Signup testing");
        let _app = ServerApplication::new().await;
        let mut router = ServerApplication::get_router().await;
        router.ready().await.unwrap();

        let client = get_client().await;

        let url = "/signup";
        let client_url = format!("http://{}{}", "localhost:8080", url);

        println!("Sending req to: {client_url}");

        let request: LoginPayload = LoginPayload {
            email: "jere".to_string(),
            password: "mypassword".to_string(),
        };

        let response = client
            .post(&client_url)
            .json(&request)
            .send()
            .await
            .expect("Should recieve repsonse from app");

        let results = response.text().await;
        // let results = response.json::<Value>().await;
        dbg!(results);
        // assert_eq!(
        //     &response.json::<Value>().await.unwrap(),
        //     json!({"result": false}).get("result").unwrap()
        // )

        // attempt to login
        let url = "/login";
        let client_url = format!("http://{}{}", "localhost:8080", url);

        println!("Sending req to: {client_url}");

        let request = LoginPayload {
            email: "jere".to_string(),
            password: "failpassword".to_string(),
        };

        let response = client
            .post(&client_url)
            .json(&request)
            .send()
            .await
            .expect("Should recieve repsonse from app");

        println!("Headers after first login attempt:");
        dbg!(&response);

        let results = response.text().await;
        dbg!(results);

        println!("Sending req to: {client_url}");

        let request = LoginPayload {
            email: "jere".to_string(),
            password: "mypassword".to_string(),
        };

        let response = client
            .post(&client_url)
            .json(&request)
            .send()
            .await
            .expect("Should recieve repsonse from app");
        println!("Headers after second login attempt:");
        dbg!(response.headers());
        let results = response.text().await;
        dbg!(results);
    }
}
