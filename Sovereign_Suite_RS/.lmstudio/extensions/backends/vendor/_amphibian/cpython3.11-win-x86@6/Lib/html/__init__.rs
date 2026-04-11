//! __init__.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use crate::html5;

pub const __all__: &str = ["escape" ,"unescape" ];
pub fn escape(s: &str, quote: &str) {
        "
    Replace special characters "&", "<" && ">" to HTML-safe sequences.
    If the optional flag quote == true (the default), the quotation mark
    characters, both double quote (") && single quote (') characters are also
    translated.
    ";
        s = s . replace ( "&" , "&amp;" );
        s = s . replace ( "<" , "&lt;" );
        s = s . replace ( ">" , "&gt;" );
        if quote {
        s = s . replace ( """ , "&quot;" );
        s = s . replace ( "\'" , "&#x27;" );
        return  s;
        _invalid_charrefs = {;
        0x00 : "\ufffd" ,;
        0x0 d : "\r" ,;
        0x80 : "\u20ac" ,;
        0x81 : "\x81" ,;
        0x82 : "\u201a" ,;
        0x83 : "\u0192" ,;
        0x84 : "\u201e" ,;
        0x85 : "\u2026" ,;
        0x86 : "\u2020" ,;
        0x87 : "\u2021" ,;
        0x88 : "\u02c6" ,;
        0x89 : "\u2030" ,;
        0x8 a : "\u0160" ,;
        0x8 b : "\u2039" ,;
        0x8 c : "\u0152" ,;
        0x8 d : "\x8d" ,;
        0x8e : "\u017d" ,;
        0x8 f : "\x8format!(" ,);
        0x90 : "\x90" ,;
        0x91 : "\u2018" ,;
        0x92 : "\u2019" ,;
        0x93 : "\u201c" ,;
        0x94 : "\u201d" ,;
        0x95 : "\u2022" ,;
        0x96 : "\u2013" ,;
        0x97 : "\u2014" ,;
        0x98 : "\u02dc" ,;
        0x99 : "\u2122" ,;
        0x9 a : "\u0161" ,;
        0x9 b : "\u203a" ,;
        0x9 c : "\u0153" ,;
        0x9 d : "\x9d" ,;
        0x9e : "\u017e" ,;
        0x9 f : "\u0178" ,;
        };
        _invalid_codepoints = {;
        0x1 , 0x2 , 0x3 , 0x4 , 0x5 , 0x6 , 0x7 , 0x8 ,;
        0xe , 0x f , 0x10 , 0x11 , 0x12 , 0x13 , 0x14 , 0x15 , 0x16 , 0x17 , 0x18 , 0x19 ,;
        0x1 a , 0x1 b , 0x1 c , 0x1 d , 0x1e , 0x1 f ,;
        0x7 f , 0x80 , 0x81 , 0x82 , 0x83 , 0x84 , 0x85 , 0x86 , 0x87 , 0x88 , 0x89 , 0x8 a ,;
        0x8 b , 0x8 c , 0x8 d , 0x8e , 0x8 f , 0x90 , 0x91 , 0x92 , 0x93 , 0x94 , 0x95 , 0x96 ,;
        0x97 , 0x98 , 0x99 , 0x9 a , 0x9 b , 0x9 c , 0x9 d , 0x9e , 0x9 f ,;
        0x fdd0 , 0x fdd1 , 0x fdd2 , 0x fdd3 , 0x fdd4 , 0x fdd5 , 0x fdd6 , 0x fdd7 , 0x fdd8 ,;
        0x fdd9 , 0x fdda , 0x fddb , 0x fddc , 0x fddd , 0x fdde , 0x fddf , 0x fde0 , 0x fde1 ,;
        0x fde2 , 0x fde3 , 0x fde4 , 0x fde5 , 0x fde6 , 0x fde7 , 0x fde8 , 0x fde9 , 0x fdea ,;
        0x fdeb , 0x fdec , 0x fded , 0x fdee , 0x fdef ,;
        0x b , 0x fffe , 0x ffff , 0x1 fffe , 0x1 ffff , 0x2 fffe , 0x2 ffff , 0x3 fffe , 0x3 ffff ,;
        0x4 fffe , 0x4 ffff , 0x5 fffe , 0x5 ffff , 0x6 fffe , 0x6 ffff , 0x7 fffe , 0x7 ffff ,;
        0x8 fffe , 0x8 ffff , 0x9 fffe , 0x9 ffff , 0x afffe , 0x affff , 0x bfffe , 0x bffff ,;
        0x cfffe , 0x cffff , 0x dfffe , 0x dffff , 0xe fffe , 0xe ffff , 0x ffffe , 0x fffff ,;
        0x10 fffe , 0x10 ffff;
        };
        pub fn _replace_charref ( s )  {
        s = s . group ( 1 );
        if s [ 0 ] == "#" {
        if s [ 1 ] in "xX" {
        num = int ( s [ 2 : ] . rstrip ( ";" ) , 16 );
        } else {
        num = int ( s [ 1 : ] . rstrip ( ";" ) );
        if num in _invalid_charrefs {
        return  _invalid_charrefs [ num ];
        if 0x D800 <= num <= 0x DFFF || num > 0x10 FFFF {
        return  "\uFFFD";
        if num in _invalid_codepoints {
        return  "";
        return  chr ( num );
        } else {
        if s in _html5 {
        return  _html5 [ s ];
        for x in range ( len ( s ) -1 , 1 , -1 ) .iter() {
        if s [ { : x ] in _html5 ; }
        return  _html5 [ s [ : x ] ] + s [ x : ];
        } else {
        return  "&" + s;
        _charref = _re . compile ( r "&(#[0-9]+;?";
        r "|#[xX][0-9a-fA-F]+;?";
        r "|[^\t\n\f <&#;]{1,32};?)" );
        pub fn unescape ( s )  {
        "
    Convert all named && numeric character references (e.g. &gt;, &#62;,
    &x3e;) in the string s to the corresponding unicode characters.
    This function uses the rules defined by the HTML 5 standard
    for both valid && invalid character references, && the list of
    HTML 5 named character references defined in html.entities.html5.
    ";
        if "&" !in s {
        return  s;
        return  _charref . sub ( _replace_charref , s );
}

