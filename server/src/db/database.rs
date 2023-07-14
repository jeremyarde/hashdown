use std::{
    collections::HashMap,
    fmt::{self, Display},
    str::FromStr,
};

use anyhow::{self, Context};

use chrono::{DateTime, Utc};
use markdownparser::{nanoid_gen, parse_markdown_v3, Survey};
use ormlite::{postgres::PgPool, Model};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
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

#[derive(Clone, Debug, Serialize, Deserialize, Model)]
pub struct SurveyModel {
    pub id: i32,
    pub plaintext: String,
    pub user_id: String,
    pub created_at: String,
    pub modified_at: String,
    // pub questions: Option<Vec<Question>>,
    pub version: String,
    pub parse_version: String,
}

use super::models;

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
            nanoid_length: Some(12 as usize),
        }
    }
}

impl fmt::Display for ConnectionDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let databasetype = self.0.split("://").nth(0);
        match databasetype {
            Some(x) => write!(f, "DB: {}", x),
            None => write!(f, "DB: {}", &"Not sure DB type"),
        }
    }
}

impl fmt::Debug for ConnectionDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DBConnection")
            .field("type", &self.0.split("://").nth(0))
            .finish()
    }
}

// pub type InsertTodosRequest = Vec<todo::TodoModel>;

impl Database {
    #[instrument]
    pub async fn new(in_memory: bool, database_url: ConnectionDetails) -> anyhow::Result<Self> {
        // println!("{:?}", std::env::current_dir()); //Ok("/Users/jarde/Documents/code/markdownparser/server")
        // let database_url = dotenvy::var("DATABASE_URL")?;
        let database_url = database_url.0;
        let pool = match in_memory {
            true => {
                info!("Creating in-memory database");

                // let conn = SqliteConnectOptions::from_str("sqlite::memory:")?
                //     .journal_mode(SqliteJournalMode::Wal)
                //     .read_only(false)
                //     .create_if_missing(true);
                // SqlitePool::connect_with(conn).await?

                // let conn = PgConnectOptions::default();
                PgPool::connect(&database_url).await?
                // PgPool::connect_with(conn).await?

                // let connection_options = SqliteConnectOptions::new()
                //     .create_if_missing(true)
                //     // .filename("~/Library/todowatcher_data.db"); // maybe try to save state in a common location
                //     .filename(database_url);
                // PgPool::connect(&database_url).await?
            }
            false => {
                info!("Creating new database");
                // let conn = SqliteConnectOptions::from_str(database_url.as_str())?
                //     .journal_mode(SqliteJournalMode::Wal)
                //     .read_only(false)
                //     .create_if_missing(true);
                // SqlitePool::connect_with(conn).await?

                // let conn = PgConnectOptions::default();
                // PgPool::connect_with(conn).await?
                PgPool::connect(&database_url).await?

                // let connection_options = SqliteConnectOptions::new()
                //     .create_if_missing(true)
                //     // .filename("~/Library/todowatcher_data.db"); // maybe try to save state in a common location
                //     .filename(database_url);
                // SqlitePool::connect_with(connection_options).await?
                // PgPool::connect(&database_url).await?
            }
        };

        // info!("Running migrations");
        // sqlx::migrate!().run(&pool).await?;
        // info!("Finished running migrations");

        // let settings = sqlx::query_as::<_, Settings>("select * from settings")
        //     .fetch_one(&mut pool)
        //     .await?;

        Ok(Database {
            pool: pool,
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

#[derive(Debug, Deserialize, Serialize, Model)]
pub struct CreateAnswersModel {
    pub id: Option<String>,
    pub answer_id: String,
    // pub external_id: String,
    pub survey_id: String,
    // pub survey_version: String,
    pub submitted_at: String,
    pub answers: Value,
}

#[derive(Debug, Model, Serialize, Deserialize)]
pub struct UserModel {
    #[ormlite(primary_key)]
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub verified: bool,
    pub user_id: String,
    // pub user_id: String,
}

pub struct CreateUserRequest {
    pub email: String,
    pub password_hash: String,
}

impl Database {
    pub async fn create_user(&self, request: CreateUserRequest) -> anyhow::Result<UserModel> {
        println!("->> create_user");

        let time = chrono::Utc::now();
        let user = UserModel {
            // id: todo!(),
            email: request.email,
            password_hash: request.password_hash,
            created_at: time,
            modified_at: time,
            verified: false,
            user_id: nanoid_gen(24),
            id: 0,
        }
        .insert(&self.pool)
        .await?;
        // let result = sqlx::query_as::<_, UserModel>(
        //     "insert into users (user_id, password_hash, email, created_at, modified_at) values($1, $2, $3, $4, $5) returning *",
        // )
        // .bind(nanoid_gen(self.settings.nanoid_length.expect("Settings not set.")))
        // .bind(request.password_hash)
        // .bind(request.email)
        // .bind(chrono::Utc::now().to_string())
        // .bind(chrono::Utc::now().to_string())
        // .fetch_one(&self.pool)
        // .await?;

        return Ok(user);
    }

    pub async fn get_user_by_email(&self, email: String) -> anyhow::Result<UserModel> {
        // let result = sqlx::query_as::<_, UserModel>(
        //     "select email, password_hash from users where users.email = $1",
        // )
        // .bind(email)
        // .fetch_one(&self.pool)
        // .await?;
        info!("Search for user with email: {email:?}");

        // let row_result = sqlx::query("select * from users where email = $1")
        //     .bind(email)
        //     .fetch_one(&self.pool)
        //     .await?;

        let res = UserModel::select()
            .where_bind("email = ", email)
            .fetch_one(&self.pool)
            .await?;

        info!("Found user");
        // let result = UserModel::from_row(&row_result).expect("Could not turn row into user model");

        info!("Successfully found user");
        return Ok(res);
    }

    pub async fn get_survey(&self, survey_id: &String) -> anyhow::Result<Option<SurveyModel>> {
        // let result =
        //     sqlx::query_as::<_, SurveyModel>("select * from surveys where surveys.survey_id = $1")
        //         .bind(survey_id)
        //         .fetch_one(&self.pool)
        //         .await?;
        let result = SurveyModel::select()
            .where_bind("survey_id = ?", survey_id)
            .fetch_one(&self.pool)
            .await?;
        // let survey = parse_markdown_v3(result.plaintext);
        Ok(Some(result))
    }

    pub async fn create_survey(&self, survey: SurveyModel) -> anyhow::Result<SurveyModel> {
        // let partial_survey = parse_markdown_v3(payload.plaintext.clone());
        // // let survey = Survey::from(partial_survey);
        // // let survey = Survey::from(payload.plaintext.clone());
        // // let response_survey = survey.clone();
        // let now = chrono::offset::Utc::now();
        // let nowstr = now.to_string();
        // let _res = sqlx::query!(
        //     r#"insert into surveys (plaintext, user_id, created_at, modified_at, version, parse_version)
        //     values
        //     ($1, $2, $3, $4, $5, $6)
        //     "#,
        //     survey.plaintext,
        //     survey.user_id,
        //     survey.created_at,
        //     survey.modified_at,
        //     survey.version,
        //     survey.parse_version
        // ).execute(&self.pool).await?;

        let res = survey.insert(&self.pool).await?;

        // let res = Survey:

        // let rows = _res.rows_affected();
        // println!("create survey rows affected={rows}");
        info!("Successfully created a new survey");

        Ok(res)
    }

    pub async fn create_answer(&self, answer: CreateAnswersModel) -> anyhow::Result<()> {
        info!("Creating answers in database");

        // let res = sqlx::query(
        //     r#"insert into surveys_submissions (survey_id, submitted_at, answers)
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

        answer.insert(&self.pool).await?;

        // info!("created rows={}", res.rows_affected());

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    // use dotenvy::dotenv;

    // use crate::{database::Database, todo};

    // #[tokio::test]
    // async fn test_create_table() -> anyhow::Result<()> {
    //     let mut db = Database::new(true).await?;

    //     let res = sqlx::query("insert into todos(description, status) values ($1, $2)")
    //         .bind("this is a test")
    //         .bind("done")
    //         .execute(&mut db.pool)
    //         .await?;

    //     println!("result: {:?}", res);

    //     let todos = sqlx::query_as::<_, todo::TodoModel>("select * from todos")
    //         .fetch_all(&mut db.pool)
    //         .await?;

    //     println!("query result: {:?}", todos);
    //     Ok(())
    // }

    // use db::{database::Database, models::CreateSurveyRequest};

    // use crate::{database::Database, models::CreateSurveyRequest};
}
