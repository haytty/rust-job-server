use anyhow::Result;
use clap::{Args, ValueHint};
use rust_job_server_config::ConfigPath;
use rust_job_server_di::DI;

#[derive(Debug, Args)]
pub struct ServerArgs {
    #[arg(short, long, value_hint = ValueHint::Unknown, default_value = "config.toml")]
    config: String,
}

impl ConfigPath for ServerArgs {
    fn config_path(&self) -> &str {
        &self.config
    }
}

pub async fn execute(server_args: ServerArgs) -> Result<()> {
    #[cfg(debug_assertions)]
    {
        println!("{:?}", server_args);
    }

    let config = rust_job_server_config::load_config(server_args)?;
    let service = DI::server_service_build(config).await;
    service.run().await?;

    Ok(())
}
