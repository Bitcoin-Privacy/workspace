use shared::intf::coinjoin::{GetStatusRes, GetUnsignedTxnRes};

use bitcoin::{
    consensus,
    secp256k1::{Message, Secp256k1, SecretKey},
    sighash::SighashCache,
    Amount, EcdsaSighashType, ScriptBuf, Transaction, Witness,
};
use tauri::State;
use wallet::core::{MasterAccount, Unlocker};

use crate::svc::account::parse_derivation_path;
use crate::svc::coinjoin;
use crate::{
    cfg::PASSPHRASE, db::PoolWrapper, model::RoomEntity, store::master_account::get_master,
};

/// Register to CoinJoin Protocol
#[tauri::command]
pub async fn register(
    pool: State<'_, PoolWrapper>,
    // window: tauri::Window,
    deriv: &str,
    address: &str,
    amount: u64,
) -> Result<(), String> {
    coinjoin::register(&pool, deriv, amount, address)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn sign_tx(
    pool: State<'_, PoolWrapper>,
    deriv: &str,
    room_id: &str,
) -> Result<(), String> {
    let master_account = get_master().expect("Master account does not exist");
    let parsed_path = parse_derivation_path(deriv).map_err(|e| e.to_string())?;
    let account = master_account.accounts().get(&parsed_path).unwrap();

    let res = crate::api::coinjoin::get_txn(&room_id).await.unwrap();
    let parsed_tx =
        consensus::deserialize::<Transaction>(&hex::decode(&res.tx.clone()).unwrap()).unwrap();

    let room = pool
        .get_room(deriv, &room_id)
        .map_err(|e| format!("Error: {:?}", e))
        .unwrap();

    // TODO:
    // - find inputs which is from this account
    // - sign all these inputs
    let mut unsigned_tx = parsed_tx.clone();

    let sighash_type = EcdsaSighashType::All;
    let mut sighasher = SighashCache::new(&mut unsigned_tx);

    let future_tasks: Vec<_> = parsed_tx
        .input
        .iter()
        .enumerate()
        .filter(|(_, input)| {
            room.utxos
                .iter()
                .find(|utxo| input.previous_output.txid.to_string() == utxo.txid.to_string())
                .is_some()
        })
        .map(|(index, input)| tokio::spawn(coinjoin::find_and_join_txn(index, input.clone())))
        .collect();
    let mut results = Vec::new();
    for job in future_tasks {
        results.push(job.await.unwrap().unwrap());
    }

    let mut unlocker = Unlocker::new_for_master(&master_account, PASSPHRASE).unwrap();
    let secp = Secp256k1::new();

    let vins: Vec<u16> = results
        .iter()
        .map(|(index, input, tx)| {
            let vout = tx
                .vout
                .get(input.previous_output.vout as usize)
                .expect("Cannot get the vout");
            let amount = Amount::from_sat(vout.value);
            println!(
                "Script code 1: {}",
                ScriptBuf::from_hex(&vout.scriptpubkey).unwrap()
            );
            let script_pubkey =
                ScriptBuf::from_hex(&vout.scriptpubkey).expect("Invalid script public key");

            let sighash = sighasher
                .p2wpkh_signature_hash(*index, &script_pubkey, amount, sighash_type)
                .expect("failed to create sighash");

            let priv_key = account
                .get_privkey(script_pubkey.clone(), &mut unlocker)
                .expect("Cannot get private key");
            // input.script_sig = ScriptBuf::new();
            let msg = Message::from(sighash);
            let sk = SecretKey::from_slice(&priv_key.to_bytes()).unwrap();

            let sig = secp.sign_ecdsa(&msg, &sk);

            // Update the witness stack.
            let signature = bitcoin::ecdsa::Signature {
                sig,
                hash_ty: EcdsaSighashType::All,
            };

            let pk = sk.public_key(&secp);
            *sighasher.witness_mut(*index).unwrap() = Witness::p2wpkh(&signature, &pk);
            *index as u16
        })
        .collect();
    let tx_hex = consensus::encode::serialize_hex(&unsigned_tx);
    println!("hash: {:?}", tx_hex);
    println!("{:#?}", unsigned_tx);

    let res = crate::api::coinjoin::sign(room_id, vins, &tx_hex).await;
    match res {
        Ok(response) => {
            println!("RES {:#?}", response);
            Ok(())
        }
        Err(e) => Err(format!("Error: {}", e)),
    }
}

#[tauri::command]
pub async fn get_tx(room_id: &str) -> Result<GetUnsignedTxnRes, String> {
    let res = crate::api::coinjoin::get_txn(room_id).await;
    match res {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error: {}", e)),
    }
}

#[tauri::command]
pub async fn get_status(room_id: &str) -> Result<GetStatusRes, String> {
    let res = crate::api::coinjoin::get_status(room_id).await;
    match res {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error: {}", e)),
    }
}

#[tauri::command]
pub async fn get_rooms(
    pool: State<'_, PoolWrapper>,
    deriv: &str,
) -> Result<Vec<RoomEntity>, String> {
    pool.get_all_rooms(deriv)
        .map_err(|e| format!("Error: {:?}", e))
}
