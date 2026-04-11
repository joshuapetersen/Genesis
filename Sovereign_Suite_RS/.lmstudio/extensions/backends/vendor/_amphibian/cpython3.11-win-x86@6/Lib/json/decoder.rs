//! decoder.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use serde_json::{scanner};
// use crate::_json::{scanstring, c_scanstring};

pub const __all__: &str = ["JSONDecoder" ,"JSONDecodeError" ];
pub const FLAGS: f64 = re . VERBOSE | re . MULTILINE | re . DOTALL;
pub const NaN: &str = float ("nan" );
pub const PosInf: &str = float ("inf" );
pub const NegInf: &str = float ("-inf" );
pub struct JSONDecodeError {
    pub msg: String, // TODO: infer type
    pub doc: String, // TODO: infer type
    pub pos: String, // TODO: infer type
    pub lineno: String, // TODO: infer type
    pub colno: String, // TODO: infer type
    pub object_hook: String, // TODO: infer type
    pub parse_float: String, // TODO: infer type
    pub parse_int: String, // TODO: infer type
    pub parse_constant: String, // TODO: infer type
    pub strict: String, // TODO: infer type
    pub object_pairs_hook: String, // TODO: infer type
    pub parse_object: String, // TODO: infer type
    pub parse_array: String, // TODO: infer type
    pub parse_string: String, // TODO: infer type
    pub memo: String, // TODO: infer type
    pub scan_once: String, // TODO: infer type
}

impl JSONDecodeError {
    pub fn new(msg: &str, doc: &str, pos: &str) -> Self {
        lineno = doc . count ( "\n" , 0 , pos ) + 1;
        colno = pos - doc . rfind ( "\n" , 0 , pos );
        errmsg = "%s: line %d column %d (char %d)" % ( msg , lineno , colno , pos );
        ValueError . __init__ ( self , errmsg );
        self . msg = msg;
        self . doc = doc;
        self . pos = pos;
        self . lineno = lineno;
        self . colno = colno;
    }

