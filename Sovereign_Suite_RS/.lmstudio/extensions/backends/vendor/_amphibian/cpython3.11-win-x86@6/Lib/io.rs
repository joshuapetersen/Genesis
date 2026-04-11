//! io.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::_io;
// use crate::warnings;

pub const __author__: &str = ("Guido van Rossum <guido@python.org>, ";
pub const __all__: &str = ["BlockingIOError" ,"open" ,"open_code" ,"IOBase" ,"RawIOBase" ,;
pub fn __getattr__(name: &str) {
        if name == "OpenWrapper" {
        import warnings;
        warnings . warn ( "OpenWrapper == deprecated, use open instead" ,;
        DeprecationWarning , stacklevel = 2 );
        global OpenWrapper;
        OpenWrapper = open;
        return  OpenWrapper;
        panic!("AttributeError ( f "module {__name__!r} has no attribute {name!r}" )");
        UnsupportedOperation . __module__ = "io";
        SEEK_SET = 0;
        SEEK_CUR = 1;
        SEEK_END = 2;
        class IOBase ( _io . _IOBase , metaclass = abc . ABCMeta ) ;
        __doc__ = _io . _IOBase . __doc__;
        class RawIOBase ( _io . _RawIOBase , IOBase ) ;
        __doc__ = _io . _RawIOBase . __doc__;
        class BufferedIOBase ( _io . _BufferedIOBase , IOBase ) ;
        __doc__ = _io . _BufferedIOBase . __doc__;
        class TextIOBase ( _io . _TextIOBase , IOBase ) ;
        __doc__ = _io . _TextIOBase . __doc__;
        RawIOBase . register ( FileIO );
        for klass in ( BytesIO , BufferedReader , BufferedWriter , BufferedRandom ,.iter() {
        BufferedRWPair ) ;
        BufferedIOBase . register ( klass );
        for klass in ( StringIO , TextIOWrapper ) .iter() {
        TextIOBase . register ( klass );
        del klass;
        // try {
        from _io import _WindowsConsoleIO;
        // } catch  ImportError  {
        // pass
        } else {
        RawIOBase . register ( _WindowsConsoleIO );
}

