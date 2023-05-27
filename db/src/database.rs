use std::{collections::HashMap, str::FromStr};

use anyhow::{self, Context};

use markdownparser::{nanoid_gen, parse_markdown_v3, Survey};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
// use models::CreateAnswersModel;
// use chrono::Local;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    ConnectOptions, FromRow, Row, SqliteConnection, SqlitePool,
};
use tracing::{info, instrument};

use crate::models::{self, CreateAnswersModel, CreateSurveyRequest, SurveyModel};

// mod models;

#[derive(Debug, Clone)]
pub struct Database {
    // data: Arc<RwLock<TodoModel>>,
    pub pool: SqlitePool,
    // pool: SqliteConnection,
    // pub pool: SqlitePool,
    // options: Option<DatabaseOptions>,
    pub settings: Settings,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Settings {
    pub base_path: Option<String>,
}

impl Settings {
    fn default() -> Settings {
        Settings { base_path: None }
    }
}

// pub type InsertTodosRequest = Vec<todo::TodoModel>;

impl Database {
    #[instrument]
    pub async fn new(in_memory: bool) -> anyhow::Result<Self> {
        println!("{:?}", std::env::current_dir()); //Ok("/Users/jarde/Documents/code/markdownparser/server")
        let database_url = dotenvy::var("DATABASE_URL")?;

        let mut pool = match in_memory {
            true => {
                info!("Creating in-memory database");

                let conn = SqliteConnectOptions::from_str("sqlite::memory:")?
                    .journal_mode(SqliteJournalMode::Wal)
                    .read_only(false)
                    .create_if_missing(true);
                SqlitePool::connect_with(conn).await?

                // let connection_options = SqliteConnectOptions::new()
                //     .create_if_missing(true)
                //     // .filename("~/Library/todowatcher_data.db"); // maybe try to save state in a common location
                //     .filename(database_url);
                // PgPool::connect(&database_url).await?
            }
            false => {
                info!("Creating new database");
                let conn = SqliteConnectOptions::from_str(database_url.as_str())?
                    .journal_mode(SqliteJournalMode::Wal)
                    .read_only(false)
                    .create_if_missing(true);
                SqlitePool::connect_with(conn).await?

                // let connection_options = SqliteConnectOptions::new()
                //     .create_if_missing(true)
                //     // .filename("~/Library/todowatcher_data.db"); // maybe try to save state in a common location
                //     .filename(database_url);
                // SqlitePool::connect_with(connection_options).await?
                // PgPool::connect(&database_url).await?
            }
        };

        sqlx::migrate!().run(&pool).await?;

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

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserModel {
    pub user_id: String,
    pub email: String,
    pub password_hash: String,
    // pub user_id: String,
}

pub struct CreateUserRequest {
    pub email: String,
    pub password_hash: String,
}

impl Database {
    pub async fn create_user(&self, request: CreateUserRequest) -> anyhow::Result<UserModel> {
        println!("->> create_user");
        let result = sqlx::query_as::<_, UserModel>(
            "insert into users (id, password_hash, email, created_at, modified_at) values($1, $2, $3, $4, $5) returning *",
        )
        .bind(nanoid_gen())
        .bind(request.password_hash)
        .bind(request.email)
        .bind(chrono::Utc::now().to_string())
        .bind(chrono::Utc::now().to_string())
        .fetch_one(&self.pool)
        .await?;

        return Ok(result);
    }

    pub async fn get_user_by_email(&self, email: String) -> anyhow::Result<UserModel> {
        let result = sqlx::query_as::<_, UserModel>(
            "select email, password_hash from users where users.email = $1",
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        return Ok(result);
    }

    pub async fn get_survey(&self, survey_id: &String) -> anyhow::Result<Option<SurveyModel>> {
        let result =
            sqlx::query_as::<_, SurveyModel>("select * from surveys where surveys.id = $1")
                .bind(survey_id)
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
        let _res = sqlx::query!(
            r#"insert into surveys (id, plaintext, user_id, created_at, modified_at, version, parse_version)
            values 
            ($1, $2, $3, $4, $5, $6, $7)
            "#,
            survey.id,
            survey.plaintext,
            survey.user_id,
            survey.created_at,
            survey.modified_at,
            survey.version,
            survey.parse_version
        ).execute(&self.pool).await?;

        let rows = _res.rows_affected();
        println!("create survey rows affected={rows}");

        Ok(survey)
    }

    pub async fn create_answer(&self, answer: CreateAnswersModel) -> anyhow::Result<()> {
        info!("Creating answers in database");

        let res = sqlx::query(
            r#"insert into answers (answer_id, survey_id, survey_version, answers, created_at)
        values
        ($1, $2, $3, $4, $5)
        "#,
        )
        .bind(answer.answer_id)
        .bind(answer.survey_id)
        .bind(answer.survey_version)
        .bind(json!(answer.answers))
        .bind(answer.start_time)
        .execute(&self.pool)
        .await
        .unwrap();

        info!("created rows={}", res.rows_affected());
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

    use crate::{database::Database, models::CreateSurveyRequest};
}
