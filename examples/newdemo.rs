extern crate pancurses;
extern crate rand;

use pancurses::*;
use pancurses::colorpair::ColorPair;

use rand::Rng;

const AUS_MAP: [&'static str; 13] = ["                       A ",
                                     "           AA         AA ",
                                     "    N.T. AAAAA       AAAA ",
                                     "     AAAAAAAAAAA  AAAAAAAA ",
                                     "   AAAAAAAAAAAAAAAAAAAAAAAAA Qld.",
                                     " AAAAAAAAAAAAAAAAAAAAAAAAAAAA ",
                                     " AAAAAAAAAAAAAAAAAAAAAAAAAAAAA ",
                                     " AAAAAAAAAAAAAAAAAAAAAAAAAAAA ",
                                     "   AAAAAAAAAAAAAAAAAAAAAAAAA N.S.W.",
                                     "W.A. AAAAAAAAA      AAAAAA Vic.",
                                     "       AAA   S.A.     AA",
                                     "                       A  Tas.",
                                     ""];

const MESSAGES: [&'static str; 6] = ["Hello from the Land Down Under",
                                     "The Land of crocs, and a big Red Rock",
                                     "Where the sunflower runs along the highways",
                                     "The dusty red roads lead one to loneliness",
                                     "Blue sky in the morning and",
                                     "Freezing nights and twinkling stars"];

fn main() {
    let main_window = initscr();

    start_color();

    use_default_colors();

    cbreak();
    noecho();

    curs_set(0);

    // Refresh stdscr so that reading from it will not cause it to  overwrite the other windows
    // that are being created
    main_window.refresh();

    // Create a drawing window
    let width = 48;
    let height = 15;

    let win = newwin(height,
                     width,
                     (main_window.get_max_y() - height) / 2,
                     (main_window.get_max_x() - width) / 2);

    loop {
        init_pair(1, COLOR_WHITE, COLOR_BLUE);
        win.bkgd(COLOR_PAIR(1));
        win.erase();

        init_pair(2, COLOR_RED, COLOR_RED);
        win.attrset(ColorPair(2));
        win.draw_box(' ', ' ');
        win.refresh();

        win.attrset(Attribute::Normal);

        // Do random output of a character

        let mut ch = 'a';

        main_window.nodelay(true);

        let mut rng = rand::thread_rng();

        for i in 0..5000 {
            let x = (rng.gen::<i32>() % (width - 2)).abs() + 1;
            let y = (rng.gen::<i32>() % (height - 2)).abs() + 1;

            assert!(x != 0);
            assert!(y != 0);
            win.mvaddch(y, x, ch);
            win.refresh();

            if main_window.getch().is_some() {
                break;
            }

            if i == 2000 {
                ch = 'b';
                init_pair(3, COLOR_CYAN, COLOR_YELLOW);
                win.attrset(ColorPair(3));
            }
        }

        main_window.nodelay(false);

        sub_win_test(&main_window, &win).unwrap();

        // Erase and draw green window

        init_pair(4, COLOR_YELLOW, COLOR_GREEN);
        win.bkgd(COLOR_PAIR(4));
        win.attrset(Attribute::Bold);
        win.erase();
        win.refresh();

        // Draw RED bounding box

        win.attrset(ColorPair(2));
        win.draw_box(' ', ' ');
        win.refresh();

        // Display Australia map

        win.attrset(Attribute::Bold);

        for (i, s) in AUS_MAP.into_iter().enumerate() {
            win.mvaddstr(i as i32 + 1, 8, s);
            win.refresh();
            napms(100);
        }

        init_pair(5, COLOR_BLUE, COLOR_WHITE);
        win.attrset(ColorPair(5) | Attribute::Blink);
        win.mvaddstr(height - 2, 3, " pancurses - Linux, Win32");
        win.refresh();

        // Draw running messages

        init_pair(6, COLOR_BLACK, COLOR_WHITE);
        win.attrset(ColorPair(6));
        let w = width - 2;
        win.nodelay(true);

        // jbuhler's re-hacked scrolling messages

        for message in &MESSAGES {
            let msg_len = message.len() as i32;
            let mut visbuf = String::with_capacity(w as usize);
            let mut stop = false;

            for i in (0..w + msg_len).rev() {
                visbuf.clear();
                for visbuf_i in 0..visbuf.capacity() {
                    let i = i - msg_len as i32;
                    let char_index = visbuf_i as i32 - i;
                    let ch = if char_index >= 0 && char_index < message.len() as i32 {
                        let char_index = char_index as usize;
                        match message[char_index..char_index + 1].chars().next() {
                            Some(c) => c,
                            None => ' ',
                        }
                    } else {
                        ' '
                    };
                    visbuf.push(ch);
                }

                win.mvaddstr(height / 2, 1, &visbuf);
                win.refresh();

                if win.getch().is_some() {
                    flushinp();
                    stop = true;
                    break;
                }

                napms(100);
            }

            if stop {
                break;
            }
        }

        // Draw running 'A's across in RED

        init_pair(7, COLOR_RED, COLOR_GREEN);
        win.attron(ColorPair(7));

        let mut save: [chtype; 80] = [0; 80];

        for (index, pos) in (2..width - 4).enumerate() {
            let ch = win.mvinch(5, pos);
            save[index] = ch;
            let ch = ch & 0x7F;
            win.mvaddch(5, pos, ch);
        }

        win.refresh();

        // Put a message up; wait for a key

        let i = height - 2;
        win.attrset(ColorPair(5));
        win.mvaddstr(i, 3, "   Type a key to continue or ESC to quit  ");
        win.refresh();

        if wait_for_user(&main_window) {
            break;
        }

        // Restore the old line

        win.attrset(Attribute::Normal);

        for (index, pos) in (2..width - 4).enumerate() {
            win.mvaddch(5, pos, save[index]);
        }

        win.refresh();

        bouncing_balls(&main_window, &win, &mut rng);

        // bouncing_balls() leaves a keystroke in the queue

        if wait_for_user(&main_window) {
            break;
        }
    }

    endwin();
}

