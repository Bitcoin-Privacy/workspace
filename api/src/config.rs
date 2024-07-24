use dotenv::dotenv;
use lazy_static::lazy_static;

use shared::{
    blindsign::{BlindKeypair, BlindSession},
    util::get_env,
};

lazy_static! {
    pub static ref CFG: Config = Config::new();
}

pub struct Config {
    pub port: String,
    pub postgres_uri: String,
    pub coinjoin_fee: u32,
    pub due_time_1: u32,
    pub due_time_2: u32,
    pub blind_keypair: BlindKeypair,
    pub blind_session: BlindSession,
}

impl Config {
    fn new() -> Self {
        dotenv().ok(); // Load `.env` file

        let port = get_env::<String>("PORT", Some("6080".to_string()));
        let postgres_uri = get_env::<String>("POSTGRES_URI", None);

        // Coin join config
        let coinjoin_fee = get_env::<u32>("COINJOIN_FEE", None);
        let due_time_1 = get_env::<u32>("DUE_TIME_1", None);
        let due_time_2 = get_env::<u32>("DUE_TIME_2", None);

        // Blind sign config
        let blind_pubkey = get_env::<String>("BLIND_PUBLIC_KEY", None);
        let blind_privkey = get_env::<String>("BLIND_PRIVATE_KEY", None);
        let blind_keypair = BlindKeypair::from_strs(&blind_pubkey, &blind_privkey)
            .expect("Cannot load blind key pair");

        let blind_session_k = get_env::<String>("BLIND_SESSION_K", None);
        let blind_session =
            BlindSession::from_k(&blind_session_k).expect("Cannot load blind session k");

        Config {
            port,
            postgres_uri,
            coinjoin_fee,
            due_time_1,
            due_time_2,
            blind_keypair,
            blind_session,
        }
    }
}
