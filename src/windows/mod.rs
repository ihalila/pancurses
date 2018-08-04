#![allow(non_camel_case_types, non_snake_case)]
use pdcurses::*;
use libc::c_int;

use std::ffi::{CStr, CString};
use std::char::decode_utf16;
use std::iter;
use std::cmp;

pub mod constants;
use self::constants::*;

use input::Input;

#[cfg(any(feature = "win32a", all(not(feature = "win32"), not(feature = "win32a"))))]
mod win32a;
#[cfg(any(feature = "win32a", all(not(feature = "win32"), not(feature = "win32a"))))]
use self::win32a as flavor;

#[cfg(feature = "win32")]
mod win32;
#[cfg(feature = "win32")]
use self::win32 as flavor;

pub use self::flavor::pre_init;

pub fn _attron(w: *mut WINDOW, attributes: chtype) -> i32 {
    unsafe { wattron(w, attributes) }
}

pub fn _attroff(w: *mut WINDOW, attributes: chtype) -> i32 {
    unsafe { wattroff(w, attributes) }
}

pub fn _attrset(w: *mut WINDOW, attributes: chtype) -> i32 {
    unsafe { wattrset(w, attributes) }
}

pub fn _COLORS() -> i32 {
    unsafe { COLORS }
}

pub fn _COLOR_PAIRS() -> i32 {
    unsafe { COLOR_PAIRS }
}

pub fn _draw_box(w: *mut WINDOW, verch: chtype, horch: chtype) -> i32 {
    unsafe { _box(w, verch, horch) }
}

pub fn _getmouse() -> Result<MEVENT, i32> {
    let mut mevent = MEVENT {
        id: 0,
        x: 0,
        y: 0,
        z: 0,
        bstate: 0,
    };
    let error = unsafe { nc_getmouse(&mut mevent) };
    if error == 0 {
        Ok(mevent)
    } else {
        Err(error)
    }
}

pub fn _keyname(code: i32) -> Option<String> {
    let ptr = unsafe { keyname(code) };
    if ptr.is_null() {
        None
    } else {
        unsafe {
            // First, get a byte slice of the returned name
            let bytes = CStr::from_ptr(ptr).to_bytes();
            // Then assume it's proper UF8 and allocate a String for it.
            Some(String::from_utf8_unchecked(bytes.to_vec()))
        }
    }
}

pub fn _mouse_trafo(w: &mut *mut WINDOW, y: &mut i32, x: &mut i32, to_screen: bool) {
    unsafe {
        wmouse_trafo(*w, y, x, to_screen as u8);
    }
}

pub fn _resize_term(nlines: i32, ncols: i32) -> i32 {
    unsafe { resize_term(nlines, ncols) }
}

pub fn _set_blink(enabled: bool) -> i32 {
    unsafe { PDC_set_blink(enabled as u8) }
}

pub fn _set_title(title: &str) {
    let s = CString::new(title).unwrap();
    unsafe { PDC_set_title(s.as_ptr()) }
}

/// Converts an integer returned by getch() to an Input value
pub fn to_special_keycode(i: i32) -> Option<Input> {
    // There's two sets of integer constants defined:
    // - The SPECIAL_KEY_CODES array that contains all codes that are adjacent to each
    // other for easy indexing into it
    // - A bunch of scattered constants that need to be checked for
    // TODO: Unify the constants into a map
    match i {
        KEY_RESIZE => Some(Input::KeyResize),
        KEY_MOUSE => Some(Input::KeyMouse),

        KEY_NUMPAD_UP => Some(Input::KeyUp),
        KEY_NUMPAD_DOWN => Some(Input::KeyDown),
        KEY_NUMPAD_LEFT => Some(Input::KeyLeft),
        KEY_NUMPAD_RIGHT => Some(Input::KeyRight),
        KEY_NUMPAD_END => Some(Input::KeyEnd),
        KEY_NUMPAD_HOME => Some(Input::KeyHome),
        KEY_NUMPAD_PAGE_UP => Some(Input::KeyPPage),
        KEY_NUMPAD_PAGE_DOWN => Some(Input::KeyNPage),
        KEY_NUMPAD_INSERT => Some(Input::KeyIC),
        KEY_NUMPAD_DELETE => Some(Input::KeyDC),
        KEY_NUMPAD_ENTER => Some(Input::Character('\n')),
        KEY_NUMPAD_PLUS => Some(Input::Character('+')),
        KEY_NUMPAD_MINUS => Some(Input::Character('-')),
        KEY_NUMPAD_ASTERISK => Some(Input::Character('*')),
        KEY_NUMPAD_SLASH => Some(Input::Character('/')),

        _ => {
            // Since not all special key codes have been added to the SPECIAL_KEY_CODES array,
            // we need to do some basic math if this input lands into it.
            let index = if i <= KEY_F15 {
                i - KEY_OFFSET // Input that is less than KEY_F15 can be converted directly into an
                               // an index of the SPECIAL_KEY_CODES array.
            } else {
                i - KEY_OFFSET - 48 // Input past KEY_F15 has to be offset down a bit, since PDCurses
                                    // has values for 64 function keys
            };
            if index < 0 || index as usize >= SPECIAL_KEY_CODES.len() {
                // Input is something else. This may require more processing to convert properly into utf8
                None
            } else {
                Some(SPECIAL_KEY_CODES[index as usize])
            }
        }
    }
}

