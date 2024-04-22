mod cli;
mod process;
mod utils;

pub use cli::Opts;
pub use utils::*;

/// Trait for command execution
#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
