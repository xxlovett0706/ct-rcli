mod base64;
mod csv;
mod genpass;

use std::path::Path;

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
};
use self::{csv::CsvOpts, genpass::GenpassOpts};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    Genpass(GenpassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist".into())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".to_string()));
        assert_eq!(verify_input_file("*"), Err("File does not exist".into()));
        assert_eq!(
            verify_input_file("Cargo.toml"),
            Ok("Cargo.toml".to_string())
        );
        assert_eq!(
            verify_input_file("nonexistent.txt"),
            Err("File does not exist".into())
        );
    }
}
