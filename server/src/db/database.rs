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
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{
    postgres::{PgPoolOptions, PgQueryResult, PgTypeInfo},
    Decode, Encode, FromRow, PgPool, Postgres, Type,
};
use tracing::info;

use crate::{mware::ctext::SessionContext, survey_responses::SubmitResponseRequest, ServerError};

use super::{
    // sessions::Session,
    surveys::{CreateSurveyRequest, SurveyModel},
};

use entity::{
    sessions::{self, ActiveModel, Entity as Session, Model as SessionModel},
    users::{ActiveModel as UserActiveModel, Entity as User, Model as UserModel},
};

use migration::{Migrator, MigratorTrait};

// mod models;

#[derive(Debug, Clone)]
pub struct MdpDatabase {
    // data: Arc<RwLock<TodoModel>>,
    // pub pool: SqlitePool,
    pub pool: PgPool,
    // pool: SqliteConnection,
    // pub pool: SqlitePool,
    // options: Option<DatabaseOptions>,
    pub settings: Settings,
    pub sea_pool: DatabaseConnection,
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
        let db_url = ConnectionDetails(uri.clone());
        // println!("{:?}", std::env::current_dir()); //Ok("/Users/jarde/Documents/code/markdownparser/server")
        // let database_url = dotenvy::var("DATABASE_URL")?;
        let database_url = db_url.0;
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await?;
        // let pool = PgPool::connect(&database_url).await?;

        // info!("Running migrations");
        // sqlx::migrate!().run(&pool).await?;
        info!("Finished running migrations");

        // let settings = sqlx::query_as::<_, Settings>("select * from settings")
        //     .fetch_one(&mut pool)
        //     .await?;
        let db: DatabaseConnection = Database::connect(uri).await?;

        // let connection = sea_orm::Database::connect(&database_url).await?;
        Migrator::up(&db, None).await?;

