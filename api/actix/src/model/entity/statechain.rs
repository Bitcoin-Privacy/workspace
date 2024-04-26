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
    #[sqlx(try_from = "i64")]
    pub sequence: u32,
    pub sec_nonce: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct AuthPubkey {
    pub auth_xonly_public_key: String,
}
