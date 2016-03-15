extern crate pancurses;

use pancurses::*;

fn main() {
    let window = initialize();
    window.set_nodelay(true);
    if (has_colors()) { start_color(); }
    noecho();
    end();
}
