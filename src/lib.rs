#[macro_use]
extern crate log;
extern crate libc;

#[cfg(windows)]
extern crate pdcurses;
#[cfg(unix)]
extern crate ncurses;

use std::ffi::CString;
use std::ptr;

#[cfg(windows)]
use pdcurses as curses;
#[cfg(windows)]
pub use pdcurses::{chtype, mmask_t, MEVENT};
#[cfg(unix)]
use ncurses::ll as curses;
#[cfg(unix)]
pub use ncurses::ll::{chtype, mmask_t, MEVENT};

mod input;
pub use self::input::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::constants::*;
#[cfg(windows)]
use self::windows as platform_specific;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use self::unix::constants::*;
#[cfg(unix)]
use self::unix as platform_specific;

pub const OK: i32 = 0;
pub const ERR: i32 = -1;

pub trait ToChtype {
    fn to_chtype(&self) -> chtype;
}

impl ToChtype for char {
    fn to_chtype(&self) -> chtype {
        *self as chtype
    }
}

impl ToChtype for chtype {
    fn to_chtype(&self) -> chtype {
        *self
    }
}

#[derive(Debug)]
pub struct Window {
    #[cfg(windows)]
    _window: *mut curses::WINDOW,
    #[cfg(unix)]
    _window: curses::WINDOW,
}

impl Window {
    /// Adds the chtype ch to the window at the current cursor position, and advances the cursor.
    ///
    /// Note that chtypes can convey both text (a single character) and attributes, including a
    /// color pair.
    pub fn addch<T: ToChtype>(&self, ch: T) -> i32 {
        unsafe { curses::waddch(self._window, ch.to_chtype()) }
    }

    /// Write all the characters of the string to the given window.
    ///
    /// The functionality is similar to calling window.addch() once for each character in the
    /// string.
    pub fn addstr(&self, string: &str) -> i32 {
        let s = CString::new(string).unwrap();
        unsafe { curses::waddstr(self._window, s.as_ptr()) }
    }

    /// Write at most length characters; if length is negative, then the entire string will be
    /// added.
    pub fn addnstr(&self, string: &str, length: usize) -> i32 {
        let s = CString::new(string).unwrap();
        unsafe { curses::waddnstr(self._window, s.as_ptr(), length as i32) }
    }

    /// Retrieve attributes for the given window.
    ///
    /// ```rust
    /// use pancurses::{A_BOLD, initscr, endwin};
    /// let window = initscr();
    /// window.attron(A_BOLD);
    /// let (active_attributes, color_pair) = window.attrget();
    /// assert_eq!(A_BOLD, active_attributes);
    /// endwin();
    /// ```
    pub fn attrget(&self) -> (chtype, i16) {
        let mut attributes: chtype = 0;
        let mut color_pair: i16 = 0;
        unsafe {
            curses::wattr_get(self._window,
                              &mut attributes,
                              &mut color_pair,
                              ptr::null_mut());
        }
        (attributes, color_pair)
    }

    /// Turns off the named attributes without affecting any other attributes.
    pub fn attroff(&self, attributes: chtype) -> i32 {
        platform_specific::_attroff(self._window, attributes)
    }

    /// Turns on the named attributes without affecting any other attributes.
    pub fn attron(&self, attributes: chtype) -> i32 {
        platform_specific::_attron(self._window, attributes)
    }

    /// Sets the current attributes of the given window to attributes.
    pub fn attrset(&self, attributes: chtype) -> i32 {
        platform_specific::_attrset(self._window, attributes)
    }

    /// Not only change the background, but apply it immediately to every cell in the window.
    pub fn bkgd(&self, ch: chtype) -> i32 {
        unsafe { curses::wbkgd(self._window, ch) }
    }

    /// Manipulate the background of a window. The background is a chtype consisting of any
    /// combination of attributes and a character; it is combined with each chtype added or
    /// inserted to the window by addch() or insch(). Only the attribute part is used to set
    /// the background of non-blank characters, while both character and attributes are used
    /// for blank positions.
    pub fn bgkdset(&self, ch: chtype) {
        unsafe { curses::wbkgdset(self._window, ch) }
    }

