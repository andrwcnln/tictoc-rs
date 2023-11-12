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

fn main() {
    let args = Cli::parse();
    let time = Instant::now();
    Command::new(args.command).args(args.args).status().unwrap();
    //let time = Instant::now();
    //let _result = child.wait().unwrap();
    let elap = time.elapsed().as_secs_f64();
    println!("time elapsed {}", elap)
}
