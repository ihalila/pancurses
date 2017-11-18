#![allow(non_camel_case_types, non_snake_case)]
extern crate winreg;

use pdcurses::*;
use libc::c_int;

use std::env;
use std::ffi::{CString, OsStr};
use std::i32;
use std::path::Path;

use self::winreg::RegKey;
use windows::winreg::enums::HKEY_CURRENT_USER;

pub mod constants;
use self::constants::*;

use input::Input;

pub fn pre_init() {
    #[cfg(any(feature = "win32a", all(not(feature = "win32"), not(feature = "win32a"))))]
    win32a_pre_init();
}

#[cfg(any(feature = "win32a", all(not(feature = "win32"), not(feature = "win32a"))))]
fn win32a_pre_init() {

    let exe_name = env::current_exe()
        .ok()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(String::from);

    if exe_name.is_none() {
        warn!("Could not determine name of exe");
        return;
    }
    let exe_name = exe_name.unwrap();

    // The format of the registry value is:
    // <cols>x<rows>,<font_size>,<x_loc>,<y_loc>,<menu_shown>;<min_lines>,<max_lines>,<min_cols>,<max_cols>:<font_name>
    // Example: 80x25,12,312,312,1;2,2147483647,2,2147483647:Courier New
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let existing_value = hkcu.open_subkey("Software\\PDCurses")
        .and_then(|reg_key| reg_key.get_value::<String, &str>(&exe_name))
        .unwrap_or("80x25,12,0,0,1;25,25,80,80:Courier New".to_string());

    let rejoined_menu: String;
    let maxi32 = format!("{}", i32::MAX);
    let mut split: Vec<&str> = existing_value.split(',').collect();

    // The value at index 4 will be <menu_shown>;<min_lines> because we split on ,
    let mut menu_split: Vec<&str> = split[4].split(';').collect();

    // Set the menu_shown value to 0 to always hide it
    #[cfg(not(feature = "show_menu"))]
    {
        menu_split[0] = "0";
    }

    #[cfg(not(feature = "disable_resize"))]
    {
        // Set the resize limits to 2 - i32::MAX to allow unlimited resizing
        menu_split[1] = "2";
        split[5] = &maxi32;
        split[6] = "2";
        split[7] = &maxi32;
    }

    // Re-join the values back into a string
    rejoined_menu = menu_split.join(";");
    split[4] = &rejoined_menu;

    // Write the modified values back into the registry
    match hkcu.open_subkey("Software\\PDCurses").and_then(|reg_key| {
        reg_key.set_value::<String, &str>(&exe_name, &split.join(","))
    }) {
        Err(e) => warn!("Failed to set registry value: {}", e),
        _ => (),
    }
}

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
