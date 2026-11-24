use crate::{font, graphics, paging, user};

// アプリからの「int 0x80」の入口
#[unsafe(naked)]
pub extern "C" fn syscall_entry() {
    core::arch::naked_asm!(
        "push rax", "push rcx", "push rdx", "push rsi", "push rdi",
        "push r8", "push r9", "push r10", "push r11",
        "cld",
        // アプリは rax=番号, rdi=引数1, rsi=引数2 で渡してくる
        // Rustの関数は rdi=1番目, rsi=2番目, rdx=3番目 で受け取るので並べ替える
        "mov rdx, rsi",
        "mov rsi, rdi",
        "mov rdi, rax",
        "call {handler}",
        "mov [rsp + 64], rax", // 戻り値を、退避してある rax の場所に上書きする
        "pop r11", "pop r10", "pop r9", "pop r8",
        "pop rdi", "pop rsi", "pop rdx", "pop rcx", "pop rax",
        "iretq",
        handler = sym syscall_handler,
    )
}

const WRITE: u64 = 1;
const EXIT: u64 = u64::MAX - 1;
const ERROR: u64 = u64::MAX;
const USER_END: u64 = 0x0000_8000_0000_0000; // アプリが使える番地の終わり

const TEXT_TOP: usize = 365;
static mut TEXT_Y: usize = TEXT_TOP;

extern "C" fn syscall_handler(number: u64, arg1: u64, arg2: u64) -> u64 {
    match number {
        WRITE => write(arg1, arg2),
        EXIT => user::exit(arg1),
        _ => ERROR, // 知らない番号
    }
}

fn write(addr: u64, len: u64) -> u64 {
    // 文字列がアプリの領域に収まっているか確かめる
    let Some(end) = addr.checked_add(len) else {
        return ERROR;
    };
    if addr < paging::USER_BASE as u64 || end > USER_END || len > 64 {
        return ERROR;
    }
    let text = unsafe { core::slice::from_raw_parts(addr as *const u8, len as usize) };
    print(text, graphics::color::WHITE);
    0
}

// 文字を出す場所に1行書いて、次の行に進む
pub fn print(text: &[u8], color: u8) {
    unsafe {
        font::draw_text(10, TEXT_Y, text, color);
        TEXT_Y += font::LINE;
    }
}

// 終了コードを「exit 42」の形で1行書く
pub fn print_exit(code: u64) {
    unsafe {
        font::draw_text(10, TEXT_Y, b"exit", graphics::color::YELLOW);
        font::draw_number(10 + 16 * 5, TEXT_Y, code, graphics::color::YELLOW);
        TEXT_Y += font::LINE;
    }
}

// 文字を出す場所を消して、一番上に戻る
pub fn clear() {
    graphics::rect(0, TEXT_TOP, graphics::WIDTH, graphics::HEIGHT - TEXT_TOP, graphics::color::CYAN);
    unsafe { TEXT_Y = TEXT_TOP };
}