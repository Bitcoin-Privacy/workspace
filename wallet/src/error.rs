use std::{convert, error, fmt, io};

use bitcoin::bip32;
use crypto::symmetriccipher;

/// An error class to offer a unified error interface upstream
pub enum Error {
    /// Unsupported
    Unsupported(&'static str),
    /// mnemonic related error
    Mnemonic(&'static str),
    /// wrong passphrase
    Passphrase,
    /// wrong network
    Network,
    /// Network IO error
    IO(io::Error),
    /// key derivation error
    KeyDerivation(bip32::Error),
    /// sekp256k1 error
    SecpError(bitcoin::secp256k1::Error),
    /// cipher error
    SymmetricCipherError(symmetriccipher::SymmetricCipherError),
    /// sighash error
    SigHash(bitcoin::sighash::Error),
    PushBytesError(bitcoin::script::PushBytesError),
}
impl error::Error for Error {
    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Network => None,
            Error::Passphrase => None,
            Error::Unsupported(_) => None,
            Error::Mnemonic(_) => None,
            Error::IO(ref err) => Some(err),
            Error::KeyDerivation(ref err) => Some(err),
            Error::SecpError(ref err) => Some(err),
            Error::SymmetricCipherError(_) => None,
            Error::SigHash(_) => None,
            Error::PushBytesError(_) => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            Error::Passphrase => write!(f, "wrong passphrase"),
            Error::Network => write!(f, "wrong network"),
            Error::Unsupported(ref s) => write!(f, "Unsupported: {}", s),
            Error::Mnemonic(ref s) => write!(f, "Mnemonic: {}", s),
            Error::IO(ref err) => write!(f, "IO error: {}", err),
            Error::KeyDerivation(ref err) => write!(f, "BIP32 error: {}", err),
            Error::SecpError(ref err) => write!(f, "Secp256k1 error: {}", err),
            Error::SymmetricCipherError(ref err) => write!(
                f,
                "Cipher error: {}",
                match err {
                    symmetriccipher::SymmetricCipherError::InvalidLength => "invalid length",
                    symmetriccipher::SymmetricCipherError::InvalidPadding => "invalid padding",
                }
            ),
            Error::SigHash(ref err) => write!(f, "Sighash error: {}", err),
            Error::PushBytesError(ref err) => write!(f, "PushBytesError error: {}", err),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &dyn fmt::Display).fmt(f)
    }
}

impl convert::From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        match err {
            Error::IO(e) => e,
            _ => io::Error::new(io::ErrorKind::Other, err.to_string()),
        }
    }
}

impl convert::From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl convert::From<bip32::Error> for Error {
    fn from(err: bip32::Error) -> Error {
        Error::KeyDerivation(err)
    }
}

impl convert::From<symmetriccipher::SymmetricCipherError> for Error {
    fn from(err: symmetriccipher::SymmetricCipherError) -> Error {
        Error::SymmetricCipherError(err)
    }
}

impl convert::From<bitcoin::secp256k1::Error> for Error {
    fn from(err: bitcoin::secp256k1::Error) -> Error {
        Error::SecpError(err)
    }
}
