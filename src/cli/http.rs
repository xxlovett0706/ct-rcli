use super::verify_path;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum HttpSubCommand {
    #[command(about = "Serve a file over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(long, value_parser = verify_path, default_value = ".")]
    pub path: PathBuf,
    #[arg(long, default_value = "8080")]
    pub port: u16,
}
