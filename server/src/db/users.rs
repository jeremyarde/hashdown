use markdownparser::NanoId;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    QueryFilter, Set, TryIntoModel,
};
use tracing::{debug, info};

use sqlx;

use chrono::{self, DateTime, Utc};

use crate::db::stripe;
use crate::{MdpDatabase, ServerError};

use crate::db::database::MdpUser;

use super::database::CreateUserRequest;

use entity::users::{self, Entity as User};

trait UsersTrait {
    fn new(req: CreateUserRequest) -> users::ActiveModel;
}

impl UsersTrait for users::ActiveModel {
    fn new(req: CreateUserRequest) -> users::ActiveModel {
        return users::ActiveModel {
            ..Default::default()
        };
    }
}

use entity::workspaces::{self, Entity as Workspace};
impl MdpDatabase {
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<MdpUser, ServerError> {
        println!("->> create_user");

        let mut ws_id: String;
        if request.workspace_id.is_none() {
            let new_workspace = workspaces::ActiveModel {
                workspace_id: Set(NanoId::from("ws").to_string()),
                name: Set(String::from("default")),
                ..Default::default()
            }
            .insert(&self.sea_pool)
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
            .insert(&self.sea_pool)
            .await
            .map_err(|err| ServerError::Database(format!("Could not create user: {err}")))?;

        let stripe_customer = &self
            .create_customer(user.name.as_ref(), user.email.as_ref())
            .await?;

        let mut active_user = user.into_active_model();

        active_user.stripe_id = Set(Some(
            stripe_customer
                .get("id")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        ));
        active_user.update(&self.sea_pool).await.map_err(|err| {
            ServerError::Database(format!("Failed to update user with stripe_id: {}", err))
        })?;

        info!("Created new user: {:?}", new_user.0.email);

        Ok(new_user)
    }

    pub async fn get_user_by_email(&self, email: String) -> Result<Option<MdpUser>, ServerError> {
        info!("Search for user with email: {email:?}");

        let user = User::find()
            .filter(users::Column::Email.eq(email.clone()))
            .one(&self.sea_pool)
            .await
            .map_err(|err| {
                ServerError::Database(format!("Could not find user with email. Error: {err}"))
            })?;

        debug!("user: {:#?}", user);

        match user {
            Some(x) => return Ok(Some(MdpUser(x))),
            None => return Ok(None),
        }
    }

    pub async fn get_user_by_confirmation_code(
        &self,
        confirmation_token: String,
    ) -> Result<MdpUser, ServerError> {
        info!("Search for user with email confirm code: {confirmation_token:?}");

        let user = User::find()
            .filter(users::Column::ConfirmationToken.eq(confirmation_token.clone()))
            .one(&self.sea_pool)
            .await
            .map_err(|err| {
                ServerError::Database(format!("Could not find user with email. Error: {err}"))
            })?;
        // let res: UserModel =
        //     sqlx::query_as(r#"select * from mdp.users where confirmation_token = $1"#)
        //         .bind(confirmation_token)
        //         .fetch_one(&self.pool)
        //         .await
        //         .map_err(|err| {
        //             ServerError::Database(format!(
        //                 "Could not find user with confirmation_token: {err}"
        //             ))
        //         })?;
        if let Some(x) = user {
            return Ok(MdpUser(x));
        } else {
            return Err(ServerError::Database("Could not find user".to_string()));
        }
        // info!("Found user");
        // Ok(res)
    }

    pub async fn verify_user(&self, user: MdpUser) -> Result<MdpUser, ServerError> {
        let mut active = user.0.into_active_model();
        active.email_status = Set("verified".to_string());
        active.modified_at = Set(Utc::now().fixed_offset());
        active.email_confirmed_at = Set(Some(Utc::now().fixed_offset()));

        let res = active
            .update(&self.sea_pool)
            .await
            .map_err(|err| ServerError::Database(format!("Could not update User: {err:?}")))?;

        //         pear.name = Set("Sweet pear".to_owned());

        // // SQL: `UPDATE "fruit" SET "name" = 'Sweet pear' WHERE "id" = 28`
        // let pear: fruit::Model = pear.update(db).await?;
        // let user: UserModel = sqlx::query_as(
        //     "update mdp.users set
        //     email_status = $1,
        //     confirmation_token = NULL,
        //     confirmation_token_expire_at = NULL,
        //     modified_at = $2,
        //     email_confirmed_at = $3

        //     where mdp.users.user_id = $4 and mdp.users.workspace_id = $5
        //     returning *;
        //     ",
        // )
        // .bind("verified") //email status
        // .bind(Utc::now()) //mod
        // .bind(Utc::now()) //confirmed
        // .bind(&user.user_id)
        // .bind(&user.workspace_id)
        // .fetch_one(&self.pool)
        // .await
        // .map_err(|err| ServerError::Database(format!("Could not update User: {err:?}")))?;

        return Ok(MdpUser(res));
    }
}

#[cfg(test)]
mod tests {
    use crate::db::database::{CreateUserRequest, MdpDatabase};

    #[tokio::test]
    async fn test_create_user() {
        let create_user_request = CreateUserRequest {
            name: "fake".to_string(),
            email: "test@jeremyarde.com".to_string(),
            password_hash: "fakepwhash".to_string(),
            workspace_id: Some("ws_test".to_string()),
        };

        let db = MdpDatabase::new().await.unwrap();
        let user = db.create_user(create_user_request).await.unwrap();

        println!("success: {user:?}")
    }

    #[tokio::test]
    async fn test_verify_user() {
        let create_user_request = CreateUserRequest {
            name: "fake".to_string(),
            email: "test@jeremyarde.com".to_string(),
            password_hash: "fakepwhash".to_string(),
            workspace_id: Some("ws_test".to_string()),
        };

        let db = MdpDatabase::new().await.unwrap();
        let user = db.create_user(create_user_request).await.unwrap();

        let updated_user = db.verify_user(user.clone()).await.unwrap();

        println!("User: {user:#?}");
        println!("Updated User: {updated_user:#?}");
    }
}
