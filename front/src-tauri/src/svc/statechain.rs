use anyhow::anyhow;
use anyhow::Result;
use bitcoin::absolute::LockTime;
use bitcoin::consensus;

use bitcoin::hex::DisplayHex;
use bitcoin::transaction;
use bitcoin::Amount;
use bitcoin::OutPoint;
use bitcoin::ScriptBuf;
use bitcoin::Sequence;
use bitcoin::TapSighash;
use bitcoin::TapSighashType;
use bitcoin::Transaction;
use bitcoin::TxIn;
use bitcoin::TxOut;
use bitcoin::Txid;
use bitcoin::Witness;
use bitcoin::{
    hashes::sha256,
    secp256k1::{rand, Keypair, PublicKey, Secp256k1, SecretKey},
    Address, Network,
};
use musig2::AggNonce;
use musig2::BinaryEncoding;
use musig2::KeyAggContext;

use musig2::PartialSignature;
use musig2::PubNonce;
use musig2::SecNonce;
use rand::RngCore;

use secp256k1::Parity;
use secp256k1::{schnorr::Signature, Message, Scalar};

use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::cfg::BASE_TX_FEE;
use crate::model::StatechainKeypairs;
use crate::model::Statecoin;
use crate::model::StatecoinDetail;
use crate::{
    api::statechain,
    db::PoolWrapper,
    model::{AccountActions, StatecoinCard, TransferStateCoinInfo},
};

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

