use base64::engine::general_purpose::STANDARD;
use base64::engine::Engine;
use clap::Parser;

use super::verify_file;
use crate::CmdExecutor;

#[derive(Debug, Parser)]
pub struct DecryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
}

impl CmdExecutor for DecryptOpts {
    async fn execute(self) -> Result<(), anyhow::Error> {
        let mut reader = crate::get_reader(&self.input)?;
        let key = crate::process::chacha20::gen_chacha20_key(&self.key);
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let decoded = STANDARD.decode(&buf)?;
        let ret = crate::process::chacha20::process_chacha20_decrypt(&key, &decoded);
        println!("{}", String::from_utf8_lossy(&ret));
        Ok(())
    }
}
