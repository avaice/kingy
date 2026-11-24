use crate::graphics;

const CLEAR_ON_FREE: bool = true; // メモリを解放するときに0でクリアするかどうか（解放したはずの情報が残ってるのはなんか気持ち悪いと思った）
const START: usize = 0x100000; // 貸し出しに使う範囲の最初（1MB）
const END: usize = 0x200000;   // 貸し出しに使う範囲の最後（2MB）
pub const PAGE_SIZE: usize = 4096;
const PAGE_COUNT: usize = (END - START) / PAGE_SIZE; // =256ページ

// 帳簿：1ページにつき1bit（0 = 空き、1 = 使用中）
// 256ページ ÷ 8bit = 32バイト
static mut USED: [u8; PAGE_COUNT / 8] = [0; PAGE_COUNT / 8];

fn is_used(index: usize) -> bool {
    unsafe { USED[index / 8] & (1 << (index % 8)) != 0 }
}

fn set_used(index: usize, used: bool) {
    unsafe {
        if used {
            USED[index / 8] |= 1 << (index % 8); // そのbitだけ1にする
        } else {
            USED[index / 8] &= !(1 << (index % 8)); // そのbitだけ0にする
        }
    }
}

fn clear_memory(addr: usize) {
    unsafe { core::ptr::write_bytes(addr as *mut u8, 0, PAGE_SIZE) };
}

pub fn alloc() -> Option<usize> {
    for index in 0..PAGE_COUNT {
        if !is_used(index) {
            set_used(index, true);
            let addr = START + index * PAGE_SIZE;
            clear_memory(addr);
            return Some(addr);
        }
    }
    None // 全部使用中
}

pub fn free(addr: usize) -> Option<usize> {
    if addr < START || addr >= END || addr % PAGE_SIZE != 0 {
        return None; // 範囲外やページサイズがおかしいアドレスは無視
    }
    if CLEAR_ON_FREE {
        clear_memory(addr);
    }
    set_used((addr - START) / PAGE_SIZE, false);
    Some(addr)
}

pub fn draw_usage(x: usize, y: usize) {
    for index in 0..PAGE_COUNT {
        let color = if is_used(index) { graphics::color::YELLOW } else { graphics::color::GRAY };
        let cell_x = x + (index % 16) * 9;
        let cell_y = y + (index / 16) * 9;
        graphics::rect(cell_x, cell_y, 7, 7, color);
    }
}