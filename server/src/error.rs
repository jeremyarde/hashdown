use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Server;
use serde::{Deserialize, Serialize};
use tracing::info;

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
}

// So that errors get printed to the browser?
impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        return response;
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
