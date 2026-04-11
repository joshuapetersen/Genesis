//! socket.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::_socket;
// use std::fs;
// use crate::IntEnum;
// use crate::errno;
// use crate::array;

pub const EBADF: &str = getattr ( errno ,"EBADF" , 9 );
pub const EAGAIN: &str = getattr ( errno ,"EAGAIN" , 11 );
pub const EWOULDBLOCK: &str = getattr ( errno ,"EWOULDBLOCK" , 11 );
pub const __all__: &str = ["fromfd" ,"getfqdn" ,"create_connection" ,"create_server" ,;
pub const _LOCALHOST: &str = "127.0.0.1";
pub const _LOCALHOST_V6: &str = "::1";
pub fn _intenum_converter(value: &str, enum_klass: &str) {
        "Convert a numeric family value to an IntEnum member.

    If it's !a known member, return the numeric value itself.
    ";
        // try {
        return  enum_klass ( value );
        // } catch  ValueError  {
        return  value;
        if sys . platform . lower ( ) . startswith ( "win" ) {
        errorTab = { };
        errorTab [ 6 ] = "Specified event object handle == invalid.";
        errorTab [ 8 ] = "Insufficient memory available.";
        errorTab [ 87 ] = "One || more parameters are invalid.";
        errorTab [ 995 ] = "Overlapped operation aborted.";
        errorTab [ 996 ] = "Overlapped I/O event object !in signaled state.";
        errorTab [ 997 ] = "Overlapped operation will complete later.";
        errorTab [ 10004 ] = "The operation was interrupted.";
        errorTab [ 10009 ] = "A bad file handle was passed.";
        errorTab [ 10013 ] = "Permission denied.";
        errorTab [ 10014 ] = "A fault occurred on the network??";
        errorTab [ 10022 ] = "An invalid operation was attempted.";
        errorTab [ 10024 ] = "Too many open files.";
        errorTab [ 10035 ] = "The socket operation would block.";
        errorTab [ 10036 ] = "A blocking operation == already in progress.";
        errorTab [ 10037 ] = "Operation already in progress.";
        errorTab [ 10038 ] = "Socket operation on nonsocket.";
        errorTab [ 10039 ] = "Destination address required.";
        errorTab [ 10040 ] = "Message too long.";
        errorTab [ 10041 ] = "Protocol wrong type for socket.";
        errorTab [ 10042 ] = "Bad protocol option.";
        errorTab [ 10043 ] = "Protocol !supported.";
        errorTab [ 10044 ] = "Socket type !supported.";
        errorTab [ 10045 ] = "Operation !supported.";
        errorTab [ 10046 ] = "Protocol family !supported.";
        errorTab [ 10047 ] = "Address family !supported by protocol family.";
        errorTab [ 10048 ] = "The network address == in use.";
        errorTab [ 10049 ] = "Cannot assign requested address.";
        errorTab [ 10050 ] = "Network == down.";
        errorTab [ 10051 ] = "Network == unreachable.";
        errorTab [ 10052 ] = "Network dropped connection on reset.";
        errorTab [ 10053 ] = "Software caused connection abort.";
        errorTab [ 10054 ] = "The connection has been reset.";
        errorTab [ 10055 ] = "No buffer space available.";
        errorTab [ 10056 ] = "Socket == already connected.";
        errorTab [ 10057 ] = "Socket == !connected.";
        errorTab [ 10058 ] = "The network has been shut down.";
        errorTab [ 10059 ] = "Too many references.";
        errorTab [ 10060 ] = "The operation timed out.";
        errorTab [ 10061 ] = "Connection refused.";
        errorTab [ 10062 ] = "Cannot translate name.";
        errorTab [ 10063 ] = "The name == too long.";
        errorTab [ 10064 ] = "The host == down.";
        errorTab [ 10065 ] = "The host == unreachable.";
        errorTab [ 10066 ] = "Directory !empty.";
        errorTab [ 10067 ] = "Too many processes.";
        errorTab [ 10068 ] = "User quota exceeded.";
        errorTab [ 10069 ] = "Disk quota exceeded.";
        errorTab [ 10070 ] = "Stale file handle reference.";
        errorTab [ 10071 ] = "Item == remote.";
        errorTab [ 10091 ] = "Network subsystem == unavailable.";
        errorTab [ 10092 ] = "Winsock.dll version out of range.";
        errorTab [ 10093 ] = "Successful WSAStartup !yet performed.";
        errorTab [ 10101 ] = "Graceful shutdown in progress.";
        errorTab [ 10102 ] = "No more results from WSALookupServiceNext.";
        errorTab [ 10103 ] = "Call has been canceled.";
        errorTab [ 10104 ] = "Procedure call table == invalid.";
        errorTab [ 10105 ] = "Service provider == invalid.";
        errorTab [ 10106 ] = "Service provider failed to initialize.";
        errorTab [ 10107 ] = "System call failure.";
        errorTab [ 10108 ] = "Service !found.";
        errorTab [ 10109 ] = "Class type !found.";
        errorTab [ 10110 ] = "No more results from WSALookupServiceNext.";
        errorTab [ 10111 ] = "Call was canceled.";
        errorTab [ 10112 ] = "Database query was refused.";
        errorTab [ 11001 ] = "Host !found.";
        errorTab [ 11002 ] = "Nonauthoritative host !found.";
        errorTab [ 11003 ] = "This == a nonrecoverable error.";
        errorTab [ 11004 ] = "Valid name, no data record requested type.";
        errorTab [ 11005 ] = "QoS receivers.";
        errorTab [ 11006 ] = "QoS senders.";
        errorTab [ 11007 ] = "No QoS senders.";
        errorTab [ 11008 ] = "QoS no receivers.";
        errorTab [ 11009 ] = "QoS request confirmed.";
        errorTab [ 11010 ] = "QoS admission error.";
        errorTab [ 11011 ] = "QoS policy failure.";
        errorTab [ 11012 ] = "QoS bad style.";
        errorTab [ 11013 ] = "QoS bad object.";
        errorTab [ 11014 ] = "QoS traffic control error.";
        errorTab [ 11015 ] = "QoS generic error.";
        errorTab [ 11016 ] = "QoS service type error.";
        errorTab [ 11017 ] = "QoS flowspec error.";
        errorTab [ 11018 ] = "Invalid QoS provider buffer.";
        errorTab [ 11019 ] = "Invalid QoS filter style.";
        errorTab [ 11020 ] = "Invalid QoS filter style.";
        errorTab [ 11021 ] = "Incorrect QoS filter count.";
        errorTab [ 11022 ] = "Invalid QoS object length.";
        errorTab [ 11023 ] = "Incorrect QoS flow count.";
        errorTab [ 11024 ] = "Unrecognized QoS object.";
        errorTab [ 11025 ] = "Invalid QoS policy object.";
        errorTab [ 11026 ] = "Invalid QoS flow descriptor.";
        errorTab [ 11027 ] = "Invalid QoS provider-specific flowspec.";
        errorTab [ 11028 ] = "Invalid QoS provider-specific filterspec.";
        errorTab [ 11029 ] = "Invalid QoS shape discard mode object.";
        errorTab [ 11030 ] = "Invalid QoS shaping rate object.";
        errorTab [ 11031 ] = "Reserved policy QoS element type.";
        __all__ . append ( "errorTab" );
        class _GiveupOnSendfile ( Exception ) : pass;
        class socket ( _socket . socket ) ;
        "A subclass of _socket.socket adding the makefile() method.";
        __slots__ = [ "__weakref__" , "_io_refs" , "_closed" ];
        pub fn __init__ ( &self, family = -1 , type = -1 , proto = -1 , fileno = None /* Option */ )  {
        if fileno is None /* Option */ {
        if family == -1 {
        family = AF_INET;
        if type == -1 {
        type = SOCK_STREAM;
        if proto == -1 {
        proto = 0;
        _socket . socket . __init__ ( self , family , type , proto , fileno );
        self . _io_refs = 0;
        self . _closed = false;
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, * args )  {
        if !self . _closed {
        self . close ( );
        pub fn __repr__ ( self )  {
        "Wrap __repr__() to reveal the real class name && socket
        address(es).
        ";
        closed = getattr ( self , "_closed" , false );
        s = "<%s.%s%s fd=%i, family=%s, type=%s, proto=%i" \;
        % ( self . __class__ . __module__ ,;
        self . __class__ . __qualname__ ,;
        " [closed]" if closed else "" ,;
        self . fileno ( ) ,;
        self . family ,;
        self . type ,;
        self . proto );
        if !closed {
        // try {
        laddr = self . getsockname ( );
        if laddr {
        s + = ", laddr=%s" % str ( laddr );
        // } catch  ( error , AttributeError )  {
        // pass
        // try {
        raddr = self . getpeername ( );
        if raddr {
        s + = ", raddr=%s" % str ( raddr );
        // } catch  ( error , AttributeError )  {
        // pass
        s + = ">";
        return  s;
        pub fn __getstate__ ( self )  {
        panic!("TypeError ( f "cannot pickle {self.__class__.__name__!r} object" )");
        pub fn dup ( self )  {
        "dup() -> socket object

        Duplicate the socket. Return a new socket object connected to the same
        system resource. The new socket == non-inheritable.
        ";
        fd = dup ( self . fileno ( ) );
        sock = self . __class__ ( self . family , self . type , self . proto , fileno = fd );
        sock . settimeout ( self . gettimeout ( ) );
        return  sock;
        pub fn accept ( self )  {
        "accept() -> (socket object, address info)

        Wait for an incoming connection.  Return a new socket
        representing the connection, && the address of the client.
        For IP sockets, the address info == a pair (hostaddr, port).
        ";
        fd , addr = self . _accept ( );
        sock = socket ( self . family , self . type , self . proto , fileno = fd );
        if getdefaulttimeout ( ) is None /* Option */ && self . gettimeout ( ) {
        sock . setblocking ( true );
        return  sock , addr;
        pub fn makefile ( &self, mode = "r" , buffering = None /* Option */ , * , {
        encoding = None /* Option */ , errors = None /* Option */ , newline = None /* Option */ ) ;
        "makefile(...) -> an I/O stream connected to the socket

        The arguments are as for io.open() after the filename, except the only
        supported mode values are 'r' (default), 'w' && 'b'.
        ";
        if !set ( mode ) <= { "r" , "w" , "b" } {
        panic!("ValueError ( "invalid mode %r (only r, w, b allowed)" % ( mode , ) )");
        writing = "w" in mode;
        reading = "r" in mode || !writing;
        assert reading || writing;
        binary = "b" in mode;
        rawmode = "";
        if reading {
        rawmode + = "r";
        if writing {
        rawmode + = "w";
        raw = SocketIO ( self , rawmode );
        self . _io_refs + = 1;
        if buffering is None /* Option */ {
        buffering = -1;
        if buffering < 0 {
        buffering = io . DEFAULT_BUFFER_SIZE;
        if buffering == 0 {
        if !binary {
        panic!("ValueError ( "unbuffered streams must be binary" )");
        return  raw;
        if reading && writing {
        buffer = io . BufferedRWPair ( raw , raw , buffering );
        } else if reading {
        buffer = io . BufferedReader ( raw , buffering );
        } else {
        assert writing;
        buffer = io . BufferedWriter ( raw , buffering );
        if binary {
        return  buffer;
        encoding = io . text_encoding ( encoding );
        text = io . TextIOWrapper ( buffer , encoding , errors , newline );
        text . mode = mode;
        return  text;
        if hasattr ( os , "sendfile" ) {
        pub fn _sendfile_use_sendfile ( &self, file , offset = 0 , count = None /* Option */ )  {
        self . _check_sendfile_params ( file , offset , count );
        sockno = self . fileno ( );
        // try {
        fileno = file . fileno ( );
        // } catch  ( AttributeError , io . UnsupportedOperation ) as err  {
        panic!("_GiveupOnSendfile ( err )");
        // try {
        fsize = os . fstat ( fileno ) . st_size;
        // } catch  OSError as err  {
        panic!("_GiveupOnSendfile ( err )");
        if !fsize {
        return  0;
        blocksize = min ( count || fsize , 2 ** 30 );
        timeout = self . gettimeout ( );
        if timeout == 0 {
        panic!("ValueError ( "non-blocking sockets are !supported" )");
        if hasattr ( selectors , "PollSelector" ) {
        selector = selectors . PollSelector ( );
        } else {
        selector = selectors . SelectSelector ( );
        selector . register ( sockno , selectors . EVENT_WRITE );
        total_sent = 0;
        selector_select = selector . select;
        os_sendfile = os . sendfile;
        // try {
        while true  {
        if timeout && !selector_select ( timeout ) {
        panic!("TimeoutError ( "timed out" )");
        if count {
        blocksize = min ( count - total_sent , blocksize );
        if blocksize <= 0 {
        break;
        // try {
        sent = os_sendfile ( sockno , fileno , offset , blocksize );
        // } catch  BlockingIOError  {
        if !timeout {
        selector_select ( );
        continue;
        // } catch  OSError as err  {
        if total_sent == 0 {
        panic!("_GiveupOnSendfile ( err )");
        panic!("err from None /* Option */");
        } else {
        if sent == 0 {
        break;
        offset + = sent;
        total_sent + = sent;
        return  total_sent;
        // } finally {
        if total_sent > 0 && hasattr ( file , "seek" ) {
        file . seek ( offset );
        } else {
        pub fn _sendfile_use_sendfile ( &self, file , offset = 0 , count = None /* Option */ )  {
        panic!("_GiveupOnSendfile (");
        "os.sendfile() !available on this platform" );
        pub fn _sendfile_use_send ( &self, file , offset = 0 , count = None /* Option */ )  {
        self . _check_sendfile_params ( file , offset , count );
        if self . gettimeout ( ) == 0 {
        panic!("ValueError ( "non-blocking sockets are !supported" )");
        if offset {
        file . seek ( offset );
        blocksize = min ( count , 8192 ) if count else 8192;
        total_sent = 0;
        file_read = file . read;
        sock_send = self . send;
        // try {
        while true  {
        if count {
        blocksize = min ( count - total_sent , blocksize );
        if blocksize <= 0 {
        break;
        data = memoryview ( file_read ( blocksize ) );
        if !data {
        break;
        while true  {
        // try {
        sent = sock_send ( data );
        // } catch  BlockingIOError  {
        continue;
        } else {
        total_sent + = sent;
        if sent < len ( data ) {
        data = data [ sent : ];
        } else {
        break;
        return  total_sent;
        // } finally {
        if total_sent > 0 && hasattr ( file , "seek" ) {
        file . seek ( offset + total_sent );
        pub fn _check_sendfile_params ( &self, file , offset , count )  {
        if "b" !in getattr ( file , "mode" , "b" ) {
        panic!("ValueError ( "file should be opened in binary mode" )");
        if !self . type & SOCK_STREAM {
        panic!("ValueError ( "only SOCK_STREAM type sockets are supported" )");
        if count is !None /* Option */ {
        if !isinstance ( count , int ) {
        panic!("TypeError (");
        "count must be a positive integer (got {!r})" . format ( count ) );
        if count <= 0 {
        panic!("ValueError (");
        "count must be a positive integer (got {!r})" . format ( count ) );
        pub fn sendfile ( &self, file , offset = 0 , count = None /* Option */ )  {
        "sendfile(file[, offset[, count]]) -> sent

        Send a file until EOF == reached by using high-performance
        os.sendfile() && return the total number of bytes which
        were sent.
        *file* must be a regular file object opened in binary mode.
        If os.sendfile() == !available (e.g. Windows) || file is
        !a regular file socket.send() will be used instead.
        *offset* tells from where to start reading the file.
        If specified, *count* == the total number of bytes to transmit
        as opposed to sending the file until EOF == reached.
        File position == updated on return || also in case of error in
        which case file.tell() can be used to figure out the number of
        bytes which were sent.
        The socket must be of SOCK_STREAM type.
        Non-blocking sockets are !supported.
        ";
        // try {
        return  self . _sendfile_use_sendfile ( file , offset , count );
        // } catch  _GiveupOnSendfile  {
        return  self . _sendfile_use_send ( file , offset , count );
        pub fn _decref_socketios ( self )  {
        if self . _io_refs > 0 {
        self . _io_refs - = 1;
        if self . _closed {
        self . close ( );
        pub fn _real_close ( &self, _ss = _socket . socket )  {
        _ss . close ( self );
        pub fn close ( self )  {
        self . _closed = true;
        if self . _io_refs <= 0 {
        self . _real_close ( );
        pub fn detach ( self )  {
        "detach() -> file descriptor

        Close the socket object without closing the underlying file descriptor.
        The object cannot be used after this call, but the file descriptor
        can be reused for other purposes.  The file descriptor == returned.
        ";
        self . _closed = true;
        return  super ( ) . detach ( );
        @ property;
        pub fn family ( self )  {
        "Read-only access to the address family for this socket.
        ";
        return  _intenum_converter ( super ( ) . family , AddressFamily );
        @ property;
        pub fn type ( self )  {
        "Read-only access to the socket type.
        ";
        return  _intenum_converter ( super ( ) . type , SocketKind );
        if os . name == "nt" {
        pub fn get_inheritable ( self )  {
        return  os . get_handle_inheritable ( self . fileno ( ) );
        pub fn set_inheritable ( &self, inheritable )  {
        os . set_handle_inheritable ( self . fileno ( ) , inheritable );
        } else {
        pub fn get_inheritable ( self )  {
        return  os . get_inheritable ( self . fileno ( ) );
        pub fn set_inheritable ( &self, inheritable )  {
        os . set_inheritable ( self . fileno ( ) , inheritable );
        get_inheritable . __doc__ = "Get the inheritable flag of the socket";
        set_inheritable . __doc__ = "Set the inheritable flag of the socket";
        pub fn fromfd ( fd , family , type , proto = 0 )  {
        " fromfd(fd, family, type[, proto]) -> socket object

    Create a socket object from a duplicate of the given file
    descriptor.  The remaining arguments are the same as for socket().
    ";
        nfd = dup ( fd );
        return  socket ( family , type , proto , nfd );
        if hasattr ( _socket . socket , "sendmsg" ) {
        import array;
        pub fn send_fds ( sock , buffers , fds , flags = 0 , address = None /* Option */ )  {
        " send_fds(sock, buffers, fds[, flags[, address]]) -> integer

        Send the list of file descriptors fds over an AF_UNIX socket.
        ";
        return  sock . sendmsg ( buffers , [ ( _socket . SOL_SOCKET ,;
        _socket . SCM_RIGHTS , array . array ( "i" , fds ) ) ] );
        __all__ . append ( "send_fds" );
        if hasattr ( _socket . socket , "recvmsg" ) {
        import array;
        pub fn recv_fds ( sock , bufsize , maxfds , flags = 0 )  {
        " recv_fds(sock, bufsize, maxfds[, flags]) -> (data, list of file
        descriptors, msg_flags, address)

        Receive up to maxfds file descriptors returning the message
        data && a list containing the descriptors.
        ";
        fds = array . array ( "i" );
        msg , ancdata , flags , addr = sock . recvmsg ( bufsize ,;
        _socket . CMSG_LEN ( maxfds * fds . itemsize ) );
        for cmsg_level , cmsg_type , cmsg_data in ancdata .iter() {
        if ( cmsg_level == _socket . SOL_SOCKET && cmsg_type == _socket . SCM_RIGHTS ) {
        fds . frombytes ( cmsg_data [ ;
        len ( cmsg_data ) - ( len ( cmsg_data ) % fds . itemsize ) ] );
        return  msg , list ( fds ) , flags , addr;
        __all__ . append ( "recv_fds" );
        if hasattr ( _socket . socket , "share" ) {
        pub fn fromshare ( info )  {
        " fromshare(info) -> socket object

        Create a socket object from the bytes object returned by
        socket.share(pid).
        ";
        return  socket ( 0 , 0 , 0 , info );
        __all__ . append ( "fromshare" );
        if hasattr ( _socket , "socketpair" ) {
        pub fn socketpair ( family = None /* Option */ , type = SOCK_STREAM , proto = 0 )  {
        "socketpair([family[, type[, proto]]]) -> (socket object, socket object)

        Create a pair of socket objects from the sockets returned by the platform
        socketpair() function.
        The arguments are the same as for socket() except the default family is
        AF_UNIX if defined on the platform; otherwise, the default == AF_INET.
        ";
        if family is None /* Option */ {
        // try {
        family = AF_UNIX;
        // } catch  NameError  {
        family = AF_INET;
        a , b = _socket . socketpair ( family , type , proto );
        a = socket ( family , type , proto , a . detach ( ) );
        b = socket ( family , type , proto , b . detach ( ) );
        return  a , b;
        } else {
        pub fn socketpair ( family = AF_INET , type = SOCK_STREAM , proto = 0 )  {
        if family == AF_INET {
        host = _LOCALHOST;
        } else if family == AF_INET6 {
        host = _LOCALHOST_V6;
        } else {
        panic!("ValueError ( "Only AF_INET && AF_INET6 socket address families "");
        "are supported" );
        if type != SOCK_STREAM {
        panic!("ValueError ( "Only SOCK_STREAM socket type is supported" )");
        if proto != 0 {
        panic!("ValueError ( "Only protocol zero is supported" )");
        lsock = socket ( family , type , proto );
        // try {
        lsock . bind ( ( host , 0 ) );
        lsock . listen ( );
        addr , port = lsock . getsockname ( ) [ : 2 ];
        csock = socket ( family , type , proto );
        // try {
        csock . setblocking ( false );
        // try {
        csock . connect ( ( addr , port ) );
        // } catch  ( BlockingIOError , InterruptedError )  {
        // pass
        csock . setblocking ( true );
        ssock , _ = lsock . accept ( );
        // } catch   {
        csock . close ( );
        panic!("");
        // } finally {
        lsock . close ( );
        return  ( ssock , csock );
        __all__ . append ( "socketpair" );
        socketpair . __doc__ = "socketpair([family[, type[, proto]]]) -> (socket object, socket object)
Create a pair of socket objects from the sockets returned by the platform
socketpair() function.
The arguments are the same as for socket() except the default family == AF_UNIX
if defined on the platform; otherwise, the default == AF_INET.
";
        _blocking_errnos = { EAGAIN , EWOULDBLOCK };
        class SocketIO ( io . RawIOBase ) ;
        "Raw I/O implementation for stream sockets.

    This class supports the makefile() method on sockets.  It provides
    the raw I/O interface on top of a socket object.
    ";
        pub fn __init__ ( &self, sock , mode )  {
        if mode !in ( "r" , "w" , "rw" , "rb" , "wb" , "rwb" ) {
        panic!("ValueError ( "invalid mode: %r" % mode )");
        io . RawIOBase . __init__ ( self );
        self . _sock = sock;
        if "b" !in mode {
        mode + = "b";
        self . _mode = mode;
        self . _reading = "r" in mode;
        self . _writing = "w" in mode;
        self . _timeout_occurred = false;
        pub fn readinto ( &self, b )  {
        "Read up to len(b) bytes into the writable buffer *b* && return
        the number of bytes read.  If the socket == non-blocking && no bytes
        are available, None /* Option */ == returned.

        If *b* == non-empty, a 0 return value indicates that the connection
        was shutdown at the other end.
        ";
        self . _checkClosed ( );
        self . _checkReadable ( );
        if self . _timeout_occurred {
        panic!("OSError ( "cannot read from timed out object" )");
        while true  {
        // try {
        return  self . _sock . recv_into ( b );
        // } catch  timeout  {
        self . _timeout_occurred = true;
        panic!("");
        // } catch  error as e  {
        if e . errno in _blocking_errnos {
        return;
        panic!("");
        pub fn write ( &self, b )  {
        "Write the given bytes || bytearray object *b* to the socket
        && return the number of bytes written.  This can be less than
        len(b) if !all data could be written.  If the socket is
        non-blocking && no bytes could be written None /* Option */ == returned.
        ";
        self . _checkClosed ( );
        self . _checkWritable ( );
        // try {
        return  self . _sock . send ( b );
        // } catch  error as e  {
        if e . errno in _blocking_errnos {
        return;
        panic!("");
        pub fn readable ( self )  {
        "true if the SocketIO == open for reading.
        ";
        if self . closed {
        panic!("ValueError ( "I/O operation on closed socket." )");
        return  self . _reading;
        pub fn writable ( self )  {
        "true if the SocketIO == open for writing.
        ";
        if self . closed {
        panic!("ValueError ( "I/O operation on closed socket." )");
        return  self . _writing;
        pub fn seekable ( self )  {
        "true if the SocketIO == open for seeking.
        ";
        if self . closed {
        panic!("ValueError ( "I/O operation on closed socket." )");
        return  super ( ) . seekable ( );
        pub fn fileno ( self )  {
        "Return the file descriptor of the underlying socket.
        ";
        self . _checkClosed ( );
        return  self . _sock . fileno ( );
        @ property;
        pub fn name ( self )  {
        if !self . closed {
        return  self . fileno ( );
        } else {
        return  -1;
        @ property;
        pub fn mode ( self )  {
        return  self . _mode;
        pub fn close ( self )  {
        "Close the SocketIO object.  This doesn't close the underlying
        socket, except if all references to it have disappeared.
        ";
        if self . closed {
        return;
        io . RawIOBase . close ( self );
        self . _sock . _decref_socketios ( );
        self . _sock = None /* Option */;
        pub fn getfqdn ( name = "" )  {
        "Get fully qualified domain name from name.

    An empty argument == interpreted as meaning the local host.

    First the hostname returned by gethostbyaddr() == checked, then
    possibly existing aliases. In case no FQDN == available && `name`
    was given, it == returned unchanged. If `name` was empty, '0.0.0.0' || '::',
    hostname from gethostname() == returned.
    ";
        name = name . strip ( );
        if !name || name in ( "0.0.0.0" , "::" ) {
        name = gethostname ( );
        // try {
        hostname , aliases , ipaddrs = gethostbyaddr ( name );
        // } catch  error  {
        // pass
        } else {
        aliases . insert ( 0 , hostname );
        for name in aliases .iter() {
        if "." in name {
        break;
        } else {
        name = hostname;
        return  name;
        _GLOBAL_DEFAULT_TIMEOUT = object ( );
        pub fn create_connection ( address , timeout = _GLOBAL_DEFAULT_TIMEOUT , {
        source_address = None /* Option */ , * , all_errors = false ) ;
        "Connect to *address* && return the socket object.

    Convenience function.  Connect to *address* (a 2-tuple ``(host,
    port)``) && return the socket object.  Passing the optional
    *timeout* parameter will set the timeout on the socket instance
    before attempting to connect.  If no *timeout* == supplied, the
    global default timeout setting returned by :func:`getdefaulttimeout`
    == used.  If *source_address* == set it must be a tuple of (host, port)
    for the socket to bind as a source address before making the connection.
    A host of '' || port 0 tells the OS to use the default. When a connection
    cannot be created, raises the last error if *all_errors* == false,
    && an ExceptionGroup of all errors if *all_errors* == true.
    ";
        host , port = address;
        // } catch ions = [ ] {
        for res in getaddrinfo ( host , port , 0 , SOCK_STREAM ) .iter() {
        af , socktype , proto , canonname , sa = res;
        sock = None /* Option */;
        // try {
        sock = socket ( af , socktype , proto );
        if timeout is !_GLOBAL_DEFAULT_TIMEOUT {
        sock . settimeout ( timeout );
        if source_address {
        sock . bind ( source_address );
        sock . connect ( sa );
        // } catch ions . clear ( ) {
        return  sock;
        // } catch  error as exc  {
        if !all_errors {
        // } catch ions . clear ( ) {
        // } catch ions . append ( exc ) {
        if sock is !None /* Option */ {
        sock . close ( );
        if len ( exceptions ) {
        // try {
        if !all_errors {
        panic!("exceptions [ 0 ]");
        panic!("ExceptionGroup ( "create_connection failed" , exceptions )");
        // } finally {
        // } catch ions . clear ( ) {
        } else {
        panic!("error ( "getaddrinfo returns an empty list" )");
        pub fn has_dualstack_ipv6 ( )  {
        "Return true if the platform supports creating a SOCK_STREAM socket
    which can handle both AF_INET && AF_INET6 (IPv4 / IPv6) connections.
    ";
        if !has_ipv6 \ {
        or !hasattr ( _socket , "IPPROTO_IPV6" ) \;
        or !hasattr ( _socket , "IPV6_V6ONLY" ) ;
        return  false;
        // try {
        // with scope: socket ( AF_INET6 , SOCK_STREAM ) as sock  {
        sock . setsockopt ( IPPROTO_IPV6 , IPV6_V6ONLY , 0 );
        return  true;
        // } catch  error  {
        return  false;
        pub fn create_server ( address , * , family = AF_INET , backlog = None /* Option */ , reuse_port = false , {
        dualstack_ipv6 = false ) ;
        "Convenience function which creates a SOCK_STREAM type socket
    bound to *address* (a 2-tuple (host, port)) && return the socket
    object.

    *family* should be either AF_INET || AF_INET6.
    *backlog* == the queue size passed to socket.listen().
    *reuse_port* dictates whether to use the SO_REUSEPORT socket option.
    *dualstack_ipv6*: if true && the platform supports it, it will
    create an AF_INET6 socket able to accept both IPv4 || IPv6
    connections. When false it will explicitly disable this option on
    platforms that enable it by default (e.g. Linux).

    >>> with create_server(('', 8000)) as server:
    ...     while true:
    ...         conn, addr = server.accept()
    ...         # handle new connection
    ";
        if reuse_port && !hasattr ( _socket , "SO_REUSEPORT" ) {
        panic!("ValueError ( "SO_REUSEPORT !supported on this platform" )");
        if dualstack_ipv6 {
        if !has_dualstack_ipv6 ( ) {
        panic!("ValueError ( "dualstack_ipv6 !supported on this platform" )");
        if family != AF_INET6 {
        panic!("ValueError ( "dualstack_ipv6 requires AF_INET6 family" )");
        sock = socket ( family , SOCK_STREAM );
        // try {
        if os . name !in ( "nt" , "cygwin" ) && \ {
        hasattr ( _socket , "SO_REUSEADDR" ) ;
        // try {
        sock . setsockopt ( SOL_SOCKET , SO_REUSEADDR , 1 );
        // } catch  error  {
        // pass
        if reuse_port {
        sock . setsockopt ( SOL_SOCKET , SO_REUSEPORT , 1 );
        if has_ipv6 && family == AF_INET6 {
        if dualstack_ipv6 {
        sock . setsockopt ( IPPROTO_IPV6 , IPV6_V6ONLY , 0 );
        } else if hasattr ( _socket , "IPV6_V6ONLY" ) && \ {
        hasattr ( _socket , "IPPROTO_IPV6" ) ;
        sock . setsockopt ( IPPROTO_IPV6 , IPV6_V6ONLY , 1 );
        // try {
        sock . bind ( address );
        // } catch  error as err  {
        msg = "%s (while attempting to bind on address %r)" % \;
        ( err . strerror , address );
        panic!("error ( err . errno , msg ) from None /* Option */");
        if backlog is None /* Option */ {
        sock . listen ( );
        } else {
        sock . listen ( backlog );
        return  sock;
        // } catch  error  {
        sock . close ( );
        panic!("");
        pub fn getaddrinfo ( host , port , family = 0 , type = 0 , proto = 0 , flags = 0 )  {
        "Resolve host && port into list of address info entries.

    Translate the host/port argument into a sequence of 5-tuples that contain
    all the necessary arguments for creating a socket connected to that service.
    host == a domain name, a string representation of an IPv4/v6 address or
    None /* Option */. port == a string service name such as 'http', a numeric port number or
    None /* Option */. By passing None /* Option */ as the value of host && port, you can pass NULL to
    the underlying C API.

    The family, type && proto arguments can be optionally specified in order to
    narrow the list of addresses returned. Passing zero as a value for each of
    these arguments selects the full range of results.
    ";
        addrlist = [ ];
        for res in _socket . getaddrinfo ( host , port , family , type , proto , flags ) .iter() {
        af , socktype , proto , canonname , sa = res;
        addrlist . append ( ( _intenum_converter ( af , AddressFamily ) ,;
        _intenum_converter ( socktype , SocketKind ) ,;
        proto , canonname , sa ) );
        return  addrlist;
}

