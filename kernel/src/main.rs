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
mod system_ui;
mod kbmp;

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

    system_ui::draw_init_screen();

    let mut addr = None;

    let mut keyboard = Keyboard::new();
    loop {
        let Some(code) = keyboard::pop() else {
            unsafe { core::arch::asm!("hlt") };
            continue;
        };
        match keyboard.process(code) {
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
            },
            Some(Key::F3) => start(user::hello), // Rust のアプリ
            Some(Key::F4) => start(user::bad),   // カーネルを覗こうとするアプリ
            Some(Key::F5) => start(user::count), // 3, 2, 1 と数えて終わる Rust のアプリ
            _ => {}
        }
    }
}

// fn halt() -> ! {
//     loop {
//         unsafe { core::arch::asm!("hlt") }
//     }
// }


fn start(app: fn() -> u64) {
    syscall::clear();
    let code = app();
    if code == user::KILLED {
        syscall::crushed();
        system_ui::draw_init_screen();
    } else {
        system_ui::draw_init_screen();
        syscall::print_exit(code);
    }
    
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
