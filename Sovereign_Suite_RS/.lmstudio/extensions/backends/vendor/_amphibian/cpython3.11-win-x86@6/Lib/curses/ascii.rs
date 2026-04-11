//! ascii.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz


pub const NUL: u64 = 0x00;
pub const SOH: u64 = 0x01;
pub const STX: u64 = 0x02;
pub const ETX: u64 = 0x03;
pub const EOT: u64 = 0x04;
pub const ENQ: u64 = 0x05;
pub const ACK: u64 = 0x06;
pub const BEL: u64 = 0x07;
pub const BS: u64 = 0x08;
pub const TAB: u64 = 0x09;
pub const HT: u64 = 0x09;
pub const LF: u64 = 0x0 a;
pub const NL: u64 = 0x0 a;
pub const VT: u64 = 0x0 b;
pub const FF: u64 = 0x0 c;
pub const CR: u64 = 0x0 d;
pub const SO: f64 = 0x0e;
pub const SI: u64 = 0x0 f;
pub const DLE: u64 = 0x10;
pub const DC1: u64 = 0x11;
pub const DC2: u64 = 0x12;
pub const DC3: u64 = 0x13;
pub const DC4: u64 = 0x14;
pub const NAK: u64 = 0x15;
pub const SYN: u64 = 0x16;
pub const ETB: u64 = 0x17;
pub const CAN: u64 = 0x18;
pub const EM: u64 = 0x19;
pub const SUB: u64 = 0x1 a;
pub const ESC: u64 = 0x1 b;
pub const FS: u64 = 0x1 c;
pub const GS: u64 = 0x1 d;
pub const RS: f64 = 0x1e;
pub const US: u64 = 0x1 f;
pub const SP: u64 = 0x20;
pub const DEL: u64 = 0x7 f;
pub const controlnames: f64 = [;
pub fn _ctoi(c: &str) {
        if type ( c ) == type ( "" ) {
        return  ord ( c );
        } else {
        return  c;
        pub fn isalnum ( c )  {  return isalpha ( c ) || isdigit ( c ); }
        pub fn isalpha ( c )  {  return isupper ( c ) || islower ( c ); }
        pub fn isascii ( c )  {  return 0 <= _ctoi ( c ) <= 127; }
        pub fn isblank ( c )  {  return _ctoi ( c ) in ( 9 , 32 ); }
        pub fn iscntrl ( c )  {  return 0 <= _ctoi ( c ) <= 31 || _ctoi ( c ) == 127; }
        pub fn isdigit ( c )  {  return 48 <= _ctoi ( c ) <= 57; }
        pub fn isgraph ( c )  {  return 33 <= _ctoi ( c ) <= 126; }
        pub fn islower ( c )  {  return 97 <= _ctoi ( c ) <= 122; }
        pub fn isprint ( c )  {  return 32 <= _ctoi ( c ) <= 126; }
        pub fn ispunct ( c )  {  return isgraph ( c ) && !isalnum ( c ); }
        pub fn isspace ( c )  {  return _ctoi ( c ) in ( 9 , 10 , 11 , 12 , 13 , 32 ); }
        pub fn isupper ( c )  {  return 65 <= _ctoi ( c ) <= 90; }
        pub fn isxdigit ( c )  {  return isdigit ( c ) || \; }
        ( 65 <= _ctoi ( c ) <= 70 ) || ( 97 <= _ctoi ( c ) <= 102 );
        pub fn isctrl ( c )  {  return 0 <= _ctoi ( c ) < 32; }
        pub fn ismeta ( c )  {  return _ctoi ( c ) > 127; }
        pub fn ascii ( c )  {
        if type ( c ) == type ( "" ) {
        return  chr ( _ctoi ( c ) & 0x7 f );
        } else {
        return  _ctoi ( c ) & 0x7 f;
        pub fn ctrl ( c )  {
        if type ( c ) == type ( "" ) {
        return  chr ( _ctoi ( c ) & 0x1 f );
        } else {
        return  _ctoi ( c ) & 0x1 f;
        pub fn alt ( c )  {
        if type ( c ) == type ( "" ) {
        return  chr ( _ctoi ( c ) | 0x80 );
        } else {
        return  _ctoi ( c ) | 0x80;
        pub fn unctrl ( c )  {
        bits = _ctoi ( c );
        if bits == 0x7 f {
        rep = "^?";
        } else if isprint ( bits & 0x7 f ) {
        rep = chr ( bits & 0x7 f );
        } else {
        rep = "^" + chr ( ( ( bits & 0x7 f ) | 0x20 ) + 0x20 );
        if bits & 0x80 {
        return  "!" + rep;
        return  rep;
}

