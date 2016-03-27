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
pub const A_BLINK: attr_t = (1u32 << (11u32 + NCURSES_ATTR_SHIFT)) as attr_t;

// pub fn A_STANDOUT() -> attr_t
// { NCURSES_BITS(1u32, 8u32) as attr_t }
//
// pub fn A_UNDERLINE() -> attr_t
// { NCURSES_BITS(1u32, 9u32) as attr_t }
//
// pub fn A_REVERSE() -> attr_t
// { NCURSES_BITS(1u32, 10u32) as attr_t }
//
// pub fn A_DIM() -> attr_t
// { NCURSES_BITS(1u32, 12u32) as attr_t }
//

pub const A_BOLD: attr_t = (1u32 << (13u32 + NCURSES_ATTR_SHIFT)) as attr_t;

pub const KEY_RESIZE: i32 = 0632;

pub fn COLOR_PAIR(n: chtype) -> attr_t {
    NCURSES_BITS(n as u32, 0u32) as attr_t
}

pub fn _attron(w: WINDOW, attributes: chtype) -> i32 {
    unsafe { wattron(w, attributes as i32) }
}

pub fn _attrset(w: WINDOW, attributes: chtype) -> i32 {
    unsafe { wattrset(w, attributes as i32) }
}

pub fn _draw_box(w: *mut WINDOW, verch: chtype, horch: chtype) -> i32 {
    unsafe { box_(w, verch, horch) }
}

pub fn _resize_term(_nlines: i32, _ncols: i32) -> i32 {
    error!("resize_term is not implemented in ncurses-rs");
    -1
}

use input::Input;

const KEY_OFFSET: i32 = 0o0400;
const KEY_F15: i32 = (KEY_OFFSET + 0x17);
const KEY_EVENT: i32 = (KEY_OFFSET + 0o633);

const SPECIAL_KEY_CODES: [Input; 108] = [Input::KeyCodeYes,

                                         Input::KeyBreak,
                                         Input::KeyDown,
                                         Input::KeyUp,
                                         Input::KeyLeft,
                                         Input::KeyRight,
                                         Input::KeyHome,
                                         Input::KeyBackspace,

                                         Input::KeyF0,
                                         Input::KeyF1,
                                         Input::KeyF2,
                                         Input::KeyF3,
                                         Input::KeyF4,
                                         Input::KeyF5,
                                         Input::KeyF6,
                                         Input::KeyF7,
                                         Input::KeyF8,
                                         Input::KeyF9,
                                         Input::KeyF10,
                                         Input::KeyF11,
                                         Input::KeyF12,
                                         Input::KeyF13,
                                         Input::KeyF14,
                                         Input::KeyF15,
                                         // ncurses reserves space for 64 function keys, but we've
                                         // only implemented 15. This has to be taken into account
                                         // when converting the integer into an index of this array
                                         Input::KeyDL,
                                         Input::KeyIL,
                                         Input::KeyDC,
                                         Input::KeyIC,
                                         Input::KeyEIC,
                                         Input::KeyClear,
                                         Input::KeyEOS,
                                         Input::KeyEOL,
                                         Input::KeySF,
                                         Input::KeySR,
                                         Input::KeyNPage,
                                         Input::KeyPPage,
                                         Input::KeySTab,
                                         Input::KeyCTab,
                                         Input::KeyCATab,
                                         Input::KeyEnter,
                                         Input::KeySReset,
                                         Input::KeyReset,
                                         Input::KeyPrint,
                                         Input::KeyLL,
                                         Input::KeyA1,
                                         Input::KeyA3,
                                         Input::KeyB2,
                                         Input::KeyC1,
                                         Input::KeyC3,
                                         Input::KeyBTab,
                                         Input::KeyBeg,
                                         Input::KeyCancel,
                                         Input::KeyClose,
                                         Input::KeyCommand,
                                         Input::KeyCopy,
                                         Input::KeyCreate,
                                         Input::KeyEnd,
                                         Input::KeyExit,
                                         Input::KeyFind,
                                         Input::KeyHelp,
                                         Input::KeyMark,
                                         Input::KeyMessage,
                                         Input::KeyMove,
                                         Input::KeyNext,
                                         Input::KeyOpen,
                                         Input::KeyOptions,
                                         Input::KeyPrevious,
                                         Input::KeyRedo,
                                         Input::KeyReference,
                                         Input::KeyRefresh,
                                         Input::KeyReplace,
                                         Input::KeyRestart,
                                         Input::KeyResume,
                                         Input::KeySave,
                                         Input::KeySBeg,
                                         Input::KeySCancel,
                                         Input::KeySCommand,
                                         Input::KeySCopy,
                                         Input::KeySCreate,
                                         Input::KeySDC,
                                         Input::KeySDL,
                                         Input::KeySelect,
                                         Input::KeySEnd,
                                         Input::KeySEOL,
                                         Input::KeySExit,
                                         Input::KeySFind,
                                         Input::KeySHelp,
                                         Input::KeySHome,
                                         Input::KeySIC,

                                         Input::KeySLeft,
                                         Input::KeySMessage,
                                         Input::KeySMove,
                                         Input::KeySNext,
                                         Input::KeySOptions,
                                         Input::KeySPrevious,
                                         Input::KeySPrint,
                                         Input::KeySRedo,
                                         Input::KeySReplace,
                                         Input::KeySRight,
                                         Input::KeySResume,
                                         Input::KeySSave,
                                         Input::KeySSuspend,
                                         Input::KeySUndo,
                                         Input::KeySuspend,
                                         Input::KeyUndo,
                                         Input::KeyMouse,
                                         Input::KeyResize,
                                         Input::KeyEvent,
];

/// Converts an integer returned by getch() to a Input value
pub fn to_special_keycode(i: i32) -> Input {
    assert!(i >= KEY_OFFSET && i <= KEY_EVENT);
    let i = if i <= KEY_F15 {
        i - KEY_OFFSET
    } else {
        i - KEY_OFFSET - 48
    };
    SPECIAL_KEY_CODES[i as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    use input::Input;

    #[test]
    fn test_key_dl_to_special_keycode() {
        let keyDl = 0o510;
        assert_eq!(Input::KeyDL, to_special_keycode(keyDl));
    }

    #[test]
    fn test_key_f15_to_input() {
        let keyF15 = 0o410 + 15;
        assert_eq!(Input::KeyF15, to_special_keycode(keyF15));
    }

    #[test]
    fn test_key_up_to_input() {
        let keyUp = 0o403;
        assert_eq!(Input::KeyUp, to_special_keycode(keyUp));
    }
}
