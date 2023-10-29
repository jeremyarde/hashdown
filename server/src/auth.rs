use crate::constants::SESSION_ID_KEY;

use crate::mware::ctext::{create_jwt_token, Ctext};
use crate::routes::routes::LoginPayload;

use anyhow::Context;
use argon2::{PasswordHash, PasswordHasher};

// use axum::extract::TypedHeader;
// use axum::headers::authorization::{Authorization, Bearer};

use axum::headers::Server;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::{cookie, CookieJar};

use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use chrono::{Duration, Offset, Utc};
use hyper::header::SET_COOKIE;
use hyper::{HeaderMap, Request};

use markdownparser::nanoid_gen;
use serde_json::json;
// use sqlx::types::time::OffsetDateTime;

// use tower_sessions::cookie::Cookie;

use crate::db::database::{CreateUserRequest, Session};

use argon2::password_hash::rand_core::OsRng;

use argon2::password_hash::SaltString;

use tracing::log::info;

use crate::{db, ServerError};

use serde_json::Value;

use axum::{Extension, Json};

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
) -> impl IntoResponse {
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
    // let _password_hash_string = hash.to_string();

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

    let session = state.db.create_session(user.user_id.clone()).await?;

    // let _ = jar.add(Cookie::new("session_id", session.session_id.clone()));
    let headers = create_session_headers(&session);

    return Ok((headers, Json(json!({"email": user.email}))));
}

#[axum::debug_handler]
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

    state
        .db
        .delete_session(session_header.clone().to_string())
        .await?;
    let _ = &jar.remove(session_header);
    return Ok(());
}

#[axum::debug_handler]
pub async fn login(
    state: State<ServerState>,
    jar: CookieJar,
    // headers: HeaderMap,
    // ctext: Extension<Ctext>,
    // ctext: Ctext,
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
    let current_password_hash = PasswordHash::new(&user.password_hash).unwrap();

    match argon2.verify_password(payload.password.as_bytes(), &current_password_hash) {
        Ok(_) => true,
        Err(_) => return Err(ServerError::AuthPasswordsDoNotMatch),
    };

    match user.email_status {
        db::database::EmailStatus::Verified => {
            // state.mail.send(
            //     &state.mail.test_to,
            //     &state.mail.test_from,
            //     "Yo here is your magic link: {}".to_string(),
            // );
        }
        _ => {
            // lets ask user to verify, or send a magic link?
            // state.mail.send(
            //     &state.mail.test_to,
            //     &state.mail.test_from,
            //     generate_magic_link(&state, ctext.).await,
            // );

            // return Err(ServerError::UserEmailNotVerified(
            //     "Email not verified".to_string(),
            // ));
        }
    };

    // TODO: create success body
    let username = payload.email.clone();

    let session = state.db.create_session(user.user_id.clone()).await?;

    // let _ = jar.add(Cookie::new(SESSION_ID_KEY, session.session_id.clone()));

    let headers = create_session_headers(&session);

    // let offset =
    //     OffsetDateTime::from_unix_timestamp(session.active_period_expires_at.timestamp()).unwrap();

    let session_cookie = Cookie::build("session_id", session.session_id.clone())
        .http_only(true)
        // .expires(offset)
        .finish();
    let cookies = jar.add(session_cookie);

    return Ok((
        cookies,
        headers,
        Json(
            json!({"email": username, "auth_token": "not implemented", "session_id": session.session_id}),
        ),
    ));
}

async fn generate_magic_link(state: &ServerState, ctext: Ctext) -> String {
    let jwt = create_jwt_token(&ctext).expect("JWT was not created properly");

    // let session = state.db.create_session(user.user_id().to_string()).await;
    let magic_link = format!("http://localhost:5173/auth/verify?token={jwt}");

    return magic_link;
}

