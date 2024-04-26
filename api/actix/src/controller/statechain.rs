use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use shared::intf::statechain::{
    CreateBkTxnReq, CreateTokenReq, DepositReq, GetNonceReq, GetPartialSignatureReq,
    ListStatecoinsReq, StatecoinDto, TransferReq, UpdateKeyReq,
};
use statechain_core::deposit::DepositMsg1;

use crate::{repo::statechain::StatechainRepo, svc::statechain, util::response};

pub async fn create_token(payload: Json<CreateTokenReq>) -> HttpResponse {
    response::success("hello from statechain endpoint")
}

pub async fn deposit(
    statechain_repo: Data<StatechainRepo>,
    payload: Json<DepositMsg1>,
) -> HttpResponse {
    match statechain::create_deposit(&statechain_repo, payload.0).await {
        Ok(status) => response::success(status),
        Err(message) => {
            println!("Deposit got error: {}", message);
            response::error(message.to_string())
        }
    }
}

pub async fn create_bk_txn(
    statechain_repo: Data<StatechainRepo>,
    payload: Json<CreateBkTxnReq>,
) -> HttpResponse {
    match statechain::sign_txn_bk(
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
    id: web::Path<String>,
    statechain_repo: Data<StatechainRepo>,
    payload: Json<GetNonceReq>,
) -> HttpResponse {
    let statechain_id = id.into_inner();
    match statechain::get_nonce(
        &statechain_repo,
        &statechain_id,
        &payload.signed_statechain_id,
    )
    .await
    {
        Ok(status) => response::success(status),
        Err(message) => {
            println!("get nonce got error: {}", message);
            response::error(message.to_string())
        }
    }
}

pub async fn get_sig(
    id: web::Path<String>,
    statechain_repo: Data<StatechainRepo>,
    payload: Json<GetPartialSignatureReq>,
) -> HttpResponse {
    let statechain_id = id.into_inner();

    match statechain::get_sig(
        &statechain_repo,
        &payload.serialized_key_agg_ctx,
        &statechain_id,
        &payload.signed_statechain_id,
        &payload.parsed_tx,
        &payload.agg_pubnonce,
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

pub async fn list_statecoins(
    statechain_repo: Data<StatechainRepo>,
    payload: Json<ListStatecoinsReq>,
) -> HttpResponse {
    match statechain::list_statecoins(&statechain_repo, &payload.addr).await {
        Ok(sc) => response::success(sc.iter().map(|e| e.into()).collect::<Vec<StatecoinDto>>()),
        Err(message) => {
            println!("Sign backup transaction got error: {}", message);
            response::error(message.to_string())
        }
    }
}

pub async fn update_key(payload: Json<UpdateKeyReq>) -> HttpResponse {
    response::success("hello from statechain endpoint")
}
