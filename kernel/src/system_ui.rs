use crate::{font, graphics, kbmp, launcher, memory};

static KINGY_BMP: &[u8] = include_bytes!("../../assets/kingy.kbmp");


pub fn draw_init_screen() {
    graphics::clear(graphics::color::LIGHT_BLUE);

    draw_status_bar();
    launcher::draw();

    let kbmp = kbmp::read_kbmp(KINGY_BMP);
    kbmp::draw_kbmp(kbmp, 320, 250, 6,true);
}

pub fn draw_status_bar(){
    graphics::rect(0, 0, 640, 40, graphics::color::GRAY);
    font::draw_text(10, 10, b"KingyOS", graphics::color::WHITE);
    memory::draw_usage();
}