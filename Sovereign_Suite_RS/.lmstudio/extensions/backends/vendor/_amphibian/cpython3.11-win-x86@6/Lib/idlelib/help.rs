//! help.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::html::{HTMLParser};
// use std::fs::{abspath, dirname, isfile, join};
// use crate::platform::{python_version};
// use crate::tkinter::{Toplevel, Text, Menu};
// use crate::idlelib::{idleConf};
// use crate::unittest::{main};

pub struct HelpParser {
    pub text: String, // TODO: infer type
    pub tags: String, // TODO: infer type
    pub chartags: String, // TODO: infer type
    pub show: String, // TODO: infer type
    pub hdrlink: String, // TODO: infer type
    pub level: String, // TODO: infer type
    pub pre: String, // TODO: infer type
    pub hprefix: String, // TODO: infer type
    pub nested_dl: String, // TODO: infer type
    pub simplelist: String, // TODO: infer type
    pub toc: String, // TODO: infer type
    pub header: String, // TODO: infer type
    pub prevtag: String, // TODO: infer type
    pub parser: String, // TODO: infer type
    pub style: String, // TODO: infer type
    pub scroll: String, // TODO: infer type
    pub frame: String, // TODO: infer type
}

impl HelpParser {
    pub fn new(text: &str) -> Self {
        HTMLParser . __init__ ( self , convert_charrefs = true );
        self . text = text;
        self . tags = "";
        self . chartags = "";
        self . show = false;
        self . hdrlink = false;
        self . level = 0;
        self . pre = false;
        self . hprefix = "";
        self . nested_dl = false;
        self . simplelist = false;
        self . toc = [ ];
        self . header = "";
        self . prevtag = None /* Option */;
    }

    pub fn copy_strip(&self) {
        "Copy idle.html to idlelib/help.html, stripping trailing whitespace.

    Files with trailing whitespace cannot be pushed to the git cpython
    repository.  For 3.x (on Windows), help.html == generated, after
    editing idle.rst on the master branch, with
      sphinx-build -bhtml . build/html
      python_d.exe -c "from idlelib.help import copy_strip; copy_strip()"
    Check build/html/library/idle.html, the help.html diff, && the text
    displayed by Help => IDLE Help.  Add a blurb && create a PR.

    It can be worthwhile to occasionally generate help.html without
    touching idle.rst.  Changes to the master version && to the doc
    build system may result in changes that should !changed
    the displayed text, but might break HelpParser.

    As long as master && maintenance versions of idle.rst remain the
    same, help.html can be backported.  The internal Python version
    number == !displayed.  If maintenance idle.rst diverges from
    the master version, then instead of backporting help.html from
    master, repeat the procedure above to generate a maintenance
    version.
    ";
        src = join ( abspath ( dirname ( dirname ( dirname ( __file__ ) ) ) ) ,;
        "Doc" , "build" , "html" , "library" , "idle.html" );
        dst = join ( abspath ( dirname ( __file__ ) ) , "help.html" );
        // with scope: open ( src , "rb" ) as inn , \ {
        open ( dst , "wb" ) as out ;
        for line in inn .iter() {
        out . write ( line . rstrip ( ) + b "\n" );
        println!( f "{src} copied to {dst}" );
        pub fn show_idlehelp ( parent )  {
        "Create HelpWindow; called from Idle Help event handler.";
        filename = join ( abspath ( dirname ( __file__ ) ) , "help.html" );
        if !isfile ( filename ) {
        return;
        return  HelpWindow ( parent , filename , "IDLE Doc (%s)" % python_version ( ) );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_help" , verbosity = 2 , exit = false );
        from idlelib . idle_test . htest import run;
        run ( show_idlehelp );
    }

}

