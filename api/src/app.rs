use actix_web::{
    http,
    middleware::{ErrorHandlers, Logger},
    web,
};

use crate::{db, middleware, repo, route};

pub fn config(cfg: &mut web::ServiceConfig, db: db::Database) {
    // Here you can configure app-wide middleware, etc.
    let logger = Logger::default();
    let coinjoin_repo = web::Data::new(repo::coinjoin::CoinJoinRepo::new(db.clone()));
    let statechain_repo = web::Data::new(repo::statechain::StatechainRepo::new(db.clone()));
    cfg.service(
        web::scope("")
            .app_data(coinjoin_repo)
            .app_data(statechain_repo)
            .wrap(logger)
            .wrap(ErrorHandlers::new().handler(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                middleware::add_error_header,
            ))
            // .wrap(middleware::LoggingMiddleware)
            .configure(route::config),
    );
}
