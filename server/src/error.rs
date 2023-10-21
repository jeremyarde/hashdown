use axum::response::IntoResponse;
use axum::Json;
use axum::{http::StatusCode, response::Response};
use hyper::{Client, Method, Uri};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;
use uuid::Uuid;

use crate::mware::ctext::Ctext;
use crate::mware::log::log_request;

#[derive(Debug, Deserialize, strum_macros::AsRefStr, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerError {
    SurveyFail(String),
    BadRequest(String),
    Database(String),

    // Auth
    MissingCredentials,
    UserAlreadyExists,
    WrongCredentials,
    AuthPasswordsDoNotMatch,
    AuthFailNoTokenCookie,
    AuthTokenCreationFail,
    PasswordDoesNotMatch,
    AuthFailTokenNotVerified(String),
    AuthFailTokenDecodeIssue,
    AuthFailTokenExpired,
    UserEmailNotVerified(String),
}

pub async fn main_response_mapper(
    ctx: Option<Ctext>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // -- Get the eventual response error.
    let service_error = res.extensions().get::<ServerError>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // -- If client error, build the new reponse.
    let error_response =
        client_status_error
            .as_ref()
            .map(|(status_code, client_error, message)| {
                let client_error_body = json!({
                    "error": {
                        "type": client_error.as_ref(),
                        "req_uuid": uuid.to_string(),
                        "message": message,
                    }
                });

                info!("    ->> client_error_body: {client_error_body}");

                // Build the new response from the client_error_body
                (*status_code, Json(client_error_body)).into_response()
            });

    // Build and log the server log line.
    // let client_error = client_status_error.unzip().1;
    let client_error = match client_status_error {
        Some(x) => Some(x.1),
        None => None,
    };
    log_request(uuid, req_method, uri, ctx, service_error, client_error)
        .await
        .expect("Did not log request properly");

    info!("Mapped response, returning...");
    error_response.unwrap_or(res)
}

// So that errors get printed to the browser?
impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
    }
}

impl ServerError {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError, Option<&String>) {
        match self {
            ServerError::BadRequest(x) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
                Some(x),
            ),
            ServerError::AuthPasswordsDoNotMatch => {
                (StatusCode::UNAUTHORIZED, ClientError::LOGIN_FAIL, None)
            }
            ServerError::UserAlreadyExists => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ClientError::LOGIN_FAIL,
                None,
            ),
            ServerError::SurveyFail(x) => (
                StatusCode::BAD_REQUEST,
                ClientError::INVALID_PARAMS,
                Some(x),
            ),
            ServerError::Database(x) => (
                StatusCode::BAD_REQUEST,
                ClientError::INVALID_PARAMS,
                Some(x),
            ),
            ServerError::MissingCredentials => {
                (StatusCode::UNAUTHORIZED, ClientError::LOGIN_FAIL, None)
            }
            ServerError::WrongCredentials => {
                (StatusCode::UNAUTHORIZED, ClientError::LOGIN_FAIL, None)
            }
            ServerError::AuthFailNoTokenCookie => {
                (StatusCode::UNAUTHORIZED, ClientError::LOGIN_FAIL, None)
            }
            ServerError::AuthTokenCreationFail => {
                (StatusCode::UNAUTHORIZED, ClientError::LOGIN_FAIL, None)
            }
            ServerError::PasswordDoesNotMatch => {
                (StatusCode::UNAUTHORIZED, ClientError::LOGIN_FAIL, None)
            }
            ServerError::AuthFailTokenNotVerified(x) => {
                (StatusCode::UNAUTHORIZED, ClientError::LOGIN_FAIL, Some(x))
            }
            ServerError::AuthFailTokenDecodeIssue => {
                (StatusCode::UNAUTHORIZED, ClientError::LOGIN_FAIL, None)
            }
            ServerError::AuthFailTokenExpired => {
                (StatusCode::UNAUTHORIZED, ClientError::LOGIN_FAIL, None)
            }
            ServerError::UserEmailNotVerified(x) => {
                (StatusCode::UNAUTHORIZED, ClientError::LOGIN_FAIL, Some(x))
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, strum_macros::AsRefStr)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
