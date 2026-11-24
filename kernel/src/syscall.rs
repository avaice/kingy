use crate::{font, graphics, kbmp, keyboard, paging, pic, system_ui, user};

// アプリからの「int 0x80」の入口
#[unsafe(naked)]
pub extern "C" fn syscall_entry() {
    core::arch::naked_asm!(
        "push rax", "push rcx", "push rdx", "push rsi", "push rdi",
        "push r8", "push r9", "push r10", "push r11",
        "cld",
        "mov rdi, rsp",
        "call {handler}",
        "mov [rsp + 64], rax", // 戻り値を、退避してある rax の場所に上書きする
        "pop r11", "pop r10", "pop r9", "pop r8",
        "pop rdi", "pop rsi", "pop rdx", "pop rcx", "pop rax",
        "iretq",
        handler = sym syscall_handler,
    )
}

#[repr(C)]
struct Frame {
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rdi: u64,
    rsi: u64,
    rdx: u64,
    rcx: u64,
    rax: u64,
}

const WRITE: u64 = 1;
const PRINT: u64 = 2;
const RECT: u64 = 3;
const CLEAR: u64 = 4;
const DRAW_KBMP: u64 = 5;
const GET_KEY_CODE: u64 = 6;
const EXIT: u64 = u64::MAX - 1;
const ERROR: u64 = u64::MAX;
const USER_END: u64 = 0x0000_8000_0000_0000; // アプリが使える番地の終わり

const APP_DISPLAY_AREA_START_X: usize = 0;
const APP_DISPLAY_AREA_START_Y: usize = 40;

const TEXT_TOP: usize = 10;
static mut TEXT_Y: usize = TEXT_TOP;

// rax=番号, rdi, rsi, rdx, r10, r8, r9 = 引数1〜6
extern "C" fn syscall_handler(frame: &Frame) -> u64 {
    let args = [frame.rdi, frame.rsi, frame.rdx, frame.r10, frame.r8, frame.r9];
    match frame.rax {
        WRITE => write(args[0], args[1]),
        PRINT => print(args[0], args[1], args[2], args[3], args[4]),
        RECT => rect(args[0], args[1], args[2], args[3], args[4]),
        CLEAR => clear_call_back(),
        DRAW_KBMP => draw_kbmp(args[0], args[1], args[2], args[3], args[4], args[5]),
        GET_KEY_CODE => get_key_code(),
        EXIT => user::exit(args[0]),
        _ => ERROR, // 知らない番号
    }
}

fn user_slice(addr: u64, len: u64) -> Option<&'static [u8]> {
    let end = addr.checked_add(len)?;
    if addr < paging::USER_BASE as u64 || end > USER_END {
        return None;
    }
    Some(unsafe { core::slice::from_raw_parts(addr as *const u8, len as usize) })
}

fn screen_pos(x: u64, y: u64) -> Option<(usize, usize)> {
    if x >= graphics::WIDTH as u64 || y >= graphics::HEIGHT as u64 {
        return None;
    }
    Some((x as usize + APP_DISPLAY_AREA_START_X, y as usize + APP_DISPLAY_AREA_START_Y))
}

fn write(addr: u64, len: u64) -> u64 {
    let y = unsafe { TEXT_Y } as u64;
    let result = print(addr, len, 10, y, graphics::color::WHITE as u64);
    if result != ERROR {
        unsafe { TEXT_Y += font::LINE };
    }
    result
}

fn print(addr: u64, len: u64, x: u64, y: u64, color: u64) -> u64 {
    if len > 64 {
        return ERROR;
    }
    let Some(text) = user_slice(addr, len) else {
        return ERROR;
    };
    let Some((x, y)) = screen_pos(x, y) else {
        return ERROR;
    };
    font::draw_text(x, y, text, color as u8);
    0
}

fn rect(x: u64, y: u64, width: u64, height: u64, color: u64) -> u64 {
    let Some((x, y)) = screen_pos(x, y) else {
        return ERROR;
    };
    graphics::rect(x, y, width as usize, height as usize, color as u8);
    0
}

fn clear_call_back() -> u64 {
    clear();
    0
}

const KBMP_HEADER: u64 = 8;
const KBMP_MAX_SCALE: u64 = 16;



fn draw_kbmp(addr: u64, len: u64, x: u64, y: u64, scale: u64, transparent: u64) -> u64 {
    let Some(bytes) = user_slice(addr, len) else {
        return ERROR;
    };
    if len < KBMP_HEADER || scale == 0 || scale > KBMP_MAX_SCALE {
        return ERROR;
    }
    let Some((x, y)) = screen_pos(x, y) else {
        return ERROR;
    };
    let kbmp = kbmp::read_kbmp(bytes);
    // ヘッダに書かれた大きさの分だけ画素が揃っているか
    let stride = (kbmp.width as usize + 1) / 2;
    if kbmp.pixels.len() < stride * kbmp.height as usize {
        return ERROR;
    }
    kbmp::draw_kbmp(kbmp, x, y, scale as usize, transparent != 0);
    0
}

fn get_key_code() -> u64 {
    match keyboard::pop() {
        Some(code) => code as u64,
        None => ERROR,
    }
}

// 終了コードを「exit 42」の形で1行書く
pub fn print_exit(code: u64) {
        font::draw_text(10, 420, b"exit", graphics::color::YELLOW);
        font::draw_number(10 + 16 * 5, 420, code, graphics::color::YELLOW);
}

pub fn crushed(code: u64) {
    pic::mute_timer();
    graphics::clear(graphics::color::RED);
    font::draw_number(10, 10, code, graphics::color::WHITE);
    font::draw_text(10, 420, b"Application Crushed", graphics::color::WHITE);
    font::draw_text(10, 440, b"Press Esc key to continue", graphics::color::WHITE);
    let mut keyboard = keyboard::Keyboard::new();
    loop {
        let Some(code) = keyboard::pop() else {
            unsafe { core::arch::asm!("hlt") };
            continue;
        };
        match keyboard.process(code) {
            Some(keyboard::Key::Escape) => {
                break;
            }
            _ => continue,
        }
    }
    pic::unmute_timer();
}

// 文字を出す場所を消して、一番上に戻る
pub fn clear() {
    graphics::clear(graphics::color::BLACK);
    system_ui::draw_status_bar();
    unsafe { TEXT_Y = TEXT_TOP };
}