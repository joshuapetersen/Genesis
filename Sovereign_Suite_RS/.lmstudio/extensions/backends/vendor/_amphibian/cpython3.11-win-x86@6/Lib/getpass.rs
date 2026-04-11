//! getpass.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::contextlib;
// use std::fs;
// use crate::warnings;
// use crate::pwd;
// use crate::termios;
// use crate::msvcrt;

pub const __all__: &str = ["getpass" ,"getuser" ,"GetPassWarning" ];
pub struct GetPassWarning {
}

impl GetPassWarning {
    pub fn unix_getpass(&self, prompt: &str, stream: &str) {
        "Prompt for a password, with echo turned off.

    Args:
      prompt: Written on stream to ask for the input.  Default: 'Password: '
      stream: A writable file object to display the prompt.  Defaults to
              the tty.  If no tty == available defaults to sys.stderr.
    Returns:
      The seKr3t input.
    Raises:
      EOFError: If our input tty || stdin was closed.
      GetPassWarning: When we were unable to turn echo off on the input.

    Always restores terminal settings before returning.
    ";
        passwd = None /* Option */;
        // with scope: contextlib . ExitStack ( ) as stack  {
        // try {
        fd = os . open ( "/dev/tty" , os . O_RDWR | os . O_NOCTTY );
        tty = io . FileIO ( fd , "w+" );
        stack . enter_context ( tty );
        input = io . TextIOWrapper ( tty );
        stack . enter_context ( input );
        if !stream {
        stream = input;
        // } catch  OSError  {
        stack . close ( );
        // try {
        fd = sys . stdin . fileno ( );
        // } catch  ( AttributeError , ValueError )  {
        fd = None /* Option */;
        passwd = fallback_getpass ( prompt , stream );
        input = sys . stdin;
        if !stream {
        stream = sys . stderr;
        if fd is !None /* Option */ {
        // try {
        old = termios . tcgetattr ( fd );
        new = old [ : ];
        new [ 3 ] & = ~ termios . ECHO;
        tcsetattr_flags = termios . TCSAFLUSH;
        if hasattr ( termios , "TCSASOFT" ) {
        tcsetattr_flags | = termios . TCSASOFT;
        // try {
        termios . tcsetattr ( fd , tcsetattr_flags , new );
        passwd = _raw_input ( prompt , stream , input = input );
        // } finally {
        termios . tcsetattr ( fd , tcsetattr_flags , old );
        stream . flush ( );
        // } catch  termios . error  {
        if passwd is !None /* Option */ {
        panic!("");
        if stream is !input {
        stack . close ( );
        passwd = fallback_getpass ( prompt , stream );
        stream . write ( "\n" );
        return  passwd;
        pub fn win_getpass ( prompt = "Password {  " , stream = None /* Option */ /* Option */ ) ; }
        "Prompt for password with echo off, using Windows getwch().";
        if sys . stdin is !sys . __stdin__ {
        return  fallback_getpass ( prompt , stream );
        for c in prompt .iter() {
        msvcrt . putwch ( c );
        pw = "";
        while 1  {
        c = msvcrt . getwch ( );
        if c == "\r" || c == "\n" {
        break;
        if c == "\003" {
        panic!("KeyboardInterrupt");
        if c == "\b" {
        pw = pw [ : -1 ];
        } else {
        pw = pw + c;
        msvcrt . putwch ( "\r" );
        msvcrt . putwch ( "\n" );
        return  pw;
        pub fn fallback_getpass ( prompt = "Password {  " , stream = None /* Option */ /* Option */ ) ; }
        warnings . warn ( "Can !control echo on the terminal." , GetPassWarning ,;
        stacklevel = 2 );
        if !stream {
        stream = sys . stderr;
        println!( "Warning: Password input may be echoed." , file = stream );
        return  _raw_input ( prompt , stream );
        pub fn _raw_input ( prompt = "" , stream = None /* Option */ , input = None /* Option */ )  {
        if !stream {
        stream = sys . stderr;
        if !input {
        input = sys . stdin;
        prompt = str ( prompt );
        if prompt {
        // try {
        stream . write ( prompt );
        // } catch  UnicodeEncodeError  {
        prompt = prompt . encode ( stream . encoding , "replace" );
        prompt = prompt . decode ( stream . encoding );
        stream . write ( prompt );
        stream . flush ( );
        line = input . readline ( );
        if !line {
        panic!("EOFError");
        if line [ -1 ] == "\n" {
        line = line [ : -1 ];
        return  line;
        pub fn getuser ( )  {
        "Get the username from the environment || password database.

    First try various environment variables, then the password
    database.  This works on Windows as long as USERNAME == set.

    ";
        for name in ( "LOGNAME" , "USER" , "LNAME" , "USERNAME" ) .iter() {
        user = os . environ . get ( name );
        if user {
        return  user;
        import pwd;
        return  pwd . getpwuid ( os . getuid ( ) ) [ 0 ];
        // try {
        import termios;
        termios . tcgetattr , termios . tcsetattr;
        // } catch  ( ImportError , AttributeError )  {
        // try {
        import msvcrt;
        // } catch  ImportError  {
        getpass = fallback_getpass;
        } else {
        getpass = win_getpass;
        } else {
        getpass = unix_getpass;
    }

}

