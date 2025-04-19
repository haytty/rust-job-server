use anyhow::Result;
use clap::{Args, Command};
use clap_complete::{generate, Generator, Shell};
use std::io;

/// Structure to hold arguments for generating shell completion scripts.
#[derive(Debug, Args)]
pub struct CompletionArgs {
    /// The shell to generate the completion for.
    #[arg(long, value_enum, value_name = "SHELL")]
    shell: Shell,
}

pub fn execute(completion_args: CompletionArgs, mut cmd: Command) -> Result<()> {
    print_completions(completion_args.shell, &mut cmd)
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) -> Result<()> {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
    Ok(())
}
