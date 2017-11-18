extern crate winreg;
#[macro_use]
extern crate log;

use std::env;
use std::ffi::OsStr;
use std::i32;
use std::path::Path;

use self::winreg::RegKey;
use self::winreg::enums::HKEY_CURRENT_USER;

pub fn pre_init() {
    let exe_name = env::current_exe()
        .ok()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(String::from)
        .map(|mut x| {
            let last_dot = x.rfind('.').unwrap_or(0);
            x.truncate(last_dot);
            x
        });

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

    // Set the resize limits to 2 - i32::MAX to allow unlimited resizing
    #[cfg(not(feature = "disable_resize"))]
    {
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
