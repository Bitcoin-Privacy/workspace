//! Generate and manage the ECC keys
use super::BlindSignError;
use super::Result;
use curve25519_dalek::{
    constants::RISTRETTO_BASEPOINT_POINT,
    ristretto::{CompressedRistretto, RistrettoPoint},
    scalar::Scalar,
};
use rand::rngs::OsRng;

/// An elliptic curve cryptography keypair. The private key (Xs) is used by the
/// signer for creating the blind signature on the blinded hash(msg||R), and the
/// public key (Qs) is usable by anyone for verifying the authenticity of the
/// unblinded signature on the unblinded hash(msg||R).
#[derive(Copy, Clone, Debug)]
pub struct BlindKeypair {
    private: Scalar,
    public: RistrettoPoint,
}

impl BlindKeypair {
    /// Generates an ECC keypair for use with the blind signature protocol.
    /// The private key is a random scalar, and the public key is an elliptic
    /// curve point equal to this scalar multiplied by the Ristretto generator
    /// point. This is based on the wikipedia description of ECDSA key
    /// generation seeing as the whitepaper doesn't specify key generation.
    ///
    /// # Returns
    ///
    /// * Ok(BlindKeypair) on success.
    ///
    /// * Err(::Error) on error, which can only be the failure to initiate the
    /// internal RNG.
    ///
    /// # Mathematics
    ///
    /// * Xs = a randomly generated scalar
    /// * Qs = Xs * P
    /// * P = The ECC generator point
    pub fn generate() -> Self {
        let mut rng = OsRng;
        let private = Scalar::random(&mut rng);
        let public = private * RISTRETTO_BASEPOINT_POINT;
        BlindKeypair { private, public }
    }

    /// Creates a new BlindKeypair object from the provided private and public
    /// key components (in wired form).
    ///
    /// # Returns
    ///
    /// * Ok(BlindKeypair) on success.
    ///
    /// * Err(::Error) on failure, which can indicate either that the private
    /// or public key inputs were malformed.
    pub fn from_wired(private: [u8; 32], public: [u8; 32]) -> Result<Self> {
        let priv_key: Option<Scalar> = Scalar::from_canonical_bytes(private).into();
        Ok(BlindKeypair {
            private: priv_key.ok_or(BlindSignError::WiredScalarMalformed)?,
            public: CompressedRistretto(public)
                .decompress()
                .ok_or(BlindSignError::WiredRistrettoPointMalformed)?,
        })
    }

    /// Returns the private key in Scalar form
    pub fn private(&self) -> Scalar {
        self.private
    }

    /// Returns the public key in RistrettoPoint form
    pub fn public(&self) -> RistrettoPoint {
        self.public
    }

    /// Returns the public key in wired form
    pub fn public_wired(&self) -> [u8; 32] {
        self.public.compress().to_bytes()
    }

    /// Returns the private key in wired form
    pub fn private_wired(&self) -> [u8; 32] {
        self.private.to_bytes()
    }

    /// Returns the keypair from hex form
    pub fn from_strs(pubkey: &str, privkey: &str) -> Result<Self> {
        // Convert the hex strings back to byte arrays
        let private_key_bytes = hex::decode(privkey);
        let public_key_bytes = hex::decode(pubkey);

        // Assuming private_key_bytes and public_key_bytes are correct lengths ([u8; 32])
        let private_key_bytes: [u8; 32] = private_key_bytes
            .map_err(|e| BlindSignError::Other(e.to_string()))?
            .try_into()
            .map_err(|_| BlindSignError::Other("Private key has invalid sized?".to_string()))?;
        let public_key_bytes: [u8; 32] = public_key_bytes
            .map_err(|e| BlindSignError::Other(e.to_string()))?
            .try_into()
            .map_err(|_| BlindSignError::Other("Public key has invalid sized?".to_string()))?;

        // Use the from_wired function to create a BlindKeypair
        Self::from_wired(private_key_bytes, public_key_bytes)
    }
}
