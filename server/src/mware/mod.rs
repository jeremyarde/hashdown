use axum::http::{self, HeaderMap, HeaderName, HeaderValue, Method, Request, Uri};
use axum::{
    extract::{self, State},
    http::StatusCode,
};
use axum::{response::Response, Json};
use serde_json::json;
use tracing::info;
use uuid::Uuid;

use crate::{mware::log::log_request, ServerError};

use self::ctext::Ctext;

pub mod ctext;
pub mod log;
// pub mod middleware_require_auth;
