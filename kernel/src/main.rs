#![no_std]
#![no_main]

use core::panic::PanicInfo;

unsafe extern "C" {
    static __bss_start: u8;
    static __bss_end: u8;
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let fb = unsafe { core::ptr::read_volatile(0x600 as *const u32) };
    let fb = fb as usize;

    // bss(初期値0の変数) の値を初期化する
    unsafe {
        // ポインタを取る
        let start = &raw const __bss_start as *mut u8;
        let end = &raw const __bss_end as usize; // startは始点のポインタとして使うけど、endはアドレスの値だけ知れればいいからusize

        core::ptr::write_bytes(start, 0, end - start as usize);
    }

    // w=640px * h=16px
    for i in 0..640 * 16 {
        unsafe {
            core::ptr::write_volatile((fb + i) as *mut u8, 1);
        }
    }

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
