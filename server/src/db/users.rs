use hyper::Server;
use tracing::info;

use markdownparser::nanoid_gen;

use sqlx;

use chrono::Utc;

use chrono;

use crate::{Database, ServerError};

use crate::db::database::UserModel;

use anyhow;

use super::database::CreateUserRequest;

pub trait UserCrud {
    async fn create_user(&self, request: CreateUserRequest) -> Result<UserModel, ServerError>;
    async fn get_user_by_email(&self, email: String) -> Result<UserModel, ServerError>;
}

impl UserCrud for Database {
    async fn create_user(&self, request: CreateUserRequest) -> Result<UserModel, ServerError> {
        println!("->> create_user");

        let _time = chrono::Utc::now();
        let user = sqlx::query_as::<_, UserModel>(
                "insert into mdp.users (user_id, password_hash, email, created_at, modified_at) values($1, $2, $3, $4, $5) returning *",
            )
            .bind(nanoid_gen(self.settings.nanoid_length.expect("Settings not set.")))
            .bind(request.password_hash)
            .bind(request.email)
            .bind(chrono::Utc::now())
            .bind(chrono::Utc::now())
            .fetch_one(&self.pool)
            .await.map_err(|err| ServerError::Database(format!("Could not create user: {err}")))?;

        Ok(user)
    }

    async fn get_user_by_email(&self, email: String) -> Result<UserModel, ServerError> {
        info!("Search for user with email: {email:?}");

        let res: UserModel = sqlx::query_as(r#"select * from mdp.users where email = $1"#)
            .bind(email)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| {
                ServerError::Database(format!("Could not find user with email: {err}"))
            })?;

        info!("Found user");
        info!("Successfully found user");
        Ok(res)
    }
}
