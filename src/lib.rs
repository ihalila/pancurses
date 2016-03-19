#[cfg(windows)]
extern crate pdcurses;
#[cfg(unix)]
extern crate ncurses;

use std::ffi::CString;

#[cfg(windows)]
use pdcurses as curses;
#[cfg(windows)]
pub use pdcurses::chtype;
#[cfg(unix)]
use ncurses::ll as curses;
#[cfg(unix)]
pub use ncurses::ll::chtype;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::*;
#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use self::unix::*;

pub const COLOR_BLACK: i16 = 0;
pub const COLOR_RED: i16 = 1;
pub const COLOR_GREEN: i16 = 2;
pub const COLOR_BLUE: i16 = 4;
pub const COLOR_CYAN: i16 = (COLOR_BLUE | COLOR_GREEN);
pub const COLOR_MAGENTA: i16 = (COLOR_RED | COLOR_BLUE);
pub const COLOR_YELLOW: i16 = (COLOR_RED | COLOR_GREEN);
pub const COLOR_WHITE: i16 = 7;

pub const ERR: i32 = -1;

#[derive(Copy, Clone, Debug)]
pub struct Window {
    #[cfg(windows)]
    _window: *mut curses::WINDOW,
    #[cfg(unix)]
    _window: curses::WINDOW,
}

impl Window {
    pub fn getch(&self) -> i32 {
        unsafe { curses::wgetch(self._window) }
    }

    pub fn get_max_x(&self) -> i32 {
        unsafe { curses::getmaxx(self._window) }
    }

    pub fn get_max_y(&self) -> i32 {
        unsafe { curses::getmaxy(self._window) }
    }

    pub fn set_nodelay(&self, enabled: bool) -> i32 {
        unsafe { curses::nodelay(self._window, enabled as u8) as i32 }
    }
}

pub fn attrset(attribute: chtype) -> i32 {
    unsafe {
        curses::attrset(attribute)
    }
}

pub fn endwin() -> i32 {
    unsafe {
        curses::endwin()
    }
}

pub fn erase() -> i32 {
    unsafe {
        curses::erase()
    }
}

pub fn has_colors() -> bool {
    unsafe { curses::has_colors() > 0 }
}

/// Initialize the curses system, this must be the first function that is called
///
/// ```
/// # use pancurses::*;
/// init();
/// # endwin();
/// ```
pub fn init() -> Window {
    let window_pointer = unsafe { curses::initscr() };
    Window { _window: window_pointer }
}

pub fn init_pair(pair_index: i16, foreground_color: i16, background_color: i16) -> i32 {
    unsafe { curses::init_pair(pair_index, foreground_color, background_color) as i32 }
}

pub fn mvaddstr(y: i32, x: i32, string: &str) -> i32 {
    let s = CString::new(string).unwrap();
    unsafe { curses::mvaddstr(y, x, s.as_ptr()) }
}

pub fn napms(ms: i32) -> i32 {
    unsafe {
        curses::napms(ms)
    }
}

pub fn noecho() -> i32 {
    unsafe {
        curses::noecho()
    }
}

pub fn refresh() -> i32 {
    unsafe {
        curses::refresh()
    }
}

pub fn start_color() -> i32 {
    unsafe { curses::start_color() as i32 }
}
