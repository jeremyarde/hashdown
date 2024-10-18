use std::ops::Add;

use argon2::{PasswordHash, PasswordHasher};

use axum::{
    extract::Query,
    http::{header::SET_COOKIE, HeaderMap, HeaderValue},
};

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::PasswordVerifier;
use axum::Json;
use axum::{extract::State, response::IntoResponse};
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

use crate::mware::ctext::SessionContext;
use crate::routes::LoginPayload;
use crate::ServerError;
use crate::ServerState;
use crate::{
    constants::{LOGIN_EMAIL_SENDER, SESSION_ID_KEY},
    mail::mailer::EmailIdentity,
};
use crate::{
    db::database::{CreateUserRequest, MdpDatabase},
    stripe::create_customer,
};
use crate::{
    db::database::{MdpSession, MdpUser},
    stripe,
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
        .create_session(
            user.0.workspace_id.clone().as_str(),
            &user.0.user_id.clone().as_str(),
        )
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
    State(state): State<ServerState>,
    payload: Json<SignupPayload>,
) -> Result<Json<Value>, ServerError> {
    info!("->> signup");

    // todo: rate limit

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
    State(state): State<ServerState>,
    headers: HeaderMap,
) -> anyhow::Result<Json<Value>, ServerError> {
    info!("->> logout");
    let session_id = get_session_header(&headers).unwrap();

    state.db.delete_session(&session_id).await?;

    Ok(Json(json!("logout success")))
}
use entity::users::Entity as User;

#[axum::debug_handler]
pub async fn login(
    State(state): State<ServerState>,
    // headers: HeaderMap,
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
    let username = payload.email.clone();
    let userid = user.user_id.clone();
    let workspaceid = user.workspace_id.clone();
    if user.stripe_customer_id.is_none() {
        let stripe_customer =
            stripe::create_customer(username.as_ref(), payload.email.as_ref()).await?;

        let mut active_user = user.into_active_model();

        active_user.stripe_customer_id = Set(Some(
            stripe_customer
                .get("id")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        ));
        active_user.update(&state.db.pool).await.map_err(|err| {
            ServerError::Database(format!("Failed to update user with stripe_id: {}", err))
        })?;

        // user = active_user.try_into_model().unwrap();
    }

    // TODO: create success body
    // let usermodel = MdpUser(user);

    // look for active session for userid
    let session = state
        .db
        .get_session_by_userid(workspaceid.clone().as_str(), userid.clone().as_str())
        .await?;
    if let Some(x) = session {
        info!("Found active session, deleting it");
        // delete old session
        x.0.delete(&state.db.pool)
            .await
            .map_err(|err| ServerError::Database(format!("Did not find session: {err}")))?;
    }

    let session = state
        .db
        .create_session(workspaceid.clone().as_str(), userid.clone().as_str())
        .await?;
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

pub fn get_session_header(headers: &HeaderMap) -> Option<String> {
    headers
        .get(SESSION_ID_KEY)
        .map(|x| x.to_str().unwrap().to_string())
}
