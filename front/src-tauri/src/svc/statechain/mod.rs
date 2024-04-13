use anyhow::{anyhow, Result};
use bitcoin::{
    absolute, consensus,
    key::{Keypair, TapTweak, TweakedKeypair},
    secp256k1::{rand, schnorr::Signature, PublicKey, Secp256k1, SecretKey},
    sighash::{Prevouts, SighashCache},
    transaction::Version,
    Address, Amount, Network, OutPoint, ScriptBuf, Sequence, TapSighashType, Transaction, TxIn,
    TxOut, Txid, Witness,
};

use curve25519_dalek::Scalar;
use secp256k1::Message;
use statechain_core::deposit::AggregatedPublicKey;

use std::str::FromStr;

use crate::{api::statechain, db::PoolWrapper, model::AccountActions};
use shared::intf::statechain::{DepositReq, DepositRes};

use crate::connector::NodeConnector;

use super::account;

mod util;

pub async fn deposit(
    pool: &PoolWrapper,
    conn: &NodeConnector,
    deriv: &str,
    amount: u64,
) -> Result<AggregatedPublicKey> {
    let secp = Secp256k1::new();

    // Create auth keypair
    let auth_keypair = Keypair::new(&secp, &mut rand::thread_rng());
    let auth_seckey = SecretKey::from_keypair(&auth_keypair);
    let auth_pubkey = PublicKey::from_keypair(&auth_keypair);

    let (account, _) = account::get_account(deriv).unwrap();
    let account_address = account.get_addr();

    // Make "Deposit" request
    let res: Result<(String, String)> = async {
        let req = DepositReq {
            token_id: "abc".to_string(),
            auth_key: auth_pubkey.to_string(),
            amount: amount as u32,
        };
        let body = serde_json::to_value(req)?;
        let res = conn.post("statechain/deposit", &body).await?;

        let json: DepositRes = serde_json::from_value(res)?;
        println!("Deposit response {:#?}", json);
        Ok((json.se_pubkey, json.statechain_id))
    }
    .await;
    let (se_pubkey, statechain_id) = res?;

    // Create  o1
    let owner_keypair = Keypair::new(&secp, &mut rand::thread_rng());
    let owner_seckey = SecretKey::from_keypair(&owner_keypair);
    let owner_pubkey = PublicKey::from_keypair(&owner_keypair);

    // combine 2 address
    let key = create_aggregated_address(owner_pubkey.to_string(), se_pubkey, Network::Testnet)?;
    if let Err(e) = pool
        .insert_statecoin(
            &statechain_id,
            &account_address,
            amount,
            &auth_seckey,
            &auth_pubkey,
            &key.aggregate_pubkey,
            &key.aggregate_address,
            &owner_seckey, // TODO: convert to derivPath
            &owner_pubkey,
        )
        .await
    {
        panic!("Failed to insert statecoin data {:?}", e);
    }

    let txid = util::create_deposit_transaction(
        pool,
        deriv,
        amount,
        &key.aggregate_address,
        &statechain_id,
    )
    .await?;

    let tx = create_bk_tx(
        pool,
        conn,
        &key.aggregate_pubkey,
        &key.aggregate_address,
        &account_address,
        &txid,
        0,
        amount,
        &statechain_id,
    )
    .await?;
    println!("bk tx: {}", consensus::encode::serialize_hex(&tx));

    Ok(key)
}

// TODO: Should use the function in statechain-core
pub fn create_aggregated_address(
    k1: String,
    k2: String,
    network: Network,
) -> Result<AggregatedPublicKey> {
    let secp = Secp256k1::new();
    let pub_k1 = PublicKey::from_str(&k1)?;
    let pub_k2 = PublicKey::from_str(&k2)?;

    let aggregated_pubkey = pub_k1.combine(&pub_k2)?;

    let aggregated_address = Address::p2tr(
        &secp,
        aggregated_pubkey.x_only_public_key().0,
        None,
        network,
    );

    Ok(AggregatedPublicKey {
        aggregate_pubkey: aggregated_pubkey.to_string(),
        aggregate_address: aggregated_address.to_string(),
    })
}

