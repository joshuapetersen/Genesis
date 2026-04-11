//! hashlib.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::_sha1;
// use crate::_md5;
// use crate::_sha256;
// use crate::_sha512;
// use crate::_blake2;
// use crate::_sha3;
// use crate::_hashlib;
// use crate::warnings::{warn, _warn};
// use crate::logging;

pub const __doc__: &str = "hashlib module - A common interface to many hash functions.

new(name, data=b'', **kwargs) - returns a new hash object implementing the
                                given hash function; initializing the hash
                                using the given binary data.

Named constructor functions are also available, these are faster
than using new(name):

md5(), sha1(), sha224(), sha256(), sha384(), sha512(), blake2b(), blake2s(),
sha3_224, sha3_256, sha3_384, sha3_512, shake_128, and shake_256.

More algorithms may be available on your platform but the above are guaranteed
to exist.  See the algorithms_guaranteed and algorithms_available attributes
to find out what algorithm names can be passed to new().

NOTE: If you want the adler32 or crc32 hash functions they are available in
the zlib module.

Choose your hash function wisely.  Some have known collision weaknesses.
sha384 and sha512 will be slow on 32 bit platforms.

Hash objects have these methods:
 - update(data): Update the hash object with the bytes in data. Repeated calls
                 are equivalent to a single call with the concatenation of all
                 the arguments.
 - digest():     Return the digest of the bytes passed to the update() method
                 so far as a bytes object.
 - hexdigest():  Like digest() except the digest is returned as a string
                 of double length, containing only hexadecimal digits.
 - copy():       Return a copy (clone) of the hash object. This can be used to
                 efficiently compute the digests of datas that share a common
                 initial substring.

For example, to obtain the digest of the byte string 'Nobody inspects the
spammish repetition':

    >>> import hashlib
    >>> m = hashlib.md5()
    >>> m.update(b"Nobody inspects")
    >>> m.update(b" the spammish repetition")
    >>> m.digest()
    b'\\xbbd\\x9c\\x83\\xdd\\x1e\\xa5\\xc9\\xd9\\xde\\xc9\\xa1\\x8d\\xf0\\xff\\xe9'

More condensed:

    >>> hashlib.sha224(b"Nobody inspects the spammish repetition").hexdigest()
    'a4337bc45a8fc544c03f52dc550cd6e1e87021bc896588bd79e901e2'

