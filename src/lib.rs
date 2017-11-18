#![allow(non_camel_case_types, non_snake_case)]

#[macro_use]
#[cfg(any(feature = "win32a", all(not(feature = "win32"), not(feature = "win32a"))))]
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
pub use pdcurses::{chtype, mmask_t, MEVENT, SCREEN};
#[cfg(windows)]
type ScrPtr = *mut SCREEN;
#[cfg(windows)]
type FILE = *mut curses::FILE;

#[cfg(unix)]
use ncurses::ll as curses;
#[cfg(unix)]
pub use ncurses::ll::{chtype, mmask_t, MEVENT, SCREEN};
#[cfg(unix)]
type ScrPtr = SCREEN;
#[cfg(unix)]
type FILE = curses::FILE_p;

mod input;
pub use self::input::*;

mod attributes;
pub use self::attributes::*;

pub mod colorpair;
pub use colorpair::ColorPair;

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

mod window;
pub use window::Window;

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

/// Return the output speed of the terminal. On Windows it simply returns `INT_MAX`
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

/// Maximum number of colors the terminal is capable of displaying.
pub fn COLORS() -> i32 {
    platform_specific::_COLORS()
}

/// Maximum number of color-pairs the terminal is capable of displaying.
pub fn COLOR_PAIRS() -> i32 {
    platform_specific::_COLOR_PAIRS()
}

/// This routine gives programmers a way to find the intensity of the red, green, and blue (RGB)
/// components in a color. It takes the color number as an argument and returns three values
/// that tell you the amounts of red, green, and blue components in the given color. The argument
/// must be a legal color value, i.e., 0 through COLORS()-1, inclusive. The values that are returned
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
/// A visibility of 0 makes it disappear; 1 makes it appear "normal" (usually an underline) and 2
/// makes it "highly visible" (usually a block).
pub fn curs_set(visibility: i32) -> i32 {
    unsafe { curses::curs_set(visibility) }
}

/// Save the current terminal modes as the "program" (in curses) state for use by the
/// `reset_prog_mode()` and `reset_shell_mode()` functions.  This is done automatically by initscr().
pub fn def_prog_mode() -> i32 {
    unsafe { curses::def_prog_mode() }
}

/// Save the current terminal modes as the "shell" (not in curses) state for use by the
/// `reset_prog_mode()` and `reset_shell_mode()` functions.  This is done automatically by initscr().
pub fn def_shell_mode() -> i32 {
    unsafe { curses::def_shell_mode() }
}

/// Inserts an 'milliseconds' millisecond pause in output. This routine should not be used extensively
/// because padding characters are used rather than a CPU pause. If no padding character is
/// specified, this uses napms to perform the delay.
pub fn delay_output(milliseconds: i32) -> i32 {
    unsafe { curses::delay_output(milliseconds) }
}

/// Frees storage associated with the SCREEN data structure.
///
/// The endwin routine does not do this, so delscreen should be called after endwin if a particular
/// SCREEN is no longer needed.
///
/// In PDCurses, the parameter must be the value of SP, and delscreen() sets SP to NULL.
pub fn delscreen(screen: ScrPtr) {
    unsafe { curses::delscreen(screen) }
}

/// Compares the virtual screen to the physical screen and performs an update of the physical
/// screen.
pub fn doupdate() -> i32 {
    unsafe { curses::doupdate() }
}

/// Enabled echoing typed characters.
///
/// Initially, input characters are echoed. Subsequent calls to echo() and noecho() do not flush
/// type-ahead.
pub fn echo() -> i32 {
    unsafe { curses::echo() }
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

/// Returns the current mouse status in an MEVENT struct.
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
    window::new_window(window_pointer, true)
}

/// Changes the definition of a color. It takes four arguments: the number of the color to be
/// changed followed by three RGB values (for the amounts of red, green, and blue components).
/// The first argument must be a legal color value; default colors are not allowed here.
/// Each of the last three arguments must be a value in the range 0 through 1000. When `init_color`
/// is used, all occurrences of that color on the screen immediately change to the new definition.
pub fn init_color(color_number: i16, red: i16, green: i16, blue: i16) -> i32 {
    unsafe { curses::init_color(color_number, red, green, blue) }
}

/// Changes the definition of a color-pair.
///
/// It takes three arguments: the number of the color-pair to be redefined, and the new values of
/// the foreground and background colors. The pair number must be between 0 and `COLOR_PAIRS` - 1,
/// inclusive. The foreground and background must be between 0 and `COLORS()` - 1, inclusive. If the
/// color pair was previously initialized, the screen is refreshed, and all occurrences of that
/// color-pair are changed to the new definition.
pub fn init_pair(pair_index: i16, foreground_color: i16, background_color: i16) -> i32 {
    unsafe { curses::init_pair(pair_index, foreground_color, background_color) as i32 }
}

/// Sets the timeout for a mouse click.
///
/// Sets the maximum time (in thousands of a second) that can elapse between press and release
/// events for them to be recognized as aclick. Use mouseinterval(0) to disable click resolution.
/// This function returns the previous interval value. Use mouseinterval(-1) to obtain the interval
/// without altering it. The default is one sixth of a second.
pub fn mouseinterval(interval: i32) -> i32 {
    unsafe { curses::mouseinterval(interval) }
}

