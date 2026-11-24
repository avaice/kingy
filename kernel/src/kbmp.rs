// kingy bitmap

use crate::graphics;

pub struct Kbmp<'a> {
    // pub color_mode: u8,
    pub width: u16,
    pub height: u16,
    pub pixels: &'a[u8],
}

pub fn read_kbmp(bytes: &[u8]) -> Kbmp<'_> {

    let kbmp = Kbmp {
        // color_mode: bytes[0] & 0b11,
        width: bytes[1] as u16,
        height: bytes[2] as u16,
        pixels: &bytes[8..],
    };

    kbmp
}

pub fn draw_kbmp(kbmp: Kbmp, x: usize, y: usize, scale: usize, transparent: bool) {
    for py in 0..kbmp.height {
        for px in 0..kbmp.width {

            let sx = px * scale as u16;
            let sy = py * scale as u16;

            let stride = (kbmp.width as usize + 1) / 2;
            let byte = kbmp.pixels[py as usize * stride + px as usize / 2];
            let pixel = if px % 2 == 0 { byte >> 4 } else { byte & 0xF }; // 上位4bit が左の画素
            if transparent && pixel == graphics::color::WHITE {
                continue;
            }
            graphics::rect(x + sx as usize, y + sy as usize, scale as usize, scale as usize, pixel);
        }
    }
}