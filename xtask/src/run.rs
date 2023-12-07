use std::process::Command;

use anyhow::{Context, Result};
use clap::Parser;

use crate::build_ebpf::{build_ebpf, Architecture, Options as BuildOptions};

// Struct for command line options
#[derive(Debug, Parser)]
pub struct Options {
    /// Set the endianness of the BPF target
    #[clap(default_value = "bpfel-unknown-none", long)]
    pub bpf_target: Architecture,
    /// Build and run the release target
    #[clap(long)]
    pub release: bool,
    /// The command used to wrap your application
    #[clap(short, long, default_value = "sudo -E")]
    pub runner: String,
    /// Arguments to pass to your application
    #[clap(name = "args", last = true)]
    pub run_args: Vec<String>,
}

/// Build the project
fn build(opts: &Options) -> Result<()> {
    let mut args = vec!["build"];
    if opts.release {
        args.push("--release");
    }

    // Handle command execution errors more gracefully
    let status = Command::new("cargo")
        .args(&args)
        .status()
        .context("Failed to build userspace")?;

    if !status.success() {
        anyhow::bail!("Build command exited with non-zero status");
    }
    Ok(())
}

/// Build and run the project
pub fn run(opts: Options) -> Result<()> {
    // Build eBPF program and userspace application
    build_ebpf(BuildOptions {
        target: opts.bpf_target,
        release: opts.release,
    })
    .context("Error while building eBPF program")?;

    build(&opts).context("Error while building userspace application")?;

    // Determine the build profile (release or debug)
    let profile = if opts.release { "release" } else { "debug" };
    let bin_path = format!("target/{profile}/xdp-hello");

    // Prepare arguments for the application
    let mut run_args: Vec<_> = opts.run_args.iter().map(String::as_str).collect();
    let mut args: Vec<_> = opts.runner.trim().split_terminator(' ').collect();
    args.push(&bin_path);
    args.append(&mut run_args);

    // Run the command with enhanced error handling
    let status = Command::new(args.first().ok_or_else(|| anyhow::Error::msg("Runner command not specified"))?)
        .args(args.iter().skip(1))
        .status()
        .context("Failed to run the command")?;

    if !status.success() {
        anyhow::bail!("Failed to run `{}`", args.join(" "));
    }
    Ok(())
}

