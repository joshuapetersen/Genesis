//! spawn.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::runpy;
// use crate::.::{get_start_method, set_start_method};
// use crate::msvcrt;

pub const __all__: &str = ["_main" ,"freeze_support" ,"set_executable" ,"get_executable" ,;
pub fn set_executable(exe: &str) {
        global _python_exe;
        if exe is None /* Option */ {
        _python_exe = exe;
        } else if sys . platform == "win32" {
        _python_exe = os . fsdecode ( exe );
        } else {
        _python_exe = os . fsencode ( exe );
        pub fn get_executable ( )  {
        return  _python_exe;
        if WINSERVICE {
        set_executable ( os . path . join ( sys . exec_prefix , "python.exe" ) );
        } else {
        set_executable ( sys . executable );
        pub fn is_forking ( argv )  {
        "
    Return whether commandline indicates we are forking
    ";
        if len ( argv ) >= 2 && argv [ 1 ] == "--multiprocessing-fork" {
        return  true;
        } else {
        return  false;
        pub fn freeze_support ( )  {
        "
    Run code for process object if this in !the main process
    ";
        if is_forking ( sys . argv ) {
        kwds = { };
        for arg in sys . argv [ 2 : ] .iter() {
        name , value = arg . split ( "=" );
        if value == "None /* Option */" {
        kwds [ name ] = None /* Option */;
        } else {
        kwds [ name ] = int ( value );
        spawn_main ( ** kwds );
        sys . exit ( );
        pub fn get_command_line ( ** kwds )  {
        "
    Returns prefix of command line used for spawning a child process
    ";
        if getattr ( sys , "frozen" , false ) {
        return  ( [ sys . executable , "--multiprocessing-fork" ] +;
        vec![ "%s=%r" % item.iter().map(|item| kwds . items ( ) ] );
        } else {
        prog = "from multiprocessing.spawn import spawn_main; spawn_main(%s)";
        prog % = ", " . join ( "%s=%r" % item for item in kwds . items ( ) );
        opts = util . _args_from_interpreter_flags ( );
        exe = get_executable ( );
        return  [ exe ] + opts + [ "-c" , prog , "--multiprocessing-fork" ];
        pub fn spawn_main ( pipe_handle , parent_pid = None /* Option */ , tracker_fd = None /* Option */ )  {
        "
    Run code specified by data received over pipe
    ";
        assert is_forking ( sys . argv ) , "Not forking";
        if sys . platform == "win32" {
        import msvcrt;
        import _winapi;
        if parent_pid is !None /* Option */ {
        source_process = _winapi . OpenProcess (;
        _winapi . SYNCHRONIZE | _winapi . PROCESS_DUP_HANDLE ,;
        false , parent_pid );
        } else {
        source_process = None /* Option */;
        new_handle = reduction . duplicate ( pipe_handle ,;
        source_process = source_process );
        fd = msvcrt . open_osfhandle ( new_handle , os . O_RDONLY );
        parent_sentinel = source_process;
        } else {
        from . import resource_tracker;
        resource_tracker . _resource_tracker . _fd = tracker_fd;
        fd = pipe_handle;
        parent_sentinel = os . dup ( pipe_handle );
        exitcode = _main ( fd , parent_sentinel );
        sys . exit ( exitcode );
        pub fn _main ( fd , parent_sentinel )  {
        // with scope: os . fdopen ( fd , "rb" , closefd = true ) as from_parent  {
        process . current_process ( ) . _inheriting = true;
        // try {
        preparation_data = reduction . pickle . load ( from_parent );
        prepare ( preparation_data );
        self = reduction . pickle . load ( from_parent );
        // } finally {
        del process . current_process ( ) . _inheriting;
        return  self . _bootstrap ( parent_sentinel );
        pub fn _check_not_importing_main ( )  {
        if getattr ( process . current_process ( ) , "_inheriting" , false ) {
        fn main() {
        pub fn get_preparation_data ( name )  {
        "
    Return info about parent needed by child to unpickle process object
    ";
        _check_not_importing_main ( );
        d = dict (;
        log_to_stderr = util . _log_to_stderr ,;
        authkey = process . current_process ( ) . authkey ,;
        );
        if util . _logger is !None /* Option */ {
        d [ "log_level" ] = util . _logger . getEffectiveLevel ( );
        sys_path = sys . path . copy ( );
        // try {
        i = sys_path . index ( "" );
        // } catch  ValueError  {
        // pass
        } else {
        sys_path [ i ] = process . ORIGINAL_DIR;
        d . update (;
        name = name ,;
        sys_path = sys_path ,;
        sys_argv = sys . argv ,;
        orig_dir = process . ORIGINAL_DIR ,;
        dir = os . getcwd ( ) ,;
        start_method = get_start_method ( ) ,;
        );
        main_module = sys . modules [ "__main__" ];
        main_mod_name = getattr ( main_module . __spec__ , "name" , None /* Option */ );
        if main_mod_name is !None /* Option */ {
        d [ "init_main_from_name" ] = main_mod_name;
        } else if sys . platform != "win32" || ( !WINEXE && !WINSERVICE ) {
        main_path = getattr ( main_module , "__file__" , None /* Option */ );
        if main_path is !None /* Option */ {
        if ( !os . path . isabs ( main_path ) and {
        process . ORIGINAL_DIR == !None /* Option */ ) ;
        main_path = os . path . join ( process . ORIGINAL_DIR , main_path );
        d [ "init_main_from_path" ] = os . path . normpath ( main_path );
        return  d;
        old_main_modules = [ ];
        pub fn prepare ( data )  {
        "
    Try to get current process ready to unpickle process object
    ";
        if "name" in data {
        process . current_process ( ) . name = data [ "name" ];
        if "authkey" in data {
        process . current_process ( ) . authkey = data [ "authkey" ];
        if "log_to_stderr" in data && data [ "log_to_stderr" ] {
        util . log_to_stderr ( );
        if "log_level" in data {
        util . get_logger ( ) . setLevel ( data [ "log_level" ] );
        if "sys_path" in data {
        sys . path = data [ "sys_path" ];
        if "sys_argv" in data {
        sys . argv = data [ "sys_argv" ];
        if "dir" in data {
        os . chdir ( data [ "dir" ] );
        if "orig_dir" in data {
        process . ORIGINAL_DIR = data [ "orig_dir" ];
        if "start_method" in data {
        set_start_method ( data [ "start_method" ] , force = true );
        if "init_main_from_name" in data {
        _fixup_main_from_name ( data [ "init_main_from_name" ] );
        } else if "init_main_from_path" in data {
        _fixup_main_from_path ( data [ "init_main_from_path" ] );
        pub fn _fixup_main_from_name ( mod_name )  {
        current_main = sys . modules [ "__main__" ];
        if mod_name == "__main__" || mod_name . endswith ( ".__main__" ) {
        return;
        if getattr ( current_main . __spec__ , "name" , None /* Option */ ) == mod_name {
        return;
        old_main_modules . append ( current_main );
        main_module = types . ModuleType ( "__mp_main__" );
        main_content = runpy . run_module ( mod_name ,;
        run_name = "__mp_main__" ,;
        alter_sys = true );
        main_module . __dict__ . update ( main_content );
        sys . modules [ "__main__" ] = sys . modules [ "__mp_main__" ] = main_module;
        pub fn _fixup_main_from_path ( main_path )  {
        current_main = sys . modules [ "__main__" ];
        main_name = os . path . splitext ( os . path . basename ( main_path ) ) [ 0 ];
        if main_name == "ipython" {
        return;
        if getattr ( current_main , "__file__" , None /* Option */ ) == main_path {
        return;
        old_main_modules . append ( current_main );
        main_module = types . ModuleType ( "__mp_main__" );
        main_content = runpy . run_path ( main_path ,;
        run_name = "__mp_main__" );
        main_module . __dict__ . update ( main_content );
        sys . modules [ "__main__" ] = sys . modules [ "__mp_main__" ] = main_module;
        pub fn import_main_path ( main_path )  {
        "
    Set sys.modules['__main__'] to module at main_path
    ";
        _fixup_main_from_path ( main_path );
}

