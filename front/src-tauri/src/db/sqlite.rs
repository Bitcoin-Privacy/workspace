use anyhow::Result;
use bitcoin::hex::{Case, DisplayHex};
use secp256k1::{PublicKey, SecretKey};
use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, Row, Sqlite, SqlitePool};

use crate::cfg::CFG;

pub async fn init_db() -> sqlx::Result<SqlitePool> {
    let db_url = &CFG.database_url;
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

pub async fn insert_statecoin(
    pool: &SqlitePool,
    statechain_id: &str,
    deriv: &str,
    amount: u64,
    auth_seckey: &SecretKey,
    auth_pubkey: &PublicKey,
    aggregated_pubkey: &str,
    aggregated_address: &str,
    owner_seckey: &SecretKey,
    owner_pubkey: &PublicKey,
) -> Result<SqliteQueryResult, sqlx::Error> {
    let amount_i64: i64 = amount as i64;
    let owner_seckey_bytes = owner_seckey.secret_bytes().to_hex_string(Case::Lower);
    let owner_pubkey_bytes = owner_pubkey.to_string();

    let auth_seckey_bytes = auth_seckey.secret_bytes().to_hex_string(Case::Lower);
    let auth_pubkey_bytes = auth_pubkey.to_string();

    let res = sqlx::query(
        r#"INSERT INTO StateCoin (statechain_id, deriv, amount,aggregated_pubkey, aggregated_address, auth_pubkey, auth_seckey, owner_pubkey,owner_seckey) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)"#)
        .bind(statechain_id)
        .bind(deriv)
        .bind(amount_i64)
        .bind(aggregated_pubkey)
        .bind(aggregated_address)
        .bind(auth_seckey_bytes)
        .bind(auth_pubkey_bytes)
        .bind(owner_pubkey_bytes)
        .bind(owner_seckey_bytes)
        .execute(pool)
        .await;

    match res {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}
