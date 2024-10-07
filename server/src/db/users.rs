use markdownparser::NanoId;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    QueryFilter, Set,
};
use tracing::{debug, info};

use chrono::{self, Utc};

// use crate::db::stripe;
use crate::{stripe, MdpDatabase, ServerError};

use crate::db::database::MdpUser;

use super::database::CreateUserRequest;

use entity::users::{self, Entity as User};

// trait UsersTrait {
//     fn new(req: CreateUserRequest) -> users::ActiveModel;
// }

// impl UsersTrait for users::ActiveModel {
//     fn new(req: CreateUserRequest) -> users::ActiveModel {
//         return users::ActiveModel {
//             name: todo!(),
//             email: todo!(),
//             password_hash: todo!(),
//             created_at: todo!(),
//             modified_at: todo!(),
//             deleted_at: todo!(),
//             email_status: todo!(),
//             user_id: todo!(),
//             workspace_id: todo!(),
//             email_confirmed_at: todo!(),
//             confirmation_token: todo!(),
//             confirmation_token_expire_at: todo!(),
//             role: todo!(),
//             stripe_customer_id: todo!(),
//             stripe_subscription_id: todo!(),
//             stripe_subscription_modified_at: todo!(),
//         };
//     }
// }

use entity::workspaces::{self};
impl MdpDatabase {
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<MdpUser, ServerError> {
        info!("->> create_user");

        let ws_id: String;
        if request.workspace_id.is_none() {
            let new_workspace = workspaces::ActiveModel {
                workspace_id: Set(NanoId::from("ws").to_string()),
                name: Set(String::from("default")),
            }
            .insert(&self.pool)
            .await
            .map_err(|err| ServerError::Database("Failed".to_string()))?
            .workspace_id;

            ws_id = new_workspace.clone();
        } else {
            ws_id = request.workspace_id.unwrap();
        }

        let new_user = MdpUser::from(
            &request.name,
            &request.email,
            &request.password_hash,
            ws_id.as_str(),
        );
        let user = new_user
            .0
            .clone()
            .into_active_model()
            .insert(&self.pool)
            .await
            .map_err(|err| ServerError::Database(format!("Could not create user: {err}")))?;

        let stripe_customer =
            stripe::create_customer(user.name.as_ref(), user.email.as_ref()).await?;

        let mut active_user = user.into_active_model();

        active_user.stripe_customer_id = Set(Some(
            stripe_customer
                .get("id")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        ));
        active_user.update(&self.pool).await.map_err(|err| {
            ServerError::Database(format!("Failed to update user with stripe_id: {}", err))
        })?;

        info!("Created new user: {:?}", new_user.inner().email);

        Ok(new_user)
    }

    pub async fn get_user_by_email(&self, email: String) -> Result<Option<MdpUser>, ServerError> {
        info!("Search for user with email: {email:?}");

        let user = User::find()
            .filter(users::Column::Email.eq(email.clone()))
            .one(&self.pool)
            .await
            .map_err(|err| ServerError::Database(format!("Error in database: {err}")))?;

        debug!("user: {:#?}", user);

        match user {
            Some(x) => Ok(Some(MdpUser(x))),
            None => Ok(None),
        }
    }

    pub async fn get_user_by_confirmation_code(
        &self,
        confirmation_token: String,
    ) -> Result<MdpUser, ServerError> {
        info!("Search for user with email confirm code: {confirmation_token:?}");

        let user = User::find()
            .filter(users::Column::ConfirmationToken.eq(confirmation_token.clone()))
            .one(&self.pool)
            .await
            .map_err(|err| {
                ServerError::Database(format!("Could not find user with email. Error: {err}"))
            })?;

        if let Some(x) = user {
            Ok(MdpUser(x))
        } else {
            Err(ServerError::Database("Could not find user".to_string()))
        }
        // info!("Found user");
        // Ok(res)
    }

    pub async fn verify_user(&self, user: MdpUser) -> Result<MdpUser, ServerError> {
        let mut active = user.inner().clone().into_active_model();

        let current_time = Utc::now().fixed_offset();
        active.email_status = Set("verified".to_string());
        active.modified_at = Set(current_time);
        active.email_confirmed_at = Set(Some(current_time));
        active.confirmation_token = Set(None);

        let res = active
            .update(&self.pool)
            .await
            .map_err(|err| ServerError::Database(format!("Could not update User: {err:?}")))?;

        Ok(MdpUser(res))
    }
}

#[cfg(test)]
mod tests {
    use tracing::info;

    use crate::db::database::{CreateUserRequest, MdpDatabase};

    #[tokio::test]
    async fn test_create_user() {
        let create_user_request = CreateUserRequest {
            name: "fake".to_string(),
            email: "test@jeremyarde.com".to_string(),
            password_hash: "fakepwhash".to_string(),
            workspace_id: Some("ws_test".to_string()),
        };

        let database_url = dotenvy::var("DATABASE_URL").unwrap();
        let db = MdpDatabase::new(database_url).await.unwrap();
        let user = &db.create_user(create_user_request).await.unwrap();

        info!("success: {user:?}")
    }

    #[tokio::test]
    async fn test_verify_user() {
        let create_user_request = CreateUserRequest {
            name: "fake".to_string(),
            email: "test@jeremyarde.com".to_string(),
            password_hash: "fakepwhash".to_string(),
            workspace_id: Some("ws_test".to_string()),
        };

        let database_url = dotenvy::var("DATABASE_URL").unwrap();
        let db = MdpDatabase::new(database_url).await.unwrap();
        let user = &db.create_user(create_user_request).await.unwrap();

        let updated_user = &db.verify_user(user.clone()).await.unwrap();

        info!("User: {user:#?}");
        info!("Updated User: {updated_user:#?}");
    }
}
