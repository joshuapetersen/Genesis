//! bdist.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::Command;
// use crate::get_platform;
// use crate::distutils::{FancyGetopt};

pub fn show_formats() {
        "Print list of available formats (arguments to "--format" option).
    ";
        from distutils . fancy_getopt import FancyGetopt;
        formats = [ ];
        for format in bdist . format_commands .iter() {
        formats . append ( ( "formats=" + format , None /* Option */ ,;
        bdist . format_command [ format ] [ 1 ] ) );
        pretty_printer = FancyGetopt ( formats );
        pretty_printer . print_help ( "List of available distribution formats:" );
        class bdist ( Command ) ;
        description = "create a built (binary) distribution";
        user_options = [ ( "bdist-base=" , "b" ,;
        "temporary directory for creating built distributions" ) ,;
        ( "plat-name=" , "p" ,;
        "platform name to embed in generated filenames ";
        "(default: %s)" % get_platform ( ) ) ,;
        ( "formats=" , None /* Option */ ,;
        "formats for distribution (comma-separated list)" ) ,;
        ( "dist-dir=" , "d" ,;
        "directory to put final built distributions in ";
        "[default: dist]" ) ,;
        ( "skip-build" , None /* Option */ ,;
        "skip rebuilding everything (for testing/debugging)" ) ,;
        ( "owner=" , "u" ,;
        "Owner name used when creating a tar file";
        " [default: current user]" ) ,;
        ( "group=" , "g" ,;
        "Group name used when creating a tar file";
        " [default: current group]" ) ,;
        ];
        boolean_options = [ "skip-build" ];
        help_options = [;
        ( "help-formats" , None /* Option */ ,;
        "lists available distribution formats" , show_formats ) ,;
        ];
        no_format_option = ( "bdist_rpm" , );
        default_format = { "posix" : "gztar" ,;
        "nt" : "zip" };
        format_commands = [ "rpm" , "gztar" , "bztar" , "xztar" , "ztar" , "tar" , "zip" ];
        format_command = { "rpm" : ( "bdist_rpm" , "RPM distribution" ) ,;
        "gztar" : ( "bdist_dumb" , "gzip'ed tar file" ) ,;
        "bztar" : ( "bdist_dumb" , "bzip2'ed tar file" ) ,;
        "xztar" : ( "bdist_dumb" , "xz'ed tar file" ) ,;
        "ztar" : ( "bdist_dumb" , "compressed tar file" ) ,;
        "tar" : ( "bdist_dumb" , "tar file" ) ,;
        "zip" : ( "bdist_dumb" , "ZIP file" ) ,;
        };
        pub fn initialize_options ( self )  {
        self . bdist_base = None /* Option */;
        self . plat_name = None /* Option */;
        self . formats = None /* Option */;
        self . dist_dir = None /* Option */;
        self . skip_build = 0;
        self . group = None /* Option */;
        self . owner = None /* Option */;
        pub fn finalize_options ( self )  {
        if self . plat_name is None /* Option */ {
        if self . skip_build {
        self . plat_name = get_platform ( );
        } else {
        self . plat_name = self . get_finalized_command ( "build" ) . plat_name;
        if self . bdist_base is None /* Option */ {
        build_base = self . get_finalized_command ( "build" ) . build_base;
        self . bdist_base = os . path . join ( build_base ,;
        "bdist." + self . plat_name );
        self . ensure_string_list ( "formats" );
        if self . formats is None /* Option */ {
        // try {
        self . formats = [ self . default_format [ os . name ] ];
        // } catch  KeyError  {
        panic!("DistutilsPlatformError (");
        "don't know how to create built distributions ";
        "on platform %s" % os . name );
        if self . dist_dir is None /* Option */ {
        self . dist_dir = "dist";
        pub fn run ( self )  {
        commands = [ ];
        for format in self . formats .iter() {
        // try {
        commands . append ( self . format_command [ format ] [ 0 ] );
        // } catch  KeyError  {
        panic!("DistutilsOptionError ( "invalid format '%s'" % format )");
        for i in range ( len ( self . formats ) ) .iter() {
        cmd_name = commands [ i ];
        sub_cmd = self . reinitialize_command ( cmd_name );
        if cmd_name !in self . no_format_option {
        sub_cmd . format = self . formats [ i ];
        if cmd_name == "bdist_dumb" {
        sub_cmd . owner = self . owner;
        sub_cmd . group = self . group;
        if cmd_name in commands [ i + 1 { : ] ; }
        sub_cmd . keep_temp = 1;
        self . run_command ( cmd_name );
}

