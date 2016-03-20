# pancurses [![Build Status](https://travis-ci.org/ihalila/pancurses.svg?branch=master)](https://travis-ci.org/ihalila/pancurses) [![Build status](https://ci.appveyor.com/api/projects/status/x4j52ihig9n2e25y?svg=true)](https://ci.appveyor.com/project/ihalila/pancurses) [![Crates.io](https://img.shields.io/crates/v/pancurses.svg)](https://crates.io/crates/pancurses)

pancurses is a curses libary for Rust that supports both Unix and Windows
platforms by abstracting away the backend that it uses
([ncurses-rs](https://github.com/jeaye/ncurses-rs) and
[pdcurses-sys](https://github.com/ihalila/pdcurses-sys) respectively).

The aim is to provide a more Rustic interface over the usual curses functions
for ease of use while remaining close enough to curses to make porting easy.

## [Documentation](http://ihalila.github.io/pancurses/pancurses/)

## Requirements
#### Unix platforms
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
pancurses = "0.1"
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

## Status

I'm working through implementing the various functions using the PDCurses
examples as a priority list. Version 0.1 has everything that a simple hello
world program and the firework example need. Version 0.2 will include everything
needed for the rain example.

## License

Licensed under the MIT license, see [LICENSE.md](LICENSE.md)
