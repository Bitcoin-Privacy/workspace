use crate::controller::statechain;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/tokens", web::get().to(statechain::create_token));
    cfg.route("/deposit", web::post().to(statechain::deposit));
    cfg.route("/create-bk-txn", web::post().to(statechain::create_bk_txn));
    cfg.route("/transfer-ownership", web::post().to(statechain::transfer));
    cfg.route("/statecoins", web::get().to(statechain::list_statecoins));
    cfg.route("/update-key", web::post().to(statechain::update_key));
}
