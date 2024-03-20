use actix_web::{web::Json, HttpResponse};
use shared::intf::statechain::{
    CreateBkTxnReq, DepositReq, ListStatecoinsReq, TransferReq, UpdateKeyReq,
};

use crate::util::response;

pub async fn deposit(payload: Json<DepositReq>) -> HttpResponse {
    response::success("hello from statechain endpoint")
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
