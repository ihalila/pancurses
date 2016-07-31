extern crate pancurses;

use pancurses::*;

fn main() {
    let window = initscr();

    start_color();
    use_default_colors();
    set_blink(true);

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

    let mut unicode_offset = 0x80;
    let mut blink_state = true;

    while !quit {
        let (y_max, x_max) = window.get_max_yx();
        let color_block_start = 54;
        let mut color_block_cols = (x_max - color_block_start) / 2;
        let color_block_lines = 19;

        let mut cursor_state_1 = 2;
        let mut cursor_state_2 = 3;
        let cursor_state_text = ["Invisible (click to change) ",
                                 "Underscore (click to change)",
                                 "Block (click to change)     ",
                                 "Outline (click to change)   ",
                                 "Caret (click to change)     ",
                                 "Half-block (click to change)",
                                 "Central (click to change)   ",
                                 "Cross (click to change)     ",
                                 "Heavy box (click to change) "];

        if color_block_cols < 0 {
            color_block_cols = 0;
        }

        if redraw {
            window.mvaddstr(1, COL1, "'Normal' white-on-black");

            window.attron(A_DIM);
            window.mvaddstr(2, COL1, "Dimmed text");
            window.attroff(A_DIM);

            window.attron(A_BLINK);
            window.mvaddstr(6, 40, "Blinking");
            window.attron(A_BOLD);
            window.mvaddstr(8, 40, "BlinkBold");
            window.attron(A_ITALIC);
            window.mvaddstr(0, COL2, "BlinkBoldItalic");
            window.attrset(COLOR_PAIR(3));
            window.attron(A_UNDERLINE);

            window.mvaddstr(1, COL2, "Underlined");

            window.attrset(COLOR_PAIR(1));
            window.attron(A_UNDERLINE | A_ITALIC);
            window.mvaddstr(2, COL2, "UnderlinedItalic");
            window.attrset(COLOR_PAIR(2));
            window.attron(A_BLINK);
            window.mvaddstr(4, COL1, "Black-on-yellow blinking");

            window.attrset(COLOR_PAIR(1));
            window.mv(4, COL2);
            text_in_a_box("Text in a box", &window);

            window.attrset(COLOR_PAIR(6));
            window.attron(A_STRIKEOUT);
            window.mvaddstr(10, 40, "Strikeout");
            window.attrset(COLOR_PAIR(1));

            window.mv(11, 40);
            text_in_a_box("Next Ucode pg", &window);
            if unicode_offset != 0 {
                window.mv(12, 40);
                text_in_a_box("Prev Ucode pg", &window);
            }
            window.mvprintw(13, 40, &format!("U+{:04X} ", unicode_offset));

            for i in 0..128 {
                // Show extended characters
                window.mvaddstr(5 + i % 16,
                                (i / 16) * 5,
                                &format!("{:02X} ", i + unicode_offset));
                if i + unicode_offset > ' ' as i32 {
                    window.addch((i + unicode_offset) as chtype);
                } else {
                    window.addch(' ');
                }
                window.addch(' ');
            }

            // Skipped the full RGB example, I don't think there's a suitable feature in ncurses

            redraw = false;
            window.attrset(COLOR_PAIR(1));

            for i in 0..6 {
                let line = 24 + i / 3;
                let col = 5 + 25 * (i % 3);

                let spanish = "Español";
                let russian = "Русский язык";
                let greek = "Ελληνικά";
                let georgian = "ქართული ენა";
                let fullwidth = " Ｆｕｌｌｗｉｄｔｈ";
                let combining_marks = "Cmin 	r";

                let texts = [spanish, russian, greek, georgian, fullwidth, combining_marks];

                if line < y_max && col < x_max {
                    window.mvaddnstr(line, 5 + 25 * (i % 3), texts[i as usize], x_max - col);
                }
            }

            window.mvaddstr(1, COL3, "Click on cursor descriptions to");
            window.mvaddstr(2, COL3, "cycle through possible cursors");
            window.mvaddstr(3, COL3, "Click on colors at left to change");
            window.mvaddstr(4, COL3, "colors used for under/over/outlining");
            window.mvaddstr(5, COL3, "Click 'Blink' at bottom to toggle");
            window.mvaddstr(6, COL3, "'real' blinking vs. 'highlit' blink");
        }

        window.mvaddnstr(19,
                         color_block_start,
                         cursor_state_text[cursor_state_1],
                         x_max - color_block_start);
        window.mvaddnstr(20,
                         color_block_start,
                         cursor_state_text[cursor_state_2],
                         x_max - color_block_start);

        for i in 0..color_block_cols * color_block_lines {
            let n_color_blocks = 256;

            window.attrset(COLOR_PAIR(if i >= n_color_blocks { 2 } else { i as chtype }));
            if i > 2 && i < n_color_blocks {
                init_pair(i as i16, i as i16, COLOR_BLACK);
            }
            if i % color_block_cols == 0 {
                window.mv(i / color_block_cols, color_block_start);
            }
            window.attron(A_REVERSE);
            window.addstr(" ");
        }

        window.mv(19, color_block_start - 3);
        window.refresh();
        let c = window.getch();
        window.attrset(COLOR_PAIR(1));

        match c {
            Some(Input::KeyF1) => quit = true,
            Some(Input::Character(x)) if x == 27 as char => quit = true,
            Some(Input::KeyF2) => {
                blink_state = !blink_state;
                set_blink(blink_state);
            },
            Some(x) if x != Input::KeyMouse => {
                window.mvaddstr(0, COL1, &format!("Key {:?} hit          ", x));
            },
            Some(Input::KeyMouse) => {
                
            }
            _ => (),
        }
    }

    endwin();
}

fn text_in_a_box(text: &str, window: &Window) {
    let len = text.len();

    window.attron(A_OVERLINE | A_UNDERLINE | A_LEFTLINE);
    if len == 1 {
        window.attron(A_RIGHTLINE);
    }

    window.addnstr(text, 1);
    if len > 1 {
        window.attroff(A_LEFTLINE);
        if len > 2 {
            window.addnstr(&text[1..], len - 2);
        }
        window.attron(A_RIGHTLINE);
        window.addnstr(&text[len - 1..], 1);
    }

    window.attroff(A_OVERLINE | A_UNDERLINE | A_LEFTLINE | A_RIGHTLINE);
}
