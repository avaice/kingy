use crate::{font, graphics};

pub extern "C" fn divide_error() -> ! { show_error(b"DIVIDE ERROR") } // ゼロ除算
pub extern "C" fn invalid_opcode() -> ! { show_error(b"INVALID OPCODE") } // 無効な命令
pub extern "C" fn double_fault() -> ! { show_error(b"DOUBLE FAULT") } // ダブルフォルト（エラー処理自体が失敗）
pub extern "C" fn general_protection() -> ! { show_error(b"GENERAL PROTECTION") } // 一般保護違反
pub extern "C" fn page_fault() -> ! { show_error(b"PAGE FAULT") } // ページフォルト

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