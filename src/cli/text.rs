use clap::Parser;

use super::verify_file;
use super::verify_path;
use crate::CmdExecutor;

mod decrypt;
mod encrypt;
mod format;
mod generate;
mod sign;
mod verify;

pub use format::TextSignFormat;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a text with a private/session key and return a signature")]
    Sign(sign::TextSignOpts),
    #[command(about = "Verify a signature with a public/session key")]
    Verify(verify::TextVerifyOpts),
    #[command(
        name = "gen",
        about = "Generate a random blake3 key or ed25519 key pair"
    )]
    KeyGenerate(generate::KeyGenerateOpts),
    // 以下是新增的加密解密命令
    #[command(name = "encrypt", about = "Encrypt a text with a key")]
    Encrypt(encrypt::EncryptOpts),
    #[command(name = "decrypt", about = "Decrypt a text with a key")]
    Decrypt(decrypt::DecryptOpts),
}

impl CmdExecutor for TextSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            TextSubCommand::Sign(opts) => opts.execute().await,
            TextSubCommand::Verify(opts) => opts.execute().await,
            TextSubCommand::KeyGenerate(opts) => opts.execute().await,
            TextSubCommand::Encrypt(opts) => opts.execute().await,
            TextSubCommand::Decrypt(opts) => opts.execute().await,
        }
    }
}
