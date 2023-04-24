use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, State},
    http::Request,
    middleware::Next,
    response::Response,
    RequestPartsExt,
};
use tower_cookies::{Cookie, Cookies};
use tower_http::auth;

use crate::{ServerError, ServerState};

pub async fn mw_ctx_resolver<B>(
    _state: State<ServerState>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, ServerError> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    const AUTH_TOKEN: &str = "authtoken";
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // todo: actually validate the auth token
    let result_ctx = match auth_token
        .ok_or(ServerError::AuthFailNoTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id)) => Ok(Ctext::new(user_id)),
        Err(e) => Err(e),
    };
    // Remove the cookie if something went wrong other than NoAuthTokenCookie.
    if result_ctx.is_err() && !matches!(result_ctx, Err(ServerError::AuthFailNoTokenCookie)) {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctext {
    type Rejection = ServerError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, ServerError> {
        let cookies = parts.extract::<Cookies>().await.unwrap();
        let auth_token = cookies.get("authtoken").map(|c| c.value().to_string());
        let user_id = auth_token.ok_or(ServerError::AuthFailNoTokenCookie)?;

        Ok(Ctext::new(user_id))
    }
}

fn parse_token(token: String) -> Result<String, ServerError> {
    let user_id = token.split("user_id=").nth(0);
    match user_id {
        Some(x) => return Ok(x.to_string()),
        None => return Err(ServerError::AuthFailNoTokenCookie),
    }
}

#[derive(Clone, Debug)]
pub struct Ctext {
    user_id: String,
}

impl Ctext {
    pub fn user_id(&self) -> &String {
        &self.user_id
    }

    pub fn new(user_id: String) -> Self {
        return Ctext { user_id };
    }
}
