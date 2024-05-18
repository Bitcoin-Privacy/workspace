use crate::controller::statechain;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/tokens", web::get().to(statechain::create_token));
    cfg.route("/deposit", web::post().to(statechain::deposit));
    cfg.route("/{id}/nonce", web::post().to(statechain::get_nonce));
    cfg.route("/{id}/sig", web::post().to(statechain::get_sig));
    cfg.route("/{id}/withdraw", web::post().to(statechain::withdraw));

    cfg.route(
        "/transfer/key-register",
        web::post().to(statechain::register_key),
    );
    cfg.route(
        "/transfer/transfer-message",
        web::post().to(statechain::create_transfer_message),
    );
    cfg.route(
        "/transfer/transfer-message/{authkey}",
        web::get().to(statechain::get_transfer_message),
    );
    cfg.route(
        "/transfer/verify",
        web::post().to(statechain::verify_statecoin),
    );
    cfg.route(
        "/transfer/update-key",
        web::post().to(statechain::update_key),
    );

    cfg.route("/transfer-ownership", web::post().to(statechain::transfer));
    cfg.route("/statecoins", web::get().to(statechain::list_statecoins));
    cfg.route("/update-key", web::post().to(statechain::update_key));
}
