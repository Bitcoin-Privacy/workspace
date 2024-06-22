use anyhow::anyhow;
use anyhow::Result;
use bitcoin::{
    absolute, consensus,
    hex::DisplayHex,
    secp256k1::{PublicKey, Secp256k1},
    sighash::SighashCache,
    transaction::Version,
    Address, Amount, EcdsaSighashType, Network, OutPoint, ScriptBuf, Sequence, Transaction, TxIn,
    TxOut, Witness,
};

use std::{ops::ControlFlow, str::FromStr};

use crate::connector::NodeConnector;
use crate::model::Statecoin;
use crate::svc::statechain::aggregate_pubkeys;
use crate::svc::statechain::generate_auth_owner_keypairs;
use crate::svc::statechain::sign_message;
use crate::svc::statechain_sender::create_bk_tx_for_receiver;
use crate::{cfg::BASE_TX_FEE, db::PoolWrapper, model::AccountActions};
use shared::api::broadcast_tx;
use shared::intf::statechain::{DepositInfo, DepositReq, DepositRes};

use super::account;

pub async fn execute(
    pool: &PoolWrapper,
    conn: &NodeConnector,
    deriv: &str,
    amount: u64,
) -> Result<DepositInfo> {
    let statechain_keypairs = generate_auth_owner_keypairs()?;

    let (account, _) = account::get_account(deriv).unwrap();
    let account_address = account.get_addr();
    let authkey = statechain_keypairs.auth_pubkey.x_only_public_key().0;

    let req = DepositReq {
        token_id: "abc".to_string(),
        addr: authkey.to_string(),
        amount: amount as u32,
    };
    let body = serde_json::to_value(req)?;
    let res = conn.post("statechain/deposit", &body).await?;

    let json: DepositRes = serde_json::from_value(res)?;
    // response
    let se_pubkey = json.se_pubkey_1;
    let statechain_id = json.statechain_id;

    let signed_statechain_id =
        sign_message(&statechain_id, &statechain_keypairs.auth_seckey).to_string();

    //gen auth_key

    // combine 2 address
    let (aggregated_pubkey, aggregated_pubkey_tw, aggregated_address, key_agg_ctx) =
        aggregate_pubkeys(
            statechain_keypairs.owner_pubkey,
            PublicKey::from_str(&se_pubkey).unwrap(),
        );

    println!("agg pub key {}", aggregated_pubkey_tw.x_only_public_key().0);

    let (funding_txid, vout, deposit_tx) =
        create_deposit_transaction(deriv, amount, &aggregated_pubkey).await?;
    let output_address = Address::from_str(&account_address).unwrap();
    let checked_output_address = output_address.require_network(Network::Testnet).unwrap();
    // seckey: &SecretKey,
    // statechain_id: &str,
    // signed_statechain_id: &str,
    // txid: &str,
    // amount: u64,
    // vout: i64,
    // agg_pubkey: &PublicKey,
    // key_agg_ctx: &KeyAggContext,
    // receiver_address: &Address,

    // pub struct Statecoin {
    //     pub tx_n: i64,
    //     pub owner_seckey: String,
    //     pub signed_statechain_id: String,
    //     pub aggregated_pubkey: String,
    //     pub aggregated_address: String,
    //     pub funding_txid: String,
    //     pub funding_vout: i64,
    //     pub key_agg_ctx: String,
    //     pub amount: i64,
    //     pub account: String,
    //     pub spend_key: String,
    // }

    // let statecoin = Statecoin {
    //     tx_n : 1,
    //     owner_seckey : statechain_keypairs.owner_seckey.secret_bytes().to_lower_hex_string(),
    //     signed_statechain_id: signed_statechain_id,
    //     aggregated_pubkey : aggregated_pubkey.to_string(),

    // }

    let sign = signed_statechain_id.clone();
    let txid = funding_txid.clone();
    let statecoin = Statecoin {
        tx_n: 0,
        signed_statechain_id: sign,
        aggregated_pubkey: aggregated_pubkey.to_string(),
        funding_txid: txid,
        funding_vout: 0,
        key_agg_ctx: key_agg_ctx.to_string(),
        amount: amount as i64,
        spend_key: statechain_keypairs
            .owner_seckey
            .secret_bytes()
            .to_lower_hex_string(),
    };
    // let bk_tx = create_bk_tx(
    //     &conn,
    //     &statechain_keypairs.owner_seckey,
    //     &statechain_id,
    //     &signed_statechain_id,
    //     &funding_txid,
    //     amount,
    //     0,
    //     &aggregated_pubkey,
    //     &key_agg_ctx,
    //     &checked_output_address,
    // )
    // .await?;

    let bk_tx =
        create_bk_tx_for_receiver(conn, &statechain_id, &statecoin, &checked_output_address)
            .await?;
    let deposit_tx_clone = deposit_tx.clone();
    let broadcast_res = broadcast_tx(deposit_tx_clone).await?;
    println!("broad cast response: {:?}", broadcast_res);

    if let Err(e) = pool
        .create_statecoin(
            &statechain_id,
            &signed_statechain_id,
            &account_address,
            amount,
            &statechain_keypairs.auth_seckey,
            &authkey,
            &aggregated_pubkey.to_string(),
            &aggregated_address.to_string(),
            &statechain_keypairs.owner_seckey,
            &statechain_keypairs.owner_pubkey,
            &key_agg_ctx,
            &funding_txid,
            vout,
            &deposit_tx,
            1,
            0,
            &bk_tx,
        )
        .await
    {
        panic!("Failed to insert statecoin data {:?}", e);
    }

    Ok(DepositInfo {
        aggregated_address: aggregated_address.to_string(),
    })
}

