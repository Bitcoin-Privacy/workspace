/// The Error types
#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "failed to initialize the RNG")]
    RngInitFailed,
    #[fail(display = "failed to convert wired scalar to scalar")]
    WiredScalarMalformed,
    #[fail(display = "failed to convert wired ristretto point to ristretto point")]
    WiredRistrettoPointMalformed,

    #[fail(display = "Some error occured {:?}", _0)]
    Other(String),
}

/// The Result type used
pub type Result<T> = ::std::result::Result<T, Error>;

mod keypair;
mod request;
mod session;
mod signature;

use failure_derive::Fail;
pub use keypair::BlindKeypair;
pub use request::BlindRequest;
pub use session::BlindSession;
pub use signature::WiredUnblindedSigData;
