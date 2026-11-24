use crate::{glyph::glyph, graphics};

pub const SCALE: usize = 3; // 文字のサイズ
pub const SPACE: usize = 1; // 文字間のスペース
pub const LINE: usize = 10 * SCALE; // 文字の高さ

pub fn draw_char(x: usize, y: usize, c: u8, color: u8) {
    let rows = glyph(c);
    for (dy, row) in rows.iter().enumerate() {
        for dx in 0..5 {
            if row & (1 << (4 - dx)) != 0 {
                graphics::rect(x + dx * SCALE, y + dy * SCALE, SCALE, SCALE, color);
            }
        }
    }
}

pub fn draw_text(x: usize, y: usize, text: &[u8], color: u8) {
    for (i, &c) in text.iter().enumerate() {
        draw_char(x + i * (5 * SCALE + SPACE), y, c, color);
    }
}

// 数字を16進数で描く（16桁）
pub fn draw_hex(x: usize, y: usize, value: u64, color: u8) {
    let mut text = [0u8; 16];
    for i in 0..16 {
        let digit = ((value >> ((15 - i) * 4)) & 0xF) as u8; // 上の桁から4bitずつ取り出す
        text[i] = if digit < 10 { b'0' + digit } else { b'A' + (digit - 10) };
    }
    draw_text(x, y, &text, color);
}

// 数字を10進数で描く
pub fn draw_number(x: usize, y: usize, mut value: u64, color: u8) {
    let mut text = [0u8; 20];
    let mut i = text.len();
    loop {
        i -= 1;
        text[i] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    draw_text(x, y, &text[i..], color);
}