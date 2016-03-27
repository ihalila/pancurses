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
pub const A_BLINK: chtype = 0x040 << PDC_CHARTEXT_BITS;

pub fn COLOR_PAIR(n: chtype) -> chtype {
    (n << PDC_COLOR_SHIFT) & A_COLOR
}

pub fn _attron(w: *mut WINDOW, attributes: chtype) -> i32 {
    unsafe { wattron(w, attributes) }
}

pub fn _attrset(w: *mut WINDOW, attributes: chtype) -> i32 {
    unsafe { wattrset(w, attributes) }
}

pub fn _draw_box(w: *mut WINDOW, verch: chtype, horch: chtype) -> i32 {
    unsafe { _box(w, verch, horch) }
}

pub fn _resize_term(nlines: i32, ncols: i32) -> i32 {
    unsafe { resize_term(nlines, ncols) }
}

use input::Input;

const KEY_OFFSET: i32 = 0xec00;
const KEY_F15: i32 = (KEY_OFFSET + 0x17);
const KEY_UNDO: i32 = (KEY_OFFSET + 0x96);
const KEY_RESIZE: i32 = (KEY_OFFSET + 0x122);

const SPECIAL_KEY_CODES: [Input; 102] = [Input::KeyCodeYes,

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
                                         // PDcurses reserves space for 64 function keys, but we've
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
                                         Input::KeyAbort,
                                         Input::KeySHelp,
                                         Input::KeyLHelp,
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
                                         Input::KeyUndo];

/// Converts an integer returned by getch() to a Input value
pub fn to_special_keycode(i: i32) -> Input {
    // Not the best, but until I add _all_ the keys between UNDO and RESIZE this will have to do
    // Most of them are PDCurses specific anyway and would not map to Input if I want to keep it
    // clean of implementation specific keys.
    if i == KEY_RESIZE {
        Input::KeyResize
    } else {
        assert!(i >= KEY_OFFSET && i <= KEY_UNDO);
        let i = if i <= KEY_F15 {
            i - KEY_OFFSET
        } else {
            i - KEY_OFFSET - 48
        };
        SPECIAL_KEY_CODES[i as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use input::Input;

    #[test]
    fn test_key_dl_to_special_keycode() {
        let keyOffset = 0xec00;
        let keyDl = keyOffset + 0x48;
        assert_eq!(Input::KeyDL, to_special_keycode(keyDl));
    }

    #[test]
    fn test_key_f15_to_input() {
        let keyOffset = 0xec00;
        let keyF15 = keyOffset + 0x08 + 15;
        assert_eq!(Input::KeyF15, to_special_keycode(keyF15));
    }

    #[test]
    fn test_key_up_to_input() {
        let keyOffset = 0xec00;
        let keyUp = keyOffset + 3;
        assert_eq!(Input::KeyUp, to_special_keycode(keyUp));
    }
}
