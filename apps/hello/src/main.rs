#![no_std]
#![no_main]

use kingy::write;

#[unsafe(no_mangle)]
fn main() {
    write(b"Hello from Rust app!");
}
