use std::env;
use sqlx::sqlite::SqlitePool;
use env::var;
use sqlx::Error;

pub async fn establish_db_connection() -> Result<SqlitePool, Error> {
    let db_url = var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePool::connect(&db_url).await
}