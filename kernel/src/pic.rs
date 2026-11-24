use crate::io::outb;

const MASTER_COMMAND: u16 = 0x20;
const MASTER_DATA: u16 = 0x21;
const SLAVE_COMMAND: u16 = 0xA0;
const SLAVE_DATA: u16 = 0xA1;

pub fn init() {
    unsafe {
        outb(MASTER_COMMAND, 0x11);
        outb(SLAVE_COMMAND, 0x11);
        // 32まではエラーで使っているので、配置を変える
        outb(MASTER_DATA, 32);
        outb(SLAVE_DATA, 40);
        // マスターとスレーブのつなぎ方（決まった値）
        outb(MASTER_DATA, 0x04);
        outb(SLAVE_DATA, 0x02);
        // 動作モード（決まった値）
        outb(MASTER_DATA, 0x01);
        outb(SLAVE_DATA, 0x01);
        // ミュートするビットを1に
        outb(MASTER_DATA, 0b1111_1110);
        outb(SLAVE_DATA, 0b1111_1111);
    }
}

pub fn end_of_interrupt() {
    unsafe { outb(MASTER_COMMAND, 0x20) };
}