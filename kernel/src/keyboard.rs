use crate::{io, keymap, pic};

pub enum Key {
    Character(u8), // 値を持てる便利なenum
    Enter,
    Backspace,
    Escape,
    Up,
    Down,
    Left,
    Right,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

pub struct Keyboard {
    left_shift: bool,
    right_shift: bool,

    // 矢印キーとかだと最初に0xE0がくる。2バイトで届くキーコードにも対応させるために必要
    // [TODO] 3バイト以上のキーコードもある。例えばPauseキーは0xE1 0x1D 0x45 0xE1 0x9D 0xC5の6バイト
    extended: bool,
}

impl Keyboard {
    pub const fn new() -> Self {
        Self {
            left_shift: false,
            right_shift: false,
            extended: false,
        }
    }
    pub fn process(&mut self, code: u8) -> Option<Key> {

        // 拡張キー
        if code == 0xE0 {
            self.extended = true;
            return None;
        }
        if self.extended {
            self.extended = false;
            return match code {
                0x48 => Some(Key::Up),
                0x50 => Some(Key::Down),
                0x4B => Some(Key::Left),
                0x4D => Some(Key::Right),
                _ => None,
            };
        }

        match code {
            0x2A => {
                self.left_shift = true;
                return None;
            }
            0xAA => {
                self.left_shift = false;
                return None;
            }
            0x36 => {
                self.right_shift = true;
                return None;
            }
            0xB6 => {
                self.right_shift = false;
                return None;
            }
            _ => {}
        }

        // キーを離した場合
        if code & 0x80 != 0 {
            return None;
        }

        match code {
            0x01 => Some(Key::Escape),
            0x1C => Some(Key::Enter),
            0x0E => Some(Key::Backspace),
            0x3B => Some(Key::F1),
            0x3C => Some(Key::F2),
            0x3D => Some(Key::F3),
            0x3E => Some(Key::F4),
            0x3F => Some(Key::F5),
            0x40 => Some(Key::F6),
            0x41 => Some(Key::F7),
            0x42 => Some(Key::F8),
            0x43 => Some(Key::F9),
            0x44 => Some(Key::F10),
            0x57 => Some(Key::F11),
            0x58 => Some(Key::F12),
            _ => {
                let shift = self.left_shift || self.right_shift;
                keymap::to_ascii(code, shift).map(Key::Character)
            }
        }
    }
}

const QUEUE_SIZE: usize = 64;
static mut QUEUE: [u8; QUEUE_SIZE] = [0; QUEUE_SIZE];
static mut HEAD: usize = 0; // 挿入する位置
static mut TAIL: usize = 0; // 取り出す位置

fn push(code: u8) {
    unsafe {
        let next = (HEAD + 1) % QUEUE_SIZE;
        if next == TAIL {
            return; // リングバッファが満杯なら入れない
        }
        QUEUE[HEAD] = code;
        HEAD = next;
    }
}

pub fn pop() -> Option<u8> {
    unsafe {
        core::arch::asm!("cli"); // 割り込みを禁止
        let result = if TAIL == HEAD {
            None
        } else {
            let code = QUEUE[TAIL];
            TAIL = (TAIL + 1) % QUEUE_SIZE;
            Some(code)
        };
        core::arch::asm!("sti"); // 割り込みを再開
        result
    }
}

#[unsafe(naked)]
pub extern "C" fn keyboard_entry() {
    core::arch::naked_asm!(
        "push rax", "push rcx", "push rdx", "push rsi", "push rdi",
        "push r8", "push r9", "push r10", "push r11",
        "cld",
        "call {handler}",
        "pop r11", "pop r10", "pop r9", "pop r8",
        "pop rdi", "pop rsi", "pop rdx", "pop rcx", "pop rax",
        "iretq",
        handler = sym keyboard_handler,
    )
}

extern "C" fn keyboard_handler() {
    let code = unsafe { io::inb(0x60) }; // キーボードから1バイト読む
    push(code);                          // リングバッファに入れる
    pic::end_of_interrupt();
}