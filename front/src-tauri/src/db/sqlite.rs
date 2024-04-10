use anyhow::Result;
use bitcoin::hex::{Case, DisplayHex};
use secp256k1::{PublicKey, SecretKey};
use sqlx::{migrate::MigrateDatabase, query, sqlite::SqliteQueryResult, Row, Sqlite, SqlitePool};

use crate::{cfg::CFG, model::StateCoin};

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
pub async fn get_statecoin_by_id(pool: &SqlitePool, statechain_id: &str) -> Result<StateCoin> {
    let row = sqlx::query_as::<_,StateCoin>(r#"select statechain_id, deriv, amount, owner_pubkey, owner_seckey, funding_txid, funding_vout, status, funding_tx from StateCoin where statechain_id = $1 "#).bind(statechain_id).fetch_one(pool).await.unwrap();

    Ok(row)
}

pub async fn get_seckey_by_id(pool: &SqlitePool, statechain_id: &str) -> Result<Option<String>> {
    let row = sqlx::query("select owner_seckey from StateCoin where statechain_id = $1")
        .bind(&statechain_id)
        .fetch_optional(pool)
        .await?;

    let val = match row {
        Some(r) => Some(r.try_get::<String, _>("owner_seckey")?),
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

pub async fn update_deposit_tx(
    pool: &SqlitePool,
    statechain_id: &str,
    funding_txid: &str,
    funding_vout: u64,
    status: &str,
    funding_tx: &str,
) -> Result<SqliteQueryResult, sqlx::Error> {
    let vout_i64: i64 = funding_vout as i64;
    let res = sqlx::query(r#"UPDATE StateCoin SET funding_txid = $1, funding_vout = $2, funding_tx = $3, status = $4  WHERE statechain_id = $5"#)
        .bind(funding_txid)
        .bind(vout_i64)
        .bind(funding_tx)
        .bind(status)
        .bind(statechain_id)
        .execute(pool)
        .await;
    match res {
        Ok(result) => Ok(result),
        Err(err) => {
            println!("error when update database {}", err.to_string());
            Err(err)
        }
    }
}
