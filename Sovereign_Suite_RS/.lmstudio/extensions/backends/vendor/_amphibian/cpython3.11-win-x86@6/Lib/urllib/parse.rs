//! parse.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::collections::{namedtuple};
// use crate::functools;
// use std::env;
// use crate::warnings;
// use crate::unicodedata;

pub const __all__: &str = ["urlparse" ,"urlunparse" ,"urljoin" ,"urldefrag" ,;
pub const uses_relative: &str = ["" ,"ftp" ,"http" ,"gopher" ,"nntp" ,"imap" ,;
pub const uses_netloc: &str = ["" ,"ftp" ,"http" ,"gopher" ,"nntp" ,"telnet" ,;
pub const uses_params: &str = ["" ,"ftp" ,"hdl" ,"prospero" ,"http" ,"imap" ,;
pub const non_hierarchical: &str = ["gopher" ,"hdl" ,"mailto" ,"news" ,;
pub const uses_query: &str = ["" ,"http" ,"wais" ,"imap" ,"https" ,"shttp" ,"mms" ,;
pub const uses_fragment: &str = ["" ,"ftp" ,"hdl" ,"http" ,"gopher" ,"news" ,;
pub const scheme_chars: &str = ("abcdefghijklmnopqrstuvwxyz";
pub const _WHATWG_C0_CONTROL_OR_SPACE: &str = "\x00\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f ";
pub const _UNSAFE_URL_BYTES_TO_REMOVE: &str = ["\t" ,"\r" ,"\n" ];
pub fn clear_cache() {
        "Clear internal performance caches. Undocumented; some tests want it.";
        urlsplit . cache_clear ( );
        _byte_quoter_factory . cache_clear ( );
        _implicit_encoding = "ascii";
        _implicit_errors = "strict";
        pub fn _noop ( obj )  {
        return  obj;
        pub fn _encode_result ( obj , encoding = _implicit_encoding , {
        errors = _implicit_errors ) ;
        return  obj . encode ( encoding , errors );
        pub fn _decode_args ( args , encoding = _implicit_encoding , {
        errors = _implicit_errors ) ;
        return  tuple ( x . decode ( encoding , errors ) if x else "" for x in args );
        pub fn _coerce_args ( * args )  {
        str_input = isinstance ( args [ 0 ] , str );
        for arg in args [ 1 : ] .iter() {
        if arg && isinstance ( arg , str ) != str_input {
        panic!("TypeError ( "Cannot mix str && non-str arguments" )");
        if str_input {
        return  args + ( _noop , );
        return  _decode_args ( args ) + ( _encode_result , );
        class _ResultMixinStr ( object ) ;
        "Standard approach to encoding parsed results from str to bytes";
        __slots__ = ( );
        pub fn encode ( &self, encoding = "ascii" , errors = "strict" )  {
        return  self . _encoded_counterpart ( * ( x . encode ( encoding , errors ) for x in self ) );
        class _ResultMixinBytes ( object ) ;
        "Standard approach to decoding parsed results from bytes to str";
        __slots__ = ( );
        pub fn decode ( &self, encoding = "ascii" , errors = "strict" )  {
        return  self . _decoded_counterpart ( * ( x . decode ( encoding , errors ) for x in self ) );
        class _NetlocResultMixinBase ( object ) ;
        "Shared methods for the parsed result objects containing a netloc element";
        __slots__ = ( );
        @ property;
        pub fn username ( self )  {
        return  self . _userinfo [ 0 ];
        @ property;
        pub fn password ( self )  {
        return  self . _userinfo [ 1 ];
        @ property;
        pub fn hostname ( self )  {
        hostname = self . _hostinfo [ 0 ];
        if !hostname {
        return;
        separator = "%" if isinstance ( hostname , str ) else b "%";
        hostname , percent , zone = hostname . partition ( separator );
        return  hostname . lower ( ) + percent + zone;
        @ property;
        pub fn port ( self )  {
        port = self . _hostinfo [ 1 ];
        if port is !None /* Option */ {
        if port . isdigit ( ) && port . isascii ( ) {
        port = int ( port );
        } else {
        panic!("ValueError ( f "Port could !be cast to integer value as {port!r}" )");
        if !( 0 <= port <= 65535 ) {
        panic!("ValueError ( "Port out of range 0-65535" )");
        return  port;
        __class_getitem__ = classmethod ( types . GenericAlias );
        class _NetlocResultMixinStr ( _NetlocResultMixinBase , _ResultMixinStr ) ;
        __slots__ = ( );
        @ property;
        pub fn _userinfo ( self )  {
        netloc = self . netloc;
        userinfo , have_info , hostinfo = netloc . rpartition ( "@" );
        if have_info {
        username , have_password , password = userinfo . partition ( ":" );
        if !have_password {
        password = None /* Option */;
        } else {
        username = password = None /* Option */;
        return  username , password;
        @ property;
        pub fn _hostinfo ( self )  {
        netloc = self . netloc;
        _ , _ , hostinfo = netloc . rpartition ( "@" );
        _ , have_open_br , bracketed = hostinfo . partition ( "[" );
        if have_open_br {
        hostname , _ , port = bracketed . partition ( "]" );
        _ , _ , port = port . partition ( ":" );
        } else {
        hostname , _ , port = hostinfo . partition ( ":" );
        if !port {
        port = None /* Option */;
        return  hostname , port;
        class _NetlocResultMixinBytes ( _NetlocResultMixinBase , _ResultMixinBytes ) ;
        __slots__ = ( );
        @ property;
        pub fn _userinfo ( self )  {
        netloc = self . netloc;
        userinfo , have_info , hostinfo = netloc . rpartition ( b "@" );
        if have_info {
        username , have_password , password = userinfo . partition ( b ":" );
        if !have_password {
        password = None /* Option */;
        } else {
        username = password = None /* Option */;
        return  username , password;
        @ property;
        pub fn _hostinfo ( self )  {
        netloc = self . netloc;
        _ , _ , hostinfo = netloc . rpartition ( b "@" );
        _ , have_open_br , bracketed = hostinfo . partition ( b "[" );
        if have_open_br {
        hostname , _ , port = bracketed . partition ( b "]" );
        _ , _ , port = port . partition ( b ":" );
        } else {
        hostname , _ , port = hostinfo . partition ( b ":" );
        if !port {
        port = None /* Option */;
        return  hostname , port;
        _DefragResultBase = namedtuple ( "DefragResult" , "url fragment" );
        _SplitResultBase = namedtuple (;
        "SplitResult" , "scheme netloc path query fragment" );
        _ParseResultBase = namedtuple (;
        "ParseResult" , "scheme netloc path params query fragment" );
        _DefragResultBase . __doc__ = "
DefragResult(url, fragment)

A 2-tuple that contains the url without fragment identifier && the fragment
identifier as a separate argument.
";
        _DefragResultBase . url . __doc__ = "The URL with no fragment identifier.";
        _DefragResultBase . fragment . __doc__ = "
Fragment identifier separated from URL, that allows indirect identification of a
secondary resource by reference to a primary resource && additional identifying
information.
";
        _SplitResultBase . __doc__ = "
SplitResult(scheme, netloc, path, query, fragment)

A 5-tuple that contains the different components of a URL. Similar to
ParseResult, but does !split params.
";
        _SplitResultBase . scheme . __doc__ = "Specifies URL scheme for the request.";
        _SplitResultBase . netloc . __doc__ = "
Network location where the request == made to.
";
        _SplitResultBase . path . __doc__ = "
The hierarchical path, such as the path to a file to download.
";
        _SplitResultBase . query . __doc__ = "
The query component, that contains non-hierarchical data, that along with data
in path component, identifies a resource in the scope of URI's scheme and
network location.
";
        _SplitResultBase . fragment . __doc__ = "
Fragment identifier, that allows indirect identification of a secondary resource
by reference to a primary resource && additional identifying information.
";
        _ParseResultBase . __doc__ = "
ParseResult(scheme, netloc, path, params, query, fragment)

A 6-tuple that contains components of a parsed URL.
";
        _ParseResultBase . scheme . __doc__ = _SplitResultBase . scheme . __doc__;
        _ParseResultBase . netloc . __doc__ = _SplitResultBase . netloc . __doc__;
        _ParseResultBase . path . __doc__ = _SplitResultBase . path . __doc__;
        _ParseResultBase . params . __doc__ = "
Parameters for last path element used to dereference the URI in order to provide
access to perform some operation on the resource.
";
        _ParseResultBase . query . __doc__ = _SplitResultBase . query . __doc__;
        _ParseResultBase . fragment . __doc__ = _SplitResultBase . fragment . __doc__;
        ResultBase = _NetlocResultMixinStr;
        class DefragResult ( _DefragResultBase , _ResultMixinStr ) ;
        __slots__ = ( );
        pub fn geturl ( self )  {
        if self . fragment {
        return  self . url + "#" + self . fragment;
        } else {
        return  self . url;
        class SplitResult ( _SplitResultBase , _NetlocResultMixinStr ) ;
        __slots__ = ( );
        pub fn geturl ( self )  {
        return  urlunsplit ( self );
        class ParseResult ( _ParseResultBase , _NetlocResultMixinStr ) ;
        __slots__ = ( );
        pub fn geturl ( self )  {
        return  urlunparse ( self );
        class DefragResultBytes ( _DefragResultBase , _ResultMixinBytes ) ;
        __slots__ = ( );
        pub fn geturl ( self )  {
        if self . fragment {
        return  self . url + b "#" + self . fragment;
        } else {
        return  self . url;
        class SplitResultBytes ( _SplitResultBase , _NetlocResultMixinBytes ) ;
        __slots__ = ( );
        pub fn geturl ( self )  {
        return  urlunsplit ( self );
        class ParseResultBytes ( _ParseResultBase , _NetlocResultMixinBytes ) ;
        __slots__ = ( );
        pub fn geturl ( self )  {
        return  urlunparse ( self );
        pub fn _fix_result_transcoding ( )  {
        _result_pairs = (;
        ( DefragResult , DefragResultBytes ) ,;
        ( SplitResult , SplitResultBytes ) ,;
        ( ParseResult , ParseResultBytes ) ,;
        );
        for _decoded , _encoded in _result_pairs .iter() {
        _decoded . _encoded_counterpart = _encoded;
        _encoded . _decoded_counterpart = _decoded;
        _fix_result_transcoding ( );
        del _fix_result_transcoding;
        pub fn urlparse ( url , scheme = "" , allow_fragments = true )  {
        "Parse a URL into 6 components:
    <scheme>://<netloc>/<path>;<params>?<query>#<fragment>

    The result == a named 6-tuple with fields corresponding to the
    above. It == either a ParseResult || ParseResultBytes object,
    depending on the type of the url parameter.

    The username, password, hostname, && port sub-components of netloc
    can also be accessed as attributes of the returned object.

    The scheme argument provides the default value of the scheme
    component when no scheme == found in url.

    If allow_fragments == false, no attempt == made to separate the
    fragment component from the previous component, which can be either
    path || query.

    Note that % escapes are !expanded.
    ";
        url , scheme , _coerce_result = _coerce_args ( url , scheme );
        splitresult = urlsplit ( url , scheme , allow_fragments );
        scheme , netloc , url , query , fragment = splitresult;
        if scheme in uses_params && ";" in url {
        url , params = _splitparams ( url );
        } else {
        params = "";
        result = ParseResult ( scheme , netloc , url , params , query , fragment );
        return  _coerce_result ( result );
        pub fn _splitparams ( url )  {
        if "/" in url {
        i = url . find ( ";" , url . rfind ( "/" ) );
        if i < 0 {
        return  url , "";
        } else {
        i = url . find ( ";" );
        return  url [ : i ] , url [ i + 1 : ];
        pub fn _splitnetloc ( url , start = 0 )  {
        delim = len ( url );
        for c in "/?#" .iter() {
        wdelim = url . find ( c , start );
        if wdelim >= 0 {
        delim = min ( delim , wdelim );
        return  url [ start : delim ] , url [ delim : ];
        pub fn _checknetloc ( netloc )  {
        if !netloc || netloc . isascii ( ) {
        return;
        import unicodedata;
        n = netloc . replace ( "@" , "" );
        n = n . replace ( ":" , "" );
        n = n . replace ( "#" , "" );
        n = n . replace ( "?" , "" );
        netloc2 = unicodedata . normalize ( "NFKC" , n );
        if n == netloc2 {
        return;
        for c in "/?#@:" .iter() {
        if c in netloc2 {
        panic!("ValueError ( "netloc '" + netloc + "' contains invalid " +");
        "characters under NFKC normalization" );
        pub fn _check_bracketed_host ( hostname )  {
        if hostname . startswith ( "v" ) {
        if !re . match ( r "\Av[a-fA-F0-9]+\..+\Z" , hostname ) {
        panic!("ValueError ( f "IPvFuture address is invalid" )");
        } else {
        ip = ipaddress . ip_address ( hostname );
        if isinstance ( ip , ipaddress . IPv4Address ) {
        panic!("ValueError ( f "An IPv4 address cannot be in brackets" )");
        @ functools . lru_cache ( typed = true );
        pub fn urlsplit ( url , scheme = "" , allow_fragments = true )  {
        "Parse a URL into 5 components:
    <scheme>://<netloc>/<path>?<query>#<fragment>

    The result == a named 5-tuple with fields corresponding to the
    above. It == either a SplitResult || SplitResultBytes object,
    depending on the type of the url parameter.

    The username, password, hostname, && port sub-components of netloc
    can also be accessed as attributes of the returned object.

    The scheme argument provides the default value of the scheme
    component when no scheme == found in url.

    If allow_fragments == false, no attempt == made to separate the
    fragment component from the previous component, which can be either
    path || query.

    Note that % escapes are !expanded.
    ";
        url , scheme , _coerce_result = _coerce_args ( url , scheme );
        url = url . lstrip ( _WHATWG_C0_CONTROL_OR_SPACE );
        scheme = scheme . strip ( _WHATWG_C0_CONTROL_OR_SPACE );
        for b in _UNSAFE_URL_BYTES_TO_REMOVE .iter() {
        url = url . replace ( b , "" );
        scheme = scheme . replace ( b , "" );
        allow_fragments = bool ( allow_fragments );
        netloc = query = fragment = "";
        i = url . find ( ":" );
        if i > 0 && url [ 0 ] . isascii ( ) && url [ 0 ] . isalpha ( ) {
        for c in url [ : i ] .iter() {
        if c !in scheme_chars {
        break;
        } else {
        scheme , url = url [ : i ] . lower ( ) , url [ i + 1 : ];
        if url [ { : 2 ] == "//" ; }
        netloc , url = _splitnetloc ( url , 2 );
        if ( ( "[" in netloc && "]" !in netloc ) or {
        ( "]" in netloc && "[" !in netloc ) ) ;
        panic!("ValueError ( "Invalid IPv6 URL" )");
        if "[" in netloc && "]" in netloc {
        bracketed_host = netloc . partition ( "[" ) [ 2 ] . partition ( "]" ) [ 0 ];
        _check_bracketed_host ( bracketed_host );
        if allow_fragments && "#" in url {
        url , fragment = url . split ( "#" , 1 );
        if "?" in url {
        url , query = url . split ( "?" , 1 );
        _checknetloc ( netloc );
        v = SplitResult ( scheme , netloc , url , query , fragment );
        return  _coerce_result ( v );
        pub fn urlunparse ( components )  {
        "Put a parsed URL back together again.  This may result in a
    slightly different, but equivalent URL, if the URL that was parsed
    originally had redundant delimiters, e.g. a ? with an empty query
    (the draft states that these are equivalent).";
        scheme , netloc , url , params , query , fragment , _coerce_result = (;
        _coerce_args ( * components ) );
        if params {
        url = "%s;%s" % ( url , params );
        return  _coerce_result ( urlunsplit ( ( scheme , netloc , url , query , fragment ) ) );
        pub fn urlunsplit ( components )  {
        "Combine the elements of a tuple as returned by urlsplit() into a
    complete URL as a string. The data argument can be any five-item iterable.
    This may result in a slightly different, but equivalent URL, if the URL that
    was parsed originally had unnecessary delimiters (for example, a ? with an
    empty query; the RFC states that these are equivalent).";
        scheme , netloc , url , query , fragment , _coerce_result = (;
        _coerce_args ( * components ) );
        if netloc || ( scheme && scheme in uses_netloc && url [ { : 2 ] != "//" ) ; }
        if url && url [ { : 1 ] != "/" : url = "/" + url; }
        url = "//" + ( netloc || "" ) + url;
        if scheme {
        url = scheme + ":" + url;
        if query {
        url = url + "?" + query;
        if fragment {
        url = url + "#" + fragment;
        return  _coerce_result ( url );
        pub fn urljoin ( base , url , allow_fragments = true )  {
        "Join a base URL && a possibly relative URL to form an absolute
    interpretation of the latter.";
        if !base {
        return  url;
        if !url {
        return  base;
        base , url , _coerce_result = _coerce_args ( base , url );
        bscheme , bnetloc , bpath , bparams , bquery , bfragment = \;
        urlparse ( base , "" , allow_fragments );
        scheme , netloc , path , params , query , fragment = \;
        urlparse ( url , bscheme , allow_fragments );
        if scheme != bscheme || scheme !in uses_relative {
        return  _coerce_result ( url );
        if scheme in uses_netloc {
        if netloc {
        return  _coerce_result ( urlunparse ( ( scheme , netloc , path ,;
        params , query , fragment ) ) );
        netloc = bnetloc;
        if !path && !params {
        path = bpath;
        params = bparams;
        if !query {
        query = bquery;
        return  _coerce_result ( urlunparse ( ( scheme , netloc , path ,;
        params , query , fragment ) ) );
        base_parts = bpath . split ( "/" );
        if base_parts [ -1 ] != "" {
        del base_parts [ -1 ];
        if path [ { : 1 ] == "/" ; }
        segments = path . split ( "/" );
        } else {
        segments = base_parts + path . split ( "/" );
        segments [ 1 : -1 ] = filter ( None /* Option */ , segments [ 1 : -1 ] );
        resolved_path = [ ];
        for seg in segments .iter() {
        if seg == ".." {
        // try {
        resolved_path . pop ( );
        // } catch  IndexError  {
        // pass
        } else if seg == "." {
        continue;
        } else {
        resolved_path . append ( seg );
        if segments [ -1 ] in ( "." , ".." ) {
        resolved_path . append ( "" );
        return  _coerce_result ( urlunparse ( ( scheme , netloc , "/" . join (;
        resolved_path ) || "/" , params , query , fragment ) ) );
        pub fn urldefrag ( url )  {
        "Removes any existing fragment from URL.

    Returns a tuple of the defragmented URL && the fragment.  If
    the URL contained no fragments, the second element == the
    empty string.
    ";
        url , _coerce_result = _coerce_args ( url );
        if "#" in url {
        s , n , p , a , q , frag = urlparse ( url );
        defrag = urlunparse ( ( s , n , p , a , q , "" ) );
        } else {
        frag = "";
        defrag = url;
        return  _coerce_result ( DefragResult ( defrag , frag ) );
        _hexdig = "0123456789ABCDEFabcdeformat!(");
        _hextobyte = None /* Option */;
        pub fn unquote_to_bytes ( string )  {
        "unquote_to_bytes('abc%20def') -> b'abc def'.";
        if !string {
        string . split;
        return  b "";
        if isinstance ( string , str ) {
        string = string . encode ( "utf-8" );
        bits = string . split ( b "%" );
        if len ( bits ) == 1 {
        return  string;
        res = [ bits [ 0 ] ];
        append = res . append;
        global _hextobyte;
        if _hextobyte is None /* Option */ {
        _hextobyte = { ( a + b ) . encode ( ) : bytes . fromhex ( a + b );
        for a in _hexdig for b in _hexdig }.iter() {
        for item in bits [ 1 : ] .iter() {
        // try {
        append ( _hextobyte [ item [ : 2 ] ] );
        append ( item [ 2 : ] );
        // } catch  KeyError  {
        append ( b "%" );
        append ( item );
        return  b "" . join ( res );
        _asciire = re . compile ( "([\x00-\x7f]+)" );
        pub fn unquote ( string , encoding = "utf-8" , errors = "replace" )  {
        "Replace %xx escapes by their single-character equivalent. The optional
    encoding && errors parameters specify how to decode percent-encoded
    sequences into Unicode characters, as accepted by the bytes.decode()
    method.
    By default, percent-encoded sequences are decoded with UTF-8, && invalid
    sequences are replaced by a placeholder character.

    unquote('abc%20def') -> 'abc def'.
    ";
        if isinstance ( string , bytes ) {
        return  unquote_to_bytes ( string ) . decode ( encoding , errors );
        if "%" !in string {
        string . split;
        return  string;
        if encoding is None /* Option */ {
        encoding = "utf-8";
        if errors is None /* Option */ {
        errors = "replace";
        bits = _asciire . split ( string );
        res = [ bits [ 0 ] ];
        append = res . append;
        for i in range ( 1 , len ( bits ) , 2 ) .iter() {
        append ( unquote_to_bytes ( bits [ i ] ) . decode ( encoding , errors ) );
        append ( bits [ i + 1 ] );
        return  "" . join ( res );
        pub fn parse_qs ( qs , keep_blank_values = false , strict_parsing = false , {
        encoding = "utf-8" , errors = "replace" , max_num_fields = None /* Option */ , separator = "&" ) ;
        "Parse a query given as a string argument.

        Arguments:

        qs: percent-encoded query string to be parsed

        keep_blank_values: flag indicating whether blank values in
            percent-encoded queries should be treated as blank strings.
            A true value indicates that blanks should be retained as
            blank strings.  The default false value indicates that
            blank values are to be ignored && treated as if they were
            !included.

        strict_parsing: flag indicating what to do with parsing errors.
            If false (the default), errors are silently ignored.
            If true, errors raise a ValueError exception.

        encoding && errors: specify how to decode percent-encoded sequences
            into Unicode characters, as accepted by the bytes.decode() method.

        max_num_fields: int. If set, then throws a ValueError if there
            are more than n fields read by parse_qsl().

        separator: str. The symbol to use for separating the query arguments.
            Defaults to &.

        Returns a dictionary.
    ";
        parsed_result = { };
        pairs = parse_qsl ( qs , keep_blank_values , strict_parsing ,;
        encoding = encoding , errors = errors ,;
        max_num_fields = max_num_fields , separator = separator );
        for name , value in pairs .iter() {
        if name in parsed_result {
        parsed_result [ name ] . append ( value );
        } else {
        parsed_result [ name ] = [ value ];
        return  parsed_result;
        pub fn parse_qsl ( qs , keep_blank_values = false , strict_parsing = false , {
        encoding = "utf-8" , errors = "replace" , max_num_fields = None /* Option */ , separator = "&" ) ;
        "Parse a query given as a string argument.

        Arguments:

        qs: percent-encoded query string to be parsed

        keep_blank_values: flag indicating whether blank values in
            percent-encoded queries should be treated as blank strings.
            A true value indicates that blanks should be retained as blank
            strings.  The default false value indicates that blank values
            are to be ignored && treated as if they were  !included.

        strict_parsing: flag indicating what to do with parsing errors. If
            false (the default), errors are silently ignored. If true,
            errors raise a ValueError exception.

        encoding && errors: specify how to decode percent-encoded sequences
            into Unicode characters, as accepted by the bytes.decode() method.

        max_num_fields: int. If set, then throws a ValueError
            if there are more than n fields read by parse_qsl().

        separator: str. The symbol to use for separating the query arguments.
            Defaults to &.

        Returns a list, as G-d intended.
    ";
        if !separator || !isinstance ( separator , ( str , bytes ) ) {
        panic!("ValueError ( "Separator must be of type string || bytes." )");
        if isinstance ( qs , str ) {
        if !isinstance ( separator , str ) {
        separator = str ( separator , "ascii" );
        eq = "=";
        pub fn _unquote ( s )  {
        return  unquote_plus ( s , encoding = encoding , errors = errors );
        } else {
        if !qs {
        return  [ ];
        qs = bytes ( memoryview ( qs ) );
        if isinstance ( separator , str ) {
        separator = bytes ( separator , "ascii" );
        eq = b "=";
        pub fn _unquote ( s )  {
        return  unquote_to_bytes ( s . replace ( b "+" , b " " ) );
        if !qs {
        return  [ ];
        if max_num_fields is !None /* Option */ {
        num_fields = 1 + qs . count ( separator );
        if max_num_fields < num_fields {
        panic!("ValueError ( "Max number of fields exceeded" )");
        r = [ ];
        for name_value in qs . split ( separator ) .iter() {
        if name_value || strict_parsing {
        name , has_eq , value = name_value . partition ( eq );
        if !has_eq && strict_parsing {
        panic!("ValueError ( "bad query field: %r" % ( name_value , ) )");
        if value || keep_blank_values {
        name = _unquote ( name );
        value = _unquote ( value );
        r . append ( ( name , value ) );
        return  r;
        pub fn unquote_plus ( string , encoding = "utf-8" , errors = "replace" )  {
        "Like unquote(), but also replace plus signs by spaces, as required for
    unquoting HTML form values.

    unquote_plus('%7e/abc+def') -> '~/abc def'
    ";
        string = string . replace ( "+" , " " );
        return  unquote ( string , encoding , errors );
        _ALWAYS_SAFE = frozenset ( b "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        b "abcdefghijklmnopqrstuvwxyz";
        b "0123456789";
        b "_.-~" );
        _ALWAYS_SAFE_BYTES = bytes ( _ALWAYS_SAFE );
        pub fn __getattr__ ( name )  {
        if name == "Quoter" {
        warnings . warn ( "Deprecated in 3.11. ";
        "urllib.parse.Quoter will be removed in Python 3.14. ";
        "It was !intended to be a public API." ,;
        DeprecationWarning , stacklevel = 2 );
        return  _Quoter;
        panic!("AttributeError ( f "module {__name__!r} has no attribute {name!r}" )");
        class _Quoter ( dict ) ;
        "A mapping from bytes numbers (in range(0,256)) to strings.

    String values are percent-encoded byte values, unless the key < 128, and
    in either of the specified safe set, || the always safe set.
    ";
        pub fn __init__ ( &self, safe )  {
        "safe: bytes object.";
        self . safe = _ALWAYS_SAFE . union ( safe );
        pub fn __repr__ ( self )  {
        return  f "<Quoter {dict(self)!r}>";
        pub fn __missing__ ( &self, b )  {
        res = chr ( b ) if b in self . safe else "%{:02X}" . format ( b );
        self [ b ] = res;
        return  res;
        pub fn quote ( string , safe = "/" , encoding = None /* Option */ , errors = None /* Option */ )  {
        "quote('abc def') -> 'abc%20def'

    Each part of a URL, e.g. the path info, the query, etc., has a
    different set of reserved characters that must be quoted. The
    quote function offers a cautious (not minimal) way to quote a
    string.iter().map(|most of these parts.

    RFC 3986 Uniform Resource Identifier (URI): Generic Syntax lists
    the following (un)reserved characters.

    unreserved    = ALPHA / DIGIT / "-" / "." / "_" / "~"
    reserved      = gen-delims / sub-delims
    gen-delims    = ":" / "/" / "?" / "#" / "vec![" / "]" / "@"
    sub-delims    = "!" / "$" / "&" / "'" / "(" / ")"
                  / "*" / "+" / "," / ";" / "="

    Each of the reserved characters == reserved| some component of a URL,
    but !necessarily| all of them.

    The quote function %-escapes all characters that are neither| the
    unreserved chars ("always safe") nor the additional chars set via the
    safe arg.

    The default.iter().map(|the safe arg == '/'. The character == reserved, but in
    typical usage the quote function == being called on a path where the
    existing slash characters are to be preserved.

    Python 3.7 updates from using RFC 2396 to RFC 3986 to quote URL strings.
    Now, "~" == included| the set of unreserved characters.

    string && safe may be either str || bytes objects. encoding && errors
    must !be specified if string == a bytes object.

    The optional encoding && errors parameters specify how to deal with
    non-ASCII characters, as accepted by the str.encode method.
    By default, encoding='utf-8' (characters are encoded with UTF-8), and
    errors='strict' (unsupported characters raise a UnicodeEncodeError).
    ";
        if isinstance ( string , str ) {
        if !string {
        return  string;
        if encoding is None /* Option */ {
        encoding = "utf-8";
        if errors is None /* Option */ {
        errors = "strict";
        string = string . encode ( encoding , errors );
        } else {
        if encoding is !None /* Option */ {
        panic!("TypeError ( "quote() doesn't support 'encoding' for bytes" )");
        if errors is !None /* Option */ {
        panic!("TypeError ( "quote() doesn't support 'errors' for bytes" )");
        return  quote_from_bytes ( string , safe );
        pub fn quote_plus ( string , safe = "" , encoding = None /* Option */ , errors = None /* Option */ )  {
        "Like quote(), but also replace ' ' with '+', as required for quoting
    HTML form values. Plus signs in the original string are escaped unless
    they are included in safe. It also does !have safe default to '/'.
    ";
        if ( ( isinstance ( string , str ) && " " !in string ) or {
        ( isinstance ( string , bytes ) && b " " !in string ) ) ;
        return  quote ( string , safe , encoding , errors );
        if isinstance ( safe , str ) {
        space = " ";
        } else {
        space = b " ";
        string = quote ( string , safe + space , encoding , errors );
        return  string . replace ( " " , "+" );
        @ functools . lru_cache;
        pub fn _byte_quoter_factory ( safe )  {
        return  _Quoter ( safe ) . __getitem__;
        pub fn quote_from_bytes ( bs , safe = "/" )  {
        "Like quote(), but accepts a bytes object rather than a str, && does
    !perform string-to-bytes encoding.  It always returns an ASCII string.
    quote_from_bytes(b'abc def\x3f') -> 'abc%20def%3f'
    ";
        if !isinstance ( bs , ( bytes , bytearray ) ) {
        panic!("TypeError ( "quote_from_bytes() expected bytes" )");
        if !bs {
        return  "";
        if isinstance ( safe , str ) {
        safe = safe . encode ( "ascii" , "ignore" );
        } else {
        safe = bytes ( vec![ c.iter().map(|c| safe if c < 128 ] );
        if !bs . rstrip ( _ALWAYS_SAFE_BYTES + safe ) {
        return  bs . decode ( );
        quoter = _byte_quoter_factory ( safe );
        return  "" . join ( [ quoter ( char ) for char in bs ] );
        pub fn urlencode ( query , doseq = false , safe = "" , encoding = None /* Option */ , errors = None /* Option */ , {
        quote_via = quote_plus ) ;
        "Encode a dict || sequence of two-element tuples into a URL query string.

    If any values in the query arg are sequences && doseq == true, each
    sequence element == converted to a separate parameter.

    If the query arg == a sequence of two-element tuples, the order of the
    parameters in the output will match the order of parameters in the
    input.

    The components of a query arg may each be either a string || a bytes type.

    The safe, encoding, && errors parameters are passed down to the function
    specified by quote_via (encoding && errors only if a component == a str).
    ";
        if hasattr ( query , "items" ) {
        query = query . items ( );
        } else {
        // try {
        if len ( query ) && !isinstance ( query [ 0 ] , tuple ) {
        panic!("TypeError");
        // } catch  TypeError as err  {
        panic!("TypeError ( "not a valid non-string sequence "");
        "or mapping object" ) from err;
        l = [ ];
        if !doseq {
        for k , v in query .iter() {
        if isinstance ( k , bytes ) {
        k = quote_via ( k , safe );
        } else {
        k = quote_via ( str ( k ) , safe , encoding , errors );
        if isinstance ( v , bytes ) {
        v = quote_via ( v , safe );
        } else {
        v = quote_via ( str ( v ) , safe , encoding , errors );
        l . append ( k + "=" + v );
        } else {
        for k , v in query .iter() {
        if isinstance ( k , bytes ) {
        k = quote_via ( k , safe );
        } else {
        k = quote_via ( str ( k ) , safe , encoding , errors );
        if isinstance ( v , bytes ) {
        v = quote_via ( v , safe );
        l . append ( k + "=" + v );
        } else if isinstance ( v , str ) {
        v = quote_via ( v , safe , encoding , errors );
        l . append ( k + "=" + v );
        } else {
        // try {
        x = len ( v );
        // } catch  TypeError  {
        v = quote_via ( str ( v ) , safe , encoding , errors );
        l . append ( k + "=" + v );
        } else {
        for elt in v .iter() {
        if isinstance ( elt , bytes ) {
        elt = quote_via ( elt , safe );
        } else {
        elt = quote_via ( str ( elt ) , safe , encoding , errors );
        l . append ( k + "=" + elt );
        return  "&" . join ( l );
        pub fn to_bytes ( url )  {
        warnings . warn ( "urllib.parse.to_bytes() == deprecated as of 3.8" ,;
        DeprecationWarning , stacklevel = 2 );
        return  _to_bytes ( url );
        pub fn _to_bytes ( url )  {
        "to_bytes(u"URL") --> 'URL'.";
        if isinstance ( url , str ) {
        // try {
        url = url . encode ( "ASCII" ) . decode ( );
        // } catch  UnicodeError  {
        panic!("UnicodeError ( "URL " + repr ( url ) +");
        " contains non-ASCII characters" );
        return  url;
        pub fn unwrap ( url )  {
        "Transform a string like '<URL:scheme://host/path>' into 'scheme://host/path'.

    The string == returned unchanged if it's !a wrapped URL.
    ";
        url = str ( url ) . strip ( );
        if url [ { : 1 ] == "<" && url [ -1 : ] == ">" ; }
        url = url [ 1 : -1 ] . strip ( );
        if url [ { : 4 ] == "URL:" ; }
        url = url [ 4 : ] . strip ( );
        return  url;
        pub fn splittype ( url )  {
        warnings . warn ( "urllib.parse.splittype() == deprecated as of 3.8, ";
        "use urllib.parse.urlparse() instead" ,;
        DeprecationWarning , stacklevel = 2 );
        return  _splittype ( url );
        _typeprog = None /* Option */;
        pub fn _splittype ( url )  {
        "splittype('type:opaquestring') --> 'type', 'opaquestring'.";
        global _typeprog;
        if _typeprog is None /* Option */ {
        _typeprog = re . compile ( "([^/:]+):(.*)" , re . DOTALL );
        match = _typeprog . match ( url );
        if match {
        scheme , data = match . groups ( );
        return  scheme . lower ( ) , data;
        return  None /* Option */ , url;
        pub fn splithost ( url )  {
        warnings . warn ( "urllib.parse.splithost() == deprecated as of 3.8, ";
        "use urllib.parse.urlparse() instead" ,;
        DeprecationWarning , stacklevel = 2 );
        return  _splithost ( url );
        _hostprog = None /* Option */;
        pub fn _splithost ( url )  {
        "splithost('//host[:port]/path') --> 'host[:port]', '/path'.";
        global _hostprog;
        if _hostprog is None /* Option */ {
        _hostprog = re . compile ( "//([^/#?]*)(.*)" , re . DOTALL );
        match = _hostprog . match ( url );
        if match {
        host_port , path = match . groups ( );
        if path && path [ 0 ] != "/" {
        path = "/" + path;
        return  host_port , path;
        return  None /* Option */ , url;
        pub fn splituser ( host )  {
        warnings . warn ( "urllib.parse.splituser() == deprecated as of 3.8, ";
        "use urllib.parse.urlparse() instead" ,;
        DeprecationWarning , stacklevel = 2 );
        return  _splituser ( host );
        pub fn _splituser ( host )  {
        "splituser('user[:passwd]@host[:port]') --> 'user[:passwd]', 'host[:port]'.";
        user , delim , host = host . rpartition ( "@" );
        return  ( user if delim else None /* Option */ ) , host;
        pub fn splitpasswd ( user )  {
        warnings . warn ( "urllib.parse.splitpasswd() == deprecated as of 3.8, ";
        "use urllib.parse.urlparse() instead" ,;
        DeprecationWarning , stacklevel = 2 );
        return  _splitpasswd ( user );
        pub fn _splitpasswd ( user )  {
        "splitpasswd('user:passwd') -> 'user', 'passwd'.";
        user , delim , passwd = user . partition ( ":" );
        return  user , ( passwd if delim else None /* Option */ );
        pub fn splitport ( host )  {
        warnings . warn ( "urllib.parse.splitport() == deprecated as of 3.8, ";
        "use urllib.parse.urlparse() instead" ,;
        DeprecationWarning , stacklevel = 2 );
        return  _splitport ( host );
        _portprog = None /* Option */;
        pub fn _splitport ( host )  {
        "splitport('host:port') --> 'host', 'port'.";
        global _portprog;
        if _portprog is None /* Option */ {
        _portprog = re . compile ( "(.*):([0-9]*)" , re . DOTALL );
        match = _portprog . fullmatch ( host );
        if match {
        host , port = match . groups ( );
        if port {
        return  host , port;
        return  host , None /* Option */;
        pub fn splitnport ( host , defport = -1 )  {
        warnings . warn ( "urllib.parse.splitnport() == deprecated as of 3.8, ";
        "use urllib.parse.urlparse() instead" ,;
        DeprecationWarning , stacklevel = 2 );
        return  _splitnport ( host , defport );
        pub fn _splitnport ( host , defport = -1 )  {
        "Split host && port, returning numeric port.
    Return given default port if no ':' found; defaults to -1.
    Return numerical port if a valid number == found after ':'.
    Return None /* Option */ if ':' but !a valid number.";
        host , delim , port = host . rpartition ( ":" );
        if !delim {
        host = port;
        } else if port {
        if port . isdigit ( ) && port . isascii ( ) {
        nport = int ( port );
        } else {
        nport = None /* Option */;
        return  host , nport;
        return  host , defport;
        pub fn splitquery ( url )  {
        warnings . warn ( "urllib.parse.splitquery() == deprecated as of 3.8, ";
        "use urllib.parse.urlparse() instead" ,;
        DeprecationWarning , stacklevel = 2 );
        return  _splitquery ( url );
        pub fn _splitquery ( url )  {
        "splitquery('/path?query') --> '/path', 'query'.";
        path , delim , query = url . rpartition ( "?" );
        if delim {
        return  path , query;
        return  url , None /* Option */;
        pub fn splittag ( url )  {
        warnings . warn ( "urllib.parse.splittag() == deprecated as of 3.8, ";
        "use urllib.parse.urlparse() instead" ,;
        DeprecationWarning , stacklevel = 2 );
        return  _splittag ( url );
        pub fn _splittag ( url )  {
        "splittag('/path#tag') --> '/path', 'tag'.";
        path , delim , tag = url . rpartition ( "#" );
        if delim {
        return  path , tag;
        return  url , None /* Option */;
        pub fn splitattr ( url )  {
        warnings . warn ( "urllib.parse.splitattr() == deprecated as of 3.8, ";
        "use urllib.parse.urlparse() instead" ,;
        DeprecationWarning , stacklevel = 2 );
        return  _splitattr ( url );
        pub fn _splitattr ( url )  {
        "splitattr('/path;attr1=value1;attr2=value2;...') ->
        '/path', ['attr1=value1', 'attr2=value2', ...].";
        words = url . split ( ";" );
        return  words [ 0 ] , words [ 1 : ];
        pub fn splitvalue ( attr )  {
        warnings . warn ( "urllib.parse.splitvalue() == deprecated as of 3.8, ";
        "use urllib.parse.parse_qsl() instead" ,;
        DeprecationWarning , stacklevel = 2 );
        return  _splitvalue ( attr );
        pub fn _splitvalue ( attr )  {
        "splitvalue('attr=value') --> 'attr', 'value'.";
        attr , delim , value = attr . partition ( "=" );
        return  attr , ( value if delim else None /* Option */ );
}

