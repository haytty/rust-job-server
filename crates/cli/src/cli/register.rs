use anyhow::Result;
use clap::{Args, ValueHint};
use rust_job_server_config::ConfigPath;
use rust_job_server_di::cli_container::CliContainer;

#[derive(Debug, Args)]
pub struct RegisterArgs {
    #[arg(short, long, value_hint = ValueHint::Unknown, default_value = "config.toml")]
    config: String,
}

impl ConfigPath for RegisterArgs {
    fn config_path(&self) -> &str {
        &self.config
    }
}

pub async fn execute(serve_args: RegisterArgs) -> Result<()> {
    #[cfg(debug_assertions)]
    {
        println!("{:?}", serve_args);
    }

    let config = rust_job_server_config::load_config(serve_args)?;
    let handlers = CliContainer::build_xxx_handler(config).await?;
    // let a = handlers.run().await?;

    Ok(())
}
