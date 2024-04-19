use anyhow::{anyhow, Result};
use bitcoin::{
    absolute,
    consensus::{self, Encodable},
    hashes::sha256,
    hex::DisplayHex,
    key::UntweakedPublicKey,
    psbt::{Input, PsbtSighashType},
    secp256k1::{rand, Keypair, PublicKey, Secp256k1, SecretKey},
    sighash::{self, Prevouts, SighashCache},
    transaction::{self, Version},
    Address, Amount, EcdsaSighashType, Network, OutPoint, Psbt, ScriptBuf, Sequence,
    TapSighashType, Transaction, TxIn, TxOut, Txid, Witness, XOnlyPublicKey,
};
use musig2::{
    AdaptorSignature, AggNonce, BinaryEncoding, KeyAggContext, PartialSignature, PubNonce, SecNonce,
};

use secp256k1::{schnorr::Signature, Message};

use std::{collections::BTreeMap, ops::ControlFlow, str::FromStr};

use crate::{api::statechain, cfg::BASE_TX_FEE, db::PoolWrapper, model::AccountActions};
use shared::intf::statechain::{DepositInfo, DepositReq, DepositRes};

use crate::connector::NodeConnector;

use super::account;

pub async fn deposit(
    pool: &PoolWrapper,
    conn: &NodeConnector,
    deriv: &str,
    amount: u64,
) -> Result<DepositInfo> {
    let secp = Secp256k1::new();

    let auth_keypair = Keypair::new(&secp, &mut rand::thread_rng());
    let auth_seckey = SecretKey::from_keypair(&auth_keypair);
    let xonly_auth_pubkey = XOnlyPublicKey::from_keypair(&auth_keypair).0;

    let (account, _) = account::get_account(deriv).unwrap();
    let account_address = account.get_addr();

    let req = DepositReq {
        token_id: "abc".to_string(),
        addr: xonly_auth_pubkey.to_string(),
        amount: amount as u32,
    };
    let body = serde_json::to_value(req)?;
    let res = conn.post("statechain/deposit", &body).await?;

    let json: DepositRes = serde_json::from_value(res)?;
    // response
    let se_pubkey = json.se_pubkey_1;
    let statechain_id = json.statechain_id;

    //gen o1
    let owner_keypair = Keypair::new(&secp, &mut rand::thread_rng());
    let owner_seckey = SecretKey::from_keypair(&owner_keypair);
    let owner_pubkey = PublicKey::from_keypair(&owner_keypair);

    //gen auth_key

    // combine 2 address
    let (aggregated_pubkey, aggregated_address, key_agg_ctx) =
        aggregate_pubkeys(owner_pubkey, PublicKey::from_str(&se_pubkey).unwrap());

    if let Err(e) = pool
        .insert_statecoin(
            &statechain_id,
            &account_address,
            amount,
            &auth_seckey,
            &xonly_auth_pubkey,
            &aggregated_pubkey.to_string(),
            &aggregated_address.to_string(),
            &owner_seckey,
            &owner_pubkey,
        )
        .await
    {
        panic!("Failed to insert statecoin data {:?}", e);
    }

    let deposit_tx =
        create_deposit_transaction(&pool, &deriv, amount, &aggregated_pubkey, &statechain_id)
            .await?;

    let txid = deposit_tx.txid().to_string();

    create_bk_tx(
        &pool,
        &conn,
        &key_agg_ctx,
        &aggregated_pubkey,
        &aggregated_address,
        &account_address,
        &txid,
        0,
        amount,
        &statechain_id,
    )
    .await?;

    Ok(DepositInfo {
        aggregated_address: aggregated_address.to_string(),
        deposit_tx_hex: consensus::encode::serialize_hex(&deposit_tx),
    })
}

pub async fn create_deposit_transaction(
    pool: &PoolWrapper,
    deriv: &str,
    amount: u64,
    aggregated_pubkey: &PublicKey,
    statechain_id: &str,
) -> Result<Transaction> {
    let (account, mut unlocker) = account::get_account(deriv).unwrap();
    let selected_utxos = account::get_utxos_set(&account.get_addr(), amount + BASE_TX_FEE).await?;
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

    // let agg_addr = Address::from_str(aggregated_address).unwrap();
    // let checked_agg_addr = agg_addr.require_network(Network::Testnet).unwrap();

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
        input: input,
        output: output,
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
    println!("deposit tx hex: {:?}", tx_hex);
    println!("deposit tx raw {:#?}", unsigned_deposit_tx);
    let funding_txid = unsigned_deposit_tx.txid().to_string();
    let funding_vout = 0 as u64;
    let _ = pool
        .update_deposit_tx(
            &statechain_id,
            &funding_txid,
            funding_vout,
            "CONFIRM",
            &tx_hex,
        )
        .await?;

    Ok(unsigned_deposit_tx)
}

