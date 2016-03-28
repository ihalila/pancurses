#![allow(non_camel_case_types, non_snake_case)]
mod constants;
pub use self::constants::*;

use ncurses::{box_, NCURSES_ATTR_SHIFT};
use ncurses::ll::{chtype, attr_t, WINDOW, wattron, wattrset, ungetch};
use libc::c_int;
use input::Input;

pub fn NCURSES_BITS(mask: u32, shift: u32) -> u32 {
    mask << (shift + NCURSES_ATTR_SHIFT) as usize
}

pub fn _attron(w: WINDOW, attributes: chtype) -> i32 {
    unsafe { wattron(w, attributes as i32) }
}

pub fn _attrset(w: WINDOW, attributes: chtype) -> i32 {
    unsafe { wattrset(w, attributes as i32) }
}

pub fn COLOR_PAIR(n: chtype) -> attr_t {
    NCURSES_BITS(n as u32, 0u32) as attr_t
}

pub fn _draw_box(w: WINDOW, verch: chtype, horch: chtype) -> i32 {
    box_(w, verch, horch)
}

pub fn _resize_term(_nlines: i32, _ncols: i32) -> i32 {
    error!("resize_term is not implemented in ncurses-rs");
    -1
}

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

pub fn _ungetch(input: &Input) -> i32 {
    let i = convert_input_to_c_int(input);
    unsafe { ungetch(i) }
}

fn convert_input_to_c_int(input: &Input) -> c_int {
    match *input {
        Input::Character(c) => c as c_int,
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
    use super::convert_input_to_c_int;
    use super::*;
    use input::Input;
    use libc::c_int;

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

    #[test]
    fn test_convert_input_to_c_int() {
        let i = convert_input_to_c_int(&Input::Character('a'));
        assert_eq!('a' as c_int, i);
    }

    #[test]
    fn test_convert_backspace_to_c_int() {
        let i = convert_input_to_c_int(&Input::KeyBackspace);
        assert_eq!(0o407, i);
    }

    #[test]
    fn test_convert_sdl_to_c_int() {
        let i = convert_input_to_c_int(&Input::KeySDL);
        assert_eq!(0o600, i);
    }

}
