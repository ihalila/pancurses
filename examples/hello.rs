extern crate pancurses;

use pancurses::{initscr, printw, refresh, endwin};

fn main() {
    let window = initscr();
    printw("Hello Rust");
    refresh();
    window.getch();
    endwin();
}
