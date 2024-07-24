use anyhow::{anyhow, Result};
use bitcoin::{
    secp256k1::{All, Message, Secp256k1, SecretKey},
    sighash::SighashCache,
    Amount, EcdsaSighashType, ScriptBuf, Transaction, TxIn, Witness,
};
use shared::{api, model::Txn};
use wallet::core::{Account, MasterAccount, Unlocker};

use crate::{cfg::PASSPHRASE, store::master_account::get_master};

pub fn get_account(deriv: &str) -> Result<(Account, Unlocker)> {
    let master_account = get_master().expect("Master account does not exist");
    let parsed_path = parse_derivation_path(deriv)?;
    let account = master_account
        .accounts()
        .get(&parsed_path)
        .ok_or(anyhow!("Account is not found"))?;
    let unlocker = Unlocker::new_for_master(&master_account, PASSPHRASE)?;
    Ok((account.clone(), unlocker))
}

pub fn get_internal_account(deriv: &str) -> Result<Account> {
    let master_account: MasterAccount = get_master().expect("Master account does not exist");
    let parsed_path = parse_derivation_path(deriv)?;
    let account = master_account.accounts().get(&parsed_path);
    match account {
        Some(account) => Ok(account.clone()),
        None => Err(anyhow!("Account not found")),
    }
}

pub fn parse_derivation_path(deriv: &str) -> Result<(u32, u32)> {
    let parts: Vec<&str> = deriv.split('/').collect();
    if parts.len() == 2 {
        let part0 = parts[0]
            .parse::<u32>()
            .map_err(|_| anyhow!("First part of the path is not a valid u32"))?;
        let part1 = parts[1]
            .parse::<u32>()
            .map_err(|_| anyhow!("Second part of the path is not a valid u32"))?;
        Ok((part0, part1))
    } else {
        Err(anyhow!(
            "Derivation path must be exactly two components separated by '/'"
        ))
    }
}

pub async fn find_and_join_txn(index: usize, input: TxIn) -> Result<(usize, TxIn, Txn)> {
    match api::get_onchain_tx(&input.previous_output.txid.to_string()).await {
        Ok(tx) => Ok((index, input, tx)),
        Err(e) => Err(anyhow!("Failed to get transaction for input {}", e)),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn sign(
    secp: &Secp256k1<All>,
    sighasher: &mut SighashCache<&mut Transaction>,
    sighash_type: EcdsaSighashType,
    account: &Account,
    unlocker: &mut Unlocker,
    index: &usize,
    input: &TxIn,
    tx: &Txn,
) -> Result<()> {
    let vout = tx
        .vout
        .get(input.previous_output.vout as usize)
        .ok_or(anyhow!("Vout is not found in the given transaction"))?;
    let amount = Amount::from_sat(vout.value);
    let script_pubkey = ScriptBuf::from_hex(&vout.scriptpubkey)?;

    let sighash = sighasher.p2wpkh_signature_hash(*index, &script_pubkey, amount, sighash_type)?;

    let msg = Message::from(sighash);
    let priv_key = account.get_privkey(script_pubkey, unlocker)?;
    let sk = SecretKey::from_slice(&priv_key.to_bytes()).unwrap();
    let pk = sk.public_key(secp);

    let sig = secp.sign_ecdsa(&msg, &sk);

    // Update the witness stack.
    let signature = bitcoin::ecdsa::Signature {
        sig,
        hash_ty: sighash_type,
    };
    *sighasher.witness_mut(*index).unwrap() = Witness::p2wpkh(&signature, &pk);
    Ok(())
}
