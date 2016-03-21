#![allow(non_camel_case_types, non_snake_case)]
extern crate pdcurses;

use pdcurses::*;

pub const COLOR_BLACK: i16 = 0;
pub const COLOR_BLUE: i16 = 1;
pub const COLOR_GREEN: i16 = 2;
pub const COLOR_RED: i16 = 4;
pub const COLOR_CYAN: i16 = (COLOR_BLUE | COLOR_GREEN);
pub const COLOR_MAGENTA: i16 = (COLOR_RED | COLOR_BLUE);
pub const COLOR_YELLOW: i16 = (COLOR_RED | COLOR_GREEN);
pub const COLOR_WHITE: i16 = 7;

const PDC_CHARTEXT_BITS: chtype = 21;
const PDC_COLOR_SHIFT: chtype = PDC_CHARTEXT_BITS + 12;

pub const A_NORMAL: chtype = 0;

const A_COLOR: chtype = 0x7fffffff << PDC_COLOR_SHIFT;
pub const A_BOLD: chtype = 0x080 << PDC_CHARTEXT_BITS;

pub const KEY_OFFSET: i32 = 0xec00;
pub const KEY_RESIZE: i32 = (KEY_OFFSET + 0x122); /* Window resize */

pub fn COLOR_PAIR(n: chtype) -> chtype {
    (n << PDC_COLOR_SHIFT) & A_COLOR
}

pub fn _attrset(w: *mut WINDOW, attributes: chtype) -> i32 {
    unsafe { wattrset(w, attributes) }
}

pub fn _resize_term(nlines: i32, ncols: i32) -> i32 {
    unsafe { resize_term(nlines, ncols) }
}