pub fn create_session_headers(session: &Session) -> HeaderMap {
    let mut headers = HeaderMap::new();
    // headers.insert(
    //     SET_COOKIE,
    //     HeaderValue::from_str(format!("{}={}", SESSION_ID_KEY, &session.session_id).as_str())
    //         .unwrap(),
    // );
    // let session_cookie = Cookie::build("session_id", session.session_id.clone())
    //     // .domain("http://localhost:3000")
    //     .path("/")
    //     .http_only(true)
    //     .secure(true)
    //     .finish();

    // let session_key = session_cookie.name().to_owned();
    // let session_value = session_cookie.to_string();

    headers.insert(
        SESSION_ID_KEY,
        HeaderValue::from_str(&session.session_id).unwrap(),
    );
    // headers.insert(
    //     SET_COOKIE,
    //     HeaderValue::from_str(&format!(
    //         "session_id={}; HttpOnly; Secure; Path=/",
    //         session.session_id
    //     ))
    //     .unwrap(),
    // );

    info!("Session_id: {headers:?}");

    return headers;
}

pub async fn validate_session_middleware<B>(
    State(state): State<ServerState>,
    // you can add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    jar: CookieJar,
    mut request: Request<B>,
    next: Next<B>,
) -> anyhow::Result<Response, ServerError> {
    info!("--> validate_session_middleware");

    // let session_cookie = jar.get(SESSION_ID_KEY);
    // info!("session cookie? {:?}", session_cookie);

    // let session_header = request
    //     .headers()
    //     .get(SESSION_ID_KEY)
    //     .and_then(|header| header.to_str().ok());

    // if session_header.is_some() {
    //     match state
    //         .db
    //         .get_session(session_header.unwrap().to_owned())
    //         .await
    //     {
    //         Ok(x) => {
    //             request.headers_mut().insert(
    //                 SESSION_ID_KEY,
    //                 HeaderValue::from_str(&x.session_id.clone())
    //                     .expect("Session Id is not available"),
    //             );
    //             request.extensions_mut().insert(x);
    //             info!(" --> validate_session_middleware - found active session");
    //             return Ok(next.run(request).await);
    //         }
    //         Err(_) => return Err(ServerError::AuthFailNoTokenCookie),
    //     }
    // }

    // other version
    info!("->> Validating session");

    let session_header = request
        .headers()
        .get(SESSION_ID_KEY)
        .and_then(|header| header.to_str().ok());

    let session_cookie = jar
        .get(SESSION_ID_KEY)
        .and_then(|cookie| Some(cookie.value()));

    info!("Session header: {session_header:?}");
    info!("Session cookies: {session_cookie:?}");

    // let session_id = match session_header {
    //     Some(x) => x.to_string(),
    //     None => session_cookie.unwrap_or({

    //     }),
    // };

    // request
    //     .extensions_mut()
    //     .insert(Ctext::new("fake".to_string(), Session::new()));
    // return Ok(next.run(request).await);

    let session_id = match session_header {
        Some(x) => x.to_string(),
        None => return Err(ServerError::AuthFailNoTokenCookie),
    };

    info!("Using session_id: {session_id:?}");

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

    info!("Current session: {:?}", curr_session);
    if Utc::now() > curr_session.active_period_expires_at {
        info!("session not active anymore?");
        let new_active_expires = Utc::now() + Duration::hours(1);
        let new_idle_expires = Utc::now() + Duration::hours(2);
        let updated_session = state
            .db
            .update_session(Session {
                id: 0,
                session_id: curr_session.session_id,
                active_period_expires_at: new_active_expires,
                idle_period_expires_at: new_idle_expires,
                user_id: curr_session.user_id,
            })
            .await?;
        info!("Putting Ctext into extensions");
        request.extensions_mut().insert(updated_session.clone());
        request.extensions_mut().insert(Ctext {
            user_id: updated_session.user_id.to_string(),
            session: updated_session,
        });
    } else {
        // remove this later
        info!("Session still active, not updating");
        // let new_active_expires = Utc::now() + Duration::hours(1);
        // let new_idle_expires = Utc::now() + Duration::hours(2);
        // let updated_session = state
        //     .db
        //     .update_session(Session {
        //         id: 0,
        //         session_id: curr_session.session_id,
        //         active_period_expires_at: new_active_expires,
        //         idle_period_expires_at: new_idle_expires,
        //         user_id: curr_session.user_id,
        //     })
        //     .await?;
        info!("Putting Ctext into extensions");
        // request.extensions_mut().insert(curr_session.clone());
        request.extensions_mut().insert(Ctext {
            user_id: curr_session.user_id.to_string(),
            session: curr_session,
        });
    }

    return Ok(next.run(request).await);
}
