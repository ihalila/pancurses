use input::Input;
use pdcurses::chtype;
use pdcurses::mmask_t;

pub fn COLOR_PAIR(n: chtype) -> chtype {
    (n << PDC_COLOR_SHIFT) & A_COLOR
}

pub const COLOR_BLACK: i16 = 0;
pub const COLOR_RED: i16 = 1;
pub const COLOR_GREEN: i16 = 2;
pub const COLOR_YELLOW: i16 = 3;
pub const COLOR_BLUE: i16 = 4;
pub const COLOR_MAGENTA: i16 = 5;
pub const COLOR_CYAN: i16 = 6;
pub const COLOR_WHITE: i16 = 7;

pub const PDC_CHARTEXT_BITS: chtype = 21;
pub const PDC_COLOR_SHIFT: chtype = PDC_CHARTEXT_BITS + 12;

pub const A_ALTCHARSET: chtype = 0x001 << PDC_CHARTEXT_BITS;
pub const A_BOLD: chtype = 0x080 << PDC_CHARTEXT_BITS;
pub const A_BLINK: chtype = 0x040 << PDC_CHARTEXT_BITS;
pub const A_COLOR: chtype = 0x7fffffff << PDC_COLOR_SHIFT;
pub const A_CHARTEXT: chtype = (0x1 << PDC_CHARTEXT_BITS) - 1;
pub const A_DIM: chtype = 0x400 << PDC_CHARTEXT_BITS;
pub const A_LEFTLINE: chtype = 0x004 << PDC_CHARTEXT_BITS;
pub const A_INVIS: chtype = 0x008 << PDC_CHARTEXT_BITS;
pub const A_ITALIC: chtype = A_INVIS;
pub const A_NORMAL: chtype = 0;
pub const A_OVERLINE: chtype = 0x100 << PDC_CHARTEXT_BITS;
pub const A_REVERSE: chtype = 0x020 << PDC_CHARTEXT_BITS;
pub const A_RIGHTLINE: chtype = 0x002 << PDC_CHARTEXT_BITS;
pub const A_STRIKEOUT: chtype = 0x200 << PDC_CHARTEXT_BITS;
pub const A_UNDERLINE: chtype = 0x010 << PDC_CHARTEXT_BITS;

pub const KEY_OFFSET: i32 = 0xec00;
pub const KEY_F15: i32 = (KEY_OFFSET + 0x17);
pub const KEY_UNDO: i32 = (KEY_OFFSET + 0x96);
pub const KEY_RESIZE: i32 = (KEY_OFFSET + 0x122);
pub const KEY_MOUSE: i32 = (KEY_OFFSET + 0x11b);

pub const SPECIAL_KEY_CODES: [Input; 102] = [
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
    // PDcurses reserves space for 64 function keys, but we've
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
    Input::KeyAbort,
    Input::KeySHelp,
    Input::KeyLHelp,
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
];

pub const ALL_MOUSE_EVENTS: mmask_t = 0x1fffffff;

pub const BUTTON1_RELEASED: mmask_t = 0x00000001;
pub const BUTTON1_PRESSED: mmask_t = 0x00000002;
pub const BUTTON1_CLICKED: mmask_t = 0x00000004;
pub const BUTTON1_DOUBLE_CLICKED: mmask_t = 0x00000008;
pub const BUTTON1_TRIPLE_CLICKED: mmask_t = 0x00000010;

pub const BUTTON2_RELEASED: mmask_t = 0x00000020;
pub const BUTTON2_PRESSED: mmask_t = 0x00000040;
pub const BUTTON2_CLICKED: mmask_t = 0x00000080;
pub const BUTTON2_DOUBLE_CLICKED: mmask_t = 0x00000100;
pub const BUTTON2_TRIPLE_CLICKED: mmask_t = 0x00000200;

pub const BUTTON3_RELEASED: mmask_t = 0x00000400;
pub const BUTTON3_PRESSED: mmask_t = 0x00000800;
pub const BUTTON3_CLICKED: mmask_t = 0x00001000;
pub const BUTTON3_DOUBLE_CLICKED: mmask_t = 0x00002000;
pub const BUTTON3_TRIPLE_CLICKED: mmask_t = 0x00004000;

pub const BUTTON4_RELEASED: mmask_t = 0x00008000;
pub const BUTTON4_PRESSED: mmask_t = 0x00010000;
pub const BUTTON4_CLICKED: mmask_t = 0x00020000;
pub const BUTTON4_DOUBLE_CLICKED: mmask_t = 0x00040000;
pub const BUTTON4_TRIPLE_CLICKED: mmask_t = 0x00080000;

pub const BUTTON5_RELEASED: mmask_t = 0x00100000;
pub const BUTTON5_PRESSED: mmask_t = 0x00200000;
pub const BUTTON5_CLICKED: mmask_t = 0x00400000;
pub const BUTTON5_DOUBLE_CLICKED: mmask_t = 0x00800000;
pub const BUTTON5_TRIPLE_CLICKED: mmask_t = 0x01000000;

