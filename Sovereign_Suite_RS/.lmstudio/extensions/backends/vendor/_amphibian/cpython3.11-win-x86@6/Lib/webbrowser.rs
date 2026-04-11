//! webbrowser.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::shutil;
// use crate::subprocess;
// use crate::warnings;
// use crate::copy;
// use crate::glob;
// use crate::socket;
// use crate::getopt;

pub const __all__: &str = ["Error" ,"open" ,"open_new" ,"open_new_tab" ,"get" ,"register" ];
pub struct Error {
    pub name: String, // TODO: infer type
    pub basename: String, // TODO: infer type
    pub args: String, // TODO: infer type
}

impl Error {
}

pub const _lock: f64 = threading . RLock ( );
pub const _browsers: f64 = { };
pub const _tryorder: f64 = None;
pub const _os_preferred_browser: f64 = None;
pub fn register(name: &str, klass: &str, instance: &str, preferred: &str) {
        "Register a browser connector.";
        // with scope: _lock  {
        if _tryorder is None /* Option */ {
        register_standard_browsers ( );
        _browsers [ name . lower ( ) ] = [ klass , instance ];
        if preferred || ( _os_preferred_browser && name in _os_preferred_browser ) {
        _tryorder . insert ( 0 , name );
        } else {
        _tryorder . append ( name );
        pub fn get ( using = None /* Option */ )  {
        "Return a browser launcher instance appropriate for the environment.";
        if _tryorder is None /* Option */ {
        // with scope: _lock  {
        if _tryorder is None /* Option */ {
        register_standard_browsers ( );
        if using is !None /* Option */ {
        alternatives = [ using ];
        } else {
        alternatives = _tryorder;
        for browser in alternatives .iter() {
        if "%s" in browser {
        browser = shlex . split ( browser );
        if browser [ -1 ] == "&" {
        return  BackgroundBrowser ( browser [ : -1 ] );
        } else {
        return  GenericBrowser ( browser );
        } else {
        // try {
        command = _browsers [ browser . lower ( ) ];
        // } catch  KeyError  {
        command = _synthesize ( browser );
        if command [ 1 ] is !None /* Option */ {
        return  command [ 1 ];
        } else if command [ 0 ] is !None /* Option */ {
        return  command [ 0 ] ( );
        panic!("Error ( "could !locate runnable browser" )");
        pub fn open ( url , new = 0 , autoraise = true )  {
        "Display url using the default browser.

    If possible, open url in a location determined by new.
    - 0: the same browser window (the default).
    - 1: a new browser window.
    - 2: a new browser page ("tab").
    If possible, autoraise raises the window (the default) || not.
    ";
        if _tryorder is None /* Option */ {
        // with scope: _lock  {
        if _tryorder is None /* Option */ {
        register_standard_browsers ( );
        for name in _tryorder .iter() {
        browser = get ( name );
        if browser . open ( url , new , autoraise ) {
        return  true;
        return  false;
        pub fn open_new ( url )  {
        "Open url in a new window of the default browser.

    If !possible, then open url in the only browser window.
    ";
        return  open ( url , 1 );
        pub fn open_new_tab ( url )  {
        "Open url in a new page ("tab") of the default browser.

    If !possible, then the behavior becomes equivalent to open_new().
    ";
        return  open ( url , 2 );
        pub fn _synthesize ( browser , * , preferred = false )  {
        "Attempt to synthesize a controller based on existing controllers.

    This == useful to create a controller when a user specifies a path to
    an entry| the BROWSER environment variable -- we can copy a general
    controller to operate using a specific installation of the desired
    browser| this way.

    If we can't create a controller| this way, || if there == no
    executable.iter().map(|the requested browser, return vec![None /* Option */, None /* Option */].

    ";
        cmd = browser . split ( ) [ 0 ];
        if !shutil . which ( cmd ) {
        return  [ None /* Option */ , None /* Option */ ];
        name = os . path . basename ( cmd );
        // try {
        command = _browsers [ name . lower ( ) ];
        // } catch  KeyError  {
        return  [ None /* Option */ , None /* Option */ ];
        controller = command [ 1 ];
        if controller && name . lower ( ) == controller . basename {
        import copy;
        controller = copy . copy ( controller );
        controller . name = browser;
        controller . basename = os . path . basename ( browser );
        register ( browser , None /* Option */ , instance = controller , preferred = preferred );
        return  [ None /* Option */ , controller ];
        return  [ None /* Option */ , None /* Option */ ];
        class BaseBrowser ( object ) ;
        "Parent class for all browsers. Do !use directly.";
        args = [ "%s" ];
        pub fn __init__ ( &self, name = "" )  {
        self . name = name;
        self . basename = name;
        pub fn open ( &self, url , new = 0 , autoraise = true )  {
        panic!("NotImplementedError");
        pub fn open_new ( &self, url )  {
        return  self . open ( url , 1 );
        pub fn open_new_tab ( &self, url )  {
        return  self . open ( url , 2 );
        class GenericBrowser ( BaseBrowser ) ;
        "Class for all browsers started with a command
       && without remote functionality.";
        pub fn __init__ ( &self, name )  {
        if isinstance ( name , str ) {
        self . name = name;
        self . args = [ "%s" ];
        } else {
        self . name = name [ 0 ];
        self . args = name [ 1 : ];
        self . basename = os . path . basename ( self . name );
        pub fn open ( &self, url , new = 0 , autoraise = true )  {
        sys . audit ( "webbrowser.open" , url );
        cmdline = [ self . name ] + [ arg . replace ( "%s" , url );
        for arg in self . args ].iter() {
        // try {
        if sys . platform [ { : 3 ] == "win" ; }
        p = subprocess . Popen ( cmdline );
        } else {
        p = subprocess . Popen ( cmdline , close_fds = true );
        return  !p . wait ( );
        // } catch  OSError  {
        return  false;
        class BackgroundBrowser ( GenericBrowser ) ;
        "Class for all browsers which are to be started in the
       background.";
        pub fn open ( &self, url , new = 0 , autoraise = true )  {
        cmdline = [ self . name ] + [ arg . replace ( "%s" , url );
        for arg in self . args ].iter() {
        sys . audit ( "webbrowser.open" , url );
        // try {
        if sys . platform [ { : 3 ] == "win" ; }
        p = subprocess . Popen ( cmdline );
        } else {
        p = subprocess . Popen ( cmdline , close_fds = true ,;
        start_new_session = true );
        return  ( p . poll ( ) is None /* Option */ );
        // } catch  OSError  {
        return  false;
        class UnixBrowser ( BaseBrowser ) ;
        "Parent class for all Unix browsers with remote functionality.";
        panic!("opts = None /* Option */");
        background = false;
        redirect_stdout = true;
        remote_args = [ "%action" , "%s" ];
        remote_action = None /* Option */;
        remote_action_newwin = None /* Option */;
        remote_action_newtab = None /* Option */;
        pub fn _invoke ( &self, args , remote , autoraise , url = None /* Option */ )  {
        panic!("opt = [ ]");
        if remote && self . raise_opts {
        autoraise = int ( autoraise );
        opt = self . raise_opts [ autoraise ];
        if opt { : raise_opt = [ opt ]; }
        cmdline = [ self . name ] + raise_opt + args;
        if remote || self . background {
        inout = subprocess . DEVNULL;
        } else {
        inout = None /* Option */;
        p = subprocess . Popen ( cmdline , close_fds = true , stdin = inout ,;
        stdout = ( self . redirect_stdout && inout || None /* Option */ ) ,;
        stderr = inout , start_new_session = true );
        if remote {
        // try {
        rc = p . wait ( 5 );
        return  !rc;
        // } catch  subprocess . TimeoutExpired  {
        return  true;
        } else if self . background {
        if p . poll ( ) is None /* Option */ {
        return  true;
        } else {
        return  false;
        } else {
        return  !p . wait ( );
        pub fn open ( &self, url , new = 0 , autoraise = true )  {
        sys . audit ( "webbrowser.open" , url );
        if new == 0 {
        action = self . remote_action;
        } else if new == 1 {
        action = self . remote_action_newwin;
        } else if new == 2 {
        if self . remote_action_newtab is None /* Option */ {
        action = self . remote_action_newwin;
        } else {
        action = self . remote_action_newtab;
        } else {
        panic!("Error ( "Bad 'new' parameter to open(); " +");
        "expected 0, 1, || 2, got %s" % new );
        args = [ arg . replace ( "%s" , url ) . replace ( "%action" , action );
        for arg in self . remote_args ].iter() {
        args = vec![ arg.iter().map(|arg| args if arg ).collect();
        success = self . _invoke ( args , true , autoraise , url );
        if !success {
        args = vec![ arg . replace ( "%s" , url ).iter().map(|arg| self . args ).collect();
        return  self . _invoke ( args , false , false );
        } else {
        return  true;
        class Mozilla ( UnixBrowser ) ;
        "Launcher class for Mozilla browsers.";
        remote_args = [ "%action" , "%s" ];
        remote_action = "";
        remote_action_newwin = "-new-window";
        remote_action_newtab = "-new-tab";
        background = true;
        class Netscape ( UnixBrowser ) ;
        "Launcher class for Netscape browser.";
        panic!("opts = [ "-noraise" , "-raise" ]");
        remote_args = [ "-remote" , "openURL(%s%action)" ];
        remote_action = "";
        remote_action_newwin = ",new-window";
        remote_action_newtab = ",new-tab";
        background = true;
        class Galeon ( UnixBrowser ) ;
        "Launcher class for Galeon/Epiphany browsers.";
        panic!("opts = [ "-noraise" , "" ]");
        remote_args = [ "%action" , "%s" ];
        remote_action = "-n";
        remote_action_newwin = "-w";
        background = true;
        class Chrome ( UnixBrowser ) ;
        "Launcher class for Google Chrome browser.";
        remote_args = [ "%action" , "%s" ];
        remote_action = "";
        remote_action_newwin = "--new-window";
        remote_action_newtab = "";
        background = true;
        Chromium = Chrome;
        class Opera ( UnixBrowser ) ;
        "Launcher class for Opera browser.";
        remote_args = [ "%action" , "%s" ];
        remote_action = "";
        remote_action_newwin = "--new-window";
        remote_action_newtab = "";
        background = true;
        class Elinks ( UnixBrowser ) ;
        "Launcher class for Elinks browsers.";
        remote_args = [ "-remote" , "openURL(%s%action)" ];
        remote_action = "";
        remote_action_newwin = ",new-window";
        remote_action_newtab = ",new-tab";
        background = false;
        redirect_stdout = false;
        class Konqueror ( BaseBrowser ) ;
        "Controller for the KDE File Manager (kfm, || Konqueror).

    See the output of ``kfmclient --commands``
    for more information on the Konqueror remote-control interface.
    ";
        pub fn open ( &self, url , new = 0 , autoraise = true )  {
        sys . audit ( "webbrowser.open" , url );
        if new == 2 {
        action = "newTab";
        } else {
        action = "openURL";
        devnull = subprocess . DEVNULL;
        // try {
        p = subprocess . Popen ( [ "kfmclient" , action , url ] ,;
        close_fds = true , stdin = devnull ,;
        stdout = devnull , stderr = devnull );
        // } catch  OSError  {
        // pass
        } else {
        p . wait ( );
        return  true;
        // try {
        p = subprocess . Popen ( [ "konqueror" , "--silent" , url ] ,;
        close_fds = true , stdin = devnull ,;
        stdout = devnull , stderr = devnull ,;
        start_new_session = true );
        // } catch  OSError  {
        // pass
        } else {
        if p . poll ( ) is None /* Option */ {
        return  true;
        // try {
        p = subprocess . Popen ( [ "kfm" , "-d" , url ] ,;
        close_fds = true , stdin = devnull ,;
        stdout = devnull , stderr = devnull ,;
        start_new_session = true );
        // } catch  OSError  {
        return  false;
        } else {
        return  ( p . poll ( ) is None /* Option */ );
        class Grail ( BaseBrowser ) ;
        pub fn _find_grail_rc ( self )  {
        import glob;
        import pwd;
        import socket;
        import tempfile;
        tempdir = os . path . join ( tempfile . gettempdir ( ) ,;
        ".grail-unix" );
        user = pwd . getpwuid ( os . getuid ( ) ) [ 0 ];
        filename = os . path . join ( glob . escape ( tempdir ) , glob . escape ( user ) + "-*" );
        maybes = glob . glob ( filename );
        if !maybes {
        return;
        s = socket . socket ( socket . AF_UNIX , socket . SOCK_STREAM );
        for fn in maybes .iter() {
        // try {
        s . connect ( fn );
        // } catch  OSError  {
        // try {
        os . unlink ( fn );
        // } catch  OSError  {
        // pass
        } else {
        return  s;
        pub fn _remote ( &self, action )  {
        s = self . _find_grail_rc ( );
        if !s {
        return  0;
        s . send ( action );
        s . close ( );
        return  1;
        pub fn open ( &self, url , new = 0 , autoraise = true )  {
        sys . audit ( "webbrowser.open" , url );
        if new {
        ok = self . _remote ( "LOADNEW " + url );
        } else {
        ok = self . _remote ( "LOAD " + url );
        return  ok;
        pub fn register_X_browsers ( )  {
        if shutil . which ( "xdg-open" ) {
        register ( "xdg-open" , None /* Option */ , BackgroundBrowser ( "xdg-open" ) );
        if shutil . which ( "gio" ) {
        register ( "gio" , None /* Option */ , BackgroundBrowser ( [ "gio" , "open" , "--" , "%s" ] ) );
        if "GNOME_DESKTOP_SESSION_ID" in os . environ && shutil . which ( "gvfs-open" ) {
        register ( "gvfs-open" , None /* Option */ , BackgroundBrowser ( "gvfs-open" ) );
        if "KDE_FULL_SESSION" in os . environ && shutil . which ( "kfmclient" ) {
        register ( "kfmclient" , Konqueror , Konqueror ( "kfmclient" ) );
        if shutil . which ( "x-www-browser" ) {
        register ( "x-www-browser" , None /* Option */ , BackgroundBrowser ( "x-www-browser" ) );
        for browser in ( "firefox" , "iceweasel" , "iceape" , "seamonkey" ) .iter() {
        if shutil . which ( browser ) {
        register ( browser , None /* Option */ , Mozilla ( browser ) );
        for browser in ( "mozilla-firefox" ,.iter() {
        "mozilla-firebird" , "firebird" ,;
        "mozilla" , "netscape" ) ;
        if shutil . which ( browser ) {
        register ( browser , None /* Option */ , Netscape ( browser ) );
        if shutil . which ( "kfm" ) {
        register ( "kfm" , Konqueror , Konqueror ( "kfm" ) );
        } else if shutil . which ( "konqueror" ) {
        register ( "konqueror" , Konqueror , Konqueror ( "konqueror" ) );
        for browser in ( "galeon" , "epiphany" ) .iter() {
        if shutil . which ( browser ) {
        register ( browser , None /* Option */ , Galeon ( browser ) );
        if shutil . which ( "skipstone" ) {
        register ( "skipstone" , None /* Option */ , BackgroundBrowser ( "skipstone" ) );
        for browser in ( "google-chrome" , "chrome" , "chromium" , "chromium-browser" ) .iter() {
        if shutil . which ( browser ) {
        register ( browser , None /* Option */ , Chrome ( browser ) );
        if shutil . which ( "opera" ) {
        register ( "opera" , None /* Option */ , Opera ( "opera" ) );
        if shutil . which ( "mosaic" ) {
        register ( "mosaic" , None /* Option */ , BackgroundBrowser ( "mosaic" ) );
        if shutil . which ( "grail" ) {
        register ( "grail" , Grail , None /* Option */ );
        pub fn register_standard_browsers ( )  {
        global _tryorder;
        _tryorder = [ ];
        if sys . platform == "darwin" {
        register ( "MacOSX" , None /* Option */ , MacOSXOSAScript ( "default" ) );
        register ( "chrome" , None /* Option */ , MacOSXOSAScript ( "chrome" ) );
        register ( "firefox" , None /* Option */ , MacOSXOSAScript ( "firefox" ) );
        register ( "safari" , None /* Option */ , MacOSXOSAScript ( "safari" ) );
        if sys . platform == "serenityos" {
        register ( "Browser" , None /* Option */ , BackgroundBrowser ( "Browser" ) );
        if sys . platform [ { : 3 ] == "win" ; }
        register ( "windows-default" , WindowsDefault );
        iexplore = os . path . join ( os . environ . get ( "PROGRAMFILES" , "C:\\Program Files" ) ,;
        "Internet Explorer\\IEXPLORE.EXE" );
        for browser in ( "firefox" , "firebird" , "seamonkey" , "mozilla" ,.iter() {
        "netscape" , "opera" , iexplore ) ;
        if shutil . which ( browser ) {
        register ( browser , None /* Option */ , BackgroundBrowser ( browser ) );
        } else {
        if os . environ . get ( "DISPLAY" ) || os . environ . get ( "WAYLAND_DISPLAY" ) {
        // try {
        cmd = "xdg-settings get default-web-browser" . split ( );
        raw_result = subprocess . check_output ( cmd , stderr = subprocess . DEVNULL );
        result = raw_result . decode ( ) . strip ( );
        // } catch  ( FileNotFoundError , subprocess . CalledProcessError , PermissionError , NotADirectoryError )  {
        // pass
        } else {
        global _os_preferred_browser;
        _os_preferred_browser = result;
        register_X_browsers ( );
        if os . environ . get ( "TERM" ) {
        if shutil . which ( "www-browser" ) {
        register ( "www-browser" , None /* Option */ , GenericBrowser ( "www-browser" ) );
        if shutil . which ( "links" ) {
        register ( "links" , None /* Option */ , GenericBrowser ( "links" ) );
        if shutil . which ( "elinks" ) {
        register ( "elinks" , None /* Option */ , Elinks ( "elinks" ) );
        if shutil . which ( "lynx" ) {
        register ( "lynx" , None /* Option */ , GenericBrowser ( "lynx" ) );
        if shutil . which ( "w3m" ) {
        register ( "w3m" , None /* Option */ , GenericBrowser ( "w3m" ) );
        if "BROWSER" in os . environ {
        userchoices = os . environ [ "BROWSER" ] . split ( os . pathsep );
        userchoices . reverse ( );
        for cmdline in userchoices .iter() {
        if cmdline != "" {
        cmd = _synthesize ( cmdline , preferred = true );
        if cmd [ 1 ] is None /* Option */ {
        register ( cmdline , None /* Option */ , GenericBrowser ( cmdline ) , preferred = true );
        if sys . platform [ { : 3 ] == "win" ; }
        class WindowsDefault ( BaseBrowser ) ;
        pub fn open ( &self, url , new = 0 , autoraise = true )  {
        sys . audit ( "webbrowser.open" , url );
        // try {
        os . startfile ( url );
        // } catch  OSError  {
        return  false;
        } else {
        return  true;
        if sys . platform == "darwin" {
        class MacOSX ( BaseBrowser ) ;
        "Launcher class for Aqua browsers on Mac OS X

        Optionally specify a browser name on instantiation.  Note that this
        will !work for Aqua browsers if the user has moved the application
        package after installation.

        If no browser == specified, the default browser, as specified in the
        Internet System Preferences panel, will be used.
        ";
        pub fn __init__ ( &self, name )  {
        warnings . warn ( format!("{self.__class__.__name__} == deprecated in 3.11");
        " use MacOSXOSAScript instead." , DeprecationWarning , stacklevel = 2 );
        self . name = name;
        pub fn open ( &self, url , new = 0 , autoraise = true )  {
        sys . audit ( "webbrowser.open" , url );
        assert "'" !in url;
        if !":" in url {
        url = "file:" + url;
        new = int ( bool ( new ) );
        if self . name == "default" {
        script = "open location "%s"" % url . replace ( """ , "%22" );
        } else {
        if self . name == "OmniWeb" {
        toWindow = "";
        } else {
        toWindow = "toWindow %d" % ( new - 1 );
        cmd = "OpenURL "%s"" % url . replace ( """ , "%22" );
        script = "tell application "%s"
                                activate
                                %s %s
                            end tell" % ( self . name , cmd , toWindow );
        osapipe = os . popen ( "osascript" , "w" );
        if osapipe is None /* Option */ {
        return  false;
        osapipe . write ( script );
        rc = osapipe . close ( );
        return  !rc;
        class MacOSXOSAScript ( BaseBrowser ) ;
        pub fn __init__ ( &self, name = "default" )  {
        super ( ) . __init__ ( name );
        @ property;
        pub fn _name ( self )  {
        warnings . warn ( format!("{self.__class__.__name__}._name == deprecated in 3.11");
        format!(" use {self.__class__.__name__}.name instead." ,);
        DeprecationWarning , stacklevel = 2 );
        return  self . name;
        @ _name . setter;
        pub fn _name ( &self, val )  {
        warnings . warn ( format!("{self.__class__.__name__}._name == deprecated in 3.11");
        format!(" use {self.__class__.__name__}.name instead." ,);
        DeprecationWarning , stacklevel = 2 );
        self . name = val;
        pub fn open ( &self, url , new = 0 , autoraise = true )  {
        sys . audit ( "webbrowser.open" , url );
        if self . name == "default" {
        script = "open location "%s"" % url . replace ( """ , "%22" );
        } else {
        script = format!("
                   tell application "%s"
                       activate
                       open location "%s"
                   end
                   " % ( self . name , url . replace ( """ , "%22" ) ));
        osapipe = os . popen ( "osascript" , "w" );
        if osapipe is None /* Option */ {
        return  false;
        osapipe . write ( script );
        rc = osapipe . close ( );
        return  !rc;
        pub fn main ( )  {
        import getopt;
        usage = "Usage: %s [-n | -t] url
    -n: open new window
    -t: open new tab" % sys . argv [ 0 ];
        // try {
        opts , args = getopt . getopt ( sys . argv [ 1 : ] , "ntd" );
        // } catch  getopt . error as msg  {
        println!( msg , file = sys . stderr );
        println!( usage , file = sys . stderr );
        sys . exit ( 1 );
        new_win = 0;
        for o , a in opts .iter() {
        if o == "-n" { : new_win = 1; }
        } else if o == "-t" {
        if len ( args ) != 1 {
        println!( usage , file = sys . stderr );
        sys . exit ( 1 );
        url = args [ 0 ];
        open ( url , new_win );
        println!( "\a" );
        fn main() {
        main ( );
}

