use std::ops::Add;

use argon2::{PasswordHash, PasswordHasher};

use axum::{
    extract::{Path, Query},
    http::{header::SET_COOKIE, HeaderMap, HeaderValue},
};

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::PasswordVerifier;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum::{Extension, Json};
use axum_extra::extract::cookie::Cookie;
use chrono::{format::OffsetFormat, offset, Days, Duration, FixedOffset, Utc};
use entity::{
    sessions::Column,
    users::{self, Model},
};
use markdownparser::nanoid_gen;
use sea_orm::{
    prelude::DateTimeUtc, ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    QueryFilter, Set,
};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::types::time::OffsetDateTime;
use tracing::{debug, log::info};

use crate::db::database::CreateUserRequest;
use crate::db::database::{MdpSession, MdpUser};
use crate::mware::ctext::SessionContext;
use crate::routes::LoginPayload;
use crate::ServerError;
use crate::ServerState;
use crate::{
    constants::{LOGIN_EMAIL_SENDER, SESSION_ID_KEY},
    mail::mailer::EmailIdentity,
};

#[derive(Debug, Deserialize)]
pub struct EmailConfirmationToken {
    t: String, // token
}

// async fn login_authorized(
//     Query(query): Query<AuthRequest>,

#[axum::debug_handler]
pub async fn confirm(
    State(state): State<ServerState>,
    Query(query): Query<EmailConfirmationToken>,
    // Extension(ctx): Extension<SessionContext>,
) -> Result<Json<Value>, ServerError> {
    info!("->> confirm");

    /*
    steps to confirm:
    1. get confirm token from url
    2. check expiration of token, maybe 24h?
    3. mark email as verified
    4.
    */

    // is this unneccesary? session probably already gets the user
    let mut user = state
        .db
        .get_user_by_confirmation_code(query.t.clone())
        .await?;

    match verify_confirmation_token(&query.t, &user) {
        true => {}
        false => return Err(ServerError::LoginFail),
    }

    user = state.db.verify_user(user).await?;

    return Ok(Json(json!({"user_id": user.0.user_id})));
}

fn verify_confirmation_token(token: &String, user: &MdpUser) -> bool {
    if !user.0.confirmation_token.clone().unwrap().eq(token) {
        info!("Confirmation token does not match");
        return false;
    }
    if user.0.confirmation_token_expire_at.is_some()
        && user.0.confirmation_token_expire_at.unwrap() > Utc::now()
    {
        info!("Confirmation token has not expired");
        return true;
    }
    return false;
}

#[axum::debug_handler]
pub async fn signup(
    state: State<ServerState>,
    payload: Json<LoginPayload>,
) -> Result<(HeaderMap, Json<Value>), ServerError> {
    info!("->> signup");

    match state.db.get_user_by_email(payload.email.clone()).await {
        Ok(user) => {
            match user {
                None => {}
                Some(_) => return Err(ServerError::LoginFail), // user already exists
            }
        }
        Err(x) => return Err(x),
    };

    let argon2 = argon2::Argon2::default();
    let salt = SaltString::generate(OsRng);
    let hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .unwrap();

    // let mut transactions = state.db.pool.begin().await.unwrap();
    // need to create a workspace if request does not have oneshot
    let workspace = state.db.create_workspace().await?;

    let user = state
        .db
        .create_user(CreateUserRequest {
            name: payload.name.clone(),
            email: payload.email.clone(),
            password_hash: hash.to_string(),
            workspace_id: Some(workspace.0.workspace_id),
        })
        .await?;

    // Don't create a session for signing up - we need to verify email first
    // let transaction_result = transactions.commit().await;

    info!("Sending confirmation email");
    state.mail.send(
        EmailIdentity::new(&user.0.name, &payload.email),
        EmailIdentity::new("Hashdown - Email confirmation", LOGIN_EMAIL_SENDER),
        format!(
            "Welcome to hashdown!\n\n Please click on this link to confirm your email: {}/{}?t={}",
            state.config.frontend_url,
            "signup/confirm",
            user.0.confirmation_token.clone().unwrap()
        )
        .as_str(),
        "Email confirmation",
    );

    // TODO: turn this section off, should get new session once they confirm email
    let email = user.0.email.clone();
    let session = state.db.create_session(user).await?;

    let headers = create_session_headers(&session);

    Ok((
        headers,
        Json(json!({"email": email, "session_id": session.0.session_id.to_string()})),
    ))
}

// #[axum::debug_handler]
// pub async fn delete(
//     state: State<ServerState>,
//     // jar: CookieJar,
//     // headers: HeaderMap,
//     // payload: Json<LoginPayload>,
//     Extension(ctx): Extension<Option<SessionContext>>,
// ) -> Result<Json<Value>, ServerError> {
//     info!("->> delete user");

//     let Some(ctx) = ctx else {
//         return Err(ServerError::AuthFailCtxNotInRequest);
//     };
//     // let session_header = if let Some(x) = headers.get(SESSION_ID_KEY) {
//     //     x.to_owned().to_str().unwrap().to_string()
//     // } else {
//     //     return Err(ServerError::AuthFailNoTokenCookie);
//     // };
//     // must be signed in to delete yourself
//     state
//         .db
//         .delete_session(&ctx.session.0.session_id, &ctx.session.0.workspace_id)
//         .await?;
//     state
//         .db
//         .delete_user(&ctx.session.0.session_id, &ctx.session.0.workspace_id)
//         .await?;
//     Ok(Json(json!("delete successful")))
// }

