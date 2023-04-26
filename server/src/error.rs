use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Server;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, strum_macros::AsRefStr, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerError {
    SurveyFail(String),
    BadRequest(String),
    Database(String),

    // Auth
    UserAlreadyExists,
    LoginFail,
    AuthPasswordsDoNotMatch,
    AuthFailNoTokenCookie,
    PasswordDoesNotMatch,
}

// So that errors get printed to the browser?
impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        // let (status, error_message) = match self {
        //     CustomError::Database(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
        //     CustomError::BadRequest(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
        //     CustomError::LoginFail => (StatusCode::OK, "logged in message I geuss".to_string()),
        //     CustomError::AuthFailNoTokenCookie => {
        //         (StatusCode::FORBIDDEN, "Authentication failed".to_string())
        //     }
        // };

        // format!("status = {}, message = {}", status, error_message).into_response()
        println!("->> {:<12} - {self:?}", "INTO_RES");
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        return response;
    }
}

impl ServerError {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            ServerError::BadRequest(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
            ServerError::AuthPasswordsDoNotMatch => {
                (StatusCode::UNAUTHORIZED, ClientError::LOGIN_FAIL)
            }
            ServerError::UserAlreadyExists => {
                (StatusCode::UNPROCESSABLE_ENTITY, ClientError::LOGIN_FAIL)
            }
            // CustomError::Database(_) => ,
            // CustomError::LoginFail => todo!(),
            // CustomError::AuthFailNoTokenCookie => todo!(),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
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
