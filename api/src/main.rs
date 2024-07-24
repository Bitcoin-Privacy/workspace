use db::TraitDatabase;

mod app;
mod config;
mod controller;
mod db;
mod middleware;
mod model;
mod repo;
mod route;
mod svc;
mod util;

use config::CFG;

use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    dotenv::dotenv().ok();

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    let mut db = db::Database::new().await;
    let result = db.init_database().await;
    println!("INIT DATABASE {:?}", result);

    let app = move |cfg: &mut ServiceConfig| app::config(cfg, db);

    Ok(app.into())
}
