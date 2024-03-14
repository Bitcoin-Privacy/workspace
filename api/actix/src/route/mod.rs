use actix_web::web;

mod blindsign;
mod coinjoin;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("coinjoin").configure(coinjoin::config))
        .service(web::scope("blindsign").configure(blindsign::config));
}
