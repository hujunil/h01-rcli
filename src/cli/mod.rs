pub mod base64;
pub mod csv;
pub mod genpass;
pub mod text;

use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::{Path, PathBuf};

use crate::CmdExecutor;

use self::{base64::Base64SubCommand, csv::CsvOpts, genpass::GenPassOpts, text::TextSubCommand};

#[derive(Debug, Parser)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to JSON or YAML")]
    Csv(CsvOpts),
    #[command(subcommand, name = "base64", about = "Encode or decode base64 string")]
    Base64(Base64SubCommand),
    #[command(name = "genpass", about = "Generate random password")]
    GenPass(GenPassOpts),
    #[command(
        subcommand,
        name = "text",
        about = "Sign, verify text or generate key pair"
    )]
    Text(TextSubCommand),
}

impl CmdExecutor for SubCommand {
    async fn execute(self) -> Result<()> {
        match self {
            SubCommand::Csv(opts) => opts.execute().await,
            SubCommand::Base64(opts) => opts.execute().await,
            SubCommand::GenPass(opts) => opts.execute().await,
            SubCommand::Text(opts) => opts.execute().await,
        }
    }
}

fn verify_file(filename: &str) -> Result<String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(anyhow!("File not found: {}", filename))
    }
}

fn verify_path(path: &str) -> Result<PathBuf> {
    let path = Path::new(path);
    if path.exists() && path.is_dir() {
        Ok(path.to_path_buf())
    } else {
        Err(anyhow!("Path not found: {}", path.display()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_file() {
        assert!(verify_file("Cargo.toml").is_ok());
        assert!(verify_file("nonexistent").is_err());
        assert_eq!(verify_file("-").unwrap(), "-");
    }

    #[test]
    fn test_verify_path() {
        assert!(verify_path(".").is_ok());
        assert!(verify_path("nonexistent").is_err());
    }
}
