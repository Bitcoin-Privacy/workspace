// ---------------------------
// Statecoin table
// ---------------------------
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct StateCoin {
    pub id: uuid::Uuid,
    pub server_public_key: String,
    pub server_private_key: String,
    #[sqlx(try_from = "i64")]
    pub amount: u32,
    #[sqlx(try_from = "i64")]
    pub txn: u32,
    #[sqlx(try_from = "i64")]
    pub n_lock_time: u64,
    pub sec_nonce: Option<String>,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct AuthPubkey {
    pub authkey: String,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct StatecoinVerificationInfo {
    #[sqlx(try_from = "i64")]
    pub txn: u32,
    pub server_public_key: String,
    pub random_point: String,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct StatecoinSecret {
    pub server_private_key: String,
    pub random_key: String,
}
