#![no_std] // Disabling the standard library.
#![no_main] // No standard main entry point.

use aya_bpf::{bindings::xdp_action, macros::xdp, programs::XdpContext};
use aya_log_ebpf::info;

// The entry point for the XDP program.
#[xdp]
pub fn xdp_hello(ctx: XdpContext) -> u32 {
    // Wrapping the unsafe operation in a safe function.
    // Returns XDP_ABORTED in case of an error.
    match try_xdp_hello(&ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

// Attempts to safely process the packet.
// Unsafe operations are encapsulated and handled safely.
fn try_xdp_hello(ctx: &XdpContext) -> Result<u32, u32> {
    // Logging the receipt of a packet using aya_log_ebpf.
    info!(ctx, "received a packet");

    // Safely passing the packet: returns XDP_PASS.
    // Ensures that all operations within are valid for the given context.
    Ok(xdp_action::XDP_PASS)
}

// Custom panic handler.
// Defines behavior when a panic occurs.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // Using an unsafe hint to terminate the process.
    // This is a last-resort operation when a panic is encountered.
    unsafe { core::hint::unreachable_unchecked() }
}

