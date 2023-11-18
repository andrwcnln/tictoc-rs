use anyhow::{Context, Result};
use clap::Parser;
use std::process::Command;
use std::time::Instant;

/// Time the execution of any command.
#[derive(Parser)]
struct Cli {
    /// The command
    command: String,
    /// Any arguments to pass
    #[clap(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

fn main() -> Result<()> {
    let input = Cli::parse();

    let time = Instant::now();

    Command::new(&input.command)
        .args(input.args)
        .status()
        .with_context(|| format!("Could not execute command `{}`", input.command))?;

    let elapsed_time = time.elapsed().as_secs_f64();
    println!("time elapsed {}", elapsed_time);
    Ok(())
}
