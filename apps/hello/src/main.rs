#![no_std]
#![no_main]

mod kingy;

use core::panic::PanicInfo;

// アプリの入口。カーネルはここ（USER_BASE）から実行を始める
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    main();
    loop {} // 終わったら待つ（終了のしくみはまだない）
}

fn main() {
    kingy::write(b"Hello from Rust app!");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
