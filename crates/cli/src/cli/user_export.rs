use anyhow::Result;
use clap::{Args, ValueHint};
use rust_job_server_config::ConfigPath;
use rust_job_server_di::cli_container::CliContainer;
use rust_job_server_interface::cli::handler::user_export::user_export_handler::UserExportHandleInput;
use uuid::Uuid;
use rust_job_server_interface::cli::handler::Handler;

#[derive(Debug, Args)]
pub struct UserExportArgs {
    #[arg(short, long, value_hint = ValueHint::Unknown, default_value = "config.toml")]
    config: String,
}

impl ConfigPath for UserExportArgs {
    fn config_path(&self) -> &str {
        &self.config
    }
}

pub async fn execute(user_export_args: UserExportArgs) -> Result<()> {
    #[cfg(debug_assertions)]
    {
        println!("{:?}", user_export_args);
    }

    let config = rust_job_server_config::load_config(user_export_args)?;
    let handlers = CliContainer::build_user_export_handler(config).await?;
    let uuid = Uuid::new_v4();
    let input = UserExportHandleInput::new(uuid.to_string());
    let a = handlers.handle(input).await?;

    Ok(())
}
