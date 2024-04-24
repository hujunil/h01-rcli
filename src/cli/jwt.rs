use crate::CmdExecutor;

#[derive(Debug, clap::Parser)]
pub enum JwtSubCommand {
    #[command(about = "Sign a jwt with a subject, audience and expiration")]
    Sign(JwtSignOpts),
    #[command(about = "Verify a jwt with a token")]
    Verify(JwtVerifyOpts),
}

impl CmdExecutor for JwtSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            JwtSubCommand::Sign(opts) => opts.execute().await,
            JwtSubCommand::Verify(opts) => opts.execute().await,
        }
    }
}

#[derive(Debug, clap::Parser)]
pub struct JwtSignOpts {
    #[clap(short, long)]
    pub sub: String,
    #[clap(short, long)]
    pub aud: String,
    #[clap(short, long)]
    pub exp: String,
}

impl CmdExecutor for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let jwt = crate::process::jwt::sign_jwt(&self.sub, &self.aud, &self.exp)?;
        println!("{}", jwt);
        Ok(())
    }
}

#[derive(Debug, clap::Parser)]
pub struct JwtVerifyOpts {
    #[clap(short, long)]
    token: String,
}

impl CmdExecutor for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let claims = crate::process::jwt::verify_jwt(&self.token)?;
        println!("验证成功\n{:?}", claims);
        Ok(())
    }
}
