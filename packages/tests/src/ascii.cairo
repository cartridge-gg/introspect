pub const NUL: u8 = 0; /// Null character
pub const SOH: u8 = 1; /// Start of Heading
pub const STX: u8 = 2; /// Start of Text
pub const ETX: u8 = 3; /// End of Text
pub const EOT: u8 = 4; /// End of Transmission
pub const ENQ: u8 = 5; /// Enquiry
pub const ACK: u8 = 6; /// Acknowledgment
pub const BEL: u8 = 7; /// Bell
pub const BS: u8 = 8; /// Backspace
pub const TAB: u8 = 9; /// Horizontal Tab
pub const LF: u8 = 10; /// Line Feed
pub const VT: u8 = 11; /// Vertical Tab
pub const FF: u8 = 12; /// Form Feed
pub const CR: u8 = 13; /// Carriage Return
pub const SO: u8 = 14; /// Shift Out
pub const SI: u8 = 15; /// Shift In
pub const DLE: u8 = 16; /// Data Link Escape
pub const DC1: u8 = 17; /// Device Control 1
pub const DC2: u8 = 18; /// Device Control 2
pub const DC3: u8 = 19; /// Device Control 3
pub const DC4: u8 = 20; /// Device Control 4
pub const NAK: u8 = 21; /// Negative Acknowledgment
pub const SYN: u8 = 22; /// Synchronous Idle
pub const ETB: u8 = 23; /// End of Transmission Block
pub const CAN: u8 = 24; /// Cancel
pub const EM: u8 = 25; /// End of Medium
pub const SUB: u8 = 26; /// Substitute
pub const ESC: u8 = 27; /// Escape
pub const FS: u8 = 28; /// File Separator
pub const GS: u8 = 29; /// Group Separator
pub const RS: u8 = 30; /// Record Separator
pub const US: u8 = 31; /// Unit Separator
pub const SPACE: u8 = 32;
pub const EXCLAMATION: u8 = 33;
pub const DOUBLE_QUOTES: u8 = 34;
pub const HASH: u8 = 35;
pub const DOLLAR: u8 = 36;
pub const PERCENT: u8 = 37;
pub const AMPERSAND: u8 = 38;
pub const SINGLE_QUOTE: u8 = 39;
pub const OPEN_PARENTHESIS: u8 = 40;
pub const CLOSE_PARENTHESIS: u8 = 41;
pub const ASTERISK: u8 = 42;
pub const PLUS: u8 = 43;
pub const COMMA: u8 = 44;
pub const HYPHEN: u8 = 45;
pub const PERIOD: u8 = 46;
pub const SLASH: u8 = 47;
pub const _0: u8 = 48;
pub const _1: u8 = 49;
pub const _2: u8 = 50;
pub const _3: u8 = 51;
pub const _4: u8 = 52;
pub const _5: u8 = 53;
pub const _6: u8 = 54;
pub const _7: u8 = 55;
pub const _8: u8 = 56;
pub const _9: u8 = 57;
pub const COLON: u8 = 58;
pub const SEMICOLON: u8 = 59;
pub const LESS_THAN: u8 = 60;
pub const EQUALS: u8 = 61;
pub const GREATER_THAN: u8 = 62;
pub const QUESTION: u8 = 63;
pub const AT: u8 = 64;
pub const _A: u8 = 65;
pub const _B: u8 = 66;
pub const _C: u8 = 67;
pub const _D: u8 = 68;
pub const _E: u8 = 69;
pub const _F: u8 = 70;
pub const _G: u8 = 71;
pub const _H: u8 = 72;
pub const _I: u8 = 73;
pub const _J: u8 = 74;
pub const _K: u8 = 75;
pub const _L: u8 = 76;
pub const _M: u8 = 77;
pub const _N: u8 = 78;
pub const _O: u8 = 79;
pub const _P: u8 = 80;
pub const _Q: u8 = 81;
pub const _R: u8 = 82;
pub const _S: u8 = 83;
pub const _T: u8 = 84;
pub const _U: u8 = 85;
pub const _V: u8 = 86;
pub const _W: u8 = 87;
pub const _X: u8 = 88;
pub const _Y: u8 = 89;
pub const _Z: u8 = 90;
pub const OPEN_BRACKET: u8 = 91;
pub const BACKSLASH: u8 = 92;
pub const CLOSE_BRACKET: u8 = 93;
pub const CARET: u8 = 94;
pub const UNDERSCORE: u8 = 95;
pub const GRAVE: u8 = 96;
pub const _a: u8 = 97;
pub const _b: u8 = 98;
pub const _c: u8 = 99;
pub const _d: u8 = 100;
pub const _e: u8 = 101;
pub const _f: u8 = 102;
pub const _g: u8 = 103;
pub const _h: u8 = 104;
pub const _i: u8 = 105;
pub const _j: u8 = 106;
pub const _k: u8 = 107;
pub const _l: u8 = 108;
pub const _m: u8 = 109;
pub const _n: u8 = 110;
pub const _o: u8 = 111;
pub const _p: u8 = 112;
pub const _q: u8 = 113;
pub const _r: u8 = 114;
pub const _s: u8 = 115;
pub const _t: u8 = 116;
pub const _u: u8 = 117;
pub const _v: u8 = 118;
pub const _w: u8 = 119;
pub const _x: u8 = 120;
pub const _y: u8 = 121;
pub const _z: u8 = 122;
pub const OPEN_BRACE: u8 = 123;
pub const VERTICAL_BAR: u8 = 124;
pub const CLOSE_BRACE: u8 = 125;
pub const TILDE: u8 = 126;
pub const DELETE: u8 = 127;

