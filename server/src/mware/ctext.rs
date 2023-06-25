use anyhow::Context;
use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, State},
    http::{HeaderMap, Request},
    middleware::Next,
    response::Response,
    RequestPartsExt,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies};
use tower_http::auth;
use tracing::log::info;

use crate::{ServerError, ServerState};

pub async fn mw_ctx_resolver<B>(
    _state: State<ServerState>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, ServerError> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    const AUTH_TOKEN: &str = "x-auth-token";
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // todo: actually validate the auth token
    let result_ctx = match auth_token
        .ok_or(ServerError::AuthFailNoTokenCookie)
        .and_then(parse_token)
    {
        Ok(token) => validate_jwt_claim(token),
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

    // #[tracing::instrument]
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, ServerError> {
        info!("->> from_request_parts");

        let cookies = match parts.extract::<Cookies>().await {
            Ok(x) => {
                info!("found cookies");
                let token = x.get("x-auth-token").map(|c| c.value().to_string());
                token
            }
            Err((status, err)) => {
                info!(
                    "No cookies found. Looking in headers now. Status: {status:?}, Error: {err:?}"
                );
                None
            }
        };

        info!("After cookies");
        let auth_token = match cookies {
            Some(x) => x,
            None => {
                let headers = parts
                    .extract::<HeaderMap>()
                    .await
                    .expect("Could not extract headers");

                match headers.get("x-auth-token") {
                    Some(x) => x.to_str().expect("Auth token was not a string").to_string(),
                    None => "".to_string(),
                }
            }
        };

        if auth_token == "".to_string() {
            return Err(ServerError::AuthFailNoTokenCookie);
        }

        // let auth_token = cookies.get("x-auth-token").map(|c| c.value().to_string());
        // let jwt = auth_token.ok_or(ServerError::AuthFailNoTokenCookie)?;
        let jwt_claim = validate_jwt_claim(auth_token)?;

        Ok(Ctext::new(jwt_claim.uid))
    }
}

#[tracing::instrument]
fn parse_token(token: String) -> Result<String, ServerError> {
    info!("->> parse_token");
    info!("parsing auth token");
    let user_id = token.split("user_id=").nth(0);
    match user_id {
        Some(x) => return Ok(x.to_string()),
        None => return Err(ServerError::AuthFailNoTokenCookie),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // subject
    exp: usize,  // expire
    iat: usize,  // issued at
    // iss: String, // issuer
    // aud: String, // audience
    uid: String,  // customer - user_id
    role: String, // custom - role of the user
    nbf: usize,   // not before
    tenant: String,
}

#[derive(Serialize)]
pub struct JwtResult {
    pub token: String,
    pub expires: usize,
}

#[tracing::instrument]
fn validate_jwt_claim(
    // payload: &Json<LoginPayload>,
    // key: &[u8; 10],
    jwt_token: String,
) -> anyhow::Result<Claims, ServerError> {
    info!("->> validate_jwt_claim");

    let key = b"privatekey";
    let decode_key = DecodingKey::from_secret(key);
    let decode_result =
        match decode::<Claims>(&jwt_token, &decode_key, &Validation::new(Algorithm::HS256)) {
            Ok(x) => x.claims,
            Err(e) => return Err(ServerError::AuthFailTokenNotVerified(e.to_string())),
        };
    return Ok(decode_result);
}

#[tracing::instrument]
pub fn create_jwt_claim(
    // payload: &Json<LoginPayload>,
    // key: &[u8; 10],
    user_id: String,
    role: &str, // jwt_token: String,
) -> anyhow::Result<JwtResult, ServerError> {
    info!("->> create_jwt_claim");
    let key = b"privatekey";
    // let decode_key = DecodingKey::from_secret(key);
    // let decode_result =
    //     match decode::<Claims>(&jwt_token, &decode_key, &Validation::new(Algorithm::HS256)) {
    //         Ok(x) => x.claims,
    //         Err(e) => return Err(ServerError::AuthFailTokenNotVerified(e.to_string())),
    //     };

    let nowutc = chrono::Utc::now();
    let now: usize = match nowutc
        .timestamp()
        .try_into()
        .with_context(|| "Could not turn time into timestamp")
    {
        Ok(x) => x,
        Err(e) => return Err(ServerError::LoginFail),
    };
    let expire: usize = match (nowutc + chrono::Duration::minutes(5))
        .timestamp()
        .try_into()
    {
        Ok(x) => x,
        Err(e) => return Err(ServerError::LoginFail),
    };

    let claim = Claims {
        sub: "myemailsub@email.com".to_string(),
        exp: expire,
        iat: now,
        uid: user_id.to_string(),
        nbf: now,
        tenant: "tenant".to_string(),
        role: role.to_string(),
    };

    // return Ok(claim);
    let jwt = match encode(&Header::default(), &claim, &EncodingKey::from_secret(key)) {
        Ok(t) => t,
        Err(_) => {
            return Err(ServerError::BadRequest(
                "yo this request is messed".to_string(),
            ))
        }
    };

    return Ok(JwtResult {
        token: jwt,
        expires: expire,
    });
}

#[derive(Clone, Debug)]
pub struct Ctext {
    user_id: String,
    // parse cookies in here?
}

impl Ctext {
    pub fn user_id(&self) -> &String {
        &self.user_id
    }

    pub fn new(user_id: String) -> Self {
        return Ctext { user_id };
    }
}
