use input::Input;
use ncurses;
use ncurses::ll::{attr_t, chtype, mmask_t};
use ncurses::NCURSES_ATTR_SHIFT;

pub use ncurses::{ACS_LRCORNER, ACS_URCORNER, ACS_ULCORNER, ACS_LLCORNER};
pub use ncurses::{ACS_PLUS, ACS_LTEE, ACS_RTEE, ACS_BTEE, ACS_TTEE, ACS_HLINE, ACS_VLINE};
pub use ncurses::{ACS_S1, ACS_S9, ACS_DIAMOND, ACS_CKBOARD, ACS_DEGREE, ACS_PLMINUS, ACS_BULLET};
pub use ncurses::{ACS_LARROW, ACS_RARROW, ACS_DARROW, ACS_UARROW, ACS_BOARD, ACS_LANTERN, ACS_BLOCK};
pub use ncurses::{ACS_S3, ACS_S7, ACS_LEQUAL, ACS_GEQUAL, ACS_PI, ACS_NEQUAL, ACS_STERLING};
pub use ncurses::{ACS_BSSB, ACS_SSBB, ACS_BBSS, ACS_SBBS, ACS_SBSS, ACS_SSSB, ACS_SSBS}
pub use ncurses::{ACS_BSSS, ACS_BSBS, ACS_SBSB, ACS_SSSS}

fn NCURSES_BITS(mask: u32, shift: u32) -> u32 {
    mask << (shift + NCURSES_ATTR_SHIFT) as usize
}

pub fn COLOR_PAIR(n: chtype) -> attr_t {
    NCURSES_BITS(n as u32, 0u32) as attr_t
}

pub const COLOR_BLACK: i16 = 0;
pub const COLOR_RED: i16 = 1;
pub const COLOR_GREEN: i16 = 2;
pub const COLOR_YELLOW: i16 = 3;
pub const COLOR_BLUE: i16 = 4;
pub const COLOR_MAGENTA: i16 = 5;
pub const COLOR_CYAN: i16 = 6;
pub const COLOR_WHITE: i16 = 7;

pub const A_ALTCHARSET: attr_t = (1u32 << (14u32 + NCURSES_ATTR_SHIFT)) as attr_t;
pub const A_ATTRIBUTES: attr_t = (!0u32 << (0u32 + NCURSES_ATTR_SHIFT)) as attr_t;
pub const A_BLINK: attr_t = (1u32 << (11u32 + NCURSES_ATTR_SHIFT)) as attr_t;
pub const A_BOLD: attr_t = (1u32 << (13u32 + NCURSES_ATTR_SHIFT)) as attr_t;
pub const A_CHARTEXT: attr_t = (1u32 << (0u32 + NCURSES_ATTR_SHIFT)) as attr_t;
pub const A_COLOR: attr_t = ((((1u32) << 8) - 1u32) << (0u32 + NCURSES_ATTR_SHIFT)) as attr_t;
pub const A_DIM: attr_t = (1u32 << (12u32 + NCURSES_ATTR_SHIFT)) as attr_t;
pub const A_ITALIC: attr_t = (1u32 << (23 + NCURSES_ATTR_SHIFT)) as attr_t;
pub const A_INVIS: attr_t = (1u32 << (15u32 + NCURSES_ATTR_SHIFT)) as attr_t;
pub const A_LEFTLINE: attr_t = 0; // Not supported on ncurses
pub const A_NORMAL: attr_t = 0u32 as attr_t;
pub const A_OVERLINE: attr_t = 0; // Not supported on ncurses
pub const A_REVERSE: attr_t = (1u32 << (10 + NCURSES_ATTR_SHIFT)) as attr_t;
pub const A_RIGHTLINE: attr_t = 0; // Not supported on ncurses
pub const A_STANDOUT: attr_t = (1u32 << (8 + NCURSES_ATTR_SHIFT)) as attr_t;
pub const A_STRIKEOUT: attr_t = 0; // Not supported on ncurses
pub const A_UNDERLINE: attr_t = (1u32 << (9 + NCURSES_ATTR_SHIFT)) as attr_t;

pub const KEY_RESIZE: i32 = 0632;

