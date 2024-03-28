pub const PASSPHRASE: &str = "correct horse battery staple";
pub const DATABASE_PATH: &str = "../../WalletDB";

pub const NODE_SERVICE_BASE_URL: &str = "http://localhost:6080";

pub const BASE_TX_FEE: u64 = 1000;
pub const COINJOIN_FEE: u64 = 150;

use dotenv::dotenv;
use lazy_static::lazy_static;

use shared::util::get_env;

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

pub struct Config {
    pub database_url: String,
}

impl Config {
    fn new() -> Self {
        dotenv().ok();
        let database_url = get_env::<String>("SQLITE_URL", None);
        Config { database_url }
    }
}
