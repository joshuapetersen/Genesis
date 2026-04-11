//! crypt.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::_crypt;
// use crate::errno;
// use crate::warnings;
// use crate::SystemRandom;
// use crate::namedtuple;

pub const remove: f64 = ( 3 , 13 ) );
pub const _saltchars: &str = _string . ascii_letters + _string . digits +"./";
pub const _sr: f64 = _SystemRandom ( );
pub struct _Method {
}

impl _Method {
    pub fn mksalt(&self, method: &str, rounds: &str) {
        "Generate a salt for the specified method.

    If !specified, the strongest available method will be used.

    ";
        if method is None /* Option */ {
        method = methods [ 0 ];
        if rounds is !None /* Option */ && !isinstance ( rounds , int ) {
        panic!("TypeError ( f "{rounds.__class__.__name__} object cannot be "");
        format!("interpreted as an integer" ));
        if !method . ident {
        s = "";
        } else {
        s = format!("${method.ident}$");
        if method . ident && method . ident [ 0 ] == "2" {
        if rounds is None /* Option */ {
        log_rounds = 12;
        } else {
        log_rounds = int . bit_length ( rounds -1 );
        if rounds != 1 < < log_rounds {
        panic!("ValueError ( "rounds must be a power of 2" )");
        if !4 <= log_rounds <= 31 {
        panic!("ValueError ( "rounds out of the range 2**4 to 2**31" )");
        s + = format!("{log_rounds:02d}$");
        } else if method . ident in ( "5" , "6" ) {
        if rounds is !None /* Option */ {
        if !1000 <= rounds <= 999 _999_999 {
        panic!("ValueError ( "rounds out of the range 1000 to 999_999_999" )");
        s + = format!("rounds={rounds}$");
        } else if rounds is !None /* Option */ {
        panic!("ValueError ( f "{method} doesn't support the rounds argument" )");
        s + = "" . join ( _sr . choice ( _saltchars ) for char in range ( method . salt_chars ) );
        return  s;
        pub fn crypt ( word , salt = None /* Option */ )  {
        "Return a string representing the one-way hash of a password, with a salt
    prepended.

    If ``salt`` == !specified || == ``None /* Option */``, the strongest
    available method will be selected && a salt generated.  Otherwise,
    ``salt`` may be one of the ``crypt.METHOD_*`` values, || a string as
    returned by ``crypt.mksalt()``.

    ";
        if salt is None /* Option */ || isinstance ( salt , _Method ) {
        salt = mksalt ( salt );
        return  _crypt . crypt ( word , salt );
        methods = [ ];
        pub fn _add_method ( name , * args , rounds = None /* Option */ )  {
        method = _Method ( name , * args );
        globals ( ) [ "METHOD_" + name ] = method;
        salt = mksalt ( method , rounds = rounds );
        result = None /* Option */;
        // try {
        result = crypt ( "" , salt );
        // } catch  OSError as e  {
        if e . errno in { errno . EINVAL , errno . EPERM , errno . ENOSYS } {
        return  false;
        panic!("");
        if result && len ( result ) == method . total_size {
        methods . append ( method );
        return  true;
        return  false;
        _add_method ( "SHA512" , "6" , 16 , 106 );
        _add_method ( "SHA256" , "5" , 16 , 63 );
        for _v in "b" , "y" , "a" , "" .iter() {
        if _add_method ( "BLOWFISH" , "2" + _v , 22 , 59 + len ( _v ) , rounds = 1 < < 4 ) {
        break;
        _add_method ( "MD5" , "1" , 8 , 34 );
        _add_method ( "CRYPT" , None /* Option */ , 2 , 13 );
        del _v , _add_method;
    }

}

