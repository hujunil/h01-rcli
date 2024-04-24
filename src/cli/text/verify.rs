use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;

use super::format;
use super::verify_file;
use crate::process::text::process_text_verify;
use crate::CmdExecutor;

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long)]
    pub signature: String,
    #[arg(short, long, value_parser = verify_file, default_value = "blake3", value_parser = format::parse_text_sign_format)]
    pub format: format::TextSignFormat,
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("info: {:?}", self);
        let mut reader = crate::get_reader(&self.input)?;
        let key = crate::get_content(&self.key)?;
        let decoded = URL_SAFE_NO_PAD.decode(self.signature.as_bytes())?;
        let verified = process_text_verify(&mut reader, &key, &decoded, self.format)?;
        if verified {
            println!("✓ Signature verified");
        } else {
            println!("⚠ Signature not verified");
        }
        Ok(())
    }
}
