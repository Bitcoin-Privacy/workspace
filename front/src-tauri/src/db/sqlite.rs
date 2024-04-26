use anyhow::Result;
use bitcoin::{
    hex::{Case, DisplayHex},
    XOnlyPublicKey,
};
use secp256k1::{PublicKey, SecretKey};
use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, Row, Sqlite, SqlitePool};

use crate::{cfg::CFG, model::StateCoin, model::StateCoinInfo};

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
    let row = sqlx::query_as::<_,StateCoin>(r#"select signed_statechain_id, aggregated_pubkey, aggregated_address, funding_txid, funding_vout, key_agg_ctx, amount, deriv from StateCoin where statechain_id = $1"#).bind(statechain_id).fetch_one(pool).await.unwrap();
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

pub async fn get_statecoins_by_account(
    pool: &SqlitePool,
    account: &str,
) -> Result<Vec<StateCoinInfo>> {
    let statecoins = sqlx::query_as::<_,StateCoinInfo>("select statechain_id, aggregated_address, amount, funding_txid, funding_vout, n_lock_time from StateCoin where deriv = $1").bind(account).fetch_all(pool).await?;
    Ok(statecoins)
}

pub async fn create_statecoin(
    pool: &SqlitePool,
    statechain_id: &str,
    signed_statechain_id: &str,
    deriv: &str,
    amount: i64,
    auth_seckey: &str,
    auth_pubkey: &str,
    aggregated_pubkey: &str,
    aggregated_address: &str,
    owner_seckey: &str,
    owner_pubkey: &str,
    key_agg_ctx: &str,
) -> Result<SqliteQueryResult, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO StateCoin (statechain_id, signed_statechain_id, deriv, amount,aggregated_pubkey, aggregated_address, auth_pubkey, auth_seckey, owner_pubkey,owner_seckey, key_agg_ctx) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)"#)
        .bind(statechain_id)
        .bind(signed_statechain_id)
        .bind(deriv)
        .bind(amount)
        .bind(aggregated_pubkey)
        .bind(aggregated_address)
        .bind(auth_seckey)
        .bind(auth_pubkey)
        .bind(owner_pubkey)
        .bind(owner_seckey)
        .bind(key_agg_ctx)
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
    funding_tx: &str,
) -> Result<SqliteQueryResult, sqlx::Error> {
    let vout_i64: i64 = funding_vout as i64;
    let res = sqlx::query(r#"UPDATE StateCoin SET funding_txid = $1, funding_vout = $2, funding_tx = $3  WHERE statechain_id = $4"#)
        .bind(funding_txid)
        .bind(vout_i64)
        .bind(funding_tx)
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

pub async fn update_bk_tx(
    pool: &SqlitePool,
    statechain_id: &str,
    backup_tx: &str,
    agg_pubnonce: &str,
) -> Result<SqliteQueryResult, sqlx::Error> {
    match sqlx::query(
        "update StateCoin set backup_tx = $1, agg_pubnonce = $2 where statechain_id = $3",
    )
    .bind(backup_tx)
    .bind(agg_pubnonce)
    .bind(statechain_id)
    .execute(pool)
    .await
    {
        Ok(result) => Ok(result),
        Err(err) => {
            println!("error when update database {}", err.to_string());
            Err(err)
        }
    }
}
