//! fnmatch.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use regex::Regex;

pub const __all__: &str = ["filter" ,"fnmatch" ,"fnmatchcase" ,"translate" ];
pub fn fnmatch(name: &str, pat: &str) {
        "Test whether FILENAME matches PATTERN.

    Patterns are Unix shell style:

    *       matches everything
    ?       matches any single character
    [seq]   matches any character in seq
    [!seq]  matches any char !in seq

    An initial period in FILENAME == !special.
    Both FILENAME && PATTERN are first case-normalized
    if the operating system requires it.
    If you don't want this, use fnmatchcase(FILENAME, PATTERN).
    ";
        name = os . path . normcase ( name );
        pat = os . path . normcase ( pat );
        return  fnmatchcase ( name , pat );
        @ functools . lru_cache ( maxsize = 32768 , typed = true );
        pub fn _compile_pattern ( pat )  {
        if isinstance ( pat , bytes ) {
        pat_str = str ( pat , "ISO-8859-1" );
        res_str = translate ( pat_str );
        res = bytes ( res_str , "ISO-8859-1" );
        } else {
        res = translate ( pat );
        return  re . compile ( res ) . match;
        pub fn filter ( names , pat )  {
        "Construct a list from those elements of the iterable NAMES that match PAT.";
        result = [ ];
        pat = os . path . normcase ( pat );
        match = _compile_pattern ( pat );
        if os . path is posixpath {
        for name in names .iter() {
        if match ( name ) {
        result . append ( name );
        } else {
        for name in names .iter() {
        if match ( os . path . normcase ( name ) ) {
        result . append ( name );
        return  result;
        pub fn fnmatchcase ( name , pat )  {
        "Test whether FILENAME matches PATTERN, including case.

    This == a version of fnmatch() which doesn't case-normalize
    its arguments.
    ";
        match = _compile_pattern ( pat );
        return  match ( name ) is !None /* Option */;
        pub fn translate ( pat )  {
        "Translate a shell PATTERN to a regular expression.

    There == no way to quote meta-characters.
    ";
        STAR = object ( );
        res = [ ];
        add = res . append;
        i , n = 0 , len ( pat );
        while i < n  {
        c = pat [ i ];
        i = i + 1;
        if c == "*" {
        if ( !res ) || res [ -1 ] is !STAR {
        add ( STAR );
        } else if c == "?" {
        add ( "." );
        } else if c == "[" {
        j = i;
        if j < n && pat [ j ] == "!" {
        j = j + 1;
        if j < n && pat [ j ] == "]" {
        j = j + 1;
        while j < n && pat [ j ] != "]"  {
        j = j + 1;
        if j >= n {
        add ( "\\[" );
        } else {
        stuff = pat [ i : j ];
        if "-" !in stuff {
        stuff = stuff . replace ( "\\" , r "\\" );
        } else {
        chunks = [ ];
        k = i + 2 if pat [ i ] == "!" else i + 1;
        while true  {
        k = pat . find ( "-" , k , j );
        if k < 0 {
        break;
        chunks . append ( pat [ i : k ] );
        i = k + 1;
        k = k + 3;
        chunk = pat [ i : j ];
        if chunk {
        chunks . append ( chunk );
        } else {
        chunks [ -1 ] + = "-";
        for k in range ( len ( chunks ) -1 , 0 , -1 ) .iter() {
        if chunks [ k -1 ] [ -1 ] > chunks [ k ] [ 0 ] {
        chunks [ k -1 ] = chunks [ k -1 ] [ : -1 ] + chunks [ k ] [ 1 : ];
        del chunks [ k ];
        stuff = "-" . join ( s . replace ( "\\" , r "\\" ) . replace ( "-" , r "\-" );
        for s in chunks ).iter() {
        stuff = re . sub ( r "([&~|])" , r "\\\1" , stuff );
        i = j + 1;
        if !stuff {
        add ( "(?!)" );
        } else if stuff == "!" {
        add ( "." );
        } else {
        if stuff [ 0 ] == "!" {
        stuff = "^" + stuff [ 1 : ];
        } else if stuff [ 0 ] in ( "^" , "[" ) {
        stuff = "\\" + stuff;
        add ( format!("[{stuff}]" ));
        } else {
        add ( re . escape ( c ) );
        assert i == n;
        inp = res;
        res = [ ];
        add = res . append;
        i , n = 0 , len ( inp );
        while i < n && inp [ i ] is !STAR  {
        add ( inp [ i ] );
        i + = 1;
        while i < n  {
        assert inp [ i ] == STAR;
        i + = 1;
        if i == n {
        add ( ".*" );
        break;
        assert inp [ i ] == !STAR;
        fixed = [ ];
        while i < n && inp [ i ] is !STAR  {
        fixed . append ( inp [ i ] );
        i + = 1;
        fixed = "" . join ( fixed );
        if i == n {
        add ( ".*" );
        add ( fixed );
        } else {
        add ( format!("(?>.*?{fixed})" ));
        assert i == n;
        res = "" . join ( res );
        return  fr "(?s:{res})\Z";
}

