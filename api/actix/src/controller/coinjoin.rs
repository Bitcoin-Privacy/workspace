use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use bitcoin::{consensus, Transaction};
use shared::intf::coinjoin::{
    GetRoomByIdRes, GetStatusReq, GetStatusRes, GetUnsignedTxnReq, GetUnsignedTxnRes, RegisterReq,
    RegisterRes, RoomQueryReq, SetOutputReq, SetOutputRes, SignTxnReq,
};

use crate::{
    repo::coinjoin::{CoinJoinRepo, TraitCoinJoinRepo},
    svc::{account, coinjoin},
    util::response,
};

/// Register to CoinJoin Room
/// - Verify UTXOs and proofs
/// - Find/create peer room -> add to this room
/// - Blind sign output address
pub async fn register(
    coinjoin_repo: Data<CoinJoinRepo>,
    payload: Json<RegisterReq>,
) -> HttpResponse {
    // Check valid UTXOs
    if let Err(err) = account::validate_utxos(&payload.utxos).await {
        return response::error(err);
    }

    // Validate proof_signatures
    payload
        .utxos
        .iter()
        .zip(payload.proofs.iter())
        .map(|(utxo, proof)| account::proof_validator(utxo, proof))
        .reduce(|acc, e| acc && e);

    match coinjoin::register(
        coinjoin_repo,
        payload.utxos.clone(),
        payload.amount,
        &payload.change_address,
        &payload.blinded_output_address,
    )
    .await
    {
        Ok((room, sig)) => response::success(RegisterRes {
            room: room.into(),
            utxos: payload.utxos.clone(),
            signed_blined_output: sig,
        }),
        Err(e) => response::error(format!("Failed: {}", e)),
    }
}

/// Set output address
/// - Set plain output address with a sig
pub async fn set_output(
    coinjoin_repo: Data<CoinJoinRepo>,
    payload: Json<SetOutputReq>,
) -> HttpResponse {
    match coinjoin::set_output(
        coinjoin_repo,
        &payload.room_id,
        &payload.output_address,
        &payload.sig,
    )
    .await
    {
        Ok(status) => response::success(SetOutputRes { status }),
        Err(message) => response::error(message),
    }
}

pub async fn set_signature(
    coinjoin_repo: Data<CoinJoinRepo>,
    payload: Json<SignTxnReq>,
) -> HttpResponse {
    // TODO: verify sig
    let parsed_tx =
        consensus::deserialize::<Transaction>(&hex::decode(&payload.txn.clone()).unwrap()).unwrap();

    for vin in payload.vins.iter() {
        let signed_input = parsed_tx.input.get(*vin as usize);
        if let Some(signed_input) = signed_input {
            let witness = &signed_input.witness;
            if witness.is_empty() {
                return HttpResponse::BadRequest().into();
            };
            let result = coinjoin_repo
                .add_script(
                    &payload.room_id,
                    *vin,
                    &serde_json::to_string(signed_input).expect("Cannot encode input"),
                )
                .await;
            return match result {
                Ok(_) => continue,
                Err(e) => {
                    response::error("Cannot write your unlock script to database".to_string() + &e)
                }
            };
        } else {
            return response::error("Cannot get signed input".to_string());
        }
    }

    let completed =
        coinjoin::check_tx_completed(Data::clone(&coinjoin_repo), &payload.room_id).await;
    match completed {
        Ok(tx) => {
            let tx_hex = consensus::encode::serialize_hex(&tx);
            println!("TX: {:#?}", tx);
            println!("TX completed: {}", tx_hex)
        }
        Err(e) => println!("Check completed got error: {}", e),
    }

    response::success({})
}

pub async fn get_room_by_id(
    coinjoin_repo: Data<CoinJoinRepo>,
    path: web::Path<RoomQueryReq>,
) -> HttpResponse {
    let room = coinjoin_repo.get_room_by_id(&path.id).await.unwrap();

    let res: GetRoomByIdRes = room.into();

    response::success(res)
}

pub async fn get_status(
    coinjoin_repo: Data<CoinJoinRepo>,
    path: web::Path<RoomQueryReq>,
) -> HttpResponse {
    match coinjoin_repo.get_room_by_id(&path.id).await {
        Ok(room) => response::success(GetStatusRes {
            status: room.status,
        }),
        Err(e) => response::error(e),
    }
}

pub async fn get_transaction(
    coinjoin_repo: Data<CoinJoinRepo>,
    path: web::Path<RoomQueryReq>,
) -> HttpResponse {
    match coinjoin::get_txn_hex(coinjoin_repo, &path.id).await {
        Ok(tx) => response::success(GetUnsignedTxnRes { tx }),
        Err(e) => response::error(e),
    }
}
