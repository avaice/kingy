use crate::{gdt, memory, paging};

const APP_SIZE: usize = 0x10000;                        // アプリ用の領域（apps/hello/app.ld と同じにする）
const STACK_BOTTOM: usize = paging::USER_BASE + APP_SIZE + memory::PAGE_SIZE; // 間を開けると、スタックオーバーフローした時にページフォルトにできる
const STACK_SIZE: usize = 0x4000;                       // スタックの大きさ

static HELLO: &[u8] = include_bytes!("../../build/hello.bin");
static COUNT: &[u8] = include_bytes!("../../build/count.bin");
static TTT: &[u8] = include_bytes!("../../build/ttt.bin");

// エラーで止められたときの終了コード
pub const KILLED: u64 = u64::MAX;
pub const PANIC: u64 = 101;

static mut APP_SPACE: usize = 0;  // 動いているアプリのアドレス空間
static mut KERNEL_RSP: usize = 0; // アプリを起動したときのカーネルのスタック位置

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

pub fn hello() -> u64 {
    run(HELLO)
}

pub fn count() -> u64 {
    run(COUNT)
}

pub fn ttt() -> u64 {
    run(TTT)
}
pub fn bad() -> u64 {
    let start = &raw const bad_start as usize;
    let end = &raw const bad_end as usize;
    run(unsafe { core::slice::from_raw_parts(start as *const u8, end - start) })
}

// アプリを起動する
fn run(app: &[u8]) -> u64 {
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
        APP_SPACE = space;
    }

    let code = enter_user(paging::USER_BASE, STACK_BOTTOM + STACK_SIZE);

    unsafe { core::arch::asm!("sti") }; // システムコールやエラーの中から戻ってくるので、割り込みが止まっている
    code
}

// Ring3 に降りて、entry 番地から実行を始める。アプリが終わると、終了コードを持って戻ってくる
#[unsafe(naked)]
extern "C" fn enter_user(entry: usize, stack: usize) -> u64 {
    core::arch::naked_asm!(
        // あとで戻ってこられるように、スタックの位置を保存する
        "push rbp", "push rbx", "push r12", "push r13", "push r14", "push r15",
        "mov [rip + {saved}], rsp",
        // iretq が取り出す5つを積んで、Ring3 に降りる
        "push {ss}",     // スタックの番号（Ring3）
        "push rsi",      // スタックの番地（2番目の引数 stack）
        "push 0x202",    // フラグ（割り込み有効）
        "push {cs}",     // コードの番号（Ring3）
        "push rdi",      // 実行を始める番地（1番目の引数 entry）
        "iretq",
        saved = sym KERNEL_RSP,
        ss = const gdt::USER_DATA | 3,
        cs = const gdt::USER_CODE | 3,
    )
}

// アプリを終わらせて、enter_user を呼んだ場所に戻る（exit システムコールや、アプリのエラーから呼ばれる）
pub fn exit(code: u64) -> ! {
    unsafe {
        paging::switch(paging::kernel_space()); // 解放するアドレス空間を使ったままにしない
        paging::free_address_space(APP_SPACE);  // アプリが使っていたメモリを全部解放する
        return_to_kernel(code, KERNEL_RSP)
    }
}

// 保存しておいたスタックに戻ってレジスタを取り出し、enter_user から戻る
#[unsafe(naked)]
extern "C" fn return_to_kernel(code: u64, rsp: usize) -> ! {
    core::arch::naked_asm!(
        "mov rax, rdi", // 戻り値 = 終了コード
        "mov rsp, rsi", // enter_user で保存したスタックの位置に戻る
        "pop r15", "pop r14", "pop r13", "pop r12", "pop rbx", "pop rbp",
        "ret",          // enter_user を呼んだ場所（run の中）に戻る
    )
}