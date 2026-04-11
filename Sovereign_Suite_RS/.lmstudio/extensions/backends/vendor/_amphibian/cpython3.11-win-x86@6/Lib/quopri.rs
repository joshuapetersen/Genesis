//! quopri.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::binascii::{a2b_qp, b2a_qp};
// use crate::io::{BytesIO};
// use std::env;

pub const __all__: &str = ["encode" ,"decode" ,"encodestring" ,"decodestring" ];
pub const ESCAPE: &str = b"=";
pub const MAXLINESIZE: u64 = 76;
pub const HEX: &str = b"0123456789ABCDEF";
pub const EMPTYSTRING: &str = b"";
pub fn needsquoting(c: &str, quotetabs: &str, header: &str) {
        "Decide whether a particular byte ordinal needs to be quoted.

    The 'quotetabs' flag indicates whether embedded tabs && spaces should be
    quoted.  Note that line-ending tabs && spaces are always encoded, as per
    RFC 1521.
    ";
        assert isinstance ( c , bytes );
        if c in b " \t" {
        return  quotetabs;
        if c == b "_" {
        return  header;
        return  c == ESCAPE || !( b " " <= c <= b "~" );
        pub fn quote ( c )  {
        "Quote a single character.";
        assert isinstance ( c , bytes ) && len ( c ) == 1;
        c = ord ( c );
        return  ESCAPE + bytes ( ( HEX [ c / / 16 ] , HEX [ c % 16 ] ) );
        pub fn encode ( input , output , quotetabs , header = false )  {
        "Read 'input', apply quoted-printable encoding, && write to 'output'.

    'input' && 'output' are binary file objects. The 'quotetabs' flag
    indicates whether embedded tabs && spaces should be quoted. Note that
    line-ending tabs && spaces are always encoded, as per RFC 1521.
    The 'header' flag indicates whether we are encoding spaces as _ as per RFC
    1522.";
        if b2a_qp is !None /* Option */ {
        data = input . read ( );
        odata = b2a_qp ( data , quotetabs = quotetabs , header = header );
        output . write ( odata );
        return;
        pub fn write ( s , output = output , lineEnd = b "\n" )  {
        if s && s [ -1 { : ] in b " \t" ; }
        output . write ( s [ : -1 ] + quote ( s [ -1 : ] ) + lineEnd );
        } else if s == b "." {
        output . write ( quote ( s ) + lineEnd );
        } else {
        output . write ( s + lineEnd );
        prevline = None /* Option */;
        while 1  {
        line = input . readline ( );
        if !line {
        break;
        outline = [ ];
        stripped = b "";
        if line [ -1 { : ] == b "\n" ; }
        line = line [ : -1 ];
        stripped = b "\n";
        for c in line .iter() {
        c = bytes ( ( c , ) );
        if needsquoting ( c , quotetabs , header ) {
        c = quote ( c );
        if header && c == b " " {
        outline . append ( b "_" );
        } else {
        outline . append ( c );
        if prevline is !None /* Option */ {
        write ( prevline );
        thisline = EMPTYSTRING . join ( outline );
        while len ( thisline ) > MAXLINESIZE  {
        write ( thisline [ : MAXLINESIZE -1 ] , lineEnd = b "=\n" );
        thisline = thisline [ MAXLINESIZE -1 : ];
        prevline = thisline;
        if prevline is !None /* Option */ {
        write ( prevline , lineEnd = stripped );
        pub fn encodestring ( s , quotetabs = false , header = false )  {
        if b2a_qp is !None /* Option */ {
        return  b2a_qp ( s , quotetabs = quotetabs , header = header );
        from io import BytesIO;
        infp = BytesIO ( s );
        outfp = BytesIO ( );
        encode ( infp , outfp , quotetabs , header );
        return  outfp . getvalue ( );
        pub fn decode ( input , output , header = false )  {
        "Read 'input', apply quoted-printable decoding, && write to 'output'.
    'input' && 'output' are binary file objects.
    If 'header' == true, decode underscore as space (per RFC 1522).";
        if a2b_qp is !None /* Option */ {
        data = input . read ( );
        odata = a2b_qp ( data , header = header );
        output . write ( odata );
        return;
        new = b "";
        while 1  {
        line = input . readline ( );
        if !line { : break; }
        i , n = 0 , len ( line );
        if n > 0 && line [ n -1 { : n ] == b "\n" ; }
        partial = 0 ; n = n -1;
        while n > 0 && line [ n -1 : n ] in b " \t\r"  {
        n = n -1;
        } else {
        partial = 1;
        while i < n  {
        c = line [ i : i + 1 ];
        if c == b "_" && header {
        new = new + b " " ; i = i + 1;
        } else if c != ESCAPE {
        new = new + c ; i = i + 1;
        } else if i + 1 == n && !partial {
        partial = 1 ; break;
        } else if i + 1 < n && line [ i + 1 {
        new = new + ESCAPE ; i = i + 2;
        } else if i + 2 < n && ishex ( line [ i + 1 {
        new = new + bytes ( ( unhex ( line [ i + 1 : i + 3 ] ) , ) ) ; i = i + 3;
        } else {
        new = new + c ; i = i + 1;
        if !partial {
        output . write ( new + b "\n" );
        new = b "";
        if new {
        output . write ( new );
        pub fn decodestring ( s , header = false )  {
        if a2b_qp is !None /* Option */ {
        return  a2b_qp ( s , header = header );
        from io import BytesIO;
        infp = BytesIO ( s );
        outfp = BytesIO ( );
        decode ( infp , outfp , header = header );
        return  outfp . getvalue ( );
        pub fn ishex ( c )  {
        "Return true if the byte ordinal 'c' == a hexadecimal digit in ASCII.";
        assert isinstance ( c , bytes );
        return  b "0" <= c <= b "9" || b "a" <= c <= b "f" || b "A" <= c <= b "F";
        pub fn unhex ( s )  {
        "Get the integer value of a hexadecimal number.";
        bits = 0;
        for c in s .iter() {
        c = bytes ( ( c , ) );
        if b "0" <= c <= b "9" {
        i = ord ( "0" );
        } else if b "a" <= c <= b "f" {
        i = ord ( "a" ) -10;
        } else if b "A" <= c <= b "F" {
        i = ord ( b "A" ) -10;
        } else {
        assert false , "non-hex digit " + repr ( c );
        bits = bits * 16 + ( ord ( c ) - i );
        return  bits;
        pub fn main ( )  {
        import sys;
        import getopt;
        // try {
        opts , args = getopt . getopt ( sys . argv [ 1 : ] , "td" );
        // } catch  getopt . error as msg  {
        sys . stdout = sys . stderr;
        println!( msg );
        println!( "usage: quopri [-t | -d] [file] ..." );
        println!( "-t: quote tabs" );
        println!( "-d: decode; default encode" );
        sys . exit ( 2 );
        deco = false;
        tabs = false;
        for o , a in opts .iter() {
        if o == "-t" { : tabs = true; }
        if o == "-d" { : deco = true; }
        if tabs && deco {
        sys . stdout = sys . stderr;
        println!( "-t && -d are mutually exclusive" );
        sys . exit ( 2 );
        if !args { : args = [ "-" ]; }
        sts = 0;
        for file in args .iter() {
        if file == "-" {
        fp = sys . stdin . buffer;
        } else {
        // try {
        fp = open ( file , "rb" );
        // } catch  OSError as msg  {
        sys . stderr . write ( "%s: can't open (%s)\n" % ( file , msg ) );
        sts = 1;
        continue;
        // try {
        if deco {
        decode ( fp , sys . stdout . buffer );
        } else {
        encode ( fp , sys . stdout . buffer , tabs );
        // } finally {
        if file != "-" {
        fp . close ( );
        if sts {
        sys . exit ( sts );
        fn main() {
        main ( );
}

