use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

use crate::cfg::CONFIG;

pub async fn init_db() -> sqlx::Result<SqlitePool> {
    let db_url = &CONFIG.database_url;
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        println!("Creating database {}", &db_url);
        match Sqlite::create_database(&db_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    let pool = SqlitePool::connect(&db_url).await?;

    Ok(pool)
}
