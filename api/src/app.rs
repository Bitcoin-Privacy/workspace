use actix_web::{
    http,
    middleware::{ErrorHandlers, Logger},
    web,
};

use crate::{db, middleware, route};

pub fn config(cfg: &mut web::ServiceConfig, db: db::Database) {
    let logger = Logger::default();

    cfg.service(
        web::scope("")
            .wrap(logger)
            .wrap(ErrorHandlers::new().handler(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                middleware::add_error_header,
            ))
            // .wrap(middleware::LoggingMiddleware)
            .configure(|cfg| route::config(cfg, db)),
    );
}
