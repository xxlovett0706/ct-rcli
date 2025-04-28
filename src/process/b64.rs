use crate::{get_reader, Base64Format};
use anyhow::Ok;
use base64::prelude::*;
use std::io::Read;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(buf),
        Base64Format::UrlSafe => BASE64_URL_SAFE.encode(buf),
    };
    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<Vec<u8>> {
    let mut reader = get_reader(input)?;

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    // avoid accidental newlines

    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(buf)?,
        Base64Format::UrlSafe => BASE64_URL_SAFE.decode(buf)?,
    };

    Ok(decoded)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        process_encode(input, format).unwrap();
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::Standard;
        process_encode(input, format).unwrap();
    }
}
