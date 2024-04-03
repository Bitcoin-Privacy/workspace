use async_trait::async_trait;
use secp256k1::{PublicKey, SecretKey};

mod source;
pub use source::StatechainRepo;
pub type StatechainError = String;
pub type StatechainResult<T> = Result<T, StatechainError>;

#[async_trait]
pub trait TraitStatechainRepo: Send + Sync + 'static {
    async fn create_deposit_tx(
        &self,
        token_id: &str,
        auth_pubkey: &PublicKey,
        server_pubkey: &PublicKey,
        server_privkey: &SecretKey,
        statechain_id: &str,
        amount: u32,
    ) -> StatechainResult<()>;
}
