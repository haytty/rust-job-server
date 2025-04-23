use anyhow::Result;
use clap::{Args, ValueHint};
use rust_job_server_config::ConfigPath;
use rust_job_server_di::cli_container::CliContainer;
use rust_job_server_interface::cli::handler::aggregation::aggregation_handler::AggregationHandleInput;
use rust_job_server_interface::cli::handler::Handler;
use uuid::Uuid;

#[derive(Debug, Args)]
pub struct AggregationArgs {
    #[arg(short, long, value_hint = ValueHint::Unknown, default_value = "config.toml")]
    config: String,
}

impl ConfigPath for AggregationArgs {
    fn config_path(&self) -> &str {
        &self.config
    }
}

pub async fn execute(aggregation_args: AggregationArgs) -> Result<()> {
    #[cfg(debug_assertions)]
    {
        println!("{:?}", aggregation_args);
    }

    let config = rust_job_server_config::load_config(aggregation_args)?;
    let handlers = CliContainer::build_aggregation_handler(config).await?;
    let uuid = Uuid::new_v4();
    let input = AggregationHandleInput::new(uuid.to_string());

    let a = handlers.handle(input).await?;

    Ok(())
}
