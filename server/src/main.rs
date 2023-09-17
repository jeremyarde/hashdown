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

    let server_app = ServerApplication::new().await;
    try_join!(server_app.server).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{borrow::BorrowMut, collections::HashMap};

    use anyhow::{anyhow, Error};
    use axum::{
        body::Body,
        http::{HeaderMap, HeaderValue, Request},
        Json, Router,
    };
    use dotenvy::dotenv;
    use lettre::transport::smtp::client::{Tls, TlsParameters};
    use markdownparser::nanoid_gen;
    use mime::{Mime, APPLICATION_JSON};
    use reqwest::{header::CONTENT_TYPE, Client, StatusCode};

    use serde_json::{json, Value};
    use serial_test::serial;
    use tower::ServiceExt;
    use tracing::info;

    use crate::{
        db,
        mware::ctext::AUTH_TOKEN,
        routes::routes::{CreateSurveyRequest, ListSurveyResponse, LoginPayload},
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
    async fn test_setup_server() {
        setup_environment();
        let mut router = ServerApplication::get_router().await;
        router.ready().await.unwrap();

        let client_url = format!("http://localhost:3000{}", "/ping");
        println!("Client sending to: {client_url}");

        let request = Request::builder()
            .method("GET")
            .uri(client_url)
            // .header("x-auth-token", "mytoken")
            .body(Body::empty())
            // .body(Body::from(
            //     serde_json::to_vec(&json!([1, 2, 3, 4])).unwrap(),
            // ))
            .unwrap();

        let response = router.borrow_mut().oneshot(request).await.unwrap();

        dbg!(&response);
        assert!(response.status() != 500);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({ "result": "Ok" }));
    }

    #[serial]
    #[tokio::test]
    async fn test_create_survey() {
        setup_environment();
        let mut router = ServerApplication::get_router().await;
        router.ready().await.unwrap();

        let client_url = format!("http://localhost:3000{}", "/auth/login");
        println!("Client sending to: {client_url}");

        let token = signup_or_login(&mut router).await;
        println!("{}", token);

        // let request = LoginPayload {
        //     email: "jere".to_string(),
        //     password: "mypassword".to_string(),
        // };

        // let request: Request<Body> = Request::builder()
        //     .method("POST")
        //     .uri(client_url)
        //     // .header("x-auth-token", "mytoken")
        //     // .body(Body::empty())
        //     .header("content-type", "application/json")
        //     .body(Body::from(serde_json::to_vec(&json!(request)).unwrap()))
        //     .unwrap();

        // let response = router.borrow_mut().oneshot(request).await.unwrap();

        // dbg!(&response);
        // assert!(response.status() == 200);
        // let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        // let body: Value = serde_json::from_slice(&body).expect("Failed to deserialize messages");
        // assert!(body.get("auth_token").is_some());

        // List surveys
        let client_url = format!("http://localhost:3000{}", "/surveys");
        println!("Sending create survey with headers...");

        let create_request: Request<Body> = Request::builder()
            .method("POST")
            .uri(client_url)
            .header("x-auth-token", token.to_string())
            // .body(Body::empty())
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_vec(&json!({"plaintext": "- this is a survey"})).unwrap(),
            ))
            .unwrap();
        let response = router.borrow_mut().oneshot(create_request).await.unwrap();

        let list_response: Value =
            serde_json::from_slice(&hyper::body::to_bytes(response.into_body()).await.unwrap())
                .unwrap();

        dbg!(&list_response);
        assert!(list_response.is_object());
        assert!(list_response.get("error").is_none());
    }

    #[tokio::test]
    #[serial]
    async fn login_test() {
        setup_environment();

        let _app = ServerApplication::new().await;
        let mut router = ServerApplication::get_router().await;
        router.ready().await.unwrap();

        let url = "/auth/login";
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

        let results = response.json::<Value>().await.unwrap();
        dbg!(&results);
        assert!(results.get("auth_token").is_some())
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

        let url = "/auth/signup";
        let client_url = format!("http://{}{}", "localhost:3000", url);

        println!("Sending req to: {client_url}");

        let username = nanoid_gen(5);
        let request: LoginPayload = LoginPayload {
            email: username.clone(),
            password: "mypassword".to_string(),
        };

        let response = client
            .post(&client_url)
            .json(&request)
            .send()
            .await
            .expect("Should recieve repsonse from app");

        let results = response.json::<Value>().await.unwrap();
        assert_eq!(results.get("email").unwrap(), &username);
        assert!(results.get("auth_token").is_some());

        // attempt to login
        let url = "/auth/login";
        let client_url = format!("http://{}{}", "localhost:3000", url);

        println!("Sending req to: {client_url}");

        let request = LoginPayload {
            email: username,
            password: "failpassword".to_string(),
        };

        let response = client
            .post(&client_url)
            .json(&request)
            .send()
            .await
            .expect("Should recieve response from app");

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

    async fn signup_or_login(router: &mut Router) -> String {
        // Attempt signup
        let credentials_payload = LoginPayload {
            email: "jere".to_string(),
            password: "mypassword".to_string(),
        };

        let client_url = format!("http://localhost:3000{}", "/auth/signup");
        println!("Client sending to: {client_url}");
        let request: Request<Body> = Request::builder()
            .method("POST")
            .uri(client_url)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(Body::from(
                serde_json::to_vec(&json!(credentials_payload)).unwrap(),
            ))
            .unwrap();

        let response = router.borrow_mut().oneshot(request).await.unwrap();

        if response.status() == 200 {
            println!("Was able to signup, returning token");
            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            let body: Value = serde_json::from_slice(&body).unwrap();
            assert_eq!(body, json!({ "auth_token": "Ok" }));
            return body
                .get("auth_token")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
        }
        println!("Was NOT able to signup, attempting login...");

        let client_url = format!("http://localhost:3000{}", "/auth/login");
        println!("Client sending to: {client_url}");
        let request: Request<Body> = Request::builder()
            .method("POST")
            .uri(client_url)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(Body::from(
                serde_json::to_vec(&json!(credentials_payload)).unwrap(),
            ))
            .unwrap();

        let response = router.borrow_mut().oneshot(request).await.unwrap();
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        dbg!(&body);
        assert!(body.get("auth_token").is_some());
        return body
            .get("auth_token")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
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
