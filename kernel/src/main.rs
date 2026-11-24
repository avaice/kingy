#![no_std]
#![no_main]

mod gdt;
mod font;
mod glyph;
mod graphics;
mod io;
mod keyboard;
mod keymap;
mod exception;
mod idt;
mod syscall;
mod pic;
mod timer;
mod memory;
mod paging;
mod user;

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

    gdt::init();
    paging::init();
    idt::init();
    pic::init();
    timer::init();
    unsafe { core::arch::asm!("sti") }; // 割り込みを受け付ける

    graphics::clear(3);
    graphics::rect(160, 120, 320, 240, 1);

    font::draw_text(170, 130, b"Hello, Kingy!", 2);
    font::draw_text(170, 130 + font::LINE * 1, b"0123456789~!?", 15);

    let mut addr = None;
    memory::draw_usage(490, 120);

    let mut keyboard = Keyboard::new();
    let mut x = 170;
    let mut y = 200;
    loop {
        let Some(code) = keyboard::pop() else {
            unsafe { core::arch::asm!("hlt") };
            continue;
        };
        match keyboard.process(code) {
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
            Some(Key::F1) => {
                graphics::rect(10, 370, 480, font::LINE, graphics::color::CYAN);
                graphics::rect(10, 370 + font::LINE, 480, font::LINE, graphics::color::CYAN);
                let old_addr = addr;
                addr = memory::alloc();
                if addr.is_none() {
                    font::draw_text(10, 370 + font::LINE, b"Memory allocation failed", graphics::color::RED);
                    addr = old_addr;
                    continue;
                }
                font::draw_hex(10, 370, addr.unwrap() as u64, 15);
                memory::draw_usage(490, 120);
            }
            Some(Key::F2) => {
                if addr.is_none() {
                    continue;
                }
                graphics::rect(10, 370, 480, font::LINE, graphics::color::CYAN);
                graphics::rect(10, 370 + font::LINE, 480, font::LINE, graphics::color::CYAN);
                addr = memory::free(addr.unwrap());
                if addr.is_none() {
                    font::draw_text(10, 370 + font::LINE, b"Memory free failed", graphics::color::RED);
                    continue;
                }
                addr = Some(addr.unwrap() - memory::PAGE_SIZE);
                memory::draw_usage(490, 120);
            },
            Some(Key::F3) => user::hello(), // 正しいアプリ
            Some(Key::F4) => user::bad(),   // カーネルを覗こうとするアプリ
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
