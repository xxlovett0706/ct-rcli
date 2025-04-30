use super::{verify_file, verify_path};
use clap::{Parser, Subcommand};
use std::{fmt, path::PathBuf, str::FromStr};

#[derive(Debug, Subcommand)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a private key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a message with a public key")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a new key")]
    Generate(TextGenerateOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, value_parser = text_sign_format_parser, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, value_parser = text_sign_format_parser, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(short, long)]
    pub signature: String,
}

#[derive(Debug, Parser)]
pub struct TextGenerateOpts {
    #[arg(long, value_parser = text_sign_format_parser, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

fn text_sign_format_parser(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "blake3" => TextSignFormat::Blake3,
            "ed25519" => TextSignFormat::Ed25519,
            _ => return Err(anyhow::anyhow!("Invalid format: {}", s)),
        })
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
