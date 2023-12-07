use std::{path::PathBuf, process::Command};
use clap::Parser;
use anyhow::Result; // Using anyhow for error handling

// Enum representing different architectures
#[derive(Debug, Copy, Clone)]
pub enum Architecture {
    BpfEl,
    BpfEb,
}

// Implementing string parsing for the Architecture enum
impl std::str::FromStr for Architecture {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bpfel-unknown-none" => Ok(Architecture::BpfEl),
            "bpfeb-unknown-none" => Ok(Architecture::BpfEb),
            _ => Err("invalid target".to_owned()),
        }
    }
}

// Display trait for Architecture enum
impl std::fmt::Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Architecture::BpfEl => "bpfel-unknown-none",
            Architecture::BpfEb => "bpfeb-unknown-none",
        })
    }
}

// Struct to parse command line options
#[derive(Debug, Parser)]
pub struct Options {
    /// Set the endianness of the BPF target
    #[clap(default_value = "bpfel-unknown-none", long)]
    pub target: Architecture,
    /// Build the release target
    #[clap(long)]
    pub release: bool,
}

// Function to build the eBPF program
pub fn build_ebpf(opts: Options) -> Result<()> {
    let dir = PathBuf::from("xdp-ebpf");
    let target = format!("--target={}", opts.target);
    let mut args = vec!["build", &target, "-Z", "build-std=core"];
    
    if opts.release {
        args.push("--release");
    }

    // Spawn a child process to run the build command
    let status = Command::new("cargo")
        .current_dir(&dir)
        .env_remove("RUSTUP_TOOLCHAIN") // Remove env var to honor rust-toolchain.toml
        .args(&args)
        .status()
        .context("failed to build bpf program")?; // Improved error handling

    if !status.success() {
        anyhow::bail!("Command exited with non-zero status"); // Handle non-zero exit status
    }

    Ok(())
}

