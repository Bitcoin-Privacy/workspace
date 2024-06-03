use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use bitcoin::{consensus, Transaction};
use shared::intf::coinjoin::{
    CoinjoinRegisterReq, CoinjoinRegisterRes, GetRoomByIdRes, GetStatusRes, GetUnsignedTxnRes,
    RoomDto, RoomListQuery, RoomQueryReq, SetOutputReq, SetOutputRes, SignTxnReq, SignTxnRes,
};

use crate::{
    svc::{account, CoinjoinService},
    util::response,
};

/// Register to CoinJoin Room
/// - Verify UTXOs and proofs
/// - Find/create peer room -> add to this room
/// - Blind sign output address
pub async fn register(
    coinjoin_service: Data<CoinjoinService>,
    payload: Json<CoinjoinRegisterReq>,
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

    match coinjoin_service
        .register(
            &payload.utxos,
            payload.amount,
            &payload.change_addr,
            &payload.blinded_out_addr,
        )
        .await
    {
        Ok((room, sig)) => response::success(CoinjoinRegisterRes {
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
    coinjoin_service: Data<CoinjoinService>,
    payload: Json<SetOutputReq>,
) -> HttpResponse {
    match coinjoin_service
        .set_output(&payload.room_id, &payload.out_addr, &payload.sig)
        .await
    {
        Ok(status) => response::success(SetOutputRes { status }),
        Err(message) => response::error(message),
    }
}

/// Set signature for coinjoin transaction
///
pub async fn set_signature(
    coinjoin_service: Data<CoinjoinService>,
    payload: Json<SignTxnReq>,
) -> HttpResponse {
    match coinjoin_service
        .set_sig(&payload.room_id, &payload.vins, &payload.txn)
        .await
    {
        Ok(status) => response::success(SignTxnRes { status }),
        Err(message) => response::error(message.to_string()),
    }
}

pub async fn get_room_list(
    coinjoin_service: Data<CoinjoinService>,
    query: web::Query<RoomListQuery>,
) -> HttpResponse {
    match coinjoin_service.get_room_by_addr(&query.address).await {
        Ok(tx) => response::success(tx.iter().map(|dto| dto.into()).collect::<Vec<RoomDto>>()),
        Err(e) => response::error(e),
    }
}

pub async fn get_room_by_id(
    coinjoin_service: Data<CoinjoinService>,
    path: web::Path<RoomQueryReq>,
) -> HttpResponse {
    let room = coinjoin_service.get_room_by_id(&path.id).await.unwrap();

    let res: GetRoomByIdRes = room.into();

    response::success(res)
}

pub async fn get_status(
    coinjoin_service: Data<CoinjoinService>,
    path: web::Path<RoomQueryReq>,
) -> HttpResponse {
    match coinjoin_service.get_room_by_id(&path.id).await {
        Ok(room) => response::success(GetStatusRes {
            status: room.status,
        }),
        Err(e) => response::error(e),
    }
}

pub async fn get_txn(
    coinjoin_service: Data<CoinjoinService>,
    path: web::Path<RoomQueryReq>,
) -> HttpResponse {
    match coinjoin_service.get_txn_hex(&path.id).await {
        Ok(tx) => response::success(GetUnsignedTxnRes { tx }),
        Err(e) => response::error(e),
    }
}
