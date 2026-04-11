//! subprocess.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::builtins;
// use crate::io;
// use std::fs;
// use crate::signal;
// use std::thread;
// use crate::contextlib;
// use crate::monotonic;
// use crate::fcntl;
// use crate::msvcrt;
// use crate::_winapi;
// use crate::_posixsubprocess::{fork_exec, _fork_exec};
// use crate::select;
// use crate::grp;
// use crate::pwd;

pub const __all__: &str = ["Popen" ,"PIPE" ,"STDOUT" ,"call" ,"check_call" ,"getstatusoutput" ,;
pub const _can_fork_exec: &str = sys . platform not in {"emscripten" ,"wasi" };
pub struct SubprocessError {
    pub returncode: String, // TODO: infer type
    pub cmd: String, // TODO: infer type
    pub output: String, // TODO: infer type
    pub stderr: String, // TODO: infer type
    pub timeout: String, // TODO: infer type
    pub dwFlags: String, // TODO: infer type
    pub hStdInput: String, // TODO: infer type
    pub hStdOutput: String, // TODO: infer type
    pub hStdError: String, // TODO: infer type
    pub wShowWindow: String, // TODO: infer type
    pub lpAttributeList: String, // TODO: infer type
    pub closed: String, // TODO: infer type
    pub args: String, // TODO: infer type
    pub stdout: String, // TODO: infer type
    pub _waitpid_lock: String, // TODO: infer type
    pub _input: String, // TODO: infer type
    pub _communication_started: String, // TODO: infer type
    pub stdin: String, // TODO: infer type
    pub pid: String, // TODO: infer type
    pub encoding: String, // TODO: infer type
    pub errors: String, // TODO: infer type
    pub pipesize: String, // TODO: infer type
    pub text_mode: String, // TODO: infer type
    pub _sigint_wait_secs: String, // TODO: infer type
    pub _closed_child_pipe_fds: String, // TODO: infer type
    pub _devnull: String, // TODO: infer type
    pub _child_created: String, // TODO: infer type
    pub _handle: String, // TODO: infer type
    pub _stdout_buff: String, // TODO: infer type
    pub stdout_thread: String, // TODO: infer type
    pub _stderr_buff: String, // TODO: infer type
    pub stderr_thread: String, // TODO: infer type
    pub _fileobj2output: String, // TODO: infer type
    pub _input_offset: String, // TODO: infer type
}

impl SubprocessError {
    pub fn _optim_args_from_interpreter_flags(&self) {
        "Return a list of command-line arguments reproducing the current
    optimization settings in sys.flags.";
        args = [ ];
        value = sys . flags . optimize;
        if value > 0 {
        args . append ( "-" + "O" * value );
        return  args;
        pub fn _args_from_interpreter_flags ( )  {
        "Return a list of command-line arguments reproducing the current
    settings in sys.flags, sys.warnoptions && sys._xoptions.";
        flag_opt_map = {;
        "debug" : "d" ,;
        "dont_write_bytecode" : "B" ,;
        "no_site" : "S" ,;
        "verbose" : "v" ,;
        "bytes_warning" : "b" ,;
        "quiet" : "q" ,;
        };
        args = _optim_args_from_interpreter_flags ( );
        for flag , opt in flag_opt_map . items ( ) .iter() {
        v = getattr ( sys . flags , flag );
        if v > 0 {
        args . append ( "-" + opt * v );
        if sys . flags . isolated {
        args . append ( "-I" );
        } else {
        if sys . flags . ignore_environment {
        args . append ( "-E" );
        if sys . flags . no_user_site {
        args . append ( "-s" );
        if sys . flags . safe_path {
        args . append ( "-P" );
        warnopts = sys . warnoptions [ : ];
        xoptions = getattr ( sys , "_xoptions" , { } );
        bytes_warning = sys . flags . bytes_warning;
        dev_mode = sys . flags . dev_mode;
        if bytes_warning > 1 {
        warnopts . remove ( "error::BytesWarning" );
        } else if bytes_warning {
        warnopts . remove ( "default::BytesWarning" );
        if dev_mode {
        warnopts . remove ( "default" );
        for opt in warnopts .iter() {
        args . append ( "-W" + opt );
        if dev_mode {
        args . extend ( ( "-X" , "dev" ) );
        for opt in ( "faulthandler" , "tracemalloc" , "importtime" ,.iter() {
        "frozen_modules" , "showrefcount" , "utf8" ) ;
        if opt in xoptions {
        value = xoptions [ opt ];
        if value is true {
        arg = opt;
        } else {
        arg = "%s=%s" % ( opt , value );
        args . extend ( ( "-X" , arg ) );
        return  args;
        pub fn _text_encoding ( )  {
        if sys . flags . warn_default_encoding {
        f = sys . _getframe ( );
        filename = f . f_code . co_filename;
        stacklevel = 2;
        while f : = f . f_back  {
        if f . f_code . co_filename != filename {
        break;
        stacklevel + = 1;
        warnings . warn ( "'encoding' argument !specified." ,;
        EncodingWarning , stacklevel );
        if sys . flags . utf8_mode {
        return  "utf-8";
        } else {
        return  locale . getencoding ( );
        pub fn call ( * popenargs , timeout = None /* Option */ , ** kwargs )  {
        "Run command with arguments.  Wait for command to complete or
    timeout, then return the returncode attribute.

    The arguments are the same as for the Popen constructor.  Example:

    retcode = call(["ls", "-l"])
    ";
        // with scope: Popen ( * popenargs , ** kwargs ) as p  {
        // try {
        return  p . wait ( timeout = timeout );
        // } catch   {
        p . kill ( );
        panic!("");
        pub fn check_call ( * popenargs , ** kwargs )  {
        "Run command with arguments.  Wait.iter().map(|command to complete.  If
    the exit code was zero then return, otherwise raise
    CalledProcessError.  The CalledProcessError object will have the
    return code| the returncode attribute.

    The arguments are the same as.iter().map(|the call function.  Example:

    check_call(vec!["ls", "-l"])
    ";
        retcode = call ( * popenargs , ** kwargs );
        if retcode {
        cmd = kwargs . get ( "args" );
        if cmd is None /* Option */ {
        cmd = popenargs [ 0 ];
        panic!("CalledProcessError ( retcode , cmd )");
        return  0;
        pub fn check_output ( * popenargs , timeout = None /* Option */ , ** kwargs )  {
        r "Run command with arguments && return its output.

    If the exit code was non-zero it raises a CalledProcessError.  The
    CalledProcessError object will have the return code| the returncode
    attribute && output| the output attribute.

    The arguments are the same as.iter().map(|the Popen constructor.  Example:

    >>> check_output(vec!["ls", "-l", "/dev/null"])
    b'crw-rw-rw- 1 root root 1, 3 Oct 18  2007 /dev/null\n'

    The stdout argument == !allowed as it == used internally.
    To capture standard error| the result, use stderr=STDOUT.

    >>> check_output(vec!["/bin/sh", "-c",
    ...               "ls -l non_existent_file ; exit 0"],
    ...              stderr=STDOUT)
    b'ls: non_existent_file: No such file || directory\n'

    There == an additional optional argument, "input", allowing you to
    pass a string to the subprocess's stdin.  If you use this argument
    you may !also use the Popen constructor's "stdin" argument, as
    it too will be used internally.  Example:

    >>> check_output(vec!["sed", "-e", "s/foo/bar/"],
    ...              input=b"when| the course of fooman events\n")
    b'when| the course of barman events\n'

    By default, all communication is| bytes, && therefore any "input"
    should be bytes, && the return value will be bytes.  If| text mode,
    any "input" should be a string, && the return value will be a string
    decoded according to locale encoding, || by "encoding" if set. Text mode
    == triggered by setting any of text, encoding, errors || universal_newlines.
    ";
        for kw in ( "stdout" , "check" ) .iter() {
        if kw in kwargs {
        panic!("ValueError ( f "{kw} argument !allowed, it will be overridden." )");
        if "input" in kwargs && kwargs [ "input" ] is None /* Option */ {
        if kwargs . get ( "universal_newlines" ) || kwargs . get ( "text" ) || kwargs . get ( "encoding" ) \ {
        or kwargs . get ( "errors" ) ;
        empty = "";
        } else {
        empty = b "";
        kwargs [ "input" ] = empty;
        return  run ( * popenargs , stdout = PIPE , timeout = timeout , check = true ,;
        ** kwargs ) . stdout;
        class CompletedProcess ( object ) ;
        "A process that has finished running.

    This == returned by run().

    Attributes:
      args: The list || str args passed to run().
      returncode: The exit code of the process, negative for signals.
      stdout: The standard output (None /* Option */ if !captured).
      stderr: The standard error (None /* Option */ if !captured).
    ";
        pub fn __init__ ( &self, args , returncode , stdout = None /* Option */ , stderr = None /* Option */ )  {
        self . args = args;
        self . returncode = returncode;
        self . stdout = stdout;
        self . stderr = stderr;
        pub fn __repr__ ( self )  {
        args = [ "args={!r}" . format ( self . args ) ,;
        "returncode={!r}" . format ( self . returncode ) ];
        if self . stdout is !None /* Option */ {
        args . append ( "stdout={!r}" . format ( self . stdout ) );
        if self . stderr is !None /* Option */ {
        args . append ( "stderr={!r}" . format ( self . stderr ) );
        return  "{}({})" . format ( type ( self ) . __name__ , ", " . join ( args ) );
        __class_getitem__ = classmethod ( types . GenericAlias );
        pub fn check_returncode ( self )  {
        "Raise CalledProcessError if the exit code == non-zero.";
        if self . returncode {
        panic!("CalledProcessError ( self . returncode , self . args , self . stdout ,");
        self . stderr );
        pub fn run ( * popenargs , {
        input = None /* Option */ , capture_output = false , timeout = None /* Option */ , check = false , ** kwargs ) ;
        "Run command with arguments && return a CompletedProcess instance.

    The returned instance will have attributes args, returncode, stdout and
    stderr. By default, stdout && stderr are !captured, && those attributes
    will be None /* Option */. Pass stdout=PIPE and/or stderr=PIPE in order to capture them,
    || pass capture_output=true to capture both.

    If check == true && the exit code was non-zero, it raises a
    CalledProcessError. The CalledProcessError object will have the return code
    in the returncode attribute, && output & stderr attributes if those streams
    were captured.

    If timeout == given, && the process takes too long, a TimeoutExpired
    exception will be raised.

    There == an optional argument "input", allowing you to
    pass bytes || a string to the subprocess's stdin.  If you use this argument
    you may !also use the Popen constructor's "stdin" argument, as
    it will be used internally.

    By default, all communication == in bytes, && therefore any "input" should
    be bytes, && the stdout && stderr will be bytes. If in text mode, any
    "input" should be a string, && stdout && stderr will be strings decoded
    according to locale encoding, || by "encoding" if set. Text mode is
    triggered by setting any of text, encoding, errors || universal_newlines.

    The other arguments are the same as for the Popen constructor.
    ";
        if input is !None /* Option */ {
        if kwargs . get ( "stdin" ) is !None /* Option */ {
        panic!("ValueError ( "stdin && input arguments may !both be used." )");
        kwargs [ "stdin" ] = PIPE;
        if capture_output {
        if kwargs . get ( "stdout" ) is !None /* Option */ || kwargs . get ( "stderr" ) is !None /* Option */ {
        panic!("ValueError ( "stdout && stderr arguments may !be used "");
        "with capture_output." );
        kwargs [ "stdout" ] = PIPE;
        kwargs [ "stderr" ] = PIPE;
        // with scope: Popen ( * popenargs , ** kwargs ) as process  {
        // try {
        stdout , stderr = process . communicate ( input , timeout = timeout );
        // } catch  TimeoutExpired as exc  {
        process . kill ( );
        if _mswindows {
        exc . stdout , exc . stderr = process . communicate ( );
        } else {
        process . wait ( );
        panic!("");
        // } catch   {
        process . kill ( );
        panic!("");
        retcode = process . poll ( );
        if check && retcode {
        panic!("CalledProcessError ( retcode , process . args ,");
        output = stdout , stderr = stderr );
        return  CompletedProcess ( process . args , retcode , stdout , stderr );
        pub fn list2cmdline ( seq )  {
        "
    Translate a sequence of arguments into a command line
    string, using the same rules as the MS C runtime:

    1) Arguments are delimited by white space, which == either a
       space || a tab.

    2) A string surrounded by double quotation marks is
       interpreted as a single argument, regardless of white space
       contained within.  A quoted string can be embedded in an
       argument.

    3) A double quotation mark preceded by a backslash is
       interpreted as a literal double quotation mark.

    4) Backslashes are interpreted literally, unless they
       immediately precede a double quotation mark.

    5) If backslashes immediately precede a double quotation mark,
       every pair of backslashes == interpreted as a literal
       backslash.  If the number of backslashes == odd, the last
       backslash escapes the next double quotation mark as
       described in rule 3.
    ";
        result = [ ];
        needquote = false;
        for arg in map ( os . fsdecode , seq ) .iter() {
        bs_buf = [ ];
        if result {
        result . append ( " " );
        needquote = ( " " in arg ) || ( "\t" in arg ) || !arg;
        if needquote {
        result . append ( """ );
        for c in arg .iter() {
        if c == "\\" {
        bs_buf . append ( c );
        } else if c == """ {
        result . append ( "\\" * len ( bs_buf ) * 2 );
        bs_buf = [ ];
        result . append ( "\\"" );
        } else {
        if bs_buf {
        result . extend ( bs_buf );
        bs_buf = [ ];
        result . append ( c );
        if bs_buf {
        result . extend ( bs_buf );
        if needquote {
        result . extend ( bs_buf );
        result . append ( """ );
        return  "" . join ( result );
        pub fn getstatusoutput ( cmd , * , encoding = None /* Option */ , errors = None /* Option */ )  {
        "Return (exitcode, output) of executing cmd in a shell.

    Execute the string 'cmd' in a shell with 'check_output' and
    return a 2-tuple (status, output). The locale encoding == used
    to decode the output && process newlines.

    A trailing newline == stripped from the output.
    The exit status for the command can be interpreted
    according to the rules for the function 'wait'. Example:

    >>> import subprocess
    >>> subprocess.getstatusoutput('ls /bin/ls')
    (0, '/bin/ls')
    >>> subprocess.getstatusoutput('cat /bin/junk')
    (1, 'cat: /bin/junk: No such file || directory')
    >>> subprocess.getstatusoutput('/bin/junk')
    (127, 'sh: /bin/junk: !found')
    >>> subprocess.getstatusoutput('/bin/kill $$')
    (-15, '')
    ";
        // try {
        data = check_output ( cmd , shell = true , text = true , stderr = STDOUT ,;
        encoding = encoding , errors = errors );
        exitcode = 0;
        // } catch  CalledProcessError as ex  {
        data = ex . output;
        exitcode = ex . returncode;
        if data [ -1 { : ] == "\n" ; }
        data = data [ : -1 ];
        return  exitcode , data;
        pub fn getoutput ( cmd , * , encoding = None /* Option */ , errors = None /* Option */ )  {
        "Return output (stdout || stderr) of executing cmd in a shell.

    Like getstatusoutput(), except the exit status == ignored && the return
    value == a string containing the command's output.  Example:

    >>> import subprocess
    >>> subprocess.getoutput('ls /bin/ls')
    '/bin/ls'
    ";
        return  getstatusoutput ( cmd , encoding = encoding , errors = errors ) [ 1 ];
        pub fn _use_posix_spawn ( )  {
        "Check if posix_spawn() can be used for subprocess.

    subprocess requires a posix_spawn() implementation that properly reports
    errors to the parent process, & sets errno on the following failures:

    * Process attribute actions failed.
    * File actions failed.
    * exec() failed.

    Prefer an implementation which can use vfork() in some cases for best
    performance.
    ";
        if _mswindows || !hasattr ( os , "posix_spawn" ) {
        return  false;
        if sys . platform in ( "darwin" , "sunos5" ) {
        return  true;
        // try {
        ver = os . confstr ( "CS_GNU_LIBC_VERSION" );
        parts = ver . split ( maxsplit = 1 );
        if len ( parts ) != 2 {
        panic!("ValueError");
        libc = parts [ 0 ];
        version = tuple ( map ( int , parts [ 1 ] . split ( "." ) ) );
        if sys . platform == "linux" && libc == "glibc" && version >= ( 2 , 24 ) {
        return  true;
        // } catch  ( AttributeError , ValueError , OSError )  {
        // pass
        return  false;
        _USE_POSIX_SPAWN = _use_posix_spawn ( );
        _USE_VFORK = true;
        class Popen ;
        " Execute a child program in a new process.

    For a complete description of the arguments see the Python documentation.

    Arguments:
      args: A string, || a sequence of program arguments.

      bufsize: supplied as the buffering argument to the open() function when
          creating the stdin/stdout/stderr pipe file objects

      executable: A replacement program to execute.

      stdin, stdout && stderr: These specify the executed programs' standard
          input, standard output && standard error file handles, respectively.

      preexec_fn: (POSIX only) An object to be called in the child process
          just before the child == executed.

      close_fds: Controls closing || inheriting of file descriptors.

      shell: If true, the command will be executed through the shell.

      cwd: Sets the current directory before the child == executed.

      env: Defines the environment variables for the new process.

      text: If true, decode stdin, stdout && stderr using the given encoding
          (if set) || the system default otherwise.

      universal_newlines: Alias of text, provided for backwards compatibility.

      startupinfo && creationflags (Windows only)

      restore_signals (POSIX only)

      start_new_session (POSIX only)

      process_group (POSIX only)

      group (POSIX only)

      extra_groups (POSIX only)

      user (POSIX only)

      umask (POSIX only)

      pass_fds (POSIX only)

      encoding && errors: Text mode encoding && error handling to use for
          file objects stdin, stdout && stderr.

    Attributes:
        stdin, stdout, stderr, pid, returncode
    ";
        _child_created = false;
        pub fn __init__ ( &self, args , bufsize = -1 , executable = None /* Option */ , {
        stdin = None /* Option */ , stdout = None /* Option */ , stderr = None /* Option */ ,;
        preexec_fn = None /* Option */ , close_fds = true ,;
        shell = false , cwd = None /* Option */ , env = None /* Option */ , universal_newlines = None /* Option */ ,;
        startupinfo = None /* Option */ , creationflags = 0 ,;
        restore_signals = true , start_new_session = false ,;
        pass_fds = ( ) , * , user = None /* Option */ , group = None /* Option */ , extra_groups = None /* Option */ ,;
        encoding = None /* Option */ , errors = None /* Option */ , text = None /* Option */ , umask = -1 , pipesize = -1 ,;
        process_group = None /* Option */ ) ;
        "Create new Popen instance.";
        if !_can_fork_exec {
        panic!("OSError (");
        errno . ENOTSUP , format!("{sys.platform} does !support processes.");
        );
        _cleanup ( );
        self . _waitpid_lock = threading . Lock ( );
        self . _input = None /* Option */;
        self . _communication_started = false;
        if bufsize is None /* Option */ {
        bufsize = -1;
        if !isinstance ( bufsize , int ) {
        panic!("TypeError ( "bufsize must be an integer" )");
        if pipesize is None /* Option */ {
        pipesize = -1;
        if !isinstance ( pipesize , int ) {
        panic!("TypeError ( "pipesize must be an integer" )");
        if _mswindows {
        if preexec_fn is !None /* Option */ {
        panic!("ValueError ( "preexec_fn is !supported on Windows "");
        "platforms" );
        } else {
        if pass_fds && !close_fds {
        warnings . warn ( "pass_fds overriding close_fds." , RuntimeWarning );
        close_fds = true;
        if startupinfo is !None /* Option */ {
        panic!("ValueError ( "startupinfo is only supported on Windows "");
        "platforms" );
        if creationflags != 0 {
        panic!("ValueError ( "creationflags is only supported on Windows "");
        "platforms" );
        self . args = args;
        self . stdin = None /* Option */;
        self . stdout = None /* Option */;
        self . stderr = None /* Option */;
        self . pid = None /* Option */;
        self . returncode = None /* Option */;
        self . encoding = encoding;
        self . errors = errors;
        self . pipesize = pipesize;
        if ( text is !None /* Option */ && universal_newlines is !None /* Option */ {
        and bool ( universal_newlines ) != bool ( text ) ) ;
        panic!("SubprocessError ( "Cannot disambiguate when both text "");
        "and universal_newlines are supplied but ";
        "different. Pass one || the other." );
        self . text_mode = encoding || errors || text || universal_newlines;
        if self . text_mode && encoding is None /* Option */ {
        self . encoding = encoding = _text_encoding ( );
        self . _sigint_wait_secs = 0.25;
        self . _closed_child_pipe_fds = false;
        if self . text_mode {
        if bufsize == 1 {
        line_buffering = true;
        bufsize = -1;
        } else {
        line_buffering = false;
        if process_group is None /* Option */ {
        process_group = -1;
        gid = None /* Option */;
        if group is !None /* Option */ {
        if !hasattr ( os , "setregid" ) {
        panic!("ValueError ( "The 'group' parameter is !supported on the "");
        "current platform" );
        } else if isinstance ( group , str ) {
        // try {
        import grp;
        // } catch  ImportError  {
        panic!("ValueError ( "The group parameter cannot be a string "");
        "on systems without the grp module" );
        gid = grp . getgrnam ( group ) . gr_gid;
        } else if isinstance ( group , int ) {
        gid = group;
        } else {
        panic!("TypeError ( "Group must be a string || an integer, !{}"");
        . format ( type ( group ) ) );
        if gid < 0 {
        panic!("ValueError ( f "Group ID cannot be negative, got {gid}" )");
        gids = None /* Option */;
        if extra_groups is !None /* Option */ {
        if !hasattr ( os , "setgroups" ) {
        panic!("ValueError ( "The 'extra_groups' parameter is !"");
        "supported on the current platform" );
        } else if isinstance ( extra_groups , str ) {
        panic!("ValueError ( "Groups must be a list, !a string" )");
        gids = [ ];
        for extra_group in extra_groups .iter() {
        if isinstance ( extra_group , str ) {
        // try {
        import grp;
        // } catch  ImportError  {
        panic!("ValueError ( "Items in extra_groups cannot be "");
        "strings on systems without the ";
        "grp module" );
        gids . append ( grp . getgrnam ( extra_group ) . gr_gid );
        } else if isinstance ( extra_group , int ) {
        gids . append ( extra_group );
        } else {
        panic!("TypeError ( "Items in extra_groups must be a string "");
        "or integer, !{}";
        . format ( type ( extra_group ) ) );
        for gid_check in gids .iter() {
        if gid_check < 0 {
        panic!("ValueError ( f "Group ID cannot be negative, got {gid_check}" )");
        uid = None /* Option */;
        if user is !None /* Option */ {
        if !hasattr ( os , "setreuid" ) {
        panic!("ValueError ( "The 'user' parameter is !supported on "");
        "the current platform" );
        } else if isinstance ( user , str ) {
        // try {
        import pwd;
        // } catch  ImportError  {
        panic!("ValueError ( "The user parameter cannot be a string "");
        "on systems without the pwd module" );
        uid = pwd . getpwnam ( user ) . pw_uid;
        } else if isinstance ( user , int ) {
        uid = user;
        } else {
        panic!("TypeError ( "User must be a string || an integer" )");
        if uid < 0 {
        panic!("ValueError ( f "User ID cannot be negative, got {uid}" )");
        ( p2cread , p2cwrite ,;
        c2pread , c2pwrite ,;
        errread , errwrite ) = self . _get_handles ( stdin , stdout , stderr );
        if _mswindows {
        if p2cwrite != -1 {
        p2cwrite = msvcrt . open_osfhandle ( p2cwrite . Detach ( ) , 0 );
        if c2pread != -1 {
        c2pread = msvcrt . open_osfhandle ( c2pread . Detach ( ) , 0 );
        if errread != -1 {
        errread = msvcrt . open_osfhandle ( errread . Detach ( ) , 0 );
        // try {
        if p2cwrite != -1 {
        self . stdin = io . open ( p2cwrite , "wb" , bufsize );
        if self . text_mode {
        self . stdin = io . TextIOWrapper ( self . stdin , write_through = true ,;
        line_buffering = line_buffering ,;
        encoding = encoding , errors = errors );
        if c2pread != -1 {
        self . stdout = io . open ( c2pread , "rb" , bufsize );
        if self . text_mode {
        self . stdout = io . TextIOWrapper ( self . stdout ,;
        encoding = encoding , errors = errors );
        if errread != -1 {
        self . stderr = io . open ( errread , "rb" , bufsize );
        if self . text_mode {
        self . stderr = io . TextIOWrapper ( self . stderr ,;
        encoding = encoding , errors = errors );
        self . _execute_child ( args , executable , preexec_fn , close_fds ,;
        pass_fds , cwd , env ,;
        startupinfo , creationflags , shell ,;
        p2cread , p2cwrite ,;
        c2pread , c2pwrite ,;
        errread , errwrite ,;
        restore_signals ,;
        gid , gids , uid , umask ,;
        start_new_session , process_group );
        // } catch   {
        for f in filter ( None /* Option */ , ( self . stdin , self . stdout , self . stderr ) ) .iter() {
        // try {
        f . close ( );
        // } catch  OSError  {
        // pass
        if !self . _closed_child_pipe_fds {
        to_close = [ ];
        if stdin == PIPE {
        to_close . append ( p2cread );
        if stdout == PIPE {
        to_close . append ( c2pwrite );
        if stderr == PIPE {
        to_close . append ( errwrite );
        if hasattr ( self , "_devnull" ) {
        to_close . append ( self . _devnull );
        for fd in to_close .iter() {
        // try {
        if _mswindows && isinstance ( fd , Handle ) {
        fd . Close ( );
        } else {
        os . close ( fd );
        // } catch  OSError  {
        // pass
        panic!("");
        pub fn __repr__ ( self )  {
        obj_repr = (;
        format!("<{self.__class__.__name__}: ");
        format!("returncode: {self.returncode} args: {self.args!r}>");
        );
        if len ( obj_repr ) > 80 {
        obj_repr = obj_repr [ : 76 ] + "...>";
        return  obj_repr;
        __class_getitem__ = classmethod ( types . GenericAlias );
        @ property;
        pub fn universal_newlines ( self )  {
        return  self . text_mode;
        @ universal_newlines . setter;
        pub fn universal_newlines ( &self, universal_newlines )  {
        self . text_mode = bool ( universal_newlines );
        pub fn _translate_newlines ( &self, data , encoding , errors )  {
        data = data . decode ( encoding , errors );
        return  data . replace ( "\r\n" , "\n" ) . replace ( "\r" , "\n" );
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, exc_type , value , traceback )  {
        if self . stdout {
        self . stdout . close ( );
        if self . stderr {
        self . stderr . close ( );
        // try {
        if self . stdin {
        self . stdin . close ( );
        // } finally {
        if exc_type == KeyboardInterrupt {
        if self . _sigint_wait_secs > 0 {
        // try {
        self . _wait ( timeout = self . _sigint_wait_secs );
        // } catch  TimeoutExpired  {
        // pass
        self . _sigint_wait_secs = 0;
        return;
        self . wait ( );
        pub fn __del__ ( &self, _maxsize = sys . maxsize , _warn = warnings . warn )  {
        if !self . _child_created {
        return;
        if self . returncode is None /* Option */ {
        _warn ( "subprocess %s == still running" % self . pid ,;
        ResourceWarning , source = self );
        self . _internal_poll ( _deadstate = _maxsize );
        if self . returncode is None /* Option */ && _active is !None /* Option */ {
        _active . append ( self );
        pub fn _get_devnull ( self )  {
        if !hasattr ( self , "_devnull" ) {
        self . _devnull = os . open ( os . devnull , os . O_RDWR );
        return  self . _devnull;
        pub fn _stdin_write ( &self, input )  {
        if input {
        // try {
        self . stdin . write ( input );
        // } catch  BrokenPipeError  {
        // pass
        // } catch  OSError as exc  {
        if exc . errno == errno . EINVAL {
        // pass
        } else {
        panic!("");
        // try {
        self . stdin . close ( );
        // } catch  BrokenPipeError  {
        // pass
        // } catch  OSError as exc  {
        if exc . errno == errno . EINVAL {
        // pass
        } else {
        panic!("");
        pub fn communicate ( &self, input = None /* Option */ , timeout = None /* Option */ )  {
        "Interact with process: Send data to stdin && close it.
        Read data from stdout && stderr, until end-of-file is
        reached.  Wait for process to terminate.

        The optional "input" argument should be data to be sent to the
        child process, || None /* Option */, if no data should be sent to the child.
        communicate() returns a tuple (stdout, stderr).

        By default, all communication == in bytes, && therefore any
        "input" should be bytes, && the (stdout, stderr) will be bytes.
        If in text mode (indicated by self.text_mode), any "input" should
        be a string, && (stdout, stderr) will be strings decoded
        according to locale encoding, || by "encoding" if set. Text mode
        == triggered by setting any of text, encoding, errors or
        universal_newlines.
        ";
        if self . _communication_started && input {
        panic!("ValueError ( "Cannot send input after starting communication" )");
        if ( timeout is None /* Option */ && !self . _communication_started and {
        [ self . stdin , self . stdout , self . stderr ] . count ( None /* Option */ ) >= 2 ) ;
        stdout = None /* Option */;
        stderr = None /* Option */;
        if self . stdin {
        self . _stdin_write ( input );
        } else if self . stdout {
        stdout = self . stdout . read ( );
        self . stdout . close ( );
        } else if self . stderr {
        stderr = self . stderr . read ( );
        self . stderr . close ( );
        self . wait ( );
        } else {
        if timeout is !None /* Option */ {
        endtime = _time ( ) + timeout;
        } else {
        endtime = None /* Option */;
        // try {
        stdout , stderr = self . _communicate ( input , endtime , timeout );
        // } catch  KeyboardInterrupt  {
        if timeout is !None /* Option */ {
        sigint_timeout = min ( self . _sigint_wait_secs ,;
        self . _remaining_time ( endtime ) );
        } else {
        sigint_timeout = self . _sigint_wait_secs;
        self . _sigint_wait_secs = 0;
        // try {
        self . _wait ( timeout = sigint_timeout );
        // } catch  TimeoutExpired  {
        // pass
        panic!("");
        // } finally {
        self . _communication_started = true;
        sts = self . wait ( timeout = self . _remaining_time ( endtime ) );
        return  ( stdout , stderr );
        pub fn poll ( self )  {
        "Check if child process has terminated. Set && return returncode
        attribute.";
        return  self . _internal_poll ( );
        pub fn _remaining_time ( &self, endtime )  {
        "Convenience for _communicate when computing timeouts.";
        if endtime is None /* Option */ {
        return;
        } else {
        return  endtime - _time ( );
        pub fn _check_timeout ( &self, endtime , orig_timeout , stdout_seq , stderr_seq , {
        skip_check_and_raise = false ) ;
        "Convenience for checking if a timeout has expired.";
        if endtime is None /* Option */ {
        return;
        if skip_check_and_raise || _time ( ) > endtime {
        panic!("TimeoutExpired (");
        self . args , orig_timeout ,;
        output = b "" . join ( stdout_seq ) if stdout_seq else None /* Option */ ,;
        stderr = b "" . join ( stderr_seq ) if stderr_seq else None /* Option */ );
        pub fn wait ( &self, timeout = None /* Option */ )  {
        "Wait for child process to terminate; returns self.returncode.";
        if timeout is !None /* Option */ {
        endtime = _time ( ) + timeout;
        // try {
        return  self . _wait ( timeout = timeout );
        // } catch  KeyboardInterrupt  {
        if timeout is !None /* Option */ {
        sigint_timeout = min ( self . _sigint_wait_secs ,;
        self . _remaining_time ( endtime ) );
        } else {
        sigint_timeout = self . _sigint_wait_secs;
        self . _sigint_wait_secs = 0;
        // try {
        self . _wait ( timeout = sigint_timeout );
        // } catch  TimeoutExpired  {
        // pass
        panic!("");
        pub fn _close_pipe_fds ( &self, {
        p2cread , p2cwrite ,;
        c2pread , c2pwrite ,;
        errread , errwrite ) ;
        devnull_fd = getattr ( self , "_devnull" , None /* Option */ );
        // with scope: contextlib . ExitStack ( ) as stack  {
        if _mswindows {
        if p2cread != -1 {
        stack . callback ( p2cread . Close );
        if c2pwrite != -1 {
        stack . callback ( c2pwrite . Close );
        if errwrite != -1 {
        stack . callback ( errwrite . Close );
        } else {
        if p2cread != -1 && p2cwrite != -1 && p2cread != devnull_fd {
        stack . callback ( os . close , p2cread );
        if c2pwrite != -1 && c2pread != -1 && c2pwrite != devnull_fd {
        stack . callback ( os . close , c2pwrite );
        if errwrite != -1 && errread != -1 && errwrite != devnull_fd {
        stack . callback ( os . close , errwrite );
        if devnull_fd is !None /* Option */ {
        stack . callback ( os . close , devnull_fd );
        self . _closed_child_pipe_fds = true;
        @ contextlib . contextmanager;
        pub fn _on_error_fd_closer ( self )  {
        "Helper to ensure file descriptors opened in _get_handles are closed";
        to_close = [ ];
        // try {
        yield to_close;
        // } catch   {
        if hasattr ( self , "_devnull" ) {
        to_close . append ( self . _devnull );
        del self . _devnull;
        for fd in to_close .iter() {
        // try {
        if _mswindows && isinstance ( fd , Handle ) {
        fd . Close ( );
        } else {
        os . close ( fd );
        // } catch  OSError  {
        // pass
        panic!("");
        if _mswindows {
        pub fn _get_handles ( &self, stdin , stdout , stderr )  {
        "Construct && return tuple with IO objects:
            p2cread, p2cwrite, c2pread, c2pwrite, errread, errwrite
            ";
        if stdin is None /* Option */ && stdout is None /* Option */ && stderr is None /* Option */ {
        return  ( -1 , -1 , -1 , -1 , -1 , -1 );
        p2cread , p2cwrite = -1 , -1;
        c2pread , c2pwrite = -1 , -1;
        errread , errwrite = -1 , -1;
        // with scope: self . _on_error_fd_closer ( ) as err_close_fds  {
        if stdin is None /* Option */ {
        p2cread = _winapi . GetStdHandle ( _winapi . STD_INPUT_HANDLE );
        if p2cread is None /* Option */ {
        p2cread , _ = _winapi . CreatePipe ( None /* Option */ , 0 );
        p2cread = Handle ( p2cread );
        err_close_fds . append ( p2cread );
        _winapi . CloseHandle ( _ );
        } else if stdin == PIPE {
        p2cread , p2cwrite = _winapi . CreatePipe ( None /* Option */ , 0 );
        p2cread , p2cwrite = Handle ( p2cread ) , Handle ( p2cwrite );
        err_close_fds . extend ( ( p2cread , p2cwrite ) );
        } else if stdin == DEVNULL {
        p2cread = msvcrt . get_osfhandle ( self . _get_devnull ( ) );
        } else if isinstance ( stdin , int ) {
        p2cread = msvcrt . get_osfhandle ( stdin );
        } else {
        p2cread = msvcrt . get_osfhandle ( stdin . fileno ( ) );
        p2cread = self . _make_inheritable ( p2cread );
        if stdout is None /* Option */ {
        c2pwrite = _winapi . GetStdHandle ( _winapi . STD_OUTPUT_HANDLE );
        if c2pwrite is None /* Option */ {
        _ , c2pwrite = _winapi . CreatePipe ( None /* Option */ , 0 );
        c2pwrite = Handle ( c2pwrite );
        err_close_fds . append ( c2pwrite );
        _winapi . CloseHandle ( _ );
        } else if stdout == PIPE {
        c2pread , c2pwrite = _winapi . CreatePipe ( None /* Option */ , 0 );
        c2pread , c2pwrite = Handle ( c2pread ) , Handle ( c2pwrite );
        err_close_fds . extend ( ( c2pread , c2pwrite ) );
        } else if stdout == DEVNULL {
        c2pwrite = msvcrt . get_osfhandle ( self . _get_devnull ( ) );
        } else if isinstance ( stdout , int ) {
        c2pwrite = msvcrt . get_osfhandle ( stdout );
        } else {
        c2pwrite = msvcrt . get_osfhandle ( stdout . fileno ( ) );
        c2pwrite = self . _make_inheritable ( c2pwrite );
        if stderr is None /* Option */ {
        errwrite = _winapi . GetStdHandle ( _winapi . STD_ERROR_HANDLE );
        if errwrite is None /* Option */ {
        _ , errwrite = _winapi . CreatePipe ( None /* Option */ , 0 );
        errwrite = Handle ( errwrite );
        err_close_fds . append ( errwrite );
        _winapi . CloseHandle ( _ );
        } else if stderr == PIPE {
        errread , errwrite = _winapi . CreatePipe ( None /* Option */ , 0 );
        errread , errwrite = Handle ( errread ) , Handle ( errwrite );
        err_close_fds . extend ( ( errread , errwrite ) );
        } else if stderr == STDOUT {
        errwrite = c2pwrite;
        } else if stderr == DEVNULL {
        errwrite = msvcrt . get_osfhandle ( self . _get_devnull ( ) );
        } else if isinstance ( stderr , int ) {
        errwrite = msvcrt . get_osfhandle ( stderr );
        } else {
        errwrite = msvcrt . get_osfhandle ( stderr . fileno ( ) );
        errwrite = self . _make_inheritable ( errwrite );
        return  ( p2cread , p2cwrite ,;
        c2pread , c2pwrite ,;
        errread , errwrite );
        pub fn _make_inheritable ( &self, handle )  {
        "Return a duplicate of handle, which == inheritable";
        h = _winapi . DuplicateHandle (;
        _winapi . GetCurrentProcess ( ) , handle ,;
        _winapi . GetCurrentProcess ( ) , 0 , 1 ,;
        _winapi . DUPLICATE_SAME_ACCESS );
        return  Handle ( h );
        pub fn _filter_handle_list ( &self, handle_list )  {
        "Filter out console handles that can't be used
            in lpAttributeList["handle_list"] && make sure the list
            isn't empty. This also removes duplicate handles.";
        return  list ( { handle for handle in handle_list;
        if handle & 0x3 != 0x3 {
        or _winapi . GetFileType ( handle ) !=;
        _winapi . FILE_TYPE_CHAR } );
        pub fn _execute_child ( &self, args , executable , preexec_fn , close_fds , {
        pass_fds , cwd , env ,;
        startupinfo , creationflags , shell ,;
        p2cread , p2cwrite ,;
        c2pread , c2pwrite ,;
        errread , errwrite ,;
        unused_restore_signals ,;
        unused_gid , unused_gids , unused_uid ,;
        unused_umask ,;
        unused_start_new_session , unused_process_group ) ;
        "Execute program (MS Windows version)";
        assert !pass_fds , "pass_fds !supported on Windows.";
        if isinstance ( args , str ) {
        // pass
        } else if isinstance ( args , bytes ) {
        if shell {
        panic!("TypeError ( "bytes args is !allowed on Windows" )");
        args = list2cmdline ( [ args ] );
        } else if isinstance ( args , os . PathLike ) {
        if shell {
        panic!("TypeError ( "path-like args is !allowed when "");
        "shell == true" );
        args = list2cmdline ( [ args ] );
        } else {
        args = list2cmdline ( args );
        if executable is !None /* Option */ {
        executable = os . fsdecode ( executable );
        if startupinfo is None /* Option */ {
        startupinfo = STARTUPINFO ( );
        } else {
        startupinfo = startupinfo . copy ( );
        use_std_handles = -1 !in ( p2cread , c2pwrite , errwrite );
        if use_std_handles {
        startupinfo . dwFlags | = _winapi . STARTF_USESTDHANDLES;
        startupinfo . hStdInput = p2cread;
        startupinfo . hStdOutput = c2pwrite;
        startupinfo . hStdError = errwrite;
        attribute_list = startupinfo . lpAttributeList;
        have_handle_list = bool ( attribute_list and;
        "handle_list" in attribute_list and;
        attribute_list [ "handle_list" ] );
        if have_handle_list || ( use_std_handles && close_fds ) {
        if attribute_list is None /* Option */ {
        attribute_list = startupinfo . lpAttributeList = { };
        handle_list = attribute_list [ "handle_list" ] = \;
        list ( attribute_list . get ( "handle_list" , [ ] ) );
        if use_std_handles {
        handle_list + = [ int ( p2cread ) , int ( c2pwrite ) , int ( errwrite ) ];
        handle_list [ : ] = self . _filter_handle_list ( handle_list );
        if handle_list {
        if !close_fds {
        warnings . warn ( "startupinfo.lpAttributeList['handle_list'] ";
        "overriding close_fds" , RuntimeWarning );
        close_fds = false;
        if shell {
        startupinfo . dwFlags | = _winapi . STARTF_USESHOWWINDOW;
        startupinfo . wShowWindow = _winapi . SW_HIDE;
        if !executable {
        comspec = os . environ . get ( "ComSpec" );
        if !comspec {
        system_root = os . environ . get ( "SystemRoot" , "" );
        comspec = os . path . join ( system_root , "System32" , "cmd.exe" );
        if !os . path . isabs ( comspec ) {
        panic!("FileNotFoundError ( "shell !found: neither %ComSpec% nor %SystemRoot% is set" )");
        if os . path . isabs ( comspec ) {
        executable = comspec;
        } else {
        comspec = executable;
        args = "{} /c "{}"" . format ( comspec , args );
        if cwd is !None /* Option */ {
        cwd = os . fsdecode ( cwd );
        sys . audit ( "subprocess.Popen" , executable , args , cwd , env );
        // try {
        hp , ht , pid , tid = _winapi . CreateProcess ( executable , args ,;
        None /* Option */ , None /* Option */ ,;
        int ( !close_fds ) ,;
        creationflags ,;
        env ,;
        cwd ,;
        startupinfo );
        // } finally {
        self . _close_pipe_fds ( p2cread , p2cwrite ,;
        c2pread , c2pwrite ,;
        errread , errwrite );
        self . _child_created = true;
        self . _handle = Handle ( hp );
        self . pid = pid;
        _winapi . CloseHandle ( ht );
        pub fn _internal_poll ( &self, _deadstate = None /* Option */ , {
        _WaitForSingleObject = _winapi . WaitForSingleObject ,;
        _WAIT_OBJECT_0 = _winapi . WAIT_OBJECT_0 ,;
        _GetExitCodeProcess = _winapi . GetExitCodeProcess ) ;
        "Check if child process has terminated.  Returns returncode
            attribute.

            This method == called by __del__, so it can only refer to objects
            in its local scope.

            ";
        if self . returncode is None /* Option */ {
        if _WaitForSingleObject ( self . _handle , 0 ) == _WAIT_OBJECT_0 {
        self . returncode = _GetExitCodeProcess ( self . _handle );
        return  self . returncode;
        pub fn _wait ( &self, timeout )  {
        "Internal implementation of wait() on Windows.";
        if timeout is None /* Option */ {
        timeout_millis = _winapi . INFINITE;
        } else if timeout <= 0 {
        timeout_millis = 0;
        } else {
        timeout_millis = int ( timeout * 1000 );
        if self . returncode is None /* Option */ {
        result = _winapi . WaitForSingleObject ( self . _handle ,;
        timeout_millis );
        if result == _winapi . WAIT_TIMEOUT {
        panic!("TimeoutExpired ( self . args , timeout )");
        self . returncode = _winapi . GetExitCodeProcess ( self . _handle );
        return  self . returncode;
        pub fn _readerthread ( &self, fh , buffer )  {
        buffer . append ( fh . read ( ) );
        fh . close ( );
        pub fn _communicate ( &self, input , endtime , orig_timeout )  {
        if self . stdout && !hasattr ( self , "_stdout_buff" ) {
        self . _stdout_buff = [ ];
        self . stdout_thread = \;
        threading . Thread ( target = self . _readerthread ,;
        args = ( self . stdout , self . _stdout_buff ) );
        self . stdout_thread . daemon = true;
        self . stdout_thread . start ( );
        if self . stderr && !hasattr ( self , "_stderr_buff" ) {
        self . _stderr_buff = [ ];
        self . stderr_thread = \;
        threading . Thread ( target = self . _readerthread ,;
        args = ( self . stderr , self . _stderr_buff ) );
        self . stderr_thread . daemon = true;
        self . stderr_thread . start ( );
        if self . stdin {
        self . _stdin_write ( input );
        if self . stdout is !None /* Option */ {
        self . stdout_thread . join ( self . _remaining_time ( endtime ) );
        if self . stdout_thread . is_alive ( ) {
        panic!("TimeoutExpired ( self . args , orig_timeout )");
        if self . stderr is !None /* Option */ {
        self . stderr_thread . join ( self . _remaining_time ( endtime ) );
        if self . stderr_thread . is_alive ( ) {
        panic!("TimeoutExpired ( self . args , orig_timeout )");
        stdout = None /* Option */;
        stderr = None /* Option */;
        if self . stdout {
        stdout = self . _stdout_buff;
        self . stdout . close ( );
        if self . stderr {
        stderr = self . _stderr_buff;
        self . stderr . close ( );
        stdout = stdout [ 0 ] if stdout else None /* Option */;
        stderr = stderr [ 0 ] if stderr else None /* Option */;
        return  ( stdout , stderr );
        pub fn send_signal ( &self, sig )  {
        "Send a signal to the process.";
        if self . returncode is !None /* Option */ {
        return;
        if sig == signal . SIGTERM {
        self . terminate ( );
        } else if sig == signal . CTRL_C_EVENT {
        os . kill ( self . pid , signal . CTRL_C_EVENT );
        } else if sig == signal . CTRL_BREAK_EVENT {
        os . kill ( self . pid , signal . CTRL_BREAK_EVENT );
        } else {
        panic!("ValueError ( "Unsupported signal: {}" . format ( sig ) )");
        pub fn terminate ( self )  {
        "Terminates the process.";
        if self . returncode is !None /* Option */ {
        return;
        // try {
        _winapi . TerminateProcess ( self . _handle , 1 );
        // } catch  PermissionError  {
        rc = _winapi . GetExitCodeProcess ( self . _handle );
        if rc == _winapi . STILL_ACTIVE {
        panic!("");
        self . returncode = rc;
        kill = terminate;
        } else {
        pub fn _get_handles ( &self, stdin , stdout , stderr )  {
        "Construct && return tuple with IO objects:
            p2cread, p2cwrite, c2pread, c2pwrite, errread, errwrite
            ";
        p2cread , p2cwrite = -1 , -1;
        c2pread , c2pwrite = -1 , -1;
        errread , errwrite = -1 , -1;
        // with scope: self . _on_error_fd_closer ( ) as err_close_fds  {
        if stdin is None /* Option */ {
        // pass
        } else if stdin == PIPE {
        p2cread , p2cwrite = os . pipe ( );
        err_close_fds . extend ( ( p2cread , p2cwrite ) );
        if self . pipesize > 0 && hasattr ( fcntl , "F_SETPIPE_SZ" ) {
        fcntl . fcntl ( p2cwrite , fcntl . F_SETPIPE_SZ , self . pipesize );
        } else if stdin == DEVNULL {
        p2cread = self . _get_devnull ( );
        } else if isinstance ( stdin , int ) {
        p2cread = stdin;
        } else {
        p2cread = stdin . fileno ( );
        if stdout is None /* Option */ {
        // pass
        } else if stdout == PIPE {
        c2pread , c2pwrite = os . pipe ( );
        err_close_fds . extend ( ( c2pread , c2pwrite ) );
        if self . pipesize > 0 && hasattr ( fcntl , "F_SETPIPE_SZ" ) {
        fcntl . fcntl ( c2pwrite , fcntl . F_SETPIPE_SZ , self . pipesize );
        } else if stdout == DEVNULL {
        c2pwrite = self . _get_devnull ( );
        } else if isinstance ( stdout , int ) {
        c2pwrite = stdout;
        } else {
        c2pwrite = stdout . fileno ( );
        if stderr is None /* Option */ {
        // pass
        } else if stderr == PIPE {
        errread , errwrite = os . pipe ( );
        err_close_fds . extend ( ( errread , errwrite ) );
        if self . pipesize > 0 && hasattr ( fcntl , "F_SETPIPE_SZ" ) {
        fcntl . fcntl ( errwrite , fcntl . F_SETPIPE_SZ , self . pipesize );
        } else if stderr == STDOUT {
        if c2pwrite != -1 {
        errwrite = c2pwrite;
        } else {
        errwrite = sys . __stdout__ . fileno ( );
        } else if stderr == DEVNULL {
        errwrite = self . _get_devnull ( );
        } else if isinstance ( stderr , int ) {
        errwrite = stderr;
        } else {
        errwrite = stderr . fileno ( );
        return  ( p2cread , p2cwrite ,;
        c2pread , c2pwrite ,;
        errread , errwrite );
        pub fn _posix_spawn ( &self, args , executable , env , restore_signals , {
        p2cread , p2cwrite ,;
        c2pread , c2pwrite ,;
        errread , errwrite ) ;
        "Execute program using os.posix_spawn().";
        if env is None /* Option */ {
        env = os . environ;
        kwargs = { };
        if restore_signals {
        sigset = [ ];
        for signame in ( "SIGPIPE" , "SIGXFZ" , "SIGXFSZ" ) .iter() {
        signum = getattr ( signal , signame , None /* Option */ );
        if signum is !None /* Option */ {
        sigset . append ( signum );
        kwargs [ "setsigdeformat!(" ] = sigset);
        file_actions = [ ];
        for fd in ( p2cwrite , c2pread , errread ) .iter() {
        if fd != -1 {
        file_actions . append ( ( os . POSIX_SPAWN_CLOSE , fd ) );
        for fd , fd2 in (.iter() {
        ( p2cread , 0 ) ,;
        ( c2pwrite , 1 ) ,;
        ( errwrite , 2 ) ,;
        ) ;
        if fd != -1 {
        file_actions . append ( ( os . POSIX_SPAWN_DUP2 , fd , fd2 ) );
        if file_actions {
        kwargs [ "file_actions" ] = file_actions;
        self . pid = os . posix_spawn ( executable , args , env , ** kwargs );
        self . _child_created = true;
        self . _close_pipe_fds ( p2cread , p2cwrite ,;
        c2pread , c2pwrite ,;
        errread , errwrite );
        pub fn _execute_child ( &self, args , executable , preexec_fn , close_fds , {
        pass_fds , cwd , env ,;
        startupinfo , creationflags , shell ,;
        p2cread , p2cwrite ,;
        c2pread , c2pwrite ,;
        errread , errwrite ,;
        restore_signals ,;
        gid , gids , uid , umask ,;
        start_new_session , process_group ) ;
        "Execute program (POSIX version)";
        if isinstance ( args , ( str , bytes ) ) {
        args = [ args ];
        } else if isinstance ( args , os . PathLike ) {
        if shell {
        panic!("TypeError ( "path-like args is !allowed when "");
        "shell == true" );
        args = [ args ];
        } else {
        args = list ( args );
        if shell {
        unix_shell = ( "/system/bin/sh" if;
        hasattr ( sys , "getandroidapilevel" ) else "/bin/sh" );
        args = [ unix_shell , "-c" ] + args;
        if executable {
        args [ 0 ] = executable;
        if executable is None /* Option */ {
        executable = args [ 0 ];
        sys . audit ( "subprocess.Popen" , executable , args , cwd , env );
        if ( _USE_POSIX_SPAWN {
        and os . path . dirname ( executable );
        and preexec_fn == None /* Option */;
        and !close_fds;
        and !pass_fds;
        and cwd == None /* Option */;
        and ( p2cread == -1 || p2cread > 2 );
        and ( c2pwrite == -1 || c2pwrite > 2 );
        and ( errwrite == -1 || errwrite > 2 );
        and !start_new_session;
        and process_group == -1;
        and gid == None /* Option */;
        and gids == None /* Option */;
        and uid == None /* Option */;
        and umask < 0 ) ;
        self . _posix_spawn ( args , executable , env , restore_signals ,;
        p2cread , p2cwrite ,;
        c2pread , c2pwrite ,;
        errread , errwrite );
        return;
        orig_executable = executable;
        errpipe_read , errpipe_write = os . pipe ( );
        low_fds_to_close = [ ];
        while errpipe_write < 3  {
        low_fds_to_close . append ( errpipe_write );
        errpipe_write = os . dup ( errpipe_write );
        for low_fd in low_fds_to_close .iter() {
        os . close ( low_fd );
        // try {
        // try {
        if env is !None /* Option */ {
        env_list = [ ];
        for k , v in env . items ( ) .iter() {
        k = os . fsencode ( k );
        if b "=" in k {
        panic!("ValueError ( "illegal environment variable name" )");
        env_list . append ( k + b "=" + os . fsencode ( v ) );
        } else {
        env_list = None /* Option */;
        executable = os . fsencode ( executable );
        if os . path . dirname ( executable ) {
        executable_list = ( executable , );
        } else {
        executable_list = tuple (;
        os . path . join ( os . fsencode ( dir ) , executable );
        for dir in os . get_exec_path ( env ) ).iter() {
        fds_to_keep = set ( pass_fds );
        fds_to_keep . add ( errpipe_write );
        self . pid = _fork_exec (;
        args , executable_list ,;
        close_fds , tuple ( sorted ( map ( int , fds_to_keep ) ) ) ,;
        cwd , env_list ,;
        p2cread , p2cwrite , c2pread , c2pwrite ,;
        errread , errwrite ,;
        errpipe_read , errpipe_write ,;
        restore_signals , start_new_session ,;
        process_group , gid , gids , uid , umask ,;
        preexec_fn , _USE_VFORK );
        self . _child_created = true;
        // } finally {
        os . close ( errpipe_write );
        self . _close_pipe_fds ( p2cread , p2cwrite ,;
        c2pread , c2pwrite ,;
        errread , errwrite );
        errpipe_data = bytearray ( );
        while true  {
        part = os . read ( errpipe_read , 50000 );
        errpipe_data + = part;
        if !part || len ( errpipe_data ) > 50000 {
        break;
        // } finally {
        os . close ( errpipe_read );
        if errpipe_data {
        // try {
        pid , sts = os . waitpid ( self . pid , 0 );
        if pid == self . pid {
        self . _handle_exitstatus ( sts );
        } else {
        self . returncode = sys . maxsize;
        // } catch  ChildProcessError  {
        // pass
        // try {
        // } catch ion_name , hex_errno , err_msg = ( {
        errpipe_data . split ( b ":" , 2 ) );
        err_msg = err_msg . decode ( );
        // } catch  ValueError  {
        // } catch ion_name = b "SubprocessError" {
        hex_errno = b "0";
        err_msg = "Bad exception data from child: {!r}" . format (;
        bytes ( errpipe_data ) );
        child_exception_type = getattr (;
        builtins , exception_name . decode ( "ascii" ) ,;
        SubprocessError );
        if issubclass ( child_exception_type , OSError ) && hex_errno {
        errno_num = int ( hex_errno , 16 );
        if err_msg == "noexec:chdir" {
        err_msg = "";
        err_filename = cwd;
        } else if err_msg == "noexec" {
        err_msg = "";
        err_filename = None /* Option */;
        } else {
        err_filename = orig_executable;
        if errno_num != 0 {
        err_msg = os . strerror ( errno_num );
        if err_filename is !None /* Option */ {
        panic!("child_exception_type ( errno_num , err_msg , err_filename )");
        } else {
        panic!("child_exception_type ( errno_num , err_msg )");
        panic!("child_exception_type ( err_msg )");
        pub fn _handle_exitstatus ( &self, sts , {
        _waitstatus_to_exitcode = _waitstatus_to_exitcode ,;
        _WIFSTOPPED = _WIFSTOPPED ,;
        _WSTOPSIG = _WSTOPSIG ) ;
        "All callers to this function MUST hold self._waitpid_lock.";
        if _WIFSTOPPED ( sts ) {
        self . returncode = - _WSTOPSIG ( sts );
        } else {
        self . returncode = _waitstatus_to_exitcode ( sts );
        pub fn _internal_poll ( &self, _deadstate = None /* Option */ , _waitpid = _waitpid , {
        _WNOHANG = _WNOHANG , _ECHILD = errno . ECHILD ) ;
        "Check if child process has terminated.  Returns returncode
            attribute.

            This method == called by __del__, so it cannot reference anything
            outside of the local scope (nor can any methods it calls).

            ";
        if self . returncode is None /* Option */ {
        if !self . _waitpid_lock . acquire ( false ) {
        return;
        // try {
        if self . returncode is !None /* Option */ {
        return  self . returncode;
        pid , sts = _waitpid ( self . pid , _WNOHANG );
        if pid == self . pid {
        self . _handle_exitstatus ( sts );
        // } catch  OSError as e  {
        if _deadstate is !None /* Option */ {
        self . returncode = _deadstate;
        } else if e . errno == _ECHILD {
        self . returncode = 0;
        // } finally {
        self . _waitpid_lock . release ( );
        return  self . returncode;
        pub fn _try_wait ( &self, wait_flags )  {
        "All callers to this function MUST hold self._waitpid_lock.";
        // try {
        ( pid , sts ) = os . waitpid ( self . pid , wait_flags );
        // } catch  ChildProcessError  {
        pid = self . pid;
        sts = 0;
        return  ( pid , sts );
        pub fn _wait ( &self, timeout )  {
        "Internal implementation of wait() on POSIX.";
        if self . returncode is !None /* Option */ {
        return  self . returncode;
        if timeout is !None /* Option */ {
        endtime = _time ( ) + timeout;
        delay = 0.0005;
        while true  {
        if self . _waitpid_lock . acquire ( false ) {
        // try {
        if self . returncode is !None /* Option */ {
        break;
        ( pid , sts ) = self . _try_wait ( os . WNOHANG );
        assert pid == self . pid || pid == 0;
        if pid == self . pid {
        self . _handle_exitstatus ( sts );
        break;
        // } finally {
        self . _waitpid_lock . release ( );
        remaining = self . _remaining_time ( endtime );
        if remaining <= 0 {
        panic!("TimeoutExpired ( self . args , timeout )");
        delay = min ( delay * 2 , remaining , . 05 );
        time . sleep ( delay );
        } else {
        while self . returncode is None /* Option */  {
        // with scope: self . _waitpid_lock  {
        if self . returncode is !None /* Option */ {
        break;
        ( pid , sts ) = self . _try_wait ( 0 );
        if pid == self . pid {
        self . _handle_exitstatus ( sts );
        return  self . returncode;
        pub fn _communicate ( &self, input , endtime , orig_timeout )  {
        if self . stdin && !self . _communication_started {
        // try {
        self . stdin . flush ( );
        // } catch  BrokenPipeError  {
        // pass
        if !input {
        // try {
        self . stdin . close ( );
        // } catch  BrokenPipeError  {
        // pass
        stdout = None /* Option */;
        stderr = None /* Option */;
        if !self . _communication_started {
        self . _fileobj2output = { };
        if self . stdout {
        self . _fileobj2output [ self . stdout ] = [ ];
        if self . stderr {
        self . _fileobj2output [ self . stderr ] = [ ];
        if self . stdout {
        stdout = self . _fileobj2output [ self . stdout ];
        if self . stderr {
        stderr = self . _fileobj2output [ self . stderr ];
        self . _save_input ( input );
        if self . _input {
        input_view = memoryview ( self . _input );
        // with scope: _PopenSelector ( ) as selector  {
        if self . stdin && input {
        selector . register ( self . stdin , selectors . EVENT_WRITE );
        if self . stdout && !self . stdout . closed {
        selector . register ( self . stdout , selectors . EVENT_READ );
        if self . stderr && !self . stderr . closed {
        selector . register ( self . stderr , selectors . EVENT_READ );
        while selector . get_map ( )  {
        timeout = self . _remaining_time ( endtime );
        if timeout is !None /* Option */ && timeout < 0 {
        self . _check_timeout ( endtime , orig_timeout ,;
        stdout , stderr ,;
        skip_check_and_raise = true );
        panic!("RuntimeError (");
        "_check_timeout(..., skip_check_and_raise=true) ";
        "failed to raise TimeoutExpired." );
        ready = selector . select ( timeout );
        self . _check_timeout ( endtime , orig_timeout , stdout , stderr );
        for key , events in ready .iter() {
        if key . fileobj is self . stdin {
        chunk = input_view [ self . _input_offset ;
        self . _input_offset + _PIPE_BUF ];
        // try {
        self . _input_offset + = os . write ( key . fd , chunk );
        // } catch  BrokenPipeError  {
        selector . unregister ( key . fileobj );
        key . fileobj . close ( );
        } else {
        if self . _input_offset >= len ( self . _input ) {
        selector . unregister ( key . fileobj );
        key . fileobj . close ( );
        } else if key . fileobj in ( self . stdout , self . stderr ) {
        data = os . read ( key . fd , 32768 );
        if !data {
        selector . unregister ( key . fileobj );
        key . fileobj . close ( );
        self . _fileobj2output [ key . fileobj ] . append ( data );
        self . wait ( timeout = self . _remaining_time ( endtime ) );
        if stdout is !None /* Option */ {
        stdout = b "" . join ( stdout );
        if stderr is !None /* Option */ {
        stderr = b "" . join ( stderr );
        if self . text_mode {
        if stdout is !None /* Option */ {
        stdout = self . _translate_newlines ( stdout ,;
        self . stdout . encoding ,;
        self . stdout . errors );
        if stderr is !None /* Option */ {
        stderr = self . _translate_newlines ( stderr ,;
        self . stderr . encoding ,;
        self . stderr . errors );
        return  ( stdout , stderr );
        pub fn _save_input ( &self, input )  {
        if self . stdin && self . _input is None /* Option */ {
        self . _input_offset = 0;
        self . _input = input;
        if input is !None /* Option */ && self . text_mode {
        self . _input = self . _input . encode ( self . stdin . encoding ,;
        self . stdin . errors );
        pub fn send_signal ( &self, sig )  {
        "Send a signal to the process.";
        self . poll ( );
        if self . returncode is !None /* Option */ {
        return;
        // try {
        os . kill ( self . pid , sig );
        // } catch  ProcessLookupError  {
        // pass
        pub fn terminate ( self )  {
        "Terminate the process with SIGTERM
            ";
        self . send_signal ( signal . SIGTERM );
        pub fn kill ( self )  {
        "Kill the process with SIGKILL
            ";
        self . send_signal ( signal . SIGKILL );
    }

}