pub const CHARACTERS: [u8; 128] = [
    NUL, SOH, STX, ETX, EOT, ENQ, ACK, BEL, BS, TAB, LF, VT, FF, CR, SO, SI, DLE, DC1, DC2, DC3,
    DC4, NAK, SYN, ETB, CAN, EM, SUB, ESC, FS, GS, RS, US, SPACE, EXCLAMATION, DOUBLE_QUOTES, HASH,
    DOLLAR, PERCENT, AMPERSAND, SINGLE_QUOTE, OPEN_PARENTHESIS, CLOSE_PARENTHESIS, ASTERISK, PLUS,
    COMMA, HYPHEN, PERIOD, SLASH, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, COLON, SEMICOLON,
    LESS_THAN, EQUALS, GREATER_THAN, QUESTION, AT, _A, _B, _C, _D, _E, _F, _G, _H, _I, _J, _K, _L,
    _M, _N, _O, _P, _Q, _R, _S, _T, _U, _V, _W, _X, _Y, _Z, OPEN_BRACKET, BACKSLASH, CLOSE_BRACKET,
    CARET, UNDERSCORE, GRAVE, _a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l, _m, _n, _o, _p, _q,
    _r, _s, _t, _u, _v, _w, _x, _y, _z, OPEN_BRACE, VERTICAL_BAR, CLOSE_BRACE, TILDE, DELETE,
];

pub const LOWERCASE: [u8; 26] = [
    _a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l, _m, _n, _o, _p, _q, _r, _s, _t, _u, _v, _w, _x,
    _y, _z,
];

pub const UPPERCASE: [u8; 26] = [
    _A, _B, _C, _D, _E, _F, _G, _H, _I, _J, _K, _L, _M, _N, _O, _P, _Q, _R, _S, _T, _U, _V, _W, _X,
    _Y, _Z,
];

pub const LETTERS: [u8; 52] = [
    _A, _B, _C, _D, _E, _F, _G, _H, _I, _J, _K, _L, _M, _N, _O, _P, _Q, _R, _S, _T, _U, _V, _W, _X,
    _Y, _Z, _a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l, _m, _n, _o, _p, _q, _r, _s, _t, _u, _v,
    _w, _x, _y, _z,
];

pub const DIGITS: [u8; 10] = [_0, _1, _2, _3, _4, _5, _6, _7, _8, _9];

pub const ALPHANUMERIC: [u8; 62] = [
    _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _A, _B, _C, _D, _E, _F, _G, _H, _I, _J, _K, _L, _M, _N,
    _O, _P, _Q, _R, _S, _T, _U, _V, _W, _X, _Y, _Z, _a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l,
    _m, _n, _o, _p, _q, _r, _s, _t, _u, _v, _w, _x, _y, _z,
];

pub const PUNCTUATION: [u8; 32] = [
    EXCLAMATION, DOUBLE_QUOTES, HASH, DOLLAR, PERCENT, AMPERSAND, SINGLE_QUOTE, OPEN_PARENTHESIS,
    CLOSE_PARENTHESIS, ASTERISK, PLUS, COMMA, HYPHEN, PERIOD, SLASH, COLON, SEMICOLON, LESS_THAN,
    EQUALS, GREATER_THAN, QUESTION, AT, OPEN_BRACKET, BACKSLASH, CLOSE_BRACKET, CARET, UNDERSCORE,
    GRAVE, OPEN_BRACE, VERTICAL_BAR, CLOSE_BRACE, TILDE,
];

pub const PRINTABLE: [u8; 94] = [
    EXCLAMATION, DOUBLE_QUOTES, HASH, DOLLAR, PERCENT, AMPERSAND, SINGLE_QUOTE, OPEN_PARENTHESIS,
    CLOSE_PARENTHESIS, ASTERISK, PLUS, COMMA, HYPHEN, PERIOD, SLASH, _0, _1, _2, _3, _4, _5, _6, _7,
    _8, _9, COLON, SEMICOLON, LESS_THAN, EQUALS, GREATER_THAN, QUESTION, AT, _A, _B, _C, _D, _E, _F,
    _G, _H, _I, _J, _K, _L, _M, _N, _O, _P, _Q, _R, _S, _T, _U, _V, _W, _X, _Y, _Z, OPEN_BRACKET,
    BACKSLASH, CLOSE_BRACKET, CARET, UNDERSCORE, GRAVE, _a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k,
    _l, _m, _n, _o, _p, _q, _r, _s, _t, _u, _v, _w, _x, _y, _z, OPEN_BRACE, VERTICAL_BAR,
    CLOSE_BRACE, TILDE,
];

