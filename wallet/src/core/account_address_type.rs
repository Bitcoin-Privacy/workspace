use crate::error::Error;
use serde::{Deserialize, Serialize};

/// Address type an account is using
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum AddrType {
    /// legacy pay to public key hash (BIP44)
    P2PKH,
    /// native segwit pay to public key hash in bech format (BIP84)
    P2WPKH,
}

impl AddrType {
    pub fn as_u32(&self) -> u32 {
        match self {
            AddrType::P2PKH => 44,
            AddrType::P2WPKH => 84,
        }
    }

    pub fn from_u32(n: u32) -> Result<AddrType, Error> {
        match n {
            44 => Ok(AddrType::P2PKH),
            84 => Ok(AddrType::P2WPKH),
            _ => Err(Error::Unsupported("Do not support this key type")),
        }
    }
}
