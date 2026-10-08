#![no_std]
#![no_main]

mod font;
mod glyph;
mod graphics;

use core::panic::PanicInfo;

unsafe extern "C" {
    static __bss_start: u8;
    static __bss_end: u8;
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // bss(初期値0の変数) の値を初期化する
    unsafe {
        // ポインタを取る
        let start = &raw const __bss_start as *mut u8;
        let end = &raw const __bss_end as usize; // startは始点のポインタとして使うけど、endはアドレスの値だけ知れればいいからusize

        core::ptr::write_bytes(start, 0, end - start as usize);
    }

    graphics::clear(3);
    graphics::rect(160, 120, 320, 240, 1);

    font::draw_text(170, 130, b"Hello, Kingy!", 2);
    font::draw_text(170, 130 + font::LINE * 1, b"0123456789~!?", 15);

    halt()
}

fn halt() -> ! {
    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
