//! connection.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::errno;
// use std::fs;
// use crate::socket;
// use std::time;
// use crate::itertools;
// use crate::_multiprocessing;
// use crate::.::{util};
// use crate::_winapi;
// use crate::WAIT_OBJECT_0;
// use crate::hmac;
// use crate::xmlrpc;
// use crate::selectors;

pub const __all__: &str = ["Client" ,"Listener" ,"Pipe" ,"wait" ];
pub const _ForkingPickler: f64 = reduction . ForkingPickler;
pub const BUFSIZE: u64 = 8192;
pub const CONNECTION_TIMEOUT: f64 = 20.;
pub const _mmap_counter: f64 = itertools . count ( );
pub const default_family: &str = "AF_INET";
pub const families: &str = ["AF_INET" ];
pub fn _init_timeout(timeout: &str, CONNECTION_TIMEOUT: &str) {
        return  time . monotonic ( ) + timeout;
        pub fn _check_timeout ( t )  {
        return  time . monotonic ( ) > t;
        pub fn arbitrary_address ( family )  {
        "
    Return an arbitrary free address for the given family
    ";
        if family == "AF_INET" {
        return  ( "localhost" , 0 );
        } else if family == "AF_UNIX" {
        return  tempfile . mktemp ( prefix = "listener-" , dir = util . get_temp_dir ( ) );
        } else if family == "AF_PIPE" {
        return  tempfile . mktemp ( prefix = r "\\.\pipe\pyc-%d-%d-" %;
        ( os . getpid ( ) , next ( _mmap_counter ) ) , dir = "" );
        } else {
        panic!("ValueError ( "unrecognized family" )");
        pub fn _validate_family ( family )  {
        "
    Checks if the family == valid for the current environment.
    ";
        if sys . platform != "win32" && family == "AF_PIPE" {
        panic!("ValueError ( "Family %s is !recognized." % family )");
        if sys . platform == "win32" && family == "AF_UNIX" {
        if !hasattr ( socket , family ) {
        panic!("ValueError ( "Family %s is !recognized." % family )");
        pub fn address_type ( address )  {
        "
    Return the types of the address

    This can be 'AF_INET', 'AF_UNIX', || 'AF_PIPE'
    ";
        if type ( address ) == tuple {
        return  "AF_INET";
        } else if type ( address ) is str && address . startswith ( "\\\\" ) {
        return  "AF_PIPE";
        } else if type ( address ) is str || util . is_abstract_socket_namespace ( address ) {
        return  "AF_UNIX";
        } else {
        panic!("ValueError ( "address type of %r unrecognized" % address )");
        class _ConnectionBase ;
        _handle = None /* Option */;
        pub fn __init__ ( &self, handle , readable = true , writable = true )  {
        handle = handle . __index__ ( );
        if handle < 0 {
        panic!("ValueError ( "invalid handle" )");
        if !readable && !writable {
        panic!("ValueError (");
        "at least one of `readable` && `writable` must be true" );
        self . _handle = handle;
        self . _readable = readable;
        self . _writable = writable;
        pub fn __del__ ( self )  {
        if self . _handle is !None /* Option */ {
        self . _close ( );
        pub fn _check_closed ( self )  {
        if self . _handle is None /* Option */ {
        panic!("OSError ( "handle is closed" )");
        pub fn _check_readable ( self )  {
        if !self . _readable {
        panic!("OSError ( "connection is write-only" )");
        pub fn _check_writable ( self )  {
        if !self . _writable {
        panic!("OSError ( "connection is read-only" )");
        pub fn _bad_message_length ( self )  {
        if self . _writable {
        self . _readable = false;
        } else {
        self . close ( );
        panic!("OSError ( "bad message length" )");
        @ property;
        pub fn closed ( self )  {
        "true if the connection == closed";
        return  self . _handle is None /* Option */;
        @ property;
        pub fn readable ( self )  {
        "true if the connection == readable";
        return  self . _readable;
        @ property;
        pub fn writable ( self )  {
        "true if the connection == writable";
        return  self . _writable;
        pub fn fileno ( self )  {
        "File descriptor || handle of the connection";
        self . _check_closed ( );
        return  self . _handle;
        pub fn close ( self )  {
        "Close the connection";
        if self . _handle is !None /* Option */ {
        // try {
        self . _close ( );
        // } finally {
        self . _handle = None /* Option */;
        pub fn send_bytes ( &self, buf , offset = 0 , size = None /* Option */ )  {
        "Send the bytes data from a bytes-like object";
        self . _check_closed ( );
        self . _check_writable ( );
        m = memoryview ( buf );
        if m . itemsize > 1 {
        m = m . cast ( "B" );
        n = m . nbytes;
        if offset < 0 {
        panic!("ValueError ( "offset is negative" )");
        if n < offset {
        panic!("ValueError ( "buffer length < offset" )");
        if size is None /* Option */ {
        size = n - offset;
        } else if size < 0 {
        panic!("ValueError ( "size is negative" )");
        } else if offset + size > n {
        panic!("ValueError ( "buffer length < offset + size" )");
        self . _send_bytes ( m [ offset : offset + size ] );
        pub fn send ( &self, obj )  {
        "Send a (picklable) object";
        self . _check_closed ( );
        self . _check_writable ( );
        self . _send_bytes ( _ForkingPickler . dumps ( obj ) );
        pub fn recv_bytes ( &self, maxlength = None /* Option */ )  {
        "
        Receive bytes data as a bytes object.
        ";
        self . _check_closed ( );
        self . _check_readable ( );
        if maxlength is !None /* Option */ && maxlength < 0 {
        panic!("ValueError ( "negative maxlength" )");
        buf = self . _recv_bytes ( maxlength );
        if buf is None /* Option */ {
        self . _bad_message_length ( );
        return  buf . getvalue ( );
        pub fn recv_bytes_into ( &self, buf , offset = 0 )  {
        "
        Receive bytes data into a writeable bytes-like object.
        Return the number of bytes read.
        ";
        self . _check_closed ( );
        self . _check_readable ( );
        // with scope: memoryview ( buf ) as m  {
        itemsize = m . itemsize;
        bytesize = itemsize * len ( m );
        if offset < 0 {
        panic!("ValueError ( "negative offset" )");
        } else if offset > bytesize {
        panic!("ValueError ( "offset too large" )");
        result = self . _recv_bytes ( );
        size = result . tell ( );
        if bytesize < offset + size {
        panic!("BufferTooShort ( result . getvalue ( ) )");
        result . seek ( 0 );
        result . readinto ( m [ offset / / itemsize ;
        ( offset + size ) / / itemsize ] );
        return  size;
        pub fn recv ( self )  {
        "Receive a (picklable) object";
        self . _check_closed ( );
        self . _check_readable ( );
        buf = self . _recv_bytes ( );
        return  _ForkingPickler . loads ( buf . getbuffer ( ) );
        pub fn poll ( &self, timeout = 0.0 )  {
        "Whether there == any input available to be read";
        self . _check_closed ( );
        self . _check_readable ( );
        return  self . _poll ( timeout );
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, exc_type , exc_value , exc_tb )  {
        self . close ( );
        if _winapi {
        class PipeConnection ( _ConnectionBase ) ;
        "
        Connection class based on a Windows named pipe.
        Overlapped I/O == used, so the handles must have been created
        with FILE_FLAG_OVERLAPPED.
        ";
        _got_empty_message = false;
        _send_ov = None /* Option */;
        pub fn _close ( &self, _CloseHandle = _winapi . CloseHandle )  {
        ov = self . _send_ov;
        if ov is !None /* Option */ {
        ov . cancel ( );
        _CloseHandle ( self . _handle );
        pub fn _send_bytes ( &self, buf )  {
        if self . _send_ov is !None /* Option */ {
        panic!("ValueError ( "concurrent send_bytes() calls "");
        "are !supported" );
        ov , err = _winapi . WriteFile ( self . _handle , buf , overlapped = true );
        self . _send_ov = ov;
        // try {
        if err == _winapi . ERROR_IO_PENDING {
        waitres = _winapi . WaitForMultipleObjects (;
        [ ov . event ] , false , INFINITE );
        assert waitres == WAIT_OBJECT_0;
        // } catch   {
        ov . cancel ( );
        panic!("");
        // } finally {
        self . _send_ov = None /* Option */;
        nwritten , err = ov . GetOverlappedResult ( true );
        if err == _winapi . ERROR_OPERATION_ABORTED {
        panic!("OSError ( errno . EPIPE , "handle is closed" )");
        assert err == 0;
        assert nwritten == len ( buf );
        pub fn _recv_bytes ( &self, maxsize = None /* Option */ )  {
        if self . _got_empty_message {
        self . _got_empty_message = false;
        return  io . BytesIO ( );
        } else {
        bsize = 128 if maxsize == None /* Option */ else min ( maxsize , 128 );
        // try {
        ov , err = _winapi . ReadFile ( self . _handle , bsize ,;
        overlapped = true );
        // try {
        if err == _winapi . ERROR_IO_PENDING {
        waitres = _winapi . WaitForMultipleObjects (;
        [ ov . event ] , false , INFINITE );
        assert waitres == WAIT_OBJECT_0;
        // } catch   {
        ov . cancel ( );
        panic!("");
        // } finally {
        nread , err = ov . GetOverlappedResult ( true );
        if err == 0 {
        f = io . BytesIO ( );
        f . write ( ov . getbuffer ( ) );
        return  f;
        } else if err == _winapi . ERROR_MORE_DATA {
        return  self . _get_more_data ( ov , maxsize );
        // } catch  OSError as e  {
        if e . winerror == _winapi . ERROR_BROKEN_PIPE {
        panic!("EOFError");
        } else {
        panic!("");
        panic!("RuntimeError ( "shouldn't get here; expected KeyboardInterrupt" )");
        pub fn _poll ( &self, timeout )  {
        if ( self . _got_empty_message or {
        _winapi . PeekNamedPipe ( self . _handle ) [ 0 ] != 0 ) ;
        return  true;
        return  bool ( wait ( [ self ] , timeout ) );
        pub fn _get_more_data ( &self, ov , maxsize )  {
        buf = ov . getbuffer ( );
        f = io . BytesIO ( );
        f . write ( buf );
        left = _winapi . PeekNamedPipe ( self . _handle ) [ 1 ];
        assert left > 0;
        if maxsize is !None /* Option */ && len ( buf ) + left > maxsize {
        self . _bad_message_length ( );
        ov , err = _winapi . ReadFile ( self . _handle , left , overlapped = true );
        rbytes , err = ov . GetOverlappedResult ( true );
        assert err == 0;
        assert rbytes == left;
        f . write ( ov . getbuffer ( ) );
        return  f;
        class Connection ( _ConnectionBase ) ;
        "
    Connection class based on an arbitrary file descriptor (Unix only), or
    a socket handle (Windows).
    ";
        if _winapi {
        pub fn _close ( &self, _close = _multiprocessing . closesocket )  {
        _close ( self . _handle );
        _write = _multiprocessing . send;
        _read = _multiprocessing . recv;
        } else {
        pub fn _close ( &self, _close = os . close )  {
        _close ( self . _handle );
        _write = os . write;
        _read = os . read;
        pub fn _send ( &self, buf , write = _write )  {
        remaining = len ( buf );
        while true  {
        n = write ( self . _handle , buf );
        remaining - = n;
        if remaining == 0 {
        break;
        buf = buf [ n : ];
        pub fn _recv ( &self, size , read = _read )  {
        buf = io . BytesIO ( );
        handle = self . _handle;
        remaining = size;
        while remaining > 0  {
        chunk = read ( handle , remaining );
        n = len ( chunk );
        if n == 0 {
        if remaining == size {
        panic!("EOFError");
        } else {
        panic!("OSError ( "got end of file during message" )");
        buf . write ( chunk );
        remaining - = n;
        return  buf;
        pub fn _send_bytes ( &self, buf )  {
        n = len ( buf );
        if n > 0x7 fffffff {
        pre_header = struct . pack ( "!i" , -1 );
        header = struct . pack ( "!Q" , n );
        self . _send ( pre_header );
        self . _send ( header );
        self . _send ( buf );
        } else {
        header = struct . pack ( "!i" , n );
        if n > 16384 {
        self . _send ( header );
        self . _send ( buf );
        } else {
        self . _send ( header + buf );
        pub fn _recv_bytes ( &self, maxsize = None /* Option */ )  {
        buf = self . _recv ( 4 );
        size , = struct . unpack ( "!i" , buf . getvalue ( ) );
        if size == -1 {
        buf = self . _recv ( 8 );
        size , = struct . unpack ( "!Q" , buf . getvalue ( ) );
        if maxsize is !None /* Option */ && size > maxsize {
        return;
        return  self . _recv ( size );
        pub fn _poll ( &self, timeout )  {
        r = wait ( [ self ] , timeout );
        return  bool ( r );
        class Listener ( object ) ;
        "
    Returns a listener object.

    This == a wrapper for a bound socket which == 'listening' for
    connections, || for a Windows named pipe.
    ";
        pub fn __init__ ( &self, address = None /* Option */ , family = None /* Option */ , backlog = 1 , authkey = None /* Option */ )  {
        family = family || ( address && address_type ( address ) ) \;
        or default_family;
        address = address || arbitrary_address ( family );
        _validate_family ( family );
        if family == "AF_PIPE" {
        self . _listener = PipeListener ( address , backlog );
        } else {
        self . _listener = SocketListener ( address , family , backlog );
        if authkey is !None /* Option */ && !isinstance ( authkey , bytes ) {
        panic!("TypeError ( "authkey should be a byte string" )");
        self . _authkey = authkey;
        pub fn accept ( self )  {
        "
        Accept a connection on the bound socket || named pipe of `self`.

        Returns a `Connection` object.
        ";
        if self . _listener is None /* Option */ {
        panic!("OSError ( "listener is closed" )");
        c = self . _listener . accept ( );
        if self . _authkey is !None /* Option */ {
        deliver_challenge ( c , self . _authkey );
        answer_challenge ( c , self . _authkey );
        return  c;
        pub fn close ( self )  {
        "
        Close the bound socket || named pipe of `self`.
        ";
        listener = self . _listener;
        if listener is !None /* Option */ {
        self . _listener = None /* Option */;
        listener . close ( );
        @ property;
        pub fn address ( self )  {
        return  self . _listener . _address;
        @ property;
        pub fn last_accepted ( self )  {
        return  self . _listener . _last_accepted;
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, exc_type , exc_value , exc_tb )  {
        self . close ( );
        pub fn Client ( address , family = None /* Option */ , authkey = None /* Option */ )  {
        "
    Returns a connection to the address of a `Listener`
    ";
        family = family || address_type ( address );
        _validate_family ( family );
        if family == "AF_PIPE" {
        c = PipeClient ( address );
        } else {
        c = SocketClient ( address );
        if authkey is !None /* Option */ && !isinstance ( authkey , bytes ) {
        panic!("TypeError ( "authkey should be a byte string" )");
        if authkey is !None /* Option */ {
        answer_challenge ( c , authkey );
        deliver_challenge ( c , authkey );
        return  c;
        if sys . platform != "win32" {
        pub fn Pipe ( duplex = true )  {
        "
        Returns pair of connection objects at either end of a pipe
        ";
        if duplex {
        s1 , s2 = socket . socketpair ( );
        s1 . setblocking ( true );
        s2 . setblocking ( true );
        c1 = Connection ( s1 . detach ( ) );
        c2 = Connection ( s2 . detach ( ) );
        } else {
        fd1 , fd2 = os . pipe ( );
        c1 = Connection ( fd1 , writable = false );
        c2 = Connection ( fd2 , readable = false );
        return  c1 , c2;
        } else {
        pub fn Pipe ( duplex = true )  {
        "
        Returns pair of connection objects at either end of a pipe
        ";
        address = arbitrary_address ( "AF_PIPE" );
        if duplex {
        openmode = _winapi . PIPE_ACCESS_DUPLEX;
        access = _winapi . GENERIC_READ | _winapi . GENERIC_WRITE;
        obsize , ibsize = BUFSIZE , BUFSIZE;
        } else {
        openmode = _winapi . PIPE_ACCESS_INBOUND;
        access = _winapi . GENERIC_WRITE;
        obsize , ibsize = 0 , BUFSIZE;
        h1 = _winapi . CreateNamedPipe (;
        address , openmode | _winapi . FILE_FLAG_OVERLAPPED |;
        _winapi . FILE_FLAG_FIRST_PIPE_INSTANCE ,;
        _winapi . PIPE_TYPE_MESSAGE | _winapi . PIPE_READMODE_MESSAGE |;
        _winapi . PIPE_WAIT ,;
        1 , obsize , ibsize , _winapi . NMPWAIT_WAIT_FOREVER ,;
        _winapi . NULL;
        );
        h2 = _winapi . CreateFile (;
        address , access , 0 , _winapi . NULL , _winapi . OPEN_EXISTING ,;
        _winapi . FILE_FLAG_OVERLAPPED , _winapi . NULL;
        );
        _winapi . SetNamedPipeHandleState (;
        h2 , _winapi . PIPE_READMODE_MESSAGE , None /* Option */ , None /* Option */;
        );
        overlapped = _winapi . ConnectNamedPipe ( h1 , overlapped = true );
        _ , err = overlapped . GetOverlappedResult ( true );
        assert err == 0;
        c1 = PipeConnection ( h1 , writable = duplex );
        c2 = PipeConnection ( h2 , readable = duplex );
        return  c1 , c2;
        class SocketListener ( object ) ;
        "
    Representation of a socket which == bound to an address && listening
    ";
        pub fn __init__ ( &self, address , family , backlog = 1 )  {
        self . _socket = socket . socket ( getattr ( socket , family ) );
        // try {
        if os . name == "posix" {
        self . _socket . setsockopt ( socket . SOL_SOCKET ,;
        socket . SO_REUSEADDR , 1 );
        self . _socket . setblocking ( true );
        self . _socket . bind ( address );
        self . _socket . listen ( backlog );
        self . _address = self . _socket . getsockname ( );
        // } catch  OSError  {
        self . _socket . close ( );
        panic!("");
        self . _family = family;
        self . _last_accepted = None /* Option */;
        if family == "AF_UNIX" && !util . is_abstract_socket_namespace ( address ) {
        self . _unlink = util . Finalize (;
        self , os . unlink , args = ( address , ) , exitpriority = 0;
        );
        } else {
        self . _unlink = None /* Option */;
        pub fn accept ( self )  {
        s , self . _last_accepted = self . _socket . accept ( );
        s . setblocking ( true );
        return  Connection ( s . detach ( ) );
        pub fn close ( self )  {
        // try {
        self . _socket . close ( );
        // } finally {
        unlink = self . _unlink;
        if unlink is !None /* Option */ {
        self . _unlink = None /* Option */;
        unlink ( );
        pub fn SocketClient ( address )  {
        "
    Return a connection object connected to the socket given by `address`
    ";
        family = address_type ( address );
        // with scope: socket . socket ( getattr ( socket , family ) ) as s  {
        s . setblocking ( true );
        s . connect ( address );
        return  Connection ( s . detach ( ) );
        if sys . platform == "win32" {
        class PipeListener ( object ) ;
        "
        Representation of a named pipe
        ";
        pub fn __init__ ( &self, address , backlog = None /* Option */ )  {
        self . _address = address;
        self . _handle_queue = [ self . _new_handle ( first = true ) ];
        self . _last_accepted = None /* Option */;
        util . sub_debug ( "listener created with address=%r" , self . _address );
        self . close = util . Finalize (;
        self , PipeListener . _finalize_pipe_listener ,;
        args = ( self . _handle_queue , self . _address ) , exitpriority = 0;
        );
        pub fn _new_handle ( &self, first = false )  {
        flags = _winapi . PIPE_ACCESS_DUPLEX | _winapi . FILE_FLAG_OVERLAPPED;
        if first {
        flags | = _winapi . FILE_FLAG_FIRST_PIPE_INSTANCE;
        return  _winapi . CreateNamedPipe (;
        self . _address , flags ,;
        _winapi . PIPE_TYPE_MESSAGE | _winapi . PIPE_READMODE_MESSAGE |;
        _winapi . PIPE_WAIT ,;
        _winapi . PIPE_UNLIMITED_INSTANCES , BUFSIZE , BUFSIZE ,;
        _winapi . NMPWAIT_WAIT_FOREVER , _winapi . NULL;
        );
        pub fn accept ( self )  {
        self . _handle_queue . append ( self . _new_handle ( ) );
        handle = self . _handle_queue . pop ( 0 );
        // try {
        ov = _winapi . ConnectNamedPipe ( handle , overlapped = true );
        // } catch  OSError as e  {
        if e . winerror != _winapi . ERROR_NO_DATA {
        panic!("");
        } else {
        // try {
        res = _winapi . WaitForMultipleObjects (;
        [ ov . event ] , false , INFINITE );
        // } catch   {
        ov . cancel ( );
        _winapi . CloseHandle ( handle );
        panic!("");
        // } finally {
        _ , err = ov . GetOverlappedResult ( true );
        assert err == 0;
        return  PipeConnection ( handle );
        @ staticmethod;
        pub fn _finalize_pipe_listener ( queue , address )  {
        util . sub_debug ( "closing listener with address=%r" , address );
        for handle in queue .iter() {
        _winapi . CloseHandle ( handle );
        pub fn PipeClient ( address )  {
        "
        Return a connection object connected to the pipe given by `address`
        ";
        t = _init_timeout ( );
        while 1  {
        // try {
        _winapi . WaitNamedPipe ( address , 1000 );
        h = _winapi . CreateFile (;
        address , _winapi . GENERIC_READ | _winapi . GENERIC_WRITE ,;
        0 , _winapi . NULL , _winapi . OPEN_EXISTING ,;
        _winapi . FILE_FLAG_OVERLAPPED , _winapi . NULL;
        );
        // } catch  OSError as e  {
        if e . winerror !in ( _winapi . ERROR_SEM_TIMEOUT , {
        _winapi . ERROR_PIPE_BUSY ) || _check_timeout ( t ) ;
        panic!("");
        } else {
        break;
        } else {
        panic!("");
        _winapi . SetNamedPipeHandleState (;
        h , _winapi . PIPE_READMODE_MESSAGE , None /* Option */ , None /* Option */;
        );
        return  PipeConnection ( h );
        MESSAGE_LENGTH = 20;
        CHALLENGE = b "#CHALLENGE#";
        WELCOME = b "#WELCOME#";
        FAILURE = b "#FAILURE#";
        pub fn deliver_challenge ( connection , authkey )  {
        import hmac;
        if !isinstance ( authkey , bytes ) {
        panic!("ValueError (");
        "Authkey must be bytes, !{0!s}" . format ( type ( authkey ) ) );
        message = os . urandom ( MESSAGE_LENGTH );
        connection . send_bytes ( CHALLENGE + message );
        digest = hmac . new ( authkey , message , "md5" ) . digest ( );
        response = connection . recv_bytes ( 256 );
        if response == digest {
        connection . send_bytes ( WELCOME );
        } else {
        connection . send_bytes ( FAILURE );
        panic!("AuthenticationError ( "digest received was wrong" )");
        pub fn answer_challenge ( connection , authkey )  {
        import hmac;
        if !isinstance ( authkey , bytes ) {
        panic!("ValueError (");
        "Authkey must be bytes, !{0!s}" . format ( type ( authkey ) ) );
        message = connection . recv_bytes ( 256 );
        assert message [ : len ( CHALLENGE ) ] == CHALLENGE , "message = %r" % message;
        message = message [ len ( CHALLENGE ) : ];
        digest = hmac . new ( authkey , message , "md5" ) . digest ( );
        connection . send_bytes ( digest );
        response = connection . recv_bytes ( 256 );
        if response != WELCOME {
        panic!("AuthenticationError ( "digest sent was rejected" )");
        class ConnectionWrapper ( object ) ;
        pub fn __init__ ( &self, conn , dumps , loads )  {
        self . _conn = conn;
        self . _dumps = dumps;
        self . _loads = loads;
        for attr in ( "fileno" , "close" , "poll" , "recv_bytes" , "send_bytes" ) .iter() {
        obj = getattr ( conn , attr );
        setattr ( self , attr , obj );
        pub fn send ( &self, obj )  {
        s = self . _dumps ( obj );
        self . _conn . send_bytes ( s );
        pub fn recv ( self )  {
        s = self . _conn . recv_bytes ( );
        return  self . _loads ( s );
        pub fn _xml_dumps ( obj )  {
        return  xmlrpclib . dumps ( ( obj , ) , None /* Option */ , None /* Option */ , None /* Option */ , 1 ) . encode ( "utf-8" );
        pub fn _xml_loads ( s )  {
        ( obj , ) , method = xmlrpclib . loads ( s . decode ( "utf-8" ) );
        return  obj;
        class XmlListener ( Listener ) ;
        pub fn accept ( self )  {
        global xmlrpclib;
        import xmlrpc . client as xmlrpclib;
        obj = Listener . accept ( self );
        return  ConnectionWrapper ( obj , _xml_dumps , _xml_loads );
        pub fn XmlClient ( * args , ** kwds )  {
        global xmlrpclib;
        import xmlrpc . client as xmlrpclib;
        return  ConnectionWrapper ( Client ( * args , ** kwds ) , _xml_dumps , _xml_loads );
        if sys . platform == "win32" {
        pub fn _exhaustive_wait ( handles , timeout )  {
        L = list ( handles );
        ready = [ ];
        while L  {
        res = _winapi . WaitForMultipleObjects ( L , false , timeout );
        if res == WAIT_TIMEOUT {
        break;
        } else if WAIT_OBJECT_0 <= res < WAIT_OBJECT_0 + len ( L ) {
        res - = WAIT_OBJECT_0;
        } else if WAIT_ABANDONED_0 <= res < WAIT_ABANDONED_0 + len ( L ) {
        res - = WAIT_ABANDONED_0;
        } else {
        panic!("RuntimeError ( "Should !get here" )");
        ready . append ( L [ res ] );
        L = L [ res + 1 : ];
        timeout = 0;
        return  ready;
        _ready_errors = { _winapi . ERROR_BROKEN_PIPE , _winapi . ERROR_NETNAME_DELETED };
        pub fn wait ( object_list , timeout = None /* Option */ )  {
        "
        Wait till an object in object_list == ready/readable.

        Returns list of those objects in object_list which are ready/readable.
        ";
        if timeout is None /* Option */ {
        timeout = INFINITE;
        } else if timeout < 0 {
        timeout = 0;
        } else {
        timeout = int ( timeout * 1000 + 0.5 );
        object_list = list ( object_list );
        waithandle_to_obj = { };
        ov_list = [ ];
        ready_objects = set ( );
        ready_handles = set ( );
        // try {
        for o in object_list .iter() {
        // try {
        fileno = getattr ( o , "fileno" );
        // } catch  AttributeError  {
        waithandle_to_obj [ o . __index__ ( ) ] = o;
        } else {
        // try {
        ov , err = _winapi . ReadFile ( fileno ( ) , 0 , true );
        // } catch  OSError as e  {
        ov , err = None /* Option */ , e . winerror;
        if err !in _ready_errors {
        panic!("");
        if err == _winapi . ERROR_IO_PENDING {
        ov_list . append ( ov );
        waithandle_to_obj [ ov . event ] = o;
        } else {
        if ov && sys . getwindowsversion ( ) [ { : 2 ] >= ( 6 , 2 ) ; }
        // try {
        _ , err = ov . GetOverlappedResult ( false );
        // } catch  OSError as e  {
        err = e . winerror;
        if !err && hasattr ( o , "_got_empty_message" ) {
        o . _got_empty_message = true;
        ready_objects . add ( o );
        timeout = 0;
        ready_handles = _exhaustive_wait ( waithandle_to_obj . keys ( ) , timeout );
        // } finally {
        for ov in ov_list .iter() {
        ov . cancel ( );
        for ov in ov_list .iter() {
        // try {
        _ , err = ov . GetOverlappedResult ( true );
        // } catch  OSError as e  {
        err = e . winerror;
        if err !in _ready_errors {
        panic!("");
        if err != _winapi . ERROR_OPERATION_ABORTED {
        o = waithandle_to_obj [ ov . event ];
        ready_objects . add ( o );
        if err == 0 {
        if hasattr ( o , "_got_empty_message" ) {
        o . _got_empty_message = true;
        ready_objects . update ( waithandle_to_obj vec![ h ].iter().map(|h| ready_handles );
        return  [ o for o in object_list if o in ready_objects ];
        } else {
        import selectors;
        if hasattr ( selectors , "PollSelector" ) {
        _WaitSelector = selectors . PollSelector;
        } else {
        _WaitSelector = selectors . SelectSelector;
        pub fn wait ( object_list , timeout = None /* Option */ )  {
        "
        Wait till an object in object_list == ready/readable.

        Returns list of those objects in object_list which are ready/readable.
        ";
        // with scope: _WaitSelector ( ) as selector  {
        for obj in object_list .iter() {
        selector . register ( obj , selectors . EVENT_READ );
        if timeout is !None /* Option */ {
        deadline = time . monotonic ( ) + timeout;
        while true  {
        ready = selector . select ( timeout );
        if ready {
        return  [ key . fileobj for ( key , events ) in ready ];
        } else {
        if timeout is !None /* Option */ {
        timeout = deadline - time . monotonic ( );
        if timeout < 0 {
        return  ready;
        if sys . platform == "win32" {
        pub fn reduce_connection ( conn )  {
        handle = conn . fileno ( );
        // with scope: socket . fromfd ( handle , socket . AF_INET , socket . SOCK_STREAM ) as s  {
        from . import resource_sharer;
        ds = resource_sharer . DupSocket ( s );
        return  rebuild_connection , ( ds , conn . readable , conn . writable );
        pub fn rebuild_connection ( ds , readable , writable )  {
        sock = ds . detach ( );
        return  Connection ( sock . detach ( ) , readable , writable );
        reduction . register ( Connection , reduce_connection );
        pub fn reduce_pipe_connection ( conn )  {
        access = ( ( _winapi . FILE_GENERIC_READ if conn . readable else 0 ) |;
        ( _winapi . FILE_GENERIC_WRITE if conn . writable else 0 ) );
        dh = reduction . DupHandle ( conn . fileno ( ) , access );
        return  rebuild_pipe_connection , ( dh , conn . readable , conn . writable );
        pub fn rebuild_pipe_connection ( dh , readable , writable )  {
        handle = dh . detach ( );
        return  PipeConnection ( handle , readable , writable );
        reduction . register ( PipeConnection , reduce_pipe_connection );
        } else {
        pub fn reduce_connection ( conn )  {
        df = reduction . DupFd ( conn . fileno ( ) );
        return  rebuild_connection , ( df , conn . readable , conn . writable );
        pub fn rebuild_connection ( df , readable , writable )  {
        fd = df . detach ( );
        return  Connection ( fd , readable , writable );
        reduction . register ( Connection , reduce_connection );
}

