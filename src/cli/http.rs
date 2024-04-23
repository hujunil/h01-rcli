use std::path::PathBuf;

use crate::CmdExecutor;

#[derive(Debug, clap::Parser)]
pub enum HttpSubCommand {
    #[command(name = "serve", about = "Serve a directory via HTTP")]
    Serve(HttpServerOpts),
}

#[derive(Debug, clap::Parser)]
pub struct HttpServerOpts {
    #[arg(short, long, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExecutor for HttpServerOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("info: {:?}", self);
        crate::process::http::process_http_server(self.dir, self.port).await
    }
}

impl CmdExecutor for HttpSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpSubCommand::Serve(opts) => opts.execute().await,
        }
    }
}
