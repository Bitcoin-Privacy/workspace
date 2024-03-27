use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use shared::intf::statechain::{
    CreateBkTxnReq, CreateTokenReq, DepositReq, DepositRes, ListStatecoinsReq, TransferReq, UpdateKeyReq
};

use crate::{
    repo::statechain::{StatechainRepo,TraitStatechainRepo}, svc::statechain, util::response
};


pub async fn create_token(payload: Json<CreateTokenReq>) -> HttpResponse {
    response::success("hello from statechain endpoint")
}

pub async fn deposit(statechain_repo: Data<StatechainRepo>, payload: Json<DepositReq>) -> HttpResponse {
    match statechain::create_deposit(&statechain_repo, &payload.token_id, &payload.addr, payload.amount).await {
        Ok(status) =>response::success(status),
        Err(message) => response::error(message),
    }
    
}

pub async fn create_bk_txn(payload: Json<CreateBkTxnReq>) -> HttpResponse { 
    response::success("hello from statechain endpoint")
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
