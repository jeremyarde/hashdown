use std::{
    collections::HashSet,
    env,
    path::PathBuf,
    ptr::null,
    sync::{Arc, RwLock},
};

use anyhow;

// use chrono::Local;
use sqlx::{
    database, migrate::MigrateDatabase, sqlite::SqliteConnectOptions, ConnectOptions, Connection,
    Execute, Pool, QueryBuilder, Sqlite, SqliteConnection, SqlitePool,
};
use tracing::info;

// use crate::todo::{self, TodoModel};

#[derive(Debug)]
pub struct Database {
    // data: Arc<RwLock<TodoModel>>,
    // pool: PgPool,
    // pool: SqliteConnection,
    pool: SqlitePool,
    // options: Option<DatabaseOptions>,
    settings: Settings,
}

#[derive(sqlx::FromRow, Debug)]
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
    pub async fn new(in_memory: bool) -> anyhow::Result<Self> {
        // let database_url = dotenvy::var("DATABASE_URL")?.as_str();
        // let pool = PgPool::connect(&database_url).await?;
        let mut pool = match in_memory {
            true => {
                info!("Creating in-memory database");
                SqlitePool::connect("sqlite::memory:").await?
            }
            false => {
                info!("Creating new database");
                let connection_options = SqliteConnectOptions::new()
                    .create_if_missing(true)
                    // .filename("~/Library/todowatcher_data.db"); // maybe try to save state in a common location
                    .filename("./todowatcher_data.db");
                SqlitePool::connect_with(connection_options).await?
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

    // pub async fn create_settings(&mut self, folder: String) -> anyhow::Result<Settings> {
    //     let res = sqlx::query_as::<_, Settings>(
    //         "insert into settings (base_path) values ($1) returning *",
    //     )
    //     .bind(folder)
    //     .fetch_one(&mut self.pool)
    //     .await?;

    //     Ok(res)
    // }

    pub fn get_home(&self) -> Option<String> {
        return self.settings.base_path.clone();
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
