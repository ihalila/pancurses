extern crate pdcurses;

use pdcurses::*;

/// Initialize the curses system, this must be the first function that is called
///
/// ```
/// # use pancurses::initialize;
/// initialize();
/// ```
pub fn initialize() -> *mut WINDOW {
    unsafe { initscr() }
}
