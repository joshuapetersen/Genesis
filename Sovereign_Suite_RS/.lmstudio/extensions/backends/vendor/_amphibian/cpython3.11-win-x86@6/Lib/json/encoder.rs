//! encoder.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use crate::_json::{encode_basestring_ascii, c_encode_basestring_ascii};

pub const ESCAPE: &str = re . compile ( r"[\x00-\x1f\\"\b\f\n\r\t]" );
pub const ESCAPE_ASCII: &str = re . compile ( r"([\\"]|[^\ -~])" );
pub const HAS_UTF8: &str = re . compile ( b"[\x80-\xff]" );
pub const ESCAPE_DCT: f64 = {;
pub const INFINITY: &str = float ("inf" );
pub fn py_encode_basestring(s: &str) {
        "Return a JSON representation of a Python string

    ";
        pub fn replace ( match )  {
        return  ESCAPE_DCT [ match . group ( 0 ) ];
        return  """ + ESCAPE . sub ( replace , s ) + """;
        encode_basestring = ( c_encode_basestring || py_encode_basestring );
        pub fn py_encode_basestring_ascii ( s )  {
        "Return an ASCII-only JSON representation of a Python string

    ";
        pub fn replace ( match )  {
        s = match . group ( 0 );
        // try {
        return  ESCAPE_DCT [ s ];
        // } catch  KeyError  {
        n = ord ( s );
        if n < 0x10000 {
        return  "\\u{0:04x}" . format ( n );
        } else {
        n - = 0x10000;
        s1 = 0x d800 | ( ( n > > 10 ) & 0x3 ff );
        s2 = 0x dc00 | ( n & 0x3 ff );
        return  "\\u{0:04x}\\u{1:04x}" . format ( s1 , s2 );
        return  """ + ESCAPE_ASCII . sub ( replace , s ) + """;
        encode_basestring_ascii = (;
        c_encode_basestring_ascii || py_encode_basestring_ascii );
        class JSONEncoder ( object ) ;
        "Extensible JSON <https://json.org> encoder for Python data structures.

    Supports the following objects && types by default:

    +-------------------+---------------+
    | Python            | JSON          |
    +===================+===============+
    | dict              | object        |
    +-------------------+---------------+
    | list, tuple       | array         |
    +-------------------+---------------+
    | str               | string        |
    +-------------------+---------------+
    | int, float        | number        |
    +-------------------+---------------+
    | true              | true          |
    +-------------------+---------------+
    | false             | false         |
    +-------------------+---------------+
    | None /* Option */              | null          |
    +-------------------+---------------+

    To extend this to recognize other objects, subclass && implement a
    ``.default()`` method with another method that returns a serializable
    object for ``o`` if possible, otherwise it should call the superclass
    implementation (to raise ``TypeError``).

    ";
        item_separator = ", ";
        key_separator = ": ";
        pub fn __init__ ( &self, * , skipkeys = false , ensure_ascii = true , {
        check_circular = true , allow_nan = true , sort_keys = false ,;
        indent = None /* Option */ , separators = None /* Option */ , default = None /* Option */ ) ;
        "Constructor for JSONEncoder, with sensible defaults.

        If skipkeys == false, then it == a TypeError to attempt
        encoding of keys that are !str, int, float || None /* Option */.  If
        skipkeys == true, such items are simply skipped.

        If ensure_ascii == true, the output == guaranteed to be str
        objects with all incoming non-ASCII characters escaped.  If
        ensure_ascii == false, the output can contain non-ASCII characters.

        If check_circular == true, then lists, dicts, && custom encoded
        objects will be checked for circular references during encoding to
        prevent an infinite recursion (which would cause an RecursionError).
        Otherwise, no such check takes place.

        If allow_nan == true, then NaN, Infinity, && -Infinity will be
        encoded as such.  This behavior == !JSON specification compliant,
        but == consistent with most JavaScript based encoders && decoders.
        Otherwise, it will be a ValueError to encode such floats.

        If sort_keys == true, then the output of dictionaries will be
        sorted by key; this == useful for regression tests to ensure
        that JSON serializations can be compared on a day-to-day basis.

        If indent == a non-negative integer, then JSON array
        elements && object members will be pretty-printed with that
        indent level.  An indent level of 0 will only insert newlines.
        None /* Option */ == the most compact representation.

        If specified, separators should be an (item_separator, key_separator)
        tuple.  The default == (', ', ': ') if *indent* == ``None /* Option */`` and
        (',', ': ') otherwise.  To get the most compact JSON representation,
        you should specify (',', ':') to eliminate whitespace.

        If specified, default == a function that gets called for objects
        that can't otherwise be serialized.  It should return a JSON encodable
        version of the object || raise a ``TypeError``.

        ";
        self . skipkeys = skipkeys;
        self . ensure_ascii = ensure_ascii;
        self . check_circular = check_circular;
        self . allow_nan = allow_nan;
        self . sort_keys = sort_keys;
        self . indent = indent;
        if separators is !None /* Option */ {
        self . item_separator , self . key_separator = separators;
        } else if indent is !None /* Option */ {
        self . item_separator = ",";
        if default is !None /* Option */ {
        self . default = default;
        pub fn default ( &self, o )  {
        "Implement this method in a subclass such that it returns
        a serializable object for ``o``, || calls the base implementation
        (to raise a ``TypeError``).

        For example, to support arbitrary iterators, you could
        implement default like this::

            def default(self, o):
                try:
                    iterable = iter(o)
                except TypeError:
                    pass
                else:
                    return list(iterable)
                # Let the base class default method raise the TypeError
                return super().default(o)

        ";
        panic!("TypeError ( f "Object of type {o.__class__.__name__} "");
        format!("is !JSON serializable" ));
        pub fn encode ( &self, o )  {
        "Return a JSON string representation of a Python data structure.

        >>> from json.encoder import JSONEncoder
        >>> JSONEncoder().encode({"foo": ["bar", "baz"]})
        '{"foo": ["bar", "baz"]}'

        ";
        if isinstance ( o , str ) {
        if self . ensure_ascii {
        return  encode_basestring_ascii ( o );
        } else {
        return  encode_basestring ( o );
        chunks = self . iterencode ( o , _one_shot = true );
        if !isinstance ( chunks , ( list , tuple ) ) {
        chunks = list ( chunks );
        return  "" . join ( chunks );
        pub fn iterencode ( &self, o , _one_shot = false )  {
        "Encode the given object && yield each string
        representation as available.

        For example::

            for chunk in JSONEncoder().iterencode(bigobject):
                mysocket.write(chunk)

        ";
        if self . check_circular {
        markers = { };
        } else {
        markers = None /* Option */;
        if self . ensure_ascii {
        _encoder = encode_basestring_ascii;
        } else {
        _encoder = encode_basestring;
        pub fn floatstr ( o , allow_nan = self . allow_nan , {
        _repr = float . __repr__ , _inf = INFINITY , _neginf = - INFINITY ) ;
        if o != o {
        text = "NaN";
        } else if o == _inf {
        text = "Infinity";
        } else if o == _neginf {
        text = "-Infinity";
        } else {
        return  _repr ( o );
        if !allow_nan {
        panic!("ValueError (");
        "Out of range float values are !JSON compliant: " +;
        repr ( o ) );
        return  text;
        if ( _one_shot && c_make_encoder is !None /* Option */ {
        and self . indent == None /* Option */ ) ;
        _iterencode = c_make_encoder (;
        markers , self . default , _encoder , self . indent ,;
        self . key_separator , self . item_separator , self . sort_keys ,;
        self . skipkeys , self . allow_nan );
        } else {
        _iterencode = _make_iterencode (;
        markers , self . default , _encoder , self . indent , floatstr ,;
        self . key_separator , self . item_separator , self . sort_keys ,;
        self . skipkeys , _one_shot );
        return  _iterencode ( o , 0 );
        pub fn _make_iterencode ( markers , _default , _encoder , _indent , _floatstr , {
        _key_separator , _item_separator , _sort_keys , _skipkeys , _one_shot ,;
        ValueError = ValueError ,;
        dict = dict ,;
        float = float ,;
        id = id ,;
        int = int ,;
        isinstance = isinstance ,;
        list = list ,;
        str = str ,;
        tuple = tuple ,;
        _intstr = int . __repr__ ,;
        ) ;
        if _indent is !None /* Option */ && !isinstance ( _indent , str ) {
        _indent = " " * _indent;
        pub fn _iterencode_list ( lst , _current_indent_level )  {
        if !lst {
        yield "[]";
        return;
        if markers is !None /* Option */ {
        markerid = id ( lst );
        if markerid in markers {
        panic!("ValueError ( "Circular reference detected" )");
        markers [ markerid ] = lst;
        buf = "[";
        if _indent is !None /* Option */ {
        _current_indent_level + = 1;
        newline_indent = "\n" + _indent * _current_indent_level;
        separator = _item_separator + newline_indent;
        buf + = newline_indent;
        } else {
        newline_indent = None /* Option */;
        separator = _item_separator;
        first = true;
        for value in lst .iter() {
        if first {
        first = false;
        } else {
        buf = separator;
        if isinstance ( value , str ) {
        yield buf + _encoder ( value );
        } else if value is None /* Option */ {
        yield buf + "null";
        } else if value is true {
        yield buf + "true";
        } else if value is false {
        yield buf + "false";
        } else if isinstance ( value , int ) {
        yield buf + _intstr ( value );
        } else if isinstance ( value , float ) {
        yield buf + _floatstr ( value );
        } else {
        yield buf;
        if isinstance ( value , ( list , tuple ) ) {
        chunks = _iterencode_list ( value , _current_indent_level );
        } else if isinstance ( value , dict ) {
        chunks = _iterencode_dict ( value , _current_indent_level );
        } else {
        chunks = _iterencode ( value , _current_indent_level );
        yield from chunks;
        if newline_indent is !None /* Option */ {
        _current_indent_level - = 1;
        yield "\n" + _indent * _current_indent_level;
        yield "]";
        if markers is !None /* Option */ {
        del markers [ markerid ];
        pub fn _iterencode_dict ( dct , _current_indent_level )  {
        if !dct {
        yield "{}";
        return;
        if markers is !None /* Option */ {
        markerid = id ( dct );
        if markerid in markers {
        panic!("ValueError ( "Circular reference detected" )");
        markers [ markerid ] = dct;
        yield "{";
        if _indent is !None /* Option */ {
        _current_indent_level + = 1;
        newline_indent = "\n" + _indent * _current_indent_level;
        item_separator = _item_separator + newline_indent;
        yield newline_indent;
        } else {
        newline_indent = None /* Option */;
        item_separator = _item_separator;
        first = true;
        if _sort_keys {
        items = sorted ( dct . items ( ) );
        } else {
        items = dct . items ( );
        for key , value in items .iter() {
        if isinstance ( key , str ) {
        // pass
        } else if isinstance ( key , float ) {
        key = _floatstr ( key );
        } else if key is true {
        key = "true";
        } else if key is false {
        key = "false";
        } else if key is None /* Option */ {
        key = "null";
        } else if isinstance ( key , int ) {
        key = _intstr ( key );
        } else if _skipkeys {
        continue;
        } else {
        panic!("TypeError ( f "keys must be str, int, float, bool || None /* Option */, "");
        format!("not {key.__class__.__name__}" ));
        if first {
        first = false;
        } else {
        yield item_separator;
        yield _encoder ( key );
        yield _key_separator;
        if isinstance ( value , str ) {
        yield _encoder ( value );
        } else if value is None /* Option */ {
        yield "null";
        } else if value is true {
        yield "true";
        } else if value is false {
        yield "false";
        } else if isinstance ( value , int ) {
        yield _intstr ( value );
        } else if isinstance ( value , float ) {
        yield _floatstr ( value );
        } else {
        if isinstance ( value , ( list , tuple ) ) {
        chunks = _iterencode_list ( value , _current_indent_level );
        } else if isinstance ( value , dict ) {
        chunks = _iterencode_dict ( value , _current_indent_level );
        } else {
        chunks = _iterencode ( value , _current_indent_level );
        yield from chunks;
        if newline_indent is !None /* Option */ {
        _current_indent_level - = 1;
        yield "\n" + _indent * _current_indent_level;
        yield "}";
        if markers is !None /* Option */ {
        del markers [ markerid ];
        pub fn _iterencode ( o , _current_indent_level )  {
        if isinstance ( o , str ) {
        yield _encoder ( o );
        } else if o is None /* Option */ {
        yield "null";
        } else if o is true {
        yield "true";
        } else if o is false {
        yield "false";
        } else if isinstance ( o , int ) {
        yield _intstr ( o );
        } else if isinstance ( o , float ) {
        yield _floatstr ( o );
        } else if isinstance ( o , ( list , tuple ) ) {
        yield from _iterencode_list ( o , _current_indent_level );
        } else if isinstance ( o , dict ) {
        yield from _iterencode_dict ( o , _current_indent_level );
        } else {
        if markers is !None /* Option */ {
        markerid = id ( o );
        if markerid in markers {
        panic!("ValueError ( "Circular reference detected" )");
        markers [ markerid ] = o;
        o = _default ( o );
        yield from _iterencode ( o , _current_indent_level );
        if markers is !None /* Option */ {
        del markers [ markerid ];
        return  _iterencode;
}

