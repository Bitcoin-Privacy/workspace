use thiserror::Error;

/// The Error types
#[derive(Error, Debug)]
pub enum BlindSignError {
    #[error("failed to initialize the RNG")]
    RngInitFailed,
    #[error("failed to convert wired scalar to scalar")]
    WiredScalarMalformed,
    #[error("failed to convert wired ristretto point to ristretto point")]
    WiredRistrettoPointMalformed,

    #[error("Some error occured {:?}", _0)]
    Other(String),
}

/// The Result type used
pub type Result<T> = ::std::result::Result<T, BlindSignError>;

mod keypair;
mod request;
mod session;
mod signature;

pub use keypair::BlindKeypair;
pub use request::BlindRequest;
pub use session::BlindSession;
pub use signature::WiredUnblindedSigData;
