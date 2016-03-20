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
    /// attrset() sets the current attributes of the given window to attributes.
    pub fn attrset(&self, attributes: chtype) -> i32 {
        unsafe { curses::wattrset(self._window, attributes) }
    }

    /// erase() copies blanks (i.e. the background chtype) to every cell of the window.
    pub fn erase(&self) -> i32 {
        unsafe { curses::werase(self._window) }
    }

    /// With the getch(), wgetch(), mvgetch(), and mvwgetch() functions, a character is read from
    /// the terminal associated with the window. In nodelay mode, if there is no input waiting, the
    /// value ERR is returned. In delay mode, the program will hang until the system  passes text
    /// through to the program. Depending on the setting of cbreak(), this will be after one
    /// character or after the first newline.  Unless noecho() has been set, the character will
    /// also be echoed into the designated window.
    ///
    /// If keypad() is TRUE, and a function key is pressed, the token for that function key will be
    /// returned instead of the raw characters.
    /// If nodelay(win, TRUE) has been called on the window and no input is waiting, the value ERR
    /// is returned.
    pub fn getch(&self) -> i32 {
        unsafe { curses::wgetch(self._window) }
    }

    /// Return the maximum x value of this Window, in other words the number of columns.
    pub fn get_max_x(&self) -> i32 {
        unsafe { curses::getmaxx(self._window) }
    }

    /// Return the maximum y value of this Window, in other words the number of rows.
    pub fn get_max_y(&self) -> i32 {
        unsafe { curses::getmaxy(self._window) }
    }

    /// The cursor associated with the window is moved to the given location.  This does not move
    /// the physical cursor of the terminal until refresh() is called.  The position specified is
    /// relative to the upper left corner of the window, which is (0,0).
    pub fn mv(&self, y: i32, x: i32) -> i32 {
        unsafe { curses::wmove(self._window, y, x) }
    }

    /// Write all the characters of the string str  to the given window. The functionality is
    /// similar to calling waddch() once for each character in the string.
    pub fn mvaddstr(&self, y: i32, x: i32, string: &str) -> i32 {
        let s = CString::new(string).unwrap();
        unsafe { curses::mvwaddstr(self._window, y, x, s.as_ptr()) }
    }

    ///The printw() functions add a string to the window at the current cursor position.
    pub fn printw(&self, string: &str) -> i32 {
        let s = CString::new(string).unwrap();
        unsafe { curses::wprintw(self._window, s.as_ptr()) }
    }

    /// refresh() copies the named window to the physical terminal screen, taking into account what
    /// is already there in order to optimize cursor movement. These routines must be called to get
    /// any output on the terminal, as other routines only manipulate data structures. Unless
    /// leaveok() has been enabled, the physical cursor of the terminal is left at the location of
    /// the window's cursor.
    pub fn refresh(&self) -> i32 {
        unsafe { curses::wrefresh(self._window) }
    }

    /// nodelay() controls whether wgetch() is a non-blocking call. If the option is enabled, and
    /// no input is ready, wgetch() will return ERR. If disabled, wgetch() will hang until input is
    /// ready.
    pub fn set_nodelay(&self, enabled: bool) -> i32 {
        unsafe { curses::nodelay(self._window, enabled as u8) as i32 }
    }
}


/// endwin() should be called before exiting or escaping from curses mode temporarily.  It will
/// restore tty modes, move the cursor to the lower left corner of the screen and reset the
/// terminal into the proper non-visual mode.  To resume curses after a temporary escape, call
/// refresh() or doupdate().
pub fn endwin() -> i32 {
    unsafe { curses::endwin() }
}

/// has_colors() indicates if the terminal supports, and can maniplulate color.
pub fn has_colors() -> bool {
    unsafe { curses::has_colors() > 0 }
}

/// Initialize the curses system, this must be the first function that is called.
/// Returns a Window struct that is used to access Window specific functions.
pub fn initscr() -> Window {
    let window_pointer = unsafe { curses::initscr() };
    Window { _window: window_pointer }
}

/// init_pair() changes the definition of a color-pair. It takes three arguments: the number of the
/// color-pair to be redefined, and the new values of the foreground and background colors. The
/// pair number must be between 0 and COLOR_PAIRS - 1, inclusive. The foreground and background
/// must be between 0 and COLORS - 1, inclusive. If the color pair was previously initialized, the
/// screen is refreshed, and all occurrences of that color-pair are changed to the new definition.
pub fn init_pair(pair_index: i16, foreground_color: i16, background_color: i16) -> i32 {
    unsafe { curses::init_pair(pair_index, foreground_color, background_color) as i32 }
}

/// napms() suspends the program for the specified number of milliseconds.
pub fn napms(ms: i32) -> i32 {
    unsafe { curses::napms(ms) }
}

/// echo() and noecho() control whether typed characters are echoed by the input routine.
/// Initially, input characters are echoed. Subsequent calls to echo() and noecho() do not flush
/// type-ahead.
pub fn noecho() -> i32 {
    unsafe { curses::noecho() }
}

/// start_color() initializes eight basic colors (black, red, green, yellow, blue, magenta, cyan,
/// and white), and two global variables; COLORS and COLOR_PAIRS (respectively defining the
/// maximum number of colors and color-pairs the terminal is capable of displaying).
pub fn start_color() -> i32 {
    unsafe { curses::start_color() as i32 }
}
