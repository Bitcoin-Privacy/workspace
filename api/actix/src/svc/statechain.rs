use std::str::FromStr;

use actix_web::web::Data;
use anyhow::Result;
use bitcoin::{
    bip32::Xpub,
    consensus::{self, serde::hex::Lower},
    hex::{Case, DisplayHex},
    key::{Keypair, TapTweak, TweakedKeypair},
    secp256k1::{rand, Message, PublicKey, Secp256k1, SecretKey},
    sighash::{Prevouts, SighashCache},
    Amount, ScriptBuf, TapSighashType, Transaction, TxOut,
};
use musig2::{
    secp::MaybeScalar, AggNonce, BinaryEncoding, FirstRound, KeyAggContext, PartialSignature,
    SecNonce, SecNonceSpices,
};

use crate::repo::statechain::{StatechainRepo, TraitStatechainRepo};
use shared::intf::statechain::{CreateBkTxnRes, DepositRes, GetNonceRes, GetPartialSignatureRes};

pub async fn create_deposit(
    repo: &Data<StatechainRepo>,
    token_id: &str,
    auth_pubkey: &str,
    amount: u32,
) -> Result<DepositRes, String> {
    println!("Auth pubkey {}", auth_pubkey);
    let auth_key = match PublicKey::from_str(auth_pubkey) {
        Ok(key) => key,
        Err(err) => return Err(format!("Invalid auth public key: {}", err)),
    };

    let secp = Secp256k1::new();
    let secret_key = SecretKey::new(&mut rand::thread_rng());
    let pub_key = PublicKey::from_secret_key(&secp, &secret_key);

    let statecoin = repo
        .create_deposit_tx(token_id, &auth_key, &pub_key, &secret_key, amount)
        .await
        .map_err(|e| format!("Failed to add deposit: {}", e))?;

    let res = DepositRes {
        se_pubkey_1: pub_key.to_string(),
        statechain_id: statecoin.id.to_string(),
    };

    Ok(res)
}

pub async fn create_bk_txn(
    repo: &Data<StatechainRepo>,
    statechain_id: &str,
    scriptpubkey: &str,
    txn: &str,
) -> Result<CreateBkTxnRes> {
    let statecoin = repo.get_by_id(statechain_id).await?;

    let sk = SecretKey::from_str(&statecoin.server_private_key)?;
    let secp = Secp256k1::new();
    let keypair = Keypair::from_secret_key(&secp, &sk);

    let parsed_tx = consensus::deserialize::<Transaction>(&hex::decode(txn)?)?;

    let sighash_type = TapSighashType::Default;

    let mut unsigned_txn = parsed_tx.clone();
    let mut sighasher = SighashCache::new(&mut unsigned_txn);

    let input_index = 0;

    let secp = Secp256k1::new();

    let prevouts = vec![TxOut {
        value: Amount::from_sat(statecoin.amount as u64),
        script_pubkey: ScriptBuf::from_hex(scriptpubkey)?,
    }];
    let prevouts = Prevouts::All(&prevouts);

    let sighash = sighasher
        .taproot_key_spend_signature_hash(input_index, &prevouts, sighash_type)
        .expect("failed to construct sighash");

    let tweaked: TweakedKeypair = keypair.tap_tweak(&secp, None);
    let msg = Message::from(sighash);

    let signature = secp.sign_schnorr(&msg, &tweaked.to_inner());

    let signature = bitcoin::taproot::Signature {
        sig: signature,
        hash_ty: sighash_type,
    };

    let res = CreateBkTxnRes {
        sig: hex::encode(signature.to_vec()),
        rand_key: "".to_string(),
    };

    Ok(res)
}

pub async fn get_nonce(
    repo: &Data<StatechainRepo>,
    statechain_id: &str,
    signed_statechain_id: &str,
) -> Result<GetNonceRes> {
    let nonce_seed = [0xACu8; 32];
    let secnonce = musig2::SecNonceBuilder::new(nonce_seed).build();

    let pubnonce = secnonce.public_nonce();

    repo.update_nonce(
        &statechain_id,
        &secnonce.to_bytes().to_hex_string(Case::Lower),
        &pubnonce.to_bytes().to_hex_string(Case::Lower),
    )
    .await?;

    Ok(GetNonceRes {
        server_nonce: pubnonce.to_string(),
    })
}

pub async fn get_sig(
    repo: &Data<StatechainRepo>,
    serialized_key_agg_ctx: &str,
    statechain_id: &str,
    signed_statechain_id: &str,
    parsed_tx: &str,
    agg_pubnonce: &str,
) -> Result<GetPartialSignatureRes> {
    let statecoin = repo.get_by_id(statechain_id).await?;
    let secnonce = statecoin.sec_nonce.unwrap();
    println!("nonce 2 : {}", secnonce);
    let seckey = SecretKey::from_str(&statecoin.server_private_key)?;
    let secnonce = SecNonce::from_hex(&secnonce).unwrap();
   

    let key_agg_ctx = KeyAggContext::from_hex(serialized_key_agg_ctx).unwrap();

    println!(
        "agg-ctx and pubnonce {},{}",
        serialized_key_agg_ctx, agg_pubnonce
    );

    // let first_round = FirstRound::new(
    //     key_agg_ctx.clone(),
    //     nonce_seed,
    //     1,
    //     SecNonceSpices::new()
    //         .with_seckey(seckey)
    //         .with_message(&parsed_tx),
    // )
    // .unwrap();

    let agg_nonce = AggNonce::from_str(agg_pubnonce).unwrap();
    // let our_partial_signature: MaybeScalar = first_round
    //     .sign_for_aggregator(seckey, parsed_tx, &agg_nonce)
    //     .unwrap();

    let our_partial_signature: PartialSignature =
        musig2::sign_partial(&key_agg_ctx, seckey, secnonce, &agg_nonce, parsed_tx)?;

    let final_sig = our_partial_signature.serialize().to_hex_string(Case::Lower);

    Ok(GetPartialSignatureRes {
        partial_signature: final_sig,
    })
}

// pub async fn verify_signature(
//     repo: &Data<StatechainRepo>,
//     sign_message_hex: &str,
//     statechain_id: &str,
// ) -> bool {
//     let auth_key = repo
//         .get_auth_key_by_statechain_id(&statechain_id)
//         .await
//         .unwrap();

//     let pub_key = XOnlyPublicKey::from_str(&auth_key).unwrap();
//     let signed_message = Signature::from_str(sign_message_hex).unwrap();
//     let msg = Message::from_hashed_data::<sha256::Hash>(statechain_id.to_string().as_bytes());

//     let secp = Secp256k1::new();
//     secp.verify_schnorr(&signed_message, &msg, &pub_key).is_ok()
// }
