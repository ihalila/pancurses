use {chtype, curses, ERR, Input, platform_specific, ptr, ToChtype};
use std::ffi::CString;

#[derive(Debug)]
pub struct Window {
    #[cfg(windows)]
    _window: *mut curses::WINDOW,
    #[cfg(unix)]
    _window: curses::WINDOW,
    _stdscr: bool,
    _deleted: bool,
}

#[cfg(windows)]
type WindowPointer = *mut curses::WINDOW;
#[cfg(unix)]
type WindowPointer = curses::WINDOW;

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
            curses::wattr_get(
                self._window,
                &mut attributes,
                &mut color_pair,
                ptr::null_mut(),
            );
        }
        (attributes, color_pair)
    }

    /// Turns off the named attributes without affecting any other attributes.
    pub fn attroff<T: Into<chtype>>(&self, attributes: T) -> i32 {
        platform_specific::_attroff(self._window, attributes.into())
    }

    /// Turns on the named attributes without affecting any other attributes.
    pub fn attron<T: Into<chtype>>(&self, attributes: T) -> i32 {
        platform_specific::_attron(self._window, attributes.into())
    }

    /// Sets the current attributes of the given window to attributes.
    pub fn attrset<T: Into<chtype>>(&self, attributes: T) -> i32 {
        platform_specific::_attrset(self._window, attributes.into())
    }

    /// Not only change the background, but apply it immediately to every cell in the window.
    pub fn bkgd<T: Into<chtype>>(&self, ch: T) -> i32 {
        unsafe { curses::wbkgd(self._window, ch.into()) }
    }

    /// Manipulate the background of a window. The background is a chtype consisting of any
    /// combination of attributes and a character; it is combined with each chtype added or
    /// inserted to the window by addch() or insch(). Only the attribute part is used to set
    /// the background of non-blank characters, while both character and attributes are used
    /// for blank positions.
    pub fn bgkdset<T: Into<chtype>>(&self, ch: T) {
        unsafe { curses::wbkgdset(self._window, ch.into()) }
    }

    /// Draw a border around the edges of the window.
    pub fn border<T: ToChtype>(
        &self,
        left_side: T,
        right_side: T,
        top_side: T,
        bottom_side: T,
        top_left_corner: T,
        top_right_corner: T,
        bottom_left_corner: T,
        bottom_right_corner: T,
    ) -> i32 {
        unsafe {
            curses::wborder(
                self._window,
                left_side.to_chtype(),
                right_side.to_chtype(),
                top_side.to_chtype(),
                bottom_side.to_chtype(),
                top_left_corner.to_chtype(),
                top_right_corner.to_chtype(),
                bottom_left_corner.to_chtype(),
                bottom_right_corner.to_chtype(),
            )
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
    pub fn copywin(
        &self,
        destination_window: &Window,
        src_tr: i32,
        src_tc: i32,
        dst_tr: i32,
        dst_tc: i32,
        dst_br: i32,
        dst_bc: i32,
        overlay: bool,
    ) -> i32 {
        unsafe {
            curses::copywin(
                self._window,
                destination_window._window,
                src_tr,
                src_tc,
                dst_tr,
                dst_tc,
                dst_br,
                dst_bc,
                overlay as i32,
            )
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
    pub fn delwin(mut self) -> i32 {
        self._deleted = true;
        unsafe { curses::delwin(self._window) }
    }

    /// The same as subwin(), except that begy and begx are relative to the origin of the window
    /// rather than the screen.
    ///
    /// There is no difference between subwindows and derived windows.
    pub fn derwin(&self, nlines: i32, ncols: i32, begy: i32, begx: i32) -> Result<Window, i32> {
        self.subwin(
            nlines,
            ncols,
            begy + self.get_beg_y(),
            begx + self.get_beg_x(),
        )
    }

    /// Draw a border around the edge of the window. If any argument is zero, an appropriate
    /// default is used.
    pub fn draw_box<T: ToChtype>(&self, verch: T, horch: T) -> i32 {
        platform_specific::_draw_box(self._window, verch.to_chtype(), horch.to_chtype())
    }

    /// Creates an exact duplicate of the window.
    pub fn dupwin(&self) -> Window {
        let dup_win = unsafe { curses::dupwin(self._window) };
        Window {
            _window: dup_win,
            _stdscr: false,
            _deleted: false,
        }
    }

    /// Reports whether the given screen-relative y, x coordinates fall within the window.
    pub fn enclose(&self, y: i32, x: i32) -> bool {
        unsafe { curses::wenclose(self._window, y, x) > 0 }
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

    /// Returns true if the specified line in the specified window has been changed since the last
    /// call to refresh().
    pub fn is_linetouched(&self, line: i32) -> bool {
        unsafe { curses::is_linetouched(self._window, line) > 0 }
    }

    /// Returns true if the specified window has been changed since the last call to refresh().
    pub fn is_touched(&self) -> bool {
        unsafe { curses::is_wintouched(self._window) > 0 }
    }

    /// Controls whether getch() returns function/special keys as single key codes (e.g., the left
    /// arrow key as KEY_LEFT).
    ///
    /// Per X/Open, the default for keypad mode is OFF. You'll probably want it on. With keypad
    /// mode off, if a special key is pressed, getch() does nothing or returns ERR.
    pub fn keypad(&self, use_keypad: bool) -> i32 {
        unsafe { curses::keypad(self._window, use_keypad as u8) }
    }

    /// Insert the character ch before the character under the cursor.
    ///
    /// All characters to the right of the cursor are moved one space to the right, with the
    /// possibility of the rightmost character on the line being lost. The insertion operation does
    /// not change the cursor position.
    pub fn insch<T: ToChtype>(&self, ch: T) -> i32 {
        unsafe { curses::winsch(self._window, ch.to_chtype()) }
    }

    /// Converts between screen-relative and window-relative coordinates.
    /// 
    /// A to_screen parameter of true means to convert from window to screen;
    /// otherwise the reverse.
    pub fn mouse_trafo(&mut self, y: i32, x: i32, to_screen: bool) -> (i32, i32) {
        let mut mut_y = y;
        let mut mut_x = x;
        platform_specific::_mouse_trafo(&mut self._window, &mut mut_y, &mut mut_x, to_screen);
        (mut_y, mut_x)
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
            curses::mvwchgat(
                self._window,
                y,
                x,
                n,
                attributes,
                color_pair,
                ptr::null_mut(),
            )
        }
    }

    /// Moves a derived window (or subwindow) inside its parent window.
    ///
    /// The screen-relative parameters of the window are not changed. This routine is used to
    /// display different parts of the parent window at the same physical position on the screen.
    pub fn mvderwin(&self, pary: i32, parx: i32) -> i32 {
        unsafe { curses::mvderwin(self._window, pary, parx) }
    }

    /// Retrieves the character and attribute from the specified window position, in the form of a
    /// chtype.
    pub fn mvinch(&self, y: i32, x: i32) -> chtype {
        unsafe { curses::mvwinch(self._window, y, x) }
    }

    /// Move the cursor and then insert the character ch before the character under the cursor.
    ///
    /// First performs a cursor movement using wmove, and returns an error if the position is
    /// outside the window. All characters to the right of the cursor are moved one space to the
    /// right, with the possibility of the rightmost character on the line being lost. The insertion
    /// operation does not change the cursor position.
    pub fn mvinsch<T: ToChtype>(&self, y: i32, x: i32, ch: T) -> i32 {
        unsafe { curses::mvwinsch(self._window, y, x, ch.to_chtype()) }
    }

    /// Add a string to the window at the specified cursor position.
    pub fn mvprintw(&self, y: i32, x: i32, string: &str) -> i32 {
        let s = CString::new(string).unwrap();
        unsafe { curses::mvwprintw(self._window, y, x, s.as_ptr()) }
    }

    /// Moves the window so that the upper left-hand corner is at position (y,x).
    ///
    /// If the move would cause the window to be off the screen, it is an error and the window is
    /// not moved. Moving subwindows is allowed.
    pub fn mvwin(&self, y: i32, x: i32) -> i32 {
        unsafe { curses::mvwin(self._window, y, x) }
    }

    /// Controls whether wgetch() is a non-blocking call. If the option is enabled, and
    /// no input is ready, wgetch() will return ERR. If disabled, wgetch() will hang until input is
    /// ready.
    pub fn nodelay(&self, enabled: bool) -> i32 {
        unsafe { curses::nodelay(self._window, enabled as u8) as i32 }
    }

    /// Copies the window to the virtual screen.
    pub fn noutrefresh(&self) -> i32 {
        unsafe { curses::wnoutrefresh(self._window) }
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
    /// you will often need to call touchwin() before calling refresh().
    pub fn subwin(&self, nlines: i32, ncols: i32, begy: i32, begx: i32) -> Result<Window, i32> {
        let new_window = unsafe { curses::subwin(self._window, nlines, ncols, begy, begx) };
        if new_window.is_null() {
            Err(ERR)
        } else {
            Ok(Window {
                _window: new_window,
                _stdscr: false,
                _deleted: false,
            })
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

    /// Throws away all information about which parts of the window have been touched, pretending
    /// that the entire window has been drawn on.
    ///
    ///  This is sometimes necessary when using overlapping windows, since a change to one window
    /// will affect the other window, but the records of which lines have been changed in the other
    /// window will not reflect the change.
    pub fn touch(&self) -> i32 {
        unsafe { curses::touchwin(self._window) }
    }

    /// Throws away all information about which parts of the window have been touched, pretending
    /// that the entire window has been drawn on.
    ///
    ///  This is sometimes necessary when using overlapping windows, since a change to one window
    /// will affect the other window, but the records of which lines have been changed in the other
    /// window will not reflect the change.
    pub fn touchline(&self, start: i32, count: i32) -> i32 {
        unsafe { curses::touchline(self._window, start, count) }
    }

    /// Makes n lines in the window, starting at line y, look as if they have or have not been
    /// changed since the last call to refresh().
    pub fn touchln(&self, y: i32, n: i32, changed: bool) -> i32 {
        unsafe { curses::wtouchln(self._window, y, n, if changed { 1 } else { 0 }) }
    }

    /// Places ch back onto the input queue to be returned by the next call to getch().
    pub fn ungetch(&self, input: &Input) -> i32 {
        platform_specific::_ungetch(input)
    }

    /// Marks all lines in the window as unchanged since the last call to refresh().
    pub fn untouch(&self) -> i32 {
        unsafe { curses::untouchwin(self._window) }
    }

    /// Draw a vertical line using ch from the current cursor position. The line is at most
    /// n characters long, or as many as fit into the window.
    pub fn vline<T: ToChtype>(&self, ch: T, n: i32) -> i32 {
        unsafe { curses::wvline(self._window, ch.to_chtype(), n) }
    }
}

pub fn new_window(window_pointer: WindowPointer, is_stdscr: bool) -> Window {
    Window {
        _window: window_pointer,
        _stdscr: is_stdscr,
        _deleted: false,
    }
}

/// Automatically clean up window resources when dropped
impl Drop for Window {
    fn drop(&mut self) {
        if !self._stdscr && !self._deleted {
            unsafe {
                curses::delwin(self._window);
            }
        }
    }
}
