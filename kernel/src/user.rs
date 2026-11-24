use crate::{gdt, memory, paging};

core::arch::global_asm!(
    // アプリ1：Ring3 から write を呼ぶ
    ".global hello_start",
    ".global hello_end",
    "hello_start:",
    "    mov rax, 1",                 // 番号 1 = write
    "    lea rdi, [rip + hello_msg]", // 引数1 = 文字列の番地
    "    mov rsi, 17",                // 引数2 = 文字列の長さ
    "    int 0x80",                   // カーネルに頼む
    "hello_loop:",
    "    jmp hello_loop",             // 終わったら待つ
    "hello_msg:",
    "    .ascii \"Hello from Ring3!\"",
    "hello_end:",

    // アプリ2：カーネルのメモリを読もうとする
    ".global bad_start",
    ".global bad_end",
    "bad_start:",
    "    mov rax, qword ptr [0x20000]", // カーネル本体がいる番地
    "bad_loop:",
    "    jmp bad_loop",
    "bad_end:",
);

unsafe extern "C" {
    static hello_start: u8;
    static hello_end: u8;
    static bad_start: u8;
    static bad_end: u8;
}

pub fn hello() -> ! {
    run(&raw const hello_start as usize, &raw const hello_end as usize)
}

pub fn bad() -> ! {
    run(&raw const bad_start as usize, &raw const bad_end as usize)
}

// アプリを起動する
fn run(start: usize, end: usize) -> ! {
    let code = paging::USER_BASE;                // アプリのコードを置く番地
    let stack_top = paging::USER_BASE + 0x2000;  // アプリのスタック（1ページ）の一番上

    // アプリ専用のアドレス空間を作って、コード用とスタック用のページを1枚ずつ登録する
    let space = paging::new_address_space();
    let code_page = memory::alloc().unwrap();
    let stack_page = memory::alloc().unwrap();
    paging::map_user_page(space, code, code_page);
    paging::map_user_page(space, code + 0x1000, stack_page);

    // アプリの機械語をコード用のページにコピーする
    unsafe {
        core::ptr::copy_nonoverlapping(start as *const u8, code_page as *mut u8, end - start);
    }

    paging::switch(space);
    enter_user(code, stack_top)
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