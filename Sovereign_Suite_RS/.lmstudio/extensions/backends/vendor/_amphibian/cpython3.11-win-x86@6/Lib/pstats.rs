//! pstats.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use std::time;
// use regex::Regex;
// use crate::enum::{StrEnum, _simple_enum};
// use crate::functools::{cmp_to_key};
// use crate::dataclasses::{dataclass};
// use /* typing */::{Dict};
// use crate::cmd;
// use crate::readline;

pub const __all__: &str = ["Stats" ,"SortKey" ,"FunctionProfile" ,"StatsProfile" ];
pub struct SortKey {
    pub stream: String, // TODO: infer type
    pub all_callees: String, // TODO: infer type
    pub files: String, // TODO: infer type
    pub fcn_list: String, // TODO: infer type
    pub total_tt: String, // TODO: infer type
    pub total_calls: String, // TODO: infer type
    pub prim_calls: String, // TODO: infer type
    pub max_name_len: String, // TODO: infer type
    pub top_level: String, // TODO: infer type
    pub stats: String, // TODO: infer type
    pub sort_arg_dict: String, // TODO: infer type
    pub sort_type: String, // TODO: infer type
    pub comp_select_list: String, // TODO: infer type
    pub prompt: String, // TODO: infer type
}

impl SortKey {
}

pub const unsafe_hash: f64 = True );
pub struct FunctionProfile {
    pub stream: String, // TODO: infer type
    pub all_callees: String, // TODO: infer type
    pub files: String, // TODO: infer type
    pub fcn_list: String, // TODO: infer type
    pub total_tt: String, // TODO: infer type
    pub total_calls: String, // TODO: infer type
    pub prim_calls: String, // TODO: infer type
    pub max_name_len: String, // TODO: infer type
    pub top_level: String, // TODO: infer type
    pub stats: String, // TODO: infer type
    pub sort_arg_dict: String, // TODO: infer type
    pub sort_type: String, // TODO: infer type
    pub comp_select_list: String, // TODO: infer type
    pub prompt: String, // TODO: infer type
}

impl FunctionProfile {
}

pub const unsafe_hash: f64 = True );
pub struct StatsProfile {
    pub stream: String, // TODO: infer type
    pub all_callees: String, // TODO: infer type
    pub files: String, // TODO: infer type
    pub fcn_list: String, // TODO: infer type
    pub total_tt: String, // TODO: infer type
    pub total_calls: String, // TODO: infer type
    pub prim_calls: String, // TODO: infer type
    pub max_name_len: String, // TODO: infer type
    pub top_level: String, // TODO: infer type
    pub stats: String, // TODO: infer type
    pub sort_arg_dict: String, // TODO: infer type
    pub sort_type: String, // TODO: infer type
    pub comp_select_list: String, // TODO: infer type
    pub prompt: String, // TODO: infer type
}

impl StatsProfile {
}

pub struct Stats {
    pub stream: String, // TODO: infer type
    pub all_callees: String, // TODO: infer type
    pub files: String, // TODO: infer type
    pub fcn_list: String, // TODO: infer type
    pub total_tt: String, // TODO: infer type
    pub total_calls: String, // TODO: infer type
    pub prim_calls: String, // TODO: infer type
    pub max_name_len: String, // TODO: infer type
    pub top_level: String, // TODO: infer type
    pub stats: String, // TODO: infer type
    pub sort_arg_dict: String, // TODO: infer type
    pub sort_type: String, // TODO: infer type
    pub comp_select_list: String, // TODO: infer type
    pub prompt: String, // TODO: infer type
}

impl Stats {
}

pub const old_top: f64 = self . top_level;
pub struct TupleComp {
    pub comp_select_list: String, // TODO: infer type
    pub prompt: String, // TODO: infer type
    pub stats: String, // TODO: infer type
    pub stream: String, // TODO: infer type
}

impl TupleComp {
}

