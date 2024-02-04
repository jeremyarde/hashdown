use tracing::{debug, info};

use sqlx;

use chrono;

use crate::{Database, ServerError};

use crate::db::database::UserModel;

use super::database::CreateUserRequest;

pub trait UserCrud {
    async fn create_user(&self, request: CreateUserRequest) -> Result<UserModel, ServerError>;
    async fn get_user_by_email(&self, email: String) -> Result<UserModel, ServerError>;
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
                    confirmation_token
                ) values ($1, $2, $3, $4, $5, $6, $7) returning *",
        )
        .bind(new_user.user_id)
        .bind(new_user.password_hash)
        .bind(new_user.email)
        .bind(new_user.created_at)
        .bind(new_user.modified_at)
        .bind(new_user.workspace_id)
        .bind(new_user.confirmation_token)
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
        info!("Successfully found user");
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::db::{
        database::{CreateUserRequest, Database},
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
}
