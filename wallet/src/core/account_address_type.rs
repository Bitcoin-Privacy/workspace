use serde::{Deserialize, Serialize};

/// Address type an account is using
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum AddrType {
    /// legacy pay to public key hash (BIP44)
    P2PKH,
    /// transitional segwit pay to public key hash in legacy format (BIP49)
    P2SHWPKH,
    /// native segwit pay to public key hash in bech format (BIP84)
    P2WPKH,
    /// native segwit pay to script
    /// do not use 44, 49 or 84 for this parameter, to avoid confusion with above types
    /// Only supports scripts that can be spent with following witness:
    /// <signature> <scriptCode>
    P2WSH(u32),
}

impl AddrType {
    pub fn as_u32(&self) -> u32 {
        match self {
            AddrType::P2PKH => 44,
            AddrType::P2SHWPKH => 49,
            AddrType::P2WPKH => 84,
            AddrType::P2WSH(n) => *n,
        }
    }

    pub fn from_u32(n: u32) -> AddrType {
        match n {
            44 => AddrType::P2PKH,
            49 => AddrType::P2SHWPKH,
            84 => AddrType::P2WPKH,
            n => AddrType::P2WSH(n),
        }
    }
}
