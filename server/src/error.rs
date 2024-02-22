use axum::http::{Method, Uri};
use axum::response::IntoResponse;
use axum::Json;
use axum::{http::StatusCode, response::Response};
use serde::Serialize;
use serde_json::json;
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
}

impl core::fmt::Display for ServerError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

pub type Result<T> = core::result::Result<T, ServerError>;
impl std::error::Error for ServerError {}

pub async fn main_response_mapper(
    ctx: Option<SessionContext>,
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
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            println!("    ->> client_error_body: {client_error_body}");

            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    // Build and log the server log line.
    let client_error = client_status_error.unzip().1;
    // TODO: Need to hander if log_request fail (but should not fail request)
    let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!();
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
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        // #[allow(unreachable_patterns)]
        match self {
            ServerError::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            ServerError::AuthFailNoSession | ServerError::AuthFailCtxNotInRequest => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }

            ServerError::Database { .. } => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),

            // -- Fallback.
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