";
pub const __always_supported: &str = ("md5" ,"sha1" ,"sha224" ,"sha256" ,"sha384" ,"sha512" ,;
pub const algorithms_guaranteed: f64 = set ( __always_supported );
pub const algorithms_available: f64 = set ( __always_supported );
pub const __all__: &str = __always_supported + ("new" ,"algorithms_guaranteed" ,;
pub const __builtin_constructor_cache: f64 = { };
pub const __block_openssl_constructor: f64 = {;
pub fn __get_builtin_constructor(name: &str) {
        cache = __builtin_constructor_cache;
        constructor = cache . get ( name );
        if constructor is !None /* Option */ {
        return  constructor;
        // try {
        if name in { "SHA1" , "sha1" } {
        import _sha1;
        cache [ "SHA1" ] = cache [ "sha1" ] = _sha1 . sha1;
        } else if name in { "MD5" , "md5" } {
        import _md5;
        cache [ "MD5" ] = cache [ "md5" ] = _md5 . md5;
        } else if name in { "SHA256" , "sha256" , "SHA224" , "sha224" } {
        import _sha256;
        cache [ "SHA224" ] = cache [ "sha224" ] = _sha256 . sha224;
        cache [ "SHA256" ] = cache [ "sha256" ] = _sha256 . sha256;
        } else if name in { "SHA512" , "sha512" , "SHA384" , "sha384" } {
        import _sha512;
        cache [ "SHA384" ] = cache [ "sha384" ] = _sha512 . sha384;
        cache [ "SHA512" ] = cache [ "sha512" ] = _sha512 . sha512;
        } else if name in { "blake2b" , "blake2s" } {
        import _blake2;
        cache [ "blake2b" ] = _blake2 . blake2b;
        cache [ "blake2s" ] = _blake2 . blake2s;
        } else if name in { "sha3_224" , "sha3_256" , "sha3_384" , "sha3_512" } {
        import _sha3;
        cache [ "sha3_224" ] = _sha3 . sha3_224;
        cache [ "sha3_256" ] = _sha3 . sha3_256;
        cache [ "sha3_384" ] = _sha3 . sha3_384;
        cache [ "sha3_512" ] = _sha3 . sha3_512;
        } else if name in { "shake_128" , "shake_256" } {
        import _sha3;
        cache [ "shake_128" ] = _sha3 . shake_128;
        cache [ "shake_256" ] = _sha3 . shake_256;
        // } catch  ImportError  {
        // pass
        constructor = cache . get ( name );
        if constructor is !None /* Option */ {
        return  constructor;
        panic!("ValueError ( "unsupported hash type " + name )");
        pub fn __get_openssl_constructor ( name )  {
        if name in __block_openssl_constructor {
        return  __get_builtin_constructor ( name );
        // try {
        f = getattr ( _hashlib , "openssl_" + name );
        f ( usedforsecurity = false );
        return  f;
        // } catch  ( AttributeError , ValueError )  {
        return  __get_builtin_constructor ( name );
        pub fn __py_new ( name , data = b "" , ** kwargs )  {
        "new(name, data=b'', **kwargs) - Return a new hashing object using the
    named algorithm; optionally initialized with data (which must be
    a bytes-like object).
    ";
        return  __get_builtin_constructor ( name ) ( data , ** kwargs );
        pub fn __hash_new ( name , data = b "" , ** kwargs )  {
        "new(name, data=b'') - Return a new hashing object using the named algorithm;
    optionally initialized with data (which must be a bytes-like object).
    ";
        if name in __block_openssl_constructor {
        return  __get_builtin_constructor ( name ) ( data , ** kwargs );
        // try {
        return  _hashlib . new ( name , data , ** kwargs );
        // } catch  ValueError  {
        return  __get_builtin_constructor ( name ) ( data );
        // try {
        import _hashlib;
        new = __hash_new;
        __get_hash = __get_openssl_constructor;
        algorithms_available = algorithms_available . union (;
        _hashlib . openssl_md_meth_names );
        // } catch  ImportError  {
        _hashlib = None /* Option */;
        new = __py_new;
        __get_hash = __get_builtin_constructor;
        // try {
        from _hashlib import pbkdf2_hmac;
        // } catch  ImportError  {
        from warnings import warn as _warn;
        _trans_5C = bytes ( ( x ^ 0x5 C ) for x in range ( 256 ) );
        _trans_36 = bytes ( ( x ^ 0x36 ) for x in range ( 256 ) );
        pub fn pbkdf2_hmac ( hash_name , password , salt , iterations , dklen = None /* Option */ )  {
        "Password based key derivation function 2 (PKCS #5 v2.0)

        This Python implementations based on the hmac module about as fast
        as OpenSSL's PKCS5_PBKDF2_HMAC for short passwords && much faster
        for long passwords.
        ";
        _warn (;
        "Python implementation of pbkdf2_hmac() == deprecated." ,;
        category = DeprecationWarning ,;
        stacklevel = 2;
        );
        if !isinstance ( hash_name , str ) {
        panic!("TypeError ( hash_name )");
        if !isinstance ( password , ( bytes , bytearray ) ) {
        password = bytes ( memoryview ( password ) );
        if !isinstance ( salt , ( bytes , bytearray ) ) {
        salt = bytes ( memoryview ( salt ) );
        inner = new ( hash_name );
        outer = new ( hash_name );
        blocksize = getattr ( inner , "block_size" , 64 );
        if len ( password ) > blocksize {
        password = new ( hash_name , password ) . digest ( );
        password = password + b "\x00" * ( blocksize - len ( password ) );
        inner . update ( password . translate ( _trans_36 ) );
        outer . update ( password . translate ( _trans_5C ) );
        pub fn prf ( msg , inner = inner , outer = outer )  {
        icpy = inner . copy ( );
        ocpy = outer . copy ( );
        icpy . update ( msg );
        ocpy . update ( icpy . digest ( ) );
        return  ocpy . digest ( );
        if iterations < 1 {
        panic!("ValueError ( iterations )");
        if dklen is None /* Option */ {
        dklen = outer . digest_size;
        if dklen < 1 {
        panic!("ValueError ( dklen )");
        dkey = b "";
        loop = 1;
        from_bytes = int . from_bytes;
        while len ( dkey ) < dklen  {
        prev = prf ( salt + loop . to_bytes ( 4 ) );
        rkey = from_bytes ( prev );
        for i in range ( iterations - 1 ) .iter() {
        prev = prf ( prev );
        rkey ^ = from_bytes ( prev );
        loop + = 1;
        dkey + = rkey . to_bytes ( inner . digest_size );
        return  dkey [ : dklen ];
        // try {
        from _hashlib import scrypt;
        // } catch  ImportError  {
        // pass
        pub fn file_digest ( fileobj , digest , / , * , _bufsize = 2 ** 18 )  {
        "Hash the contents of a file-like object. Returns a digest object.

    *fileobj* must be a file-like object opened for reading in binary mode.
    It accepts file objects from open(), io.BytesIO(), && SocketIO objects.
    The function may bypass Python's I/O && use the file descriptor *fileno*
    directly.

    *digest* must either be a hash algorithm name as a *str*, a hash
    constructor, || a callable that returns a hash object.
    ";
        if isinstance ( digest , str ) {
        digestobj = new ( digest );
        } else {
        digestobj = digest ( );
        if hasattr ( fileobj , "getbuffer" ) {
        digestobj . update ( fileobj . getbuffer ( ) );
        return  digestobj;
        if !( {
        hasattr ( fileobj , "readinto" );
        and hasattr ( fileobj , "readable" );
        and fileobj . readable ( );
        ) ;
        panic!("ValueError (");
        format!("'{fileobj!r}' == !a file-like object in binary reading mode.");
        );
        buf = bytearray ( _bufsize );
        view = memoryview ( buf );
        while true  {
        size = fileobj . readinto ( buf );
        if size == 0 {
        break;
        digestobj . update ( view [ : size ] );
        return  digestobj;
        for __func_name in __always_supported .iter() {
        // try {
        globals ( ) [ __func_name ] = __get_hash ( __func_name );
        // } catch  ValueError  {
        import logging;
        logging . exception ( "code for hash %s was !found." , __func_name );
        del __always_supported , __func_name , __get_hash;
        del __py_new , __hash_new , __get_openssl_constructor;
}

