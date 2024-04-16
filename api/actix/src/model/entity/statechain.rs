// ---------------------------
// Statecoin table
// ---------------------------
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct StateCoin {
    pub id: uuid::Uuid,
    pub token_id: String,
    pub auth_xonly_public_key: String,
    pub server_public_key: String,
    pub server_private_key: String,
    #[sqlx(try_from = "i64")]
    pub amount: u32,
    pub sec_nonce: Option<String>,
    pub pub_nonce: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Pubnonce {
    pub pub_nonce: String,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct AuthPubkey {
    pub auth_xonly_public_key: String,
}
