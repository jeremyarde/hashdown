use axum::response::IntoResponse;
use http::StatusCode;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum CustomError {
    BadRequest(String),
    Database(String),
}

// So that errors get printed to the browser?
impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            CustomError::Database(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            CustomError::BadRequest(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
        };

        format!("status = {}, message = {}", status, error_message).into_response()
    }
}
