#[macro_use]
extern crate log;

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

mod input;
pub use self::input::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::*;
#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use self::unix::*;

pub const OK: i32 = 0;
pub const ERR: i32 = -1;

#[derive(Debug)]
pub struct Window {
    #[cfg(windows)]
    _window: *mut curses::WINDOW,
    #[cfg(unix)]
    _window: curses::WINDOW,
}

impl Window {
    pub fn attron(&self, attributes: chtype) -> i32 {
        _attron(self._window, attributes)
    }

    /// Sets the current attributes of the given window to attributes.
    pub fn attrset(&self, attributes: chtype) -> i32 {
        _attrset(self._window, attributes)
    }

    /// Not only change the background, but apply it immediately to every cell in the window.
    pub fn bkgd(&self, ch: chtype) -> i32 {
        unsafe { curses::wbkgd(self._window, ch) }
    }

    /// The same as subwin(), except that begy and begx are relative to the origin of the window
    /// rather than the screen.
    ///
    /// There is no difference between subwindows and derived windows.
    pub fn derwin(&self, nlines: i32, ncols: i32, begy: i32, begx: i32) -> Result<Window, i32> {
        self.subwin(nlines,
                    ncols,
                    begy + self.get_beg_y(),
                    begx + self.get_beg_x())
    }

    pub fn draw_box(&self, verch: chtype, horch: chtype) -> i32 {
        _draw_box(self._window, verch, horch)
    }

    /// Copies blanks (i.e. the background chtype) to every cell of the window.
    pub fn erase(&self) -> i32 {
        unsafe { curses::werase(self._window) }
    }

    /// Get the upper-left y coordinate of this window
    pub fn get_beg_y(&self) -> i32 {
        unsafe { curses::getbegy(self._window) }
    }

    // Get the upper-left x coordinate of this window
    pub fn get_beg_x(&self) -> i32 {
        unsafe { curses::getbegx(self._window) }
    }

    /// Get the upper-left y and x coordinates of this window
    pub fn get_beg_yx(&self) -> (i32, i32) {
        (self.get_beg_y(), self.get_beg_x())
    }

    /// Read a character from the terminal associated with the window.
    ///
    /// In nodelay mode, if there is no input waiting, None is returned. In delay mode,
    /// the program will hang until the system  passes text through to the program. Depending on
    /// the setting of cbreak(), this will be after one character or after the first newline.
    /// Unless noecho() has been set, the character will also be echoed into the designated window.
    ///
    /// If keypad() is TRUE, and a function key is pressed, the token for that function key will be
    /// returned instead of the raw characters.
    /// If nodelay(win, TRUE) has been called on the window and no input is waiting, None is
    /// returned.
    pub fn getch(&self) -> Option<Input> {
        let i = unsafe { curses::wgetch(self._window) };
        if i < 0 {
            None
        } else if i <= u8::max_value() as i32 {
            Some(Input::Character(i as u8 as char))
        } else {
            Some(to_special_keycode(i))
        }
    }

    /// Return the maximum x value of this Window, in other words the number of columns.
    pub fn get_max_x(&self) -> i32 {
        unsafe { curses::getmaxx(self._window) }
    }

    /// Return the maximum y value of this Window, in other words the number of rows.
    pub fn get_max_y(&self) -> i32 {
        unsafe { curses::getmaxy(self._window) }
    }

    /// Return the maximum y and x value of this Window
    pub fn get_max_yx(&self) -> (i32, i32) {
        (self.get_max_y(), self.get_max_x())
    }

    /// Controls whether getch() returns function/special keys as single key codes (e.g., the left
    /// arrow key as KEY_LEFT).
    ///
    /// Per X/Open, the default for keypad mode is OFF. You'll probably want it on. With keypad
    /// mode off, if a special key is pressed, getch() does nothing or returns ERR.
    pub fn keypad(&self, use_keypad: bool) -> i32 {
        unsafe { curses::keypad(self._window, use_keypad as u8) }
    }

    /// The cursor associated with the window is moved to the given location.
    ///
    /// This does not move the physical cursor of the terminal until refresh() is called.  The
    /// position specified is relative to the upper left corner of the window, which is (0,0).
    pub fn mv(&self, y: i32, x: i32) -> i32 {
        unsafe { curses::wmove(self._window, y, x) }
    }