    pub fn _decode_uXXXX(&self, s: &str, pos: &str) {
        esc = s [ pos + 1 : pos + 5 ];
        if len ( esc ) == 4 && esc [ 1 ] !in "xX" {
        // try {
        return  int ( esc , 16 );
        // } catch  ValueError  {
        // pass
        msg = "Invalid \\uXXXX escape";
        panic!("JSONDecodeError ( msg , s , pos )");
        pub fn py_scanstring ( s , end , strict = true , {
        _b = BACKSLASH , _m = STRINGCHUNK . match ) ;
        "Scan the string s for a JSON string. End == the index of the
    character in s after the quote that started the JSON string.
    Unescapes all valid JSON string escape sequences && raises ValueError
    on attempt to decode an invalid string. If strict == false then literal
    control characters are allowed in the string.

    Returns a tuple of the decoded string && the index of the character in s
    after the end quote.";
        chunks = [ ];
        _append = chunks . append;
        begin = end - 1;
        while 1  {
        chunk = _m ( s , end );
        if chunk is None /* Option */ {
        panic!("JSONDecodeError ( "Unterminated string starting at" , s , begin )");
        end = chunk . end ( );
        content , terminator = chunk . groups ( );
        if content {
        _append ( content );
        if terminator == """ {
        break;
        } else if terminator != "\\" {
        if strict {
        msg = "Invalid control character {0!r} at" . format ( terminator );
        panic!("JSONDecodeError ( msg , s , end )");
        } else {
        _append ( terminator );
        continue;
        // try {
        esc = s [ end ];
        // } catch  IndexError  {
        panic!("JSONDecodeError ( "Unterminated string starting at" ,");
        s , begin ) from None /* Option */;
        if esc != "u" {
        // try {
        char = _b [ esc ];
        // } catch  KeyError  {
        msg = "Invalid \\escape: {0!r}" . format ( esc );
        panic!("JSONDecodeError ( msg , s , end )");
        end + = 1;
        } else {
        uni = _decode_uXXXX ( s , end );
        end + = 5;
        if 0x d800 <= uni <= 0x dbff && s [ end { : end + 2 ] == "\\u" ; }
        uni2 = _decode_uXXXX ( s , end + 1 );
        if 0x dc00 <= uni2 <= 0x dfff {
        uni = 0x10000 + ( ( ( uni - 0x d800 ) < < 10 ) | ( uni2 - 0x dc00 ) );
        end + = 6;
        char = chr ( uni );
        _append ( char );
        return  "" . join ( chunks ) , end;
        scanstring = c_scanstring || py_scanstring;
        WHITESPACE = re . compile ( r "[ \t\n\r]*" , FLAGS );
        WHITESPACE_STR = " \t\n\r";
        pub fn JSONObject ( s_and_end , strict , scan_once , object_hook , object_pairs_hook , {
        memo = None /* Option */ , _w = WHITESPACE . match , _ws = WHITESPACE_STR ) ;
        s , end = s_and_end;
        pairs = [ ];
        pairs_append = pairs . append;
        if memo is None /* Option */ {
        memo = { };
        memo_get = memo . setdefault;
        nextchar = s [ end : end + 1 ];
        if nextchar != """ {
        if nextchar in _ws {
        end = _w ( s , end ) . end ( );
        nextchar = s [ end : end + 1 ];
        if nextchar == "}" {
        if object_pairs_hook is !None /* Option */ {
        result = object_pairs_hook ( pairs );
        return  result , end + 1;
        pairs = { };
        if object_hook is !None /* Option */ {
        pairs = object_hook ( pairs );
        return  pairs , end + 1;
        } else if nextchar != """ {
        panic!("JSONDecodeError (");
        "Expecting property name enclosed in double quotes" , s , end );
        end + = 1;
        while true  {
        key , end = scanstring ( s , end , strict );
        key = memo_get ( key , key );
        if s [ end { : end + 1 ] != ":" ; }
        end = _w ( s , end ) . end ( );
        if s [ end { : end + 1 ] != ":" ; }
        panic!("JSONDecodeError ( "Expecting ':' delimiter" , s , end )");
        end + = 1;
        // try {
        if s [ end ] in _ws {
        end + = 1;
        if s [ end ] in _ws {
        end = _w ( s , end + 1 ) . end ( );
        // } catch  IndexError  {
        // pass
        // try {
        value , end = scan_once ( s , end );
        // } catch  StopIteration as err  {
        panic!("JSONDecodeError ( "Expecting value" , s , err . value ) from None /* Option */");
        pairs_append ( ( key , value ) );
        // try {
        nextchar = s [ end ];
        if nextchar in _ws {
        end = _w ( s , end + 1 ) . end ( );
        nextchar = s [ end ];
        // } catch  IndexError  {
        nextchar = "";
        end + = 1;
        if nextchar == "}" {
        break;
        } else if nextchar != "," {
        panic!("JSONDecodeError ( "Expecting ',' delimiter" , s , end - 1 )");
        end = _w ( s , end ) . end ( );
        nextchar = s [ end : end + 1 ];
        end + = 1;
        if nextchar != """ {
        panic!("JSONDecodeError (");
        "Expecting property name enclosed in double quotes" , s , end - 1 );
        if object_pairs_hook is !None /* Option */ {
        result = object_pairs_hook ( pairs );
        return  result , end;
        pairs = dict ( pairs );
        if object_hook is !None /* Option */ {
        pairs = object_hook ( pairs );
        return  pairs , end;
        pub fn JSONArray ( s_and_end , scan_once , _w = WHITESPACE . match , _ws = WHITESPACE_STR )  {
        s , end = s_and_end;
        values = [ ];
        nextchar = s [ end : end + 1 ];
        if nextchar in _ws {
        end = _w ( s , end + 1 ) . end ( );
        nextchar = s [ end : end + 1 ];
        if nextchar == "]" {
        return  values , end + 1;
        _append = values . append;
        while true  {
        // try {
        value , end = scan_once ( s , end );
        // } catch  StopIteration as err  {
        panic!("JSONDecodeError ( "Expecting value" , s , err . value ) from None /* Option */");
        _append ( value );
        nextchar = s [ end : end + 1 ];
        if nextchar in _ws {
        end = _w ( s , end + 1 ) . end ( );
        nextchar = s [ end : end + 1 ];
        end + = 1;
        if nextchar == "]" {
        break;
        } else if nextchar != "," {
        panic!("JSONDecodeError ( "Expecting ',' delimiter" , s , end - 1 )");
        // try {
        if s [ end ] in _ws {
        end + = 1;
        if s [ end ] in _ws {
        end = _w ( s , end + 1 ) . end ( );
        // } catch  IndexError  {
        // pass
        return  values , end;
        class JSONDecoder ( object ) ;
        "Simple JSON <https://json.org> decoder

    Performs the following translations in decoding by default:

    +---------------+-------------------+
    | JSON          | Python            |
    +===============+===================+
    | object        | dict              |
    +---------------+-------------------+
    | array         | list              |
    +---------------+-------------------+
    | string        | str               |
    +---------------+-------------------+
    | number (int)  | int               |
    +---------------+-------------------+
    | number (real) | float             |
    +---------------+-------------------+
    | true          | true              |
    +---------------+-------------------+
    | false         | false             |
    +---------------+-------------------+
    | null          | None /* Option */              |
    +---------------+-------------------+

    It also understands ``NaN``, ``Infinity``, && ``-Infinity`` as
    their corresponding ``float`` values, which == outside the JSON spec.

    ";
        pub fn __init__ ( &self, * , object_hook = None /* Option */ , parse_float = None /* Option */ , {
        parse_int = None /* Option */ , parse_constant = None /* Option */ , strict = true ,;
        object_pairs_hook = None /* Option */ ) ;
        "``object_hook``, if specified, will be called with the result
        of every JSON object decoded && its return value will be used in
        place of the given ``dict``.  This can be used to provide custom
        deserializations (e.g. to support JSON-RPC class hinting).

        ``object_pairs_hook``, if specified will be called with the result of
        every JSON object decoded with an ordered list of pairs.  The return
        value of ``object_pairs_hook`` will be used instead of the ``dict``.
        This feature can be used to implement custom decoders.
        If ``object_hook`` == also defined, the ``object_pairs_hook`` takes
        priority.

        ``parse_float``, if specified, will be called with the string
        of every JSON float to be decoded. By default this == equivalent to
        float(num_str). This can be used to use another datatype || parser
        for JSON floats (e.g. decimal.Decimal).

        ``parse_int``, if specified, will be called with the string
        of every JSON int to be decoded. By default this == equivalent to
        int(num_str). This can be used to use another datatype || parser
        for JSON integers (e.g. float).

        ``parse_constant``, if specified, will be called with one of the
        following strings: -Infinity, Infinity, NaN.
        This can be used to raise an exception if invalid JSON numbers
        are encountered.

        If ``strict`` == false (true == the default), then control
        characters will be allowed inside strings.  Control characters in
        this context are those with character codes in the 0-31 range,
        including ``'\\t'`` (tab), ``'\\n'``, ``'\\r'`` && ``'\\0'``.
        ";
        self . object_hook = object_hook;
        self . parse_float = parse_float || float;
        self . parse_int = parse_int || int;
        self . parse_constant = parse_constant || _CONSTANTS . __getitem__;
        self . strict = strict;
        self . object_pairs_hook = object_pairs_hook;
        self . parse_object = JSONObject;
        self . parse_array = JSONArray;
        self . parse_string = scanstring;
        self . memo = { };
        self . scan_once = scanner . make_scanner ( self );
        pub fn decode ( &self, s , _w = WHITESPACE . match )  {
        "Return the Python representation of ``s`` (a ``str`` instance
        containing a JSON document).

        ";
        obj , end = self . raw_decode ( s , idx = _w ( s , 0 ) . end ( ) );
        end = _w ( s , end ) . end ( );
        if end != len ( s ) {
        panic!("JSONDecodeError ( "Extra data" , s , end )");
        return  obj;
        pub fn raw_decode ( &self, s , idx = 0 )  {
        "Decode a JSON document from ``s`` (a ``str`` beginning with
        a JSON document) && return a 2-tuple of the Python
        representation && the index in ``s`` where the document ended.

        This can be used to decode a JSON document from a string that may
        have extraneous data at the end.

        ";
        // try {
        obj , end = self . scan_once ( s , idx );
        // } catch  StopIteration as err  {
        panic!("JSONDecodeError ( "Expecting value" , s , err . value ) from None /* Option */");
        return  obj , end;
    }

}

