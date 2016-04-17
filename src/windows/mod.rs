#![allow(non_camel_case_types, non_snake_case)]
use pdcurses::*;
use libc::c_int;
use std::ffi::CString;

pub mod constants;
use self::constants::*;

use input::Input;

pub fn _attron(w: *mut WINDOW, attributes: chtype) -> i32 {
    unsafe { wattron(w, attributes) }
}

pub fn _attroff(w: *mut WINDOW, attributes: chtype) -> i32 {
    unsafe { wattroff(w, attributes) }
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

pub fn _set_title(title: &str) {
    let s = CString::new(title).unwrap();
    unsafe { PDC_set_title(s.as_ptr()) }
}

/// Converts an integer returned by getch() to a Input value
pub fn to_special_keycode(i: i32) -> Input {
    // Not the best, but until I add _all_ the keys between UNDO and RESIZE this will have to do
    // Most of them are PDCurses specific anyway and would not map to Input if I want to keep it
    // clean of implementation specific keys.
    if i == KEY_RESIZE {
        Input::KeyResize
    } else {
        assert!(i >= KEY_OFFSET, format!("Input value less than expected: {:?}", i));
        if i > KEY_UNDO {
            Input::Unknown(i)
        } else {
            let i = if i <= KEY_F15 {
                i - KEY_OFFSET
            } else {
                i - KEY_OFFSET - 48
            };
            SPECIAL_KEY_CODES[i as usize]
        }
    }
}

pub fn _ungetch(input: &Input) -> i32 {
    let i = convert_input_to_c_int(input);
    unsafe { PDC_ungetch(i) }
}

fn convert_input_to_c_int(input: &Input) -> c_int {
    match *input {
        Input::Character(c) => c as c_int,
        Input::Unknown(i) => i,
        specialKeyCode => {
            for (i, skc) in SPECIAL_KEY_CODES.into_iter().enumerate() {
                if *skc == specialKeyCode {
                    let result = i as c_int + KEY_OFFSET;
                    if result <= KEY_F15 {
                        return result;
                    } else {
                        return result + 48;
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
    use super::convert_input_to_c_int;
    use super::constants::*;
    use input::Input;
    use libc::c_int;

    #[test]
    fn test_key_dl_to_special_keycode() {
        assert_eq!(Input::KeyDL, to_special_keycode(KEY_OFFSET + 0x48));
    }

    #[test]
    fn test_key_f15_to_input() {
        assert_eq!(Input::KeyF15, to_special_keycode(KEY_OFFSET + 0x08 + 15));
    }

    #[test]
    fn test_key_up_to_input() {
        assert_eq!(Input::KeyUp, to_special_keycode(KEY_OFFSET + 3));
    }

    #[test]
    fn test_convert_input_to_c_int() {
        let i = convert_input_to_c_int(&Input::Character('a'));
        assert_eq!('a' as c_int, i);
    }

    #[test]
    fn test_convert_backspace_to_c_int() {
        let i = convert_input_to_c_int(&Input::KeyBackspace);
        assert_eq!(KEY_OFFSET + 0x07, i);
    }

    #[test]
    fn test_convert_sdl_to_c_int() {
        let i = convert_input_to_c_int(&Input::KeySDL);
        assert_eq!(KEY_OFFSET + 0x7e, i);
    }
}
