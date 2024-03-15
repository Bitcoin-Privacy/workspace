/// Key derivation detail information
/// coordinates of a key as defined in BIP32 and BIP44
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyDerivation {
    /// m / purpose' / coin_type' / account' / sub / kix
    pub account: u32,
    /// m / purpose' / coin_type' / account' / sub / kix
    pub sub: u32,
    /// m / purpose' / coin_type' / account' / sub / kix
    pub kix: u32,
    /// optional additive tweak to private key
    pub tweak: Option<Vec<u8>>,
    /// optional number of blocks this can not be spent after confirmation (OP_CSV)
    pub csv: Option<u16>,
}
