use crate::{io, keymap};

pub enum Key {
    Character(u8), // 値を持てる便利なenum
    Enter,
    Backspace,
    Escape,
    Up,
    Down,
    Left,
    Right,
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
    pub fn poll(&mut self) -> Option<Key> {
        if unsafe { io::inb(0x64) } & 1 == 0 {
            // 入力がない場合
            return None;
        }

        let code = unsafe { io::inb(0x60) };

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
            _ => {
                let shift = self.left_shift || self.right_shift;
                keymap::to_ascii(code, shift).map(Key::Character)
            }
        }
    }
}
