use crate::{gdt, memory, paging};

const APP_SIZE: usize = 0x10000;                        // アプリ用の領域（apps/hello/app.ld と同じにする）
const STACK_BOTTOM: usize = paging::USER_BASE + APP_SIZE + memory::PAGE_SIZE; // 間を開けると、スタックオーバーフローした時にページフォルトにできる
const STACK_SIZE: usize = 0x4000;                       // スタックの大きさ

static HELLO: &[u8] = include_bytes!("../../build/hello.bin");

core::arch::global_asm!(
    // カーネルのメモリを読もうとする極悪アプリ
    ".global bad_start",
    ".global bad_end",
    "bad_start:",
    "    mov rax, qword ptr [0x20000]", // カーネル本体がいる番地
    "bad_loop:",
    "    jmp bad_loop",
    "bad_end:",
);


unsafe extern "C" {
    static bad_start: u8;
    static bad_end: u8;
}

pub fn hello() -> ! {
    run(HELLO)
}

pub fn bad() -> ! {
    let start = &raw const bad_start as usize;
    let end = &raw const bad_end as usize;
    run(unsafe { core::slice::from_raw_parts(start as *const u8, end - start) })
}

// アプリを起動する
fn run(app: &[u8]) -> ! {
    if app.len() > APP_SIZE {
        panic!(); // アプリ用の領域に入りきらない
    }

    // アプリ専用のアドレス空間を作って、アプリ用の領域とスタックにページを1枚ずつ登録する
    let space = paging::new_address_space();
    for offset in (0..APP_SIZE).step_by(memory::PAGE_SIZE) {
        paging::map_user_page(space, paging::USER_BASE + offset, memory::alloc().unwrap());
    }
    for offset in (0..STACK_SIZE).step_by(memory::PAGE_SIZE) {
        paging::map_user_page(space, STACK_BOTTOM + offset, memory::alloc().unwrap());
    }
    paging::switch(space);
    unsafe {
        core::ptr::copy_nonoverlapping(app.as_ptr(), paging::USER_BASE as *mut u8, app.len());
    }

    enter_user(paging::USER_BASE, STACK_BOTTOM + STACK_SIZE)
}

// Ring3 に降りて、entry 番地から実行を始める
fn enter_user(entry: usize, stack: usize) -> ! {
    unsafe {
        core::arch::asm!(
            "push {ss}",     // スタックの番号（Ring3）
            "push {rsp}",    // スタックの番地
            "push {rflags}", // フラグ（割り込み有効）
            "push {cs}",     // コードの番号（Ring3）
            "push {rip}",    // 実行を始める番地
            "iretq",
            ss = in(reg) (gdt::USER_DATA | 3) as u64,
            rsp = in(reg) stack,
            rflags = in(reg) 0x202u64,
            cs = in(reg) (gdt::USER_CODE | 3) as u64,
            rip = in(reg) entry,
            options(noreturn),
        )
    }
}