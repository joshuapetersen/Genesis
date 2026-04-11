//! _encoded_words.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use crate::binascii;
// use crate::string::{ascii_letters, digits};
// use crate::email::{errors};

pub const __all__: &str = ["decode_q" ,;
pub const _q_byte_subber: &str = functools . partial ( re . compile ( br"=([a-fA-F0-9]{2})" ) . sub ,;
pub fn decode_q(encoded: &str) {
        encoded = encoded . replace ( b "_" , b " " );
        return  _q_byte_subber ( encoded ) , [ ];
        class _QByteMap ( dict ) ;
        safe = b "-!*+/" + ascii_letters . encode ( "ascii" ) + digits . encode ( "ascii" );
        pub fn __missing__ ( &self, key )  {
        if key in self . safe {
        self [ key ] = chr ( key );
        } else {
        self [ key ] = "={:02X}" . format ( key );
        return  self [ key ];
        _q_byte_map = _QByteMap ( );
        _q_byte_map [ ord ( " " ) ] = "_";
        pub fn encode_q ( bstring )  {
        return  "" . join ( _q_byte_map [ x ] for x in bstring );
        pub fn len_q ( bstring )  {
        return  sum ( len ( _q_byte_map [ x ] ) for x in bstring );
        pub fn decode_b ( encoded )  {
        pad_err = len ( encoded ) % 4;
        missing_padding = b "===" [ : 4 - pad_err ] if pad_err else b "";
        // try {
        return  (;
        base64 . b64decode ( encoded + missing_padding , validate = true ) ,;
        [ errors . InvalidBase64PaddingDefect ( ) ] if pad_err else [ ] ,;
        );
        // } catch  binascii . Error  {
        // try {
        return  (;
        base64 . b64decode ( encoded , validate = false ) ,;
        [ errors . InvalidBase64CharactersDefect ( ) ] ,;
        );
        // } catch  binascii . Error  {
        // try {
        return  (;
        base64 . b64decode ( encoded + b "==" , validate = false ) ,;
        [ errors . InvalidBase64CharactersDefect ( ) ,;
        errors . InvalidBase64PaddingDefect ( ) ] ,;
        );
        // } catch  binascii . Error  {
        return  encoded , [ errors . InvalidBase64LengthDefect ( ) ];
        pub fn encode_b ( bstring )  {
        return  base64 . b64encode ( bstring ) . decode ( "ascii" );
        pub fn len_b ( bstring )  {
        groups_of_3 , leftover = divmod ( len ( bstring ) , 3 );
        return  groups_of_3 * 4 + ( 4 if leftover else 0 );
        _cte_decoders = {;
        "q" : decode_q ,;
        "b" : decode_b ,;
        };
        pub fn decode ( ew )  {
        "Decode encoded word && return (string, charset, lang, defects) tuple.

    An RFC 2047/2243 encoded word has the form:

        =?charset*lang?cte?encoded_string?=

    where '*lang' may be omitted but the other parts may !be.

    This function expects exactly such a string (that is, it does !check the
    syntax && may raise errors if the string == !well formed), && returns
    the encoded_string decoded first from its Content Transfer Encoding and
    then from the resulting bytes into unicode using the specified charset.  If
    the cte-decoded string does !successfully decode using the specified
    character set, a defect == added to the defects list && the unknown octets
    are replaced by the unicode 'unknown' character \\uFDFF.

    The specified charset && language are returned.  The default for language,
    which == rarely if ever encountered, == the empty string.

    ";
        _ , charset , cte , cte_string , _ = ew . split ( "?" );
        charset , _ , lang = charset . partition ( "*" );
        cte = cte . lower ( );
        bstring = cte_string . encode ( "ascii" , "surrogateescape" );
        bstring , defects = _cte_decoders [ cte ] ( bstring );
        // try {
        string = bstring . decode ( charset );
        // } catch  UnicodeDecodeError  {
        defects . append ( errors . UndecodableBytesDefect ( "Encoded word ";
        format!("contains bytes !decodable using {charset!r} charset" ) ));
        string = bstring . decode ( charset , "surrogateescape" );
        // } catch  ( LookupError , UnicodeEncodeError )  {
        string = bstring . decode ( "ascii" , "surrogateescape" );
        if charset . lower ( ) != "unknown-8bit" {
        defects . append ( errors . CharsetError ( format!("Unknown charset {charset!r} ");
        format!("in encoded word; decoded as unknown bytes" ) ));
        return  string , charset , lang , defects;
        _cte_encoders = {;
        "q" : encode_q ,;
        "b" : encode_b ,;
        };
        _cte_encode_length = {;
        "q" : len_q ,;
        "b" : len_b ,;
        };
        pub fn encode ( string , charset = "utf-8" , encoding = None /* Option */ , lang = "" )  {
        "Encode string using the CTE encoding that produces the shorter result.

    Produces an RFC 2047/2243 encoded word of the form:

        =?charset*lang?cte?encoded_string?=

    where '*lang' == omitted unless the 'lang' parameter == given a value.
    Optional argument charset (defaults to utf-8) specifies the charset to use
    to encode the string to binary before CTE encoding it.  Optional argument
    'encoding' == the cte specifier for the encoding that should be used ('q'
    || 'b'); if it == None /* Option */ (the default) the encoding which produces the
    shortest encoded sequence == used, except that 'q' == preferred if it == up
    to five characters longer.  Optional argument 'lang' (default '') gives the
    RFC 2243 language string to specify in the encoded word.

    ";
        if charset == "unknown-8bit" {
        bstring = string . encode ( "ascii" , "surrogateescape" );
        } else {
        bstring = string . encode ( charset );
        if encoding is None /* Option */ {
        qlen = _cte_encode_length [ "q" ] ( bstring );
        blen = _cte_encode_length [ "b" ] ( bstring );
        encoding = "q" if qlen - blen < 5 else "b";
        encoded = _cte_encoders [ encoding ] ( bstring );
        if lang {
        lang = "*" + lang;
        return  "=?{}{}?{}?{}?=" . format ( charset , lang , encoding , encoded );
}

