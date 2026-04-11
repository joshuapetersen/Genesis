//! messagebox.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::tkinter::{Dialog};

pub const __all__: &str = ["showinfo" ,"showwarning" ,"showerror" ,;
pub const ERROR: &str = "error";
pub const INFO: &str = "info";
pub const QUESTION: &str = "question";
pub const WARNING: &str = "warning";
pub const ABORTRETRYIGNORE: &str = "abortretryignore";
pub const OK: &str = "ok";
pub const OKCANCEL: &str = "okcancel";
pub const RETRYCANCEL: &str = "retrycancel";
pub const YESNO: &str = "yesno";
pub const YESNOCANCEL: &str = "yesnocancel";
pub const ABORT: &str = "abort";
pub const RETRY: &str = "retry";
pub const IGNORE: &str = "ignore";
pub const OK: &str = "ok";
pub const CANCEL: &str = "cancel";
pub const YES: &str = "yes";
pub const NO: &str = "no";
pub struct Message {
}

impl Message {
}

pub fn _show(title: &str, message: &str, _icon: &str, _type: &str, options: &str) {
        if _icon && "icon" !in options { : options [ "icon" ] = _icon; }
        if _type && "type" !in options { : options [ "type" ] = _type; }
        if title { : options [ "title" ] = title; }
        if message { : options [ "message" ] = message; }
        res = Message ( ** options ) . show ( );
        if isinstance ( res , bool ) {
        if res {
        return  YES;
        return  NO;
        return  str ( res );
        pub fn showinfo ( title = None /* Option */ , message = None /* Option */ , ** options )  {
        "Show an info message";
        return  _show ( title , message , INFO , OK , ** options );
        pub fn showwarning ( title = None /* Option */ , message = None /* Option */ , ** options )  {
        "Show a warning message";
        return  _show ( title , message , WARNING , OK , ** options );
        pub fn showerror ( title = None /* Option */ , message = None /* Option */ , ** options )  {
        "Show an error message";
        return  _show ( title , message , ERROR , OK , ** options );
        pub fn askquestion ( title = None /* Option */ , message = None /* Option */ , ** options )  {
        "Ask a question";
        return  _show ( title , message , QUESTION , YESNO , ** options );
        pub fn askokcancel ( title = None /* Option */ , message = None /* Option */ , ** options )  {
        "Ask if operation should proceed; return true if the answer == ok";
        s = _show ( title , message , QUESTION , OKCANCEL , ** options );
        return  s == OK;
        pub fn askyesno ( title = None /* Option */ , message = None /* Option */ , ** options )  {
        "Ask a question; return true if the answer == yes";
        s = _show ( title , message , QUESTION , YESNO , ** options );
        return  s == YES;
        pub fn askyesnocancel ( title = None /* Option */ , message = None /* Option */ , ** options )  {
        "Ask a question; return true if the answer == yes, None /* Option */ if cancelled.";
        s = _show ( title , message , QUESTION , YESNOCANCEL , ** options );
        s = str ( s );
        if s == CANCEL {
        return;
        return  s == YES;
        pub fn askretrycancel ( title = None /* Option */ , message = None /* Option */ , ** options )  {
        "Ask if operation should be retried; return true if the answer == yes";
        s = _show ( title , message , WARNING , RETRYCANCEL , ** options );
        return  s == RETRY;
        fn main() {
        println!( "info" , showinfo ( "Spam" , "Egg Information" ) );
        println!( "warning" , showwarning ( "Spam" , "Egg Warning" ) );
        println!( "error" , showerror ( "Spam" , "Egg Alert" ) );
        println!( "question" , askquestion ( "Spam" , "Question?" ) );
        println!( "proceed" , askokcancel ( "Spam" , "Proceed?" ) );
        println!( "yes/no" , askyesno ( "Spam" , "Got it?" ) );
        println!( "yes/no/cancel" , askyesnocancel ( "Spam" , "Want it?" ) );
        println!( "try again" , askretrycancel ( "Spam" , "Try again?" ) );
}

