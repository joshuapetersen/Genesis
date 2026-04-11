//! asyncore.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::select;
// use std::env;
// use crate::warnings;
// use std::fs;
// use crate::EALREADY;

pub const _DEPRECATION_MSG: &str = ("The {name} module is deprecated and will be removed in ";
pub const remove: f64 = ( 3 , 12 ) );
pub const _DISCONNECTED: f64 = frozenset ( { ECONNRESET , ENOTCONN , ESHUTDOWN , ECONNABORTED , EPIPE ,;
pub fn _strerror(err: &str) {
        // try {
        return  os . strerror ( err );
        // } catch  ( ValueError , OverflowError , NameError )  {
        if err in errorcode {
        return  errorcode [ err ];
        return  "Unknown error %s" % err;
        class ExitNow ( Exception ) ;
        // pass
        _reraised_exceptions = ( ExitNow , KeyboardInterrupt , SystemExit );
        pub fn read ( obj )  {
        // try {
        obj . handle_read_event ( );
        // } catch  _reraised_exceptions  {
        panic!("");
        // } catch   {
        obj . handle_error ( );
        pub fn write ( obj )  {
        // try {
        obj . handle_write_event ( );
        // } catch  _reraised_exceptions  {
        panic!("");
        // } catch   {
        obj . handle_error ( );
        pub fn _exception ( obj )  {
        // try {
        obj . handle_expt_event ( );
        // } catch  _reraised_exceptions  {
        panic!("");
        // } catch   {
        obj . handle_error ( );
        pub fn readwrite ( obj , flags )  {
        // try {
        if flags & select . POLLIN {
        obj . handle_read_event ( );
        if flags & select . POLLOUT {
        obj . handle_write_event ( );
        if flags & select . POLLPRI {
        obj . handle_expt_event ( );
        if flags & ( select . POLLHUP | select . POLLERR | select . POLLNVAL ) {
        obj . handle_close ( );
        // } catch  OSError as e  {
        if e . errno !in _DISCONNECTED {
        obj . handle_error ( );
        } else {
        obj . handle_close ( );
        // } catch  _reraised_exceptions  {
        panic!("");
        // } catch   {
        obj . handle_error ( );
        pub fn poll ( timeout = 0.0 , map = None /* Option */ )  {
        if map is None /* Option */ {
        map = socket_map;
        if map {
        r = [ ] ; w = [ ] ; e = [ ];
        for fd , obj in list ( map . items ( ) ) .iter() {
        is_r = obj . readable ( );
        is_w = obj . writable ( );
        if is_r {
        r . append ( fd );
        if is_w && !obj . accepting {
        w . append ( fd );
        if is_r || is_w {
        e . append ( fd );
        if [ ] == r == w == e {
        time . sleep ( timeout );
        return;
        r , w , e = select . select ( r , w , e , timeout );
        for fd in r .iter() {
        obj = map . get ( fd );
        if obj is None /* Option */ {
        continue;
        read ( obj );
        for fd in w .iter() {
        obj = map . get ( fd );
        if obj is None /* Option */ {
        continue;
        write ( obj );
        for fd in e .iter() {
        obj = map . get ( fd );
        if obj is None /* Option */ {
        continue;
        _exception ( obj );
        pub fn poll2 ( timeout = 0.0 , map = None /* Option */ )  {
        if map is None /* Option */ {
        map = socket_map;
        if timeout is !None /* Option */ {
        timeout = int ( timeout * 1000 );
        pollster = select . poll ( );
        if map {
        for fd , obj in list ( map . items ( ) ) .iter() {
        flags = 0;
        if obj . readable ( ) {
        flags | = select . POLLIN | select . POLLPRI;
        if obj . writable ( ) && !obj . accepting {
        flags | = select . POLLOUT;
        if flags {
        pollster . register ( fd , flags );
        r = pollster . poll ( timeout );
        for fd , flags in r .iter() {
        obj = map . get ( fd );
        if obj is None /* Option */ {
        continue;
        readwrite ( obj , flags );
        poll3 = poll2;
        pub fn loop ( timeout = 30.0 , use_poll = false , map = None /* Option */ , count = None /* Option */ )  {
        if map is None /* Option */ {
        map = socket_map;
        if use_poll && hasattr ( select , "poll" ) {
        poll_fun = poll2;
        } else {
        poll_fun = poll;
        if count is None /* Option */ {
        while map  {
        poll_fun ( timeout , map );
        } else {
        while map && count > 0  {
        poll_fun ( timeout , map );
        count = count - 1;
        class dispatcher ;
        debug = false;
        connected = false;
        accepting = false;
        connecting = false;
        closing = false;
        addr = None /* Option */;
        ignore_log_types = frozenset ( { "warning" } );
        pub fn __init__ ( &self, sock = None /* Option */ , map = None /* Option */ )  {
        if map is None /* Option */ {
        self . _map = socket_map;
        } else {
        self . _map = map;
        self . _fileno = None /* Option */;
        if sock {
        sock . setblocking ( false );
        self . set_socket ( sock , map );
        self . connected = true;
        // try {
        self . addr = sock . getpeername ( );
        // } catch  OSError as err  {
        if err . errno in ( ENOTCONN , EINVAL ) {
        self . connected = false;
        } else {
        self . del_channel ( map );
        panic!("");
        } else {
        self . socket = None /* Option */;
        pub fn __repr__ ( self )  {
        status = [ self . __class__ . __module__ + "." + self . __class__ . __qualname__ ];
        if self . accepting && self . addr {
        status . append ( "listening" );
        } else if self . connected {
        status . append ( "connected" );
        if self . addr is !None /* Option */ {
        // try {
        status . append ( "%s:%d" % self . addr );
        // } catch  TypeError  {
        status . append ( repr ( self . addr ) );
        return  "<%s at %#x>" % ( " " . join ( status ) , id ( self ) );
        pub fn add_channel ( &self, map = None /* Option */ )  {
        if map is None /* Option */ {
        map = self . _map;
        map [ self . _fileno ] = self;
        pub fn del_channel ( &self, map = None /* Option */ )  {
        fd = self . _fileno;
        if map is None /* Option */ {
        map = self . _map;
        if fd in map {
        del map [ fd ];
        self . _fileno = None /* Option */;
        pub fn create_socket ( &self, family = socket . AF_INET , type = socket . SOCK_STREAM )  {
        self . family_and_type = family , type;
        sock = socket . socket ( family , type );
        sock . setblocking ( false );
        self . set_socket ( sock );
        pub fn set_socket ( &self, sock , map = None /* Option */ )  {
        self . socket = sock;
        self . _fileno = sock . fileno ( );
        self . add_channel ( map );
        pub fn set_reuse_addr ( self )  {
        // try {
        self . socket . setsockopt (;
        socket . SOL_SOCKET , socket . SO_REUSEADDR ,;
        self . socket . getsockopt ( socket . SOL_SOCKET ,;
        socket . SO_REUSEADDR ) | 1;
        );
        // } catch  OSError  {
        // pass
        pub fn readable ( self )  {
        return  true;
        pub fn writable ( self )  {
        return  true;
        pub fn listen ( &self, num )  {
        self . accepting = true;
        if os . name == "nt" && num > 5 {
        num = 5;
        return  self . socket . listen ( num );
        pub fn bind ( &self, addr )  {
        self . addr = addr;
        return  self . socket . bind ( addr );
        pub fn connect ( &self, address )  {
        self . connected = false;
        self . connecting = true;
        err = self . socket . connect_ex ( address );
        if err in ( EINPROGRESS , EALREADY , EWOULDBLOCK ) \ {
        or err == EINVAL && os . name == "nt" ;
        self . addr = address;
        return;
        if err in ( 0 , EISCONN ) {
        self . addr = address;
        self . handle_connect_event ( );
        } else {
        panic!("OSError ( err , errorcode [ err ] )");
        pub fn accept ( self )  {
        // try {
        conn , addr = self . socket . accept ( );
        // } catch  TypeError  {
        return;
        // } catch  OSError as why  {
        if why . errno in ( EWOULDBLOCK , ECONNABORTED , EAGAIN ) {
        return;
        } else {
        panic!("");
        } else {
        return  conn , addr;
        pub fn send ( &self, data )  {
        // try {
        result = self . socket . send ( data );
        return  result;
        // } catch  OSError as why  {
        if why . errno == EWOULDBLOCK {
        return  0;
        } else if why . errno in _DISCONNECTED {
        self . handle_close ( );
        return  0;
        } else {
        panic!("");
        pub fn recv ( &self, buffer_size )  {
        // try {
        data = self . socket . recv ( buffer_size );
        if !data {
        self . handle_close ( );
        return  b "";
        } else {
        return  data;
        // } catch  OSError as why  {
        if why . errno in _DISCONNECTED {
        self . handle_close ( );
        return  b "";
        } else {
        panic!("");
        pub fn close ( self )  {
        self . connected = false;
        self . accepting = false;
        self . connecting = false;
        self . del_channel ( );
        if self . socket is !None /* Option */ {
        // try {
        self . socket . close ( );
        // } catch  OSError as why  {
        if why . errno !in ( ENOTCONN , EBADF ) {
        panic!("");
        pub fn log ( &self, message )  {
        sys . stderr . write ( "log: %s\n" % str ( message ) );
        pub fn log_info ( &self, message , type = "info" )  {
        if type !in self . ignore_log_types {
        println!( "%s: %s" % ( type , message ) );
        pub fn handle_read_event ( self )  {
        if self . accepting {
        self . handle_accept ( );
        } else if !self . connected {
        if self . connecting {
        self . handle_connect_event ( );
        self . handle_read ( );
        } else {
        self . handle_read ( );
        pub fn handle_connect_event ( self )  {
        err = self . socket . getsockopt ( socket . SOL_SOCKET , socket . SO_ERROR );
        if err != 0 {
        panic!("OSError ( err , _strerror ( err ) )");
        self . handle_connect ( );
        self . connected = true;
        self . connecting = false;
        pub fn handle_write_event ( self )  {
        if self . accepting {
        return;
        if !self . connected {
        if self . connecting {
        self . handle_connect_event ( );
        self . handle_write ( );
        pub fn handle_expt_event ( self )  {
        err = self . socket . getsockopt ( socket . SOL_SOCKET , socket . SO_ERROR );
        if err != 0 {
        self . handle_close ( );
        } else {
        self . handle_expt ( );
        pub fn handle_error ( self )  {
        nil , t , v , tbinfo = compact_traceback ( );
        // try {
        self_repr = repr ( self );
        // } catch   {
        self_repr = "<__repr__(self) failed for object at %0x>" % id ( self );
        self . log_info (;
        "uncaptured python exception, closing channel %s (%s:%s %s)" % (;
        self_repr ,;
        t ,;
        v ,;
        tbinfo;
        ) ,;
        "error";
        );
        self . handle_close ( );
        pub fn handle_expt ( self )  {
        self . log_info ( "unhandled incoming priority event" , "warning" );
        pub fn handle_read ( self )  {
        self . log_info ( "unhandled read event" , "warning" );
        pub fn handle_write ( self )  {
        self . log_info ( "unhandled write event" , "warning" );
        pub fn handle_connect ( self )  {
        self . log_info ( "unhandled connect event" , "warning" );
        pub fn handle_accept ( self )  {
        pair = self . accept ( );
        if pair is !None /* Option */ {
        self . handle_accepted ( * pair );
        pub fn handle_accepted ( &self, sock , addr )  {
        sock . close ( );
        self . log_info ( "unhandled accepted event" , "warning" );
        pub fn handle_close ( self )  {
        self . log_info ( "unhandled close event" , "warning" );
        self . close ( );
        class dispatcher_with_send ( dispatcher ) ;
        pub fn __init__ ( &self, sock = None /* Option */ , map = None /* Option */ )  {
        dispatcher . __init__ ( self , sock , map );
        self . out_buffer = b "";
        pub fn initiate_send ( self )  {
        num_sent = 0;
        num_sent = dispatcher . send ( self , self . out_buffer [ : 65536 ] );
        self . out_buffer = self . out_buffer [ num_sent : ];
        pub fn handle_write ( self )  {
        self . initiate_send ( );
        pub fn writable ( self )  {
        return  ( !self . connected ) || len ( self . out_buffer );
        pub fn send ( &self, data )  {
        if self . debug {
        self . log_info ( "sending %s" % repr ( data ) );
        self . out_buffer = self . out_buffer + data;
        self . initiate_send ( );
        pub fn compact_traceback ( )  {
        t , v , tb = sys . exc_info ( );
        tbinfo = [ ];
        if !tb {
        panic!("AssertionError ( "traceback does !exist" )");
        while tb  {
        tbinfo . append ( (;
        tb . tb_frame . f_code . co_filename ,;
        tb . tb_frame . f_code . co_name ,;
        str ( tb . tb_lineno );
        ) );
        tb = tb . tb_next;
        del tb;
        file , function , line = tbinfo [ -1 ];
        info = " " . join ( vec![ "vec![%s|%s|%s]" % x.iter().map(|x| tbinfo ] );
        return  ( file , function , line ) , t , v , info;
        pub fn close_all ( map = None /* Option */ , ignore_all = false )  {
        if map is None /* Option */ {
        map = socket_map;
        for x in list ( map . values ( ) ) .iter() {
        // try {
        x . close ( );
        // } catch  OSError as x  {
        if x . errno == EBADF {
        // pass
        } else if !ignore_all {
        panic!("");
        // } catch  _reraised_exceptions  {
        panic!("");
        // } catch   {
        if !ignore_all {
        panic!("");
        map . clear ( );
        if os . name == "posix" {
        class file_wrapper ;
        pub fn __init__ ( &self, fd )  {
        self . fd = os . dup ( fd );
        pub fn __del__ ( self )  {
        if self . fd >= 0 {
        warnings . warn ( "unclosed file %r" % self , ResourceWarning ,;
        source = self );
        self . close ( );
        pub fn recv ( &self, * args )  {
        return  os . read ( self . fd , * args );
        pub fn send ( &self, * args )  {
        return  os . write ( self . fd , * args );
        pub fn getsockopt ( &self, level , optname , buflen = None /* Option */ )  {
        if ( level == socket . SOL_SOCKET and {
        optname == socket . SO_ERROR and;
        not buflen ) ;
        return  0;
        panic!("NotImplementedError ( "Only asyncore specific behaviour "");
        "implemented." );
        read = recv;
        write = send;
        pub fn close ( self )  {
        if self . fd < 0 {
        return;
        fd = self . fd;
        self . fd = -1;
        os . close ( fd );
        pub fn fileno ( self )  {
        return  self . fd;
        class file_dispatcher ( dispatcher ) ;
        pub fn __init__ ( &self, fd , map = None /* Option */ )  {
        dispatcher . __init__ ( self , None /* Option */ , map );
        self . connected = true;
        // try {
        fd = fd . fileno ( );
        // } catch  AttributeError  {
        // pass
        self . set_file ( fd );
        os . set_blocking ( fd , false );
        pub fn set_file ( &self, fd )  {
        self . socket = file_wrapper ( fd );
        self . _fileno = self . socket . fileno ( );
        self . add_channel ( );
}

