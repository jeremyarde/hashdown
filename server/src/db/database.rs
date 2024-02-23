use std::{
    fmt::{self, Display},
    ops::Add,
};

use anyhow::{self, Error};

use chrono::{DateTime, Duration, Utc};
use hyper::Server;
use lettre::transport::smtp::commands::Data;
use markdownparser::{nanoid_gen, NanoId};

use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, Database, DatabaseConnection, EntityTrait,
    FromQueryResult, IntoActiveModel, QueryFilter, Set, TryIntoModel,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
// use sqlx::{
//     postgres::{PgPoolOptions, PgQueryResult, PgTypeInfo},
//     Decode, Encode, FromRow, PgPool, Postgres, Type,
// };
use tracing::info;

use crate::{mware::ctext::SessionContext, survey_responses::SubmitResponseRequest, ServerError};

use super::{
    // sessions::Session,
    surveys::CreateSurveyRequest,
};

use entity::{
    sessions::{self, ActiveModel, Entity as Session, Model as SessionModel},
    surveys,
    users::{self, ActiveModel as UserActiveModel, Entity as User, Model as UserModel},
    workspaces,
};

use migration::{Migrator, MigratorTrait};

// mod models;

#[derive(Debug, Clone)]
pub struct MdpDatabase {
    // data: Arc<RwLock<TodoModel>>,
    // pub pool: SqlitePool,
    // pub pool: PgPool,
    // pool: SqliteConnection,
    // pub pool: SqlitePool,
    // options: Option<DatabaseOptions>,
    pub settings: Settings,
    pub pool: DatabaseConnection,
}

#[derive()]
pub struct ConnectionDetails(pub String);

#[derive(Debug, Clone)]
pub struct Settings {
    pub base_path: Option<String>,
    pub nanoid_length: Option<usize>,
}

impl Settings {
    fn default() -> Settings {
        Settings {
            base_path: None,
            nanoid_length: Some(12_usize),
        }
    }
}

struct UpdateSurveyRequest {
    id: NanoIdModel,
}

// would be nice to do...
// #[orm(Create, Delete, Update, Read)]
// struct UserModel {
//     myvalue: String
// }

