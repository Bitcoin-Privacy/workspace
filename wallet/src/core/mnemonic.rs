use std::{io::Cursor, str::FromStr};

use bitcoin::bip158::{BitStreamReader, BitStreamWriter};
use crypto::{
    digest::Digest,
    hmac::Hmac,
    pbkdf2::pbkdf2,
    sha2::{Sha256, Sha512},
};
use secp256k1::rand::{thread_rng, RngCore};

use super::{master_account::MasterKeyEntropy, seed::Seed};
use crate::{cfg::MNEMONIC, error::Error};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Mnemonic(Vec<usize>);

impl ToString for Mnemonic {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|i| MNEMONIC[*i])
            .collect::<Vec<_>>()
            .as_slice()
            .join(" ")
    }
}

impl FromStr for Mnemonic {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<_> = s.split(' ').collect();
        if words.len() < 6 || words.len() % 6 != 0 {
            return Err(Error::Mnemonic(
                "Mnemonic must have a word count divisible with 6",
            ));
        }
        let mut data = Vec::new();
        let mut writer = BitStreamWriter::new(&mut data);
        let mut mnemonic = Vec::new();
        for word in &words {
            if let Ok(idx) = MNEMONIC.binary_search(word) {
                mnemonic.push(idx);
                writer.write(idx as u64, 11).unwrap();
            } else {
                return Err(Error::Mnemonic("Mnemonic contains an unknown word"));
            }
        }
        writer.flush().unwrap();
        let l = data.len();
        let (payload, checksum) = data.split_at(l - if l > 33 { 2 } else { 1 });
        if Self::checksum(payload).as_slice() != checksum {
            return Err(Error::Mnemonic("Checksum failed"));
        }

        Ok(Mnemonic(mnemonic))
    }
}

impl Mnemonic {
    pub fn to_seed_phrase(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|i| MNEMONIC[*i].to_string())
            .collect::<Vec<String>>()
    }

    /// Create a seed from mnemonic
    /// with optional passphrase for plausible deniability see BIP39
    pub fn to_seed(&self, pd_passphrase: Option<&str>) -> Seed {
        let mut mac = Hmac::new(Sha512::new(), self.to_string().as_bytes());
        let mut output = [0u8; 64];
        let passphrase = "mnemonic".to_owned() + pd_passphrase.unwrap_or("");
        pbkdf2(&mut mac, passphrase.as_bytes(), 2048, &mut output);
        Seed(output.to_vec())
    }

    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(|s| MNEMONIC[*s])
    }

    pub fn new_random(entropy: MasterKeyEntropy) -> Result<Mnemonic, Error> {
        let len = match entropy {
            MasterKeyEntropy::Sufficient => 16,
            MasterKeyEntropy::Double => 32,
            MasterKeyEntropy::Paranoid => 64,
        };
        let mut random = vec![0u8; len];
        thread_rng().fill_bytes(random.as_mut_slice());
        Self::new(random.as_slice())
    }

    pub fn new_from_seed_phrase(seed_phrase: Vec<String>) -> Result<Self, Error> {
        let indices = seed_phrase
            .iter()
            .map(|word| {
                MNEMONIC
                    .iter()
                    .position(|&w| w == word)
                    .ok_or_else(|| Error::Mnemonic("Invalid seedphrase!"))
            })
            .collect::<Result<Vec<usize>, Error>>()?;

        Ok(Mnemonic(indices))
    }

    /// Create a mnemonic for some data
    pub fn new(data: &[u8]) -> Result<Mnemonic, Error> {
        if data.len() % 4 != 0 {
            return Err(Error::Mnemonic(
                "Data for mnemonic should have a length divisible by 4",
            ));
        }
        let mut with_checksum = data.to_vec();
        with_checksum.extend_from_slice(Self::checksum(data).as_slice());
        let mut cursor = Cursor::new(&with_checksum);
        let mut reader = BitStreamReader::new(&mut cursor);
        let mlen = data.len() * 3 / 4;
        let mut mnemonic = Vec::new();
        for _ in 0..mlen {
            mnemonic.push(reader.read(11).unwrap() as usize);
        }
        Ok(Mnemonic(mnemonic))
    }

    pub fn extend(&self) -> Result<Mnemonic, Error> {
        if self.0.len() != 12 {
            return Err(Error::Mnemonic(
                "Can only extend mnemonic of 12 words to 24 words",
            ));
        }
        let mut data = Vec::new();
        let mut writer = BitStreamWriter::new(&mut data);
        for idx in &self.0 {
            writer.write(*idx as u64, 11).unwrap();
        }
        for _ in 0..11 {
            writer.write(thread_rng().next_u64(), 11).unwrap();
        }
        writer.write(thread_rng().next_u64(), 3).unwrap();
        writer.flush().unwrap();
        data.extend_from_slice(Self::checksum(&data).as_slice());
        let mut cursor = Cursor::new(&data[..]);
        let mut reader = BitStreamReader::new(&mut cursor);
        let mut mnemonic = Vec::new();
        for _ in 0..24 {
            mnemonic.push(reader.read(11).unwrap() as usize);
        }
        Ok(Mnemonic(mnemonic))
    }

    fn checksum(data: &[u8]) -> Vec<u8> {
        let mut hash = [0u8; 32];
        let mut checksum = Vec::new();
        let mut writer = BitStreamWriter::new(&mut checksum);

        let mut sha2 = Sha256::new();
        sha2.input(data);
        sha2.result(&mut hash);
        let mut check_cursor = Cursor::new(&hash);
        let mut check_reader = BitStreamReader::new(&mut check_cursor);
        for _ in 0..data.len() / 4 {
            writer.write(check_reader.read(1).unwrap(), 1).unwrap();
        }
        writer.flush().unwrap();
        checksum
    }
}
