use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use shared::{
    intf::statechain::{
        CreateBkTxnReq, CreateTokenReq, DepositReq, DepositRes, ListStatecoinsReq, SignFirstReq,
        SignFirstRes, SignSecondReq, TransferReq, UpdateKeyReq,
    },
    model::{resp, Status},
};

use crate::{
    repo::statechain::{StatechainRepo, TraitStatechainRepo},
    svc::statechain::{self, verify_signature},
    util::response,
};

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
        Err(message) => response::error(message),
    }
}

pub async fn sign_first(
    statechain_repo: Data<StatechainRepo>,
    payload: Json<SignFirstReq>,
) -> HttpResponse {
    let statechain_id = payload.0.statechain_id.clone();
    let r2_commitment = payload.0.r2_commitment.clone();
    let blind_commitment = payload.0.blind_commitment.clone();
    let signed_statechain_id = payload.0.signed_statechain_id.clone();

    if !statechain::verify_signature(&statechain_repo, &signed_statechain_id, &statechain_id).await
    {
        return response::error("Signature is invalid".to_string());
    }

    let secp = Secp256k1::new();
    let (sec_nonce, pub_nonce) = secp.generate_keypair(&mut rand::thread_rng());

    let response = SignFirstRes {
        server_pub_nonce: pub_nonce.to_string(),
    };

    let res = statechain_repo
        .insert_signature_data(
            &r2_commitment,
            &blind_commitment,
            &statechain_id,
            &pub_nonce,
            &sec_nonce,
        )
        .await;

    match res {
        Ok(_) => response::success(response),
        Err(message) => response::error(message),
    }
}

pub async fn sign_second(
    statechain_repo: Data<StatechainRepo>,
    payload: Json<SignSecondReq>,
) -> HttpResponse {
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
