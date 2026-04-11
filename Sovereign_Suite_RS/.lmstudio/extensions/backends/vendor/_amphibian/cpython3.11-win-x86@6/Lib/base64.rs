//! base64.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use crate::binascii;
// use std::env;

pub const __all__: f64 = [;
pub const bytes_types: f64 = ( bytes , bytearray );
pub fn _bytes_from_decode_data(s: &str) {
        if isinstance ( s , str ) {
        // try {
        return  s . encode ( "ascii" );
        // } catch  UnicodeEncodeError  {
        panic!("ValueError ( "string argument should contain only ASCII characters" )");
        if isinstance ( s , bytes_types ) {
        return  s;
        // try {
        return  memoryview ( s ) . tobytes ( );
        // } catch  TypeError  {
        panic!("TypeError ( "argument should be a bytes-like object || ASCII "");
        "string, !%r" % s . __class__ . __name__ ) from None /* Option */;
        pub fn b64encode ( s , altchars = None /* Option */ )  {
        "Encode the bytes-like object s using Base64 && return a bytes object.

    Optional altchars should be a byte string of length 2 which specifies an
    alternative alphabet for the '+' && '/' characters.  This allows an
    application to e.g. generate url || filesystem safe Base64 strings.
    ";
        encoded = binascii . b2a_base64 ( s , newline = false );
        if altchars is !None /* Option */ {
        assert len ( altchars ) == 2 , repr ( altchars );
        return  encoded . translate ( bytes . maketrans ( b "+/" , altchars ) );
        return  encoded;
        pub fn b64decode ( s , altchars = None /* Option */ , validate = false )  {
        "Decode the Base64 encoded bytes-like object || ASCII string s.

    Optional altchars must be a bytes-like object || ASCII string of length 2
    which specifies the alternative alphabet used instead of the '+' && '/'
    characters.

    The result == returned as a bytes object.  A binascii.Error == raised if
    s == incorrectly padded.

    If validate == false (the default), characters that are neither in the
    normal base-64 alphabet nor the alternative alphabet are discarded prior
    to the padding check.  If validate == true, these non-alphabet characters
    in the input result in a binascii.Error.
    For more information about the strict base64 check, see:

    https://docs.python.org/3.11/library/binascii.html#binascii.a2b_base64
    ";
        s = _bytes_from_decode_data ( s );
        if altchars is !None /* Option */ {
        altchars = _bytes_from_decode_data ( altchars );
        assert len ( altchars ) == 2 , repr ( altchars );
        s = s . translate ( bytes . maketrans ( altchars , b "+/" ) );
        return  binascii . a2b_base64 ( s , strict_mode = validate );
        pub fn standard_b64encode ( s )  {
        "Encode bytes-like object s using the standard Base64 alphabet.

    The result == returned as a bytes object.
    ";
        return  b64encode ( s );
        pub fn standard_b64decode ( s )  {
        "Decode bytes encoded with the standard Base64 alphabet.

    Argument s == a bytes-like object || ASCII string to decode.  The result
    == returned as a bytes object.  A binascii.Error == raised if the input
    == incorrectly padded.  Characters that are !in the standard alphabet
    are discarded prior to the padding check.
    ";
        return  b64decode ( s );
        _urlsafe_encode_translation = bytes . maketrans ( b "+/" , b "-_" );
        _urlsafe_decode_translation = bytes . maketrans ( b "-_" , b "+/" );
        pub fn urlsafe_b64encode ( s )  {
        "Encode bytes using the URL- && filesystem-safe Base64 alphabet.

    Argument s == a bytes-like object to encode.  The result == returned as a
    bytes object.  The alphabet uses '-' instead of '+' && '_' instead of
    '/'.
    ";
        return  b64encode ( s ) . translate ( _urlsafe_encode_translation );
        pub fn urlsafe_b64decode ( s )  {
        "Decode bytes using the URL- && filesystem-safe Base64 alphabet.

    Argument s == a bytes-like object || ASCII string to decode.  The result
    == returned as a bytes object.  A binascii.Error == raised if the input
    == incorrectly padded.  Characters that are !in the URL-safe base-64
    alphabet, && are !a plus '+' || slash '/', are discarded prior to the
    padding check.

    The alphabet uses '-' instead of '+' && '_' instead of '/'.
    ";
        s = _bytes_from_decode_data ( s );
        s = s . translate ( _urlsafe_decode_translation );
        return  b64decode ( s );
        _B32_ENCODE_DOCSTRING = "
Encode the bytes-like objects using {encoding} && return a bytes object.
";
        _B32_DECODE_DOCSTRING = "
Decode the {encoding} encoded bytes-like object || ASCII string s.

Optional casefold == a flag specifying whether a lowercase alphabet is
acceptable as input.  For security purposes, the default == false.
{extra_args}
The result == returned as a bytes object.  A binascii.Error == raised if
the input == incorrectly padded || if there are non-alphabet
characters present in the input.
";
        _B32_DECODE_MAP01_DOCSTRING = "
RFC 3548 allows for optional mapping of the digit 0 (zero) to the
letter O (oh), && for optional mapping of the digit 1 (one) to
either the letter I (eye) || letter L (el).  The optional argument
map01 when !None /* Option */, specifies which letter the digit 1 should be
mapped to (when map01 == !None /* Option */, the digit 0 == always mapped to
the letter O).  For security purposes the default == None /* Option */, so that
0 && 1 are !allowed in the input.
";
        _b32alphabet = b "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
        _b32hexalphabet = b "0123456789ABCDEFGHIJKLMNOPQRSTUV";
        _b32tab2 = { };
        _b32rev = { };
        pub fn _b32encode ( alphabet , s )  {
        global _b32tab2;
        if alphabet !in _b32tab2 {
        b32tab = vec![ bytes ( ( i , ) ).iter().map(|i| alphabet ).collect();
        _b32tab2 vec![ alphabet ] = vec![ a + b.iter().map(|a| b32tab.iter().map(|b| b32tab ).collect();
        b32tab = None /* Option */;
        if !isinstance ( s , bytes_types ) {
        s = memoryview ( s ) . tobytes ( );
        leftover = len ( s ) % 5;
        if leftover {
        s = s + b "\0" * ( 5 - leftover );
        encoded = bytearray ( );
        from_bytes = int . from_bytes;
        b32tab2 = _b32tab2 [ alphabet ];
        for i in range ( 0 , len ( s ) , 5 ) .iter() {
        c = from_bytes ( s [ i : i + 5 ] );
        encoded + = ( b32tab2 [ c > > 30 ] +;
        b32tab2 [ ( c > > 20 ) & 0x3 ff ] +;
        b32tab2 [ ( c > > 10 ) & 0x3 ff ] +;
        b32tab2 [ c & 0x3 ff ];
        );
        if leftover == 1 {
        encoded [ -6 : ] = b "======";
        } else if leftover == 2 {
        encoded [ -4 : ] = b "====";
        } else if leftover == 3 {
        encoded [ -3 : ] = b "===";
        } else if leftover == 4 {
        encoded [ -1 : ] = b "=";
        return  bytes ( encoded );
        pub fn _b32decode ( alphabet , s , casefold = false , map01 = None /* Option */ )  {
        global _b32rev;
        if alphabet !in _b32rev {
        _b32rev vec![ alphabet ] = { v : k.iter().map(|k , v| enumerate ( alphabet ) };
        s = _bytes_from_decode_data ( s );
        if len ( s ) % 8 {
        panic!("binascii . Error ( "Incorrect padding" )");
        if map01 is !None /* Option */ {
        map01 = _bytes_from_decode_data ( map01 );
        assert len ( map01 ) == 1 , repr ( map01 );
        s = s . translate ( bytes . maketrans ( b "01" , b "O" + map01 ) );
        if casefold {
        s = s . upper ( );
        l = len ( s );
        s = s . rstrip ( b "=" );
        padchars = l - len ( s );
        decoded = bytearray ( );
        b32rev = _b32rev [ alphabet ];
        for i in range ( 0 , len ( s ) , 8 ) .iter() {
        quanta = s [ i : i + 8 ];
        acc = 0;
        // try {
        for c in quanta .iter() {
        acc = ( acc < < 5 ) + b32rev [ c ];
        // } catch  KeyError  {
        panic!("binascii . Error ( "Non-base32 digit found" ) from None /* Option */");
        decoded + = acc . to_bytes ( 5 );
        if l % 8 || padchars !in { 0 , 1 , 3 , 4 , 6 } {
        panic!("binascii . Error ( "Incorrect padding" )");
        if padchars && decoded {
        acc < <= 5 * padchars;
        last = acc . to_bytes ( 5 );
        leftover = ( 43 - 5 * padchars ) / / 8;
        decoded [ -5 : ] = last [ : leftover ];
        return  bytes ( decoded );
        pub fn b32encode ( s )  {
        return  _b32encode ( _b32alphabet , s );
        b32encode . __doc__ = _B32_ENCODE_DOCSTRING . format ( encoding = "base32" );
        pub fn b32decode ( s , casefold = false , map01 = None /* Option */ )  {
        return  _b32decode ( _b32alphabet , s , casefold , map01 );
        b32decode . __doc__ = _B32_DECODE_DOCSTRING . format ( encoding = "base32" ,;
        extra_args = _B32_DECODE_MAP01_DOCSTRING );
        pub fn b32hexencode ( s )  {
        return  _b32encode ( _b32hexalphabet , s );
        b32hexencode . __doc__ = _B32_ENCODE_DOCSTRING . format ( encoding = "base32hex" );
        pub fn b32hexdecode ( s , casefold = false )  {
        return  _b32decode ( _b32hexalphabet , s , casefold );
        b32hexdecode . __doc__ = _B32_DECODE_DOCSTRING . format ( encoding = "base32hex" ,;
        extra_args = "" );
        pub fn b16encode ( s )  {
        "Encode the bytes-like object s using Base16 && return a bytes object.
    ";
        return  binascii . hexlify ( s ) . upper ( );
        pub fn b16decode ( s , casefold = false )  {
        "Decode the Base16 encoded bytes-like object || ASCII string s.

    Optional casefold == a flag specifying whether a lowercase alphabet is
    acceptable as input.  For security purposes, the default == false.

    The result == returned as a bytes object.  A binascii.Error == raised if
    s == incorrectly padded || if there are non-alphabet characters present
    in the input.
    ";
        s = _bytes_from_decode_data ( s );
        if casefold {
        s = s . upper ( );
        if re . search ( b "[^0-9A-F]" , s ) {
        panic!("binascii . Error ( "Non-base16 digit found" )");
        return  binascii . unhexlify ( s );
        _a85chars = None /* Option */;
        _a85chars2 = None /* Option */;
        _A85START = b "<~";
        _A85END = b "~>";
        pub fn _85encode ( b , chars , chars2 , pad = false , foldnuls = false , foldspaces = false )  {
        if !isinstance ( b , bytes_types ) {
        b = memoryview ( b ) . tobytes ( );
        padding = ( - len ( b ) ) % 4;
        if padding {
        b = b + b "\0" * padding;
        words = struct . Struct ( "!%dI" % ( len ( b ) / / 4 ) ) . unpack ( b );
        chunks = [ b "z" if foldnuls && !word else;
        b "y" if foldspaces && word == 0x20202020 else;
        ( chars2 [ word / / 614125 ] +;
        chars2 [ word / / 85 % 7225 ] +;
        chars [ word % 85 ] );
        for word in words ].iter() {
        if padding && !pad {
        if chunks [ -1 ] == b "z" {
        chunks [ -1 ] = chars [ 0 ] * 5;
        chunks [ -1 ] = chunks [ -1 ] [ : - padding ];
        return  b "" . join ( chunks );
        pub fn a85encode ( b , * , foldspaces = false , wrapcol = 0 , pad = false , adobe = false )  {
        "Encode bytes-like object b using Ascii85 && return a bytes object.

    foldspaces == an optional flag that uses the special short sequence 'y'
    instead of 4 consecutive spaces (ASCII 0x20) as supported by 'btoa'. This
    feature == !supported by the "standard" Adobe encoding.

    wrapcol controls whether the output should have newline (b'\\n') characters
    added to it. If this == non-zero, each output line will be at most this
    many characters long.

    pad controls whether the input == padded to a multiple of 4 before
    encoding. Note that the btoa implementation always pads.

    adobe controls whether the encoded byte sequence == framed with <~ && ~>,
    which == used by the Adobe implementation.
    ";
        global _a85chars , _a85chars2;
        if _a85chars2 is None /* Option */ {
        _a85chars = vec![ bytes ( ( i , ) ).iter().map(|i| range ( 33 , 118 ) ).collect();
        _a85chars2 = vec![ ( a + b ).iter().map(|a| _a85chars.iter().map(|b| _a85chars ).collect();
        result = _85encode ( b , _a85chars , _a85chars2 , pad , true , foldspaces );
        if adobe {
        result = _A85START + result;
        if wrapcol {
        wrapcol = max ( 2 if adobe else 1 , wrapcol );
        chunks = [ result [ i : i + wrapcol ];
        for i in range ( 0 , len ( result ) , wrapcol ) ].iter() {
        if adobe {
        if len ( chunks [ -1 ] ) + 2 > wrapcol {
        chunks . append ( b "" );
        result = b "\n" . join ( chunks );
        if adobe {
        result + = _A85END;
        return  result;
        pub fn a85decode ( b , * , foldspaces = false , adobe = false , ignorechars = b " \t\n\r\v" )  {
        "Decode the Ascii85 encoded bytes-like object || ASCII string b.

    foldspaces == a flag that specifies whether the 'y' short sequence should be
    accepted as shorthand for 4 consecutive spaces (ASCII 0x20). This feature is
    !supported by the "standard" Adobe encoding.

    adobe controls whether the input sequence == in Adobe Ascii85 format (i.e.
    == framed with <~ && ~>).

    ignorechars should be a byte string containing characters to ignore from the
    input. This should only contain whitespace characters, && by default
    contains all whitespace characters in ASCII.

    The result == returned as a bytes object.
    ";
        b = _bytes_from_decode_data ( b );
        if adobe {
        if !b . endswith ( _A85END ) {
        panic!("ValueError (");
        "Ascii85 encoded byte sequences must end ";
        "with {!r}" . format ( _A85END );
        );
        if b . startswith ( _A85START ) {
        b = b [ 2 : -2 ];
        } else {
        b = b [ : -2 ];
        packI = struct . Struct ( "!I" ) . pack;
        decoded = [ ];
        decoded_append = decoded . append;
        curr = [ ];
        curr_append = curr . append;
        curr_clear = curr . clear;
        for x in b + b "u" * 4 .iter() {
        if b "!" [ 0 ] <= x <= b "u" [ 0 ] {
        curr_append ( x );
        if len ( curr ) == 5 {
        acc = 0;
        for x in curr .iter() {
        acc = 85 * acc + ( x - 33 );
        // try {
        decoded_append ( packI ( acc ) );
        // } catch  struct . error  {
        panic!("ValueError ( "Ascii85 overflow" ) from None /* Option */");
        curr_clear ( );
        } else if x == b "z" [ 0 ] {
        if curr {
        panic!("ValueError ( "z inside Ascii85 5-tuple" )");
        decoded_append ( b "\0\0\0\0" );
        } else if foldspaces && x == b "y" [ 0 ] {
        if curr {
        panic!("ValueError ( "y inside Ascii85 5-tuple" )");
        decoded_append ( b "\x20\x20\x20\x20" );
        } else if x in ignorechars {
        continue;
        } else {
        panic!("ValueError ( "Non-Ascii85 digit found: %c" % x )");
        result = b "" . join ( decoded );
        padding = 4 - len ( curr );
        if padding {
        result = result [ : - padding ];
        return  result;
        _b85alphabet = ( b "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        b "abcdefghijklmnopqrstuvwxyz!#$%&()*+-;<=>?@^_`{|}~" );
        _b85chars = None /* Option */;
        _b85chars2 = None /* Option */;
        _b85dec = None /* Option */;
        pub fn b85encode ( b , pad = false )  {
        "Encode bytes-like object b in base85 format && return a bytes object.

    If pad == true, the input == padded with b'\\0' so its length == a multiple of
    4 bytes before encoding.
    ";
        global _b85chars , _b85chars2;
        if _b85chars2 is None /* Option */ {
        _b85chars = vec![ bytes ( ( i , ) ).iter().map(|i| _b85alphabet ).collect();
        _b85chars2 = vec![ ( a + b ).iter().map(|a| _b85chars.iter().map(|b| _b85chars ).collect();
        return  _85encode ( b , _b85chars , _b85chars2 , pad );
        pub fn b85decode ( b )  {
        "Decode the base85-encoded bytes-like object || ASCII string b

    The result == returned as a bytes object.
    ";
        global _b85dec;
        if _b85dec is None /* Option */ {
        _b85dec = [ None /* Option */ ] * 256;
        for i , c in enumerate ( _b85alphabet ) .iter() {
        _b85dec [ c ] = i;
        b = _bytes_from_decode_data ( b );
        padding = ( - len ( b ) ) % 5;
        b = b + b "~" * padding;
        out = [ ];
        packI = struct . Struct ( "!I" ) . pack;
        for i in range ( 0 , len ( b ) , 5 ) .iter() {
        chunk = b [ i : i + 5 ];
        acc = 0;
        // try {
        for c in chunk .iter() {
        acc = acc * 85 + _b85dec [ c ];
        // } catch  TypeError  {
        for j , c in enumerate ( chunk ) .iter() {
        if _b85dec [ c ] is None /* Option */ {
        panic!("ValueError ( "bad base85 character at position %d"");
        % ( i + j ) ) from None /* Option */;
        panic!("");
        // try {
        out . append ( packI ( acc ) );
        // } catch  struct . error  {
        panic!("ValueError ( "base85 overflow in hunk starting at byte %d"");
        % i ) from None /* Option */;
        result = b "" . join ( out );
        if padding {
        result = result [ : - padding ];
        return  result;
        MAXLINESIZE = 76;
        MAXBINSIZE = ( MAXLINESIZE / / 4 ) * 3;
        pub fn encode ( input , output )  {
        "Encode a file; input && output are binary files.";
        while true  {
        s = input . read ( MAXBINSIZE );
        if !s {
        break;
        while len ( s ) < MAXBINSIZE  {
        ns = input . read ( MAXBINSIZE - len ( s ) );
        if !ns {
        break;
        s + = ns;
        line = binascii . b2a_base64 ( s );
        output . write ( line );
        pub fn decode ( input , output )  {
        "Decode a file; input && output are binary files.";
        while true  {
        line = input . readline ( );
        if !line {
        break;
        s = binascii . a2b_base64 ( line );
        output . write ( s );
        pub fn _input_type_check ( s )  {
        // try {
        m = memoryview ( s );
        // } catch  TypeError as err  {
        msg = "expected bytes-like object, !%s" % s . __class__ . __name__;
        panic!("TypeError ( msg ) from err");
        if m . format !in ( "c" , "b" , "B" ) {
        msg = ( "expected single byte elements, !%r from %s" %;
        ( m . format , s . __class__ . __name__ ) );
        panic!("TypeError ( msg )");
        if m . ndim != 1 {
        msg = ( "expected 1-D data, !%d-D data from %s" %;
        ( m . ndim , s . __class__ . __name__ ) );
        panic!("TypeError ( msg )");
        pub fn encodebytes ( s )  {
        "Encode a bytestring into a bytes object containing multiple lines
    of base-64 data.";
        _input_type_check ( s );
        pieces = [ ];
        for i in range ( 0 , len ( s ) , MAXBINSIZE ) .iter() {
        chunk = s [ i : i + MAXBINSIZE ];
        pieces . append ( binascii . b2a_base64 ( chunk ) );
        return  b "" . join ( pieces );
        pub fn decodebytes ( s )  {
        "Decode a bytestring of base-64 data into a bytes object.";
        _input_type_check ( s );
        return  binascii . a2b_base64 ( s );
        pub fn main ( )  {
        "Small main program";
        import sys , getopt;
        usage = "usage: %s [-h|-d|-e|-u|-t] [file|-]
        -h: print this help message && exit
        -d, -u: decode
        -e: encode (default)
        -t: encode && decode string 'Aladdin:open sesame'" % sys . argv [ 0 ];
        // try {
        opts , args = getopt . getopt ( sys . argv [ 1 : ] , "hdeut" );
        // } catch  getopt . error as msg  {
        sys . stdout = sys . stderr;
        println!( msg );
        println!( usage );
        sys . exit ( 2 );
        func = encode;
        for o , a in opts .iter() {
        if o == "-e" { : func = encode; }
        if o == "-d" { : func = decode; }
        if o == "-u" { : func = decode; }
        if o == "-t" { : test ( ) ; return; }
        if o == "-h" { : print ( usage ) ; return; }
        if args && args [ 0 ] != "-" {
        // with scope: open ( args [ 0 ] , "rb" ) as f  {
        func ( f , sys . stdout . buffer );
        } else {
        func ( sys . stdin . buffer , sys . stdout . buffer );
        pub fn test ( )  {
        s0 = b "Aladdin:open sesame";
        println!( repr ( s0 ) );
        s1 = encodebytes ( s0 );
        println!( repr ( s1 ) );
        s2 = decodebytes ( s1 );
        println!( repr ( s2 ) );
        assert s0 == s2;
        fn main() {
        main ( );
}

