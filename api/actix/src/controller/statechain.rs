use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use shared::intf::statechain::{
    CreateBkTxnReq, CreateTokenReq, DepositReq, GetNonceReq, GetPartialSignatureReq,
    GetPartialSignatureRes, KeyRegisterReq, ListStatecoinsReq, TransferReq, UpdateKeyReq,
    UpdateTransferMessageReq,
};

use crate::{repo::statechain::StatechainRepo, svc::statechain, util::response};

pub async fn register_key(
    statechain_repo: Data<StatechainRepo>,
    payload: Json<KeyRegisterReq>,
) -> HttpResponse {
    //check signature corresponding to Authkey

    match statechain::register_key(
        &statechain_repo,
        &payload.statechain_id,
        &payload.auth_pubkey_2,
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
    id: web::Path<String>,
    statechain_repo: Data<StatechainRepo>,
    payload: Json<GetNonceReq>,
) -> HttpResponse {
    let statechain_id = id.into_inner();
    if !statechain::verify_signature(
        &statechain_repo,
        &payload.signed_statechain_id,
        &statechain_id,
    )
    .await
    .unwrap()
    {
        return HttpResponse::Unauthorized().body("invalid signature for id");
    }
    match statechain::get_nonce(&statechain_repo, &statechain_id).await {
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
    if !statechain::verify_signature(
        &statechain_repo,
        &payload.signed_statechain_id,
        &statechain_id,
    )
    .await
    .unwrap()
    {
        return HttpResponse::Unauthorized().body("invalid signature for id");
    }
    match statechain::get_sig(
        &statechain_repo,
        &payload.serialized_key_agg_ctx,
        &statechain_id,
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

pub async fn update_transfer_message(
    statechain_repo: Data<StatechainRepo>,
    payload: Json<UpdateTransferMessageReq>,
) -> HttpResponse {
    match statechain::update_tranfer_message(
        &statechain_repo,
        &payload.authkey,
        &payload.transfer_msg,
    )
    .await
    {
        Ok(status) => response::success(status),
        Err(message) => {
            println!("Update transfer get error: {}", message);
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
