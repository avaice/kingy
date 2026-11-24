use crate::{font, graphics};

pub extern "C" fn double_fault() -> ! { show_error(b"DOUBLE FAULT") } // ダブルフォルト（エラー処理自体が失敗）

// エラーコードがないエラー
#[unsafe(naked)]
pub extern "C" fn divide_error() {
    core::arch::naked_asm!("mov rdi, [rsp + 8]", "mov rsi, 0", "and rsp, -16", "call {f}", f = sym fault)
}
#[unsafe(naked)]
pub extern "C" fn invalid_opcode() {
    core::arch::naked_asm!("mov rdi, [rsp + 8]", "mov rsi, 6", "and rsp, -16", "call {f}", f = sym fault)
}

// エラーコードがあるエラー
#[unsafe(naked)]
pub extern "C" fn general_protection() {
    core::arch::naked_asm!("mov rdi, [rsp + 16]", "mov rsi, 13", "and rsp, -16", "call {f}", f = sym fault)
}
#[unsafe(naked)]
pub extern "C" fn page_fault() {
    core::arch::naked_asm!("mov rdi, [rsp + 16]", "mov rsi, 14", "and rsp, -16", "call {f}", f = sym fault)
}

extern "C" fn fault(cs: u64, vector: u64) -> ! {
    // CS の下2bitが 3 なら、Ring3 のアプリが起こしたエラー
    if cs & 3 == 3 {
        crate::user::exit(crate::user::KILLED);
    }
    // カーネルが起こしたエラー
    match vector {
        0 => show_error(b"DIVIDE ERROR"),
        6 => show_error(b"INVALID OPCODE"),
        13 => show_error(b"GENERAL PROTECTION"),
        _ => show_error(b"PAGE FAULT"),
    }
}

fn show_error(message: &[u8]) -> ! {

    graphics::clear(0);
    
    graphics::rect(160, 160, 320, font::LINE + 20, 4);
    font::draw_text(180, 170, b"Kingy has halted.", 15);

    let width = message.len() * (5 * font::SCALE + font::SPACE);
    font::draw_text((graphics::WIDTH - width) / 2, 240, message, 15);
    
    loop {
        unsafe { core::arch::asm!("cli", "hlt") }
    }
}