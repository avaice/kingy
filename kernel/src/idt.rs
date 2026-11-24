#[derive(Clone, Copy)]
#[repr(C, packed)] // Rustは自動でメモリを並び替えたり隙間を空けたりしてくれるので、それを防ぐためにpackedを使う
struct Entry {
    offset_low: u16,  // 関数アドレスの下位16bit
    selector: u16,    // どのモードで実行するか
    ist: u8,          // スタックが壊れている時にトリプルフォルトにならないように別のスタックを使うことができる

    //  設定項目
    //   bit 7    : P    … 1 = この行は有効
    //   bit 6〜5 : DPL  … int 命令で呼べる Ring（00 = カーネルだけ、11 = アプリからもOK）
    //   bit 4    : 0    … 常に0
    //   bit 3〜0 : 種類 … 1110 = 処理中は割り込みを止める
    //                      1111 = 止めない
    //   例: 0x8E = 1 00 0 1110 … 有効・カーネルだけ・処理中は割り込みを止める
    //       0xEE = 1 11 0 1110 … 有効・アプリからもOK・処理中は割り込みを止める
    attributes: u8,

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
            selector: crate::gdt::KERNEL_CODE, // カーネルのコード
            ist: 0,
            attributes: 0x8E, // 有効・カーネル専用・処理中は他の割り込みを止める
            offset_mid: (addr >> 16) as u16,
            offset_high: (addr >> 32) as u32,
            reserved: 0,
        }
    }

    // Ring3からも呼べるようにする
    fn user_callable(mut self) -> Self {
        self.attributes = 0xEE; // 有効・Ring3からOK・処理中は割り込みを止める
        self
    }
}



pub fn init() {
    unsafe {
        let idt = &raw mut IDT;
        (*idt)[0] = Entry::new(crate::exception::divide_error as *const ());
        (*idt)[6] = Entry::new(crate::exception::invalid_opcode as *const ());
        (*idt)[8] = Entry::new(crate::exception::double_fault as *const ());
        (*idt)[13] = Entry::new(crate::exception::general_protection as *const ());
        (*idt)[14] = Entry::new(crate::exception::page_fault as *const ());
        (*idt)[32] = Entry::new(crate::timer::timer_entry as *const ());
        (*idt)[33] = Entry::new(crate::keyboard::keyboard_entry as *const ());
        (*idt)[0x80] = Entry::new(crate::syscall::syscall_entry as *const ()).user_callable();

        let pointer = IdtPointer {
            limit: (core::mem::size_of::<[Entry; 256]>() - 1) as u16,
            base: idt as u64,
        };
        core::arch::asm!("lidt [{}]", in(reg) &pointer, options(readonly, nostack, preserves_flags));
    }
}