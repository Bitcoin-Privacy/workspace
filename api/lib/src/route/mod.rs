use actix_web::web::{self, Data};

use crate::repo::coinjoin::CoinJoinRepo;

mod blindsign;
mod coinjoin;

pub fn config(cfg: &mut web::ServiceConfig) {
    let coinjoin_repo = CoinJoinRepo::new().await;
    let room_data = Data::new(coinjoin_repo.clone());
    cfg.service(
        web::scope("coinjoin")
            .app_data(room_data)
            .configure(coinjoin::config),
    )
    .service(web::scope("blindsign").configure(blindsign::config));
}
