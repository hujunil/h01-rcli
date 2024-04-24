use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;

use super::format;
use super::verify_file;
use crate::process::text::process_text_sign;
use crate::CmdExecutor;

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long, value_parser = verify_file, default_value = "blake3", value_parser = format::parse_text_sign_format)]
    pub format: format::TextSignFormat,
}

impl CmdExecutor for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("info: {:?}", self);
        let mut reader = crate::get_reader(&self.input)?;
        let key = crate::get_content(&self.key)?;
        let sig = process_text_sign(&mut reader, &key, self.format)?;
        // Output the signature in URL-safe base64 format
        let encoded = URL_SAFE_NO_PAD.encode(sig);
        println!("{}", encoded);
        Ok(())
    }
}
