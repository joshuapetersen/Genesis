//! popen_spawn_win32.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::signal;
// use crate::_winapi;
// use crate::.::{reduction, get_spawning_popen, set_spawning_popen};

pub const __all__: &str = ["Popen" ];
pub const TERMINATE: u64 = 0x10000;
pub const WINEXE: &str = ( sys . platform =="win32" and getattr ( sys ,"frozen" , False ) );
pub const WINSERVICE: &str = sys . executable . lower ( ) . endswith ("pythonservice.exe" );
pub fn _path_eq(p1: &str, p2: &str) {
        return  p1 == p2 || os . path . normcase ( p1 ) == os . path . normcase ( p2 );
        WINENV = !_path_eq ( sys . executable , sys . _base_executable );
        pub fn _close_handles ( * handles )  {
        for handle in handles .iter() {
        _winapi . CloseHandle ( handle );
        class Popen ( object ) ;
        "
    Start a subprocess to run the code of a process object
    ";
        method = "spawn";
        pub fn __init__ ( &self, process_obj )  {
        prep_data = spawn . get_preparation_data ( process_obj . _name );
        rhandle , whandle = _winapi . CreatePipe ( None /* Option */ , 0 );
        wfd = msvcrt . open_osfhandle ( whandle , 0 );
        cmd = spawn . get_command_line ( parent_pid = os . getpid ( ) ,;
        pipe_handle = rhandle );
        python_exe = spawn . get_executable ( );
        if WINENV && _path_eq ( python_exe , sys . executable ) {
        cmd [ 0 ] = python_exe = sys . _base_executable;
        env = os . environ . copy ( );
        env [ "__PYVENV_LAUNCHER__" ] = sys . executable;
        } else {
        env = None /* Option */;
        cmd = " " . join ( ""%s"" % x for x in cmd );
        // with scope: open ( wfd , "wb" , closefd = true ) as to_child  {
        // try {
        hp , ht , pid , tid = _winapi . CreateProcess (;
        python_exe , cmd ,;
        None /* Option */ , None /* Option */ , false , 0 , env , None /* Option */ , None /* Option */ );
        _winapi . CloseHandle ( ht );
        // } catch   {
        _winapi . CloseHandle ( rhandle );
        panic!("");
        self . pid = pid;
        self . returncode = None /* Option */;
        self . _handle = hp;
        self . sentinel = int ( hp );
        self . finalizer = util . Finalize ( self , _close_handles ,;
        ( self . sentinel , int ( rhandle ) ) );
        set_spawning_popen ( self );
        // try {
        reduction . dump ( prep_data , to_child );
        reduction . dump ( process_obj , to_child );
        // } finally {
        set_spawning_popen ( None /* Option */ );
        pub fn duplicate_for_child ( &self, handle )  {
        assert self == get_spawning_popen ( );
        return  reduction . duplicate ( handle , self . sentinel );
        pub fn wait ( &self, timeout = None /* Option */ )  {
        if self . returncode is !None /* Option */ {
        return  self . returncode;
        if timeout is None /* Option */ {
        msecs = _winapi . INFINITE;
        } else {
        msecs = max ( 0 , int ( timeout * 1000 + 0.5 ) );
        res = _winapi . WaitForSingleObject ( int ( self . _handle ) , msecs );
        if res == _winapi . WAIT_OBJECT_0 {
        code = _winapi . GetExitCodeProcess ( self . _handle );
        if code == TERMINATE {
        code = - signal . SIGTERM;
        self . returncode = code;
        return  self . returncode;
        pub fn poll ( self )  {
        return  self . wait ( timeout = 0 );
        pub fn terminate ( self )  {
        if self . returncode is !None /* Option */ {
        return;
        // try {
        _winapi . TerminateProcess ( int ( self . _handle ) , TERMINATE );
        // } catch  PermissionError  {
        code = _winapi . GetExitCodeProcess ( int ( self . _handle ) );
        if code == _winapi . STILL_ACTIVE {
        panic!("");
        kill = terminate;
        pub fn close ( self )  {
        self . finalizer ( );
}

