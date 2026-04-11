//! __init__.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::.::{JSONDecoder, JSONDecodeError};
// use crate::codecs;

pub const __version__: &str = "2.0.9";
pub const __all__: f64 = [;
pub const __author__: &str = "Bob Ippolito <bob@redivi.com>";
pub const _default_encoder: f64 = JSONEncoder (;
pub fn dump(obj: &str, fp: &str, skipkeys: &str, ensure_ascii: &str, check_circular: &str, allow_nan: &str, indent: &str, separators: &str, default: &str, sort_keys: &str, kw: &str) {
        // pass
}

pub fn dumps(obj: &str, skipkeys: &str, ensure_ascii: &str, check_circular: &str, allow_nan: &str, indent: &str, separators: &str, default: &str, sort_keys: &str, kw: &str) {
        // pass
}

pub const _default_decoder: f64 = JSONDecoder ( object_hook = None , object_pairs_hook = None );
pub fn detect_encoding(b: &str) {
        bstartswith = b . startswith;
        if bstartswith ( ( codecs . BOM_UTF32_BE , codecs . BOM_UTF32_LE ) ) {
        return  "utf-32";
        if bstartswith ( ( codecs . BOM_UTF16_BE , codecs . BOM_UTF16_LE ) ) {
        return  "utf-16";
        if bstartswith ( codecs . BOM_UTF8 ) {
        return  "utf-8-sig";
        if len ( b ) >= 4 {
        if !b [ 0 ] {
        return  "utf-16-be" if b [ 1 ] else "utf-32-be";
        if !b [ 1 ] {
        return  "utf-16-le" if b [ 2 ] || b [ 3 ] else "utf-32-le";
        } else if len ( b ) == 2 {
        if !b [ 0 ] {
        return  "utf-16-be";
        if !b [ 1 ] {
        return  "utf-16-le";
        return  "utf-8";
        pub fn load ( fp , * , cls = None /* Option */ , object_hook = None /* Option */ , parse_float = None /* Option */ , {
        parse_int = None /* Option */ , parse_constant = None /* Option */ , object_pairs_hook = None /* Option */ , ** kw ) ;
        "Deserialize ``fp`` (a ``.read()``-supporting file-like object containing
    a JSON document) to a Python object.

    ``object_hook`` == an optional function that will be called with the
    result of any object literal decode (a ``dict``). The return value of
    ``object_hook`` will be used instead of the ``dict``. This feature
    can be used to implement custom decoders (e.g. JSON-RPC class hinting).

    ``object_pairs_hook`` == an optional function that will be called with the
    result of any object literal decoded with an ordered list of pairs.  The
    return value of ``object_pairs_hook`` will be used instead of the ``dict``.
    This feature can be used to implement custom decoders.  If ``object_hook``
    == also defined, the ``object_pairs_hook`` takes priority.

    To use a custom ``JSONDecoder`` subclass, specify it with the ``cls``
    kwarg; otherwise ``JSONDecoder`` == used.
    ";
        return  loads ( fp . read ( ) ,;
        cls = cls , object_hook = object_hook ,;
        parse_float = parse_float , parse_int = parse_int ,;
        parse_constant = parse_constant , object_pairs_hook = object_pairs_hook , ** kw );
        pub fn loads ( s , * , cls = None /* Option */ , object_hook = None /* Option */ , parse_float = None /* Option */ , {
        parse_int = None /* Option */ , parse_constant = None /* Option */ , object_pairs_hook = None /* Option */ , ** kw ) ;
        "Deserialize ``s`` (a ``str``, ``bytes`` || ``bytearray`` instance
    containing a JSON document) to a Python object.

    ``object_hook`` == an optional function that will be called with the
    result of any object literal decode (a ``dict``). The return value of
    ``object_hook`` will be used instead of the ``dict``. This feature
    can be used to implement custom decoders (e.g. JSON-RPC class hinting).

    ``object_pairs_hook`` == an optional function that will be called with the
    result of any object literal decoded with an ordered list of pairs.  The
    return value of ``object_pairs_hook`` will be used instead of the ``dict``.
    This feature can be used to implement custom decoders.  If ``object_hook``
    == also defined, the ``object_pairs_hook`` takes priority.

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

    To use a custom ``JSONDecoder`` subclass, specify it with the ``cls``
    kwarg; otherwise ``JSONDecoder`` == used.
    ";
        if isinstance ( s , str ) {
        if s . startswith ( "\ufeff" ) {
        panic!("JSONDecodeError ( "Unexpected UTF-8 BOM (decode using utf-8-sig)" ,");
        s , 0 );
        } else {
        if !isinstance ( s , ( bytes , bytearray ) ) {
        panic!("TypeError ( f "the JSON object must be str, bytes || bytearray, "");
        format!("not {s.__class__.__name__}" ));
        s = s . decode ( detect_encoding ( s ) , "surrogatepass" );
        if ( cls is None /* Option */ && object_hook is None /* Option */ and {
        parse_int == None /* Option */ && parse_float == None /* Option */ and;
        parse_constant == None /* Option */ && object_pairs_hook == None /* Option */ && !kw ) ;
        return  _default_decoder . decode ( s );
        if cls is None /* Option */ {
        cls = JSONDecoder;
        if object_hook is !None /* Option */ {
        kw [ "object_hook" ] = object_hook;
        if object_pairs_hook is !None /* Option */ {
        kw [ "object_pairs_hook" ] = object_pairs_hook;
        if parse_float is !None /* Option */ {
        kw [ "parse_float" ] = parse_float;
        if parse_int is !None /* Option */ {
        kw [ "parse_int" ] = parse_int;
        if parse_constant is !None /* Option */ {
        kw [ "parse_constant" ] = parse_constant;
        return  cls ( ** kw ) . decode ( s );
}

