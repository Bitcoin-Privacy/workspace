use crate::controller::blindsign;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    /*
     * Input:
     * Output:
     * - PublicKey
     * - RP
     */
    cfg.route("/session", web::get().to(blindsign::get_session));
}
