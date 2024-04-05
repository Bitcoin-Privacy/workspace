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
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
