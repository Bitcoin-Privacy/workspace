use std::str::FromStr;

use anyhow::Result;
use bitcoin::{
    absolute,
    psbt::{Input, PsbtSighashType},
    secp256k1::PublicKey,
    sighash::{self, SighashCache},
    transaction::Version,
    Address, Amount, Network, OutPoint, Psbt, ScriptBuf, TapSighashType, Transaction, TxIn, TxOut,
    Txid, Witness,
};
use statechain_core::{
    transaction::{calculate_musig_session, PartialSignatureMsg1},
    wallet::Coin,
};

pub fn get_musig_session(
    coin: &Coin,
    block_height: u32,
    output: &TxOut,
    network: Network,
) -> Result<PartialSignatureMsg1> {
    let input_pubkey = PublicKey::from_str(&coin.aggregated_pubkey.as_ref().unwrap())?;
    let input_xonly_pubkey = input_pubkey.x_only_public_key().0;

    let outputs = [output.to_owned()].to_vec();

    let lock_time = absolute::LockTime::from_height(block_height)?;

    let input_txid = Txid::from_str(&coin.utxo_txid.as_ref().unwrap())?;
    let input_vout = coin.utxo_vout.unwrap();

    let tx1 = Transaction {
        version: Version::TWO,
        lock_time,
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: input_txid,
                vout: input_vout,
            },
            script_sig: ScriptBuf::new(),
            sequence: bitcoin::Sequence(0xFFFFFFFF), // Ignore nSequence.
            witness: Witness::default(),
        }],
        output: outputs,
    };

    let mut psbt = Psbt::from_unsigned_tx(tx1)?;

    let input_amount = coin.amount.unwrap() as u64;

    let input_address =
        Address::from_str(&coin.aggregated_address.as_ref().unwrap())?.require_network(network)?;
    let input_scriptpubkey = input_address.script_pubkey();
    let mut input = Input {
        witness_utxo: Some(TxOut {
            value: Amount::from_sat(input_amount),
            script_pubkey: input_scriptpubkey,
        }),
        ..Default::default()
    };

    let ty = PsbtSighashType::from_str("SIGHASH_ALL")?;
    input.sighash_type = Some(ty);
    input.tap_internal_key = Some(input_xonly_pubkey.to_owned());
    psbt.inputs = vec![input];

    let unsigned_tx = psbt.unsigned_tx.clone();

    // There must not be more than one input.
    // The input is the funding transaction and the output the backup address.
    assert!(psbt.inputs.len() == 1);

    let vout = 0; // the vout is always 0 (only one input)
    let input = psbt.inputs.iter_mut().nth(vout).unwrap();

    let hash_ty = input
        .sighash_type
        .and_then(|psbt_sighash_type| psbt_sighash_type.taproot_hash_ty().ok())
        .unwrap_or(TapSighashType::All);

    let hash = SighashCache::new(&unsigned_tx).taproot_key_spend_signature_hash(
        vout,
        &sighash::Prevouts::All(&[TxOut {
            value: input.witness_utxo.as_ref().unwrap().value,
            script_pubkey: input.witness_utxo.as_ref().unwrap().script_pubkey.clone(),
        }]),
        hash_ty,
    )?;

    let tx_bytes = bitcoin::consensus::encode::serialize(&unsigned_tx);
    let encoded_unsigned_tx = hex::encode(tx_bytes);

    let session = calculate_musig_session(coin, hash, encoded_unsigned_tx)?;

    Ok(session)
}
