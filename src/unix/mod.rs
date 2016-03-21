#![allow(non_camel_case_types, non_snake_case)]
extern crate ncurses;

use ncurses::ll::{chtype, attr_t, WINDOW, wattrset};

use ncurses::NCURSES_ATTR_SHIFT;

pub fn NCURSES_BITS(mask: u32, shift: u32) -> u32 {
    mask << (shift + NCURSES_ATTR_SHIFT) as usize
}

pub const COLOR_BLACK: i16 = 0;
pub const COLOR_RED: i16 = 1;
pub const COLOR_GREEN: i16 = 2;
pub const COLOR_YELLOW: i16 = 3;
pub const COLOR_BLUE: i16 = 4;
pub const COLOR_MAGENTA: i16 = 5;
pub const COLOR_CYAN: i16 = 6;
pub const COLOR_WHITE: i16 = 7;

pub const A_NORMAL: attr_t = 0u32 as attr_t;
pub const A_ATTRIBUTES: attr_t = (!0u32 << (0u32 + NCURSES_ATTR_SHIFT)) as attr_t;
pub const A_CHARTEXT: attr_t = (1u32 << (0u32 + NCURSES_ATTR_SHIFT)) as attr_t;
pub const A_COLOR: attr_t = ((((1u32) << 8) - 1u32) << (0u32 + NCURSES_ATTR_SHIFT)) as attr_t;

// pub fn A_STANDOUT() -> attr_t
// { NCURSES_BITS(1u32, 8u32) as attr_t }
//
// pub fn A_UNDERLINE() -> attr_t
// { NCURSES_BITS(1u32, 9u32) as attr_t }
//
// pub fn A_REVERSE() -> attr_t
// { NCURSES_BITS(1u32, 10u32) as attr_t }
//
// pub fn A_BLINK() -> attr_t
// { NCURSES_BITS(1u32, 11u32) as attr_t }
//
// pub fn A_DIM() -> attr_t
// { NCURSES_BITS(1u32, 12u32) as attr_t }
//

pub const A_BOLD: attr_t = (1u32 << (13u32 + NCURSES_ATTR_SHIFT)) as attr_t;

pub const KEY_RESIZE: i32 = 0632;

pub fn COLOR_PAIR(n: chtype) -> attr_t {
    NCURSES_BITS(n as u32, 0u32) as attr_t
}

pub fn _attrset(w: WINDOW, attributes: chtype) -> i32 {
    unsafe { wattrset(w, attributes as i32) }
}

pub fn _resize_term(_nlines: i32, _ncols: i32) -> i32 {
    error!("resize_term is not implemented in ncurses-rs");
    -1
}
