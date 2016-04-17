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

    set_title("NewTest: tests various pancurses features");

    mousemask(ALL_MOUSE_EVENTS, std::ptr::null_mut());

    window.attrset(COLOR_PAIR(1));

    let mut quit = false;
    let mut redraw = true;

    const COL1: i32 = 2;
    const COL2: i32 = COL1 + 30;
    const COL3: i32 = 72;

    while !quit {
        let (y_max, x_max) = window.get_max_yx();
        let color_block_start = 54;
        let mut color_block_cols = (x_max - color_block_start) / 2;
        let color_block_lines = 19;

        if color_block_cols < 0 {
            color_block_cols = 0;
        }

        if redraw {
            window.mvaddstr(1, COL1, "'Normal' white-on-black");
            window.attron(A_DIM);
            window.mvaddstr(2, COL1, "Dimmed text");
            window.attroff(A_DIM);
        }

        //These two lines are just here to allow running this until its finished
        window.getch();
        break;
    }

    endwin();
}