pub async fn create_deposit_transaction(
    deriv: &str,
    amount: u64,
    aggregated_pubkey: &PublicKey,
) -> Result<(String, u64, String)> {
    let (account, mut unlocker) = account::get_account(deriv).unwrap();
    let selected_utxos = account.get_utxo(amount + BASE_TX_FEE).await?;

    let secp = Secp256k1::new();
    let mut fee: u64 = 0;
    let input: Vec<TxIn> = selected_utxos
        .iter()
        .map(|utxo| {
            fee += utxo.value;
            println!("utxos set :{}", utxo.value);
            TxIn {
                previous_output: OutPoint::new(utxo.txid.parse().unwrap(), utxo.vout.into()),
                script_sig: ScriptBuf::from_bytes(vec![]),
                sequence: Sequence::MAX,
                witness: Witness::new(),
            }
        })
        .collect();

    let mut output: Vec<TxOut> = Vec::new();

    output.push(TxOut {
        value: Amount::from_sat(amount),
        script_pubkey: ScriptBuf::new_p2tr(&secp, aggregated_pubkey.x_only_public_key().0, None),
    });

    let (change, overflow) = fee.overflowing_sub(amount + BASE_TX_FEE);
    if overflow {
        return Err(anyhow!("Total amount cannot cover amount and fee"));
    }
    if change > 0 {
        output.push(TxOut {
            value: Amount::from_sat(change),
            script_pubkey: account.get_checked_addr().script_pubkey(),
        });
    }

    let deposit_tx = Transaction {
        version: Version::TWO,
        lock_time: absolute::LockTime::ZERO,
        input,
        output,
    };

    let mut unsigned_deposit_tx = deposit_tx.clone();

    let sighash_type = EcdsaSighashType::All;
    let mut sighasher = SighashCache::new(&mut unsigned_deposit_tx);

    let future_tasks: Vec<_> = deposit_tx
        .input
        .iter()
        .enumerate()
        .map(|(index, input)| {
            tokio::spawn(tokio::spawn(account::find_and_join_txn(
                index,
                input.clone(),
            )))
        })
        .collect();

    let mut results = Vec::new();
    for job in future_tasks {
        results.push(job.await.unwrap().unwrap().unwrap());
    }

    let res = results.iter().try_for_each(|(index, input, tx)| {
        match account::sign(
            &secp,
            &mut sighasher,
            sighash_type,
            &account,
            &mut unlocker,
            index,
            input,
            tx,
        ) {
            Ok(_) => ControlFlow::Continue(()),
            Err(e) => ControlFlow::Break(e),
        }
    });
    if let ControlFlow::Break(e) = res {
        return Err(e);
    }

    let tx_hex = consensus::encode::serialize_hex(&unsigned_deposit_tx);
    let funding_txid = unsigned_deposit_tx.txid().to_string();
    let funding_vout = 0_u64;
    Ok((funding_txid, funding_vout, tx_hex.to_string()))
}

// pub async fn create_bk_tx(
//     conn: &NodeConnector,
//     seckey: &SecretKey,
//     statechain_id: &str,
//     signed_statechain_id: &str,
//     txid: &str,
//     amount: u64,
//     vout: i64,
//     agg_pubkey: &PublicKey,
//     key_agg_ctx: &KeyAggContext,
//     receiver_address: &Address,
// ) -> Result<String> {
//     let secp = Secp256k1::new();

