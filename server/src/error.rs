// use std::sync::Arc;

use std::sync::Arc;

use axum::http::{Method, Uri};
use axum::response::IntoResponse;
use axum::{http::StatusCode, response::Response};
use axum::{BoxError, Json};
use serde::Serialize;
use serde_json::json;
use tracing::{debug, info};
use uuid::Uuid;

use crate::mware::ctext::SessionContext;
use crate::mware::log::log_request;

#[derive(Debug, strum_macros::AsRefStr, Serialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum ServerError {
    LoginFail,

    // Auth stuff
    AuthFailNoSession,
    AuthFailCtxNotInRequest,

    // Model/DB errors
    Database(String),
    RequestParams(String),

    // Third party errors
    Stripe(String),

    // Other errors
    ConfigError(String),
}

impl core::fmt::Display for ServerError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

// pub type Result<T> = core::result::Result<T, ServerError>;
impl std::error::Error for ServerError {}

pub async fn main_response_mapper(
    ctx: Option<SessionContext>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    info!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // -- Get the eventual response error.
    let service_error = res.extensions().get::<ServerError>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // -- If client error, build the new reponse.
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            info!("    ->> client_error_body: {client_error_body}");

            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    // Build and log the server log line.
    let client_error = client_status_error.unzip().1;
    // TODO: Need to hander if log_request fail (but should not fail request)
    let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;
    error_response.unwrap_or(res)
}

// So that errors get printed to the browser?
impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        // info!("->> {:<12} - {self:?}", "INTO_RES");
        debug!("{:<12} - model::Error {self:?}", "INTO_RESPONSE");

        let (status, error) = self.client_status_and_error();
        return (status, Json(json!({"error": error}))).into_response();

        // let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        // response.extensions_mut().insert(Arc::new(self));
    }
}

impl ServerError {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        debug!("client_status_and_error: {self:?}");

        // #[allow(unreachable_patterns)]
        match self {
            ServerError::LoginFail => (
                StatusCode::FORBIDDEN,
                ClientError::LOGIN_FAIL("Unable to log in".to_string()),
            ),

            ServerError::AuthFailNoSession | ServerError::AuthFailCtxNotInRequest => (
                StatusCode::FORBIDDEN,
                ClientError::NO_AUTH("Not able to find your account, please log in".to_string()),
            ),

            ServerError::Database { .. } => (
                StatusCode::BAD_REQUEST,
                ClientError::INVALID_PARAMS("Could not find the requested resource".to_string()),
            ),
            // -- Fallback.
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR(
                    "Failure to complete request. Please contact support".to_string(),
                ),
            ),
        }
    }
}
pub async fn handle_error(
    // // `Method` and `Uri` are extractors so they can be used here
    // method: Method,
    // uri: Uri,
    // // the last argument must be the error itself
    // err: BoxError,
    err: anyhow::Error,
) -> (StatusCode, String) {
    let (status, error) = err
        .downcast::<ServerError>()
        .unwrap()
        .client_status_and_error();
    return (status, Json(json!({"error": error})).to_string());
}

#[allow(non_camel_case_types)]
#[derive(Debug, strum_macros::AsRefStr, Serialize)]
pub enum ClientError {
    LOGIN_FAIL(String),
    NO_AUTH(String),
    INVALID_PARAMS(String),
    SERVICE_ERROR(String),
}
