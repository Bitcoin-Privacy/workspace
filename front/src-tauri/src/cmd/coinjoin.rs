use std::ops::ControlFlow;

use bitcoin::{
    consensus, secp256k1::Secp256k1, sighash::SighashCache, EcdsaSighashType, Transaction,
};
use tauri::State;

use shared::intf::coinjoin::{GetStatusRes, GetUnsignedTxnRes};

use crate::{
    db::PoolWrapper,
    model::RoomEntity,
    svc::{account, coinjoin},
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
    let (account, mut unlocker) = account::get_account(deriv).unwrap();

    let res = crate::api::coinjoin::get_txn(&room_id).await.unwrap();
    let parsed_tx =
        consensus::deserialize::<Transaction>(&hex::decode(&res.tx.clone()).unwrap()).unwrap();

    let room = pool
        .get_room(deriv, &room_id)
        .map_err(|e| format!("Error: {:?}", e))
        .unwrap();

    let mut unsigned_tx = parsed_tx.clone();

    let secp = Secp256k1::new();
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
        .map(|(index, input)| tokio::spawn(account::find_and_join_txn(index, input.clone())))
        .collect();

    let mut results = Vec::new();
    for job in future_tasks {
        results.push(job.await.unwrap().unwrap());
    }

    let mut vins: Vec<u16> = Vec::new();
    let res = results.iter().try_for_each(|(index, input, tx)| {
        vins.push(*index as u16);
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
        return Err(e.to_string());
    }

    let tx_hex = consensus::encode::serialize_hex(&unsigned_tx);
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
