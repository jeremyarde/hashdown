use std::ops::Add;

use argon2::{PasswordHash, PasswordHasher};

use axum::{
    extract::Query,
    http::{
        header::{self, SET_COOKIE},
        HeaderMap, HeaderValue,
    },
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
use chrono::{Duration, Utc};
use entity::users::{self};
use markdownparser::nanoid_gen;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, Set,
    TransactionTrait, TryIntoModel,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::types::time::OffsetDateTime;
use tracing::log::info;

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
) -> Result<(HeaderMap, Json<Value>), ServerError> {
    info!("->> confirm");

    /*
    steps to confirm:
    1. get confirm token from url
    2. check expiration of token, maybe 24h?
    3. mark email as verified
    */

    // state.db.pool.transaction(callback)
    let txn =
        state.db.pool.begin().await.map_err(|err| {
            ServerError::Database(format!("Could not start transaction: {}", err))
        })?;

    let mut user = state
        .db
        .get_user_by_confirmation_code(query.t.clone())
        .await?;

    match verify_confirmation_token(&query.t, &user) {
        true => {}
        false => return Err(ServerError::LoginFail),
    }

    user = state.db.verify_user(user).await?;

    // create session (header and payload)
    let session = match state
        .db
        .create_session(user)
        .await
        .map_err(|err| ServerError::Database("Could not create session".to_string()))
    {
        Ok(x) => x,
        Err(_) => return Err(ServerError::LoginFail),
    };

    txn.commit()
        .await
        .map_err(|err| ServerError::Database(format!("Could not start transaction: {}", err)))?;

    let mut headers = HeaderMap::new();
    headers.insert(
        "session_id",
        HeaderValue::from_str(session.0.session_id.as_str()).unwrap(),
    );

    Ok((headers, Json(json!({"user_id": session.0.user_id}))))
}

fn verify_confirmation_token(token: &String, user: &MdpUser) -> bool {
    if !user.inner().confirmation_token.clone().unwrap().eq(token) {
        info!("Confirmation token does not match");
        return false;
    }
    if user.inner().confirmation_token_expire_at.is_some()
        && user.inner().confirmation_token_expire_at.unwrap() > Utc::now()
    {
        info!("Confirmation token has not expired");
        return true;
    }
    false
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SignupPayload {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[axum::debug_handler]
pub async fn signup(
    state: State<ServerState>,
    payload: Json<SignupPayload>,
) -> Result<Json<Value>, ServerError> {
    info!("->> signup");

    match state.db.get_user_by_email(payload.email.clone()).await {
        Ok(user) => match user {
            None => {}
            Some(_) => {
                info!("Email already exists");
                return Err(ServerError::LoginFail);
            }
        },
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
        EmailIdentity::new(&user.inner().name, &payload.email),
        EmailIdentity::new("Hashdown - Email confirmation", LOGIN_EMAIL_SENDER),
        format!(
            "Welcome to hashdown!\n\n Please click on this link to confirm your email: {}/{}?t={}",
            state.config.frontend_url,
            "signup/confirm",
            user.inner().confirmation_token.clone().unwrap()
        )
        .as_str(),
        "Email confirmation",
    );

    // TODO: turn this section off, should get new session once they confirm email
    let email = user.inner().email.clone();
    // let session = state.db.create_session(user).await?;

    // let headers = create_session_headers(&session);

    Ok(
        // headers,
        // Json(json!({"email": email, "session_id": session.0.session_id.to_string()})),
        Json(json!({"email": email})),
    )
}

#[axum::debug_handler]
pub async fn logout(
    state: State<ServerState>,
    headers: HeaderMap,
) -> anyhow::Result<Json<Value>, ServerError> {
    info!("->> logout");
    let ctx = get_session_context(&state, headers).await?;

    state.db.delete_session(&ctx.session.0).await?;

    Ok(Json(json!("logout success")))
}
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
    let user = User::find()
        .filter(users::Column::Email.eq(payload.email.clone()))
        .one(&state.db.pool)
        .await
        .map_err(|err| ServerError::Database("Error".to_string()))?;

    let Some(user) = user else {
        return Err(ServerError::Database("User not found".to_string()));
    };

    // check if password matches
    let argon2 = argon2::Argon2::default();
    let current_password_hash = PasswordHash::new(&user.password_hash).unwrap();

    match argon2.verify_password(payload.password.as_bytes(), &current_password_hash) {
        Ok(_) => true,
        Err(_) => return Err(ServerError::LoginFail),
    };

    // TODO: create success body
    let username = payload.email.clone();
    let usermodel = MdpUser(user);

    // look for active session for userid
    let session = state.db.get_session_by_userid(usermodel.clone()).await?;
    if let Some(x) = session {
        info!("Found active session, deleting it");
        // delete old session
        x.0.delete(&state.db.pool)
            .await
            .map_err(|err| ServerError::Database(format!("Did not find session: {err}")))?;
    }

    let session = state.db.create_session(usermodel).await?;
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

            if x.is_empty() {
                return Err(ServerError::AuthFailNoSession);
            } else {
                x
            }
        }
        None => {
            info!("No session was found");
            return Err(ServerError::LoginFail);
        }
    };

    info!("Using session_id: {session_id:?}");

    // get session from database using existing Session
    let curr_session = state
        .db
        .get_session(session_id.to_string())
        .await
        .map_err(|err| ServerError::AuthFailNoSession)?;

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
            .update(&state.db.pool)
            .await
            .map_err(|err| ServerError::Database(format!("Error with db: {err}")))?;

        let sessionctx =
            SessionContext::new(updated_session.user_id.clone(), MdpSession(updated_session));
        request.extensions_mut().insert(sessionctx);
    } else {
        // remove this later
        info!("Session still active, not updating");
        let model = active_session.try_into_model().unwrap();
        let sessionctx = SessionContext::new(model.user_id.clone(), MdpSession(model));
        request.extensions_mut().insert(sessionctx);
        info!("Added ctext to request data");
    }

    Ok(next.run(request).await)
}

pub async fn get_session_context(
    state: &ServerState,
    // you can add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    // _jar: CookieJar,
    // mut request: Request,
    headers: HeaderMap,
) -> anyhow::Result<SessionContext, ServerError> {
    info!("--> get_session_context");

    let session_id = match headers
        .get(SESSION_ID_KEY)
        .and_then(|header| header.to_str().ok())
    {
        Some(x) => {
            info!("Session header: {x:?}");

            if x.is_empty() {
                return Err(ServerError::AuthFailNoSession);
            } else {
                x
            }
        }
        None => {
            info!("No session was found");
            return Err(ServerError::LoginFail);
        }
    };

    info!("Using session_id: {session_id:?}");

    // get session from database using existing Session
    let curr_session = state
        .db
        .get_session(session_id.to_string())
        .await
        .map_err(|err| ServerError::AuthFailNoSession)?;

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
            .update(&state.db.pool)
            .await
            .map_err(|err| ServerError::Database(format!("Error with db: {err}")))?;

        let sessionctx =
            SessionContext::new(updated_session.user_id.clone(), MdpSession(updated_session));
        // request.extensions_mut().insert(sessionctx);
        return Ok(sessionctx);
    } else {
        // remove this later
        info!("Session still active, not updating");
        let model = active_session.try_into_model().unwrap();
        let sessionctx = SessionContext::new(model.user_id.clone(), MdpSession(model));
        // request.extensions_mut().insert(sessionctx);
        info!("Added ctext to request data");
        return Ok(sessionctx);
    }
}
