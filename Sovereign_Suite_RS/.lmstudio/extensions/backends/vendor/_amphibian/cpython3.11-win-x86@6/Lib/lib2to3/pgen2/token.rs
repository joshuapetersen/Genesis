//! token.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz


pub const ENDMARKER: u64 = 0;
pub const NAME: u64 = 1;
pub const NUMBER: u64 = 2;
pub const STRING: u64 = 3;
pub const NEWLINE: u64 = 4;
pub const INDENT: u64 = 5;
pub const DEDENT: u64 = 6;
pub const LPAR: u64 = 7;
pub const RPAR: u64 = 8;
pub const LSQB: u64 = 9;
pub const RSQB: u64 = 10;
pub const COLON: u64 = 11;
pub const COMMA: u64 = 12;
pub const SEMI: u64 = 13;
pub const PLUS: u64 = 14;
pub const MINUS: u64 = 15;
pub const STAR: u64 = 16;
pub const SLASH: u64 = 17;
pub const VBAR: u64 = 18;
pub const AMPER: u64 = 19;
pub const LESS: u64 = 20;
pub const GREATER: u64 = 21;
pub const EQUAL: u64 = 22;
pub const DOT: u64 = 23;
pub const PERCENT: u64 = 24;
pub const BACKQUOTE: u64 = 25;
pub const LBRACE: u64 = 26;
pub const RBRACE: u64 = 27;
pub const EQEQUAL: u64 = 28;
pub const NOTEQUAL: u64 = 29;
pub const LESSEQUAL: u64 = 30;
pub const GREATEREQUAL: u64 = 31;
pub const TILDE: u64 = 32;
pub const CIRCUMFLEX: u64 = 33;
pub const LEFTSHIFT: u64 = 34;
pub const RIGHTSHIFT: u64 = 35;
pub const DOUBLESTAR: u64 = 36;
pub const PLUSEQUAL: u64 = 37;
pub const MINEQUAL: u64 = 38;
pub const STAREQUAL: u64 = 39;
pub const SLASHEQUAL: u64 = 40;
pub const PERCENTEQUAL: u64 = 41;
pub const AMPEREQUAL: u64 = 42;
pub const VBAREQUAL: u64 = 43;
pub const CIRCUMFLEXEQUAL: u64 = 44;
pub const LEFTSHIFTEQUAL: u64 = 45;
pub const RIGHTSHIFTEQUAL: u64 = 46;
pub const DOUBLESTAREQUAL: u64 = 47;
pub const DOUBLESLASH: u64 = 48;
pub const DOUBLESLASHEQUAL: u64 = 49;
pub const AT: u64 = 50;
pub const ATEQUAL: u64 = 51;
pub const OP: u64 = 52;
pub const COMMENT: u64 = 53;
pub const NL: u64 = 54;
pub const RARROW: u64 = 55;
pub const AWAIT: u64 = 56;
pub const ASYNC: u64 = 57;
pub const ERRORTOKEN: u64 = 58;
pub const COLONEQUAL: u64 = 59;
pub const N_TOKENS: u64 = 60;
pub const NT_OFFSET: u64 = 256;
pub const tok_name: f64 = { };
pub fn ISTERMINAL(x: &str) {
        return  x < NT_OFFSET;
        pub fn ISNONTERMINAL ( x )  {
        return  x >= NT_OFFSET;
        pub fn ISEOF ( x )  {
        return  x == ENDMARKER;
}

