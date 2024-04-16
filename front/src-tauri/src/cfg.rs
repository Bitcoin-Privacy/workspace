pub const PASSPHRASE: &str = "correct horse battery staple";

pub const BASE_TX_FEE: u64 = 2600;
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