//     let agg_scriptpubkey = ScriptBuf::new_p2tr(&secp, agg_pubkey.x_only_public_key().0, None);
//     let scriptpubkey = agg_scriptpubkey.to_hex_string();
//     let prev_outpoint = OutPoint {
//         txid: Txid::from_str(&txid)?,
//         vout: vout as u32,
//     };

//     let input = TxIn {
//         previous_output: prev_outpoint,
//         script_sig: ScriptBuf::default(),
//         sequence: Sequence(0xFFFFFFFE),
//         witness: Witness::default(),
//     };

//     let spend = TxOut {
//         value: Amount::from_sat(amount - BASE_TX_FEE),
//         script_pubkey: receiver_address.script_pubkey(),
//     };

//     let mut unsigned_tx = Transaction {
//         version: transaction::Version::TWO, // Post BIP-68.
//         lock_time: LockTime::ZERO,          // Ignore the locktime.
//         input: vec![input],                 // Input goes into index 0.
//         output: vec![spend],                // Outputs, order does not matter.
//     };

//     let sighash_type = TapSighashType::Default;

//     let get_nonce_res = statechain::get_nonce(&conn, statechain_id, &signed_statechain_id).await?;
//     let server_pubnonce = get_nonce_res.server_nonce;

//     let mut nonce_seed = [0u8; 32];
//     rand::rngs::OsRng.fill_bytes(&mut nonce_seed);

//     let secnonce = SecNonce::build(nonce_seed).with_seckey(*seckey).build();

//     let our_public_nonce = secnonce.public_nonce();

//     let public_nonces = [
//         our_public_nonce,
//         server_pubnonce.parse::<PubNonce>().unwrap(),
//     ];

//     let agg_pubnonce: AggNonce = public_nonces.iter().sum();

//     let agg_pubnonce_str = agg_pubnonce.to_string();

//     let serialized_key_agg_ctx = key_agg_ctx
//         .to_bytes()
//         .to_hex_string(bitcoin::hex::Case::Lower);

//     let unsigned_tx_hex = consensus::encode::serialize_hex(&unsigned_tx);

//     let get_sign_res = statechain::get_partial_signature(
//         &conn,
//         &serialized_key_agg_ctx,
//         &statechain_id,
//         &signed_statechain_id,
//         &unsigned_tx_hex,
//         &agg_pubnonce_str,
//         &scriptpubkey,
//     )
//     .await?;

//     let sighash = &get_sign_res.sighash;
//     let sighash = TapSighash::from_str(sighash)?;
//     let msg = Message::from(sighash);
//     let msg = msg.as_ref();
//     let our_partial_signature: PartialSignature =
//         musig2::sign_partial(&key_agg_ctx, *seckey, secnonce, &agg_pubnonce, msg)
//             .expect("error creating partial signature");

//     let server_signature = get_sign_res.partial_sig;

//     let partial_signatures = [
//         our_partial_signature,
//         PartialSignature::from_hex(&server_signature).unwrap(),
//     ];

//     let agg_pubkey_tw: PublicKey = key_agg_ctx.aggregated_pubkey();
//     println!("tx tweaked public key : {}", agg_pubkey_tw.to_string());

//     for (i, partial_signature) in partial_signatures.into_iter().enumerate() {
//         if i == 0 {
//             // Don't bother verifying our own signature
//             continue;
//         }

//         let their_pubkey: PublicKey = key_agg_ctx.get_pubkey(i).unwrap();
//         let their_pubnonce = &public_nonces[i];

//         musig2::verify_partial(
//             &key_agg_ctx,
//             partial_signature,
//             &agg_pubnonce,
//             their_pubkey,
//             their_pubnonce,
//             msg,
//         )
//         .expect("received invalid signature from a peer");
//     }

//     let final_signature: secp256k1::schnorr::Signature =
//         musig2::aggregate_partial_signatures(&key_agg_ctx, &agg_pubnonce, partial_signatures, msg)
//             .expect("error aggregating signatures");

//     musig2::verify_single(agg_pubkey_tw, final_signature, msg)
//         .expect("aggregated signature must be valid");

//     let signature = bitcoin::taproot::Signature {
//         sig: final_signature,
//         hash_ty: sighash_type,
//     };

//     let mut wit = Witness::new();
//     wit.push(signature.to_vec());

//     unsigned_tx.input[0].witness = wit;
//     unsigned_tx.lock_time = LockTime::from_time(get_sign_res.n_lock_time as u32)?;
//     println!("new lock time after transfer: {}", get_sign_res.n_lock_time);

//     let tx_hex = consensus::encode::serialize_hex(&unsigned_tx);
//     println!("Bk tx hex: {}", tx_hex);

//     Ok(tx_hex)
// }
