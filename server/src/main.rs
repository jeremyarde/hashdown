use axum::http::StatusCode;
use config::EnvConfig;
// use ormlite::FromRow;
// use ormlite::{model::ModelBuilder, Model};

use crate::mail::mail::Mailer;
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
mod config;
mod db;
mod log;
mod mware;
mod server;
// mod survey;
use anyhow;
use db::database::Database;

mod error;
mod mail;
mod routes;

pub use self::{error::ServerError, routes::*};

#[derive(Debug, Clone)]
pub struct ServerState {
    db: Database,
    mail: Mailer,
    config: EnvConfig,
}

#[tokio::main]
#[instrument]
async fn main() -> anyhow::Result<()> {
    // cargo watch -d 1.5 -- cargo run
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter("server=debug,sqlx=debug")
        .init();

    dotenvy::from_filename("./server/.env")?;

    info!("Spinning up the server.");
    let server_app = ServerApplication::new().await;
    info!("Server is running...");
    try_join!(server_app.server).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use axum::{
        http::{HeaderMap, HeaderValue},
        Json,
    };
    use db::models::{
        AnswerDetails, AnswerType, CreateAnswersRequest, CreateAnswersResponse, CreateSurveyRequest,
    };
    use dotenvy::dotenv;
    use lettre::transport::smtp::client::{Tls, TlsParameters};
    use markdownparser::ParsedSurvey;
    // use markdownparser::{markdown_to_form, markdown_to_form_wasm};
    use reqwest::{header::CONTENT_TYPE, Client, StatusCode};

    use serde_json::{json, Value};
    use serial_test::serial;
    use tower::ServiceExt;
    use tracing::info;

    use crate::{
        db,
        mware::ctext::AUTH_TOKEN,
        routes::routes::{ListSurveyResponse, LoginPayload},
        server::CreateSurveyResponse,
        ServerApplication,
    };

    fn setup_environment() {
        dotenvy::from_filename("./server/.env").unwrap();
    }

    async fn get_client() -> Client {
        let mut headers = HeaderMap::new();
        // headers.insert("x-", HeaderValue::from_static("testuser"));
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
        // dotenvy::from_filename("./server/.env").unwrap();
        setup_environment();

        let app = ServerApplication::new().await;
        let mut router = ServerApplication::get_router().await;
        router.ready().await.unwrap();

        let client = get_client().await;

        let auth_token = get_auth_token(&client).await;

        // let results = response.text().await;

        // List surveys
        let client_url = format!("http://{}{}", app.base_url.to_string(), "/surveys");

        println!("Client sending to: {client_url}");

        // TODO! Send issues to request for headers???
        let request = client
            .post(&client_url)
            .header("Cookie", format!("x-auth-token={auth_token}"))
            .json(&CreateSurveyRequest {
                plaintext: "- another\n - this one".to_string(),
            })
            .build()
            .unwrap();

        println!("Sending create survey with headers...");
        dbg!(&request);

        // println!("Sending request={request:#?}");

        let response = client.execute(request).await.unwrap();

        let results: CreateSurveyResponse = response.json().await.unwrap();

        assert_eq!(results.survey.survey.plaintext, "- another\n - this one");

        // call list
        let listresponse = client
            .get(&client_url)
            .header("Cookie", format!("x-auth-token={auth_token}"))
            .send()
            .await
            .unwrap();

        let listresults: ListSurveyResponse = listresponse
            .json()
            .await
            .expect("Could not turn response to json");

        assert_eq!(listresults.surveys.len(), 1);
        assert_eq!(listresults.surveys[0].plaintext, "- another\n - this one");
    }

    #[tokio::test]
    #[serial]
    async fn create_survey_test() {
        setup_environment();
        // dotenvy::from_filename("./server/.env").unwrap();

        let _app = ServerApplication::new().await;
        let mut router = ServerApplication::get_router().await;
        router.ready().await.unwrap();

        let client = reqwest::Client::builder()
            // .default_headers(headers)
            .build()
            .unwrap();

        let auth_token = get_auth_token(&client).await;

        let client_url = format!("http://{}{}", "localhost:3000", "/surveys");
        // let client_url = format!("/surveys");

        println!("Client sending to: {client_url}");

        let response = client
            .post(&client_url)
            .json(&CreateSurveyRequest {
                plaintext: "- create\n - this one".to_string(),
            })
            .header("Cookie", format!("x-auth-token={auth_token}"))
            .send()
            .await
            .unwrap();

        let results: CreateSurveyResponse = response.json().await.unwrap();

        println!("Results: {results:#?}");

        assert_eq!(results.survey.survey.plaintext, "- create\n - this one");
    }

    #[tokio::test]
    #[serial]
    async fn test_submit_answer() {
        setup_environment();
        let _app = ServerApplication::new().await;
        let mut router = ServerApplication::get_router().await;
        router.ready().await.unwrap();

        let client = get_client().await;
        let client_url = format!("http://localhost:3000/surveys/testsurveyid");

        println!("Client sending to: {client_url}");

        // let request = CreateAnswersRequest {
        //     // id: "1".to_string(),
        //     // id: None,
        //     // survey_id: "testsurveyid".to_string(),
        //     // survey_version: "0".to_string(),
        //     // start_time: "now()".to_string(),
        //     answers: json!([{"first": "answer"}]),
        // };
        // let exjson = json!([{"first": "answer"}]);
        // let request_test = "- test question\n - this one";
        let response = client
            .post(&client_url)
            // .json(&request)
            .json(&json!([{"first": "answer"}]))
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
        setup_environment();

        let _app = ServerApplication::new().await;
        let mut router = ServerApplication::get_router().await;
        router.ready().await.unwrap();

        let url = "/login";
        let client = get_client().await;
        let client_url = format!("http://{}{}", "localhost:3000", url);

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
        setup_environment();

        println!("=== Signup testing");
        let _app = ServerApplication::new().await;
        let mut router = ServerApplication::get_router().await;
        router.ready().await.unwrap();

        let client = get_client().await;

        let url = "/signup";
        let client_url = format!("http://{}{}", "localhost:3000", url);

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
        let client_url = format!("http://{}{}", "localhost:3000", url);

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
        dbg!(&response);
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

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
        dbg!(&response);
        let results = response.text().await;
        dbg!(results);
    }

    #[serial]
    #[tokio::test]
    async fn test_client_only() {
        setup_environment();

        let client = get_client().await;

        let client_url = format!("http://{}{}", "localhost:3000", "/surveys");

        println!("Client sending to: {client_url}");

        let auth_token = get_auth_token(&client).await;

        // TODO! Send issues to request for headers???
        let request = client
            .post(&client_url)
            // .headers(headers)
            // .header("x-user-id", "testuser")
            .json(&CreateSurveyRequest {
                plaintext: "- another\n - this one".to_string(),
            })
            .header("Cookie", format!("x-auth-token={auth_token}"))
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

    async fn get_auth_token(client: &Client) -> String {
        // Login
        let url = "/signup";
        let client_url = format!("http://{}{}", "localhost:3000", url);

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
            .expect("Should recieve response from app");

        println!("Response after logging in:");
        dbg!(&response);
        let auth_token = response
            .json::<Value>()
            .await
            .expect("Auth token to json broken.")
            .get("auth_token")
            .expect("Auth token was not found in response")
            .to_string();

        return auth_token;
    }

    // #[test]
    // fn test_email() {
    //     dotenvy::from_filename("./server/.env").unwrap();

    //     use lettre::message::header::ContentType;
    //     use lettre::transport::smtp::authentication::Credentials;
    //     use lettre::{Message, SmtpTransport, Transport};
    //     let from_email = "Test FROM <test@jeremyarde.com>";
    //     let to_email = "Test TO <test@jeremyarde.com>";
    //     let smtp_server = "email-smtp.us-east-1.amazonaws.com";

    //     let email = Message::builder()
    //         .from(from_email.parse().unwrap())
    //         // .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
    //         .to(to_email.parse().unwrap())
    //         .subject("Test email")
    //         .header(ContentType::TEXT_PLAIN)
    //         .body(String::from("Be happy!"))
    //         .unwrap();

    //     let creds = Credentials::new(
    //         dotenvy::var("SMTP_USERNAME").expect("smtp username should be set"),
    //         dotenvy::var("SMTP_PASSWORD").expect("smtp password should be set"),
    //     );

    //     // Open a remote connection to gmail
    //     let mailer = SmtpTransport::relay(smtp_server)
    //         .unwrap()
    //         .credentials(creds)
    //         // .tls(Tls::Wrapper(TlsParameters::builder(domain)))
    //         .build();

    //     // Send the email
    //     match mailer.send(&email) {
    //         Ok(_) => println!("Email sent successfully!"),
    //         Err(e) => panic!("Could not send email: {e:?}"),
    //     }
    // }
}
