use std::str::FromStr;

use anyhow;

use markdownparser::{parse_markdown_v3, Survey};
// use models::CreateAnswersModel;
// use chrono::Local;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    ConnectOptions, SqliteConnection, SqlitePool,
};
use tracing::{info, instrument};

use crate::models::{self, nanoid_gen, CreateSurveyRequest};

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

impl Database {
    pub async fn create_survey(&self, payload: CreateSurveyRequest) -> anyhow::Result<Survey> {
        let survey = parse_markdown_v3(payload.plaintext.clone());
        // let survey = Survey::from(payload.plaintext.clone());
        let response_survey = survey.clone();
        let now = chrono::offset::Utc::now();
        let nowstr = now.to_string();
        let _res = sqlx::query!(
            r#"insert into surveys (id, plaintext, user_id, created_at, modified_at, version, parse_version)
            values 
            ($1, $2, $3, $4, $5, $6, $7)
            "#,
            response_survey.id,
            payload.plaintext,
            survey.user_id,
            survey.created_at,
            survey.modified_at,
            "1",
            nowstr
        ).execute(&self.pool).await?;

        // let response = CreateSurveyResponse {
        //     survey: Survey::from(response_survey),
        //     // metadata: res,
        // };

        // Ok(Survey {
        //     id: nanoid_gen(),
        //     plaintext: payload.plaintext,
        //     user_id: String::from("something"),
        //     created_at: now.to_string(),
        //     modified_at: now.to_string(),
        //     version: String::from("versionhere"),
        //     parse_version: String::from("parseversion"),
        //     questions: vec![],
        // })
        Ok(survey)
    }

    pub async fn test_survey(&self, payload: CreateSurveyRequest) -> anyhow::Result<Survey> {
        let survey = parse_markdown_v3(payload.plaintext.clone());
        // let survey = Survey::from(payload.plaintext.clone());
        let response_survey = survey.clone();
        let now = chrono::offset::Utc::now();
        let nowstr = now.to_string();

        // let response = CreateSurveyResponse {
        //     survey: Survey::from(response_survey),
        //     // metadata: res,
        // };

        // Ok(Survey {
        //     id: nanoid_gen(),
        //     plaintext: payload.plaintext,
        //     user_id: String::from("something"),
        //     created_at: now.to_string(),
        //     modified_at: now.to_string(),
        //     version: String::from("versionhere"),
        //     parse_version: String::from("parseversion"),
        //     questions: vec![],
        // })

        Ok(survey)
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
}
