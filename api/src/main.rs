use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use db::TraitDatabase;
use repo::coinjoin::CoinJoinRepo;
use std::io;

use crate::repo::statechain::StatechainRepo;

mod app;
mod config;
mod constance;
mod controller;
mod db;
mod middleware;
mod model;
mod repo;
mod route;
mod svc;
mod util;

use config::CFG;

// #[actix_web::main]
// async fn main() -> io::Result<()> {
//     dotenv::dotenv().ok();
//
//     std::env::set_var("RUST_LOG", "debug");
//     std::env::set_var("RUST_BACKTRACE", "1");
//
//     env_logger::init();
//
//     let mut db = db::Database::new().await;
//     let result = db.init_database().await;
//
//     println!("INIT DATABASE {:?}", result);
//
//     HttpServer::new(move || {
//         let logger = Logger::default();
//         let coinjoin_repo = Data::new(CoinJoinRepo::new(db.clone()));
//         let statechain_repo = Data::new(StatechainRepo::new(db.clone()));
//         App::new()
//             .wrap(logger)
//             .wrap(middleware::logging::LoggingMiddleware)
//             .app_data(coinjoin_repo)
//             .app_data(statechain_repo)
//             .configure(app::config)
//             .configure(route::config)
//     })
//     .bind(format!("127.0.0.1:{}", CFG.port))?
//     .run()
//     .await
// }

use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    dotenv::dotenv().ok();

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    // env_logger::init();

    let mut db = db::Database::new().await;
    let result = db.init_database().await;

    println!("INIT DATABASE {:?}", result);
    let config = move |cfg: &mut ServiceConfig| {
        let logger = Logger::default();
        let coinjoin_repo = Data::new(CoinJoinRepo::new(db.clone()));
        let statechain_repo = Data::new(StatechainRepo::new(db.clone()));
        cfg.service(
            actix_web::web::scope("/")
                .wrap(logger)
                .wrap(middleware::logging::LoggingMiddleware)
                .app_data(coinjoin_repo)
                .app_data(statechain_repo)
                .configure(app::config)
                .configure(route::config),
        );
    };

    Ok(config.into())
}
