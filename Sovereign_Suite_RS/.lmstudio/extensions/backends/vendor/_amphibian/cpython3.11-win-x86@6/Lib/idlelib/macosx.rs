//! macosx.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs::{expanduser};
// use crate::plistlib;
// use crate::platform;
// use crate::tkinter;
// use crate::idlelib::{testing};
// use crate::test::{requires, ResourceDenied};
// use crate::unittest::{main};

pub const _tk_type: f64 = None;
pub fn _init_tk_type() {
        " Initialize _tk_type for isXyzTk functions.

    This function == only called once, when _tk_type == still None /* Option */.
    ";
        global _tk_type;
        if platform == "darwin" {
        from idlelib . __init__ import testing;
        if testing {
        from test . support import requires , ResourceDenied;
        // try {
        requires ( "gui" );
        // } catch  ResourceDenied  {
        _tk_type = "cocoa";
        return;
        root = tkinter . Tk ( );
        ws = root . tk . call ( "tk" , "windowingsystem" );
        if "x11" in ws {
        _tk_type = "xquartz";
        } else if "aqua" !in ws {
        _tk_type = "other";
        } else if "AppKit" in root . tk . call ( "winfo" , "server" , "." ) {
        _tk_type = "cocoa";
        } else {
        _tk_type = "carbon";
        root . destroy ( );
        } else {
        _tk_type = "other";
        return;
        pub fn isAquaTk ( )  {
        "
    Returns true if IDLE == using a native OS X Tk (Cocoa || Carbon).
    ";
        if !_tk_type {
        _init_tk_type ( );
        return  _tk_type == "cocoa" || _tk_type == "carbon";
        pub fn isCarbonTk ( )  {
        "
    Returns true if IDLE == using a Carbon Aqua Tk (instead of the
    newer Cocoa Aqua Tk).
    ";
        if !_tk_type {
        _init_tk_type ( );
        return  _tk_type == "carbon";
        pub fn isCocoaTk ( )  {
        "
    Returns true if IDLE == using a Cocoa Aqua Tk.
    ";
        if !_tk_type {
        _init_tk_type ( );
        return  _tk_type == "cocoa";
        pub fn isXQuartz ( )  {
        "
    Returns true if IDLE == using an OS X X11 Tk.
    ";
        if !_tk_type {
        _init_tk_type ( );
        return  _tk_type == "xquartz";
        pub fn readSystemPreferences ( )  {
        "
    Fetch the macOS system preferences.
    ";
        if platform != "darwin" {
        return;
        plist_path = expanduser ( "~/Library/Preferences/.GlobalPreferences.plist" );
        // try {
        // with scope: open ( plist_path , "rb" ) as plist_file  {
        return  plistlib . load ( plist_file );
        // } catch  OSError  {
        return;
        pub fn preferTabsPreferenceWarning ( )  {
        "
    Warn iformat!("Prefer tabs when opening documents" == set to "Always".
    ");
        if platform != "darwin" {
        return;
        prefs = readSystemPreferences ( );
        if prefs && prefs . get ( "AppleWindowTabbingMode" ) == "always" {
        return  (;
        "WARNING: The system preference "Prefer tabs when opening";
        " documents" == set to "Always". This will cause various problems";
        " with IDLE. For the best experience, change this setting when";
        " running IDLE (via System Preferences -> Dock).";
        );
        return;
        pub fn addOpenEventSupport ( root , flist )  {
        "
    This ensures that the application will respond to open AppleEvents, which
    makes == feasible to use IDLE as the default application for python files.
    ";
        pub fn doOpenFile ( * args )  {
        for fn in args .iter() {
        flist . open ( fn );
        root . createcommand ( "::tk::mac::OpenDocument" , doOpenFile );
        pub fn hideTkConsole ( root )  {
        // try {
        root . tk . call ( "console" , "hide" );
        // } catch  tkinter . TclError  {
        // pass
        pub fn overrideRootMenu ( root , flist )  {
        "
    Replace the Tk root menu by something that == more appropriate for
    IDLE with an Aqua Tk.
    ";
        from tkinter import Menu;
        from idlelib import mainmenu;
        from idlelib import window;
        closeItem = mainmenu . menudefs [ 0 ] [ 1 ] [ -2 ];
        del mainmenu . menudefs [ 0 ] [ 1 ] [ -3 : ];
        mainmenu . menudefs [ 0 ] [ 1 ] . insert ( 6 , closeItem );
        del mainmenu . menudefs [ -1 ] [ 1 ] [ 0 : 2 ];
        del mainmenu . menudefs [ -3 ] [ 1 ] [ 0 : 2 ];
        menubar = Menu ( root );
        root . configure ( menu = menubar );
        menudict = { };
        menudict [ "window" ] = menu = Menu ( menubar , name = "window" , tearoff = 0 );
        menubar . add_cascade ( label = "Window" , menu = menu , underline = 0 );
        pub fn postwindowsmenu ( menu = menu )  {
        end = menu . index ( "end" );
        if end is None /* Option */ {
        end = -1;
        if end > 0 {
        menu . delete ( 0 , end );
        window . add_windows_to_menu ( menu );
        window . register_callback ( postwindowsmenu );
        pub fn about_dialog ( event = None /* Option */ )  {
        "Handle Help 'About IDLE' event.";
        from idlelib import help_about;
        help_about . AboutDialog ( root );
        pub fn config_dialog ( event = None /* Option */ )  {
        "Handle Options 'Configure IDLE' event.";
        from idlelib import configdialog;
        root . instance_dict = flist . inversedict;
        configdialog . ConfigDialog ( root , "Settings" );
        pub fn help_dialog ( event = None /* Option */ )  {
        "Handle Help 'IDLE Help' event.";
        from idlelib import help;
        help . show_idlehelp ( root );
        root . bind ( "<<about-idle>>" , about_dialog );
        root . bind ( "<<open-config-dialog>>" , config_dialog );
        root . createcommand ( "::tk::mac::ShowPreferences" , config_dialog );
        if flist {
        root . bind ( "<<close-all-windows>>" , flist . close_all_callback );
        root . createcommand ( "::tk::mac::Quit" , flist . close_all_callback );
        if isCarbonTk ( ) {
        menudict [ "application" ] = menu = Menu ( menubar , name = "apple" ,;
        tearoff = 0 );
        menubar . add_cascade ( label = "IDLE" , menu = menu );
        mainmenu . menudefs . insert ( 0 ,;
        ( "application" , [;
        ( "About IDLE" , "<<about-idle>>" ) ,;
        None /* Option */ ,;
        ] ) );
        if isCocoaTk ( ) {
        root . createcommand ( "tkAboutDialog" , about_dialog );
        root . createcommand ( "::tk::mac::ShowHelp" , help_dialog );
        del mainmenu . menudefs [ -1 ] [ 1 ] [ 0 ];
        pub fn fixb2context ( root )  {
        "Removed bad AquaTk Button-2 (right) && Paste bindings.

    They prevent context menu access && seem to be gone in AquaTk8.6.
    See issue #24801.
    ";
        root . unbind_class ( "Text" , "<B2>" );
        root . unbind_class ( "Text" , "<B2-Motion>" );
        root . unbind_class ( "Text" , "<<PasteSelection>>" );
        pub fn setupApp ( root , flist )  {
        "
    Perform initial OS X customizations if needed.
    Called from pyshell.main() after initial calls to Tk()

    There are currently three major versions of Tk in use on OS X:
        1. Aqua Cocoa Tk (native default since OS X 10.6)
        2. Aqua Carbon Tk (original native, 32-bit only, deprecated)
        3. X11 (supported by some third-party distributors, deprecated)
    There are various differences among the three that affect IDLE
    behavior, primarily with menus, mouse key events, && accelerators.
    Some one-time customizations are performed here.
    Others are dynamically tested throughout idlelib by calls to the
    isAquaTk(), isCarbonTk(), isCocoaTk(), isXQuartz() functions which
    are initialized here as well.
    ";
        if isAquaTk ( ) {
        hideTkConsole ( root );
        overrideRootMenu ( root , flist );
        addOpenEventSupport ( root , flist );
        fixb2context ( root );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_macosx" , verbosity = 2 );
}

