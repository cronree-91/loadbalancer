use anyhow::Context;
use aya::{
    include_bytes_aligned,
    programs::{Xdp, XdpFlags},
    Bpf,
};
use aya_log::BpfLogger;
use clap::Parser;
use log::info;
use tokio::signal;

// Defining command line options using clap
#[derive(Debug, Parser)]
struct Opt {
    // Specify the network interface, defaulting to 'ens18'
    #[clap(short, long, default_value = "ens18")]
    iface: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::parse(); // Parsing command line arguments

    env_logger::init(); // Initialize the environment logger

    // This will include your eBPF object file as raw bytes at compile-time and load it at
    // runtime. This approach is recommended for most real-world use cases. If you would
    // like to specify the eBPF program at runtime rather than at compile-time, you can
    // reach for `Bpf::load_file` instead.
    // (4)
    // (5)
    let mut bpf = if cfg!(debug_assertions) {
        Bpf::load(include_bytes_aligned!(
            "../../target/bpfel-unknown-none/debug/xdp-hello"
        ))?
    } else {
        Bpf::load(include_bytes_aligned!(
            "../../target/bpfel-unknown-none/release/xdp-hello"
        ))?
    };

    BpfLogger::init(&mut bpf)?; // Initialize the BPF logger

    // Retrieve the XDP program and attempt conversion into the correct type
    let program: &mut Xdp = bpf.program_mut("xdp_hello")
        .ok_or_else(|| anyhow::Error::msg("Program 'xdp_hello' not found"))?
        .try_into()?;

    program.load()?; // Load the program

    // Attach the program to the specified interface with default XDP flags
    program.attach(&opt.iface, XdpFlags::default())
        .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;

    info!("Waiting for Ctrl-C..."); // Log message indicating readiness
    signal::ctrl_c().await?; // Await a Ctrl-C signal
    info!("Exiting..."); // Log exit message

    Ok(())
}

