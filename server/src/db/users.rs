use tracing::{debug, info};

use sqlx;

use chrono::{self, Utc};

use crate::{Database, ServerError};

use crate::db::database::UserModel;

use super::database::CreateUserRequest;

pub trait UserCrud {
    async fn create_user(&self, request: CreateUserRequest) -> Result<UserModel, ServerError>;
    async fn get_user_by_email(&self, email: String) -> Result<UserModel, ServerError>;
    async fn get_user_by_confirmation_code(
        &self,
        confirmation_token: String,
    ) -> Result<UserModel, ServerError>;
    async fn verify_user(&self, new_user: UserModel) -> Result<UserModel, ServerError>;
}

impl UserCrud for Database {
    async fn create_user(&self, request: CreateUserRequest) -> Result<UserModel, ServerError> {
        println!("->> create_user");
        let new_user = UserModel::from(request.email, request.password_hash, request.workspace_id);

        debug!("debug: {:?}", new_user);
        let _time = chrono::Utc::now();
        let user = sqlx::query_as::<_, UserModel>(
            "insert into mdp.users (
                    user_id, 
                    password_hash, 
                    email, 
                    created_at, 
                    modified_at,
                    workspace_id,
                    confirmation_token,
                    confirmation_token_expire_at
                ) values ($1, $2, $3, $4, $5, $6, $7, $8) returning *",
        )
        .bind(new_user.user_id)
        .bind(new_user.password_hash)
        .bind(new_user.email)
        .bind(new_user.created_at)
        .bind(new_user.modified_at)
        .bind(new_user.workspace_id)
        .bind(new_user.confirmation_token)
        .bind(new_user.confirmation_token_expire_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| ServerError::Database(format!("Could not create user: {err}")))?;

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
        Ok(res)
    }

    async fn get_user_by_confirmation_code(
        &self,
        confirmation_token: String,
    ) -> Result<UserModel, ServerError> {
        info!("Search for user with email confirm code: {confirmation_token:?}");

        let res: UserModel =
            sqlx::query_as(r#"select * from mdp.users where confirmation_token = $1"#)
                .bind(confirmation_token)
                .fetch_one(&self.pool)
                .await
                .map_err(|err| {
                    ServerError::Database(format!(
                        "Could not find user with confirmation_token: {err}"
                    ))
                })?;

        info!("Found user");
        Ok(res)
    }

    async fn verify_user(&self, user: UserModel) -> Result<UserModel, ServerError> {
        let user: UserModel = sqlx::query_as(
            "update mdp.users set 
            email_status = $1,
            confirmation_token = NULL,
            confirmation_token_expire_at = NULL,
            modified_at = $2,
            email_confirmed_at = $3
            
            where mdp.users.user_id = $4 and mdp.users.workspace_id = $5;
            ",
        )
        .bind("verified") //email status
        .bind(Utc::now()) //mod
        .bind(Utc::now()) //confirmed
        .bind(&user.user_id)
        .bind(&user.workspace_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| ServerError::Database(format!("Could not update User: {err:?}")))?;

        return Ok(user);
    }
}

#[cfg(test)]
mod tests {
    use crate::db::{
        database::{CreateUserRequest, Database, UserModel},
        users::UserCrud,
    };

    #[tokio::test]
    async fn test_create_user() {
        let create_user_request = CreateUserRequest {
            email: "test@jeremyarde.com".to_string(),
            password_hash: "fakepwhash".to_string(),
            workspace_id: Some("ws_test".to_string()),
        };

        let db = Database::new().await.unwrap();
        let user = db.create_user(create_user_request).await.unwrap();

        println!("success: {user:?}")
    }

    #[tokio::test]
    async fn test_verify_user() {
        let create_user_request = CreateUserRequest {
            email: "test@jeremyarde.com".to_string(),
            password_hash: "fakepwhash".to_string(),
            workspace_id: Some("ws_test".to_string()),
        };

        // let user = UserModel {
        //     id: todo!(),
        //     email: todo!(),
        //     password_hash: todo!(),
        //     created_at: todo!(),
        //     modified_at: todo!(),
        //     deleted_at: todo!(),
        //     email_status: todo!(),
        //     user_id: todo!(),
        //     workspace_id: todo!(),
        //     email_confirmed_at: todo!(),
        //     confirmation_token: todo!(),
        //     confirmation_token_expire_at: todo!(),
        //     role: todo!(),
        // };

        let db = Database::new().await.unwrap();
        let user = db.create_user(create_user_request).await.unwrap();

        let updated_user = db.verify_user(user.clone()).await.unwrap();

        println!("User: {user:#?}");
        println!("Updated User: {updated_user:#?}");
    }
}
