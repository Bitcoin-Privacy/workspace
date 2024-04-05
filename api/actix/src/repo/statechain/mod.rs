use anyhow::Result;
use async_trait::async_trait;
use bitcoin::secp256k1::{PublicKey, SecretKey, XOnlyPublicKey};

mod source;
pub use source::StatechainRepo;

use crate::model::entity::statechain::StateCoin;

#[async_trait]
pub trait TraitStatechainRepo: Send + Sync + 'static {
    async fn create_deposit_tx(
        &self,
        token_id: &str,
        auth_pubkey: &PublicKey,
        server_pubkey: &PublicKey,
        server_privkey: &SecretKey,
        amount: u32,
    ) -> Result<StateCoin>;
}