    /// Draw a border around the edges of the window.
    pub fn border<T: ToChtype>(&self,
                               left_side: T,
                               right_side: T,
                               top_side: T,
                               bottom_side: T,
                               top_left_corner: T,
                               top_right_corner: T,
                               bottom_left_corner: T,
                               bottom_right_corner: T)
                               -> i32 {
        unsafe {
            curses::wborder(self._window,
                            left_side.to_chtype(),
                            right_side.to_chtype(),
                            top_side.to_chtype(),
                            bottom_side.to_chtype(),
                            top_left_corner.to_chtype(),
                            top_right_corner.to_chtype(),
                            bottom_left_corner.to_chtype(),
                            bottom_right_corner.to_chtype())
        }
    }

    /// Changes the attributes of a given number of characters starting at the current cursor
    /// location. It does not update the cursor and does not perform wrapping. A character count
    /// of -1 or greater than the remaining window width means to change attributes all the way
    /// to the end of the current line.
    pub fn chgat(&self, n: i32, attributes: chtype, color_pair: i16) -> i32 {
        unsafe { curses::wchgat(self._window, n, attributes, color_pair, ptr::null_mut()) }
    }

    /// Similar to erase(), but also calls clearok() to ensure that the the window is cleared on
    /// the next refresh().
    pub fn clear(&self) -> i32 {
        unsafe { curses::wclear(self._window) }
    }

    /// With clearok(), if bf is TRUE, the next call to refresh() with
    /// this window will clear the screen completely and redraw the
    /// entire screen.
    pub fn clearok(&self, bf: bool) -> i32 {
        unsafe { curses::clearok(self._window, bf as u8) }
    }

    /// Clear the window from the current cursor position to the end of the window.
    pub fn clrtobot(&self) -> i32 {
        unsafe { curses::wclrtobot(self._window) }
    }

    /// Clear the window from the current cursor position to the end of the current line.
    pub fn clrtoeol(&self) -> i32 {
        unsafe { curses::wclrtoeol(self._window) }
    }

    /// Sets the current color of the given window to the foreground/background combination
    /// described by the color pair parameter.
    pub fn color_set(&self, color_pair: i16) -> i32 {
        unsafe { curses::wcolor_set(self._window, color_pair, ptr::null_mut()) }
    }

    /// Copy all text from this window to the destination window. The arguments src_tc and
    /// src_tr specify the top left corner of the region to be copied. dst_tc, dst_tr, dst_br,
    /// and dst_bc specify the region within the destination window to copy to. The argument
    /// "overlay", if TRUE, indicates that the copy is done non-destructively (as in overlay());
    /// blanks in the source window are not copied to the destination window. When overlay is
    /// FALSE, blanks are copied.
    pub fn copywin(&self,
                   destination_window: &Window,
                   src_tr: i32,
                   src_tc: i32,
                   dst_tr: i32,
                   dst_tc: i32,
                   dst_br: i32,
                   dst_bc: i32,
                   overlay: bool)
                   -> i32 {
        unsafe {
            curses::copywin(self._window,
                            destination_window._window,
                            src_tr,
                            src_tc,
                            dst_tr,
                            dst_tc,
                            dst_br,
                            dst_bc,
                            overlay as i32)
        }
    }

    /// Delete the character under the cursor. All characters to the right of the cursor
    /// on the same line are moved to the left one position and hte last character on the
    /// line is filled with a blank. The cursor position does not change.
    pub fn delch(&self) -> i32 {
        unsafe { curses::wdelch(self._window) }
    }

    /// Delete the line under the cursor. All lines below are moved up one line, and the
    /// bottom line is cleared. The cursor position does not change.
    pub fn deleteln(&self) -> i32 {
        unsafe { curses::wdeleteln(self._window) }
    }

