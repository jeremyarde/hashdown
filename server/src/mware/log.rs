use axum::http::Method;
use axum::http::Uri;
use axum::Extension;
use serde::Serialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::{error::ClientError, mware::ctext::Ctext, ServerError};

#[skip_serializing_none]
#[derive(Serialize, Debug)]
struct RequestLogLine {
    uuid: String,
    timestamp: String,
    // User and context
    user_id: Option<String>,
    // Request
    req_path: String,
    req_method: String,
    // Errors
    client_error_type: Option<String>,
    error_data: Option<String>,
    error_type: Option<String>,
}

pub async fn log_request(
    uuid: Uuid,
    req_method: Method,
    uri: Uri,
    Extension(ctx): Extension<Option<Ctext>>,
    service_error: Option<&ServerError>,
    client_error: Option<ClientError>,
) -> anyhow::Result<()> {
    let timestamp = chrono::Utc::now().timestamp().to_string();

    let error_type = service_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(service_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take().to_string()));

    // let ctx = if ctx.is_none() {
    //     Ctext::new(None, None)
    // } else {
    //     ctx.unwrap()
    // };

    // Create the RequestLogLine
    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: timestamp.to_string(),

        req_path: uri.to_string(),
        req_method: req_method.to_string(),

        // user_id: ctx..map(|c| c.user_id().clone()),
        user_id: ctx.map(|x| x.user_id).or(None),

        client_error_type: client_error.map(|e| e.as_ref().to_string()),

        error_type,
        error_data,
    };

    println!("      ->> LOG: {:?}", log_line);

    Ok(())
}
