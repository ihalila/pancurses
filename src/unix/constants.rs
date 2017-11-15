use input::Input;
use ncurses;
use ncurses::ll::{attr_t, chtype, mmask_t};
use ncurses::NCURSES_ATTR_SHIFT;

pub use ncurses::{ACS_LLCORNER, ACS_LRCORNER, ACS_ULCORNER, ACS_URCORNER};
pub use ncurses::{ACS_BTEE, ACS_HLINE, ACS_LTEE, ACS_PLUS, ACS_RTEE, ACS_TTEE, ACS_VLINE};
pub use ncurses::{ACS_S1, ACS_S9, ACS_BULLET, ACS_CKBOARD, ACS_DEGREE, ACS_DIAMOND, ACS_PLMINUS};
pub use ncurses::{ACS_BLOCK, ACS_BOARD, ACS_DARROW, ACS_LANTERN, ACS_LARROW, ACS_RARROW,
                  ACS_UARROW};
pub use ncurses::{ACS_S3, ACS_S7, ACS_GEQUAL, ACS_LEQUAL, ACS_NEQUAL, ACS_PI, ACS_STERLING};
pub use ncurses::{ACS_BBSS, ACS_BSSB, ACS_SBBS, ACS_SBSS, ACS_SSBB, ACS_SSBS, ACS_SSSB};
pub use ncurses::{ACS_BSBS, ACS_BSSS, ACS_SBSB, ACS_SSSS};

// We need to re-export the BUTTONX_Y constants as 'mmask_t's, otherwise you can't compare them
// to a MEVENT's bstate without casting
pub const BUTTON1_PRESSED: mmask_t = ncurses::BUTTON1_PRESSED as mmask_t;
pub const BUTTON1_RELEASED: mmask_t = ncurses::BUTTON1_RELEASED as mmask_t;
pub const BUTTON1_CLICKED: mmask_t = ncurses::BUTTON1_CLICKED as mmask_t;
pub const BUTTON1_DOUBLE_CLICKED: mmask_t = ncurses::BUTTON1_DOUBLE_CLICKED as mmask_t;
pub const BUTTON1_TRIPLE_CLICKED: mmask_t = ncurses::BUTTON1_TRIPLE_CLICKED as mmask_t;

pub const BUTTON2_PRESSED: mmask_t = ncurses::BUTTON2_PRESSED as mmask_t;
pub const BUTTON2_RELEASED: mmask_t = ncurses::BUTTON2_RELEASED as mmask_t;
pub const BUTTON2_CLICKED: mmask_t = ncurses::BUTTON2_CLICKED as mmask_t;
pub const BUTTON2_DOUBLE_CLICKED: mmask_t = ncurses::BUTTON2_DOUBLE_CLICKED as mmask_t;
pub const BUTTON2_TRIPLE_CLICKED: mmask_t = ncurses::BUTTON2_TRIPLE_CLICKED as mmask_t;

pub const BUTTON3_PRESSED: mmask_t = ncurses::BUTTON3_PRESSED as mmask_t;
pub const BUTTON3_RELEASED: mmask_t = ncurses::BUTTON3_RELEASED as mmask_t;
pub const BUTTON3_CLICKED: mmask_t = ncurses::BUTTON3_CLICKED as mmask_t;
pub const BUTTON3_DOUBLE_CLICKED: mmask_t = ncurses::BUTTON3_DOUBLE_CLICKED as mmask_t;
pub const BUTTON3_TRIPLE_CLICKED: mmask_t = ncurses::BUTTON3_TRIPLE_CLICKED as mmask_t;

pub const BUTTON4_PRESSED: mmask_t = ncurses::BUTTON4_PRESSED as mmask_t;
pub const BUTTON4_RELEASED: mmask_t = ncurses::BUTTON4_RELEASED as mmask_t;
pub const BUTTON4_CLICKED: mmask_t = ncurses::BUTTON4_CLICKED as mmask_t;
pub const BUTTON4_DOUBLE_CLICKED: mmask_t = ncurses::BUTTON4_DOUBLE_CLICKED as mmask_t;
pub const BUTTON4_TRIPLE_CLICKED: mmask_t = ncurses::BUTTON4_TRIPLE_CLICKED as mmask_t;

pub const BUTTON5_PRESSED: mmask_t = ncurses::BUTTON5_PRESSED as mmask_t;
pub const BUTTON5_RELEASED: mmask_t = ncurses::BUTTON5_RELEASED as mmask_t;
pub const BUTTON5_CLICKED: mmask_t = ncurses::BUTTON5_CLICKED as mmask_t;
pub const BUTTON5_DOUBLE_CLICKED: mmask_t = ncurses::BUTTON5_DOUBLE_CLICKED as mmask_t;
pub const BUTTON5_TRIPLE_CLICKED: mmask_t = ncurses::BUTTON5_TRIPLE_CLICKED as mmask_t;

pub const REPORT_MOUSE_POSITION: mmask_t = ncurses::REPORT_MOUSE_POSITION as mmask_t;
pub const BUTTON_SHIFT: mmask_t = ncurses::BUTTON_SHIFT as mmask_t;
pub const BUTTON_CTRL: mmask_t = ncurses::BUTTON_CTRL as mmask_t;
pub const BUTTON_ALT: mmask_t = ncurses::BUTTON_ALT as mmask_t;

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

pub const SPECIAL_KEY_CODES: [Input; 108] = [
    Input::KeyCodeYes,
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
    Input::KeyEvent,
];

pub const ALL_MOUSE_EVENTS: mmask_t = ncurses::ALL_MOUSE_EVENTS as mmask_t;
