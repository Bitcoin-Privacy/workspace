use crypto::{
    aes, blockmodes, buffer,
    buffer::{BufferResult, ReadBuffer, WriteBuffer},
    digest::Digest,
    sha2::Sha256,
};

use crate::error::Error;

/// seed of the master key
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Seed(pub Vec<u8>);

impl Seed {
    /// encrypt seed
    /// encryption algorithm: AES256(Sha256(passphrase), ECB, PKCS padding
    pub fn encrypt(&self, passphrase: &str) -> Result<Vec<u8>, Error> {
        let mut key = [0u8; 32];
        let mut sha2 = Sha256::new();
        sha2.input(passphrase.as_bytes());
        sha2.result(&mut key);

        let mut encryptor =
            aes::ecb_encryptor(aes::KeySize::KeySize256, &key, blockmodes::PkcsPadding {});
        let mut encrypted = Vec::new();
        let mut reader = buffer::RefReadBuffer::new(self.0.as_slice());
        let mut buffer = [0u8; 1024];
        let mut writer = buffer::RefWriteBuffer::new(&mut buffer);
        loop {
            let result = encryptor.encrypt(&mut reader, &mut writer, true)?;
            encrypted.extend(
                writer
                    .take_read_buffer()
                    .take_remaining()
                    .iter()
                    .map(|i| *i),
            );
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => {}
            }
        }
        Ok(encrypted)
    }

    /// decrypt seed
    /// decryption algorithm: AES256(Sha256(passphrase), ECB, PKCS padding
    pub fn decrypt(encrypted: &[u8], passphrase: &str) -> Result<Seed, Error> {
        let mut key = [0u8; 32];
        let mut sha2 = Sha256::new();
        sha2.input(passphrase.as_bytes());
        sha2.result(&mut key);

        let mut decrypted = Vec::new();
        let mut reader = buffer::RefReadBuffer::new(encrypted);
        let mut buffer = [0u8; 1024];
        let mut writer = buffer::RefWriteBuffer::new(&mut buffer);
        let mut decryptor =
            aes::ecb_decryptor(aes::KeySize::KeySize256, &key, blockmodes::PkcsPadding {});
        loop {
            let result = decryptor.decrypt(&mut reader, &mut writer, true)?;
            decrypted.extend(
                writer
                    .take_read_buffer()
                    .take_remaining()
                    .iter()
                    .map(|i| *i),
            );
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => {}
            }
        }

        Ok(Seed(decrypted))
    }
}