fn wait_for_user(main_window: &Window) -> bool {
    main_window.nodelay(true);
    half_delay(50);

    let ch = main_window.getch();

    main_window.nodelay(false);
    nocbreak(); // Reset the halfdelay() value
    cbreak();

    match ch {
        Some(Input::Character('\x1B')) => true,
        _ => false,
    }
}

fn sub_win_test(main_window: &Window, win: &Window) -> Result<(), i32> {
    win.attrset(Attribute::Normal);
    let (h, w) = win.get_max_yx();
    let (by, bx) = win.get_beg_yx();

    let sw = w / 3;
    let sh = h / 3;

    let swin1 = try!(win.derwin(sh, sw, 3, 5));
    let swin2 = try!(win.subwin(sh, sw, by + 4, bx + 8));
    let swin3 = try!(win.subwin(sh, sw, by + 5, bx + 11));

    init_pair(8, COLOR_RED, COLOR_BLUE);
    swin1.bkgd(COLOR_PAIR(8));
    swin1.erase();
    swin1.mvaddstr(0, 3, "Sub-window 1");
    swin1.refresh();

    init_pair(9, COLOR_CYAN, COLOR_MAGENTA);
    swin2.bkgd(COLOR_PAIR(9));
    swin2.erase();
    swin2.mvaddstr(0, 3, "Sub-window 2");
    swin2.refresh();

    init_pair(10, COLOR_YELLOW, COLOR_GREEN);
    swin3.bkgd(COLOR_PAIR(10));
    swin3.erase();
    swin3.mvaddstr(0, 3, "Sub-window 3");
    swin3.refresh();

    swin1.delwin();
    swin2.delwin();
    swin3.delwin();

    wait_for_user(main_window);

    Ok(())
}

fn bouncing_balls<T: Rng>(main_window: &Window, win: &Window, rng: &mut T) {
    curs_set(0);

    win.bkgd(COLOR_PAIR(1));
    win.refresh();
    win.attrset(Attribute::Normal);

    init_pair(11, COLOR_RED, COLOR_GREEN);
    init_pair(12, COLOR_BLUE, COLOR_RED);
    init_pair(13, COLOR_YELLOW, COLOR_WHITE);

    let ball1 = 'O' as chtype | COLOR_PAIR(11);
    let ball2 = '*' as chtype | COLOR_PAIR(12);
    let ball3 = '@' as chtype | COLOR_PAIR(13);

    let (h, w) = win.get_max_yx();

    let mut x1 = 2 + (rng.gen::<i32>() % (w - 4)).abs();
    let mut y1 = 2 + (rng.gen::<i32>() % (h - 4)).abs();
    let mut x2 = 2 + (rng.gen::<i32>() % (w - 4)).abs();
    let mut y2 = 2 + (rng.gen::<i32>() % (h - 4)).abs();
    let mut x3 = 2 + (rng.gen::<i32>() % (w - 4)).abs();
    let mut y3 = 2 + (rng.gen::<i32>() % (h - 4)).abs();

    let mut xd1 = 1;
    let mut yd1 = 1;
    let mut xd2 = 1;
    let mut yd2 = -1;
    let mut xd3 = -1;
    let mut yd3 = 1;

    main_window.nodelay(true);

    let mut c = main_window.getch();
    while c.is_none() {

        x1 += xd1;
        if x1 <= 1 || x1 >= w - 2 {
            xd1 *= -1;
        }

        y1 += yd1;
        if y1 <= 1 || y1 >= h - 2 {
            yd1 *= -1;
        }

        x2 += xd2;
        if x2 <= 1 || x2 >= w - 2 {
            xd2 *= -1;
        }

        y2 += yd2;
        if y2 <= 1 || y2 >= h - 2 {
            yd2 *= -1;
        }

        x3 += xd3;
        if x3 <= 1 || x3 >= w - 2 {
            xd3 *= -1;
        }

        y3 += yd3;
        if y3 <= 1 || y3 >= h - 2 {
            yd3 *= -1;
        }

        let c1 = win.mvinch(y1, x1);
        let c2 = win.mvinch(y2, x2);
        let c3 = win.mvinch(y3, x3);

        win.mvaddch(y1, x1, ball1);
        win.mvaddch(y2, x2, ball2);
        win.mvaddch(y3, x3, ball3);

        win.mv(0, 0);
        win.refresh();

        win.mvaddch(y1, x1, c1);
        win.mvaddch(y2, x2, c2);
        win.mvaddch(y3, x3, c3);

        napms(150);
        c = main_window.getch();
    }

    main_window.nodelay(false);
    c.map(|c| main_window.ungetch(&c));
}
