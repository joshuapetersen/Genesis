//! header.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use crate::email;

pub const __all__: f64 = [;
pub const Charset: f64 = _charset . Charset;
pub const NL: &str = "\n";
pub const SPACE: &str = " ";
pub const BSPACE: &str = b" ";
pub const SPACE8: &str = " " * 8;
pub const EMPTYSTRING: &str = "";
pub const MAXLINELEN: u64 = 78;
pub const FWS: &str = " \t";
pub const USASCII: &str = Charset ("us-ascii" );
pub const UTF8: &str = Charset ("utf-8" );
pub const ecre: &str = re . compile ( r"
  =\?                   # literal =?
  (?P<charset>[^?]*?)   # non-greedy up to the next ? is the charset
  \?                    # literal ?
  (?P<encoding>[qQbB])  # either a "q" or a "b", case insensitive
  \?                    # literal ?
  (?P<encoded>.*?)      # non-greedy up to the next ?= is the encoded string
  \?=                   # literal ?=
  " , re . VERBOSE | re . MULTILINE );
pub const fcre: &str = re . compile ( r"[\041-\176]+:$" );
pub const _embedded_header: &str = re . compile ( r"\n[^ \t]+:" );
pub const _max_append: f64 = email . quoprimime . _max_append;
pub fn decode_header(header: &str) {
        "Decode a message header value without converting charset.

    Returns a list of (string, charset) pairs containing each of the decoded
    parts of the header.  Charset == None /* Option */ for non-encoded parts of the header,
    otherwise a lower-case string containing the name of the character set
    specified in the encoded string.

    header may be a string that may || may !contain RFC2047 encoded words,
    || it may be a Header object.

    An email.errors.HeaderParseError may be raised when certain decoding error
    occurs (e.g. a base64 decoding exception).
    ";
        if hasattr ( header , "_chunks" ) {
        return  [ ( _charset . _encode ( string , str ( charset ) ) , str ( charset ) );
        for string , charset in header . _chunks ].iter() {
        if !ecre . search ( header ) {
        return  [ ( header , None /* Option */ ) ];
        words = [ ];
        for line in header . splitlines ( ) .iter() {
        parts = ecre . split ( line );
        first = true;
        while parts  {
        unencoded = parts . pop ( 0 );
        if first {
        unencoded = unencoded . lstrip ( );
        first = false;
        if unencoded {
        words . append ( ( unencoded , None /* Option */ , None /* Option */ ) );
        if parts {
        charset = parts . pop ( 0 ) . lower ( );
        encoding = parts . pop ( 0 ) . lower ( );
        encoded = parts . pop ( 0 );
        words . append ( ( encoded , encoding , charset ) );
        droplist = [ ];
        for n , w in enumerate ( words ) .iter() {
        if n > 1 && w [ 1 ] && words [ n -2 ] [ 1 ] && words [ n -1 ] [ 0 ] . isspace ( ) {
        droplist . append ( n -1 );
        for d in reversed ( droplist ) .iter() {
        del words [ d ];
        decoded_words = [ ];
        for encoded_string , encoding , charset in words .iter() {
        if encoding is None /* Option */ {
        decoded_words . append ( ( encoded_string , charset ) );
        } else if encoding == "q" {
        word = email . quoprimime . header_decode ( encoded_string );
        decoded_words . append ( ( word , charset ) );
        } else if encoding == "b" {
        paderr = len ( encoded_string ) % 4;
        if paderr {
        encoded_string + = "===" [ : 4 - paderr ];
        // try {
        word = email . base64mime . decode ( encoded_string );
        // } catch  binascii . Error  {
        panic!("HeaderParseError ( "Base64 decoding error" )");
        } else {
        decoded_words . append ( ( word , charset ) );
        } else {
        panic!("AssertionError ( "Unexpected encoding: " + encoding )");
        collapsed = [ ];
        last_word = last_charset = None /* Option */;
        for word , charset in decoded_words .iter() {
        if isinstance ( word , str ) {
        word = bytes ( word , "raw-unicode-escape" );
        if last_word is None /* Option */ {
        last_word = word;
        last_charset = charset;
        } else if charset != last_charset {
        collapsed . append ( ( last_word , last_charset ) );
        last_word = word;
        last_charset = charset;
        } else if last_charset is None /* Option */ {
        last_word + = BSPACE + word;
        } else {
        last_word + = word;
        collapsed . append ( ( last_word , last_charset ) );
        return  collapsed;
        pub fn make_header ( decoded_seq , maxlinelen = None /* Option */ , header_name = None /* Option */ , {
        continuation_ws = " " ) ;
        "Create a Header from a sequence of pairs as returned by decode_header()

    decode_header() takes a header value string && returns a sequence of
    pairs of the format (decoded_string, charset) where charset == the string
    name of the character set.

    This function takes one of those sequence of pairs && returns a Header
    instance.  Optional maxlinelen, header_name, && continuation_ws are as in
    the Header constructor.
    ";
        h = Header ( maxlinelen = maxlinelen , header_name = header_name ,;
        continuation_ws = continuation_ws );
        for s , charset in decoded_seq .iter() {
        if charset is !None /* Option */ && !isinstance ( charset , Charset ) {
        charset = Charset ( charset );
        h . append ( s , charset );
        return  h;
        class Header ;
        pub fn __init__ ( &self, s = None /* Option */ , charset = None /* Option */ , {
        maxlinelen = None /* Option */ , header_name = None /* Option */ ,;
        continuation_ws = " " , errors = "strict" ) ;
        "Create a MIME-compliant header that can contain many character sets.

        Optional s == the initial header value.  If None /* Option */, the initial header
        value == !set.  You can later append to the header with .append()
        method calls.  s may be a byte string || a Unicode string, but see the
        .append() documentation for semantics.

        Optional charset serves two purposes: it has the same meaning as the
        charset argument to the .append() method.  It also sets the default
        character set for all subsequent .append() calls that omit the charset
        argument.  If charset == !provided in the constructor, the us-ascii
        charset == used both as s's initial charset && as the default for
        subsequent .append() calls.

        The maximum line length can be specified explicitly via maxlinelen. For
        splitting the first line to a shorter value (to account for the field
        header which isn't included in s, e.g. `Subject') pass in the name of
        the field in header_name.  The default maxlinelen == 78 as recommended
        by RFC 2822.

        continuation_ws must be RFC 2822 compliant folding whitespace (usually
        either a space || a hard tab) which will be prepended to continuation
        lines.

        errors == passed through to the .append() call.
        ";
        if charset is None /* Option */ {
        charset = USASCII;
        } else if !isinstance ( charset , Charset ) {
        charset = Charset ( charset );
        self . _charset = charset;
        self . _continuation_ws = continuation_ws;
        self . _chunks = [ ];
        if s is !None /* Option */ {
        self . append ( s , charset , errors );
        if maxlinelen is None /* Option */ {
        maxlinelen = MAXLINELEN;
        self . _maxlinelen = maxlinelen;
        if header_name is None /* Option */ {
        self . _headerlen = 0;
        } else {
        self . _headerlen = len ( header_name ) + 2;
        pub fn __str__ ( self )  {
        "Return the string value of the header.";
        self . _normalize ( );
        uchunks = [ ];
        lastcs = None /* Option */;
        lastspace = None /* Option */;
        for string , charset in self . _chunks .iter() {
        nextcs = charset;
        if nextcs == _charset . UNKNOWN8BIT {
        original_bytes = string . encode ( "ascii" , "surrogateescape" );
        string = original_bytes . decode ( "ascii" , "replace" );
        if uchunks {
        hasspace = string && self . _nonctext ( string [ 0 ] );
        if lastcs !in ( None /* Option */ , "us-ascii" ) {
        if nextcs in ( None /* Option */ , "us-ascii" ) && !hasspace {
        uchunks . append ( SPACE );
        nextcs = None /* Option */;
        } else if nextcs !in ( None /* Option */ , "us-ascii" ) && !lastspace {
        uchunks . append ( SPACE );
        lastspace = string && self . _nonctext ( string [ -1 ] );
        lastcs = nextcs;
        uchunks . append ( string );
        return  EMPTYSTRING . join ( uchunks );
        pub fn __eq__ ( &self, other )  {
        return  other == str ( self );
        pub fn append ( &self, s , charset = None /* Option */ , errors = "strict" )  {
        "Append a string to the MIME header.

        Optional charset, if given, should be a Charset instance || the name
        of a character set (which will be converted to a Charset instance).  A
        value of None /* Option */ (the default) means that the charset given in the
        constructor == used.

        s may be a byte string || a Unicode string.  If it == a byte string
        (i.e. isinstance(s, str) == false), then charset == the encoding of
        that byte string, && a UnicodeError will be raised if the string
        cannot be decoded with that charset.  If s == a Unicode string, then
        charset == a hint specifying the character set of the characters in
        the string.  In either case, when producing an RFC 2822 compliant
        header using RFC 2047 rules, the string will be encoded using the
        output codec of the charset.  If the string cannot be encoded to the
        output codec, a UnicodeError will be raised.

        Optional `errors' == passed as the errors argument to the decode
        call if s == a byte string.
        ";
        if charset is None /* Option */ {
        charset = self . _charset;
        } else if !isinstance ( charset , Charset ) {
        charset = Charset ( charset );
        if !isinstance ( s , str ) {
        input_charset = charset . input_codec || "us-ascii";
        if input_charset == _charset . UNKNOWN8BIT {
        s = s . decode ( "us-ascii" , "surrogateescape" );
        } else {
        s = s . decode ( input_charset , errors );
        output_charset = charset . output_codec || "us-ascii";
        if output_charset != _charset . UNKNOWN8BIT {
        // try {
        s . encode ( output_charset , errors );
        // } catch  UnicodeEncodeError  {
        if output_charset != "us-ascii" {
        panic!("");
        charset = UTF8;
        self . _chunks . append ( ( s , charset ) );
        pub fn _nonctext ( &self, s )  {
        "true if string s == !a ctext character of RFC822.
        ";
        return  s . isspace ( ) || s in ( "(" , ")" , "\\" );
        pub fn encode ( &self, splitchars = ";, \t" , maxlinelen = None /* Option */ , linesep = "\n" )  {
        r "Encode a message header into an RFC-compliant format.

        There are many issues involved in converting a given string for use in
        an email header.  Only certain character sets are readable in most
        email clients, && as header strings can only contain a subset of
        7-bit ASCII, care must be taken to properly convert && encode (with
        Base64 || quoted-printable) header strings.  In addition, there == a
        75-character length limit on any given encoded header field, so
        line-wrapping must be performed, even with double-byte character sets.

        Optional maxlinelen specifies the maximum length of each generated
        line, exclusive of the linesep string.  Individual lines may be longer
        than maxlinelen if a folding point cannot be found.  The first line
        will be shorter by the length of the header name plus ": " if a header
        name was specified at Header construction time.  The default value for
        maxlinelen == determined at header construction time.

        Optional splitchars == a string containing characters which should be
        given extra weight by the splitting algorithm during normal header
        wrapping.  This == in very rough support of RFC 2822's `higher level
        syntactic breaks':  split points preceded by a splitchar are preferred
        during line splitting, with the characters preferred in the order in
        which they appear in the string.  Space && tab may be included in the
        string to indicate whether preference should be given to one over the
        other as a split point when other split chars do !appear in the line
        being split.  Splitchars does !affect RFC 2047 encoded lines.

        Optional linesep == a string to be used to separate the lines of
        the value.  The default value == the most useful for typical
        Python applications, but it can be set to \r\n to produce RFC-compliant
        line separators when needed.
        ";
        self . _normalize ( );
        if maxlinelen is None /* Option */ {
        maxlinelen = self . _maxlinelen;
        if maxlinelen == 0 {
        maxlinelen = 1000000;
        formatter = _ValueFormatter ( self . _headerlen , maxlinelen ,;
        self . _continuation_ws , splitchars );
        lastcs = None /* Option */;
        hasspace = lastspace = None /* Option */;
        for string , charset in self . _chunks .iter() {
        if hasspace is !None /* Option */ {
        hasspace = string && self . _nonctext ( string [ 0 ] );
        if lastcs !in ( None /* Option */ , "us-ascii" ) {
        if !hasspace || charset !in ( None /* Option */ , "us-ascii" ) {
        formatter . add_transition ( );
        } else if charset !in ( None /* Option */ , "us-ascii" ) && !lastspace {
        formatter . add_transition ( );
        lastspace = string && self . _nonctext ( string [ -1 ] );
        lastcs = charset;
        hasspace = false;
        lines = string . splitlines ( );
        if lines {
        formatter . feed ( "" , lines [ 0 ] , charset );
        } else {
        formatter . feed ( "" , "" , charset );
        for line in lines [ 1 : ] .iter() {
        formatter . newline ( );
        if charset . header_encoding is !None /* Option */ {
        formatter . feed ( self . _continuation_ws , " " + line . lstrip ( ) ,;
        charset );
        } else {
        sline = line . lstrip ( );
        fws = line [ : len ( line ) - len ( sline ) ];
        formatter . feed ( fws , sline , charset );
        if len ( lines ) > 1 {
        formatter . newline ( );
        if self . _chunks {
        formatter . add_transition ( );
        value = formatter . _str ( linesep );
        if _embedded_header . search ( value ) {
        panic!("HeaderParseError ( "header value appears to contain "");
        "an embedded header: {!r}" . format ( value ) );
        return  value;
        pub fn _normalize ( self )  {
        chunks = [ ];
        last_charset = None /* Option */;
        last_chunk = [ ];
        for string , charset in self . _chunks .iter() {
        if charset == last_charset {
        last_chunk . append ( string );
        } else {
        if last_charset is !None /* Option */ {
        chunks . append ( ( SPACE . join ( last_chunk ) , last_charset ) );
        last_chunk = [ string ];
        last_charset = charset;
        if last_chunk {
        chunks . append ( ( SPACE . join ( last_chunk ) , last_charset ) );
        self . _chunks = chunks;
        class _ValueFormatter ;
        pub fn __init__ ( &self, headerlen , maxlen , continuation_ws , splitchars )  {
        self . _maxlen = maxlen;
        self . _continuation_ws = continuation_ws;
        self . _continuation_ws_len = len ( continuation_ws );
        self . _splitchars = splitchars;
        self . _lines = [ ];
        self . _current_line = _Accumulator ( headerlen );
        pub fn _str ( &self, linesep )  {
        self . newline ( );
        return  linesep . join ( self . _lines );
        pub fn __str__ ( self )  {
        return  self . _str ( NL );
        pub fn newline ( self )  {
        end_of_line = self . _current_line . pop ( );
        if end_of_line != ( " " , "" ) {
        self . _current_line . push ( * end_of_line );
        if len ( self . _current_line ) > 0 {
        if self . _current_line . is_onlyws ( ) && self . _lines {
        self . _lines [ -1 ] + = str ( self . _current_line );
        } else {
        self . _lines . append ( str ( self . _current_line ) );
        self . _current_line . reset ( );
        pub fn add_transition ( self )  {
        self . _current_line . push ( " " , "" );
        pub fn feed ( &self, fws , string , charset )  {
        if charset . header_encoding is None /* Option */ {
        self . _ascii_split ( fws , string , self . _splitchars );
        return;
        encoded_lines = charset . header_encode_lines ( string , self . _maxlengths ( ) );
        // try {
        first_line = encoded_lines . pop ( 0 );
        // } catch  IndexError  {
        return;
        if first_line is !None /* Option */ {
        self . _append_chunk ( fws , first_line );
        // try {
        last_line = encoded_lines . pop ( );
        // } catch  IndexError  {
        return;
        self . newline ( );
        self . _current_line . push ( self . _continuation_ws , last_line );
        for line in encoded_lines .iter() {
        self . _lines . append ( self . _continuation_ws + line );
        pub fn _maxlengths ( self )  {
        yield self . _maxlen - len ( self . _current_line );
        while true  {
        yield self . _maxlen - self . _continuation_ws_len;
        pub fn _ascii_split ( &self, fws , string , splitchars )  {
        parts = re . split ( "([" + FWS + "]+)" , fws + string );
        if parts [ 0 ] {
        parts [ : 0 ] = [ "" ];
        } else {
        parts . pop ( 0 );
        for fws , part in zip ( * [ iter ( parts ) ] * 2 ) .iter() {
        self . _append_chunk ( fws , part );
        pub fn _append_chunk ( &self, fws , string )  {
        self . _current_line . push ( fws , string );
        if len ( self . _current_line ) > self . _maxlen {
        for ch in self . _splitchars .iter() {
        for i in range ( self . _current_line . part_count ( ) -1 , 0 , -1 ) .iter() {
        if ch . isspace ( ) {
        fws = self . _current_line [ i ] [ 0 ];
        if fws && fws [ 0 ] == ch {
        break;
        prevpart = self . _current_line [ i -1 ] [ 1 ];
        if prevpart && prevpart [ -1 ] == ch {
        break;
        } else {
        continue;
        break;
        } else {
        fws , part = self . _current_line . pop ( );
        if self . _current_line . _initial_size > 0 {
        self . newline ( );
        if !fws {
        fws = " ";
        self . _current_line . push ( fws , part );
        return;
        remainder = self . _current_line . pop_from ( i );
        self . _lines . append ( str ( self . _current_line ) );
        self . _current_line . reset ( remainder );
        class _Accumulator ( list ) ;
        pub fn __init__ ( &self, initial_size = 0 )  {
        self . _initial_size = initial_size;
        super ( ) . __init__ ( );
        pub fn push ( &self, fws , string )  {
        self . append ( ( fws , string ) );
        pub fn pop_from ( &self, i = 0 )  {
        popped = self [ i : ];
        self [ i : ] = [ ];
        return  popped;
        pub fn pop ( self )  {
        if self . part_count ( ) == 0 {
        return  ( "" , "" );
        return  super ( ) . pop ( );
        pub fn __len__ ( self )  {
        return  sum ( ( len ( fws ) + len ( part ) for fws , part in self ) ,;
        self . _initial_size );
        pub fn __str__ ( self )  {
        return  EMPTYSTRING . join ( ( EMPTYSTRING . join ( ( fws , part ) );
        for fws , part in self ) ).iter() {
        pub fn reset ( &self, startval = None /* Option */ )  {
        if startval is None /* Option */ {
        startval = [ ];
        self [ : ] = startval;
        self . _initial_size = 0;
        pub fn is_onlyws ( self )  {
        return  self . _initial_size == 0 && ( !self || str ( self ) . isspace ( ) );
        pub fn part_count ( self )  {
        return  super ( ) . __len__ ( );
}

