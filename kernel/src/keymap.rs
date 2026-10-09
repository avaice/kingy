// us layout
pub fn to_ascii(code: u8, shift: bool) -> Option<u8> {
    let (normal, shifted) = match code {
        // 数字
        0x02 => (b'1', b'!'),
        0x03 => (b'2', b'@'),
        0x04 => (b'3', b'#'),
        0x05 => (b'4', b'$'),
        0x06 => (b'5', b'%'),
        0x07 => (b'6', b'^'),
        0x08 => (b'7', b'&'),
        0x09 => (b'8', b'*'),
        0x0A => (b'9', b'('),
        0x0B => (b'0', b')'),

        // 記号
        0x0C => (b'-', b'_'),
        0x0D => (b'=', b'+'),
        0x1A => (b'[', b'{'),
        0x1B => (b']', b'}'),
        0x2B => (b'\\', b'|'),
        0x27 => (b';', b':'),
        0x28 => (b'\'', b'"'),
        0x29 => (b'`', b'~'),
        0x33 => (b',', b'<'),
        0x34 => (b'.', b'>'),
        0x35 => (b'/', b'?'),

        // 英字
        0x1E => (b'a', b'A'),
        0x30 => (b'b', b'B'),
        0x2E => (b'c', b'C'),
        0x20 => (b'd', b'D'),
        0x12 => (b'e', b'E'),
        0x21 => (b'f', b'F'),
        0x22 => (b'g', b'G'),
        0x23 => (b'h', b'H'),
        0x17 => (b'i', b'I'),
        0x24 => (b'j', b'J'),
        0x25 => (b'k', b'K'),
        0x26 => (b'l', b'L'),
        0x32 => (b'm', b'M'),
        0x31 => (b'n', b'N'),
        0x18 => (b'o', b'O'),
        0x19 => (b'p', b'P'),
        0x10 => (b'q', b'Q'),
        0x13 => (b'r', b'R'),
        0x1F => (b's', b'S'),
        0x14 => (b't', b'T'),
        0x16 => (b'u', b'U'),
        0x2F => (b'v', b'V'),
        0x11 => (b'w', b'W'),
        0x2D => (b'x', b'X'),
        0x15 => (b'y', b'Y'),
        0x2C => (b'z', b'Z'),

        0x39 => (b' ', b' '), // スペース
        _ => return None,     // 未対応のキー
    };

    Some(if shift { shifted } else { normal })
}
