use tracing::info;

use markdownparser::nanoid_gen;

use sqlx;

use chrono::Utc;

use chrono;

use crate::Database;

use crate::db::database::UserModel;

use anyhow;

use super::database::CreateUserRequest;

pub trait UserCrud {
    async fn create_user(&self, request: CreateUserRequest) -> anyhow::Result<UserModel>;
    async fn get_user_by_email(&self, email: String) -> anyhow::Result<UserModel>;
}

impl UserCrud for Database {
    async fn create_user(&self, request: CreateUserRequest) -> anyhow::Result<UserModel> {
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
            .await?;

        Ok(user)
    }

    async fn get_user_by_email(&self, email: String) -> anyhow::Result<UserModel> {
        info!("Search for user with email: {email:?}");

        let res: UserModel = sqlx::query_as(r#"select * from mdp.users where email = $1"#)
            .bind(email)
            .fetch_one(&self.pool)
            .await?;

        info!("Found user");
        // let result = UserModel::from_row(&row_result).expect("Could not turn row into user model");

        info!("Successfully found user");
        Ok(res)
    }
}
