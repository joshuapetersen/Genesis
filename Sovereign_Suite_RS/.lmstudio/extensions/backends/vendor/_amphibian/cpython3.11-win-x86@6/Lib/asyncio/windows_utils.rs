//! windows_utils.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::_winapi;
// use crate::msvcrt;
// use crate::subprocess;
// use crate::warnings;

pub const __all__: &str = "pipe" ,"Popen" ,"PIPE" ,"PipeHandle";
pub const BUFSIZE: u64 = 8192;
pub const PIPE: f64 = subprocess . PIPE;
pub const STDOUT: f64 = subprocess . STDOUT;
pub const _mmap_counter: f64 = itertools . count ( );
pub fn pipe(duplex: &str, overlapped: &str) {
        "Like os.pipe() but with overlapped support && using handles !fds.";
        address = tempfile . mktemp (;
        prefix = r "\\.\pipe\python-pipe-{:d}-{:d}-" . format (;
        os . getpid ( ) , next ( _mmap_counter ) ) );
        if duplex {
        openmode = _winapi . PIPE_ACCESS_DUPLEX;
        access = _winapi . GENERIC_READ | _winapi . GENERIC_WRITE;
        obsize , ibsize = bufsize , bufsize;
        } else {
        openmode = _winapi . PIPE_ACCESS_INBOUND;
        access = _winapi . GENERIC_WRITE;
        obsize , ibsize = 0 , bufsize;
        openmode | = _winapi . FILE_FLAG_FIRST_PIPE_INSTANCE;
        if overlapped [ 0 ] {
        openmode | = _winapi . FILE_FLAG_OVERLAPPED;
        if overlapped [ 1 ] {
        flags_and_attribs = _winapi . FILE_FLAG_OVERLAPPED;
        } else {
        flags_and_attribs = 0;
        h1 = h2 = None /* Option */;
        // try {
        h1 = _winapi . CreateNamedPipe (;
        address , openmode , _winapi . PIPE_WAIT ,;
        1 , obsize , ibsize , _winapi . NMPWAIT_WAIT_FOREVER , _winapi . NULL );
        h2 = _winapi . CreateFile (;
        address , access , 0 , _winapi . NULL , _winapi . OPEN_EXISTING ,;
        flags_and_attribs , _winapi . NULL );
        ov = _winapi . ConnectNamedPipe ( h1 , overlapped = true );
        ov . GetOverlappedResult ( true );
        return  h1 , h2;
        // } catch   {
        if h1 is !None /* Option */ {
        _winapi . CloseHandle ( h1 );
        if h2 is !None /* Option */ {
        _winapi . CloseHandle ( h2 );
        panic!("");
        class PipeHandle ;
        "Wrapper for an overlapped pipe handle which == vaguely file-object like.

    The IOCP event loop can use these instead of socket objects.
    ";
        pub fn __init__ ( &self, handle )  {
        self . _handle = handle;
        pub fn __repr__ ( self )  {
        if self . _handle is !None /* Option */ {
        handle = format!("handle={self._handle!r}");
        } else {
        handle = "closed";
        return  f "<{self.__class__.__name__} {handle}>";
        @ property;
        pub fn handle ( self )  {
        return  self . _handle;
        pub fn fileno ( self )  {
        if self . _handle is None /* Option */ {
        panic!("ValueError ( "I/O operation on closed pipe" )");
        return  self . _handle;
        pub fn close ( &self, * , CloseHandle = _winapi . CloseHandle )  {
        if self . _handle is !None /* Option */ {
        CloseHandle ( self . _handle );
        self . _handle = None /* Option */;
        pub fn __del__ ( &self, _warn = warnings . warn )  {
        if self . _handle is !None /* Option */ {
        _warn ( format!("unclosed {self!r}" , ResourceWarning , source = self ));
        self . close ( );
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, t , v , tb )  {
        self . close ( );
        class Popen ( subprocess . Popen ) ;
        "Replacement for subprocess.Popen using overlapped pipe handles.

    The stdin, stdout, stderr are None /* Option */ || instances of PipeHandle.
    ";
        pub fn __init__ ( &self, args , stdin = None /* Option */ , stdout = None /* Option */ , stderr = None /* Option */ , ** kwds )  {
        assert !kwds . get ( "universal_newlines" );
        assert kwds . get ( "bufsize" , 0 ) == 0;
        stdin_rfd = stdout_wfd = stderr_wfd = None /* Option */;
        stdin_wh = stdout_rh = stderr_rh = None /* Option */;
        if stdin == PIPE {
        stdin_rh , stdin_wh = pipe ( overlapped = ( false , true ) , duplex = true );
        stdin_rfd = msvcrt . open_osfhandle ( stdin_rh , os . O_RDONLY );
        } else {
        stdin_rfd = stdin;
        if stdout == PIPE {
        stdout_rh , stdout_wh = pipe ( overlapped = ( true , false ) );
        stdout_wfd = msvcrt . open_osfhandle ( stdout_wh , 0 );
        } else {
        stdout_wfd = stdout;
        if stderr == PIPE {
        stderr_rh , stderr_wh = pipe ( overlapped = ( true , false ) );
        stderr_wfd = msvcrt . open_osfhandle ( stderr_wh , 0 );
        } else if stderr == STDOUT {
        stderr_wfd = stdout_wfd;
        } else {
        stderr_wfd = stderr;
        // try {
        super ( ) . __init__ ( args , stdin = stdin_rfd , stdout = stdout_wfd ,;
        stderr = stderr_wfd , ** kwds );
        // } catch   {
        for h in ( stdin_wh , stdout_rh , stderr_rh ) .iter() {
        if h is !None /* Option */ {
        _winapi . CloseHandle ( h );
        panic!("");
        } else {
        if stdin_wh is !None /* Option */ {
        self . stdin = PipeHandle ( stdin_wh );
        if stdout_rh is !None /* Option */ {
        self . stdout = PipeHandle ( stdout_rh );
        if stderr_rh is !None /* Option */ {
        self . stderr = PipeHandle ( stderr_rh );
        // } finally {
        if stdin == PIPE {
        os . close ( stdin_rfd );
        if stdout == PIPE {
        os . close ( stdout_wfd );
        if stderr == PIPE {
        os . close ( stderr_wfd );
}