    /// moves the cursor to the specified position and adds ch to the specified window
    pub fn mvaddch(&self, y: i32, x: i32, ch: chtype) -> i32 {
        unsafe { curses::mvwaddch(self._window, y, x, ch) }
    }

    /// Write all the characters of the string str to the given window. The functionality is
    /// similar to calling waddch() once for each character in the string.
    pub fn mvaddstr(&self, y: i32, x: i32, string: &str) -> i32 {
        let s = CString::new(string).unwrap();
        unsafe { curses::mvwaddstr(self._window, y, x, s.as_ptr()) }
    }

    /// Retrieves the character and attribute from the specified window position, in the form of a
    /// chtype.
    pub fn mvinch(&self, y: i32, x: i32) -> chtype {
        unsafe { curses::mvwinch(self._window, y, x) }
    }

    /// Controls whether wgetch() is a non-blocking call. If the option is enabled, and
    /// no input is ready, wgetch() will return ERR. If disabled, wgetch() will hang until input is
    /// ready.
    pub fn nodelay(&self, enabled: bool) -> i32 {
        unsafe { curses::nodelay(self._window, enabled as u8) as i32 }
    }

    ///Add a string to the window at the current cursor position.
    pub fn printw(&self, string: &str) -> i32 {
        let s = CString::new(string).unwrap();
        unsafe { curses::wprintw(self._window, s.as_ptr()) }
    }

    /// Copies the named window to the physical terminal screen, taking into account what
    /// is already there in order to optimize cursor movement.
    ///
    /// This function must be called to get any output on the terminal, as other routines only
    /// manipulate data structures. Unless leaveok() has been enabled, the physical cursor of the
    /// terminal is left at the location of the window's cursor.
    pub fn refresh(&self) -> i32 {
        unsafe { curses::wrefresh(self._window) }
    }

    /// Creates a new subwindow within a window.
    ///
    /// The dimensions of the subwindow are nlines lines and ncols columns. The subwindow is at
    /// position (begy, begx) on the screen. This position is relative to the screen, and not to
    /// the window orig. Changes made to either window will affect both. When using this routine,
    /// you will often need to call touchwin() before calling wrefresh().
    pub fn subwin(&self, nlines: i32, ncols: i32, begy: i32, begx: i32) -> Result<Window, i32> {
        let new_window = unsafe { curses::subwin(self._window, nlines, ncols, begy, begx) };
        if new_window.is_null() {
            Err(ERR)
        } else {
            Ok(Window { _window: new_window })
        }
    }

    /// Set blocking or non-blocking reads for the specified window.
    ///
    /// The delay is measured in milliseconds. If it's negative, a blocking read is used; if zero,
    /// then non-blocking reads are done -- if no input is waiting, ERR is returned immediately.
    /// If the delay is positive, the read blocks for the delay period; if the period expires,
    /// ERR is returned.
    pub fn timeout(&self, milliseconds: i32) {
        unsafe { curses::wtimeout(self._window, milliseconds) }
    }

    /// Places ch back onto the input queue to be returned by the next call to getch().
    pub fn ungetch(&self, input: &Input) -> i32 {
        _ungetch(input)
    }
}

/// Set cbreak mode.
///
/// In cbreak mode, characters typed by the user are made available immediately, and erase/kill
/// character processing is not performed.  In nocbreak mode, typed characters are buffered until
/// a newline or carriage return. Interrupt and flow control characters are unaffected by this
/// mode.
pub fn cbreak() -> i32 {
    unsafe { curses::cbreak() }
}

/// Alters the appearance of the cursor.
///
///  A visibility of 0 makes it disappear; 1 makes it appear "normal" (usually an underline) and 2
/// makes it "highly visible" (usually a block).
pub fn curs_set(visibility: i32) -> i32 {
    unsafe { curses::curs_set(visibility) }
}

/// Deletes the window, freeing all associated memory. In the case of overlapping windows,
/// subwindows should be deleted before the main window.
pub fn delwin(window: Window) -> i32 {
    unsafe { curses::delwin(window._window) }
}

/// Should be called before exiting or escaping from curses mode temporarily.
///
/// It will restore tty modes, move the cursor to the lower left corner of the screen and reset the
/// terminal into the proper non-visual mode.  To resume curses after a temporary escape, call
/// refresh() or doupdate().
pub fn endwin() -> i32 {
    unsafe { curses::endwin() }
}

