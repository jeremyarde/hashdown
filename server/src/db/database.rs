use std::fmt::{self};

use anyhow::{self, Error};

use chrono::{DateTime, Duration, Utc};
use markdownparser::nanoid_gen;
// use ormlite::{postgres::PgPool, Model};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{postgres::{PgQueryResult, PgPoolOptions}, FromRow, PgPool};

// use models::CreateAnswersModel;
// use chrono::Local;
// use sqlx::{
//     postgres::PgConnectOptions,
//     sqlite::{SqliteConnectOptions, SqliteJournalMode},
//     ConnectOptions, FromRow, PgPool, Row, SqliteConnection, SqlitePool,
// };
use tracing::{info, instrument};

// mod models;
// use models::{CreateAnswersModel, CreateSurveyRequest, SurveyModel};

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct SurveyModel {
//     pub id: i32,
//     pub plaintext: String,
//     pub user_id: String,
//     // pub created_at: String,
//     // pub modified_at: String,
//     // pub questions: Option<Vec<Question>>,
//     pub version: String,
//     pub parse_version: String,
//     pub metadata: Metadata,
// }

use crate::{error::DatabaseError, survey_responses::SubmitResponseRequest, ServerError};

use super::survey::SurveyModel;

// mod models;

#[derive(Debug, Clone)]
pub struct Database {
    // data: Arc<RwLock<TodoModel>>,
    // pub pool: SqlitePool,
    pub pool: PgPool,
    // pool: SqliteConnection,
    // pub pool: SqlitePool,
    // options: Option<DatabaseOptions>,
    pub settings: Settings,
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


// pub type InsertTodosRequest = Vec<todo::TodoModel>;

impl Database {
    pub async fn new(database_url: ConnectionDetails) -> anyhow::Result<Self> {
        // println!("{:?}", std::env::current_dir()); //Ok("/Users/jarde/Documents/code/markdownparser/server")
        // let database_url = dotenvy::var("DATABASE_URL")?;
        let database_url = database_url.0;
        let pool =  PgPoolOptions::new().max_connections(1).connect(&database_url).await?;
        // let pool = PgPool::connect(&database_url).await?;

        // info!("Running migrations");
        // sqlx::migrate!().run(&pool).await?;
        info!("Finished running migrations");

        // let settings = sqlx::query_as::<_, Settings>("select * from settings")
        //     .fetch_one(&mut pool)
        //     .await?;

        Ok(Database {
            pool,
            settings: Settings::default(),
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

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct UserModel {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub email_status: String,
    pub user_id: String,
    pub deleted_at: Option<DateTime<Utc>>,
    pub workspace_id: String,
    // pub user_id: String,
}

// #[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
// #[sqlx(type_name = "email_status", rename_all = "snake_case")]
// pub enum EmailStatus {
//     Verified,
//     Unverified,
// }
#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
pub struct AnswerModel {
    id: i32,
    submitted_at: Option<DateTime<Utc>>,
    answers: Option<Value>,
    survey_id: String,
    workspace_id: String,
}

pub struct CreateUserRequest {
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, Debug, Clone, FromRow)]
pub struct Session {
    pub id: i32,
    pub user_id: String,
    pub session_id: String,
    pub active_period_expires_at: DateTime<Utc>,
    pub idle_period_expires_at: DateTime<Utc>,
    pub workspace_id: String,
}

impl Session {
    pub fn new() -> Self {
        Session {
            id: 0,
            user_id: String::from(""),
            session_id: String::from(""),
            active_period_expires_at: chrono::Utc::now(),
            idle_period_expires_at: chrono::Utc::now(),
            workspace_id: String::from(""),
        }
    }
}

impl Database {
    // pub async fn create_magic_link(&self, _user: UserModel) -> anyhow::Result<()> {
    //     Ok(())
    // }

    pub async fn create_user(&self, request: CreateUserRequest) -> anyhow::Result<UserModel> {
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

    pub async fn get_user_by_email(&self, email: String) -> anyhow::Result<UserModel> {
        info!("Search for user with email: {email:?}");

        let res: UserModel = sqlx::query_as(
            r#"select * from mdp.users where email = $1"#)
            .bind(email)
        .fetch_one(&self.pool)
        .await?;

        info!("Found user");
        // let result = UserModel::from_row(&row_result).expect("Could not turn row into user model");

        info!("Successfully found user");
        Ok(res)
    }

    pub async fn get_survey(
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

    pub async fn create_survey(
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

        // let res = survey.insert(&self.pool).await?;

        // let res = Survey:

        // let rows = _res.rows_affected();
        // println!("create survey rows affected={rows}");
        info!("Successfully created a new survey");

        Ok(res)
    }

    pub async fn create_answer(&self, answer: SubmitResponseRequest) -> anyhow::Result<()> {
        info!("Creating answers in database");

        // let _res = sqlx::query(
        //     r#"insert into mdp.surveys_submissions (survey_id, submitted_at, answers)
        // values
        // ($1, $2, $3, $4)
        // "#,
        // )
        // // .bind(answer.answer_id)
        // .bind(answer.survey_id)
        // .bind(answer.submitted_at)
        // .bind(json!(answer.answers))
        // .execute(&self.pool)
        // .await?;

        // let _res= sqlx::query!(
        //     r#"insert into mdp.responses (submitted_at, survey_id, answers) values ($1, $2, $3) returning *"#,
        //     Utc::now(), answer.survey_id, answer.answers)
        //     .fetch_one(&self.pool).await.expect("Should insert a response");

        let _res: AnswerModel = sqlx::query_as(
            r#"insert into mdp.responses (submitted_at, survey_id, answers) values ($1, $2, $3) returning *"#)
            .bind( Utc::now()).bind(answer.survey_id).bind(answer.answers)
            .fetch_one(&self.pool).await.expect("Should insert a response");

        // let res = sqlx::query_as!(UserModel, r#"select id, user_id, email, password_hash, created_at, modified_at, deleted_at, email_status as "email_status: EmailStatus" from mdp.users where email = $1"#, email)
        // .fetch_one(&self.pool)
        // .await?;

        // answer.insert(&self.pool).await?;

        // info!("created rows={}", res.rows_affected());

        Ok(())
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
        .map_err(|_err| ServerError::Database("Did not find responses".to_string()))
        .unwrap();

        Ok(answers)
    }

    pub async fn get_session(&self, session_id: String) -> anyhow::Result<Session, ServerError> {
        // let curr_session: Session = match sqlx::query(
        //     r#"select * from sessions where sessions.session_id = $1"#
        // ).bind(session_id).execute(&self.pool).await {
        //     Ok(x) => {
        //         if x.rows_affected() == 1 {
        //             return Ok(Some(x.into()));
        //         } else if x.rows_affected() == 0 {
        //             return Ok(None);
        //         }
        //     },
        //     Err(error) => {
        //         return Err(ServerError::AuthFailTokenNotVerified("User session not available".to_string()));
        //     },
        // };

        let _curr_session: Session = match sqlx::query_as::<_, Session>(
            r#"select * from mdp.sessions sessions where sessions.session_id = $1"#,
        )
        .bind(session_id)
        .fetch_one(&self.pool)
        .await
        {
            Ok(x) => {
                info!("GET_SESSION - found");
                return Ok(x);
            }
            Err(_error) => {
                info!("GET_SESSION - not found");
                return Err(ServerError::AuthFailTokenNotVerified(
                    "User session not available".to_string(),
                ));
            }
        };
    }

    pub async fn create_session(&self, user: UserModel) -> anyhow::Result<Session, DatabaseError> {
        let session_id = nanoid_gen(32);

        let new_active_expires = Utc::now() + Duration::hours(1);
        let new_idle_expires = Utc::now() + Duration::hours(2);

        let new_session: Session = match sqlx::query_as(            r#"
            insert into mdp.sessions (session_id, user_id, active_period_expires_at, idle_period_expires_at, workspace_id) 
            values ($1, $2, $3, $4, $5)
            ON conflict (user_id) do update 
            set session_id = $1, active_period_expires_at = $3, idle_period_expires_at = $4
            where sessions.user_id = $2 returning *"#)
            .bind(session_id).bind(user.user_id)
            .bind(new_active_expires).bind( new_idle_expires)
            .bind( user.workspace_id)
        .fetch_one(&self.pool).await {
            Ok(x) => x,
            Err(err) => {
                info!("create session failed...");
                return Err(DatabaseError::Misc(err.to_string()));
            }
        };

        Ok(new_session)
    }

    pub async fn update_session(&self, session: Session) -> anyhow::Result<Session, ServerError> {
        let curr_session = sqlx::query_as::<_, Session>(
            r#"update mdp.sessions set active_period_expires_at = $1, idle_period_expires_at = $2 where mdp.sessions.session_id = $3 and mdp.sessions.user_id = $4 and mdp.sessions.workspace_id = $5 returning *"#
        ).bind(session.active_period_expires_at)
        .bind(session.idle_period_expires_at)
        .bind(session.session_id)
        .bind(session.user_id)
        .bind(session.workspace_id)
        .fetch_one(&self.pool).await.unwrap();

        Ok(curr_session)
    }

    pub async fn delete_session(
        &self,
        session_id: &str,
        // workspace_id: &str,
    ) -> anyhow::Result<bool, DatabaseError> {
        let result: PgQueryResult =
            sqlx::query(r#"delete from mdp.sessions where mdp.sessions.session_id = $1"#)
                .bind(session_id)
                .execute(&self.pool)
                .await
                .map_err(|err| {
                    return DatabaseError::Misc(format!(
                        "Could not delete session {}: {}",
                        session_id, err
                    ));
                })
                .unwrap();

        if result.rows_affected() == 1 {
            Ok(true)
        } else {
            return Err(DatabaseError::Misc("Could not find session".to_string()));
        }
    }

    pub async fn delete_user(
        &self,
        user_id: &str,
        workspace_id: &str,
    ) -> anyhow::Result<String, DatabaseError> {
        let _: () = sqlx::query_as(
            "delete from mdp.users where users.user_id = $1 and mdp.users.workspace_id = $2",
        )
        .bind(user_id)
        .bind(workspace_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| {
            DatabaseError::Misc(format!("Could not delete user: {}", err));
        })
        .unwrap();

        Ok(user_id.to_string())
    }
}

#[cfg(test)]
mod tests {}
