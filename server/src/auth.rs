use crate::mware::ctext::create_jwt_token;
use crate::routes::routes::LoginPayload;

use anyhow::Context;
use argon2::{PasswordHash, PasswordHasher};

use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
// use axum::extract::TypedHeader;
// use axum::headers::authorization::{Authorization, Bearer};
use axum::{http::StatusCode, response::Redirect};
use axum_extra::extract::cookie::CookieJar;

use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::Response;
use chrono::{DateTime, Duration, Utc};
use hyper::{HeaderMap, Request};
use markdownparser::nanoid_gen;
use serde_json::json;
use tower_sessions::cookie::Cookie;
use tower_sessions::session;

use crate::mware::ctext::create_jwt_claim;

use crate::db::database::{CreateUserRequest, Session};

use argon2::password_hash::rand_core::OsRng;

use argon2::password_hash::SaltString;

use argon2::Argon2;

use tracing::log::info;

use crate::ServerError;

use serde_json::Value;

use axum::{Json, TypedHeader};

use crate::ServerState;

use axum::extract::State;

use argon2::PasswordVerifier;

pub enum AuthError {
    PasswordDoNotMatch,
}

pub async fn validate_credentials(
    expected_password_hash: String,
    password_candidate: String,
) -> anyhow::Result<(), ServerError> {
    let expected_password_hash =
        PasswordHash::new(&expected_password_hash).expect("Should hash password properly");
    match Argon2::default().verify_password(password_candidate.as_bytes(), &expected_password_hash)
    {
        Ok(_x) => Ok(()),
        Err(_e) => Err(ServerError::PasswordDoesNotMatch),
    }
}

#[axum::debug_handler]
pub async fn signup(
    state: State<ServerState>,
    payload: Json<LoginPayload>,
    jar: CookieJar,
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

    let jwt_claim = create_jwt_claim(user.email.clone(), "somerole-pleasechange")?;

    let session = state.db.create_session(user.user_id);

    jar.add(Cookie::new("session_id", session_id));

    Ok(Json(
        json!({"email": user.email, "auth_token": jwt_claim.token}),
    ))
}

pub async fn validate_session(
    headers: HeaderMap,
    // session_id: String,
    // extract(session_token):
    state: State<ServerState>,
) -> anyhow::Result<Session, ServerError> {
    info!("->> Validating session");

    // let session_id = headers.get("session_token");
    let session_id = "this is a fake".to_string();

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
        return Ok(updated_session);
    }
    return Err(ServerError::AuthFailNoTokenCookie);
}

async fn create_session(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    jar: CookieJar,
) -> Result<(CookieJar, Redirect), StatusCode> {
    if let Some(session_id) = authorize_and_create_session(auth.token()).await {
        Ok((
            // the updated jar must be returned for the changes
            // to be included in the response
            jar.add(Cookie::new("session_id", session_id)),
            Redirect::to("/me"),
        ))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

// async fn me(jar: CookieJar) -> Result<(), StatusCode> {
//     if let Some(session_id) = jar.get("session_id") {
//         // fetch and render user...
//     } else {
//         Err(StatusCode::UNAUTHORIZED)
//     }
// }

async fn authorize_and_create_session(token: &str) -> Option<String> {
    // authorize the user and create a session...
    return Some("test".to_string());
}

pub async fn login(
    // cookies: Cookie,
    // cookies: axum_extra::extract::
    // ctx: Result<Ctext, CustomError>,
    state: State<ServerState>,
    payload: Json<LoginPayload>,
) -> anyhow::Result<Json<Value>, ServerError> {
    info!("->> login");
    info!("Payload: {payload:#?}");

    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(ServerError::MissingCredentials);
    }

    // look for email in database
    let user = match state
        .db
        .get_user_by_email(payload.email.clone())
        .await
        .with_context(|| "Could not get find user by email")
    {
        Ok(x) => x,
        Err(_) => {
            info!("Did not find user in database");
            return Err(ServerError::WrongCredentials);
        }
    };

    // check if password matches
    let argon2 = argon2::Argon2::default();

    let hash = PasswordHash::new(&user.password_hash).unwrap();
    let is_correct = match argon2.verify_password(payload.password.as_bytes(), &hash) {
        Ok(_) => true,
        Err(_) => return Err(ServerError::AuthPasswordsDoNotMatch),
    };
    println!("      ->> password matches={is_correct}");
    let jwt = create_jwt_token(user)?;

    // TODO: create success body
    let username = payload.email.clone();
    let logged_in = true;

    println!("     ->> Success logging in");

    Ok(Json(
        json!({"result": logged_in, "username": username, "auth_token": &jwt}),
    ))
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
    // do something with `request`...

    let session_header = request
        .headers()
        .get("session_token")
        .and_then(|header| header.to_str().ok());

    let session_header = if let Some(session_header) = session_header {
        Some(session_header)
    } else {
        // return Err(ServerError::MissingCredentials);
        None
    };

    if session_header.is_some() {
        match state
            .db
            .get_session(session_header.unwrap().to_owned())
            .await
        {
            Ok(x) => {
                request.headers_mut().insert(
                    "session_token",
                    HeaderValue::from_str(&x.session_id.clone())
                        .expect("Session Id is not available"),
                );
                request.extensions_mut().insert(x);
                return Ok(next.run(request).await);
            }
            Err(_) => return Err(ServerError::AuthFailNoTokenCookie),
        }
    }

    info!(" --> validate_session_middleware - no session available!");

    return Ok(next.run(request).await);
}
