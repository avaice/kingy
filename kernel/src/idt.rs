use crate::{font, graphics};

#[derive(Clone, Copy)]
#[repr(C, packed)] // Rustは自動でメモリを並び替えたり隙間を空けたりしてくれるので、それを防ぐためにpackedを使う
struct Entry {
    offset_low: u16,  // 関数アドレスの下位16bit
    selector: u16,    // どのモードで実行するか
    ist: u8,          // スタックが壊れている時にトリプルフォルトにならないように別のスタックを使うことができる
    attributes: u8,   // 「この行は有効」などの設定が8bitぶんある
    offset_mid: u16,  // 関数アドレスの真ん中16bit
    offset_high: u32, // 関数アドレスの上位32bit
    reserved: u32,    // 使わない（0）
}

#[repr(C, packed)]
struct IdtPointer {
    limit: u16, 
    base: u64, 
}

static mut IDT: [Entry; 256] = [Entry::empty(); 256];

impl Entry {
    const fn empty() -> Self {
        Entry { offset_low: 0, selector: 0, ist: 0, attributes: 0,
                offset_mid: 0, offset_high: 0, reserved: 0 }
    }

    fn new(handler: *const ()) -> Self {
        let addr = handler as usize; // 関数が置かれているメモリの番地
        Entry {
            offset_low: addr as u16,
            selector: 0x18, // stage2.asm の `jmp 0x18:long_mode` と同じ値（64bitモードのコード）
            ist: 0,
            attributes: 0x8E, // 「有効・カーネル専用・処理中は他の割り込みを止める」
            offset_mid: (addr >> 16) as u16,
            offset_high: (addr >> 32) as u32,
            reserved: 0,
        }
    }
}

extern "C" fn divide_error() -> ! { show_error(b"DIVIDE ERROR") } // ゼロ除算
extern "C" fn invalid_opcode() -> ! { show_error(b"INVALID OPCODE") } // 無効な命令
extern "C" fn double_fault() -> ! { show_error(b"DOUBLE FAULT") } // ダブルフォルト（エラー処理自体が失敗）
extern "C" fn general_protection() -> ! { show_error(b"GENERAL PROTECTION") } // 一般保護違反
extern "C" fn page_fault() -> ! { show_error(b"PAGE FAULT") } // ページフォルト

fn show_error(message: &[u8]) -> ! {

    graphics::rect(160, 160, 320, font::LINE + 20, 4);
    font::draw_text(180, 170, b"Kingy has halted.", 15);

    let width = message.len() * (5 * font::SCALE + font::SPACE);
    font::draw_text((graphics::WIDTH - width) / 2, 240, message, 15);
    
    loop {
        unsafe { core::arch::asm!("cli", "hlt") }
    }
}

pub fn init() {
    unsafe {
        let idt = &raw mut IDT;
        (*idt)[0] = Entry::new(divide_error as *const ());
        (*idt)[6] = Entry::new(invalid_opcode as *const ());
        (*idt)[8] = Entry::new(double_fault as *const ());
        (*idt)[13] = Entry::new(general_protection as *const ());
        (*idt)[14] = Entry::new(page_fault as *const ());
        (*idt)[32] = Entry::new(crate::timer::timer_entry as *const ());

        let pointer = IdtPointer {
            limit: (core::mem::size_of::<[Entry; 256]>() - 1) as u16,
            base: idt as u64,
        };
        core::arch::asm!("lidt [{}]", in(reg) &pointer, options(readonly, nostack, preserves_flags));
    }
}