pub async fn list_statecoins(pool: &PoolWrapper, deriv: &str) -> Result<Vec<StatecoinCard>> {
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

pub fn generate_auth_owner_keypairs() -> Result<StatechainKeypairs> {
    let secp = Secp256k1::new();

    let auth_keypair = Keypair::new(&secp, &mut rand::thread_rng());
    let auth_pubkey = PublicKey::from_keypair(&auth_keypair);
    let auth_seckey = SecretKey::from_keypair(&auth_keypair);

    let mut owner_seckey = SecretKey::new(&mut rand::thread_rng());
    let (_, parity) = PublicKey::from_secret_key(&secp, &owner_seckey).x_only_public_key();

    if parity == Parity::Odd {
        owner_seckey = owner_seckey.negate();
    }

    let owner_pubkey = PublicKey::from_secret_key(&secp, &owner_seckey);

    Ok(StatechainKeypairs {
        owner_seckey,
        owner_pubkey,
        auth_seckey,
        auth_pubkey,
    })
}

pub async fn get_statecoin_detail_by_id(
    pool: &PoolWrapper,
    statechain_id: &str,
) -> Result<StatecoinDetail> {
    let statecoin = pool.get_statecoin_detail_by_id(statechain_id).await?;

    Ok(statecoin)
}

pub async fn withdraw_statecoin(
    conn: &NodeConnector,
    pool: &PoolWrapper,
    statechain_id: &str,
    deriv: &str,
) -> Result<String> {
    let (account, _) = account::get_account(deriv).unwrap();
    let account_address = account.get_addr();
    let account_address = Address::from_str(&account_address)?;
    let account_address_checked = account_address.require_network(Network::Testnet)?;
    let statecoin = pool.get_statecoin_by_id(statechain_id).await?;
    let withdraw_tx =
        create_withdraw_tx(conn, statechain_id, &statecoin, &account_address_checked).await?;

    println!("withdraw tx: {}", withdraw_tx);

    let res = statechain::broadcast_tx(withdraw_tx).await?;
    println!("broad cast transaction tx: {:?}", res);
    //pool.delete_statecoin_by_statechain_id(statechain_id).await?;
    Ok(res)
}

pub async fn create_withdraw_tx(
    conn: &NodeConnector,
    statechain_id: &str,
    statecoin: &Statecoin,
    receiver_address: &Address,
) -> Result<String> {
    let amount = statecoin.amount as u64;
    let agg_pubkey = PublicKey::from_str(&statecoin.aggregated_pubkey)?;
    let vout = 0 as i64;
    let key_agg_ctx = KeyAggContext::from_hex(&statecoin.key_agg_ctx).unwrap();
    let secp = Secp256k1::new();
    let seckey = &statecoin.owner_seckey;
    let seckey = SecretKey::from_str(seckey).unwrap();
    let agg_scriptpubkey = ScriptBuf::new_p2tr(&secp, agg_pubkey.x_only_public_key().0, None);
    let scriptpubkey = agg_scriptpubkey.to_hex_string();

    let prev_outpoint = OutPoint {
        txid: Txid::from_str(&statecoin.funding_txid)?,
        vout: vout as u32,
    };

    let input = TxIn {
        previous_output: prev_outpoint,
        script_sig: ScriptBuf::default(),
        sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
        witness: Witness::default(),
    };

    let spend = TxOut {
        value: Amount::from_sat(amount - BASE_TX_FEE),
        script_pubkey: receiver_address.script_pubkey(),
    };

    let mut unsigned_tx = Transaction {
        version: transaction::Version::TWO, // Post BIP-68.
        lock_time: LockTime::ZERO,          // Ignore the locktime.
        input: vec![input],                 // Input goes into index 0.
        output: vec![spend],                // Outputs, order does not matter.
    };

    let sighash_type = TapSighashType::Default;
    let get_nonce_res =
        statechain::get_nonce(&conn, statechain_id, &statecoin.signed_statechain_id).await?;
    let server_pubnonce = get_nonce_res.server_nonce;
    let mut nonce_seed = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut nonce_seed);

    let secnonce = SecNonce::build(nonce_seed).with_seckey(seckey).build();

    let our_public_nonce = secnonce.public_nonce();

    let public_nonces = [
        our_public_nonce,
        server_pubnonce.parse::<PubNonce>().unwrap(),
    ];

    let agg_pubnonce: AggNonce = public_nonces.iter().sum();

    let agg_pubnonce_str = agg_pubnonce.to_string();

    let serialized_key_agg_ctx = key_agg_ctx
        .to_bytes()
        .to_hex_string(bitcoin::hex::Case::Lower);

    let unsigned_tx_hex = consensus::encode::serialize_hex(&unsigned_tx);

    let get_sign_res = statechain::get_partial_signature_for_bk(
        &conn,
        &serialized_key_agg_ctx,
        &statechain_id,
        &statecoin.signed_statechain_id,
        &unsigned_tx_hex,
        &agg_pubnonce_str,
        &scriptpubkey,
    )
    .await?;

    let sighash = &get_sign_res.sighash;
    let sighash = TapSighash::from_str(sighash)?;
    let msg = Message::from(sighash);
    let msg = msg.as_ref();
    let our_partial_signature: PartialSignature =
        musig2::sign_partial(&key_agg_ctx, seckey, secnonce, &agg_pubnonce, msg)
            .expect("error creating partial signature");

    let server_signature = get_sign_res.partial_sig;

    let partial_signatures = [
        our_partial_signature,
        PartialSignature::from_hex(&server_signature).unwrap(),
    ];

    let agg_pubkey_tw: PublicKey = key_agg_ctx.aggregated_pubkey();
    println!("tx tweaked public key : {}", agg_pubkey_tw.to_string());

    for (i, partial_signature) in partial_signatures.into_iter().enumerate() {
        if i == 0 {
            // Don't bother verifying our own signature
            continue;
        }

        let their_pubkey: PublicKey = key_agg_ctx.get_pubkey(i).unwrap();
        let their_pubnonce = &public_nonces[i];

        musig2::verify_partial(
            &key_agg_ctx,
            partial_signature,
            &agg_pubnonce,
            their_pubkey,
            their_pubnonce,
            msg,
        )
        .expect("received invalid signature from a peer");
    }

    let final_signature: secp256k1::schnorr::Signature =
        musig2::aggregate_partial_signatures(&key_agg_ctx, &agg_pubnonce, partial_signatures, msg)
            .expect("error aggregating signatures");

    musig2::verify_single(agg_pubkey_tw, final_signature, msg)
        .expect("aggregated signature must be valid");

    let signature = bitcoin::taproot::Signature {
        sig: final_signature,
        hash_ty: sighash_type,
    };

    let mut wit = Witness::new();
    wit.push(signature.to_vec());

    unsigned_tx.input[0].witness = wit;

    let tx_hex = consensus::encode::serialize_hex(&unsigned_tx);

    Ok(tx_hex)
}
