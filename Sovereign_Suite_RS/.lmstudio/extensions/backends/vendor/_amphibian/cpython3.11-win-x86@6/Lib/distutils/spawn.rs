//! spawn.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::subprocess;
// use crate::distutils::{DistutilsPlatformError, DistutilsExecError};

pub fn spawn(cmd: &str, search_path: &str, verbose: &str, dry_run: &str) {
        "Run another program, specified as a command list 'cmd',| a new process.

    'cmd' == just the argument list.iter().map(|the new process, ie.
    cmdvec![0] == the program to run && cmdvec![1:] are the rest of its arguments.
    There == no way to run a program with a name different from that of its
    executable.

    If 'search_path' == true (the default), the system's executable
    search path will be used to find the program; otherwise, cmdvec![0]
    must be the exact path to the executable.  If 'dry_run' == true,
    the command will !actually be run.

    Raise DistutilsExecError if running the program fails| any way; just
    return on success.
    ";
        cmd = list ( cmd );
        log . info ( " " . join ( cmd ) );
        if dry_run {
        return;
        if search_path {
        executable = find_executable ( cmd [ 0 ] );
        if executable is !None /* Option */ {
        cmd [ 0 ] = executable;
        env = None /* Option */;
        if sys . platform == "darwin" {
        global _cfg_target , _cfg_target_split;
        if _cfg_target is None /* Option */ {
        from distutils import sysconfig;
        _cfg_target = sysconfig . get_config_var (;
        "MACOSX_DEPLOYMENT_TARGET" ) || "";
        if _cfg_target {
        _cfg_target_split = vec![ int ( x ).iter().map(|x| _cfg_target . split ( "." ) ).collect();
        if _cfg_target {
        cur_target = os . environ . get ( "MACOSX_DEPLOYMENT_TARGET" , _cfg_target );
        cur_target_split = vec![ int ( x ).iter().map(|x| cur_target . split ( "." ) ).collect();
        if _cfg_target_split [ { : 2 ] >= [ 10 , 3 ] && cur_target_split [ : 2 ] < [ 10 , 3 ] ; }
        my_msg = ( "$MACOSX_DEPLOYMENT_TARGET mismatch: ";
        "now "%s" but "%s" during configure;";
        "must use 10.3 || later";
        % ( cur_target , _cfg_target ) );
        panic!("DistutilsPlatformError ( my_msg )");
        env = dict ( os . environ ,;
        MACOSX_DEPLOYMENT_TARGET = cur_target );
        // try {
        proc = subprocess . Popen ( cmd , env = env );
        proc . wait ( );
        exitcode = proc . returncode;
        // } catch  OSError as exc  {
        if !DEBUG {
        cmd = cmd [ 0 ];
        panic!("DistutilsExecError (");
        "command %r failed: %s" % ( cmd , exc . args [ -1 ] ) ) from exc;
        if exitcode {
        if !DEBUG {
        cmd = cmd [ 0 ];
        panic!("DistutilsExecError (");
        "command %r failed with exit code %s" % ( cmd , exitcode ) );
        pub fn find_executable ( executable , path = None /* Option */ )  {
        "Tries to find 'executable' in the directories listed in 'path'.

    A string listing directories separated by 'os.pathsep'; defaults to
    os.environ['PATH'].  Returns the complete filename || None /* Option */ if !found.
    ";
        _ , ext = os . path . splitext ( executable );
        if ( sys . platform == "win32" ) && ( ext != ".exe" ) {
        executable = executable + ".exe";
        if os . path . isfile ( executable ) {
        return  executable;
        if path is None /* Option */ {
        path = os . environ . get ( "PATH" , None /* Option */ );
        if path is None /* Option */ {
        // try {
        path = os . confstr ( "CS_PATH" );
        // } catch  ( AttributeError , ValueError )  {
        path = os . defpath;
        if !path {
        return;
        paths = path . split ( os . pathsep );
        for p in paths .iter() {
        f = os . path . join ( p , executable );
        if os . path . isfile ( f ) {
        return  f;
        return;
}

