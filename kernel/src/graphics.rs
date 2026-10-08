pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 480;

const FB_ADDR: usize = 0x600;

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
