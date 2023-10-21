use crate::constants::SESSION_ID_KEY;

use crate::routes::routes::LoginPayload;

use anyhow::Context;
use argon2::{PasswordHash, PasswordHasher};



// use axum::extract::TypedHeader;
// use axum::headers::authorization::{Authorization, Bearer};

use axum_extra::extract::cookie::CookieJar;

use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use chrono::{Duration, Utc};
use hyper::{HeaderMap, Request};

use serde_json::json;
use tower_sessions::cookie::Cookie;




use crate::db::database::{CreateUserRequest, Session};

use argon2::password_hash::rand_core::OsRng;

use argon2::password_hash::SaltString;



use tracing::log::info;

use crate::ServerError;

use serde_json::Value;

use axum::{Json};

use crate::ServerState;

use axum::extract::State;

use argon2::PasswordVerifier;

pub enum AuthError {
    PasswordDoNotMatch,
}

#[axum::debug_handler]
pub async fn signup(
    state: State<ServerState>,
    _jar: CookieJar,
    payload: Json<LoginPayload>,
) -> anyhow::Result<Json<Value>, ServerError> {
    info!("->> signup");

    if let Ok(_) = state
        .db
        .get_user_by_email(payload.email.clone())
        .await
        .with_context(|| "Checking if user already exists")
    {
        return Err(ServerError::UserAlreadyExists);
    };

    let argon2 = argon2::Argon2::default();
    let salt = SaltString::generate(OsRng);
    let hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .unwrap();
    let _password_hash_string = hash.to_string();

    // let mut transactions = state.db.pool.begin().await.unwrap();

    let user = match state
        .db
        .create_user(CreateUserRequest {
            email: payload.email.clone(),
            password_hash: hash.to_string(),
        })
        .await
    {
        Ok(user) => user,
        Err(e) => {
            println!("Could not create user, error in database: {e}");
            return Err(ServerError::WrongCredentials);
        }
    };

    // Don't create a session for signing up - we need to verify email first
    // let transaction_result = transactions.commit().await;

    // let _ = jar.add(Cookie::new("session_id", session.session_id.clone()));

    Ok(Json(json!({"email": user.email})))
}

pub async fn validate_session(
    _headers: HeaderMap,
    // session_id: String,
    jar: CookieJar,
    // extract(session_id):
    state: State<ServerState>,
) -> anyhow::Result<Option<Session>, ServerError> {
    info!("->> Validating session");

    let session_header = if let Some(x) = jar.get(SESSION_ID_KEY) {
        x
    } else {
        return Ok(None);
    };

    let session_id = session_header.to_string();

    // get session from database using existing Session
    let curr_session = match state.db.get_session(session_id.clone()).await {
        Ok(x) => x,
        Err(_) => {
            state.db.delete_session(session_id).await?;
            return Err(ServerError::AuthFailTokenDecodeIssue);
        }
    };

    if Utc::now() > curr_session.idle_period_expires_at {
        return Err(ServerError::AuthFailTokenExpired);
    }

    if Utc::now() > curr_session.active_period_expires_at {
        let new_active_expires = Utc::now() + Duration::hours(1);
        let new_idle_expires = Utc::now() + Duration::hours(2);
        let updated_session = state
            .db
            .update_session(Session {
                session_id: curr_session.session_id,
                active_period_expires_at: new_active_expires,
                idle_period_expires_at: new_idle_expires,
                user_id: curr_session.user_id,
            })
            .await?;
        return Ok(Some(updated_session));
    }
    return Err(ServerError::AuthFailNoTokenCookie);
}

pub async fn logout(
    state: State<ServerState>,
    jar: CookieJar,
    _headers: HeaderMap,
    // payload: Json<LoginPayload>,
) -> impl IntoResponse {
    info!("->> logout");

    let session_header = if let Some(x) = jar.get(SESSION_ID_KEY) {
        x.to_owned()
    } else {
        return Err(ServerError::AuthFailNoTokenCookie);
    };

    state.db.delete_session(session_header.clone().to_string());
    &jar.remove(session_header);
    return Ok(());
}

pub async fn login(
    state: State<ServerState>,
    jar: CookieJar,
    _headers: HeaderMap,
    payload: Json<LoginPayload>,
) -> impl IntoResponse {
    info!("->> login");
    info!("Payload: {payload:#?}");

    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(ServerError::MissingCredentials);
    }

    // look for user in database
    let user = match state
        .db
        .get_user_by_email(payload.email.clone())
        .await
        .with_context(|| "Could not get find user by email")
    {
        Ok(x) => x,
        Err(_) => {
            info!("Did not find user in database");
            return Err(ServerError::UserDoesNotExist(
                "User does not exist".to_string(),
            ));
        }
    };

    // check if password matches
    let argon2 = argon2::Argon2::default();
    let hash = PasswordHash::new(&user.password_hash).unwrap();

    match argon2.verify_password(payload.password.as_bytes(), &hash) {
        Ok(_) => true,
        Err(_) => return Err(ServerError::AuthPasswordsDoNotMatch),
    };

    match user.verified {
        true => true,
        false => {
            return Err(ServerError::UserEmailNotVerified(
                "Email not verified".to_string(),
            ))
        }
    };

    // TODO: create success body
    let username = payload.email.clone();

    let session = state.db.create_session(user.user_id.clone()).await?;

    let _ = jar.add(Cookie::new(SESSION_ID_KEY, session.session_id.clone()));

    let mut headers = HeaderMap::new();
    headers.insert(
        SESSION_ID_KEY,
        HeaderValue::from_str(&session.session_id).unwrap(),
    );

    // Redirect::to("/me");

    return Ok((
        headers,
        Json(
            json!({"email": username, "auth_token": "not implemented", "session_id": session.session_id}),
        ),
    ));
}

pub async fn validate_session_middleware<B>(
    State(state): State<ServerState>,
    // you can add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    mut request: Request<B>,
    next: Next<B>,
) -> anyhow::Result<Response, ServerError> {
    info!("--> validate_session_middleware - THIS IS GOOD");

    let session_header = request
        .headers()
        .get("session_id")
        .and_then(|header| header.to_str().ok());

    if session_header.is_some() {
        match state
            .db
            .get_session(session_header.unwrap().to_owned())
            .await
        {
            Ok(x) => {
                request.headers_mut().insert(
                    SESSION_ID_KEY,
                    HeaderValue::from_str(&x.session_id.clone())
                        .expect("Session Id is not available"),
                );
                request.extensions_mut().insert(x);
                info!(" --> validate_session_middleware - found active session");
                return Ok(next.run(request).await);
            }
            Err(_) => return Err(ServerError::AuthFailNoTokenCookie),
        }
    }

    info!(" --> validate_session_middleware - no session found");

    return Ok(next.run(request).await);
}
