use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum CustomError {
    BadRequest(String),
    Database(String),
    LoginFail,
}

// So that errors get printed to the browser?
impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            CustomError::Database(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            CustomError::BadRequest(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            CustomError::LoginFail => (StatusCode::OK, "logged in message I geuss".to_string()),
        };

        format!("status = {}, message = {}", status, error_message).into_response()
    }
}
