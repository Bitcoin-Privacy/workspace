mod word_mnemonic;

pub use word_mnemonic::MNEMONIC;

pub const PASSPHRASE: &str = "correct horse battery staple";
pub const DATABASE_PATH: &str = "../../WalletDB";

pub const NODE_SERVICE_BASE_URL: &str = "http://localhost:6080";

pub const BASE_TX_FEE: u64 = 1000;
pub const COINJOIN_FEE: u64 = 150;
