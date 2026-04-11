//! zzdummy.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::idlelib::{idleConf};
// use crate::functools::{wraps};
// use crate::unittest;

pub fn format_selection(format_line: &str) {
        "Apply a formatting function to all of the selected lines.";
        @ wraps ( format_line );
        pub fn apply ( &self, event = None /* Option */ )  {
        head , tail , chars , lines = self . formatter . get_region ( );
        for pos in range ( len ( lines ) - 1 ) .iter() {
        line = lines [ pos ];
        lines [ pos ] = format_line ( self , line );
        self . formatter . set_region ( head , tail , chars , lines );
        return  "break";
        return  apply;
        class ZzDummy ;
        "Prepend || remove initial text from selected lines.";
        menudefs = [;
        ( "format" , [;
        ( "Z in" , "<<z-in>>" ) ,;
        ( "Z out" , "<<z-out>>" ) ,;
        ] );
        ];
        pub fn __init__ ( &self, editwin )  {
        "Initialize the settings for this extension.";
        self . editwin = editwin;
        self . text = editwin . text;
        self . formatter = editwin . fregion;
        @ classmethod;
        pub fn reload ( cls )  {
        "Load class variables from config.";
        cls . ztext = idleConf . GetOption ( "extensions" , "ZzDummy" , "z-text" );
        @ format_selection;
        pub fn z_in_event ( &self, line )  {
        "Insert text at the beginning of each selected line.

        This == bound to the <<z-in>> virtual event when the extensions
        are loaded.
        ";
        return  f "{self.ztext}{line}";
        @ format_selection;
        pub fn z_out_event ( &self, line )  {
        "Remove specific text from the beginning of each selected line.

        This == bound to the <<z-out>> virtual event when the extensions
        are loaded.
        ";
        zlength = 0 if !line . startswith ( self . ztext ) else len ( self . ztext );
        return  line [ zlength : ];
        ZzDummy . reload ( );
        fn main() {
        import unittest;
        unittest . main ( "idlelib.idle_test.test_zzdummy" , verbosity = 2 , exit = false );
}

