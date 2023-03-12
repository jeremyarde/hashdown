use ormlite::model::*;

use db::Database;

#[derive(Model, Debug)]
pub struct TestingPerson {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

mod db;
// use db;

#[tokio::main]
async fn main() {
    let database = Database::new(false).await?;

    let results = TestingPerson::builder()
        .id(11)
        .name("tom")
        .age(111)
        .insert(&database.pool)
        .await;

    println!("{:?}", results);
}
