use crate::{font, graphics, syscall, system_ui, user};

static mut CURSOR: usize = 0;
const APPS: [&str; 4] = ["Hello, World", "Bad App", "Count Down", "Tic Tac Toe"];

pub fn draw() {
    
    graphics::rect(10, 50, 300, 300, graphics::color::WHITE);

    graphics::rect(10, 55 + unsafe { CURSOR } * font::LINE, 300, 30, graphics::color::LIGHT_CYAN);
    font::draw_text(10, 60 + unsafe { CURSOR } * font::LINE, b">", graphics::color::BLACK);

    for (i, app) in APPS.iter().enumerate() {
        font::draw_text(40, 60 + i * font::LINE, app.as_bytes(), graphics::color::BLACK);
    }
    
}

pub fn up() {
    if unsafe { CURSOR } > 0 {
        unsafe { CURSOR -= 1; }
        draw();
    }
}

pub fn down() {
    if unsafe { CURSOR } < APPS.len() - 1 {
        unsafe { CURSOR += 1; }
        draw();
    }
}

pub fn select() {
    match unsafe { CURSOR } {
        0 => start(user::hello),
        1 => start(user::bad),
        2 => start(user::count),
        3 => start(user::ttt),
        _ => unreachable!(),
    };
}

fn start(app: fn() -> u64) {
    syscall::clear();
    let code = app();
    if code == user::KILLED || code == user::PANIC {
        syscall::crushed(code);
        system_ui::draw_init_screen();
    } else {
        system_ui::draw_init_screen();
        syscall::print_exit(code);
    }
    
}