pub fn _wgetch(w: *mut WINDOW) -> Option<Input> {
    let i = unsafe { wgetch(w) };
    if i < 0 {
        None
    } else {
        Some(to_special_keycode(i).unwrap_or_else(|| {
            // Assume that on Windows input is UTF-16
            // If decoding the single input value fails, it should mean that it is the leading part of a
            // surrogate pair so calling getch() again should return the trailing part

            decode_utf16(iter::once(i as u16))
                .map(|result| {
                    result
                        .map(|c| Input::Character(c))
                        .unwrap_or_else(|first_error| {
                            let trailing = unsafe { wgetch(w) };
                            let data = [i as u16, trailing as u16];
                            decode_utf16(data.into_iter().cloned())
                                .map(|result| {
                                    result.map(|c| Input::Character(c)).unwrap_or_else(
                                        |second_error| {
                                            warn!("Decoding input as UTF-16 failed. The two values that could not be decoded were {} and {}.", first_error.unpaired_surrogate(), second_error.unpaired_surrogate());
                                            Input::Unknown(second_error.unpaired_surrogate() as i32)
                                        },
                                    )
                                })
                                .next()
                                .unwrap()
                        })
                })
                .next()
                .unwrap()
        }))
    }
}

pub fn _ungetch(input: &Input) -> i32 {
    match *input {
        Input::Character(c) => {
            // Need to convert to UTF-16 since Rust chars are UTF-8 while PDCurses deals with UTF-16
            let mut utf16_buffer = [0; 2];
            c.encode_utf16(&mut utf16_buffer)
                .into_iter()
                .rev()
                .map(|x| unsafe { PDC_ungetch(*x as c_int) })
                .fold(0, |res, x| cmp::min(res, x))
        }
        Input::Unknown(i) => unsafe { PDC_ungetch(i) },
        Input::KeyResize => unsafe { PDC_ungetch(KEY_RESIZE) },
        Input::KeyMouse => unsafe { PDC_ungetch(KEY_MOUSE) },
        specialKeyCode => {
            for (i, skc) in SPECIAL_KEY_CODES.into_iter().enumerate() {
                if *skc == specialKeyCode {
                    let result = i as c_int + KEY_OFFSET;
                    if result <= KEY_F15 {
                        return unsafe { PDC_ungetch(result) };
                    } else {
                        return unsafe { PDC_ungetch(result + 48) };
                    }
                }
            }
            panic!("Failed to convert Input back to a c_int");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use input::Input;

    #[test]
    fn test_key_dl_to_special_keycode() {
        assert_eq!(Input::KeyDL, to_special_keycode(KEY_OFFSET + 0x48).unwrap());
    }

    #[test]
    fn test_key_f15_to_input() {
        assert_eq!(
            Input::KeyF15,
            to_special_keycode(KEY_OFFSET + 0x08 + 15).unwrap()
        );
    }

    #[test]
    fn test_key_up_to_input() {
        assert_eq!(Input::KeyUp, to_special_keycode(KEY_OFFSET + 3).unwrap());
    }

    #[test]
    fn test_ungetch() {
        let w = unsafe { initscr() };

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

        unsafe {
            endwin();
        }
    }

}
