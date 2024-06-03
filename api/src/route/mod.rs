use actix_web::web;

mod blindsign;
mod coinjoin;
mod statechain;

use crate::{db, repo, svc};

pub fn config(cfg: &mut web::ServiceConfig, db: db::Database) {
    let coinjoin_repo = repo::coinjoin::CoinjoinRepo::new(db.clone());
    let coinjoin_service = web::Data::new(svc::CoinjoinService::new(coinjoin_repo));

    let statechain_repo = web::Data::new(repo::statechain::StatechainRepo::new(db.clone()));

    cfg.service(web::scope("blindsign").configure(blindsign::config))
        .service(
            web::scope("coinjoin")
                .app_data(coinjoin_service)
                .configure(coinjoin::config),
        )
        .service(
            web::scope("statechain")
                .app_data(statechain_repo)
                .configure(statechain::config),
        );
}
