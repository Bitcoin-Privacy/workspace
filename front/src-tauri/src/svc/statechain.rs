use anyhow::anyhow;
use anyhow::Result;
use bitcoin::key::TapTweak;
use bitcoin::{
    absolute::{self, LockTime},
    consensus,
    hashes::sha256,
    hex::DisplayHex,
    secp256k1::{rand, Keypair, PublicKey, Secp256k1, SecretKey},
    sighash::{Prevouts, SighashCache},
    transaction::{self, Version},
    Address, Amount, EcdsaSighashType, Network, OutPoint, ScriptBuf, Sequence, TapSighashType,
    Transaction, TxIn, TxOut, Txid, Witness, XOnlyPublicKey,
};
use musig2::{AggNonce, BinaryEncoding, KeyAggContext, PartialSignature, PubNonce, SecNonce};

use rand::RngCore;
use secp256k1::{schnorr::Signature, Message, Scalar};

use std::{
    ops::ControlFlow,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::model::StatechainKeypairs;
use crate::{
    api::statechain,
    cfg::{BASE_TX_FEE, INTERVAL},
    db::PoolWrapper,
    model::{AccountActions, StateCoin, StateCoinInfo, TransferStateCoinInfo},
};
use shared::intf::statechain::{DepositInfo, DepositReq, DepositRes, TransferMessage};

use crate::connector::NodeConnector;

use super::account;

pub fn sign_message(msg: &str, seckey: &SecretKey) -> Signature {
    let secp = Secp256k1::new();
    let message = Message::from_hashed_data::<sha256::Hash>(msg.to_string().as_bytes());
    let keypair = Keypair::from_seckey_slice(&secp, seckey.as_ref()).unwrap();
    let signed_message = secp.sign_schnorr(&message, &keypair);

    signed_message
}

pub fn aggregate_pubkeys(
    owner_pubkey: PublicKey,
    se_pubkey: PublicKey,
) -> (PublicKey, PublicKey, Address, KeyAggContext) {
    let secp = Secp256k1::new();
    let mut pubkeys: Vec<PublicKey> = vec![];
    pubkeys.push(owner_pubkey);
    pubkeys.push(se_pubkey);
    let key_agg_ctx_tw = KeyAggContext::new(pubkeys.clone())
        .unwrap()
        .with_unspendable_taproot_tweak()
        .unwrap();

    let aggregated_pubkey: PublicKey = key_agg_ctx_tw.aggregated_pubkey_untweaked();
    let aggregated_pubkey_tw: PublicKey = key_agg_ctx_tw.aggregated_pubkey();

    let aggregated_address = Address::p2tr(
        &secp,
        aggregated_pubkey.x_only_public_key().0,
        None,
        Network::Testnet,
    );

    (
        aggregated_pubkey,
        aggregated_pubkey_tw,
        aggregated_address,
        key_agg_ctx_tw,
    )
}

pub fn compute_t1(owner_seckey: &SecretKey, random_key: &Scalar) -> SecretKey {
    let res = owner_seckey.add_tweak(random_key).unwrap();
    res
}

pub async fn list_statecoins(pool: &PoolWrapper, deriv: &str) -> Result<Vec<StateCoinInfo>> {
    let (account, _) = account::get_account(deriv).unwrap();
    let account_address = account.get_addr();

    match pool.list_statecoins_by_account(&account_address).await {
        Ok(statecoins) => Ok(statecoins),
        Err(e) => Err(e),
    }
}

pub async fn list_transfer_statecoins(
    conn: &NodeConnector,
    pool: &PoolWrapper,
    deriv: &str,
) -> Result<Vec<TransferStateCoinInfo>> {
    let (account, _) = account::get_account(deriv).unwrap();
    let account_address = account.get_addr();

    let authkeys: Vec<String> = pool.list_authkeys_by_account(&account_address).await?;

    let mut transfer_statecoins = Vec::new();

    // Iterate over authkeys and fetch transfer messages asynchronously
    for authkey in authkeys {
        match statechain::get_transfer_msg(conn, &authkey).await {
            Ok(transfer_msg_res) => {
                // Check if the transfer message is null or not
                if let Some(transfer_msg) = transfer_msg_res {
                    println!("authkey : {}", authkey);
                    transfer_statecoins.push(TransferStateCoinInfo {
                        auth_key: authkey,
                        transfer_message: transfer_msg.transfer_message,
                    });
                } else {
                    // Handle the case when the transfer message is null (None)
                    println!("Transfer message is null for authkey: {}", authkey);
                    // You can choose to skip or handle this case accordingly
                }
            }
            Err(err) => {
                // Handle the error
                println!(
                    "Error fetching transfer message for authkey {}: {:?}",
                    authkey, err
                );
                // You can choose to skip or handle this case accordingly
            }
        }
    }
    Ok(transfer_statecoins)
}

pub fn calculate_nlocktime_for_bk(num_owner: u64, init_nlocktime: u64) -> Result<u64> {
    let current_time = SystemTime::now();

    // Calculate the Unix time by subtracting the UNIX epoch time
    let current_unix_time = current_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let new_nlocktime = init_nlocktime - (num_owner - 1) * INTERVAL;
    if current_unix_time >= new_nlocktime {
        return Ok(0);
    }

    Ok(new_nlocktime)
}

pub fn generate_auth_owner_keypairs() -> Result<StatechainKeypairs> {
    let secp = Secp256k1::new();

    let auth_keypair = Keypair::new(&secp, &mut rand::thread_rng());
    let auth_pubkey = PublicKey::from_keypair(&auth_keypair);
    let auth_seckey = SecretKey::from_keypair(&auth_keypair);
    let owner_keypair = Keypair::new(&secp, &mut rand::thread_rng());
    let tweaked_owner_keypair = owner_keypair.tap_tweak(&secp, None);
    let owner_seckey = SecretKey::from_keypair(&tweaked_owner_keypair.to_inner());
    let owner_pubkey = PublicKey::from_keypair(&tweaked_owner_keypair.to_inner());

    Ok(StatechainKeypairs {
        owner_seckey,
        owner_pubkey,
        auth_seckey,
        auth_pubkey,
    })
}
