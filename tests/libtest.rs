extern crate pancurses;

use pancurses::*;

#[test]
pub fn test_cursor_position() {
	let window = initialize();
	end();
}
