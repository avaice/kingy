use crate::{io::outb, memory, pic};

const PIT_FREQ: u32 = 1_193_182; // PITは1秒に 1,193,182 回振動する
const HZ: u32 = 1000; // 1秒に1000回タイマー発生
static mut TICKS: u64 = 0; 

pub fn init() {
    let divisor = PIT_FREQ / HZ;
    unsafe {
        outb(0x43, 0x36); // 一定間隔で鳴らし続ける
        outb(0x40, divisor as u8);        // 下位8bit
        outb(0x40, (divisor >> 8) as u8); // 上位8bit
    }
}

#[unsafe(naked)]
pub extern "C" fn timer_entry() {
    core::arch::naked_asm!(
        // 作業中のレジスタを退避
        "push rax", "push rcx", "push rdx", "push rsi", "push rdi",
        "push r8", "push r9", "push r10", "push r11",
        "cld",
        "call {handler}", // Rustの関数を呼ぶ
        // 退避したレジスタを元に戻す
        "pop r11", "pop r10", "pop r9", "pop r8",
        "pop rdi", "pop rsi", "pop rdx", "pop rcx", "pop rax",
        "iretq", // 割り込まれた場所に戻る
        handler = sym timer_handler,
    )
}

extern "C" fn timer_handler() {
    let ticks = unsafe {
        TICKS += 1;
        TICKS
    };

    if ticks % 200 == 0 {
        memory::draw_usage();
    }
    pic::end_of_interrupt();
}
