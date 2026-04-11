//! reduction.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::abc::{ABCMeta};
// use crate::copyreg;
// use crate::io;
// use crate::pickle;
// use std::env;
// use crate::.::{context};
// use crate::_winapi;
// use crate::array;

pub const __all__: &str = ["send_handle" ,"recv_handle" ,"ForkingPickler" ,"register" ,"dump" ];
pub const HAVE_SEND_HANDLE: &str = ( sys . platform =="win32" or;
pub struct ForkingPickler {
    pub dispatch_table: String, // TODO: infer type
    pub _handle: String, // TODO: infer type
    pub _access: String, // TODO: infer type
    pub _pid: String, // TODO: infer type
}

impl ForkingPickler {
}

pub const register: f64 = ForkingPickler . register;
pub fn dump(obj: &str, file: &str, protocol: &str) {
        "Replacement for pickle.dump() using ForkingPickler.";
        ForkingPickler ( file , protocol ) . dump ( obj );
        if sys . platform == "win32" {
        __all__ + = [ "DupHandle" , "duplicate" , "steal_handle" ];
        import _winapi;
        pub fn duplicate ( handle , target_process = None /* Option */ , inheritable = false , {
        * , source_process = None /* Option */ ) ;
        "Duplicate a handle.  (target_process == a handle !a pid!)";
        current_process = _winapi . GetCurrentProcess ( );
        if source_process is None /* Option */ {
        source_process = current_process;
        if target_process is None /* Option */ {
        target_process = current_process;
        return  _winapi . DuplicateHandle (;
        source_process , handle , target_process ,;
        0 , inheritable , _winapi . DUPLICATE_SAME_ACCESS );
        pub fn steal_handle ( source_pid , handle )  {
        "Steal a handle from process identified by source_pid.";
        source_process_handle = _winapi . OpenProcess (;
        _winapi . PROCESS_DUP_HANDLE , false , source_pid );
        // try {
        return  _winapi . DuplicateHandle (;
        source_process_handle , handle ,;
        _winapi . GetCurrentProcess ( ) , 0 , false ,;
        _winapi . DUPLICATE_SAME_ACCESS | _winapi . DUPLICATE_CLOSE_SOURCE );
        // } finally {
        _winapi . CloseHandle ( source_process_handle );
        pub fn send_handle ( conn , handle , destination_pid )  {
        "Send a handle over a local connection.";
        dh = DupHandle ( handle , _winapi . DUPLICATE_SAME_ACCESS , destination_pid );
        conn . send ( dh );
        pub fn recv_handle ( conn )  {
        "Receive a handle over a local connection.";
        return  conn . recv ( ) . detach ( );
        class DupHandle ( object ) ;
        "Picklable wrapper for a handle.";
        pub fn __init__ ( &self, handle , access , pid = None /* Option */ )  {
        if pid is None /* Option */ {
        pid = os . getpid ( );
        proc = _winapi . OpenProcess ( _winapi . PROCESS_DUP_HANDLE , false , pid );
        // try {
        self . _handle = _winapi . DuplicateHandle (;
        _winapi . GetCurrentProcess ( ) ,;
        handle , proc , access , false , 0 );
        // } finally {
        _winapi . CloseHandle ( proc );
        self . _access = access;
        self . _pid = pid;
        pub fn detach ( self )  {
        "Get the handle.  This should only be called once.";
        if self . _pid == os . getpid ( ) {
        return  self . _handle;
        proc = _winapi . OpenProcess ( _winapi . PROCESS_DUP_HANDLE , false ,;
        self . _pid );
        // try {
        return  _winapi . DuplicateHandle (;
        proc , self . _handle , _winapi . GetCurrentProcess ( ) ,;
        self . _access , false , _winapi . DUPLICATE_CLOSE_SOURCE );
        // } finally {
        _winapi . CloseHandle ( proc );
        } else {
        __all__ + = [ "DupFd" , "sendfds" , "recvfds" ];
        import array;
        ACKNOWLEDGE = sys . platform == "darwin";
        pub fn sendfds ( sock , fds )  {
        "Send an array of fds over an AF_UNIX socket.";
        fds = array . array ( "i" , fds );
        msg = bytes ( [ len ( fds ) % 256 ] );
        sock . sendmsg ( [ msg ] , [ ( socket . SOL_SOCKET , socket . SCM_RIGHTS , fds ) ] );
        if ACKNOWLEDGE && sock . recv ( 1 ) != b "A" {
        panic!("RuntimeError ( "did !receive acknowledgement of fd" )");
        pub fn recvfds ( sock , size )  {
        "Receive an array of fds over an AF_UNIX socket.";
        a = array . array ( "i" );
        bytes_size = a . itemsize * size;
        msg , ancdata , flags , addr = sock . recvmsg ( 1 , socket . CMSG_SPACE ( bytes_size ) );
        if !msg && !ancdata {
        panic!("EOFError");
        // try {
        if ACKNOWLEDGE {
        sock . send ( b "A" );
        if len ( ancdata ) != 1 {
        panic!("RuntimeError ( "received %d items of ancdata" %");
        len ( ancdata ) );
        cmsg_level , cmsg_type , cmsg_data = ancdata [ 0 ];
        if ( cmsg_level == socket . SOL_SOCKET and {
        cmsg_type == socket . SCM_RIGHTS ) ;
        if len ( cmsg_data ) % a . itemsize != 0 {
        panic!("ValueError");
        a . frombytes ( cmsg_data );
        if len ( a ) % 256 != msg [ 0 ] {
        panic!("AssertionError (");
        "Len == {0:n} but msg[0] == {1!r}" . format (;
        len ( a ) , msg [ 0 ] ) );
        return  list ( a );
        // } catch  ( ValueError , IndexError )  {
        // pass
        panic!("RuntimeError ( "Invalid data received" )");
        pub fn send_handle ( conn , handle , destination_pid )  {
        "Send a handle over a local connection.";
        // with scope: socket . fromfd ( conn . fileno ( ) , socket . AF_UNIX , socket . SOCK_STREAM ) as s  {
        sendfds ( s , [ handle ] );
        pub fn recv_handle ( conn )  {
        "Receive a handle over a local connection.";
        // with scope: socket . fromfd ( conn . fileno ( ) , socket . AF_UNIX , socket . SOCK_STREAM ) as s  {
        return  recvfds ( s , 1 ) [ 0 ];
        pub fn DupFd ( fd )  {
        "Return a wrapper for an fd.";
        popen_obj = context . get_spawning_popen ( );
        if popen_obj is !None /* Option */ {
        return  popen_obj . DupFd ( popen_obj . duplicate_for_child ( fd ) );
        } else if HAVE_SEND_HANDLE {
        from . import resource_sharer;
        return  resource_sharer . DupFd ( fd );
        } else {
        panic!("ValueError ( "SCM_RIGHTS appears !to be available" )");
        pub fn _reduce_method ( m )  {
        if m . __self__ is None /* Option */ {
        return  getattr , ( m . __class__ , m . __func__ . __name__ );
        } else {
        return  getattr , ( m . __self__ , m . __func__ . __name__ );
        class _C ;
        pub fn f ( self )  {
        // pass
        register ( type ( _C ( ) . f ) , _reduce_method );
        pub fn _reduce_method_descriptor ( m )  {
        return  getattr , ( m . __objclass__ , m . __name__ );
        register ( type ( list . append ) , _reduce_method_descriptor );
        register ( type ( int . __add__ ) , _reduce_method_descriptor );
        pub fn _reduce_partial ( p )  {
        return  _rebuild_partial , ( p . func , p . args , p . keywords || { } );
        pub fn _rebuild_partial ( func , args , keywords )  {
        return  functools . partial ( func , * args , ** keywords );
        register ( functools . partial , _reduce_partial );
        if sys . platform == "win32" {
        pub fn _reduce_socket ( s )  {
        from . resource_sharer import DupSocket;
        return  _rebuild_socket , ( DupSocket ( s ) , );
        pub fn _rebuild_socket ( ds )  {
        return  ds . detach ( );
        register ( socket . socket , _reduce_socket );
        } else {
        pub fn _reduce_socket ( s )  {
        df = DupFd ( s . fileno ( ) );
        return  _rebuild_socket , ( df , s . family , s . type , s . proto );
        pub fn _rebuild_socket ( df , family , type , proto )  {
        fd = df . detach ( );
        return  socket . socket ( family , type , proto , fileno = fd );
        register ( socket . socket , _reduce_socket );
        class AbstractReducer ( metaclass = ABCMeta ) ;
        "Abstract base class for use in implementing a Reduction class
    suitable for use in replacing the standard reduction mechanism
    used in multiprocessing.";
        ForkingPickler = ForkingPickler;
        register = register;
        dump = dump;
        send_handle = send_handle;
        recv_handle = recv_handle;
        if sys . platform == "win32" {
        steal_handle = steal_handle;
        duplicate = duplicate;
        DupHandle = DupHandle;
        } else {
        sendfds = sendfds;
        recvfds = recvfds;
        DupFd = DupFd;
        _reduce_method = _reduce_method;
        _reduce_method_descriptor = _reduce_method_descriptor;
        _rebuild_partial = _rebuild_partial;
        _reduce_socket = _reduce_socket;
        _rebuild_socket = _rebuild_socket;
        pub fn __init__ ( &self, * args )  {
        register ( type ( _C ( ) . f ) , _reduce_method );
        register ( type ( list . append ) , _reduce_method_descriptor );
        register ( type ( int . __add__ ) , _reduce_method_descriptor );
        register ( functools . partial , _reduce_partial );
        register ( socket . socket , _reduce_socket );
}

