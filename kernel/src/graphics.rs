pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 480;

const FB_ADDR: usize = 0x600;

// VGAの色定義
#[allow(dead_code)]
pub mod color {
    pub const BLACK: u8 = 0;
    pub const BLUE: u8 = 1;
    pub const GREEN: u8 = 2;
    pub const CYAN: u8 = 3;
    pub const RED: u8 = 4;
    pub const MAGENTA: u8 = 5;
    pub const BROWN: u8 = 6;
    pub const LIGHT_GRAY: u8 = 7;
    pub const GRAY: u8 = 8;
    pub const LIGHT_BLUE: u8 = 9;
    pub const LIGHT_GREEN: u8 = 10;
    pub const LIGHT_CYAN: u8 = 11;
    pub const LIGHT_RED: u8 = 12;
    pub const LIGHT_MAGENTA: u8 = 13;
    pub const YELLOW: u8 = 14;
    pub const WHITE: u8 = 15;
}

// VRAMのアドレスを返す
fn vram() -> usize {
    unsafe { core::ptr::read_volatile(FB_ADDR as *const u32) as usize }
}

// 1px描画する
pub fn pixel(x: usize, y: usize, color: u8) {
    if x >= WIDTH || y >= HEIGHT {
        return;
    }
    let offset = y * WIDTH + x;
    unsafe {
        core::ptr::write_volatile((vram() + offset) as *mut u8, color);
    }
}

// x, yの位置からw, hの範囲を塗りつぶす
pub fn rect(x: usize, y: usize, w: usize, h: usize, color: u8) {
    let right = x.saturating_add(w).min(WIDTH); // saturating_addはその型が表現できる最大値を超えた場合に最大値を返す
    let bottom = y.saturating_add(h).min(HEIGHT);
    for py in y..bottom {
        for px in x..right {
            pixel(px, py, color);
        }
    }
}

// 画面全体を塗りつぶす
pub fn clear(color: u8) {
    rect(0, 0, WIDTH, HEIGHT, color);
}