pub const KEY_OFFSET: i32 = 0o0400;
pub const KEY_F15: i32 = (KEY_OFFSET + 0x17);
pub const KEY_EVENT: i32 = (KEY_OFFSET + 0o633);

pub const SPECIAL_KEY_CODES: [Input; 108] = [Input::KeyCodeYes,

                                             Input::KeyBreak,
                                             Input::KeyDown,
                                             Input::KeyUp,
                                             Input::KeyLeft,
                                             Input::KeyRight,
                                             Input::KeyHome,
                                             Input::KeyBackspace,

                                             Input::KeyF0,
                                             Input::KeyF1,
                                             Input::KeyF2,
                                             Input::KeyF3,
                                             Input::KeyF4,
                                             Input::KeyF5,
                                             Input::KeyF6,
                                             Input::KeyF7,
                                             Input::KeyF8,
                                             Input::KeyF9,
                                             Input::KeyF10,
                                             Input::KeyF11,
                                             Input::KeyF12,
                                             Input::KeyF13,
                                             Input::KeyF14,
                                             Input::KeyF15,
                                             // ncurses reserves space for 64 function keys, but we've
                                             // only implemented 15. This has to be taken into account
                                             // when converting the integer into an index of this array
                                             Input::KeyDL,
                                             Input::KeyIL,
                                             Input::KeyDC,
                                             Input::KeyIC,
                                             Input::KeyEIC,
                                             Input::KeyClear,
                                             Input::KeyEOS,
                                             Input::KeyEOL,
                                             Input::KeySF,
                                             Input::KeySR,
                                             Input::KeyNPage,
                                             Input::KeyPPage,
                                             Input::KeySTab,
                                             Input::KeyCTab,
                                             Input::KeyCATab,
                                             Input::KeyEnter,
                                             Input::KeySReset,
                                             Input::KeyReset,
                                             Input::KeyPrint,
                                             Input::KeyLL,
                                             Input::KeyA1,
                                             Input::KeyA3,
                                             Input::KeyB2,
                                             Input::KeyC1,
                                             Input::KeyC3,
                                             Input::KeyBTab,
                                             Input::KeyBeg,
                                             Input::KeyCancel,
                                             Input::KeyClose,
                                             Input::KeyCommand,
                                             Input::KeyCopy,
                                             Input::KeyCreate,
                                             Input::KeyEnd,
                                             Input::KeyExit,
                                             Input::KeyFind,
                                             Input::KeyHelp,
                                             Input::KeyMark,
                                             Input::KeyMessage,
                                             Input::KeyMove,
                                             Input::KeyNext,
                                             Input::KeyOpen,
                                             Input::KeyOptions,
                                             Input::KeyPrevious,
                                             Input::KeyRedo,
                                             Input::KeyReference,
                                             Input::KeyRefresh,
                                             Input::KeyReplace,
                                             Input::KeyRestart,
                                             Input::KeyResume,
                                             Input::KeySave,
                                             Input::KeySBeg,
                                             Input::KeySCancel,
                                             Input::KeySCommand,
                                             Input::KeySCopy,
                                             Input::KeySCreate,
                                             Input::KeySDC,
                                             Input::KeySDL,
                                             Input::KeySelect,
                                             Input::KeySEnd,
                                             Input::KeySEOL,
                                             Input::KeySExit,
                                             Input::KeySFind,
                                             Input::KeySHelp,
                                             Input::KeySHome,
                                             Input::KeySIC,

                                             Input::KeySLeft,
                                             Input::KeySMessage,
                                             Input::KeySMove,
                                             Input::KeySNext,
                                             Input::KeySOptions,
                                             Input::KeySPrevious,
                                             Input::KeySPrint,
                                             Input::KeySRedo,
                                             Input::KeySReplace,
                                             Input::KeySRight,
                                             Input::KeySResume,
                                             Input::KeySSave,
                                             Input::KeySSuspend,
                                             Input::KeySUndo,
                                             Input::KeySuspend,
                                             Input::KeyUndo,
                                             Input::KeyMouse,
                                             Input::KeyResize,
                                             Input::KeyEvent];

pub const ALL_MOUSE_EVENTS: mmask_t = ncurses::ALL_MOUSE_EVENTS as mmask_t;