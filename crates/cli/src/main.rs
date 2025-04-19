use crate::cli::root::execute;
use anyhow::Result;

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    execute().await?;
    Ok(())
}
