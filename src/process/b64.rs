use crate::Base64Format;
use anyhow::Ok;
use base64::prelude::*;
use std::{fs::File, io::Read};

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(buf),
        Base64Format::UrlSafe => BASE64_URL_SAFE.encode(buf),
    };
    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    // avoid accidental newlines

    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(buf)?,
        Base64Format::UrlSafe => BASE64_URL_SAFE.decode(buf)?,
    };

    // TODO: decoded data might be String(but for the example, we assume it is)
    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);
    Ok(())
}

fn get_reader(input: &str) -> Result<Box<dyn Read>, anyhow::Error> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
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