/// Throws away any type-ahead that has been typed by the user and has not yet been read by the
/// program.
pub fn flushinp() -> i32 {
    unsafe { curses::flushinp() }
}

/// Similar to cbreak(), but allows for a time limit to be specified, in tenths of a second.
///
/// This causes getch() to block for that period before returning None if no key has been received.
/// tenths must be between 1 and 255.
pub fn half_delay(tenths: i32) -> i32 {
    unsafe { curses::halfdelay(tenths) }
}

/// Indicates if the terminal supports, and can maniplulate color.
pub fn has_colors() -> bool {
    unsafe { curses::has_colors() > 0 }
}

/// Initialize the curses system, this must be the first function that is called.
///
/// Returns a Window struct that is used to access Window specific functions.
pub fn initscr() -> Window {
    let window_pointer = unsafe { curses::initscr() };
    Window { _window: window_pointer }
}

/// Changes the definition of a color-pair.
///
/// It takes three arguments: the number of the color-pair to be redefined, and the new values of
/// the foreground and background colors. The pair number must be between 0 and COLOR_PAIRS - 1,
/// inclusive. The foreground and background must be between 0 and COLORS - 1, inclusive. If the
/// color pair was previously initialized, the screen is refreshed, and all occurrences of that
/// color-pair are changed to the new definition.
pub fn init_pair(pair_index: i16, foreground_color: i16, background_color: i16) -> i32 {
    unsafe { curses::init_pair(pair_index, foreground_color, background_color) as i32 }
}

/// Suspends the program for the specified number of milliseconds.
pub fn napms(ms: i32) -> i32 {
    unsafe { curses::napms(ms) }
}

/// Creates a new window with the given number of lines, nlines and columns, ncols.
///
/// The upper left corner of the window is at line begy, column begx. If nlines is zero, it
/// defaults to LINES - begy; ncols to COLS - begx. Create a new full-screen window by calling
/// newwin(0, 0, 0, 0).
pub fn newwin(nlines: i32, ncols: i32, begy: i32, begx: i32) -> Window {
    let window_pointer = unsafe { curses::newwin(nlines, ncols, begy, begx) };
    Window { _window: window_pointer }
}

/// Enables the translation of a carriage return into a newline on input.
///
/// nonl() disables this. Initially, the translation does occur.
pub fn nl() -> i32 {
    unsafe { curses::nl() }
}

/// Set nocbreak mode.
///
/// In cbreak mode, characters typed by the user are made available immediately, and erase/kill
/// character processing is not performed.  In nocbreak mode, typed characters are buffered until
/// a newline or carriage return. Interrupt and flow control characters are unaffected by this
/// mode.
pub fn nocbreak() -> i32 {
    unsafe { curses::nocbreak() }
}

/// Disables echoing typed characters.
///
/// Initially, input characters are echoed. Subsequent calls to echo() and noecho() do not flush
/// type-ahead.
pub fn noecho() -> i32 {
    unsafe { curses::noecho() }
}

/// Attempts to resize the screen to the given size.
///
/// resize_term() is effectively two functions: When called with nonzero values for nlines and
/// ncols, it attempts to resize the screen to the given size. When called with (0, 0), it merely
/// adjusts the internal structures to match the current size after the screen is resized by the
/// user. If you want to support user resizing, you should check for getch() returning KEY_RESIZE,
/// and/or call is_termresized() at appropriate times; if either condition occurs, call
/// resize_term(0, 0). Then, with either user or programmatic resizing, you'll have to resize any
/// windows you've created.
pub fn resize_term(nlines: i32, ncols: i32) -> i32 {
    _resize_term(nlines, ncols)
}

/// Initializes eight basic colors (black, red, green, yellow, blue, magenta, cyan,
/// and white), and two global variables; COLORS and COLOR_PAIRS (respectively defining the
/// maximum number of colors and color-pairs the terminal is capable of displaying).
pub fn start_color() -> i32 {
    unsafe { curses::start_color() as i32 }
}

/// Allows the use of -1 as a foreground or background color with init_pair().
///
/// Calls assume_default_colors(-1, -1); -1 represents the foreground or background color that
/// the terminal had at startup.
pub fn use_default_colors() -> i32 {
    unsafe { curses::use_default_colors() }
}
