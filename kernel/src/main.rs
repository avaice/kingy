#![no_std]
#![no_main]

mod font;
mod glyph;
mod graphics;
mod io;
mod keyboard;
mod keymap;

use core::panic::PanicInfo;
use keyboard::{Key, Keyboard};

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

    let mut keyboard = Keyboard::new();
    let mut x = 170;
    let mut y = 200;
    loop {
        match keyboard.poll() {
            Some(Key::Character(c)) => {
                if x + font::LINE / 2 > 480 {
                    x = 170;
                    y += font::LINE;
                }
                font::draw_char(x, y, c, 15);
                x += font::LINE / 2;
            }
            Some(Key::Enter) => {
                x = 170;
                y += font::LINE;
            }
            Some(Key::Backspace) => {
                if x <= 170 {
                    y = (y - font::LINE).max(200);
                    if y != 200 {
                        x = 480 - font::LINE;
                    }
                } else {
                    x -= font::LINE / 2;
                }

                graphics::rect(x, y, font::LINE, font::LINE, 1);
            }
            _ => {}
        }
    }
}

// fn halt() -> ! {
//     loop {
//         unsafe { core::arch::asm!("hlt") }
//     }
// }

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
