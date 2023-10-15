use crate::mware::ctext::create_jwt_token;
use crate::routes::routes::LoginPayload;

use anyhow::Context;
use argon2::{PasswordHash, PasswordHasher};

use chrono::{DateTime, Utc};
use serde_json::json;
use tower_sessions::session;

use crate::mware::ctext::create_jwt_claim;

use crate::db::database::{CreateUserRequest, Session};

use argon2::password_hash::rand_core::OsRng;

use argon2::password_hash::SaltString;

use argon2::Argon2;

use tracing::log::info;

use crate::ServerError;

use serde_json::Value;

use axum::Json;

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
) -> anyhow::Result<Json<Value>, ServerError> {
    info!("->> signup");

    // match state
    //     .db
    //     .get_user_by_email(payload.email.clone())
    //     .await
    //     .with_context(|| "Checking if user already exists")
    // {
    //     Ok(_) => return Err(ServerError::UserAlreadyExists),
    //     Err(_) => {}
    // };

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

    Ok(Json(
        json!({"email": user.email, "auth_token": jwt_claim.token}),
    ))
}

async fn update_session(
    session_id: String,
    state: State<ServerState>,
) -> anyhow::Result<Session, ServerError> {
    // get session from database using existing Session
    let curr_session = state.db.get_session(session_id).await?;
    let new_session = state.db.update_session(curr_session).await?;

    Ok(new_session)
}

fn validate_session(
    session_id: String,
    state: State<ServerState>,
    payload: Json<LoginPayload>,
) -> anyhow::Result<Session, ServerError> {
    // get session from database using existing Session
    let curr_session = state.db.get_session(session_id);

    Ok(Session {
        session_id: "fake".to_string(),
        active_period_expires_at: Utc::now(),
        idle_period_expires_at: Utc::now(),
        user_id: todo!(),
    })
}

pub async fn authorize(
    // cookies: Cookies,
    // ctx: Result<Ctext, CustomError>,
    state: State<ServerState>,
    payload: Json<LoginPayload>,
) -> anyhow::Result<Json<Value>, ServerError> {
    info!("->> api_login");
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
