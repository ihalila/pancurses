#[cfg(windows)]
extern crate pdcurses;
#[cfg(unix)]
extern crate ncurses;

#[cfg(windows)]
pub mod windows;
#[cfg(unix)]
pub mod unix;

#[cfg(windows)]
use pdcurses as curses;
#[cfg(unix)]
use ncurses::ll as curses;

#[derive(Copy, Clone, Debug)]
pub struct CursesWindow {
	#[cfg(windows)]
	_window: *mut curses::WINDOW,
	#[cfg(unix)]
	_window: curses::WINDOW
}

impl CursesWindow {
	pub fn set_nodelay(&self, enabled: bool) {
		unsafe { curses::nodelay(self._window, enabled as u8); }
	}
}

pub fn has_colors() -> bool {
	unsafe { curses::has_colors() > 0 }
}

pub fn start_color() {
	unsafe { curses::start_color(); }
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
