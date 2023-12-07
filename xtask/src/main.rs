mod build_ebpf;
mod run;

use std::process::exit;
use clap::Parser;

// Command line options struct
#[derive(Debug, Parser)]
pub struct Options {
    // Using clap's subcommand functionality to handle different commands
    #[clap(subcommand)]
    command: Command,
}

// Enum for different commands
#[derive(Debug, Parser)]
enum Command {
    BuildEbpf(build_ebpf::Options),
    Run(run::Options),
}

fn main() {
    let opts = Options::parse(); // Parsing command line arguments

    // Match the provided command and execute the corresponding function
    let ret = match opts.command {
        Command::BuildEbpf(opts) => build_ebpf::build_ebpf(opts),
        Command::Run(opts) => run::run(opts),
    };

    // Error handling: prints error and exits if an error occurs
    if let Err(e) = ret {
        eprintln!("Error: {e:#}");
        exit(1);
    }
}

