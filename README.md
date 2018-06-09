# pancurses [![Build Status](https://travis-ci.org/ihalila/pancurses.svg?branch=master)](https://travis-ci.org/ihalila/pancurses) [![Build status](https://ci.appveyor.com/api/projects/status/x4j52ihig9n2e25y?svg=true)](https://ci.appveyor.com/project/ihalila/pancurses) [![Crates.io](https://img.shields.io/crates/v/pancurses.svg)](https://crates.io/crates/pancurses)

pancurses is a curses library for Rust that supports both Linux and Windows
by abstracting away the backend that it uses
([ncurses-rs](https://github.com/jeaye/ncurses-rs) and
[pdcurses-sys](https://github.com/ihalila/pdcurses-sys) respectively).

The aim is to provide a more Rustic interface over the usual curses functions
for ease of use while remaining close enough to curses to make porting easy.

## [Documentation](https://docs.rs/pancurses)

## Requirements
#### Linux
ncurses-rs links with the native ncurses library so that needs to be installed
so that the linker can find it.

Check [ncurses-rs](https://github.com/jeaye/ncurses-rs) for more details.

#### Windows
pdcurses-sys compiles the native PDCurses library as part of the build process,
so you need to have a compatible C compiler available that matches the ABI of
the version of Rust you're using (so either gcc for the GNU ABI or cl for MSVC)

Check [pdcurses-sys](https://github.com/ihalila/pdcurses-sys) for more details.

## Usage
Cargo.toml
```toml
[dependencies]
pancurses = "0.16"
```

main.rs
```rust
extern crate pancurses;

use pancurses::{initscr, endwin};

fn main() {
  let window = initscr();
  window.printw("Hello Rust");
  window.refresh();
  window.getch();
  endwin();
}
```

## Pattern matching with getch()

```rust
extern crate pancurses;

use pancurses::{initscr, endwin, Input, noecho};

fn main() {
  let window = initscr();
  window.printw("Type things, press delete to quit\n");
  window.refresh();
  window.keypad(true);
  noecho();
  loop {
      match window.getch() {
          Some(Input::Character(c)) => { window.addch(c); },
          Some(Input::KeyDC) => break,
          Some(input) => { window.addstr(&format!("{:?}", input)); },
          None => ()
      }
  }
  endwin();
}
```

## Handling mouse input

To receive mouse events you need to both enable keypad mode and set a mouse mask that corresponds
to the events you are interested in. Mouse events are received in the same way as keyboard events,
ie. by calling getch().

```rust
extern crate pancurses;

use pancurses::{ALL_MOUSE_EVENTS, endwin, getmouse, initscr, mousemask, Input};

fn main() {
    let window = initscr();

    window.keypad(true); // Set keypad mode
    mousemask(ALL_MOUSE_EVENTS, std::ptr::null_mut()); // Listen to all mouse events

    window.printw("Click in the terminal, press q to exit\n");
    window.refresh();

    loop {
        match window.getch() {
            Some(Input::KeyMouse) => {
                if let Ok(mouse_event) = getmouse() {
                    window.mvprintw(1, 0,
                                    &format!("Mouse at {},{}", mouse_event.x, mouse_event.y),
                    );
                };
            }
            Some(Input::Character(x)) if x == 'q' => break,
            _ => (),
        }
    }
    endwin();
}
```

You can also receive events for the mouse simply moving (as long as the terminal you're running on
supports it) by also specifying the REPORT_MOUSE_POSITION flag:
```rust
mousemask(ALL_MOUSE_EVENTS | REPORT_MOUSE_POSITION, std::ptr::null_mut());
```

## Terminal resizing

Whenever the terminal is resized by the user a Input::KeyResize event is raised. You should handle
this by calling ```resize_term(0, 0)``` to have curses adjust it's internal structures to match the
new size.

## PDCurses (Windows) details

pdcurses-sys supports two flavors of PDCurses, win32a and win32. win32a is the GDI mode while win32
runs in the Windows console. win32a has better support for colors and text effects.

By default the win32a flavor is used, but you can specify which one you want to use by using Cargo
flags. Simply specify the feature in Cargo.toml like so:

```rust
[dependencies.pancurses]
version = "0.16"
features = ["win32a"]
```
or

```rust
[dependencies.pancurses]
version = "0.16"
features = ["win32"]
```

### (Font, Paste) menu

PDCurses win32a has a menu that allows you to change the font and paste text into the window.
pancurses disables the window by default, though the user can still right-click the title bar to 
access it. If you want to retain the PDCurses default behaviour of having the menu there set the 
feature ```"show_menu"```.

### Resizing

On win32a the default is to allow the user to freely resize the window. If you wish to disable
resizing set the feature ```"disable_resize"```

## License

Licensed under the MIT license, see [LICENSE.md](LICENSE.md)