pub async fn create_bk_tx(
    pool: &PoolWrapper,
    conn: &NodeConnector,
    agg_pubkey: &str,
    agg_address: &str,
    receiver_address: &str,
    txid: &str,
    vout: u32,
    amount: u64,
    statechain_id: &str,
) -> Result<Transaction> {
    let agg_addr = Address::from_str(agg_address)?;
    let checked_agg_addr = agg_addr.require_network(Network::Testnet)?;
    let agg_scriptpubkey = checked_agg_addr.script_pubkey();

    let utxo = TxOut {
        value: Amount::from_sat(amount),
        script_pubkey: checked_agg_addr.script_pubkey(),
    };

    let prev_outpoint = OutPoint {
        txid: Txid::from_str(txid).unwrap(),
        vout,
    };

    let input = TxIn {
        previous_output: prev_outpoint,
        script_sig: ScriptBuf::default(),
        sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
        witness: Witness::default(),
    };

    let output_address = Address::from_str(receiver_address)?;
    let checked_output_address = output_address.require_network(Network::Testnet)?;
    let spend = TxOut {
        value: Amount::from_sat(amount),
        script_pubkey: checked_output_address.script_pubkey(),
    };

    let mut unsigned_tx = Transaction {
        version: Version::TWO,               // Post BIP-68.
        lock_time: absolute::LockTime::ZERO, // Ignore the locktime.
        input: vec![input],                  // Input goes into index 0.
        output: vec![spend],                 // Outputs, order does not matter.
    };

    // request signature from server
    let res = statechain::request_sign_bk_tx(
        conn,
        statechain_id,
        &consensus::encode::serialize_hex(&unsigned_tx),
        &agg_scriptpubkey.to_hex_string(),
    )
    .await?;
    print!("server sign bk: {}", res.sig);

    let server_sig = Signature::from_str(&res.sig)?;

    // bee4638722356ded164fa78c66933f903af20672933ac49ed10305559e39ab2eb5ef3b7bb79852fdc5402ce5feefff45a63ad017648d791ff01451780c06ddf7

    let input_index = 0;

    let sighash_type = TapSighashType::Default;
    let prevouts = vec![utxo];
    let prevouts = Prevouts::All(&prevouts);

    let mut sighasher = SighashCache::new(&mut unsigned_tx);
    let sighash = sighasher
        .taproot_key_spend_signature_hash(input_index, &prevouts, sighash_type)
        .expect("failed to construct sighash");

    let secp = Secp256k1::new();
    let seckey = pool.get_seckey_by_id(statechain_id).await?.unwrap();
    let seckey = SecretKey::from_str(&seckey)?;
    let keypair = Keypair::from_secret_key(&secp, &seckey);

    let tweaked: TweakedKeypair = keypair.tap_tweak(&secp, None);
    let msg = Message::from(sighash);

    let owner_sig = secp.sign_schnorr(&msg, &tweaked.to_inner());

    let sig1 = BigUint::from_slice(&owner_sig.serialize()); //Scalar::from_bytes_mod_order_wide(&owner_sig.serialize());
    let sig2 = Scalar::from_bytes_mod_order_wide(&server_sig.serialize());

    // calculate_musig_session(coin, hash, encoded_unsigned_tx)
    // TODO: Sign = owner_sign + server_sign
    let sig = sig1 + sig2;
    let sig = Signature::from_slice(&sig.as_bytes());

    let signature = bitcoin::taproot::Signature {
        sig,
        hash_ty: sighash_type,
    };
    *sighasher.witness_mut(input_index).unwrap() = Witness::p2tr_key_spend(&signature);

    // Get the signed transaction.
    let tx = sighasher.into_transaction();

    // BOOM! Transaction signed and ready to broadcast.
    println!("{:#?}", tx);

    Ok(unsigned_tx)
}
