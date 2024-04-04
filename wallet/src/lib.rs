use bitcoin::key::Keypair;
use bitcoin::secp256k1::{rand, Secp256k1, SecretKey, Signing};

pub(crate) mod cfg;
pub mod core;
pub mod error;

pub fn new_keypair<C: Signing>(secp: &Secp256k1<C>) -> Keypair {
    let sk = SecretKey::new(&mut rand::thread_rng());
    Keypair::from_secret_key(secp, &sk)
}

#[cfg(test)]
mod test {
    use bitcoin::secp256k1::Secp256k1;
    use bitcoin::{Address, Network};

    #[test]
    fn address() {
        let secp = Secp256k1::new();
        let keypair = super::new_keypair(&secp);
        Address::p2tr(&secp, keypair.x_only_public_key().0, None, Network::Testnet);
    }
}
