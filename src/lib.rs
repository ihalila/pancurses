#[cfg(windows)]
extern crate pdcurses;
#[cfg(unix)]
extern crate ncurses;

use pdcurses as curses;

#[derive(Copy, Clone, Debug)]
pub struct CursesWindow {
	_window: *mut curses::WINDOW
}

impl CursesWindow {
	///Return the current x coordinate of the cursor
	pub fn cursor_x(&self) -> i32 {
		unsafe { (*self._window)._curx }
	}
	
	pub fn set_nodelay(&self, enabled: bool) {
		unsafe { curses::nodelay(self._window, if enabled { 1u8 } else { 0u8 }); }
	}
}

/// Initialize the curses system, this must be the first function that is called
///
/// ```
/// # use pancurses::*;
/// initialize();
/// # end();
/// ```
pub fn initialize() -> CursesWindow {
    let window_pointer = unsafe { curses::initscr() };
    CursesWindow { _window : window_pointer }
}

pub fn end() {
	unsafe { curses::endwin(); }
}

pub fn noecho() {
	unsafe { curses::noecho(); }
}

#[test]
pub fn test_cursor_position() {
	let window = initialize();
	assert!(window.cursor_x() == 0);
	end();
}