pub const GRAPHIC: [u8; 95] = [
    SPACE, EXCLAMATION, DOUBLE_QUOTES, HASH, DOLLAR, PERCENT, AMPERSAND, SINGLE_QUOTE,
    OPEN_PARENTHESIS, CLOSE_PARENTHESIS, ASTERISK, PLUS, COMMA, HYPHEN, PERIOD, SLASH, _0, _1, _2,
    _3, _4, _5, _6, _7, _8, _9, COLON, SEMICOLON, LESS_THAN, EQUALS, GREATER_THAN, QUESTION, AT, _A,
    _B, _C, _D, _E, _F, _G, _H, _I, _J, _K, _L, _M, _N, _O, _P, _Q, _R, _S, _T, _U, _V, _W, _X, _Y,
    _Z, OPEN_BRACKET, BACKSLASH, CLOSE_BRACKET, CARET, UNDERSCORE, GRAVE, _a, _b, _c, _d, _e, _f,
    _g, _h, _i, _j, _k, _l, _m, _n, _o, _p, _q, _r, _s, _t, _u, _v, _w, _x, _y, _z, OPEN_BRACE,
    VERTICAL_BAR, CLOSE_BRACE, TILDE,
];

pub const WHITESPACE: [u8; 6] = [TAB, LF, VT, FF, CR, SPACE];

pub const CAIRO_WHITESPACE: [u8; 5] = [TAB, LF, FF, CR, SPACE];

pub const CAIRO_DISPLAYABLE: [u8; 99] = [
    TAB, LF, FF, CR, SPACE, EXCLAMATION, DOUBLE_QUOTES, HASH, DOLLAR, PERCENT, AMPERSAND,
    SINGLE_QUOTE, OPEN_PARENTHESIS, CLOSE_PARENTHESIS, ASTERISK, PLUS, COMMA, HYPHEN, PERIOD, SLASH,
    _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, COLON, SEMICOLON, LESS_THAN, EQUALS, GREATER_THAN,
    QUESTION, AT, _A, _B, _C, _D, _E, _F, _G, _H, _I, _J, _K, _L, _M, _N, _O, _P, _Q, _R, _S, _T,
    _U, _V, _W, _X, _Y, _Z, OPEN_BRACKET, BACKSLASH, CLOSE_BRACKET, CARET, UNDERSCORE, GRAVE, _a,
    _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l, _m, _n, _o, _p, _q, _r, _s, _t, _u, _v, _w, _x, _y,
    _z, OPEN_BRACE, VERTICAL_BAR, CLOSE_BRACE, TILDE,
];

pub const CAIRO_UNDISPLAYABLE: [u8; 29] = [
    NUL, SOH, STX, ETX, EOT, ENQ, ACK, BEL, BS, VT, SO, SI, DLE, DC1, DC2, DC3, DC4, NAK, SYN, ETB,
    CAN, EM, SUB, ESC, FS, GS, RS, US, DELETE,
];

pub const HEX_DIGITS_UPPER: [u8; 16] = [
    _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _A, _B, _C, _D, _E, _F,
];

pub const HEX_DIGITS_LOWER: [u8; 16] = [
    _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _a, _b, _c, _d, _e, _f,
];

pub const OCTAL_DIGITS: [u8; 8] = [_0, _1, _2, _3, _4, _5, _6, _7];

pub const BINARY_DIGITS: [u8; 2] = [_0, _1];

pub const CONTROL: [u8; 33] = [
    NUL, SOH, STX, ETX, EOT, ENQ, ACK, BEL, BS, TAB, LF, VT, FF, CR, SO, SI, DLE, DC1, DC2, DC3,
    DC4, NAK, SYN, ETB, CAN, EM, SUB, ESC, FS, GS, RS, US, DELETE,
];

pub const IDENTIFIER_START: [u8; 53] = [
    UNDERSCORE, _A, _B, _C, _D, _E, _F, _G, _H, _I, _J, _K, _L, _M, _N, _O, _P, _Q, _R, _S, _T, _U,
    _V, _W, _X, _Y, _Z, _a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l, _m, _n, _o, _p, _q, _r, _s,
    _t, _u, _v, _w, _x, _y, _z,
];

pub const IDENTIFIER_CONTINUE: [u8; 63] = [
    UNDERSCORE, _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _A, _B, _C, _D, _E, _F, _G, _H, _I, _J, _K,
    _L, _M, _N, _O, _P, _Q, _R, _S, _T, _U, _V, _W, _X, _Y, _Z, _a, _b, _c, _d, _e, _f, _g, _h, _i,
    _j, _k, _l, _m, _n, _o, _p, _q, _r, _s, _t, _u, _v, _w, _x, _y, _z,
];

pub const BRACKETS: [u8; 6] = [
    OPEN_PARENTHESIS, CLOSE_PARENTHESIS, OPEN_BRACKET, CLOSE_BRACKET, OPEN_BRACE, CLOSE_BRACE,
];

pub const QUOTES: [u8; 3] = [SINGLE_QUOTE, DOUBLE_QUOTES, GRAVE];
