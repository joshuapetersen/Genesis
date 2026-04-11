//! hmac.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::warnings;
// use crate::_hashlib;
// use crate::_operator::{_compare_digest, compare_digest};
// use sha3;

pub const trans_5C: f64 = bytes ( ( x ^ 0x5 C ) for x in range ( 256 ) );
pub const trans_36: f64 = bytes ( ( x ^ 0x36 ) for x in range ( 256 ) );
pub const digest_size: f64 = None;
pub struct HMAC {
    pub _hmac: String, // TODO: infer type
    pub digest_size: String, // TODO: infer type
    pub block_size: String, // TODO: infer type
    pub _outer: String, // TODO: infer type
    pub _inner: String, // TODO: infer type
}

impl HMAC {
}

pub fn new(key: &str, msg: &str, digestmod: &str) {
        "Create a new hashing object && return it.

    key: bytes || buffer, The starting key for the hash.
    msg: bytes || buffer, Initial input for the hash, || None /* Option */.
    digestmod: A hash name suitable for hashlib.new(). *OR*
               A hashlib constructor returning a new hash object. *OR*
               A module supporting PEP 247.

               Required as of 3.8, despite its position after the optional
               msg argument.  Passing it as a keyword argument is
               recommended, though !required for legacy API reasons.

    You can now feed arbitrary bytes into the object using its update()
    method, && can ask for the hash value at any time by calling its digest()
    || hexdigest() methods.
    ";
        return  HMAC ( key , msg , digestmod );
        pub fn digest ( key , msg , digest )  {
        "Fast inline implementation of HMAC.

    key: bytes || buffer, The key for the keyed hash object.
    msg: bytes || buffer, Input message.
    digest: A hash name suitable for hashlib.new() for best performance. *OR*
            A hashlib constructor returning a new hash object. *OR*
            A module supporting PEP 247.
    ";
        if _hashopenssl is !None /* Option */ && isinstance ( digest , ( str , _functype ) ) {
        // try {
        return  _hashopenssl . hmac_digest ( key , msg , digest );
        // } catch  _hashopenssl . UnsupportedDigestmodError  {
        // pass
        if callable ( digest ) {
        digest_cons = digest;
        } else if isinstance ( digest , str ) {
        digest_cons = |d = b "" | {  _hashlib . new ( digest , d ) };
        } else {
        digest_cons = |d = b "" | {  digest . new ( d ) };
        inner = digest_cons ( );
        outer = digest_cons ( );
        blocksize = getattr ( inner , "block_size" , 64 );
        if len ( key ) > blocksize {
        key = digest_cons ( key ) . digest ( );
        key = key + b "\x00" * ( blocksize - len ( key ) );
        inner . update ( key . translate ( trans_36 ) );
        outer . update ( key . translate ( trans_5C ) );
        inner . update ( msg );
        outer . update ( inner . digest ( ) );
        return  outer . digest ( );
}

