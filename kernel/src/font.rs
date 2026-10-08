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
