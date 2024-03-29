use anyhow::Result;
use sqlx::{migrate::MigrateDatabase, Row, Sqlite, SqlitePool};

use crate::cfg::CONFIG;

pub async fn init_db() -> sqlx::Result<SqlitePool> {
    let db_url = &CONFIG.database_url;
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        println!("Creating sqlite database {}", &db_url);
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

pub async fn set_cfg(pool: &SqlitePool, key: &str, value: &str) -> Result<()> {
    let _ = sqlx::query(
        r#"insert into Config (key, value)
            values(?, ?) on conflict (key)
            do update set value = excluded.value;"#,
    )
    .bind(key)
    .bind(value)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_cfg(pool: &SqlitePool, key: &str) -> Result<Option<String>> {
    let row = sqlx::query(r#"select value from Config where key = ?;"#)
        .bind(key)
        .fetch_optional(pool)
        .await?;
    let val = match row {
        Some(r) => Some(r.try_get::<String, _>("value")?),
        None => None,
    };
    Ok(val)
}
