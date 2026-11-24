use crate::{font, graphics};

const CLEAR_ON_FREE: bool = true; // メモリを解放するときに0でクリアするかどうか（解放したはずの情報が残ってるのはなんか気持ち悪いと思った）
const START: usize = 0x100000; // 貸し出しに使う範囲の最初（1MB）
const END: usize = 0x180000;   // 貸し出しに使う範囲の最後（1.5MB）
pub const PAGE_SIZE: usize = 4096;
const PAGE_COUNT: usize = (END - START) / PAGE_SIZE; // =128ページ

// 帳簿：1ページにつき1bit（0 = 空き、1 = 使用中）
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

pub fn draw_usage() {
    graphics::rect(150, 0, 70, 40, graphics::color::GRAY);
    font::draw_text(150, 10, b"mem:", graphics::color::WHITE);
    for index in 0..PAGE_COUNT {
        let color = if is_used(index) { graphics::color::GREEN } else { graphics::color::WHITE };
        graphics::rect(220 + index, 5, 1, 30, color);
    }
}