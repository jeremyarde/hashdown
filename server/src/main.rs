use crate::mail::mailer;
use config::EnvConfig;
use mail::mailer::Mailer;

use crate::server::ServerApplication;
use tokio::try_join;
use tracing::instrument;

// mod answer;
// mod db;
mod auth;
mod config;
mod constants;
mod db;
mod mail;
mod mware;
mod server;
// mod survey;

use db::database::Database;

mod error;
// mod mail;
mod routes;
mod survey_responses;

pub use self::error::ServerError;

#[derive(Debug, Clone)]
pub struct ServerState {
    db: Database,
    mail: Mailer,
    config: EnvConfig,
}

#[tokio::main]
#[instrument]
async fn main() -> anyhow::Result<()> {
    println!("Starting server...");
    // println!("Ending early :)");
    // return Ok(());
    // cargo watch -d 1.5 -- cargo run
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter("server=debug,sqlx=debug")
        .init();

    // println!("Loading environment variables from file");
    // dotenvy::dotenv()?;
    // dotenvy::from_filename("./server/.env")?;

    let server_app = ServerApplication::new().await;
    try_join!(server_app.server).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use axum::{body::Body, http::Request, Router};

    use markdownparser::nanoid_gen;

    use serde_json::{json, Value};

    use tower::ServiceExt;

    use crate::{constants::SESSION_ID_KEY, routes::LoginPayload, ServerApplication};

    fn setup_environment() {
        dotenvy::from_filename("./server/.env").unwrap();
    }

    // #[serial]
    #[tokio::test]
    async fn test_setup_server() {
        setup_environment();
        let mut router = ServerApplication::get_router().await;

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
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({ "result": "Ok" }));
    }

    // #[serial]
    #[tokio::test]
    async fn test_create_survey() {
        setup_environment();
        let mut router = ServerApplication::get_router().await;
        // router.ready().await.unwrap();

        let client_url = format!("http://localhost:3000{}", "/auth/login");
        println!("Client sending to: {client_url}");

        let token = signup_or_login(&mut router).await;
        println!("{}", token);

        // List surveys
        let client_url = format!("http://localhost:3000{}", "/surveys");
        println!("Sending create survey with headers...");

        let create_request: Request<Body> = Request::builder()
            .method("POST")
            .uri(client_url)
            .header(SESSION_ID_KEY, token.to_string())
            .header(
                axum::http::header::CONTENT_TYPE,
                mime::APPLICATION_JSON.to_string(),
            )
            .body(Body::from(
                serde_json::to_vec(&json!({"plaintext": "- this is a survey"})).unwrap(),
            ))
            .unwrap();
        let response = router.borrow_mut().oneshot(create_request).await.unwrap();

        assert_ne!(response.status(), 500);
        println!("{response:?}");
        let list_response: Value = serde_json::from_slice(
            &axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .expect("Should turn response into thing"),
        )
        .expect("Turn into serde value");

        dbg!(&list_response);
        assert!(list_response.is_object());
        assert!(list_response.get("error").is_none());
    }

    #[tokio::test]
    // #[serial]
    async fn test_login() {
        setup_environment();

        let _app = ServerApplication::new().await;
        let mut router = ServerApplication::get_router().await;
        // router.ready().await.unwrap();

        let url = "/auth/login";
        let client_url = format!("http://{}{}", "localhost:3000", url);

        println!("Sending req to: {client_url}");

        let request = LoginPayload {
            email: "test@test.com".to_string(),
            password: "a".to_string(),
        };
        let create_request: Request<Body> = Request::builder()
            .method("POST")
            .uri(client_url)
            // .header("x-auth-token", token.to_string())
            .header(
                axum::http::header::CONTENT_TYPE,
                mime::APPLICATION_JSON.to_string(),
            )
            .body(Body::from(serde_json::to_vec(&json!(request)).unwrap()))
            .unwrap();

        let response = router.borrow_mut().oneshot(create_request).await.unwrap();
        let session_value = response
            .headers()
            .get(SESSION_ID_KEY)
            .unwrap()
            .clone()
            .to_str()
            .unwrap()
            .to_string();

        let results: Value = serde_json::from_slice(
            &axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();

        dbg!(&results);

        assert!(!session_value.is_empty());
        assert!(session_value.len() > 10);

        assert!(results.get("auth_token").is_some())
    }

    #[tokio::test]
    // #[serial]
    async fn test_signup() {
        setup_environment();

        println!("=== Signup testing");
        let _app = ServerApplication::new().await;
        let mut router = ServerApplication::get_router().await;
        // router.ready().await.unwrap();

        // let client = get_client().await;

        let url = "/auth/signup";
        let client_url = format!("http://{}{}", "localhost:3000", url);

        println!("Sending req to: {client_url}");

        let username = nanoid_gen(5);
        let request: LoginPayload = LoginPayload {
            email: username.clone(),
            password: "mypassword".to_string(),
        };

        let create_request: Request<Body> = Request::builder()
            .method("POST")
            .uri(client_url)
            // .header("x-auth-token", token.to_string())
            .header(
                axum::http::header::CONTENT_TYPE,
                mime::APPLICATION_JSON.to_string(),
            )
            .body(Body::from(serde_json::to_vec(&json!(request)).unwrap()))
            .unwrap();

        let response = router.borrow_mut().oneshot(create_request).await.unwrap();
        let headers = response.headers().clone();

        let results: Value = serde_json::from_slice(
            &axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();

        assert_eq!(results.get("email").unwrap(), &username);
        assert!(headers.contains_key(SESSION_ID_KEY));
        // assert!(results.get("auth_token").is_some());
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
            let headers = response.headers().clone();
            let session_id = headers.get(SESSION_ID_KEY).unwrap();
            return session_id.to_str().unwrap().to_string();
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
        let headers = response.headers().clone();
        let session_id = headers.get(SESSION_ID_KEY).unwrap();

        return session_id.to_str().unwrap().to_string();
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
