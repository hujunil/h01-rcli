use std::path::PathBuf;

use super::format;
use super::verify_path;
use super::CmdExecutor;
use crate::process::text::process_text_key_generate;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct KeyGenerateOpts {
    /// Key format
    #[arg(short, long, default_value = "blake3", value_parser = format::parse_text_sign_format)]
    pub format: format::TextSignFormat,
    /// Output directory
    #[arg(short, long, value_parser = verify_path)]
    pub directory: PathBuf,
}

impl CmdExecutor for KeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("info: {:?}", self);
        let keys = process_text_key_generate(self.format)?;
        // 将key写入到文件中
        for (filename, content) in keys {
            let path = self.directory.join(filename);
            std::fs::write(&path, &content)?;
        }
        Ok(())
    }
}
