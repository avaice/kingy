#![no_std]
#![no_main]

use kingy::{write, pause};

#[unsafe(no_mangle)]
fn main() {
    write(b"Hello from Rust app!");
    pause();
}