#[axum::debug_handler]
pub async fn logout(
    state: State<ServerState>,
    // headers: HeaderMap,
    Extension(ctx): Extension<SessionContext>,
) -> anyhow::Result<Json<Value>, ServerError> {
    info!("->> logout");

    state.db.delete_session(&ctx.session.0).await?;

    Ok(Json(json!("logout success")))
}
use entity::sessions::Entity as Session;
use entity::users::Entity as User;

#[axum::debug_handler]
pub async fn login(
    state: State<ServerState>,
    // _jar: CookieJar,
    // headers: HeaderMap,
    // ctext: Extension<Option<Ctext>>,
    // ctext: Ctext,
    payload: Json<LoginPayload>,
) -> impl IntoResponse {
    info!("->> login");
    info!("Payload: {payload:#?}");

    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(ServerError::RequestParams(
            "Missing credentials".to_string(),
        ));
    }

    // look for user in database
    let mut user = User::find()
        .filter(users::Column::Email.eq(payload.email.clone()))
        .one(&state.db.sea_pool)
        .await
        .map_err(|err| ServerError::Database("Error".to_string()))?;

    // user = if user.is_none() {
    //     return Err(ServerError::Database("User not found".to_string()));
    // } else {
    //     return user.unwrap();
    // };
    let Some(user) = user else {
        return Err(ServerError::Database("User not found".to_string()));
    };
    // .unwrap_or(return Err(ServerError::Database("Could not find user".to_string())));

    // let user = state.db.get_user_by_email(payload.email.clone()).await?;

    // check if password matches
    let argon2 = argon2::Argon2::default();
    let current_password_hash = PasswordHash::new(&user.password_hash).unwrap();

    match argon2.verify_password(payload.password.as_bytes(), &current_password_hash) {
        Ok(_) => true,
        Err(_) => return Err(ServerError::LoginFail),
    };

    // TODO: create success body
    let username = payload.email.clone();
    let session = state.db.create_session(MdpUser(user)).await?;

    let headers = create_session_headers(&session);

    Ok((
        // cookies,
        headers,
        Json(json!({"email": username, "session_id": session.0.session_id.to_string()})),
    ))
}

async fn generate_magic_link(_state: &ServerState, _ctext: SessionContext) -> String {
    // let jwt = create_jwt_token(&ctext).expect("JWT was not created properly");
    let token = nanoid_gen(16);

    // let session = state.db.create_session(user.user_id().to_string()).await;
    let magic_link = format!("http://localhost:5173/auth/verify?token={token}");

    magic_link
}

pub fn create_session_headers(session: &MdpSession) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let session_cookie = Cookie::build("session_id", session.0.session_id.clone())
        // .domain("http://localhost:8080")
        .path("/")
        .http_only(true)
        .secure(true)
        .expires(OffsetDateTime::now_utc().add(Duration::days(1).to_std().unwrap()))
        .finish();

    headers.insert(
        SESSION_ID_KEY,
        HeaderValue::from_str(&session.0.session_id).unwrap(),
    );
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&session_cookie.to_string()).unwrap(),
    );

    info!("Session_id: {headers:?}");

    headers
}

pub async fn validate_session_middleware(
    State(state): State<ServerState>,
    // you can add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    // _jar: CookieJar,
    mut request: Request,
    next: Next,
) -> anyhow::Result<Response, ServerError> {
    info!("--> validate_session_middleware");

    // other version
    info!("->> Validating session");

    let session_id = match request
        .headers()
        .get(SESSION_ID_KEY)
        .and_then(|header| header.to_str().ok())
    {
        Some(x) => {
            info!("Session header: {x:?}");

            if x.eq("") {
                x
            } else {
                return Err(ServerError::AuthFailNoSession);
            }
        }
        None => {
            info!("No session was found");
            return Err(ServerError::LoginFail);
        }
    };

    info!("Using session_id: {session_id:?}");

    // get session from database using existing Session
    let curr_session = state.db.get_session(session_id.to_string()).await?;
    let mut active_session = curr_session.0.into_active_model();
    if &Utc::now().fixed_offset() > active_session.idle_period_expires_at.as_ref() {
        return Err(ServerError::LoginFail);
    }

    info!("Current session: {:?}", active_session);
    if &Utc::now().fixed_offset() > active_session.active_period_expires_at.as_ref() {
        info!("session not active anymore?");

        let new_active_expires = Utc::now().fixed_offset() + Duration::days(1);
        let new_idle_expires = Utc::now().fixed_offset() + Duration::days(2);

        active_session.active_period_expires_at = Set(new_active_expires);
        active_session.idle_period_expires_at = Set(new_idle_expires);

        let updated_session = active_session
            .update(&state.db.sea_pool)
            .await
            .map_err(|err| ServerError::Database(format!("Error with db: {err}")))?;
        // let updated_session = state
        //     .db
        //     .update_session(Session {
        //         id: 0,
        //         session_id: curr_session.session_id,
        //         active_period_expires_at: new_active_expires,
        //         idle_period_expires_at: new_idle_expires,
        //         user_id: curr_session.user_id,
        //         workspace_id: curr_session.workspace_id,
        //     })
        //     .await?;
        // request.extensions_mut().insert(updated_session.clone());
        // request.extensions_mut().insert(SessionContext {
        //     user_id: updated_session.user_id.to_string(),
        //     session: updated_session,
        // });
        request.extensions_mut().insert(updated_session);
    } else {
        // remove this later
        info!("Session still active, not updating");
        request.extensions_mut().insert(active_session);
        info!("Added ctext to request data");
    }

    Ok(next.run(request).await)
}