pub async fn create_bk_tx(
    pool: &PoolWrapper,
    conn: &NodeConnector,
    key_agg_ctx: &KeyAggContext,
    agg_pubkey: &PublicKey,
    agg_address: &Address,
    receiver_address: &str,
    txid: &str,
    vout: u32,
    amount: u64,
    statechain_id: &str,
) -> Result<()> {
    let secp = Secp256k1::new();
    let seckey = pool
        .get_seckey_by_id(&statechain_id)
        .await
        .unwrap()
        .unwrap();
    let seckey = SecretKey::from_str(&seckey).unwrap();

    let agg_scriptpubkey = ScriptBuf::new_p2tr(&secp, agg_pubkey.x_only_public_key().0, None);
    let agg_scriptpubkey_clone = agg_scriptpubkey.clone();
    let utxo = TxOut {
        value: Amount::from_sat(amount),
        script_pubkey: agg_scriptpubkey,
    };

    println!("utxo that bk sign:{:#?}", utxo);

    let prev_outpoint = OutPoint {
        txid: Txid::from_str(txid).unwrap(),
        vout: vout,
    };

    let input = TxIn {
        previous_output: prev_outpoint,
        script_sig: ScriptBuf::default(),
        sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
        witness: Witness::default(),
    };



    let output_address = Address::from_str(receiver_address).unwrap();
    let checked_output_address = output_address.require_network(Network::Testnet).unwrap();
    let spend = TxOut {
        value: Amount::from_sat(amount - BASE_TX_FEE),
        script_pubkey: checked_output_address.script_pubkey(),
    };


    let unsigned_tx = Transaction {
        version: transaction::Version::TWO,  // Post BIP-68.
        lock_time: absolute::LockTime::ZERO, // Ignore the locktime.
        input: vec![input],                  // Input goes into index 0.
        output: vec![spend],         // Outputs, order does not matter.
    };

    let mut psbt = Psbt::from_unsigned_tx(unsigned_tx)?;

    let mut input_psbt = Input {
        witness_utxo: Some(TxOut {
            value: Amount::from_sat(amount),
            script_pubkey: agg_scriptpubkey_clone,
        }),
        ..Default::default()
    };

    let ty = PsbtSighashType::from_str("SIGHASH_ALL")?;
    input_psbt.sighash_type = Some(ty);
    input_psbt.tap_internal_key = Some(agg_pubkey.x_only_public_key().0);
    psbt.inputs = vec![input_psbt];

    let unsigned_psbt = psbt.unsigned_tx.clone();

    let input_psbt = psbt.inputs.iter_mut().nth(0).unwrap();

    let hash_ty = input_psbt
        .sighash_type
        .and_then(|psbt_sighash_type| psbt_sighash_type.taproot_hash_ty().ok())
        .unwrap_or(TapSighashType::All);

    let hash = SighashCache::new(&unsigned_psbt).taproot_key_spend_signature_hash(
        0,
        &sighash::Prevouts::All(&[TxOut {
            value: input_psbt.witness_utxo.as_ref().unwrap().value,
            script_pubkey: input_psbt
                .witness_utxo
                .as_ref()
                .unwrap()
                .script_pubkey
                .clone(),
        }]),
        hash_ty,
    )?;

    let msg: Message = hash.into();
    // //////////////////////////////////////////////////////////////////////
    // let sighash_type = TapSighashType::Default;
    // let prevouts = vec![utxo];
    // let prevouts = Prevouts::All(&prevouts);
    // let mut sighasher = SighashCache::new(&mut unsigned_tx);
    // let sighash = sighasher
    //     .taproot_key_spend_signature_hash(0, &prevouts, sighash_type)
    //     .expect("failed to construct sighash");
    // // println!("sighash : {}", sighash.to_string());
    // // println!("msg sighash : {}", msg.to_string());
    // /////////////////////////////////////////////////////////////////////

    let parsed_msg = msg.to_string();

    let msg_clone = parsed_msg.clone();
    let msg = parsed_msg.clone();

    let signed_statechain_id = sign_message(&statechain_id, &seckey).to_string();

    let get_nonce_res = statechain::get_nonce(&conn, statechain_id, &signed_statechain_id).await?;
    let server_pubnonce = get_nonce_res.server_nonce;

    let nonce_seed = [0xACu8; 32];

    let secnonce = SecNonce::build(nonce_seed).with_seckey(seckey).build();

    let our_public_nonce = secnonce.public_nonce();

    let public_nonces = [
        our_public_nonce,
        server_pubnonce.parse::<PubNonce>().unwrap(),
    ];

    let agg_pubnonce: AggNonce = public_nonces.iter().sum();

    let agg_pubnonce_str = agg_pubnonce.to_string();

    let our_partial_signature: PartialSignature =
        musig2::sign_partial(&key_agg_ctx, seckey, secnonce, &agg_pubnonce, parsed_msg)
            .expect("error creating partial signature");

    let serialized_key_agg_ctx = key_agg_ctx
        .to_bytes()
        .to_hex_string(bitcoin::hex::Case::Lower);

    let get_sign_res = statechain::get_partial_signature(
        &conn,
        &serialized_key_agg_ctx,
        &statechain_id,
        &signed_statechain_id,
        &msg_clone,
        &agg_pubnonce_str,
    )
    .await?;

    let server_signature = get_sign_res.partial_signature;

    //let server_signature = "5a476e0126583e9e0ceebb01a34bdd342c72eab92efbe8a1c7f07e793fd88f96";

    let partial_signatures = [
        our_partial_signature,
        PartialSignature::from_hex(&server_signature).unwrap(),
    ];

    let final_signature: secp256k1::schnorr::Signature = musig2::aggregate_partial_signatures(
        &key_agg_ctx,
        &agg_pubnonce,
        partial_signatures,
        msg_clone,
    )
    .expect("error aggregating signatures");

    //let fake_pubkey = PublicKey::from_str( "026e14224899cf9c780fef5dd200f92a28cc67f71c0af6fe30b5657ffc943f08f4").unwrap();

    // let fake_sig :[u8;64] =  [
    //     0x38, 0xFB, 0xD8, 0x2D, 0x1D, 0x27, 0xBB, 0x34, 0x01, 0x04, 0x20, 0x62, 0xAC, 0xFD,
    //     0x4E, 0x7F, 0x54, 0xCE, 0x93, 0xDD, 0xF2, 0x6A, 0x4A, 0xE8, 0x7C, 0xF7, 0x15, 0x68,
    //     0xC1, 0xD4, 0xE8, 0xBB, 0x8F, 0xCA, 0x20, 0xBB, 0x6F, 0x7B, 0xCE, 0x2C, 0x5B, 0x54,
    //     0x57, 0x6D, 0x31, 0x5B, 0x21, 0xEA, 0xE3, 0x1A, 0x61, 0x46, 0x41, 0xAF, 0xD2, 0x27,
    //     0xCD, 0xA2, 0x21, 0xFD, 0x6B, 0x1C, 0x54, 0xEA
    // ];
    musig2::verify_single(*agg_pubkey, final_signature, msg)
        .expect("aggregated signature must be valid");

    let signature = bitcoin::taproot::Signature {
        sig: final_signature,
        hash_ty: hash_ty,
    };

    input_psbt.tap_key_sig = Some(signature);

    // let signature_bytes = signature.to_vec();

    psbt.inputs.iter_mut().for_each(|input| {
        let mut script_witness: Witness = Witness::new();
        script_witness.push(input.tap_key_sig.unwrap().to_vec());
        input.final_script_witness = Some(script_witness);

        // Clear all the data fields as per the spec.
        input.partial_sigs = BTreeMap::new();
        input.sighash_type = None;
        input.redeem_script = None;
        input.witness_script = None;
        input.bip32_derivation = BTreeMap::new();
    });

    println!(
        "signature byte: {:#?}",
        signature.to_vec().to_lower_hex_string()
    );
    /////////////////////////////////////////////////////////////
    // let mut wit = Witness::new();
    // wit.push(signature.to_vec());
    // //  unsigned_tx_2.input[0].witness = wit.clone();
    // //  unsigned_tx_2.input[0]

    // *sighasher.witness_mut(0).unwrap() = wit;

    // let tx = sighasher.into_transaction();

    // let tx_hex = consensus::encode::serialize_hex(&tx);
    // // let tx_2_hex = consensus::encode::serialize_hex(&unsigned_tx_2);
    ///////////////////////////////////////////////////////////////
    ///
    let signed_tx = psbt.extract_tx()?;

    let tx_hex = consensus::encode::serialize_hex(&signed_tx);

    pool.update_bk_tx(&statechain_id, &tx_hex, &agg_pubnonce.to_string())
        .await?;

    //println!("Bk tx raw {:#?}", tx);

    println!("Bk tx hex: {}", tx_hex);

    Ok(())
}

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
) -> (PublicKey, Address, KeyAggContext) {
    let secp = Secp256k1::new();
    let mut pubkeys: Vec<PublicKey> = vec![];
    pubkeys.push(owner_pubkey);
    pubkeys.push(se_pubkey);
    let key_agg_ctx = KeyAggContext::new(pubkeys)
        .unwrap()
        .with_unspendable_taproot_tweak()
        .unwrap();

    let aggregated_pubkey: PublicKey = key_agg_ctx.aggregated_pubkey_untweaked();

    let aggregated_address = Address::p2tr(
        &secp,
        aggregated_pubkey.x_only_public_key().0,
        None,
        Network::Testnet,
    );

    (aggregated_pubkey, aggregated_address, key_agg_ctx)
}