/// Set the mouse events to be reported.
///
/// By default, no mouse events are reported. The function will return a mask to indicate which of
/// the specified mouse events can be reported; on complete failure it returns 0. If oldmask is
/// non-NULL, this function fills the indicated location with the previous value of the given
/// window's mouse event mask.
///
/// As a side effect, setting a zero mousemask may turn off the mouse pointer; setting a nonzero
/// mask may turn it on. Whether this happens is device-dependent.
pub fn mousemask(newmask: mmask_t, oldmask: *mut mmask_t) -> mmask_t {
    unsafe { curses::mousemask(newmask, oldmask) }
}

/// Suspends the program for the specified number of milliseconds.
pub fn napms(ms: i32) -> i32 {
    unsafe { curses::napms(ms) }
}

/// A program that outputs to more than one terminal should use the newterm routine for each
/// terminal instead of initscr.
///
/// A program that needs to inspect capabilities, so it can continue to
/// run in a line-oriented mode if the terminal cannot support a screen-oriented program, would also
/// use newterm. The routine newterm should be called once for each terminal. It returns a variable
/// of type ScrPtr which should be saved as a reference to that terminal.
///
/// (For the PDCurses backend it's just an alternative interface for initscr(). It always returns
/// SP, or NULL.)
pub fn newterm(t: Option<&str>, output: FILE, input: FILE) -> ScrPtr {
    unsafe {
        curses::newterm(
            t.map(|x| CString::new(x).unwrap().as_ptr()).unwrap_or(
                std::ptr::null(),
            ),
            output,
            input,
        )
    }
}

/// Creates a new window with the given number of lines, nlines and columns, ncols.
///
/// The upper left corner of the window is at line begy, column begx. If nlines is zero, it
/// defaults to LINES - begy; ncols to COLS - begx. Create a new full-screen window by calling
/// newwin(0, 0, 0, 0).
pub fn newwin(nlines: i32, ncols: i32, begy: i32, begx: i32) -> Window {
    let window_pointer = unsafe { curses::newwin(nlines, ncols, begy, begx) };
    window::new_window(window_pointer, false)
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

/// Disables the translation of a carriage return into a newline on input.
/// 
/// nl() enables this. Initially, the translation does occur.
pub fn nonl() -> i32 {
    unsafe { curses::nonl() }
}

/// Restore the terminal to "program" (in curses) state. This is done
/// automatically by endwin() and doupdate() after an endwin(), so this would normally not be
/// called before.
pub fn reset_prog_mode() -> i32 {
    unsafe { curses::reset_prog_mode() }
}

/// Restore the terminal to "shell" (not in curses) state. This is done automatically by
/// endwin() and doupdate() after an endwin(), so this would normally not be called before.
pub fn reset_shell_mode() -> i32 {
    unsafe { curses::reset_shell_mode() }
}

/// Attempts to resize the screen to the given size.
///
/// `resize_term()` is effectively two functions: When called with nonzero values for nlines and
/// ncols, it attempts to resize the screen to the given size. When called with (0, 0), it merely
/// adjusts the internal structures to match the current size after the screen is resized by the
/// user. If you want to support user resizing, you should check for getch() returning `KEY_RESIZE`,
/// and/or call `is_termresized()` at appropriate times; if either condition occurs, call
/// `resize_term(0, 0)`. Then, with either user or programmatic resizing, you'll have to resize any
/// windows you've created.
pub fn resize_term(nlines: i32, ncols: i32) -> i32 {
    platform_specific::_resize_term(nlines, ncols)
}

/// Toggles whether the `A_BLINK` attribute sets an actual blink mode (TRUE), or sets the background
/// color to high intensity (FALSE).
///
/// The default is platform-dependent (FALSE in most cases). It returns OK if it could set the
/// state to match the given parameter, ERR otherwise. Current platforms also adjust the value
/// of COLORS() according to this function -- 16 for FALSE, and 8 for TRUE.
/// (Only supported on Windows)
pub fn set_blink(enabled: bool) -> i32 {
    platform_specific::_set_blink(enabled)
}

/// Switches between different terminals.
///
/// The screen reference new becomes the new current terminal. The previous terminal is returned by
/// the routine. This is the only routine which manipulates ScrPtr's; all other routines  affect
/// only the current terminal.
///
/// (Does nothing meaningful in PDCurses, but is included for compatibility with other curses
/// implementations.)
pub fn set_term(new: ScrPtr) -> ScrPtr {
    unsafe { curses::set_term(new) }
}

/// Sets the title of the window in which the curses program is running. This function may not do
/// anything on some platforms. (Only supported on Windows)
pub fn set_title(title: &str) {
    platform_specific::_set_title(title);
}

/// Initializes eight basic colors (black, red, green, yellow, blue, magenta, cyan,
/// and white), and two global variables accessed through `COLORS()` and `COLOR_PAIRS()` (respectively defining the
/// maximum number of colors and color-pairs the terminal is capable of displaying).
pub fn start_color() -> i32 {
    unsafe { curses::start_color() as i32 }
}

/// Allows the use of -1 as a foreground or background color with `init_pair()`.
///
/// Calls `assume_default_colors(-1, -1);` -1 represents the foreground or background color that
/// the terminal had at startup.
pub fn use_default_colors() -> i32 {
    unsafe { curses::use_default_colors() }
}
