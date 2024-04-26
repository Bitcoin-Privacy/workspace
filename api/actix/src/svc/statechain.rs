use std::str::FromStr;

use actix_web::web::Data;
use anyhow::{anyhow, Result};
use bitcoin::{
    consensus,
    hashes::sha256,
    hex::{Case, DisplayHex},
    key::{Keypair, TapTweak, TweakedKeypair},
    secp256k1::{rand, schnorr::Signature, PublicKey, Secp256k1, SecretKey},
    sighash::{Prevouts, SighashCache},
    Amount, ScriptBuf, TapSighashType, Transaction, TxOut, XOnlyPublicKey,
};
use musig2::{secp256k1::Message, AggNonce, KeyAggContext, PartialSignature, SecNonce};
use statechain_core::deposit::{DepositMsg1, DepositMsg1Response};

use crate::{
    model::entity::statechain::StateCoin,
    repo::statechain::{StatechainRepo, TraitStatechainRepo},
};
use shared::intf::statechain::{CreateBkTxnRes, GetNonceRes, GetPartialSignatureRes};

pub async fn create_deposit(
    repo: &Data<StatechainRepo>,
    payload: DepositMsg1,
) -> Result<DepositMsg1Response> {
    let auth_key = XOnlyPublicKey::from_str(&payload.auth_key)?;
    let token_id = payload.token_id.clone();
    let signed_token_id = Signature::from_str(&payload.signed_token_id.to_string())?;

    let msg = Message::from_hashed_data::<sha256::Hash>(token_id.to_string().as_bytes());

    let secp = Secp256k1::new();
    if !secp
        .verify_schnorr(&signed_token_id, &msg, &auth_key)
        .is_ok()
    {
        return Err(anyhow!("Signature does not match authentication key."));
    }

    let secp = Secp256k1::new();
    let secret_key = SecretKey::new(&mut rand::thread_rng());
    let pub_key = PublicKey::from_secret_key(&secp, &secret_key);
    println!("KEYPAIR - PRIV: {:#?}", secret_key.display_secret());
    println!("KEYPAIR - PUBL: {:#?}", pub_key.to_string());

    let nonce_seed = [0xACu8; 32];
    let secnonce = musig2::SecNonceBuilder::new(nonce_seed).build();

    let pubnonce = secnonce.public_nonce();

    let statecoin = repo
        .create_deposit_tx(
            &token_id,
            &auth_key,
            &pub_key,
            &secret_key,
            0,
            &secnonce,
            &pubnonce,
        )
        .await?;

    let res = DepositMsg1Response {
        server_pubkey: pub_key.to_string(),
        statechain_id: statecoin.id.to_string(),
    };

    Ok(res)
}

pub async fn sign_txn_bk(
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
    // if !verify_signature(&repo, &signed_statechain_id, &statechain_id).await? {
    //     bail!("Invalid signature")
    // }
    let res = repo.get_nonce(&statechain_id).await?;

    Ok(GetNonceRes {
        server_nonce: res.pub_nonce.to_string(),
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
    // if !verify_signature(&repo, &signed_statechain_id, &statechain_id).await? {
    //     bail!("Invalid signature")
    // }

    let statecoin = repo.get_by_id(statechain_id).await?;

    println!("messsagee : {}", parsed_tx);

    let secnonce = statecoin.sec_nonce.unwrap();
    println!("nonce 2 : {}", secnonce);
    let seckey = SecretKey::from_str(&statecoin.server_private_key)?;
    let secnonce = SecNonce::from_hex(&secnonce).unwrap();

    let key_agg_ctx = KeyAggContext::from_hex(serialized_key_agg_ctx).unwrap();

    println!(
        "agg-ctx and pubnonce {},{}",
        serialized_key_agg_ctx, agg_pubnonce
    );

    let agg_nonce = AggNonce::from_str(agg_pubnonce).unwrap();

    let our_partial_signature: PartialSignature =
        musig2::sign_partial(&key_agg_ctx, seckey, secnonce, &agg_nonce, parsed_tx)?;

    let final_sig = our_partial_signature.serialize().to_hex_string(Case::Lower);

    Ok(GetPartialSignatureRes {
        partial_signature: final_sig,
    })
}

pub async fn verify_signature(
    repo: &Data<StatechainRepo>,
    signature: &str,
    statechain_id: &str,
) -> Result<bool> {
    let auth_key = repo.get_auth_key_by_statechain_id(&statechain_id).await?;

    let pub_key = XOnlyPublicKey::from_str(&auth_key.auth_xonly_public_key)?;
    let signed_message = Signature::from_str(signature).unwrap();
    let msg = Message::from_hashed_data::<sha256::Hash>(statechain_id.to_string().as_bytes());

    let secp = Secp256k1::new();
    Ok(secp.verify_schnorr(&signed_message, &msg, &pub_key).is_ok())
}

pub async fn list_statecoins(
    repo: &Data<StatechainRepo>,
    token_id: &str,
) -> Result<Vec<StateCoin>> {
    repo.get_by_token_id(token_id).await
}