impl MdpDatabase {
    pub async fn new() -> anyhow::Result<Self> {
        let uri = dotenvy::var("DATABASE_URL").expect("Could not get connection string from env");
        // let db_url = ConnectionDetails(uri.clone());

        info!("Finished running migrations");

        let db: DatabaseConnection = Database::connect(uri).await?;

        Migrator::up(&db, None).await?;

        Ok(MdpDatabase {
            // pool,
            settings: Settings::default(),
            pool: db,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnswerRequest {
    pub form_id: String,
    pub start_time: String,
    pub answers: QuestionAnswers,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionAnswers {
    pub answers: Vec<Answer>,
    // pub question_id: String,
    // pub answers: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Answer {
    pub question_id: String,
    pub answers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
pub struct Email {
    email: String,
}

impl Email {
    fn new(email: String) -> Email {
        return Email { email };
    }
}

// impl Display for Email {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.email)
//     }
// }

#[derive(sqlx::Type, Debug, Deserialize, Serialize, Clone)]
pub struct NanoIdModel(String);

// impl Type<Postgres> for NanoIdModel {
//     fn type_info() -> PgTypeInfo {
//         PgTypeInfo::String
//     }
// }

// #[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
// pub struct UserModel {
//     pub id: Option<i32>,
//     pub email: Email,
//     pub password_hash: String,
//     pub created_at: DateTime<Utc>,
//     pub modified_at: DateTime<Utc>,
//     pub deleted_at: Option<DateTime<Utc>>,
//     pub email_status: Option<String>,
//     pub user_id: NanoId,
//     pub workspace_id: String,
//     pub email_confirmed_at: Option<DateTime<Utc>>,
//     pub confirmation_token: Option<String>,
//     pub confirmation_token_expire_at: Option<DateTime<Utc>>,
//     pub role: Option<String>,
// }

// impl UserModel {
//     pub fn from(
//         email: String,
//         password_hash: String,
//         mut workspace_id: Option<String>,
//     ) -> UserModel {
//         if workspace_id.is_none() {
//             workspace_id = Some(NanoId::from("ws").to_string());
//         }

//         UserModel {
//             id: None,
//             email: Email::new(email),
//             password_hash: password_hash,
//             created_at: chrono::Utc::now(),
//             modified_at: chrono::Utc::now(),
//             email_status: Some(String::from("unverified")),
//             user_id: NanoIdModel(NanoId::from("usr").to_string()),
//             deleted_at: None,
//             workspace_id,
//             email_confirmed_at: None,
//             confirmation_token: Some(NanoId::from_len(24).to_string()),
//             confirmation_token_expire_at: Some(chrono::Utc::now().add(Duration::days(1))),
//             role: None,
//         }
//     }
// }

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AnswerModel {
    id: i32,
    response_id: String,
    submitted_at: Option<DateTime<Utc>>,
    pub answers: Option<Value>,
    survey_id: String,
    workspace_id: String,
}

pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub workspace_id: Option<String>,
}

// pub trait SurveyCrud {
//     // async fn create_user(&self, request: CreateUserRequest) -> anyhow::Result<UserModel>;
//     async fn list_survey(&self, ctx: SessionContext) -> anyhow::Result<Vec<SurveyModel>>;
//     async fn get_survey(&self, survey_id: &String) -> anyhow::Result<SurveyModel>;
//     async fn create_survey(
//         &self,
//         survey: SurveyModel,
//         workspace_id: &str,
//     ) -> anyhow::Result<SurveyModel>;
// }
use entity::surveys::Entity as Survey;
use entity::surveys::Model as SurveyModel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdpSurvey(pub SurveyModel);

impl MdpSurvey {
    pub fn inner(&self) -> &SurveyModel {
        return &self.0;
    }
}

impl MdpDatabase {
    pub async fn get_survey(
        &self,
        survey_id: &String,
        // workspace_id: &str,
    ) -> Result<MdpSurvey, ServerError> {
        let result = Survey::find()
            .filter(surveys::Column::SurveyId.eq(survey_id))
            .one(&self.pool)
            .await
            .map_err(|err| ServerError::Database(err.to_string()))?;

        Ok(MdpSurvey(result.unwrap()))
    }

    pub async fn list_survey(&self, ctx: SessionContext) -> Result<Vec<MdpSurvey>, ServerError> {
        let all_surveys = entity::surveys::Entity::find()
            .filter(entity::surveys::Column::UserId.eq(ctx.session.0.user_id))
            .all(&self.pool)
            .await
            .map_err(|err| ServerError::Database(err.to_string()))?
            .iter()
            .map(|sur| MdpSurvey(sur.clone()))
            .collect();

        Ok(all_surveys)
    }

    pub async fn create_survey(
        &self,
        survey: MdpSurvey,
        workspace_id: &str,
    ) -> anyhow::Result<MdpSurvey> {
        // let parsed_survey = parse_markdown_v3(payload.plaintext.clone())?;
        let survey = survey
            .0
            .into_active_model()
            .save(&self.pool)
            .await
            .map_err(|err| ServerError::Database(format!("Error in database: {err}")))?;

        info!("Successfully created a new survey");

        Ok(MdpSurvey(survey.try_into_model().unwrap()))
    }
}

#[derive(Debug, Clone)]
pub struct MdpSession(pub SessionModel);

#[derive(Debug, Clone)]
pub struct MdpUser(pub UserModel);

impl MdpUser {
    pub fn inner(&self) -> &UserModel {
        return &self.0;
    }
}

impl MdpUser {
    pub fn from(name: &str, email: &str, password_hash: &str, workspace_id: &str) -> MdpUser {
        let user = users::ActiveModel {
            email: Set(Email::new(email.to_string()).email),
            password_hash: Set(password_hash.to_string()),
            created_at: Set(chrono::Utc::now().fixed_offset()),
            modified_at: Set(chrono::Utc::now().fixed_offset()),
            email_status: Set(String::from("unverified")),
            user_id: Set(NanoId::from("usr").to_string()),
            workspace_id: Set(workspace_id.to_string()),
            confirmation_token: Set(Some(NanoId::from_len(24).to_string())),
            confirmation_token_expire_at: Set(Some(
                chrono::Utc::now().fixed_offset().add(Duration::days(1)),
            )),
            deleted_at: Set(None),
            role: Set(None),
            email_confirmed_at: Set(None),
            stripe_id: Set(None),
            name: Set(name.to_string()),
            ..Default::default()
        };

        return MdpUser(user.try_into_model().unwrap());

        // UserModel {
        //     id: None,
        //     email: Email::new(email.clone().to_string()),
        //     password_hash: password_hash.clone().to_string(),
        //     created_at: chrono::Utc::now(),
        //     modified_at: chrono::Utc::now(),
        //     email_status: Some(String::from("unverified")),
        //     user_id: NanoIdModel(NanoId::from("usr").to_string()),
        //     deleted_at: None,
        //     workspace_id: workspace_id.to_string(),
        //     email_confirmed_at: None,
        //     confirmation_token: Some(NanoId::from_len(24).to_string()),
        //     confirmation_token_expire_at: Some(chrono::Utc::now().add(Duration::days(1))),
        //     role: None,
        // }
    }
}

// #[derive(Clone)]
// pub struct MdpSession(pub ActiveModel);

use entity::workspaces::Model as WorkspaceModel;
pub struct MdpWorkspace(pub WorkspaceModel);

use entity::responses::Model as ResponseModel;
pub struct MdpResponse(pub ResponseModel);

impl MdpDatabase {
    pub async fn create_workspace(&self) -> Result<MdpWorkspace, ServerError> {
        let workspace_id = NanoId::from("ws").to_string();
        let name = "".to_string();
        info!("Creating new workspace with workspace_id={workspace_id}");

        let workspace = workspaces::ActiveModel {
            workspace_id: Set(workspace_id),
            name: Set(name),
            ..Default::default()
        }
        .insert(&self.pool)
        .await
        .map_err(|err| ServerError::Database(format!("Could not create workspace: {err:?}")))?;

        info!("Workspace created");

        return Ok(MdpWorkspace(workspace.try_into_model().unwrap()));
    }

    pub async fn create_answer(
        &self,
        answer: SubmitResponseRequest,
    ) -> Result<MdpResponse, ServerError> {
        info!("Creating answers in database");

        // make sure survey exists
        let survey = entity::surveys::Entity::find()
            .filter(surveys::Column::SurveyId.eq(answer.survey_id.clone()))
            .one(&self.pool)
            .await
            .map_err(|ex| ServerError::Database(format!("Could not find survey: {ex}")))?;

        if survey.is_none() {
            return Err(ServerError::Database(format!("Could not find survey")));
        }

        let answer = entity::responses::ActiveModel {
            response_id: Set(NanoId::from("ans").to_string()),
            workspace_id: Set(survey.unwrap().workspace_id),
            submitted_at: Set(Some(Utc::now().fixed_offset())),
            answers: Set(Some(answer.answers)),
            survey_id: Set(answer.survey_id),
            ..Default::default()
        }
        .insert(&self.pool)
        .await
        .map_err(|ex| ServerError::Database(format!("Could not create answer: {ex}")))?;

        Ok(MdpResponse(answer))
    }

    pub async fn list_responses(
        &self,
        survey_id: &str,
        workspace_id: &str,
    ) -> anyhow::Result<Vec<entity::responses::Model>, ServerError> {
        info!("Listing responses for survey");

        let responses = entity::responses::Entity::find()
            .filter(entity::responses::Column::SurveyId.eq(survey_id))
            .filter(entity::responses::Column::WorkspaceId.eq(workspace_id))
            .all(&self.pool)
            .await
            .map_err(|_err| ServerError::Database("Did not find responses".to_string()))?;

        Ok(responses)
    }

    pub async fn get_session(&self, session_id: String) -> anyhow::Result<MdpSession, ServerError> {
        let curr_session = Session::find()
            .filter(sessions::Column::SessionId.eq(session_id))
            .one(&self.pool)
            .await
            .map_err(|err| ServerError::Database(format!("Did not find session: {err}")))?;

        if let Some(session) = curr_session {
            return Ok(MdpSession(session.into()));
        } else {
            return Err(ServerError::Database(format!("Did not find session")));
        };
    }

    pub async fn create_session(&self, user: MdpUser) -> anyhow::Result<MdpSession, ServerError> {
        let session_id = nanoid_gen(32);

        let new_active_expires = DateTime::fixed_offset(&Utc::now().add(Duration::days(1)));
        let new_idle_expires = DateTime::fixed_offset(&Utc::now().add(Duration::days(2)));

        let new_session = sessions::ActiveModel {
            workspace_id: Set(user.inner().workspace_id.clone()),
            session_id: Set(NanoId::from("sen").to_string()),
            user_id: Set(user.inner().user_id.to_string()),
            active_period_expires_at: Set(new_active_expires),
            idle_period_expires_at: Set(new_idle_expires),
            ..Default::default()
        }
        .save(&self.pool)
        .await
        .map_err(|err| ServerError::Database(format!("Could not create session. Error: {err}")))?;

        return Ok(MdpSession(new_session.try_into_model().unwrap()));
    }

    pub async fn delete_session(
        &self,
        session: &SessionModel,
    ) -> anyhow::Result<bool, ServerError> {
        let result = entity::sessions::Entity::delete_by_id((
            session.session_id.to_string(),
            session.workspace_id.to_string(),
        ))
        .exec(&self.pool)
        .await
        .map_err(|err| {
            ServerError::Database(format!(
                "Could not delete session {}: {}",
                session.session_id, err
            ))
        })?;

        if result.rows_affected == 1 {
            Ok(true)
        } else {
            return Err(ServerError::Database(format!(
                "Session does not exist: {}",
                session.session_id,
            )));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MdpDatabase;

    #[tokio::test]
    async fn test_get_session() {
        let db = MdpDatabase::new().await.unwrap();

        let session = db
            .get_session("ut46xsy1wm6v91qcf9ew4ijzcdgbq14z".to_string())
            .await
            .unwrap();

        println!("session: {:?}", session);
    }
}
