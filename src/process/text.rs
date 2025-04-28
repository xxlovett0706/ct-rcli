use std::{fs, io::Read, path::Path};

use anyhow::Ok;
use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use crate::{get_reader, TextSignFormat};

use super::gen_pass;

pub trait TextSign {
    // Sign the data from the reader
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}

pub trait TextVerify {
    // Verify the data from the reader with the signature
    fn verify(&self, reader: impl Read, sign: &[u8]) -> anyhow::Result<bool>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self, anyhow::Error>
    where
        Self: Sized;
}

pub trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>, anyhow::Error>;
}

pub struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(mut key: Vec<u8>) -> Result<Self, anyhow::Error> {
        if key.last() == Some(&b'\n') {
            key.pop();
        }
        let key = key.try_into().unwrap();
        Ok(Self::new(key))
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self, anyhow::Error> {
        let key = fs::read(path)?;
        Self::try_new(key)
    }
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sign: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(hash == sign)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>, anyhow::Error> {
        let key = gen_pass::process_genpass(32, true, true, true, true)?;
        let key = key.as_bytes().to_vec();
        Ok(vec![key])
    }
}

pub struct Ed25519Signer {
    key: SigningKey,
}

impl Ed25519Signer {
    fn new(key: SigningKey) -> Self {
        Self { key }
    }

    fn try_new(mut key: Vec<u8>) -> Result<Self, anyhow::Error> {
        if key.last() == Some(&b'\n') {
            key.pop();
        }
        let key = SigningKey::from_bytes(&key.try_into().unwrap());
        Ok(Self::new(key))
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self, anyhow::Error> {
        let key = fs::read(path)?;
        Self::try_new(key)
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(self.key.sign(&buf).to_bytes().to_vec())
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>, anyhow::Error> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        Ok(vec![
            signing_key.to_bytes().to_vec(),
            verifying_key.to_bytes().to_vec(),
        ])
    }
}
pub struct Ed25519Verifier {
    key: VerifyingKey,
}

impl Ed25519Verifier {
    fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    fn try_new(mut key: Vec<u8>) -> Result<Self, anyhow::Error> {
        if key.last() == Some(&b'\n') {
            key.pop();
        }
        let key = VerifyingKey::from_bytes(&key.try_into().unwrap())?;
        Ok(Self::new(key))
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self, anyhow::Error> {
        let key = fs::read(path)?;
        Self::try_new(key)
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sign: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = Signature::from_bytes(sign.try_into()?);
        Ok(self.key.verify(&buf, &signature).is_ok())
    }
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    // Remove trailing newline if present
    if buf.last() == Some(&b'\n') {
        buf.pop();
    }

    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut &buf[..])?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut &buf[..])?
        }
    };

    let signed = BASE64_URL_SAFE_NO_PAD.encode(signed);
    Ok(signed)
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    format: TextSignFormat,
    sig: &str,
) -> anyhow::Result<bool> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    // Remove trailing newline if present
    if buf.last() == Some(&b'\n') {
        buf.pop();
    }

    let sig = BASE64_URL_SAFE_NO_PAD.decode(sig)?;

    let result = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut &buf[..], &sig)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut &buf[..], &sig)?
        }
    };
    Ok(result)
}

pub fn process_text_generate(format: TextSignFormat) -> anyhow::Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_sign_verify() -> anyhow::Result<()> {
        let blake3 = Blake3::load("fixtures/blake3.txt")?;

        let data = b"hello world!";

        let sig = blake3.sign(&mut &data[..])?;

        let result = blake3.verify(&mut &data[..], &sig)?;

        assert!(result);

        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> anyhow::Result<()> {
        let sk = Ed25519Signer::load("fixtures/ed25519.sk")?;
        let pk = Ed25519Verifier::load("fixtures/ed25519.pk")?;

        let data = b"hello world!";
        let sig = sk.sign(&mut &data[..])?;
        assert!(pk.verify(&data[..], &sig)?);

        Ok(())
    }
}
