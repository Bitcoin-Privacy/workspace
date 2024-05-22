use actix_web::web;

mod blindsign;
mod coinjoin;
mod statechain;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("blindsign").configure(blindsign::config))
        .service(web::scope("coinjoin").configure(coinjoin::config))
        .service(web::scope("statechain").configure(statechain::config));
}
