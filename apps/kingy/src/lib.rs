#![no_std]

use core::panic::PanicInfo;

const WRITE: u64 = 1;
const PRINT: u64 = 2;
const RECT: u64 = 3;
const CLEAR: u64 = 4;
const DRAW_KBMP: u64 = 5;
const GET_KEY_CODE: u64 = 6;
const EXIT: u64 = u64::MAX - 1;
pub const ERROR: u64 = u64::MAX;

// rax=番号, rdi, rsi, rdx, r10, r8, r9 = 引数1〜6 でカーネルに渡す
fn syscall(number: u64, args: [u64; 6]) -> u64 {
    let result: u64;
    unsafe {
        core::arch::asm!(
            "int 0x80",
            inlateout("rax") number => result,
            in("rdi") args[0],
            in("rsi") args[1],
            in("rdx") args[2],
            in("r10") args[3],
            in("r8") args[4],
            in("r9") args[5],
        );
    }
    result
}

pub fn write(text: &[u8]) -> u64 {
    syscall(WRITE, [text.as_ptr() as u64, text.len() as u64, 0, 0, 0, 0])
}

pub fn print(text: &[u8], x: u64, y: u64, color: u8) -> u64 {
    syscall(PRINT, [text.as_ptr() as u64, text.len() as u64, x, y, color as u64, 0])
}

pub fn rect(x: u64, y: u64, width: u64, height: u64, color: u8) -> u64 {
    syscall(RECT, [x, y, width, height, color as u64, 0])
}

pub fn clear() -> u64 {
    syscall(CLEAR, [0; 6])
}

pub fn draw_kbmp(data: &[u8], x: u64, y: u64, scale: u64, transparent: bool) -> u64 {
    syscall(DRAW_KBMP, [data.as_ptr() as u64, data.len() as u64, x, y, scale, transparent as u64])
}

// 押されたキーのスキャンコード。無ければ None
pub fn get_key_code() -> Option<u8> {
    match syscall(GET_KEY_CODE, [0; 6]) {
        ERROR => None,
        code => Some(code as u8),
    }
}

pub fn exit(code: u64) -> ! {
    unsafe {
        core::arch::asm!(
            "int 0x80",
            in("rax") EXIT,
            in("rdi") code,
            options(noreturn),
        );
    }
}

pub fn pause() -> ! {
    rect(10, 400, 300, 30, 15);
    print(b"[ESC] to continue..", 10, 405, 0);
    loop{
        if let Some(code) = get_key_code() {
            if code == 0x01 {
                break;
            }
        }
    }
    exit(0);
}

unsafe extern "Rust" {
    fn main();
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    unsafe { main() };
    exit(0); // main が最後まで来たら、終了コード 0 で終わる
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    exit(101); // パニックしたら、終了コード 101 で終わる
}
