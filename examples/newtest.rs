extern crate pancurses;

use pancurses::*;

fn main() {
    let window = initscr();

    start_color();
    use_default_colors();

    cbreak();
    noecho();

    window.clear();
    window.refresh();

    window.keypad(true);

    init_pair(1, 15, COLOR_BLACK);
    init_pair(2, COLOR_BLACK, COLOR_YELLOW);

    mousemask(ALL_MOUSE_EVENTS, std::ptr::null_mut());

    endwin();
}
