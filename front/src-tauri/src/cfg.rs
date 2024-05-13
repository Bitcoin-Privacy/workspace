pub const PASSPHRASE: &str = "correct horse battery staple";

pub const INIT_NLOCKTIME: u64 = 60 * 60 * 24 * 60; // 60 days
pub const INTERVAL: u64 = 60 * 60 * 24 * 3; // 3 days
pub const BASE_TX_FEE: u64 = 7000;
// pub const COINJOIN_FEE: u64 = 150;

use dotenv::dotenv;
use lazy_static::lazy_static;

use shared::util::get_env;

lazy_static! {
    pub static ref CFG: Config = Config::new();
}

pub struct Config {
    pub database_url: String,
    pub service_url: String,
}

impl Config {
    fn new() -> Self {
        dotenv().ok();
        let database_url = get_env::<String>("SQLITE_URL", None);
        let service_url = get_env::<String>("SERVICE_URL", None);
        Config {
            database_url,
            service_url,
        }
    }
}
