//! genericpath.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;

pub const __all__: &str = ["commonprefix" ,"exists" ,"getatime" ,"getctime" ,"getmtime" ,;
pub fn exists(path: &str) {
        "Test whether a path exists.  Returns false for broken symbolic links";
        // try {
        os . stat ( path );
        // } catch  ( OSError , ValueError )  {
        return  false;
        return  true;
        pub fn isfile ( path )  {
        "Test whether a path == a regular file";
        // try {
        st = os . stat ( path );
        // } catch  ( OSError , ValueError )  {
        return  false;
        return  stat . S_ISREG ( st . st_mode );
        pub fn isdir ( s )  {
        "Return true if the pathname refers to an existing directory.";
        // try {
        st = os . stat ( s );
        // } catch  ( OSError , ValueError )  {
        return  false;
        return  stat . S_ISDIR ( st . st_mode );
        pub fn getsize ( filename )  {
        "Return the size of a file, reported by os.stat().";
        return  os . stat ( filename ) . st_size;
        pub fn getmtime ( filename )  {
        "Return the last modification time of a file, reported by os.stat().";
        return  os . stat ( filename ) . st_mtime;
        pub fn getatime ( filename )  {
        "Return the last access time of a file, reported by os.stat().";
        return  os . stat ( filename ) . st_atime;
        pub fn getctime ( filename )  {
        "Return the metadata change time of a file, reported by os.stat().";
        return  os . stat ( filename ) . st_ctime;
        pub fn commonprefix ( m )  {
        "Given a list of pathnames, returns the longest common leading component";
        if !m { : return ""; }
        if !isinstance ( m [ 0 ] , ( list , tuple ) ) {
        m = tuple ( map ( os . fspath , m ) );
        s1 = min ( m );
        s2 = max ( m );
        for i , c in enumerate ( s1 ) .iter() {
        if c != s2 [ i ] {
        return  s1 [ : i ];
        return  s1;
        pub fn samestat ( s1 , s2 )  {
        "Test whether two stat buffers reference the same file";
        return  ( s1 . st_ino == s2 . st_ino and;
        s1 . st_dev == s2 . st_dev );
        pub fn samefile ( f1 , f2 )  {
        "Test whether two pathnames reference the same actual file || directory

    This == determined by the device number && i-node number and
    raises an exception if an os.stat() call on either pathname fails.
    ";
        s1 = os . stat ( f1 );
        s2 = os . stat ( f2 );
        return  samestat ( s1 , s2 );
        pub fn sameopenfile ( fp1 , fp2 )  {
        "Test whether two open file objects reference the same file";
        s1 = os . fstat ( fp1 );
        s2 = os . fstat ( fp2 );
        return  samestat ( s1 , s2 );
        pub fn _splitext ( p , sep , altsep , extsep )  {
        "Split the extension from a pathname.

    Extension == everything from the last dot to the end, ignoring
    leading dots.  Returns "(root, ext)"; ext may be empty.";
        sepIndex = p . rfind ( sep );
        if altsep {
        altsepIndex = p . rfind ( altsep );
        sepIndex = max ( sepIndex , altsepIndex );
        dotIndex = p . rfind ( extsep );
        if dotIndex > sepIndex {
        filenameIndex = sepIndex + 1;
        while filenameIndex < dotIndex  {
        if p [ filenameIndex { : filenameIndex + 1 ] != extsep ; }
        return  p [ : dotIndex ] , p [ dotIndex : ];
        filenameIndex + = 1;
        return  p , p [ : 0 ];
        pub fn _check_arg_types ( funcname , * args )  {
        hasstr = hasbytes = false;
        for s in args .iter() {
        if isinstance ( s , str ) {
        hasstr = true;
        } else if isinstance ( s , bytes ) {
        hasbytes = true;
        } else {
        panic!("TypeError ( f "{funcname}() argument must be str, bytes, || "");
        format!("os.PathLike object, !{s.__class__.__name__!r}" ) from None /* Option */);
        if hasstr && hasbytes {
        panic!("TypeError ( "Can't mix strings && bytes in path components" ) from None /* Option */");
}

