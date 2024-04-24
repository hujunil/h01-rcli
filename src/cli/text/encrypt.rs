use base64::engine::general_purpose::STANDARD;
use base64::engine::Engine;

use clap::Parser;

use super::verify_file;
use crate::CmdExecutor;

#[derive(Debug, Parser)]
pub struct EncryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
}

impl CmdExecutor for EncryptOpts {
    async fn execute(self) -> Result<(), anyhow::Error> {
        let mut reader = crate::get_reader(&self.input)?;
        let key = crate::process::chacha20::gen_chacha20_key(&self.key);
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let ret = crate::process::chacha20::process_chacha20_encrypt(&key, &buf);
        let encoded = STANDARD.encode(ret);
        println!("{}", encoded);
        Ok(())
    }
}