pub fn func_strip_path(func_name: &str) {
        filename , line , name = func_name;
        return  os . path . basename ( filename ) , line , name;
        pub fn func_get_function_name ( func )  {
        return  func [ 2 ];
        pub fn func_std_string ( func_name )  {
        if func_name [ { : 2 ] == ( "~" , 0 ) ; }
        name = func_name [ 2 ];
        if name . startswith ( "<" ) && name . endswith ( ">" ) {
        return  "{%s}" % name [ 1 : -1 ];
        } else {
        return  name;
        } else {
        return  "%s:%d(%s)" % func_name;
        pub fn add_func_stats ( target , source )  {
        "Add together all the stats for two profile entries.";
        cc , nc , tt , ct , callers = source;
        t_cc , t_nc , t_tt , t_ct , t_callers = target;
        return  ( cc + t_cc , nc + t_nc , tt + t_tt , ct + t_ct ,;
        add_callers ( t_callers , callers ) );
        pub fn add_callers ( target , source )  {
        "Combine two caller lists in a single list.";
        new_callers = { };
        for func , caller in target . items ( ) .iter() {
        new_callers [ func ] = caller;
        for func , caller in source . items ( ) .iter() {
        if func in new_callers {
        if isinstance ( caller , tuple ) {
        new_callers vec![ func ] = tuple ( i + j.iter().map(|i , j| zip ( caller , new_callers vec![ func ] ) );
        } else {
        new_callers [ func ] + = caller;
        } else {
        new_callers [ func ] = caller;
        return  new_callers;
        pub fn count_calls ( callers )  {
        "Sum the caller statistics to get total number of calls received.";
        nc = 0;
        for calls in callers . values ( ) .iter() {
        nc + = calls;
        return  nc;
        pub fn f8 ( x )  {
        return  "%8.3f" % x;
        fn main() {
        import cmd;
        // try {
        import readline;
        // } catch  ImportError  {
        // pass
        class ProfileBrowser ( cmd . Cmd ) ;
        pub fn __init__ ( &self, profile = None /* Option */ )  {
        cmd . Cmd . __init__ ( self );
        self . prompt = "% ";
        self . stats = None /* Option */;
        self . stream = sys . stdout;
        if profile is !None /* Option */ {
        self . do_read ( profile );
        pub fn generic ( &self, fn , line )  {
        args = line . split ( );
        processed = [ ];
        for term in args .iter() {
        // try {
        processed . append ( int ( term ) );
        continue;
        // } catch  ValueError  {
        // pass
        // try {
        frac = float ( term );
        if frac > 1 || frac < 0 {
        println!( "Fraction argument must be in [0, 1]" , file = self . stream );
        continue;
        processed . append ( frac );
        continue;
        // } catch  ValueError  {
        // pass
        processed . append ( term );
        if self . stats {
        getattr ( self . stats , fn ) ( * processed );
        } else {
        println!( "No statistics object is loaded." , file = self . stream );
        return  0;
        pub fn generic_help ( self )  {
        println!( "Arguments may be:" , file = self . stream );
        println!( "* An integer maximum number of entries to print." , file = self . stream );
        println!( "* A decimal fractional number between 0 && 1, controlling" , file = self . stream );
        println!( "  what fraction of selected entries to print." , file = self . stream );
        println!( "* A regular expression; only entries with function names" , file = self . stream );
        println!( "  that match it are printed." , file = self . stream );
        pub fn do_add ( &self, line )  {
        if self . stats {
        // try {
        self . stats . add ( line );
        // } catch  OSError as e  {
        println!( "Failed to load statistics for %s: %s" % ( line , e ) , file = self . stream );
        } else {
        println!( "No statistics object is loaded." , file = self . stream );
        return  0;
        pub fn help_add ( self )  {
        println!( "Add profile info from given file to current statistics object." , file = self . stream );
        pub fn do_callees ( &self, line )  {
        return  self . generic ( "print_callees" , line );
        pub fn help_callees ( self )  {
        println!( "Print callees statistics from the current stat object." , file = self . stream );
        self . generic_help ( );
        pub fn do_callers ( &self, line )  {
        return  self . generic ( "print_callers" , line );
        pub fn help_callers ( self )  {
        println!( "Print callers statistics from the current stat object." , file = self . stream );
        self . generic_help ( );
        pub fn do_EOF ( &self, line )  {
        println!( "" , file = self . stream );
        return  1;
        pub fn help_EOF ( self )  {
        println!( "Leave the profile browser." , file = self . stream );
        pub fn do_quit ( &self, line )  {
        return  1;
        pub fn help_quit ( self )  {
        println!( "Leave the profile browser." , file = self . stream );
        pub fn do_read ( &self, line )  {
        if line {
        // try {
        self . stats = Stats ( line );
        // } catch  OSError as err  {
        println!( err . args [ 1 ] , file = self . stream );
        return;
        // } catch  Exception as err  {
        println!( err . __class__ . __name__ + ":" , err , file = self . stream );
        return;
        self . prompt = line + "% ";
        } else if len ( self . prompt ) > 2 {
        line = self . prompt [ : -2 ];
        self . do_read ( line );
        } else {
        println!( "No statistics object is current -- cannot reload." , file = self . stream );
        return  0;
        pub fn help_read ( self )  {
        println!( "Read in profile data from a specified file." , file = self . stream );
        println!( "Without argument, reload the current file." , file = self . stream );
        pub fn do_reverse ( &self, line )  {
        if self . stats {
        self . stats . reverse_order ( );
        } else {
        println!( "No statistics object is loaded." , file = self . stream );
        return  0;
        pub fn help_reverse ( self )  {
        println!( "Reverse the sort order of the profiling report." , file = self . stream );
        pub fn do_sort ( &self, line )  {
        if !self . stats {
        println!( "No statistics object is loaded." , file = self . stream );
        return;
        abbrevs = self . stats . get_sort_arg_defs ( );
        if line && all ( ( x in abbrevs ) for x in line . split ( ) ) {
        self . stats . sort_stats ( * line . split ( ) );
        } else {
        println!( "Valid sort keys (unique prefixes are accepted):" , file = self . stream );
        for ( key , value ) in Stats . sort_arg_dict_default . items ( ) .iter() {
        println!( "%s -- %s" % ( key , value [ 1 ] ) , file = self . stream );
        return  0;
        pub fn help_sort ( self )  {
        println!( "Sort profile data according to specified keys." , file = self . stream );
        println!( "(Typing `sort' without arguments lists valid keys.)" , file = self . stream );
        pub fn complete_sort ( &self, text , * args )  {
        return  [ a for a in Stats . sort_arg_dict_default if a . startswith ( text ) ];
        pub fn do_stats ( &self, line )  {
        return  self . generic ( "print_stats" , line );
        pub fn help_stats ( self )  {
        println!( "Print statistics from the current stat object." , file = self . stream );
        self . generic_help ( );
        pub fn do_strip ( &self, line )  {
        if self . stats {
        self . stats . strip_dirs ( );
        } else {
        println!( "No statistics object is loaded." , file = self . stream );
        pub fn help_strip ( self )  {
        println!( "Strip leading path information from filenames in the report." , file = self . stream );
        pub fn help_help ( self )  {
        println!( "Show help for a given command." , file = self . stream );
        pub fn postcmd ( &self, stop , line )  {
        if stop {
        return  stop;
        return;
        if len ( sys . argv ) > 1 {
        initprofile = sys . argv [ 1 ];
        } else {
        initprofile = None /* Option */;
        // try {
        browser = ProfileBrowser ( initprofile );
        for profile in sys . argv [ 2 : ] .iter() {
        browser . do_add ( profile );
        println!( "Welcome to the profile statistics browser." , file = browser . stream );
        browser . cmdloop ( );
        println!( "Goodbye." , file = browser . stream );
        // } catch  KeyboardInterrupt  {
        // pass
}

