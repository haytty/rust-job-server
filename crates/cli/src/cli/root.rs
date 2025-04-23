use crate::cli::aggregation::AggregationArgs;
use crate::cli::completion::CompletionArgs;
use crate::cli::server::ServerArgs;
use crate::cli::user_export::UserExportArgs;
use crate::cli::{aggregation, completion, server, user_export};
use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use tracing::info;

/// Main CLI structure for the bloom application.
#[derive(Parser, Debug)]
#[command(version, about, author)]
struct Cli {
    /// completion
    #[command(subcommand)]
    command: SubCommands,
}

/// Enum representing the available subcommands for the CLI.
#[derive(Debug, Subcommand)]
pub enum SubCommands {
    /// UserExport Job Task.
    UserExport(UserExportArgs),
    /// UserExport Job Task.
    Aggregation(AggregationArgs),
    /// Start Job Application.
    Server(ServerArgs),
    /// Generate completion scripts for the specified shell.
    Completion(CompletionArgs),
}

pub fn initialize_logger() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Logger loaded");
    Ok(())
}

/// Enum representing the available subcommands for the CLI.
pub async fn execute() -> Result<()> {
    initialize_logger();
    let cli = Cli::parse();

    match cli.command {
        SubCommands::UserExport(user_export_args) => user_export::execute(user_export_args).await,
        SubCommands::Aggregation(aggregation_args) => aggregation::execute(aggregation_args).await,
        SubCommands::Server(server_args) => server::execute(server_args).await,
        SubCommands::Completion(completion_args) => {
            completion::execute(completion_args, Cli::command())
        }
    }
}
