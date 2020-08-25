#![allow(non_camel_case_types, non_snake_case)]
pub mod constants;
use self::constants::*;

use ncurses::{box_, getmouse, keyname, COLORS, COLOR_PAIRS};
use ncurses::ll::{chtype, ungetch, wattroff, wattron, wattrset, MEVENT, NCURSES_ATTR_T, WINDOW};
use ncurses::ll::{resize_term, wgetch};

use libc::{c_int, setlocale, LC_ALL};
use crate::input::Input;

use std::ffi::CString;
use std::string::FromUtf8Error;

pub fn pre_init() {
    let buf = CString::new("").unwrap();
    unsafe { setlocale(LC_ALL, buf.as_ptr()) };
}

pub fn _attron(w: WINDOW, attributes: chtype) -> i32 {
    unsafe { wattron(w, attributes as NCURSES_ATTR_T) }
}

pub fn _attroff(w: WINDOW, attributes: chtype) -> i32 {
    unsafe { wattroff(w, attributes as NCURSES_ATTR_T) }
}

pub fn _attrset(w: WINDOW, attributes: chtype) -> i32 {
    unsafe { wattrset(w, attributes as NCURSES_ATTR_T) }
}

pub fn _COLORS() -> i32 {
    COLORS()
}

pub fn _COLOR_PAIRS() -> i32 {
    COLOR_PAIRS()
}

pub fn _draw_box(w: WINDOW, verch: chtype, horch: chtype) -> i32 {
    box_(w, verch, horch)
}

pub fn _getmouse() -> Result<MEVENT, i32> {
    let mut mevent = MEVENT {
        id: 0,
        x: 0,
        y: 0,
        z: 0,
        bstate: 0,
    };
    let error = getmouse(&mut mevent);
    if error == 0 {
        Ok(mevent)
    } else {
        Err(error)
    }
}

pub fn _keyname(code: i32) -> Option<String> {
    keyname(code)
}

pub fn _resize_term(nlines: i32, ncols: i32) -> i32 {
    unsafe { resize_term(nlines, ncols) }
}

pub fn _set_blink(_: bool) -> i32 {
    0 // Not supported
}

pub fn _set_title(_: &str) {
    //Not supported
}

/// Converts an integer returned by getch() to a Input value
pub fn to_special_keycode(i: i32) -> Option<Input> {
    let index = if i <= KEY_F15 {
        i - KEY_OFFSET
    } else {
        i - KEY_OFFSET - 48
    };
    if index < 0 || index as usize >= SPECIAL_KEY_CODES.len() {
        None
    } else {
        Some(SPECIAL_KEY_CODES[index as usize])
    }
}

pub fn _ungetch(input: &Input) -> i32 {
    match *input {
        Input::Character(c) => {
            // Need to convert to UTF-8 bytes, it's how we get them from getch()
            let mut utf8_buffer = [0; 4];
            c.encode_utf8(&mut utf8_buffer)
                .as_bytes()
                .into_iter()
                .rev()
                .map(|x| unsafe { ungetch(*x as c_int) })
                .fold(0, |res, x| i32::min(res, x))
        }
        Input::Unknown(i) => unsafe { ungetch(i) },
        specialKeyCode => {
            for (i, skc) in SPECIAL_KEY_CODES.into_iter().enumerate() {
                if *skc == specialKeyCode {
                    let result = i as c_int + KEY_OFFSET;
                    if result <= KEY_F15 {
                        return unsafe { ungetch(result) };
                    } else {
                        return unsafe { ungetch(result + 48) };
                    }
                }
            }
            panic!("Failed to convert Input back to a c_int");
        }
    }
}

pub fn _wgetch(w: WINDOW) -> Option<Input> {
    let i = unsafe { wgetch(w) };
    if i < 0 {
        None
    } else {
        Some(to_special_keycode(i).unwrap_or_else(|| {
            // Assume that on Linux input is UTF-8
            fn try_decode(mut v: Vec<u8>, w: WINDOW) -> Result<String, FromUtf8Error> {
                let res = String::from_utf8(v.clone());
                if res.is_err() && v.len() < 4 {
                    let next_byte = unsafe { wgetch(w) };
                    v.push(next_byte as u8);
                    try_decode(v, w)
                } else {
                    res
                }
            }

            let v = vec![i as u8];
            try_decode(v, w)
                .map(|s| Input::Character(s.chars().next().unwrap()))
                .unwrap_or_else(|error| {
                    warn!("Decoding input as UTF-8 failed: {:?}", error);
                    Input::Unknown(i)
                })
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::Input;
    use ncurses::{endwin, initscr};

    #[test]
    fn test_key_dl_to_special_keycode() {
        let keyDl = 0o510;
        assert_eq!(Input::KeyDL, to_special_keycode(keyDl).unwrap());
    }

    #[test]
    fn test_key_f15_to_input() {
        let keyF15 = 0o410 + 15;
        assert_eq!(Input::KeyF15, to_special_keycode(keyF15).unwrap());
    }

    #[test]
    fn test_key_up_to_input() {
        let keyUp = 0o403;
        assert_eq!(Input::KeyUp, to_special_keycode(keyUp).unwrap());
    }

    #[test]
    fn test_ungetch() {
        let w = initscr();

        let chars = [
            'a', 'b', 'c', 'ä', 'ö', 'å', 'A', 'B', 'C', 'Ä', 'Ö', 'Å', '𤭢', '𐍈',
            '€', 'ᚠ', 'ᛇ', 'ᚻ', 'þ', 'ð', 'γ', 'λ', 'ώ', 'б', 'е', 'р', 'ვ',
            'ე', 'პ', 'ხ', 'இ', 'ங', 'க', 'ಬ', 'ಇ', 'ಲ', 'ಸ',
        ];

        chars.into_iter().for_each(|c| {
            _ungetch(&Input::Character(*c));
            assert_eq!(_wgetch(w).unwrap(), Input::Character(*c));
        });

        SPECIAL_KEY_CODES.into_iter().for_each(|i| {
            _ungetch(i);
            assert_eq!(_wgetch(w).unwrap(), *i);
        });

        endwin();
    }

}