pub const REPORT_MOUSE_POSITION: mmask_t = 0x20000000;
pub const BUTTON_SHIFT: mmask_t = 0x04000000;
pub const BUTTON_CTRL: mmask_t = 0x08000000;
pub const BUTTON_ALT: mmask_t = 0x10000000;

pub fn ACS_LRCORNER() -> chtype {
    'V' as chtype | A_ALTCHARSET
}
pub fn ACS_URCORNER() -> chtype {
    'W' as chtype | A_ALTCHARSET
}
pub fn ACS_ULCORNER() -> chtype {
    'X' as chtype | A_ALTCHARSET
}
pub fn ACS_LLCORNER() -> chtype {
    'Y' as chtype | A_ALTCHARSET
}
pub fn ACS_PLUS() -> chtype {
    'Z' as chtype | A_ALTCHARSET
}
pub fn ACS_LTEE() -> chtype {
    '[' as chtype | A_ALTCHARSET
}
pub fn ACS_RTEE() -> chtype {
    '\\' as chtype | A_ALTCHARSET
}
pub fn ACS_BTEE() -> chtype {
    ']' as chtype | A_ALTCHARSET
}
pub fn ACS_TTEE() -> chtype {
    '^' as chtype | A_ALTCHARSET
}
pub fn ACS_HLINE() -> chtype {
    '_' as chtype | A_ALTCHARSET
}
pub fn ACS_VLINE() -> chtype {
    '`' as chtype | A_ALTCHARSET
}

pub fn ACS_S1() -> chtype {
    'l' as chtype | A_ALTCHARSET
}
pub fn ACS_S9() -> chtype {
    'o' as chtype | A_ALTCHARSET
}
pub fn ACS_DIAMOND() -> chtype {
    'j' as chtype | A_ALTCHARSET
}
pub fn ACS_CKBOARD() -> chtype {
    'k' as chtype | A_ALTCHARSET
}
pub fn ACS_DEGREE() -> chtype {
    'w' as chtype | A_ALTCHARSET
}
pub fn ACS_PLMINUS() -> chtype {
    'x' as chtype | A_ALTCHARSET
}
pub fn ACS_BULLET() -> chtype {
    'h' as chtype | A_ALTCHARSET
}

pub fn ACS_LARROW() -> chtype {
    '!' as chtype | A_ALTCHARSET
}
pub fn ACS_RARROW() -> chtype {
    ' ' as chtype | A_ALTCHARSET
}
pub fn ACS_DARROW() -> chtype {
    '#' as chtype | A_ALTCHARSET
}
pub fn ACS_UARROW() -> chtype {
    '"' as chtype | A_ALTCHARSET
}
pub fn ACS_BOARD() -> chtype {
    '+' as chtype | A_ALTCHARSET
}
pub fn ACS_LANTERN() -> chtype {
    'z' as chtype | A_ALTCHARSET
}
pub fn ACS_BLOCK() -> chtype {
    't' as chtype | A_ALTCHARSET
}

pub fn ACS_S3() -> chtype {
    'm' as chtype | A_ALTCHARSET
}
pub fn ACS_S7() -> chtype {
    'n' as chtype | A_ALTCHARSET
}
pub fn ACS_LEQUAL() -> chtype {
    'u' as chtype | A_ALTCHARSET
}
pub fn ACS_GEQUAL() -> chtype {
    'v' as chtype | A_ALTCHARSET
}
pub fn ACS_PI() -> chtype {
    '$' as chtype | A_ALTCHARSET
}
pub fn ACS_NEQUAL() -> chtype {
    '%' as chtype | A_ALTCHARSET
}
pub fn ACS_STERLING() -> chtype {
    '~' as chtype | A_ALTCHARSET
}

pub fn ACS_BSSB() -> chtype {
    ACS_ULCORNER()
}
pub fn ACS_SSBB() -> chtype {
    ACS_LLCORNER()
}
pub fn ACS_BBSS() -> chtype {
    ACS_URCORNER()
}
pub fn ACS_SBBS() -> chtype {
    ACS_LRCORNER()
}
pub fn ACS_SBSS() -> chtype {
    ACS_RTEE()
}
pub fn ACS_SSSB() -> chtype {
    ACS_LTEE()
}
pub fn ACS_SSBS() -> chtype {
    ACS_BTEE()
}
pub fn ACS_BSSS() -> chtype {
    ACS_TTEE()
}
pub fn ACS_BSBS() -> chtype {
    ACS_HLINE()
}
pub fn ACS_SBSB() -> chtype {
    ACS_VLINE()
}
pub fn ACS_SSSS() -> chtype {
    ACS_PLUS()
}
