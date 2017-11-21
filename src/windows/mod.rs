#![allow(non_camel_case_types, non_snake_case)]
use pdcurses::*;
use libc::c_int;

use std::ffi::{CStr,CString};

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
    if error == 0 { Ok(mevent) } else { Err(error) }
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

/// Converts an integer returned by getch() to a Input value
pub fn to_special_keycode(i: i32) -> Input {
    // Not the best, but until I add _all_ the keys between UNDO and RESIZE this will have to do
    // Most of them are PDCurses specific anyway and would not map to Input if I want to keep it
    // clean of implementation specific keys.
    if i == KEY_RESIZE {
        Input::KeyResize
    } else if i == KEY_MOUSE {
        Input::KeyMouse
    } else {
        assert!(
            i >= KEY_OFFSET,
            format!("Input value less than expected: {:?}", i)
        );
        let i = if i <= KEY_F15 {
            i - KEY_OFFSET
        } else {
            i - KEY_OFFSET - 48
        };
        if i as usize >= SPECIAL_KEY_CODES.len() {
            Input::Unknown(i)
        } else {
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
        Input::KeyResize => KEY_RESIZE,
        Input::KeyMouse => KEY_MOUSE,
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

    #[test]
    fn test_convert_key_mouse() {
        let i = convert_input_to_c_int(&Input::KeyMouse);
        let kc = to_special_keycode(i);
        assert_eq!(i, convert_input_to_c_int(&kc));
    }
}
