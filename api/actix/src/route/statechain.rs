use crate::controller::statechain;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    // /* Input:
    //  * - Room id
    //  * Output:
    //  * - Transaction (hex - string)
    //  */
    // cfg.route("/room/{id}/txn", web::get().to(coinjoin::get_txn));

    cfg.route("/hello", web::get().to(statechain::hello));
}
