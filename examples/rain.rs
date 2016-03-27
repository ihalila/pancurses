/// **************************************************************************
/// Copyright (c) 2002 Free Software Foundation, Inc.                        *
///                                                                          *
/// Permission is hereby granted, free of charge, to any person obtaining a  *
/// copy of this software and associated documentation files (the            *
/// "Software"), to deal in the Software without restriction, including      *
/// without limitation the rights to use, copy, modify, merge, publish,      *
/// distribute, distribute with modifications, sublicense, and/or sell       *
/// copies of the Software, and to permit persons to whom the Software is    *
/// furnished to do so, subject to the following conditions:                 *
///                                                                          *
/// The above copyright notice and this permission notice shall be included  *
/// in all copies or substantial portions of the Software.                   *
///                                                                          *
/// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS  *
/// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF               *
/// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.   *
/// IN NO EVENT SHALL THE ABOVE COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,   *
/// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR    *
/// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR    *
/// THE USE OR OTHER DEALINGS IN THE SOFTWARE.                               *
///                                                                          *
/// Except as contained in this notice, the name(s) of the above copyright   *
/// holders shall not be used in advertising or otherwise to promote the     *
/// sale, use or other dealings in this Software without prior written       *
/// authorization.                                                           *
/// *************************************************************************

extern crate pancurses;
extern crate rand;

use pancurses::*;
use rand::Rng;

fn next_j<T: Rng>(mut j: usize, rng: &mut T, window: &Window) -> usize {
    if j == 0 {
        j = 4;
    } else {
        j -= 1;
    }

    if has_colors() {
        let z = rng.gen::<chtype>() % 3;
        let mut color = COLOR_PAIR(z);

        if z != 0 {
            color |= A_BOLD;
        }

        window.attrset(color);
    }

    return j;
}

fn main() {
    let window = initscr();

    let mut rng = rand::thread_rng();

    if has_colors() {
        let mut bg = COLOR_BLACK;

        start_color();
        if use_default_colors() == OK {
            bg = -1;
        }

        init_pair(1, COLOR_BLUE, bg);
        init_pair(2, COLOR_CYAN, bg);
    }

    nl();
    noecho();
    curs_set(0);
    window.timeout(0);
    window.keypad(true);

    let mut r = window.get_max_y() - 4;
    let mut c = window.get_max_x() - 4;

    let mut xpos = [0; 5];
    let mut ypos = [0; 5];

    for j in (0..5).rev() {
        xpos[j] = rng.gen::<i32>() % c + 2;
        ypos[j] = rng.gen::<i32>() % r + 2;
    }

    let mut j = 0;

    loop {
        let x = rng.gen::<i32>() % c + 2;
        let y = rng.gen::<i32>() % r + 2;

        window.mvaddch(y, x, '.' as chtype);

        window.mvaddch(ypos[j], xpos[j], 'o' as chtype);

        j = next_j(j, &mut rng, &window);
        window.mvaddch(ypos[j], xpos[j], 'O' as chtype);

        j = next_j(j, &mut rng, &window);
        window.mvaddch(ypos[j] - 1, xpos[j], '-' as chtype);
        window.mvaddstr(ypos[j], xpos[j] - 1, "|.|");
        window.mvaddch(ypos[j] + 1, xpos[j], '-' as chtype);

        j = next_j(j, &mut rng, &window);
        window.mvaddch(ypos[j] - 2, xpos[j], '-' as chtype);
        window.mvaddstr(ypos[j] - 1, xpos[j] - 1, "/ \\");
        window.mvaddstr(ypos[j], xpos[j] - 2, "| O |");
        window.mvaddstr(ypos[j] + 1, xpos[j] - 1, "\\ /");
        window.mvaddch(ypos[j] + 2, xpos[j], '-' as chtype);

        j = next_j(j, &mut rng, &window);
        window.mvaddch(ypos[j] - 2, xpos[j], ' ' as chtype);
        window.mvaddstr(ypos[j] - 1, xpos[j] - 1, "   ");
        window.mvaddstr(ypos[j], xpos[j] - 2, "     ");
        window.mvaddstr(ypos[j] + 1, xpos[j] - 1, "   ");
        window.mvaddch(ypos[j] + 2, xpos[j], ' ' as chtype);

        xpos[j] = x;
        ypos[j] = y;

        match window.getch() {
            Some(Input::Character(q)) if q == 'q' || q == 'Q' => {
                curs_set(1);
                endwin();
                return;
            }
            Some(Input::Character('s')) => {
                window.nodelay(false);
            }
            Some(Input::Character(' ')) => {
                window.nodelay(true);
            }
            Some(Input::KeyResize) => {
                resize_term(0, 0);
                window.erase();
                r = window.get_max_y() - 4;
                c = window.get_max_x() - 4;
            }
            _ => {}
        }
        napms(50);
    }
}
