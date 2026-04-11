//! _parser.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::__future__::{annotations};
// use std::collections::{Iterable};
// use crate::string;
// use crate::MappingProxyType;
// use crate::Any;
// use crate::.::{};

pub const ASCII_CTRL: f64 = frozenset ( chr ( i ) for i in range ( 32 ) ) | frozenset ( chr ( 127 ) );
pub const ILLEGAL_BASIC_STR_CHARS: &str = ASCII_CTRL - frozenset ("\t" );
pub const ILLEGAL_MULTILINE_BASIC_STR_CHARS: &str = ASCII_CTRL - frozenset ("\t\n" );
pub const ILLEGAL_LITERAL_STR_CHARS: /* inferred */ = ILLEGAL_BASIC_STR_CHARS;
pub const ILLEGAL_MULTILINE_LITERAL_STR_CHARS: /* inferred */ = ILLEGAL_MULTILINE_BASIC_STR_CHARS;
pub const ILLEGAL_COMMENT_CHARS: /* inferred */ = ILLEGAL_BASIC_STR_CHARS;
pub const TOML_WS: &str = frozenset (" \t" );
pub const TOML_WS_AND_NEWLINE: &str = TOML_WS | frozenset ("\n" );
pub const BARE_KEY_CHARS: &str = frozenset ( string . ascii_letters + string . digits +"-_" );
pub const KEY_INITIAL_CHARS: &str = BARE_KEY_CHARS | frozenset ("\"'" );
pub const HEXDIGIT_CHARS: f64 = frozenset ( string . hexdigits );
pub const BASIC_STR_ESCAPE_REPLACEMENTS: f64 = MappingProxyType (;
pub struct TOMLDecodeError {
}

impl TOMLDecodeError {
}

pub fn load(fp: &str, BinaryIO: &str, parse_float: &str, ParseFloat: &str, float: &str) {
        "Parse TOML from a binary file object.";
        b = fp . read ( );
        // try {
        s = b . decode ( );
        // } catch  AttributeError  {
        panic!("TypeError (");
        "File must be opened in binary mode, e.g. use `open('foo.toml', 'rb')`";
        ) from None /* Option */;
        return  loads ( s , parse_float = parse_float );
        pub fn loads ( s  {  str , / , * , parse_float : ParseFloat = float ) - > dict [ str , Any ] ; }
        "Parse TOML from a string.";
        src = s . replace ( "\r\n" , "\n" );
        pos = 0;
        out = Output ( NestedDict ( ) , Flags ( ) );
        header : Key = ( );
        parse_float = make_safe_parse_float ( parse_float );
        while true  {
        pos = skip_chars ( src , pos , TOML_WS );
        // try {
        char = src [ pos ];
        // } catch  IndexError  {
        break;
        if char == "\n" {
        pos + = 1;
        continue;
        if char in KEY_INITIAL_CHARS {
        pos = key_value_rule ( src , pos , out , header , parse_float );
        pos = skip_chars ( src , pos , TOML_WS );
        } else if char == "[" {
        // try {
        second_char : str | None /* Option */ = src [ pos + 1 ];
        // } catch  IndexError  {
        second_char = None /* Option */;
        out . flags . finalize_pending ( );
        if second_char == "[" {
        pos , header = create_list_rule ( src , pos , out );
        } else {
        pos , header = create_dict_rule ( src , pos , out );
        pos = skip_chars ( src , pos , TOML_WS );
        } else if char != "#" {
        panic!("suffixed_err ( src , pos , "Invalid statement" )");
        pos = skip_comment ( src , pos );
        // try {
        char = src [ pos ];
        // } catch  IndexError  {
        break;
        if char != "\n" {
        panic!("suffixed_err (");
        src , pos , "Expected newline || end of document after a statement";
        );
        pos + = 1;
        return  out . data . dict;
        class Flags ;
        "Flags that map to parsed keys/namespaces.";
        FROZEN = 0;
        EXPLICIT_NEST = 1;
        pub fn __init__ ( self ) - > None /* Option */  {
        self . _flags : dict [ str , dict ] = { };
        self . _pending_flags : set [ tuple [ Key , int ] ] = set ( );
        pub fn add_pending ( &self, key  {  Key , flag : int ) - > None /* Option */ /* Option */ ; }
        self . _pending_flags . add ( ( key , flag ) );
        pub fn finalize_pending ( self ) - > None /* Option */  {
        for key , flag in self . _pending_flags .iter() {
        self . set ( key , flag , recursive = false );
        self . _pending_flags . clear ( );
        pub fn unset_all ( &self, key  {  Key ) - > None /* Option */ /* Option */ ; }
        cont = self . _flags;
        for k in key [ : -1 ] .iter() {
        if k !in cont {
        return;
        cont = cont [ k ] [ "nested" ];
        cont . pop ( key [ -1 ] , None /* Option */ );
        pub fn set ( &self, key  {  Key , flag : int , * , recursive : bool ) - > None /* Option */ /* Option */ ; }
        cont = self . _flags;
        key_parent , key_stem = key [ : -1 ] , key [ -1 ];
        for k in key_parent .iter() {
        if k !in cont {
        cont [ k ] = { "flags" : set ( ) , "recursive_flags" : set ( ) , "nested" : { } };
        cont = cont [ k ] [ "nested" ];
        if key_stem !in cont {
        cont [ key_stem ] = { "flags" : set ( ) , "recursive_flags" : set ( ) , "nested" : { } };
        cont [ key_stem ] [ "recursive_flags" if recursive else "flags" ] . add ( flag );
        pub fn is_ ( &self, key  {  Key , flag : int ) - > bool ; }
        if !key {
        return  false;
        cont = self . _flags;
        for k in key [ : -1 ] .iter() {
        if k !in cont {
        return  false;
        inner_cont = cont [ k ];
        if flag in inner_cont [ "recursive_flags" ] {
        return  true;
        cont = inner_cont [ "nested" ];
        key_stem = key [ -1 ];
        if key_stem in cont {
        cont = cont [ key_stem ];
        return  flag in cont [ "flags" ] || flag in cont [ "recursive_flags" ];
        return  false;
        class NestedDict ;
        pub fn __init__ ( self ) - > None /* Option */  {
        self . dict : dict [ str , Any ] = { };
        pub fn get_or_create_nest ( {
        self ,;
        key : Key ,;
        * ,;
        access_lists : bool = true ,;
        ) - > dict ;
        cont : Any = self . dict;
        for k in key .iter() {
        if k !in cont {
        cont [ k ] = { };
        cont = cont [ k ];
        if access_lists && isinstance ( cont , list ) {
        cont = cont [ -1 ];
        if !isinstance ( cont , dict ) {
        panic!("KeyError ( "There is no nest behind this key" )");
        return  cont;
        pub fn append_nest_to_list ( &self, key  {  Key ) - > None /* Option */ /* Option */ ; }
        cont = self . get_or_create_nest ( key [ : -1 ] );
        last_key = key [ -1 ];
        if last_key in cont {
        list_ = cont [ last_key ];
        if !isinstance ( list_ , list ) {
        panic!("KeyError ( "An object other than list found behind this key" )");
        list_ . append ( { } );
        } else {
        cont [ last_key ] = [ { } ];
        class Output ( NamedTuple ) ;
        data : NestedDict;
        flags : Flags;
        pub fn skip_chars ( src  {  str , pos : Pos , chars : Iterable [ str ] ) - > Pos ; }
        // try {
        while src [ pos ] in chars  {
        pos + = 1;
        // } catch  IndexError  {
        // pass
        return  pos;
        pub fn skip_until ( {
        src : str ,;
        pos : Pos ,;
        expect : str ,;
        * ,;
        error_on : frozenset [ str ] ,;
        error_on_eof : bool ,;
        ) - > Pos ;
        // try {
        new_pos = src . index ( expect , pos );
        // } catch  ValueError  {
        new_pos = len ( src );
        if error_on_eof {
        panic!("suffixed_err ( src , new_pos , f "Expected {expect!r}" ) from None /* Option */");
        if !error_on . isdisjoint ( src [ pos { : new_pos ] ) ; }
        while src [ pos ] !in error_on  {
        pos + = 1;
        panic!("suffixed_err ( src , pos , f "Found invalid character {src[pos]!r}" )");
        return  new_pos;
        pub fn skip_comment ( src  {  str , pos : Pos ) - > Pos ; }
        // try {
        char : str | None /* Option */ = src [ pos ];
        // } catch  IndexError  {
        char = None /* Option */;
        if char == "#" {
        return  skip_until (;
        src , pos + 1 , "\n" , error_on = ILLEGAL_COMMENT_CHARS , error_on_eof = false;
        );
        return  pos;
        pub fn skip_comments_and_array_ws ( src  {  str , pos : Pos ) - > Pos ; }
        while true  {
        pos_before_skip = pos;
        pos = skip_chars ( src , pos , TOML_WS_AND_NEWLINE );
        pos = skip_comment ( src , pos );
        if pos == pos_before_skip {
        return  pos;
        pub fn create_dict_rule ( src  {  str , pos : Pos , out : Output ) - > tuple [ Pos , Key ] ; }
        pos + = 1;
        pos = skip_chars ( src , pos , TOML_WS );
        pos , key = parse_key ( src , pos );
        if out . flags . is_ ( key , Flags . EXPLICIT_NEST ) || out . flags . is_ ( key , Flags . FROZEN ) {
        panic!("suffixed_err ( src , pos , f "Cannot declare {key} twice" )");
        out . flags . set ( key , Flags . EXPLICIT_NEST , recursive = false );
        // try {
        out . data . get_or_create_nest ( key );
        // } catch  KeyError  {
        panic!("suffixed_err ( src , pos , "Cannot overwrite a value" ) from None /* Option */");
        if !src . startswith ( "]" , pos ) {
        panic!("suffixed_err ( src , pos , "Expected ']' at the end of a table declaration" )");
        return  pos + 1 , key;
        pub fn create_list_rule ( src  {  str , pos : Pos , out : Output ) - > tuple [ Pos , Key ] ; }
        pos + = 2;
        pos = skip_chars ( src , pos , TOML_WS );
        pos , key = parse_key ( src , pos );
        if out . flags . is_ ( key , Flags . FROZEN ) {
        panic!("suffixed_err ( src , pos , f "Cannot mutate immutable namespace {key}" )");
        out . flags . unset_all ( key );
        out . flags . set ( key , Flags . EXPLICIT_NEST , recursive = false );
        // try {
        out . data . append_nest_to_list ( key );
        // } catch  KeyError  {
        panic!("suffixed_err ( src , pos , "Cannot overwrite a value" ) from None /* Option */");
        if !src . startswith ( "]]" , pos ) {
        panic!("suffixed_err ( src , pos , "Expected ']]' at the end of an array declaration" )");
        return  pos + 2 , key;
        pub fn key_value_rule ( {
        src : str , pos : Pos , out : Output , header : Key , parse_float : ParseFloat;
        ) - > Pos ;
        pos , key , value = parse_key_value_pair ( src , pos , parse_float );
        key_parent , key_stem = key [ : -1 ] , key [ -1 ];
        abs_key_parent = header + key_parent;
        relative_path_cont_keys = ( header + key vec![ : i ].iter().map(|i| range ( 1 , len ( key ) ) );
        for cont_key in relative_path_cont_keys .iter() {
        if out . flags . is_ ( cont_key , Flags . EXPLICIT_NEST ) {
        panic!("suffixed_err ( src , pos , f "Cannot redefine namespace {cont_key}" )");
        out . flags . add_pending ( cont_key , Flags . EXPLICIT_NEST );
        if out . flags . is_ ( abs_key_parent , Flags . FROZEN ) {
        panic!("suffixed_err (");
        src , pos , format!("Cannot mutate immutable namespace {abs_key_parent}");
        );
        // try {
        nest = out . data . get_or_create_nest ( abs_key_parent );
        // } catch  KeyError  {
        panic!("suffixed_err ( src , pos , "Cannot overwrite a value" ) from None /* Option */");
        if key_stem in nest {
        panic!("suffixed_err ( src , pos , "Cannot overwrite a value" )");
        if isinstance ( value , ( dict , list ) ) {
        out . flags . set ( header + key , Flags . FROZEN , recursive = true );
        nest [ key_stem ] = value;
        return  pos;
        pub fn parse_key_value_pair ( {
        src : str , pos : Pos , parse_float : ParseFloat;
        ) - > tuple [ Pos , Key , Any ] ;
        pos , key = parse_key ( src , pos );
        // try {
        char : str | None /* Option */ = src [ pos ];
        // } catch  IndexError  {
        char = None /* Option */;
        if char != "=" {
        panic!("suffixed_err ( src , pos , "Expected '=' after a key in a key/value pair" )");
        pos + = 1;
        pos = skip_chars ( src , pos , TOML_WS );
        pos , value = parse_value ( src , pos , parse_float );
        return  pos , key , value;
        pub fn parse_key ( src  {  str , pos : Pos ) - > tuple [ Pos , Key ] ; }
        pos , key_part = parse_key_part ( src , pos );
        key : Key = ( key_part , );
        pos = skip_chars ( src , pos , TOML_WS );
        while true  {
        // try {
        char : str | None /* Option */ = src [ pos ];
        // } catch  IndexError  {
        char = None /* Option */;
        if char != "." {
        return  pos , key;
        pos + = 1;
        pos = skip_chars ( src , pos , TOML_WS );
        pos , key_part = parse_key_part ( src , pos );
        key + = ( key_part , );
        pos = skip_chars ( src , pos , TOML_WS );
        pub fn parse_key_part ( src  {  str , pos : Pos ) - > tuple [ Pos , str ] ; }
        // try {
        char : str | None /* Option */ = src [ pos ];
        // } catch  IndexError  {
        char = None /* Option */;
        if char in BARE_KEY_CHARS {
        start_pos = pos;
        pos = skip_chars ( src , pos , BARE_KEY_CHARS );
        return  pos , src [ start_pos : pos ];
        if char == "'" {
        return  parse_literal_str ( src , pos );
        if char == """ {
        return  parse_one_line_basic_str ( src , pos );
        panic!("suffixed_err ( src , pos , "Invalid initial character for a key part" )");
        pub fn parse_one_line_basic_str ( src  {  str , pos : Pos ) - > tuple [ Pos , str ] ; }
        pos + = 1;
        return  parse_basic_str ( src , pos , multiline = false );
        pub fn parse_array ( src  {  str , pos : Pos , parse_float : ParseFloat ) - > tuple [ Pos , list ] ; }
        pos + = 1;
        array : list = [ ];
        pos = skip_comments_and_array_ws ( src , pos );
        if src . startswith ( "]" , pos ) {
        return  pos + 1 , array;
        while true  {
        pos , val = parse_value ( src , pos , parse_float );
        array . append ( val );
        pos = skip_comments_and_array_ws ( src , pos );
        c = src [ pos : pos + 1 ];
        if c == "]" {
        return  pos + 1 , array;
        if c != "," {
        panic!("suffixed_err ( src , pos , "Unclosed array" )");
        pos + = 1;
        pos = skip_comments_and_array_ws ( src , pos );
        if src . startswith ( "]" , pos ) {
        return  pos + 1 , array;
        pub fn parse_inline_table ( src  {  str , pos : Pos , parse_float : ParseFloat ) - > tuple [ Pos , dict ] ; }
        pos + = 1;
        nested_dict = NestedDict ( );
        flags = Flags ( );
        pos = skip_chars ( src , pos , TOML_WS );
        if src . startswith ( "}" , pos ) {
        return  pos + 1 , nested_dict . dict;
        while true  {
        pos , key , value = parse_key_value_pair ( src , pos , parse_float );
        key_parent , key_stem = key [ : -1 ] , key [ -1 ];
        if flags . is_ ( key , Flags . FROZEN ) {
        panic!("suffixed_err ( src , pos , f "Cannot mutate immutable namespace {key}" )");
        // try {
        nest = nested_dict . get_or_create_nest ( key_parent , access_lists = false );
        // } catch  KeyError  {
        panic!("suffixed_err ( src , pos , "Cannot overwrite a value" ) from None /* Option */");
        if key_stem in nest {
        panic!("suffixed_err ( src , pos , f "Duplicate inline table key {key_stem!r}" )");
        nest [ key_stem ] = value;
        pos = skip_chars ( src , pos , TOML_WS );
        c = src [ pos : pos + 1 ];
        if c == "}" {
        return  pos + 1 , nested_dict . dict;
        if c != "," {
        panic!("suffixed_err ( src , pos , "Unclosed inline table" )");
        if isinstance ( value , ( dict , list ) ) {
        flags . set ( key , Flags . FROZEN , recursive = true );
        pos + = 1;
        pos = skip_chars ( src , pos , TOML_WS );
        pub fn parse_basic_str_escape ( {
        src : str , pos : Pos , * , multiline : bool = false;
        ) - > tuple [ Pos , str ] ;
        escape_id = src [ pos : pos + 2 ];
        pos + = 2;
        if multiline && escape_id in { "\\ " , "\\\t" , "\\\n" } {
        if escape_id != "\\\n" {
        pos = skip_chars ( src , pos , TOML_WS );
        // try {
        char = src [ pos ];
        // } catch  IndexError  {
        return  pos , "";
        if char != "\n" {
        panic!("suffixed_err ( src , pos , "Unescaped '\\' in a string" )");
        pos + = 1;
        pos = skip_chars ( src , pos , TOML_WS_AND_NEWLINE );
        return  pos , "";
        if escape_id == "\\u" {
        return  parse_hex_char ( src , pos , 4 );
        if escape_id == "\\U" {
        return  parse_hex_char ( src , pos , 8 );
        // try {
        return  pos , BASIC_STR_ESCAPE_REPLACEMENTS [ escape_id ];
        // } catch  KeyError  {
        panic!("suffixed_err ( src , pos , "Unescaped '\\' in a string" ) from None /* Option */");
        pub fn parse_basic_str_escape_multiline ( src  {  str , pos : Pos ) - > tuple [ Pos , str ] ; }
        return  parse_basic_str_escape ( src , pos , multiline = true );
        pub fn parse_hex_char ( src  {  str , pos : Pos , hex_len : int ) - > tuple [ Pos , str ] ; }
        hex_str = src [ pos : pos + hex_len ];
        if len ( hex_str ) != hex_len || !HEXDIGIT_CHARS . issuperset ( hex_str ) {
        panic!("suffixed_err ( src , pos , "Invalid hex value" )");
        pos + = hex_len;
        hex_int = int ( hex_str , 16 );
        if !is_unicode_scalar_value ( hex_int ) {
        panic!("suffixed_err ( src , pos , "Escaped character is !a Unicode scalar value" )");
        return  pos , chr ( hex_int );
        pub fn parse_literal_str ( src  {  str , pos : Pos ) - > tuple [ Pos , str ] ; }
        pos + = 1;
        start_pos = pos;
        pos = skip_until (;
        src , pos , "'" , error_on = ILLEGAL_LITERAL_STR_CHARS , error_on_eof = true;
        );
        return  pos + 1 , src [ start_pos : pos ];
        pub fn parse_multiline_str ( src  {  str , pos : Pos , * , literal : bool ) - > tuple [ Pos , str ] ; }
        pos + = 3;
        if src . startswith ( "\n" , pos ) {
        pos + = 1;
        if literal {
        delim = "'";
        end_pos = skip_until (;
        src ,;
        pos ,;
        "'''" ,;
        error_on = ILLEGAL_MULTILINE_LITERAL_STR_CHARS ,;
        error_on_eof = true ,;
        );
        result = src [ pos : end_pos ];
        pos = end_pos + 3;
        } else {
        delim = """;
        pos , result = parse_basic_str ( src , pos , multiline = true );
        if !src . startswith ( delim , pos ) {
        return  pos , result;
        pos + = 1;
        if !src . startswith ( delim , pos ) {
        return  pos , result + delim;
        pos + = 1;
        return  pos , result + ( delim * 2 );
        pub fn parse_basic_str ( src  {  str , pos : Pos , * , multiline : bool ) - > tuple [ Pos , str ] ; }
        if multiline {
        error_on = ILLEGAL_MULTILINE_BASIC_STR_CHARS;
        parse_escapes = parse_basic_str_escape_multiline;
        } else {
        error_on = ILLEGAL_BASIC_STR_CHARS;
        parse_escapes = parse_basic_str_escape;
        result = "";
        start_pos = pos;
        while true  {
        // try {
        char = src [ pos ];
        // } catch  IndexError  {
        panic!("suffixed_err ( src , pos , "Unterminated string" ) from None /* Option */");
        if char == """ {
        if !multiline {
        return  pos + 1 , result + src [ start_pos : pos ];
        if src . startswith ( """"" , pos ) {
        return  pos + 3 , result + src [ start_pos : pos ];
        pos + = 1;
        continue;
        if char == "\\" {
        result + = src [ start_pos : pos ];
        pos , parsed_escape = parse_escapes ( src , pos );
        result + = parsed_escape;
        start_pos = pos;
        continue;
        if char in error_on {
        panic!("suffixed_err ( src , pos , f "Illegal character {char!r}" )");
        pos + = 1;
        pub fn parse_value ( {
        src : str , pos : Pos , parse_float : ParseFloat;
        ) - > tuple [ Pos , Any ] ;
        // try {
        char : str | None /* Option */ = src [ pos ];
        // } catch  IndexError  {
        char = None /* Option */;
        if char == """ {
        if src . startswith ( """"" , pos ) {
        return  parse_multiline_str ( src , pos , literal = false );
        return  parse_one_line_basic_str ( src , pos );
        if char == "'" {
        if src . startswith ( "'''" , pos ) {
        return  parse_multiline_str ( src , pos , literal = true );
        return  parse_literal_str ( src , pos );
        if char == "t" {
        if src . startswith ( "true" , pos ) {
        return  pos + 4 , true;
        if char == "f" {
        if src . startswith ( "false" , pos ) {
        return  pos + 5 , false;
        if char == "[" {
        return  parse_array ( src , pos , parse_float );
        if char == "{" {
        return  parse_inline_table ( src , pos , parse_float );
        datetime_match = RE_DATETIME . match ( src , pos );
        if datetime_match {
        // try {
        datetime_obj = match_to_datetime ( datetime_match );
        // } catch  ValueError as e  {
        panic!("suffixed_err ( src , pos , "Invalid date || datetime" ) from e");
        return  datetime_match . end ( ) , datetime_obj;
        localtime_match = RE_LOCALTIME . match ( src , pos );
        if localtime_match {
        return  localtime_match . end ( ) , match_to_localtime ( localtime_match );
        number_match = RE_NUMBER . match ( src , pos );
        if number_match {
        return  number_match . end ( ) , match_to_number ( number_match , parse_float );
        first_three = src [ pos : pos + 3 ];
        if first_three in { "inf" , "nan" } {
        return  pos + 3 , parse_float ( first_three );
        first_four = src [ pos : pos + 4 ];
        if first_four in { "-inf" , "+inf" , "-nan" , "+nan" } {
        return  pos + 4 , parse_float ( first_four );
        panic!("suffixed_err ( src , pos , "Invalid value" )");
        pub fn suffixed_err ( src  {  str , pos : Pos , msg : str ) - > TOMLDecodeError ; }
        "Return a `TOMLDecodeError` where error message == suffixed with
    coordinates in source.";
        pub fn coord_repr ( src  {  str , pos : Pos ) - > str ; }
        if pos >= len ( src ) {
        return  "end of document";
        line = src . count ( "\n" , 0 , pos ) + 1;
        if line == 1 {
        column = pos + 1;
        } else {
        column = pos - src . rindex ( "\n" , 0 , pos );
        return  f "line {line}, column {column}";
        return  TOMLDecodeError ( f "{msg} (at {coord_repr(src, pos)})" );
        pub fn is_unicode_scalar_value ( codepoint  {  int ) - > bool ; }
        return  ( 0 <= codepoint <= 55295 ) || ( 57344 <= codepoint <= 1114111 );
        pub fn make_safe_parse_float ( parse_float  {  ParseFloat ) - > ParseFloat ; }
        "A decorator to make `parse_float` safe.

    `parse_float` must !return dicts || lists, because these types
    would be mixed with parsed TOML tables && arrays, thus confusing
    the parser. The returned decorated callable raises `ValueError`
    instead of returning illegal types.
    ";
        if parse_float is float {
        return  float;
        pub fn safe_parse_float ( float_str  {  str ) - > Any ; }
        float_value = parse_float ( float_str );
        if isinstance ( float_value , ( dict , list ) ) {
        panic!("ValueError ( "parse_float must !return dicts || lists" )");
        return  float_value;
        return  safe_parse_float;
}

