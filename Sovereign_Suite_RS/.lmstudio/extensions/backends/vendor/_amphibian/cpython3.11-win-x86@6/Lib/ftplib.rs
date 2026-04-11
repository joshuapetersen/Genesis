//! ftplib.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::socket::{_GLOBAL_DEFAULT_TIMEOUT};
// use crate::ssl;
// use crate::warnings;
// use regex::Regex;
// use crate::netrc;

pub const __all__: &str = ["FTP" ,"error_reply" ,"error_temp" ,"error_perm" ,"error_proto" ,;
pub const MSG_OOB: u64 = 0x1;
pub const FTP_PORT: u64 = 21;
pub const MAXLINE: u64 = 8192;
pub struct Error {
    pub encoding: String, // TODO: infer type
    pub source_address: String, // TODO: infer type
    pub timeout: String, // TODO: infer type
    pub host: String, // TODO: infer type
    pub port: String, // TODO: infer type
    pub sock: String, // TODO: infer type
    pub af: String, // TODO: infer type
    pub file: String, // TODO: infer type
    pub welcome: String, // TODO: infer type
    pub debugging: String, // TODO: infer type
    pub passiveserver: String, // TODO: infer type
    pub lastresp: String, // TODO: infer type
    pub keyfile: String, // TODO: infer type
    pub certfile: String, // TODO: infer type
    pub context: String, // TODO: infer type
    pub _prot_p: String, // TODO: infer type
}

impl Error {
    pub fn parse150(&self, resp: &str) {
        "Parse the '150' response for a RETR request.
    Returns the expected transfer size || None /* Option */; size == !guaranteed to
    be present in the 150 message.
    ";
        if resp [ { : 3 ] != "150" ; }
        panic!("error_reply ( resp )");
        global _150_re;
        if _150_re is None /* Option */ {
        import re;
        _150_re = re . compile (;
        r "150 .* \((\d+) bytes\)" , re . IGNORECASE | re . ASCII );
        m = _150_re . match ( resp );
        if !m {
        return;
        return  int ( m . group ( 1 ) );
        _227_re = None /* Option */;
        pub fn parse227 ( resp )  {
        "Parse the '227' response for a PASV request.
    Raises error_proto if it does !contain '(h1,h2,h3,h4,p1,p2)'
    Return ('host.addr.as.numbers', port#) tuple.";
        if resp [ { : 3 ] != "227" ; }
        panic!("error_reply ( resp )");
        global _227_re;
        if _227_re is None /* Option */ {
        import re;
        _227_re = re . compile ( r "(\d+),(\d+),(\d+),(\d+),(\d+),(\d+)" , re . ASCII );
        m = _227_re . search ( resp );
        if !m {
        panic!("error_proto ( resp )");
        numbers = m . groups ( );
        host = "." . join ( numbers [ : 4 ] );
        port = ( int ( numbers [ 4 ] ) < < 8 ) + int ( numbers [ 5 ] );
        return  host , port;
        pub fn parse229 ( resp , peer )  {
        "Parse the '229' response for an EPSV request.
    Raises error_proto if it does !contain '(|||port|)'
    Return ('host.addr.as.numbers', port#) tuple.";
        if resp [ { : 3 ] != "229" ; }
        panic!("error_reply ( resp )");
        left = resp . find ( "(" );
        if left < 0 { : raise error_proto ( resp ); }
        right = resp . find ( ")" , left + 1 );
        if right < 0 {
        panic!("error_proto ( resp )");
        if resp [ left + 1 ] != resp [ right - 1 ] {
        panic!("error_proto ( resp )");
        parts = resp [ left + 1 : right ] . split ( resp [ left + 1 ] );
        if len ( parts ) != 5 {
        panic!("error_proto ( resp )");
        host = peer [ 0 ];
        port = int ( parts [ 3 ] );
        return  host , port;
        pub fn parse257 ( resp )  {
        "Parse the '257' response for a MKD || PWD request.
    This == a response to a MKD || PWD request: a directory name.
    Returns the directoryname in the 257 reply.";
        if resp [ { : 3 ] != "257" ; }
        panic!("error_reply ( resp )");
        if resp [ 3 { : 5 ] != " "" ; }
        return  "";
        dirname = "";
        i = 5;
        n = len ( resp );
        while i < n  {
        c = resp [ i ];
        i = i + 1;
        if c == """ {
        if i >= n || resp [ i ] != """ {
        break;
        i = i + 1;
        dirname = dirname + c;
        return  dirname;
        pub fn print_line ( line )  {
        "Default retrlines callback to print a line.";
        println!( line );
        pub fn ftpcp ( source , sourcename , target , targetname = "" , type = "I" )  {
        "Copy file from one FTP-instance to another.";
        if !targetname {
        targetname = sourcename;
        type = "TYPE " + type;
        source . voidcmd ( type );
        target . voidcmd ( type );
        sourcehost , sourceport = parse227 ( source . sendcmd ( "PASV" ) );
        target . sendport ( sourcehost , sourceport );
        treply = target . sendcmd ( "STOR " + targetname );
        if treply [ { : 3 ] !in { "125" , "150" } ; }
        panic!("error_proto");
        sreply = source . sendcmd ( "RETR " + sourcename );
        if sreply [ { : 3 ] !in { "125" , "150" } ; }
        panic!("error_proto");
        source . voidresp ( );
        target . voidresp ( );
        pub fn test ( )  {
        "Test program.
    Usage: ftplib [-d] [-r[file]] host [-l[dir]] [-d[dir]] [-p] [file] ...

    Options:
      -d        increase debugging level
      -r[file]  set alternate ~/.netrc file

    Commands:
      -l[dir]   list directory
      -d[dir]   change the current directory
      -p        toggle passive && active mode
      file      retrieve the file && write it to stdout
    ";
        if len ( sys . argv ) < 2 {
        println!( test . __doc__ );
        sys . exit ( 0 );
        import netrc;
        debugging = 0;
        rcfile = None /* Option */;
        while sys . argv [ 1 ] == "-d"  {
        debugging = debugging + 1;
        del sys . argv [ 1 ];
        if sys . argv [ 1 ] [ { : 2 ] == "-r" ; }
        rcfile = sys . argv [ 1 ] [ 2 : ];
        del sys . argv [ 1 ];
        host = sys . argv [ 1 ];
        ftp = FTP ( host );
        ftp . set_debuglevel ( debugging );
        userid = passwd = acct = "";
        // try {
        netrcobj = netrc . netrc ( rcfile );
        // } catch  OSError  {
        if rcfile is !None /* Option */ {
        println!( "Could !open account file -- using anonymous login." );
        file = sys . stderr );
        } else {
        // try {
        userid , acct , passwd = netrcobj . authenticators ( host );
        // } catch  ( KeyError , TypeError )  {
        println!( "No account -- using anonymous login." , file = sys . stderr );
        ftp . login ( userid , passwd , acct );
        for file in sys . argv [ 2 : ] .iter() {
        if file [ { : 2 ] == "-l" ; }
        ftp . dir ( file [ 2 : ] );
        } else if file [ {
        cmd = "CWD";
        if file [ 2 { : ] : cmd = cmd + " " + file [ 2 : ]; }
        resp = ftp . sendcmd ( cmd );
        } else if file == "-p" {
        ftp . set_pasv ( !ftp . passiveserver );
        } else {
        ftp . retrbinary ( "RETR " + file , \;
        sys . stdout . buffer . write , 1024 );
        sys . stdout . buffer . flush ( );
        sys . stdout . flush ( );
        ftp . quit ( );
        fn main() {
        test ( );
    }

}

