use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use db::TraitDatabase;
use repo::coinjoin::CoinJoinRepo;
use std::io;

pub mod app;
pub mod config;
pub mod constance;
pub mod controller;
pub mod db;
pub mod middleware;
pub mod model;
pub mod repo;
pub mod route;
pub mod svc;
pub mod util;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    let mut db = db::Database::new().await;
    let result = db.init_database().await;

    println!("INIT DATABASE {:?}", result);

    HttpServer::new(move || {
        let logger = Logger::default();
        let coinjoin_repo = Data::new(CoinJoinRepo::new(db.clone()));
        App::new()
            .wrap(logger)
            .wrap(middleware::logging::LoggingMiddleware)
            .app_data(coinjoin_repo)
            .configure(app::config)
            .configure(route::config)
    })
    .bind(format!("127.0.0.1:{}", config::CONFIG.port))?
    .run()
    .await
}
