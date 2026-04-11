//! config.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::configparser::{ConfigParser};
// use std::fs;
// use crate::tkinter::{Font};
// use crate::idlelib;
// use crate::zlib::{crc32};
// use crate::unittest::{main};

pub struct InvalidConfigType {
    pub file: String, // TODO: infer type
    pub config_types: String, // TODO: infer type
    pub defaultCfg: String, // TODO: infer type
    pub userCfg: String, // TODO: infer type
    pub cfg: String, // TODO: infer type
    pub userdir: String, // TODO: infer type
    pub pages: String, // TODO: infer type
}

impl InvalidConfigType {
    pub fn _warn(&self, msg: &str, key: &str) {
        key = ( msg , ) + key;
        if key !in _warned {
        // try {
        println!( msg , file = sys . stderr );
        // } catch  OSError  {
        // pass
        _warned . add ( key );
        class ConfigChanges ( dict ) ;
        "Manage a user's proposed configuration option changes.

    Names used across multiple methods:
        page -- one of the 4 top-level dicts representing a
                .idlerc/config-x.cfg file.
        config_type -- name of a page.
        section -- a section within a page/file.
        option -- name of an option within a section.
        value -- value for the option.

    Methods
        add_option: Add option && value to changes.
        save_option: Save option && value to config parser.
        save_all: Save all the changes to the config parser && file.
        delete_section: If section exists,
                        delete from changes, userCfg, && file.
        clear: Clear all changes by clearing each page.
    ";
        pub fn __init__ ( self )  {
        "Create a page for each configuration file";
        self . pages = [ ];
        for config_type in idleConf . config_types .iter() {
        self [ config_type ] = { };
        self . pages . append ( self [ config_type ] );
        pub fn add_option ( &self, config_type , section , item , value )  {
        "Add item/value pair for config_type && section.";
        page = self [ config_type ];
        value = str ( value );
        if section !in page {
        page [ section ] = { };
        page [ section ] [ item ] = value;
        @ staticmethod;
        pub fn save_option ( config_type , section , item , value )  {
        "Return true if the configuration value was added || changed.

        Helper for save_all.
        ";
        if idleConf . defaultCfg [ config_type ] . has_option ( section , item ) {
        if idleConf . defaultCfg [ config_type ] . Get ( section , item ) == value {
        return  idleConf . userCfg [ config_type ] . RemoveOption ( section , item );
        return  idleConf . userCfg [ config_type ] . SetOption ( section , item , value );
        pub fn save_all ( self )  {
        "Save configuration changes to the user config file.

        Clear self in preparation for additional changes.
        Return changed for testing.
        ";
        idleConf . userCfg [ "main" ] . Save ( );
        changed = false;
        for config_type in self .iter() {
        cfg_type_changed = false;
        page = self [ config_type ];
        for section in page .iter() {
        if section == "HelpFiles" {
        idleConf . userCfg [ "main" ] . remove_section ( "HelpFiles" );
        cfg_type_changed = true;
        for item , value in page [ section ] . items ( ) .iter() {
        if self . save_option ( config_type , section , item , value ) {
        cfg_type_changed = true;
        if cfg_type_changed {
        idleConf . userCfg [ config_type ] . Save ( );
        changed = true;
        for config_type in [ "keys" , "highlight" ] .iter() {
        idleConf . userCfg [ config_type ] . Save ( );
        self . clear ( );
        return  changed;
        pub fn delete_section ( &self, config_type , section )  {
        "Delete a section from self, userCfg, && file.

        Used to delete custom themes && keysets.
        ";
        if section in self [ config_type ] {
        del self [ config_type ] [ section ];
        configpage = idleConf . userCfg [ config_type ];
        configpage . remove_section ( section );
        configpage . Save ( );
        pub fn clear ( self )  {
        "Clear all 4 pages.

        Called in save_all after saving to idleConf.
        XXX Mark window *title* when there are changes; unmark here.
        ";
        for page in self . pages .iter() {
        page . clear ( );
        pub fn _dump ( )  {
        from zlib import crc32;
        line , crc = 0 , 0;
        pub fn sprint ( obj )  {
        nonlocal line , crc;
        txt = str ( obj );
        line + = 1;
        crc = crc32 ( txt . encode ( encoding = "utf-8" ) , crc );
        println!( txt );
        pub fn dumpCfg ( cfg )  {
        println!( "\n" , cfg , "\n" );
        for key in sorted ( cfg ) .iter() {
        sections = cfg [ key ] . sections ( );
        sprint ( key );
        sprint ( sections );
        for section in sections .iter() {
        options = cfg [ key ] . options ( section );
        sprint ( section );
        sprint ( options );
        for option in options .iter() {
        sprint ( option + " = " + cfg [ key ] . Get ( section , option ) );
        dumpCfg ( idleConf . defaultCfg );
        dumpCfg ( idleConf . userCfg );
        println!( "\nlines = " , line , ", crc = " , crc , sep = "" );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_config" , verbosity = 2 , exit = false );
        _dump ( );
    }

}

