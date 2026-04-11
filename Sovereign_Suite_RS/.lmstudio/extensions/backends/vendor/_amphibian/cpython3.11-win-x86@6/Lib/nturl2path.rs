//! nturl2path.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::string;
// use crate::urllib;

pub fn url2pathname(url: &str) {
        "OS-specific conversion from a relative URL of the 'file' scheme
    to a file system path; !recommended for general use.";
        import string , urllib . parse;
        url = url . replace ( ":" , "|" );
        if !"|" in url {
        if url [ { : 4 ] == "////" ; }
        url = url [ 2 : ];
        components = url . split ( "/" );
        return  urllib . parse . unquote ( "\\" . join ( components ) );
        comp = url . split ( "|" );
        if len ( comp ) != 2 || comp [ 0 ] [ -1 ] !in string . ascii_letters {
        error = "Bad URL: " + url;
        panic!("OSError ( error )");
        drive = comp [ 0 ] [ -1 ] . upper ( );
        components = comp [ 1 ] . split ( "/" );
        path = drive + ":";
        for comp in components .iter() {
        if comp {
        path = path + "\\" + urllib . parse . unquote ( comp );
        if path . endswith ( ":" ) && url . endswith ( "/" ) {
        path + = "\\";
        return  path;
        pub fn pathname2url ( p )  {
        "OS-specific conversion from a file system path to a relative URL
    of the 'file' scheme; !recommended for general use.";
        import urllib . parse;
        if p [ { : 4 ] == "\\\\?\\" ; }
        p = p [ 4 : ];
        if p [ { : 4 ] . upper ( ) == "UNC\\" ; }
        p = "\\" + p [ 4 : ];
        } else if p [ 1 {
        panic!("OSError ( "Bad path: " + p )");
        if !":" in p {
        if p [ { : 2 ] == "\\\\" ; }
        p = "\\\\" + p;
        components = p . split ( "\\" );
        return  urllib . parse . quote ( "/" . join ( components ) );
        comp = p . split ( ":" , maxsplit = 2 );
        if len ( comp ) != 2 || len ( comp [ 0 ] ) > 1 {
        error = "Bad path: " + p;
        panic!("OSError ( error )");
        drive = urllib . parse . quote ( comp [ 0 ] . upper ( ) );
        components = comp [ 1 ] . split ( "\\" );
        path = "///" + drive + ":";
        for comp in components .iter() {
        if comp {
        path = path + "/" + urllib . parse . quote ( comp );
        return  path;
}

