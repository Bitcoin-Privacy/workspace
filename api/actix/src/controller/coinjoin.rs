use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use shared::intf::coinjoin::{
    GetRoomByIdRes, GetStatusRes, GetUnsignedTxnRes, RegisterReq, RegisterRes, RoomDto,
    RoomListQuery, RoomQueryReq, SetOutputReq, SetOutputRes, SignTxnReq, SignTxnRes,
};
use shared::util;

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
        &coinjoin_repo,
        &payload.utxos,
        payload.amount,
        &payload.change_addr,
        &payload.blinded_out_addr,
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
        &payload.out_addr,
        &payload.sig,
    )
    .await
    {
        Ok(status) => response::success(SetOutputRes { status }),
        Err(message) => response::error(util::to_string(message)),
    }
}

/// Set signature for coinjoin transaction
///
pub async fn set_signature(
    coinjoin_repo: Data<CoinJoinRepo>,
    payload: Json<SignTxnReq>,
) -> HttpResponse {
    match coinjoin::set_sig(coinjoin_repo, &payload.room_id, &payload.vins, &payload.txn).await {
        Ok(status) => response::success(SignTxnRes {
            status: if status { 0 } else { 1 },
        }),
        Err(message) => response::error(util::to_string(message)),
    }
}

pub async fn get_room_list(
    coinjoin_repo: Data<CoinJoinRepo>,
    query: web::Query<RoomListQuery>,
) -> HttpResponse {
    match coinjoin::get_room_by_addr(coinjoin_repo, &query.address).await {
        Ok(tx) => response::success(tx.iter().map(|dto| dto.into()).collect::<Vec<RoomDto>>()),
        Err(message) => response::error(util::to_string(message)),
    }
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
        Err(message) => response::error(util::to_string(message)),
    }
}

pub async fn get_txn(
    coinjoin_repo: Data<CoinJoinRepo>,
    path: web::Path<RoomQueryReq>,
) -> HttpResponse {
    match coinjoin::get_txn_hex(coinjoin_repo, &path.id).await {
        Ok(tx) => response::success(GetUnsignedTxnRes { tx }),
        Err(message) => response::error(util::to_string(message)),
    }
}
