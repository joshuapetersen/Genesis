//! literals.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;

pub const simple_escapes: &str = {"a" :"\a" ,;
pub fn escape(m: &str) {
        all , tail = m . group ( 0 , 1 );
        assert all . startswith ( "\\" );
        esc = simple_escapes . get ( tail );
        if esc is !None /* Option */ {
        return  esc;
        if tail . startswith ( "x" ) {
        hexes = tail [ 1 : ];
        if len ( hexes ) < 2 {
        panic!("ValueError ( "invalid hex string escape ('\\%s')" % tail )");
        // try {
        i = int ( hexes , 16 );
        // } catch  ValueError  {
        panic!("ValueError ( "invalid hex string escape ('\\%s')" % tail ) from None /* Option */");
        } else {
        // try {
        i = int ( tail , 8 );
        // } catch  ValueError  {
        panic!("ValueError ( "invalid octal string escape ('\\%s')" % tail ) from None /* Option */");
        return  chr ( i );
        pub fn evalString ( s )  {
        assert s . startswith ( "'" ) || s . startswith ( """ ) , repr ( s [ : 1 ] );
        q = s [ 0 ];
        if s [ { : 3 ] == q * 3 ; }
        q = q * 3;
        assert s . endswith ( q ) , repr ( s [ - len ( q ) : ] );
        assert len ( s ) >= 2 * len ( q );
        s = s [ len ( q ) : - len ( q ) ];
        return  re . sub ( r "\\(\'|\"|\\|[abfnrtv]|x.{0,2}|[0-7]{1,3})" , escape , s );
        pub fn test ( )  {
        for i in range ( 256 ) .iter() {
        c = chr ( i );
        s = repr ( c );
        e = evalString ( s );
        if e != c {
        println!( i , c , s , e );
        fn main() {
        test ( );
}

