use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use shared::intf::statechain::{
    CreateBkTxnReq, CreateTokenReq, DepositReq, GetNonceReq, GetPartialSignatureReq, GetPartialSignatureRes, ListStatecoinsReq, TransferReq, UpdateKeyReq
};

use crate::{repo::statechain::StatechainRepo, svc::statechain, util::response};

pub async fn create_token(payload: Json<CreateTokenReq>) -> HttpResponse {
    response::success("hello from statechain endpoint")
}

pub async fn deposit(
    statechain_repo: Data<StatechainRepo>,
    payload: Json<DepositReq>,
) -> HttpResponse {
    match statechain::create_deposit(
        &statechain_repo,
        &payload.token_id,
        &payload.addr,
        payload.amount,
    )
    .await
    {
        Ok(status) => response::success(status),
        Err(message) => {
            println!("Deposit got error: {}", message);
            response::error(message)
        }
    }
}

pub async fn create_bk_txn(
    statechain_repo: Data<StatechainRepo>,
    payload: Json<CreateBkTxnReq>,
) -> HttpResponse {
    match statechain::create_bk_txn(
        &statechain_repo,
        &payload.statechain_id,
        &payload.scriptpubkey,
        &payload.txn_bk,
    )
    .await
    {
        Ok(status) => response::success(status),
        Err(message) => {
            println!("Sign backup transaction got error: {}", message);
            response::error(message.to_string())
        }
    }
}

pub async fn get_nonce(
    statechain_repo: Data<StatechainRepo>,
    payload: Json<GetNonceReq>,
) -> HttpResponse {
    match statechain::get_nonce(
        &statechain_repo,
        &payload.statechain_id,
        &payload.signed_statechain_id,
    )
    .await
    {
        Ok(status) => response::success(status),
        Err(message) => {
            println!("Sign backup transaction got error: {}", message);
            response::error(message.to_string())
        }
    }
}

pub async fn get_sig(
    statechain_repo: Data<StatechainRepo>,
    payload: Json<GetPartialSignatureReq>,
) -> HttpResponse{

    match statechain::get_sig(
        &statechain_repo,
        &payload.serialized_key_agg_ctx,
        &payload.statechain_id,
        &payload.signed_statechain_id,
        &payload.parsed_tx,
        &payload.agg_pubnonce
    )
    .await
    {
        Ok(status) => response::success(status),
        Err(message) => {
            println!("Sign backup transaction got error: {}", message);
            response::error(message.to_string())
        }
    }
}

pub async fn transfer(payload: Json<TransferReq>) -> HttpResponse {
    response::success("hello from statechain endpoint")
}

pub async fn list_statecoins(payload: Json<ListStatecoinsReq>) -> HttpResponse {
    response::success("hello from statechain endpoint")
}

pub async fn update_key(payload: Json<UpdateKeyReq>) -> HttpResponse {
    response::success("hello from statechain endpoint")
}