        Ok(MdpDatabase {
            pool,
            settings: Settings::default(),
            sea_pool: db,
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

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
pub struct AnswerModel {
    id: i32,
    response_id: String,
    submitted_at: Option<DateTime<Utc>>,
    pub answers: Option<Value>,
    survey_id: String,
    workspace_id: String,
}

pub struct CreateUserRequest {
    pub email: String,
    pub password_hash: String,
    pub workspace_id: Option<String>,
}

pub trait SurveyCrud {
    // async fn create_user(&self, request: CreateUserRequest) -> anyhow::Result<UserModel>;
    async fn list_survey(&self, ctx: SessionContext) -> anyhow::Result<Vec<SurveyModel>>;
    async fn get_survey(&self, survey_id: &String) -> anyhow::Result<SurveyModel>;
    async fn create_survey(
        &self,
        survey: SurveyModel,
        workspace_id: &str,
    ) -> anyhow::Result<SurveyModel>;
}

impl SurveyCrud for MdpDatabase {
    async fn get_survey(
        &self,
        survey_id: &String,
        // workspace_id: &str,
    ) -> anyhow::Result<SurveyModel> {
        let result = sqlx::query_as::<_, SurveyModel>(
            "select * from mdp.surveys where surveys.survey_id = $1",
        )
        .bind(survey_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(result)
    }

    async fn list_survey(&self, ctx: SessionContext) -> anyhow::Result<Vec<SurveyModel>> {
        let result = sqlx::query_as::<_, SurveyModel>(
                "select * from mdp.surveys where mdp.surveys.user_id = $1 and mdp.surveys.workspace_id = $2",
            )
            .bind(ctx.user_id.clone())
            .bind(ctx.session.workspace_id.clone())
            .fetch_all(&self.pool)
            .await
            .map_err(|err| ServerError::Database(err.to_string()))
            .unwrap();

        Ok(result)
    }

    async fn create_survey(
        &self,
        survey: SurveyModel,
        workspace_id: &str,
    ) -> anyhow::Result<SurveyModel> {
        // let parsed_survey = parse_markdown_v3(payload.plaintext.clone())?;
        let res: SurveyModel = sqlx::query_as(
            r#"insert into mdp.surveys (
                    name, survey_id, user_id, created_at, modified_at, plaintext, version, parse_version, blocks, workspace_id
                ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) returning *"#)
            .bind(survey.name)
            .bind(survey.survey_id)
            .bind(survey.user_id)
            .bind(Utc::now())
            .bind(Utc::now())
            .bind(survey.plaintext)
            .bind(survey.version)
            .bind(survey.parse_version)
            .bind(survey.blocks)
            .bind(workspace_id)
        .fetch_one(&self.pool)
        .await
        .expect("Should insert a survey");

        info!("Successfully created a new survey");

        Ok(res)
    }
}

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
pub struct WorkspaceModel {
    pub id: i32,
    pub workspace_id: String,
    pub name: String,
}

pub struct MdpSession(pub SessionModel);

pub struct MdpUser(pub UserModel);

impl MdpUser {
    pub fn from(email: &str, password_hash: &str, workspace_id: &str) -> MdpUser {
        if workspace_id.is_none() {
            workspace_id = Some(NanoId::from("ws"));
        }

        UserModel {
            id: None,
            email: Email::new(email.clone().to_string()),
            password_hash: password_hash.clone().to_string(),
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
            email_status: Some(String::from("unverified")),
            user_id: NanoIdModel(NanoId::from("usr").to_string()),
            deleted_at: None,
            workspace_id: workspace_id.to_string(),
            email_confirmed_at: None,
            confirmation_token: Some(NanoId::from_len(24).to_string()),
            confirmation_token_expire_at: Some(chrono::Utc::now().add(Duration::days(1))),
            role: None,
        }
    }
}

#[derive(Clone)]
pub struct MdpActiveSession(pub ActiveModel);

impl MdpDatabase {
    pub async fn create_workspace(&self) -> Result<WorkspaceModel, ServerError> {
        let workspace_id = NanoId::from("ws").to_string();
        let name = "";
        info!("Creating workspace with workspace_id={workspace_id}");

        let workspace: WorkspaceModel = sqlx::query_as(
            "insert into mdp.workspaces (workspace_id, name) values ($1, $2) returning *",
        )
        .bind(workspace_id)
        .bind(name)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| ServerError::Database(format!("Could not create workspace: {err:?}")))?;

        info!("Workspace created");

        return Ok(workspace);
    }

    pub async fn create_answer(&self, answer: SubmitResponseRequest) -> Result<Value, ServerError> {
        info!("Creating answers in database");

        let workspace_id: (String,) =
            sqlx::query_as("select workspace_id from mdp.surveys where mdp.surveys.survey_id = $1")
                .bind(answer.survey_id.clone())
                .fetch_one(&self.pool)
                .await
                .map_err(|ex| ServerError::Database(format!("Could not create answer: {ex}")))?;
        let res: AnswerModel = sqlx::query_as(
            r#"insert into mdp.responses (response_id, submitted_at, survey_id, answers, workspace_id) values ($1, $2, $3, $4, $5) returning *"#)
            .bind(NanoId::from("res").to_string())
            .bind(Utc::now())
            .bind(answer.survey_id).bind(answer.answers)
            .bind(workspace_id.0)
            .fetch_one(&self.pool).await
            .map_err(|ex| ServerError::Database(format!("Could not create answer: {ex}")))?;

        Ok(json!(res))
    }

    pub async fn list_responses(
        &self,
        survey_id: &str,
        workspace_id: &str,
    ) -> anyhow::Result<Vec<AnswerModel>, ServerError> {
        info!("Listing responses for survey");

        let answers: Vec<AnswerModel> = sqlx::query_as(
            r#"select * from mdp.responses where mdp.responses.survey_id = $1 and mdp.responses.workspace_id = $2"#)
            .bind(survey_id).bind(workspace_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|_err| ServerError::Database("Did not find responses".to_string()))?;

        Ok(answers)
    }

    pub async fn get_session(
        &self,
        session_id: String,
    ) -> anyhow::Result<MdpActiveSession, ServerError> {
        // let curr_session: Session = sqlx::query_as::<_, Session>(
        //     r#"select * from mdp.sessions where mdp.sessions.session_id = $1"#,
        // )
        // .bind(session_id)
        // .fetch_one(&self.pool)
        // .await
        // .map_err(|err| ServerError::Database(format!("Did not find session: {err}")))?;
        let curr_session = Session::find()
            .filter(sessions::Column::SessionId.eq(session_id))
            .one(&self.sea_pool)
            .await
            .map_err(|err| ServerError::Database(format!("Did not find session: {err}")))?;

        if let Some(session) = curr_session {
            return Ok(MdpActiveSession(session.into()));
        } else {
            return Err(ServerError::Database(format!("Did not find session")));
        };
    }

    pub async fn create_session(
        &self,
        user: MdpUser,
    ) -> anyhow::Result<MdpActiveSession, ServerError> {
        let session_id = nanoid_gen(32);

        let new_active_expires = DateTime::fixed_offset(&Utc::now().add(Duration::days(1)));
        let new_idle_expires = DateTime::fixed_offset(&Utc::now().add(Duration::days(2)));

        // let new_session: Session = sqlx::query_as(r#"
        //     insert into mdp.sessions (session_id, user_id, active_period_expires_at, idle_period_expires_at, workspace_id)
        //     values ($1, $2, $3, $4, $5)
        //     ON conflict (user_id) do update
        //     set session_id = $1, active_period_expires_at = $3, idle_period_expires_at = $4
        //     where sessions.user_id = $2 returning *"#)
        //     .bind(session_id).bind(user.user_id)
        //     .bind(new_active_expires).bind( new_idle_expires)
        //     .bind( user.workspace_id)
        // .fetch_one(&self.pool).await.map_err(|err| ServerError::Database(err.to_string()))?;
        let new_session = sessions::ActiveModel {
            workspace_id: Set(user.0.workspace_id),
            session_id: Set(NanoId::from("sen").to_string()),
            user_id: Set(user.0.user_id.to_string()),
            active_period_expires_at: Set(new_active_expires),
            idle_period_expires_at: Set(new_idle_expires),
            ..Default::default()
        }
        .save(&self.sea_pool)
        .await
        .map_err(|err| ServerError::Database(format!("Could not create session. Error: {err}")))?;

        return Ok(MdpActiveSession(new_session));
    }

    // pub async fn update_session(
    //     &self,
    //     session: MdpActiveSession,
    // ) -> anyhow::Result<Session, ServerError> {
    //     // let curr_session = sqlx::query_as::<_, Session>(
    //     //     r#"update mdp.sessions set active_period_expires_at = $1, idle_period_expires_at = $2 where mdp.sessions.session_id = $3 and mdp.sessions.user_id = $4 and mdp.sessions.workspace_id = $5 returning *"#
    //     // ).bind(session.active_period_expires_at)
    //     // .bind(session.idle_period_expires_at)
    //     // .bind(session.session_id)
    //     // .bind(session.user_id)
    //     // .bind(session.workspace_id)
    //     // .fetch_one(&self.pool).await.unwrap();

    //     Ok(curr_session)
    // }

    pub async fn delete_session(
        &self,
        session_id: &str,
        // workspace_id: &str,
    ) -> anyhow::Result<bool, ServerError> {
        let result: PgQueryResult =
            sqlx::query(r#"delete from mdp.sessions where mdp.sessions.session_id = $1"#)
                .bind(session_id)
                .execute(&self.pool)
                .await
                .map_err(|err| {
                    ServerError::Database(format!(
                        "Could not delete session {}: {}",
                        session_id, err
                    ))
                })?;

        if result.rows_affected() == 1 {
            Ok(true)
        } else {
            return Err(ServerError::Database(format!(
                "Session does not exist: {}",
                session_id
            )));
        }
    }

    pub async fn delete_user(
        &self,
        user_id: &str,
        workspace_id: &str,
    ) -> anyhow::Result<String, ServerError> {
        let _: () = sqlx::query_as(
            "delete from mdp.users where users.user_id = $1 and mdp.users.workspace_id = $2",
        )
        .bind(user_id)
        .bind(workspace_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| ServerError::Database(format!("Could not delete user: {}", err)))?;

        Ok(user_id.to_string())
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