    /// Deletes the window, freeing all associated memory. In the case of overlapping windows,
    /// subwindows should be deleted before the main window.
    pub fn delwin(self) -> i32 {
        unsafe { curses::delwin(self._window) }
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

    /// Draw a border around the edge of the window. If any argument is zero, an appropriate
    /// default is used.
    pub fn draw_box<T: ToChtype>(&self, verch: T, horch: T) -> i32 {
        platform_specific::_draw_box(self._window, verch.to_chtype(), horch.to_chtype())
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

    /// Returns the given window's current background character and attributes.
    pub fn getbkgd(&self) -> chtype {
        unsafe { curses::getbkgd(self._window) }
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
            Some(platform_specific::to_special_keycode(i))
        }
    }

    /// Return the current x coordinate of the cursor
    pub fn get_cur_x(&self) -> i32 {
        unsafe { curses::getcurx(self._window) }
    }

    /// Return the current y coordinate of the cursor
    pub fn get_cur_y(&self) -> i32 {
        unsafe { curses::getcury(self._window) }
    }

    /// Return the current y and x coordinates of the cursor
    pub fn get_cur_yx(&self) -> (i32, i32) {
        (self.get_cur_y(), self.get_cur_x())
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

    /// Draw a horizontal line using ch from the current cursor position. The line is at most
    /// n characters long, or as many as fit into the window.
    pub fn hline<T: ToChtype>(&self, ch: T, n: i32) -> i32 {
        unsafe { curses::whline(self._window, ch.to_chtype(), n) }
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
    pub fn mvaddch<T: ToChtype>(&self, y: i32, x: i32, ch: T) -> i32 {
        unsafe { curses::mvwaddch(self._window, y, x, ch.to_chtype()) }
    }

    /// Write all the characters of the string str to the given window. The functionality is
    /// similar to calling waddch() once for each character in the string.
    pub fn mvaddstr(&self, y: i32, x: i32, string: &str) -> i32 {
        let s = CString::new(string).unwrap();
        unsafe { curses::mvwaddstr(self._window, y, x, s.as_ptr()) }
    }

    /// Write the first'n' characters of the string str to the given window.
    pub fn mvaddnstr(&self, y: i32, x: i32, string: &str, n: i32) -> i32 {
        let s = CString::new(string).unwrap();
        unsafe { curses::mvwaddnstr(self._window, y, x, s.as_ptr(), n) }
    }

    /// Moves the cursor and changes the attributes of a given number of characters starting at the
    /// cursor location. It does not update the cursor and does not perform wrapping. A character count
    /// of -1 or greater than the remaining window width means to change attributes all the way
    /// to the end of the current line.
    pub fn mvchgat(&self, y: i32, x: i32, n: i32, attributes: chtype, color_pair: i16) -> i32 {
        unsafe {
            curses::mvwchgat(self._window,
                             y,
                             x,
                             n,
                             attributes,
                             color_pair,
                             ptr::null_mut())
        }
    }

    /// Retrieves the character and attribute from the specified window position, in the form of a
    /// chtype.
    pub fn mvinch(&self, y: i32, x: i32) -> chtype {
        unsafe { curses::mvwinch(self._window, y, x) }
    }

    /// Add a string to the window at the specified cursor position.
    pub fn mvprintw(&self, y: i32, x: i32, string: &str) -> i32 {
        let s = CString::new(string).unwrap();
        unsafe { curses::mvwprintw(self._window, y, x, s.as_ptr()) }
    }

    /// Controls whether wgetch() is a non-blocking call. If the option is enabled, and
    /// no input is ready, wgetch() will return ERR. If disabled, wgetch() will hang until input is
    /// ready.
    pub fn nodelay(&self, enabled: bool) -> i32 {
        unsafe { curses::nodelay(self._window, enabled as u8) as i32 }
    }

    /// Overlays this window on top of destination_window. This window and destination_window are
    /// not required to be the same size; only text where the two windows overlap is copied.
    /// overlay() is non-destructive.
    pub fn overlay(&self, destination_window: &Window) -> i32 {
        unsafe { curses::overlay(self._window, destination_window._window) }
    }

    /// Overlays this window on top of destination_window. This window and destination_window are
    /// not required to be the same size; only text where the two windows overlap is copied.
    /// overwrite() is destructive.
    pub fn overwrite(&self, destination_window: &Window) -> i32 {
        unsafe { curses::overwrite(self._window, destination_window._window) }
    }

    /// Add a string to the window at the current cursor position.
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

    /// If enabled and a scrolling region is set with setscrreg(), any attempt to move off
    /// the bottom margin will cause all lines in the scrolling region to scroll up one line.
    pub fn scrollok(&self, bf: bool) -> i32 {
        unsafe { curses::scrollok(self._window, bf as u8) }
    }

    /// Sets a scrolling region in a window.
    ///
    /// "top" and "bot" are the line numbers for the top and bottom margins.
    pub fn setscrreg(&self, top: i32, bot: i32) -> i32 {
        unsafe { curses::wsetscrreg(self._window, top, bot) }
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
        platform_specific::_ungetch(input)
    }

    /// Draw a vertical line using ch from the current cursor position. The line is at most
    /// n characters long, or as many as fit into the window.
    pub fn vline<T: ToChtype>(&self, ch: T, n: i32) -> i32 {
        unsafe { curses::wvline(self._window, ch.to_chtype(), n) }
    }
}

/// Return the output speed of the terminal. On Windows it simply returns INT_MAX
pub fn baudrate() -> i32 {
    unsafe { curses::baudrate() }
}

/// Sounds the audible bell on the terminal, if possible; if not, it calls flash().
pub fn beep() -> i32 {
    unsafe { curses::beep() }
}

/// Indicates if the terminal has the capability to change the definition of its colors.
pub fn can_change_color() -> bool {
    unsafe { curses::can_change_color() != 0 }
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

/// This routine gives programmers a way to find the intensity of the red, green, and blue (RGB)
/// components in a color. It takes the color number as an argument and returns three values
/// that tell you the amounts of red, green, and blue components in the given color. The argument
/// must be a legal color value, i.e., 0 through COLORS-1, inclusive. The values that are returned
/// are in the range 0 (no component) through 1000 (maximum amount of component), inclusive.
///
/// ```rust
/// use pancurses::{can_change_color, color_content, endwin, init_color, initscr, start_color};
///
/// initscr();
/// start_color();
/// if can_change_color() {
///     init_color(8, 35, 502, 1000);
///     let (r, g, b) = color_content(8);
///     assert_eq!(35, r);
///     assert_eq!(502, g);
///     assert_eq!(1000, b);
/// }
/// endwin();
/// ```
pub fn color_content(color_number: i16) -> (i16, i16, i16) {
    let mut r: i16 = 0;
    let mut g: i16 = 0;
    let mut b: i16 = 0;
    unsafe {
        curses::color_content(color_number, &mut r, &mut g, &mut b);
    }
    (r, g, b)
}

/// Alters the appearance of the cursor.
///
///  A visibility of 0 makes it disappear; 1 makes it appear "normal" (usually an underline) and 2
/// makes it "highly visible" (usually a block).
pub fn curs_set(visibility: i32) -> i32 {
    unsafe { curses::curs_set(visibility) }
}

/// Should be called before exiting or escaping from curses mode temporarily.
///
/// It will restore tty modes, move the cursor to the lower left corner of the screen and reset the
/// terminal into the proper non-visual mode.  To resume curses after a temporary escape, call
/// refresh() or doupdate().
pub fn endwin() -> i32 {
    unsafe { curses::endwin() }
}

/// Flashes the screen, if possible; if not, it calls beep().
pub fn flash() -> i32 {
    unsafe { curses::flash() }
}

/// Throws away any type-ahead that has been typed by the user and has not yet been read by the
/// program.
pub fn flushinp() -> i32 {
    unsafe { curses::flushinp() }
}

pub fn getmouse() -> Result<MEVENT, i32> {
    platform_specific::_getmouse()
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
    platform_specific::pre_init();
    let window_pointer = unsafe { curses::initscr() };
    Window { _window: window_pointer }
}

/// Changes the definition of a color. It takes four arguments: the number of the color to be
/// changed followed by three RGB values (for the amounts of red, green, and blue components).
/// The first argument must be a legal color value; default colors are not allowed here.
/// Each of the last three arguments must be a value in the range 0 through 1000. When init_color
/// is used, all occurrences of that color on the screen immediately change to the new definition.
pub fn init_color(color_number: i16, red: i16, green: i16, blue: i16) -> i32 {
    unsafe { curses::init_color(color_number, red, green, blue) }
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

/// Nearly equivalent to mouse_set(), but instead of OK/ERR, it returns the value of the mask after
/// setting it.
///
/// (This isn't necessarily the same value passed in, since the mask could be altered on some
/// platforms.) And if the second parameter is a non-null pointer, mousemask() stores the previous
/// mask value there. Also, since the ncurses interface doesn't work with PDCurses' BUTTON_MOVED
/// events, mousemask() filters them out.
pub fn mousemask(arg1: mmask_t, arg2: *mut mmask_t) -> mmask_t {
    unsafe { curses::mousemask(arg1, arg2) }
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
    platform_specific::_resize_term(nlines, ncols)
}

/// Toggles whether the A_BLINK attribute sets an actual blink mode (TRUE), or sets the background
/// color to hig intensity (FALSE).
///
/// The default is platform-dependent (FALSE in most cases). It returns OK if it could set the
/// state to match the given parameter, ERR otherwise. Current platforms also adjust the value
/// of COLORS according to this function -- 16 for FALSE, and 8 for TRUE.
/// (Only supported on Windows)
pub fn set_blink(enabled: bool) -> i32 {
    platform_specific::_set_blink(enabled)
}

/// Sets the title of the window in which the curses program is running. This function may not do
/// anything on some platforms. (Only supported on Windows)
pub fn set_title(title: &str) {
    platform_specific::_set_title(title);
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
