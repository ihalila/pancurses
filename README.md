# pancurses [![Build Status](https://travis-ci.org/ihalila/pancurses.svg?branch=master)](https://travis-ci.org/ihalila/pancurses) [![Build status](https://ci.appveyor.com/api/projects/status/x4j52ihig9n2e25y?svg=true)](https://ci.appveyor.com/project/ihalila/pancurses) [![Crates.io](https://img.shields.io/crates/v/pancurses.svg)](https://crates.io/crates/pancurses)

pancurses is a curses libary for Rust that supports both Linux and Windows
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
pancurses = "0.4"
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

## Example: Pattern matching with getch()

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

## Status

I'm working through implementing the various functions using the PDCurses
demos as a priority list. Version 0.4 has everything that a simple hello
world program, the firework example, the rain example need and the newdemo
example needs. The 'newtest' demo is done, but not quite all features have
been implemented as there was a lot of PDCurses-specific stuff there, and
I'd rather implement the shared functions first.

## License

Licensed under the MIT license, see [LICENSE.md](LICENSE.md)
