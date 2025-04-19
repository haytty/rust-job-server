use crate::cli::completion::CompletionArgs;
use crate::cli::register::RegisterArgs;
use crate::cli::server::ServerArgs;
use crate::cli::{completion, register, server};
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
    /// Register Job Task.
    Register(RegisterArgs),
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
        SubCommands::Register(register_args) => register::execute(register_args).await,
        SubCommands::Server(server_args) => server::execute(server_args).await,
        SubCommands::Completion(completion_args) => {
            completion::execute(completion_args, Cli::command())
        }
    }
}
