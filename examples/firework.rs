extern crate pancurses;
extern crate rand;

use pancurses::*;
use rand::Rng;

const DELAYSIZE: i32 = 200;

const COLOR_TABLE: [i16; 8] = [COLOR_RED,
                               COLOR_BLUE,
                               COLOR_GREEN,
                               COLOR_CYAN,
                               COLOR_RED,
                               COLOR_MAGENTA,
                               COLOR_YELLOW,
                               COLOR_WHITE];

fn main() {
    let window = initscr();
    window.set_nodelay(true);
    noecho();

    if has_colors() {
        start_color();
    }

    for (i, color) in COLOR_TABLE.into_iter().enumerate() {
        init_pair(i as i16, *color, COLOR_BLACK);
    }

    let mut rng = rand::thread_rng();

    let mut flag = 0;

    let lines = window.get_max_y();
    let cols = window.get_max_x();

    while window.getch() == ERR {
        let mut start = 0;
        let mut direction = 0;
        let mut diff = 0;

        while diff < 2 || diff >= lines - 2 {
            start = rng.gen::<i32>() % (cols - 3);
            let mut end = rng.gen::<i32>() % (cols - 3);
            start = if start < 2 {
                2
            } else {
                start
            };
            end = if end < 2 {
                2
            } else {
                end
            };
            direction = if start > end {
                -1
            } else {
                1
            };
            diff = (start - end).abs();
        }

        attrset(A_NORMAL);

        for row in 0..diff {
            mvaddstr(lines - row,
                     row * direction + start,
                     if direction < 0 {
                         "\\"
                     } else {
                         "/"
                     });

            if flag != 0 {
                myrefresh(&window);
                erase();
                flag = 0;
            } else {
                flag += 1;
            }
        }
        let row = diff;

        if flag != 0 {
            myrefresh(&window);
            flag = 0;
        } else {
            flag += 1;
        }

        explode(lines - row, diff * direction + start, &window, &mut rng);
        erase();
        myrefresh(&window);
    }

    endwin();
}

fn explode<T: Rng>(row: i32, mut col: i32, window: &Window, rng: &mut T) {
    erase();
    mvaddstr(row, col, "-");
    myrefresh(window);

    col -= 1;

    get_color(rng);
    mvaddstr(row - 1, col, " - ");
    mvaddstr(row,     col, "-+-");
    mvaddstr(row + 1, col, " - ");
    myrefresh(window);

    col -= 1;

    get_color(rng);
    mvaddstr(row - 2, col, " --- ");
    mvaddstr(row - 1, col, "-+++-");
    mvaddstr(row,     col, "-+#+-");
    mvaddstr(row + 1, col, "-+++-");
    mvaddstr(row + 2, col, " --- ");
    myrefresh(window);

    get_color(rng);
    mvaddstr(row - 2, col, " +++ ");
    mvaddstr(row - 1, col, "++#++");
    mvaddstr(row,     col, "+# #+");
    mvaddstr(row + 1, col, "++#++");
    mvaddstr(row + 2, col, " +++ ");
    myrefresh(window);

    get_color(rng);
    mvaddstr(row - 2, col, "  #  ");
    mvaddstr(row - 1, col, "## ##");
    mvaddstr(row,     col, "#   #");
    mvaddstr(row + 1, col, "## ##");
    mvaddstr(row + 2, col, "  #  ");
    myrefresh(window);

    get_color(rng);
    mvaddstr(row - 2, col, " # # ");
    mvaddstr(row - 1, col, "#   #");
    mvaddstr(row,     col, "     ");
    mvaddstr(row + 1, col, "#   #");
    mvaddstr(row + 2, col, " # # ");
    myrefresh(window);
}

fn myrefresh(window: &Window) {
    napms(DELAYSIZE);
    mv(window.get_max_y() - 1, window.get_max_x() - 1);
    refresh();
}

fn get_color<T: Rng>(rng: &mut T) {
    let bold = if rng.gen::<bool>() {
        A_BOLD
    } else {
        A_NORMAL
    } as chtype;
    attrset(COLOR_PAIR(rng.gen::<chtype>() % 8) | bold);
}
