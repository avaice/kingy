use crate::{font, graphics, memory};

pub fn draw_init_screen() {
    graphics::clear(graphics::color::LIGHT_BLUE);

    draw_status_bar();

    font::draw_text(10, 60, b"Paging Test: F1, F2", graphics::color::WHITE);
    font::draw_text(10, 60 + font::LINE, b"Hello, World: F3", graphics::color::WHITE);
    font::draw_text(10, 60 + font::LINE * 2, b"Bad App: F4", graphics::color::WHITE);
    font::draw_text(10, 60 + font::LINE * 3, b"Count Down: F5", graphics::color::WHITE);
}

pub fn draw_status_bar(){
    graphics::rect(0, 0, 640, 40, graphics::color::GRAY);
    font::draw_text(10, 10, b"KingyOS", graphics::color::WHITE);
    memory::draw_usage();
}