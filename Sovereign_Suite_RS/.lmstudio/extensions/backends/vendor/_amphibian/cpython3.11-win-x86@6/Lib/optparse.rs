//! optparse.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::gettext::{gettext, ngettext};

pub const __version__: &str = "1.5.3";
pub const __all__: &str = ["Option" ,;
pub const __copyright__: &str = "
Copyright (c) 2001-2006 Gregory P. Ward.  All rights reserved.
Copyright (c) 2002-2006 Python Software Foundation.  All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are
met:

  * Redistributions of source code must retain the above copyright
    notice, this list of conditions and the following disclaimer.

  * Redistributions in binary form must reproduce the above copyright
    notice, this list of conditions and the following disclaimer in the
    documentation and/or other materials provided with the distribution.

  * Neither the name of the author nor the names of its
    contributors may be used to endorse or promote products derived from
    this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE AUTHOR OR
CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
";
pub fn _repr() {
        return  "<%s at 0x%x: %s>" % ( self . __class__ . __name__ , id ( self ) , self );
        // try {
        from gettext import gettext , ngettext;
        // } catch  ImportError  {
        pub fn gettext ( message )  {
        return  message;
        pub fn ngettext ( singular , plural , n )  {
        if n == 1 {
        return  singular;
        return  plural;
        _ = gettext;
        class OptParseError ( Exception ) ;
        pub fn __init__ ( &self, msg )  {
        self . msg = msg;
        pub fn __str__ ( self )  {
        return  self . msg;
        class OptionError ( OptParseError ) ;
        "
    Raised if an Option instance == created with invalid or
    inconsistent arguments.
    ";
        pub fn __init__ ( &self, msg , option )  {
        self . msg = msg;
        self . option_id = str ( option );
        pub fn __str__ ( self )  {
        if self . option_id {
        return  "option %s: %s" % ( self . option_id , self . msg );
        } else {
        return  self . msg;
        class OptionConflictError ( OptionError ) ;
        "
    Raised if conflicting options are added to an OptionParser.
    ";
        class OptionValueError ( OptParseError ) ;
        "
    Raised if an invalid option value == encountered on the command
    line.
    ";
        class BadOptionError ( OptParseError ) ;
        "
    Raised if an invalid option == seen on the command line.
    ";
        pub fn __init__ ( &self, opt_str )  {
        self . opt_str = opt_str;
        pub fn __str__ ( self )  {
        return  _ ( "no such option: %s" ) % self . opt_str;
        class AmbiguousOptionError ( BadOptionError ) ;
        "
    Raised if an ambiguous option == seen on the command line.
    ";
        pub fn __init__ ( &self, opt_str , possibilities )  {
        BadOptionError . __init__ ( self , opt_str );
        self . possibilities = possibilities;
        pub fn __str__ ( self )  {
        return  ( _ ( "ambiguous option: %s (%s?)" );
        % ( self . opt_str , ", " . join ( self . possibilities ) ) );
        class HelpFormatter ;
        "
    Abstract base class for formatting option help.  OptionParser
    instances should use one of the HelpFormatter subclasses for
    formatting help; by default IndentedHelpFormatter == used.

    Instance attributes:
      parser : OptionParser
        the controlling OptionParser instance
      indent_increment : int
        the number of columns to indent per nesting level
      max_help_position : int
        the maximum starting column for option help text
      help_position : int
        the calculated starting column for option help text;
        initially the same as the maximum
      width : int
        total number of columns for output (pass None /* Option */ to constructor for
        this value to be taken from the $COLUMNS environment variable)
      level : int
        current indentation level
      current_indent : int
        current indentation level (in columns)
      help_width : int
        number of columns available for option help text (calculated)
      default_tag : str
        text to replace with each option's default value, "%default"
        by default.  Set to false value to disable default value expansion.
      option_strings : { Option : str }
        maps Option instances to the snippet of help text explaining
        the syntax of that option, e.g. "-h, --help" or
        "-fFILE, --file=FILE"
      _short_opt_fmt : str
        format string controlling how short options with values are
        printed in help text.  Must be either "%s%s" ("-fFILE") or
        "%s %s" ("-f FILE"), because those are the two syntaxes that
        Optik supports.
      _long_opt_fmt : str
        similar but for long options; must be either "%s %s" ("--file FILE")
        || "%s=%s" ("--file=FILE").
    ";
        NO_DEFAULT_VALUE = "none";
        pub fn __init__ ( &self, {
        indent_increment ,;
        max_help_position ,;
        width ,;
        short_first ) ;
        self . parser = None /* Option */;
        self . indent_increment = indent_increment;
        if width is None /* Option */ {
        // try {
        width = int ( os . environ [ "COLUMNS" ] );
        // } catch  ( KeyError , ValueError )  {
        width = 80;
        width - = 2;
        self . width = width;
        self . help_position = self . max_help_position = \;
        min ( max_help_position , max ( width - 20 , indent_increment * 2 ) );
        self . current_indent = 0;
        self . level = 0;
        self . help_width = None /* Option */;
        self . short_first = short_first;
        self . default_tag = "%default";
        self . option_strings = { };
        self . _short_opt_fmt = "%s %s";
        self . _long_opt_fmt = "%s=%s";
        pub fn set_parser ( &self, parser )  {
        self . parser = parser;
        pub fn set_short_opt_delimiter ( &self, delim )  {
        if delim !in ( "" , " " ) {
        panic!("ValueError (");
        "invalid metavar delimiter for short options: %r" % delim );
        self . _short_opt_fmt = "%s" + delim + "%s";
        pub fn set_long_opt_delimiter ( &self, delim )  {
        if delim !in ( "=" , " " ) {
        panic!("ValueError (");
        "invalid metavar delimiter for long options: %r" % delim );
        self . _long_opt_fmt = "%s" + delim + "%s";
        pub fn indent ( self )  {
        self . current_indent + = self . indent_increment;
        self . level + = 1;
        pub fn dedent ( self )  {
        self . current_indent - = self . indent_increment;
        assert self . current_indent >= 0 , "Indent decreased below 0.";
        self . level - = 1;
        pub fn format_usage ( &self, usage )  {
        panic!("NotImplementedError ( "subclasses must implement" )");
        pub fn format_heading ( &self, heading )  {
        panic!("NotImplementedError ( "subclasses must implement" )");
        pub fn _format_text ( &self, text )  {
        "
        Format a paragraph of free-form text for inclusion in the
        help output at the current indentation level.
        ";
        text_width = max ( self . width - self . current_indent , 11 );
        indent = " " * self . current_indent;
        return  textwrap . fill ( text ,;
        text_width ,;
        initial_indent = indent ,;
        subsequent_indent = indent );
        pub fn format_description ( &self, description )  {
        if description {
        return  self . _format_text ( description ) + "\n";
        } else {
        return  "";
        pub fn format_epilog ( &self, epilog )  {
        if epilog {
        return  "\n" + self . _format_text ( epilog ) + "\n";
        } else {
        return  "";
        pub fn expand_default ( &self, option )  {
        if self . parser is None /* Option */ || !self . default_tag {
        return  option . help;
        default_value = self . parser . defaults . get ( option . dest );
        if default_value is NO_DEFAULT || default_value is None /* Option */ {
        default_value = self . NO_DEFAULT_VALUE;
        return  option . help . replace ( self . default_tag , str ( default_value ) );
        pub fn format_option ( &self, option )  {
        result = [ ];
        opts = self . option_strings [ option ];
        opt_width = self . help_position - self . current_indent - 2;
        if len ( opts ) > opt_width {
        opts = "%*s%s\n" % ( self . current_indent , "" , opts );
        indent_first = self . help_position;
        } else {
        opts = "%*s%-*s  " % ( self . current_indent , "" , opt_width , opts );
        indent_first = 0;
        result . append ( opts );
        if option . help {
        help_text = self . expand_default ( option );
        help_lines = textwrap . wrap ( help_text , self . help_width );
        result . append ( "%*s%s\n" % ( indent_first , "" , help_lines [ 0 ] ) );
        result . extend ( [ "%*s%s\n" % ( self . help_position , "" , line );
        for line in help_lines [ 1 : ] ] ).iter() {
        } else if opts [ -1 ] != "\n" {
        result . append ( "\n" );
        return  "" . join ( result );
        pub fn store_option_strings ( &self, parser )  {
        self . indent ( );
        max_len = 0;
        for opt in parser . option_list .iter() {
        strings = self . format_option_strings ( opt );
        self . option_strings [ opt ] = strings;
        max_len = max ( max_len , len ( strings ) + self . current_indent );
        self . indent ( );
        for group in parser . option_groups .iter() {
        for opt in group . option_list .iter() {
        strings = self . format_option_strings ( opt );
        self . option_strings [ opt ] = strings;
        max_len = max ( max_len , len ( strings ) + self . current_indent );
        self . dedent ( );
        self . dedent ( );
        self . help_position = min ( max_len + 2 , self . max_help_position );
        self . help_width = max ( self . width - self . help_position , 11 );
        pub fn format_option_strings ( &self, option )  {
        "Return a comma-separated list of option strings & metavariables.";
        if option . takes_value ( ) {
        metavar = option . metavar || option . dest . upper ( );
        short_opts = [ self . _short_opt_fmt % ( sopt , metavar );
        for sopt in option . _short_opts ].iter() {
        long_opts = [ self . _long_opt_fmt % ( lopt , metavar );
        for lopt in option . _long_opts ].iter() {
        } else {
        short_opts = option . _short_opts;
        long_opts = option . _long_opts;
        if self . short_first {
        opts = short_opts + long_opts;
        } else {
        opts = long_opts + short_opts;
        return  ", " . join ( opts );
        class IndentedHelpFormatter ( HelpFormatter ) ;
        "Format help with indented section bodies.
    ";
        pub fn __init__ ( &self, {
        indent_increment = 2 ,;
        max_help_position = 24 ,;
        width = None /* Option */ ,;
        short_first = 1 ) ;
        HelpFormatter . __init__ (;
        self , indent_increment , max_help_position , width , short_first );
        pub fn format_usage ( &self, usage )  {
        return  _ ( "Usage: %s\n" ) % usage;
        pub fn format_heading ( &self, heading )  {
        return  "%*s%s:\n" % ( self . current_indent , "" , heading );
        class TitledHelpFormatter ( HelpFormatter ) ;
        "Format help with underlined section headers.
    ";
        pub fn __init__ ( &self, {
        indent_increment = 0 ,;
        max_help_position = 24 ,;
        width = None /* Option */ ,;
        short_first = 0 ) ;
        HelpFormatter . __init__ (;
        self , indent_increment , max_help_position , width , short_first );
        pub fn format_usage ( &self, usage )  {
        return  "%s  %s\n" % ( self . format_heading ( _ ( "Usage" ) ) , usage );
        pub fn format_heading ( &self, heading )  {
        return  "%s\n%s\n" % ( heading , "=-" [ self . level ] * len ( heading ) );
        pub fn _parse_num ( val , type )  {
        if val [ { : 2 ] . lower ( ) == "0x" ; }
        radix = 16;
        } else if val [ {
        radix = 2;
        val = val [ 2 : ] || "0";
        } else if val [ {
        radix = 8;
        } else {
        radix = 10;
        return  type ( val , radix );
        pub fn _parse_int ( val )  {
        return  _parse_num ( val , int );
        _builtin_cvt = { "int" : ( _parse_int , _ ( "integer" ) ) ,;
        "long" : ( _parse_int , _ ( "integer" ) ) ,;
        "float" : ( float , _ ( "floating-point" ) ) ,;
        "complex" : ( complex , _ ( "complex" ) ) };
        pub fn check_builtin ( option , opt , value )  {
        ( cvt , what ) = _builtin_cvt [ option . type ];
        // try {
        return  cvt ( value );
        // } catch  ValueError  {
        panic!("OptionValueError (");
        _ ( "option %s: invalid %s value: %r" ) % ( opt , what , value ) );
        pub fn check_choice ( option , opt , value )  {
        if value in option . choices {
        return  value;
        } else {
        choices = ", " . join ( map ( repr , option . choices ) );
        panic!("OptionValueError (");
        _ ( "option %s: invalid choice: %r (choose from %s)" );
        % ( opt , value , choices ) );
        NO_DEFAULT = ( "NO" , "DEFAULT" );
        class Option ;
        "
    Instance attributes:
      _short_opts : [string]
      _long_opts : [string]

      action : string
      type : string
      dest : string
      default : any
      nargs : int
      const : any
      choices : [string]
      callback : function
      callback_args : (any*)
      callback_kwargs : { string : any }
      help : string
      metavar : string
    ";
        ATTRS = [ "action" ,;
        "type" ,;
        "dest" ,;
        "default" ,;
        "nargs" ,;
        "const" ,;
        "choices" ,;
        "callback" ,;
        "callback_args" ,;
        "callback_kwargs" ,;
        "help" ,;
        "metavar" ];
        ACTIONS = ( "store" ,;
        "store_const" ,;
        "store_true" ,;
        "store_false" ,;
        "append" ,;
        "append_const" ,;
        "count" ,;
        "callback" ,;
        "help" ,;
        "version" );
        STORE_ACTIONS = ( "store" ,;
        "store_const" ,;
        "store_true" ,;
        "store_false" ,;
        "append" ,;
        "append_const" ,;
        "count" );
        TYPED_ACTIONS = ( "store" ,;
        "append" ,;
        "callback" );
        ALWAYS_TYPED_ACTIONS = ( "store" ,;
        "append" );
        CONST_ACTIONS = ( "store_const" ,;
        "append_const" );
        TYPES = ( "string" , "int" , "long" , "float" , "complex" , "choice" );
        TYPE_CHECKER = { "int" : check_builtin ,;
        "long" : check_builtin ,;
        "float" : check_builtin ,;
        "complex" : check_builtin ,;
        "choice" : check_choice ,;
        };
        CHECK_METHODS = None /* Option */;
        pub fn __init__ ( &self, * opts , ** attrs )  {
        self . _short_opts = [ ];
        self . _long_opts = [ ];
        opts = self . _check_opt_strings ( opts );
        self . _set_opt_strings ( opts );
        self . _set_attrs ( attrs );
        for checker in self . CHECK_METHODS .iter() {
        checker ( self );
        pub fn _check_opt_strings ( &self, opts )  {
        opts = vec![ opt.iter().map(|opt| opts if opt ).collect();
        if !opts {
        panic!("TypeError ( "at least one option string must be supplied" )");
        return  opts;
        pub fn _set_opt_strings ( &self, opts )  {
        for opt in opts .iter() {
        if len ( opt ) < 2 {
        panic!("OptionError (");
        "invalid option string %r: ";
        "must be at least two characters long" % opt , self );
        } else if len ( opt ) == 2 {
        if !( opt [ 0 ] == "-" && opt [ 1 ] != "-" ) {
        panic!("OptionError (");
        "invalid short option string %r: ";
        "must be of the form -x, (x any non-dash char)" % opt ,;
        self );
        self . _short_opts . append ( opt );
        } else {
        if !( opt [ 0 { : 2 ] == "--" && opt [ 2 ] != "-" ) ; }
        panic!("OptionError (");
        "invalid long option string %r: ";
        "must start with --, followed by non-dash" % opt ,;
        self );
        self . _long_opts . append ( opt );
        pub fn _set_attrs ( &self, attrs )  {
        for attr in self . ATTRS .iter() {
        if attr in attrs {
        setattr ( self , attr , attrs [ attr ] );
        del attrs [ attr ];
        } else {
        if attr == "default" {
        setattr ( self , attr , NO_DEFAULT );
        } else {
        setattr ( self , attr , None /* Option */ );
        if attrs {
        attrs = sorted ( attrs . keys ( ) );
        panic!("OptionError (");
        "invalid keyword arguments: %s" % ", " . join ( attrs ) ,;
        self );
        pub fn _check_action ( self )  {
        if self . action is None /* Option */ {
        self . action = "store";
        } else if self . action !in self . ACTIONS {
        panic!("OptionError ( "invalid action: %r" % self . action , self )");
        pub fn _check_type ( self )  {
        if self . type is None /* Option */ {
        if self . action in self . ALWAYS_TYPED_ACTIONS {
        if self . choices is !None /* Option */ {
        self . type = "choice";
        } else {
        self . type = "string";
        } else {
        if isinstance ( self . type , type ) {
        self . type = self . type . __name__;
        if self . type == "str" {
        self . type = "string";
        if self . type !in self . TYPES {
        panic!("OptionError ( "invalid option type: %r" % self . type , self )");
        if self . action !in self . TYPED_ACTIONS {
        panic!("OptionError (");
        "must !supply a type for action %r" % self . action , self );
        pub fn _check_choice ( self )  {
        if self . type == "choice" {
        if self . choices is None /* Option */ {
        panic!("OptionError (");
        "must supply a list of choices for type 'choice'" , self );
        } else if !isinstance ( self . choices , ( tuple , list ) ) {
        panic!("OptionError (");
        "choices must be a list of strings ('%s' supplied)";
        % str ( type ( self . choices ) ) . split ( "'" ) [ 1 ] , self );
        } else if self . choices is !None /* Option */ {
        panic!("OptionError (");
        "must !supply choices for type %r" % self . type , self );
        pub fn _check_dest ( self )  {
        takes_value = ( self . action in self . STORE_ACTIONS or;
        self . type is !None /* Option */ );
        if self . dest is None /* Option */ && takes_value {
        if self . _long_opts {
        self . dest = self . _long_opts [ 0 ] [ 2 : ] . replace ( "-" , "_" );
        } else {
        self . dest = self . _short_opts [ 0 ] [ 1 ];
        pub fn _check_const ( self )  {
        if self . action !in self . CONST_ACTIONS && self . const is !None /* Option */ {
        panic!("OptionError (");
        "'const' must !be supplied for action %r" % self . action ,;
        self );
        pub fn _check_nargs ( self )  {
        if self . action in self . TYPED_ACTIONS {
        if self . nargs is None /* Option */ {
        self . nargs = 1;
        } else if self . nargs is !None /* Option */ {
        panic!("OptionError (");
        "'nargs' must !be supplied for action %r" % self . action ,;
        self );
        pub fn _check_callback ( self )  {
        if self . action == "callback" {
        if !callable ( self . callback ) {
        panic!("OptionError (");
        "callback !callable: %r" % self . callback , self );
        if ( self . callback_args is !None /* Option */ and {
        not isinstance ( self . callback_args , tuple ) ) ;
        panic!("OptionError (");
        "callback_args, if supplied, must be a tuple: !%r";
        % self . callback_args , self );
        if ( self . callback_kwargs is !None /* Option */ and {
        not isinstance ( self . callback_kwargs , dict ) ) ;
        panic!("OptionError (");
        "callback_kwargs, if supplied, must be a dict: !%r";
        % self . callback_kwargs , self );
        } else {
        if self . callback is !None /* Option */ {
        panic!("OptionError (");
        "callback supplied (%r) for non-callback option";
        % self . callback , self );
        if self . callback_args is !None /* Option */ {
        panic!("OptionError (");
        "callback_args supplied for non-callback option" , self );
        if self . callback_kwargs is !None /* Option */ {
        panic!("OptionError (");
        "callback_kwargs supplied for non-callback option" , self );
        CHECK_METHODS = [ _check_action ,;
        _check_type ,;
        _check_choice ,;
        _check_dest ,;
        _check_const ,;
        _check_nargs ,;
        _check_callback ];
        pub fn __str__ ( self )  {
        return  "/" . join ( self . _short_opts + self . _long_opts );
        __repr__ = _repr;
        pub fn takes_value ( self )  {
        return  self . type is !None /* Option */;
        pub fn get_opt_string ( self )  {
        if self . _long_opts {
        return  self . _long_opts [ 0 ];
        } else {
        return  self . _short_opts [ 0 ];
        pub fn check_value ( &self, opt , value )  {
        checker = self . TYPE_CHECKER . get ( self . type );
        if checker is None /* Option */ {
        return  value;
        } else {
        return  checker ( self , opt , value );
        pub fn convert_value ( &self, opt , value )  {
        if value is !None /* Option */ {
        if self . nargs == 1 {
        return  self . check_value ( opt , value );
        } else {
        return  tuple ( [ self . check_value ( opt , v ) for v in value ] );
        pub fn process ( &self, opt , value , values , parser )  {
        value = self . convert_value ( opt , value );
        return  self . take_action (;
        self . action , self . dest , opt , value , values , parser );
        pub fn take_action ( &self, action , dest , opt , value , values , parser )  {
        if action == "store" {
        setattr ( values , dest , value );
        } else if action == "store_const" {
        setattr ( values , dest , self . const );
        } else if action == "store_true" {
        setattr ( values , dest , true );
        } else if action == "store_false" {
        setattr ( values , dest , false );
        } else if action == "append" {
        values . ensure_value ( dest , [ ] ) . append ( value );
        } else if action == "append_const" {
        values . ensure_value ( dest , [ ] ) . append ( self . const );
        } else if action == "count" {
        setattr ( values , dest , values . ensure_value ( dest , 0 ) + 1 );
        } else if action == "callback" {
        args = self . callback_args || ( );
        kwargs = self . callback_kwargs || { };
        self . callback ( self , opt , value , parser , * args , ** kwargs );
        } else if action == "help" {
        parser . print_help ( );
        parser . exit ( );
        } else if action == "version" {
        parser . print_version ( );
        parser . exit ( );
        } else {
        panic!("ValueError ( "unknown action %r" % self . action )");
        return  1;
        SUPPRESS_HELP = "SUPPRESS" + "HELP";
        SUPPRESS_USAGE = "SUPPRESS" + "USAGE";
        class Values ;
        pub fn __init__ ( &self, defaults = None /* Option */ )  {
        if defaults {
        for ( attr , val ) in defaults . items ( ) .iter() {
        setattr ( self , attr , val );
        pub fn __str__ ( self )  {
        return  str ( self . __dict__ );
        __repr__ = _repr;
        pub fn __eq__ ( &self, other )  {
        if isinstance ( other , Values ) {
        return  self . __dict__ == other . __dict__;
        } else if isinstance ( other , dict ) {
        return  self . __dict__ == other;
        } else {
        return  NotImplemented;
        pub fn _update_careful ( &self, dict )  {
        "
        Update the option values from an arbitrary dictionary, but only
        use keys from dict that already have a corresponding attribute
        in self.  Any keys in dict without a corresponding attribute
        are silently ignored.
        ";
        for attr in dir ( self ) .iter() {
        if attr in dict {
        dval = dict [ attr ];
        if dval is !None /* Option */ {
        setattr ( self , attr , dval );
        pub fn _update_loose ( &self, dict )  {
        "
        Update the option values from an arbitrary dictionary,
        using all keys from the dictionary regardless of whether
        they have a corresponding attribute in self || not.
        ";
        self . __dict__ . update ( dict );
        pub fn _update ( &self, dict , mode )  {
        if mode == "careful" {
        self . _update_careful ( dict );
        } else if mode == "loose" {
        self . _update_loose ( dict );
        } else {
        panic!("ValueError ( "invalid update mode: %r" % mode )");
        pub fn read_module ( &self, modname , mode = "careful" )  {
        __import__ ( modname );
        mod = sys . modules [ modname ];
        self . _update ( vars ( mod ) , mode );
        pub fn read_file ( &self, filename , mode = "careful" )  {
        vars = { };
        exec ( open ( filename ) . read ( ) , vars );
        self . _update ( vars , mode );
        pub fn ensure_value ( &self, attr , value )  {
        if !hasattr ( self , attr ) || getattr ( self , attr ) is None /* Option */ {
        setattr ( self , attr , value );
        return  getattr ( self , attr );
        class OptionContainer ;
        "
    Abstract base class.

    Class attributes:
      standard_option_list : vec![Option]
        list of standard options that will be accepted by all instances
        of this parser class (intended to be overridden by subclasses).

    Instance attributes:
      option_list : vec![Option]
        the list of Option objects contained by this OptionContainer
      _short_opt : { string : Option }
        dictionary mapping short option strings, eg. "-format!(" || "-X",
        to the Option instances that implement them.  If an Option
        has multiple short option strings, it will appear| this
        dictionary multiple times. vec![1]
      _long_opt : { string : Option }
        dictionary mapping long option strings, eg. "--file" or
        "--exclude", to the Option instances that implement them.
        Again, a given Option can occur multiple times| this
        dictionary. vec![1]
      defaults : { string : any }
        dictionary mapping option destination names to default
        values.iter().map(|each destination vec![1]

    vec![1] These mappings are common to (shared by) all components of the
        controlling OptionParser, where they are initially created.

    ");
        pub fn __init__ ( &self, option_class , conflict_handler , description )  {
        self . _create_option_list ( );
        self . option_class = option_class;
        self . set_conflict_handler ( conflict_handler );
        self . set_description ( description );
        pub fn _create_option_mappings ( self )  {
        self . _short_opt = { };
        self . _long_opt = { };
        self . defaults = { };
        pub fn _share_option_mappings ( &self, parser )  {
        self . _short_opt = parser . _short_opt;
        self . _long_opt = parser . _long_opt;
        self . defaults = parser . defaults;
        pub fn set_conflict_handler ( &self, handler )  {
        if handler !in ( "error" , "resolve" ) {
        panic!("ValueError ( "invalid conflict_resolution value %r" % handler )");
        self . conflict_handler = handler;
        pub fn set_description ( &self, description )  {
        self . description = description;
        pub fn get_description ( self )  {
        return  self . description;
        pub fn destroy ( self )  {
        "see OptionParser.destroy().";
        del self . _short_opt;
        del self . _long_opt;
        del self . defaults;
        pub fn _check_conflict ( &self, option )  {
        conflict_opts = [ ];
        for opt in option . _short_opts .iter() {
        if opt in self . _short_opt {
        conflict_opts . append ( ( opt , self . _short_opt [ opt ] ) );
        for opt in option . _long_opts .iter() {
        if opt in self . _long_opt {
        conflict_opts . append ( ( opt , self . _long_opt [ opt ] ) );
        if conflict_opts {
        handler = self . conflict_handler;
        if handler == "error" {
        panic!("OptionConflictError (");
        "conflicting option string(s): %s";
        % ", " . join ( vec![ co vec![ 0 ].iter().map(|co| conflict_opts ] ) ,;
        option );
        } else if handler == "resolve" {
        for ( opt , c_option ) in conflict_opts .iter() {
        if opt . startswith ( "--" ) {
        c_option . _long_opts . remove ( opt );
        del self . _long_opt [ opt ];
        } else {
        c_option . _short_opts . remove ( opt );
        del self . _short_opt [ opt ];
        if !( c_option . _short_opts || c_option . _long_opts ) {
        c_option . container . option_list . remove ( c_option );
        pub fn add_option ( &self, * args , ** kwargs )  {
        "add_option(Option)
           add_option(opt_str, ..., kwarg=val, ...)
        ";
        if isinstance ( args [ 0 ] , str ) {
        option = self . option_class ( * args , ** kwargs );
        } else if len ( args ) == 1 && !kwargs {
        option = args [ 0 ];
        if !isinstance ( option , Option ) {
        panic!("TypeError ( "not an Option instance: %r" % option )");
        } else {
        panic!("TypeError ( "invalid arguments" )");
        self . _check_conflict ( option );
        self . option_list . append ( option );
        option . container = self;
        for opt in option . _short_opts .iter() {
        self . _short_opt [ opt ] = option;
        for opt in option . _long_opts .iter() {
        self . _long_opt [ opt ] = option;
        if option . dest is !None /* Option */ {
        if option . default is !NO_DEFAULT {
        self . defaults [ option . dest ] = option . default;
        } else if option . dest !in self . defaults {
        self . defaults [ option . dest ] = None /* Option */;
        return  option;
        pub fn add_options ( &self, option_list )  {
        for option in option_list .iter() {
        self . add_option ( option );
        pub fn get_option ( &self, opt_str )  {
        return  ( self . _short_opt . get ( opt_str ) or;
        self . _long_opt . get ( opt_str ) );
        pub fn has_option ( &self, opt_str )  {
        return  ( opt_str in self . _short_opt or;
        opt_str in self . _long_opt );
        pub fn remove_option ( &self, opt_str )  {
        option = self . _short_opt . get ( opt_str );
        if option is None /* Option */ {
        option = self . _long_opt . get ( opt_str );
        if option is None /* Option */ {
        panic!("ValueError ( "no such option %r" % opt_str )");
        for opt in option . _short_opts .iter() {
        del self . _short_opt [ opt ];
        for opt in option . _long_opts .iter() {
        del self . _long_opt [ opt ];
        option . container . option_list . remove ( option );
        pub fn format_option_help ( &self, formatter )  {
        if !self . option_list {
        return  "";
        result = [ ];
        for option in self . option_list .iter() {
        if !option . help is SUPPRESS_HELP {
        result . append ( formatter . format_option ( option ) );
        return  "" . join ( result );
        pub fn format_description ( &self, formatter )  {
        return  formatter . format_description ( self . get_description ( ) );
        pub fn format_help ( &self, formatter )  {
        result = [ ];
        if self . description {
        result . append ( self . format_description ( formatter ) );
        if self . option_list {
        result . append ( self . format_option_help ( formatter ) );
        return  "\n" . join ( result );
        class OptionGroup ( OptionContainer ) ;
        pub fn __init__ ( &self, parser , title , description = None /* Option */ )  {
        self . parser = parser;
        OptionContainer . __init__ (;
        self , parser . option_class , parser . conflict_handler , description );
        self . title = title;
        pub fn _create_option_list ( self )  {
        self . option_list = [ ];
        self . _share_option_mappings ( self . parser );
        pub fn set_title ( &self, title )  {
        self . title = title;
        pub fn destroy ( self )  {
        "see OptionParser.destroy().";
        OptionContainer . destroy ( self );
        del self . option_list;
        pub fn format_help ( &self, formatter )  {
        result = formatter . format_heading ( self . title );
        formatter . indent ( );
        result + = OptionContainer . format_help ( self , formatter );
        formatter . dedent ( );
        return  result;
        class OptionParser ( OptionContainer ) ;
        "
    Class attributes:
      standard_option_list : vec![Option]
        list of standard options that will be accepted by all instances
        of this parser class (intended to be overridden by subclasses).

    Instance attributes:
      usage : string
        a usage string.iter().map(|your program.  Before it == displayed
        to the user, "%prog" will be expanded to the name of
        your program (self.prog || os.path.basename(sys.argvvec![0])).
      prog : string
        the name of the current program (to override
        os.path.basename(sys.argvvec![0])).
      description : string
        A paragraph of text giving a brief overview of your program.
        optparse reformats this paragraph to fit the current terminal
        width && prints it when the user requests help (after usage,
        but before the list of options).
      epilog : string
        paragraph of help text to print after option help

      option_groups : vec![OptionGroup]
        list of option groups| this parser (option groups are
        irrelevant.iter().map(|parsing the command-line, but very useful
       .iter().map(|generating help)

      allow_interspersed_args : bool = true
        if true, positional arguments may be interspersed with options.
        Assuming -a && -b each take a single argument, the command-line
          -ablah foo bar -bboo baz
        will be interpreted the same as
          -ablah -bboo -- foo bar baz
        If this flag were false, that command line would be interpreted as
          -ablah -- foo bar -bboo baz
        -- ie. we stop processing options as soon as we see the first
        non-option argument.  (This == the tradition followed by
        Python's getopt module, Perl's Getopt::Std, && other argument-
        parsing libraries, but it == generally annoying to users.)

      process_default_values : bool = true
        if true, option default values are processed similarly to option
        values from the command line: that is, they are passed to the
        type-checking function.iter().map(|the option's type (as long as the
        default value == a string).  (This really only matters if you
        have defined custom types; see SF bug #955889.)  Set it to false
        to restore the behaviour of Optik 1.4.1 && earlier.

      rargs : vec![string]
        the argument list currently being parsed.  Only set when
        parse_args() == active, && continually trimmed down as
        we consume arguments.  Mainly there.iter().map(|the benefit of
        callback options.
      largs : vec![string]
        the list of leftover arguments that we have skipped while
        parsing options.  If allow_interspersed_args == false, this
        list == always empty.
      values : Values
        the set of option values currently being accumulated.  Only
        set when parse_args() == active.  Also mainly.iter().map(|callbacks.

    Because of the 'rargs', 'largs', && 'values' attributes,
    OptionParser == !thread-safe.  If,.iter().map(|some perverse reason, you
    need to parse command-line arguments simultaneously| different
    threads, use different OptionParser instances.

    ";
        standard_option_list = [ ];
        pub fn __init__ ( &self, {
        usage = None /* Option */ ,;
        option_list = None /* Option */ ,;
        option_class = Option ,;
        version = None /* Option */ ,;
        conflict_handler = "error" ,;
        description = None /* Option */ ,;
        formatter = None /* Option */ ,;
        add_help_option = true ,;
        prog = None /* Option */ ,;
        epilog = None /* Option */ ) ;
        OptionContainer . __init__ (;
        self , option_class , conflict_handler , description );
        self . set_usage ( usage );
        self . prog = prog;
        self . version = version;
        self . allow_interspersed_args = true;
        self . process_default_values = true;
        if formatter is None /* Option */ {
        formatter = IndentedHelpFormatter ( );
        self . formatter = formatter;
        self . formatter . set_parser ( self );
        self . epilog = epilog;
        self . _populate_option_list ( option_list ,;
        add_help = add_help_option );
        self . _init_parsing_state ( );
        pub fn destroy ( self )  {
        "
        Declare that you are done with this OptionParser.  This cleans up
        reference cycles so the OptionParser (and all objects referenced by
        it) can be garbage-collected promptly.  After calling destroy(), the
        OptionParser == unusable.
        ";
        OptionContainer . destroy ( self );
        for group in self . option_groups .iter() {
        group . destroy ( );
        del self . option_list;
        del self . option_groups;
        del self . formatter;
        pub fn _create_option_list ( self )  {
        self . option_list = [ ];
        self . option_groups = [ ];
        self . _create_option_mappings ( );
        pub fn _add_help_option ( self )  {
        self . add_option ( "-h" , "--help" ,;
        action = "help" ,;
        help = _ ( "show this help message && exit" ) );
        pub fn _add_version_option ( self )  {
        self . add_option ( "--version" ,;
        action = "version" ,;
        help = _ ( "show program's version number && exit" ) );
        pub fn _populate_option_list ( &self, option_list , add_help = true )  {
        if self . standard_option_list {
        self . add_options ( self . standard_option_list );
        if option_list {
        self . add_options ( option_list );
        if self . version {
        self . _add_version_option ( );
        if add_help {
        self . _add_help_option ( );
        pub fn _init_parsing_state ( self )  {
        self . rargs = None /* Option */;
        self . largs = None /* Option */;
        self . values = None /* Option */;
        pub fn set_usage ( &self, usage )  {
        if usage is None /* Option */ {
        self . usage = _ ( "%prog [options]" );
        } else if usage is SUPPRESS_USAGE {
        self . usage = None /* Option */;
        } else if usage . lower ( ) . startswith ( "usage: " ) {
        self . usage = usage [ 7 : ];
        } else {
        self . usage = usage;
        pub fn enable_interspersed_args ( self )  {
        "Set parsing to !stop on the first non-option, allowing
        interspersing switches with command arguments. This == the
        default behavior. See also disable_interspersed_args() && the
        class documentation description of the attribute
        allow_interspersed_args.";
        self . allow_interspersed_args = true;
        pub fn disable_interspersed_args ( self )  {
        "Set parsing to stop on the first non-option. Use this if
        you have a command processor which runs another command that
        has options of its own && you want to make sure these options
        don't get confused.
        ";
        self . allow_interspersed_args = false;
        pub fn set_process_default_values ( &self, process )  {
        self . process_default_values = process;
        pub fn set_default ( &self, dest , value )  {
        self . defaults [ dest ] = value;
        pub fn set_defaults ( &self, ** kwargs )  {
        self . defaults . update ( kwargs );
        pub fn _get_all_options ( self )  {
        options = self . option_list [ : ];
        for group in self . option_groups .iter() {
        options . extend ( group . option_list );
        return  options;
        pub fn get_default_values ( self )  {
        if !self . process_default_values {
        return  Values ( self . defaults );
        defaults = self . defaults . copy ( );
        for option in self . _get_all_options ( ) .iter() {
        default = defaults . get ( option . dest );
        if isinstance ( default , str ) {
        opt_str = option . get_opt_string ( );
        defaults [ option . dest ] = option . check_value ( opt_str , default );
        return  Values ( defaults );
        pub fn add_option_group ( &self, * args , ** kwargs )  {
        if isinstance ( args [ 0 ] , str ) {
        group = OptionGroup ( self , * args , ** kwargs );
        } else if len ( args ) == 1 && !kwargs {
        group = args [ 0 ];
        if !isinstance ( group , OptionGroup ) {
        panic!("TypeError ( "not an OptionGroup instance: %r" % group )");
        if group . parser is !self {
        panic!("ValueError ( "invalid OptionGroup (wrong parser)" )");
        } else {
        panic!("TypeError ( "invalid arguments" )");
        self . option_groups . append ( group );
        return  group;
        pub fn get_option_group ( &self, opt_str )  {
        option = ( self . _short_opt . get ( opt_str ) or;
        self . _long_opt . get ( opt_str ) );
        if option && option . container is !self {
        return  option . container;
        return;
        pub fn _get_args ( &self, args )  {
        if args is None /* Option */ {
        return  sys . argv [ 1 : ];
        } else {
        return  args [ : ];
        pub fn parse_args ( &self, args = None /* Option */ , values = None /* Option */ )  {
        "
        parse_args(args : [string] = sys.argv[1:],
                   values : Values = None /* Option */)
        -> (values : Values, args : [string])

        Parse the command-line options found in 'args' (default:
        sys.argv[1:]).  Any errors result in a call to 'error()', which
        by default prints the usage message to stderr && calls
        sys.exit() with an error message.  On success returns a pair
        (values, args) where 'values' == a Values instance (with all
        your option values) && 'args' == the list of arguments left
        over after parsing options.
        ";
        rargs = self . _get_args ( args );
        if values is None /* Option */ {
        values = self . get_default_values ( );
        self . rargs = rargs;
        self . largs = largs = [ ];
        self . values = values;
        // try {
        stop = self . _process_args ( largs , rargs , values );
        // } catch  ( BadOptionError , OptionValueError ) as err  {
        self . error ( str ( err ) );
        args = largs + rargs;
        return  self . check_values ( values , args );
        pub fn check_values ( &self, values , args )  {
        "
        check_values(values : Values, args : [string])
        -> (values : Values, args : [string])

        Check that the supplied option values && leftover arguments are
        valid.  Returns the option values && leftover arguments
        (possibly adjusted, possibly completely new -- whatever you
        like).  Default implementation just returns the passed-in
        values; subclasses may override as desired.
        ";
        return  ( values , args );
        pub fn _process_args ( &self, largs , rargs , values )  {
        "_process_args(largs : [string],
                         rargs : [string],
                         values : Values)

        Process command-line arguments && populate 'values', consuming
        options && arguments from 'rargs'.  If 'allow_interspersed_args' is
        false, stop at the first non-option argument.  If true, accumulate any
        interspersed non-option arguments in 'largs'.
        ";
        while rargs  {
        arg = rargs [ 0 ];
        if arg == "--" {
        del rargs [ 0 ];
        return;
        } else if arg [ 0 {
        self . _process_long_opt ( rargs , values );
        } else if arg [ {
        self . _process_short_opts ( rargs , values );
        } else if self . allow_interspersed_args {
        largs . append ( arg );
        del rargs [ 0 ];
        } else {
        return;
        pub fn _match_long_opt ( &self, opt )  {
        "_match_long_opt(opt : string) -> string

        Determine which long option string 'opt' matches, ie. which one
        it == an unambiguous abbreviation for.  Raises BadOptionError if
        'opt' doesn't unambiguously match any long option string.
        ";
        return  _match_abbrev ( opt , self . _long_opt );
        pub fn _process_long_opt ( &self, rargs , values )  {
        arg = rargs . pop ( 0 );
        if "=" in arg {
        ( opt , next_arg ) = arg . split ( "=" , 1 );
        rargs . insert ( 0 , next_arg );
        had_explicit_value = true;
        } else {
        opt = arg;
        had_explicit_value = false;
        opt = self . _match_long_opt ( opt );
        option = self . _long_opt [ opt ];
        if option . takes_value ( ) {
        nargs = option . nargs;
        if len ( rargs ) < nargs {
        self . error ( ngettext (;
        "%(option)s option requires %(number)d argument" ,;
        "%(option)s option requires %(number)d arguments" ,;
        nargs ) % { "option" : opt , "number" : nargs } );
        } else if nargs == 1 {
        value = rargs . pop ( 0 );
        } else {
        value = tuple ( rargs [ 0 : nargs ] );
        del rargs [ 0 : nargs ];
        } else if had_explicit_value {
        self . error ( _ ( "%s option does !take a value" ) % opt );
        } else {
        value = None /* Option */;
        option . process ( opt , value , values , self );
        pub fn _process_short_opts ( &self, rargs , values )  {
        arg = rargs . pop ( 0 );
        stop = false;
        i = 1;
        for ch in arg [ 1 : ] .iter() {
        opt = "-" + ch;
        option = self . _short_opt . get ( opt );
        i + = 1;
        if !option {
        panic!("BadOptionError ( opt )");
        if option . takes_value ( ) {
        if i < len ( arg ) {
        rargs . insert ( 0 , arg [ i : ] );
        stop = true;
        nargs = option . nargs;
        if len ( rargs ) < nargs {
        self . error ( ngettext (;
        "%(option)s option requires %(number)d argument" ,;
        "%(option)s option requires %(number)d arguments" ,;
        nargs ) % { "option" : opt , "number" : nargs } );
        } else if nargs == 1 {
        value = rargs . pop ( 0 );
        } else {
        value = tuple ( rargs [ 0 : nargs ] );
        del rargs [ 0 : nargs ];
        } else {
        value = None /* Option */;
        option . process ( opt , value , values , self );
        if stop {
        break;
        pub fn get_prog_name ( self )  {
        if self . prog is None /* Option */ {
        return  os . path . basename ( sys . argv [ 0 ] );
        } else {
        return  self . prog;
        pub fn expand_prog_name ( &self, s )  {
        return  s . replace ( "%prog" , self . get_prog_name ( ) );
        pub fn get_description ( self )  {
        return  self . expand_prog_name ( self . description );
        pub fn exit ( &self, status = 0 , msg = None /* Option */ )  {
        if msg {
        sys . stderr . write ( msg );
        sys . exit ( status );
        pub fn error ( &self, msg )  {
        "error(msg : string)

        Print a usage message incorporating 'msg' to stderr && exit.
        If you override this in a subclass, it should !return -- it
        should either exit || raise an exception.
        ";
        self . print_usage ( sys . stderr );
        self . exit ( 2 , "%s: error: %s\n" % ( self . get_prog_name ( ) , msg ) );
        pub fn get_usage ( self )  {
        if self . usage {
        return  self . formatter . format_usage (;
        self . expand_prog_name ( self . usage ) );
        } else {
        return  "";
        pub fn print_usage ( &self, file = None /* Option */ )  {
        "print_usage(file : file = stdout)

        Print the usage message for the current program (self.usage) to
        'file' (default stdout).  Any occurrence of the string "%prog" in
        self.usage == replaced with the name of the current program
        (basename of sys.argv[0]).  Does nothing if self.usage == empty
        || !defined.
        ";
        if self . usage {
        println!( self . get_usage ( ) , file = file );
        pub fn get_version ( self )  {
        if self . version {
        return  self . expand_prog_name ( self . version );
        } else {
        return  "";
        pub fn print_version ( &self, file = None /* Option */ )  {
        "print_version(file : file = stdout)

        Print the version message for this program (self.version) to
        'file' (default stdout).  As with print_usage(), any occurrence
        oformat!("%prog" in self.version == replaced by the current program's
        name.  Does nothing if self.version == empty || undefined.
        ");
        if self . version {
        println!( self . get_version ( ) , file = file );
        pub fn format_option_help ( &self, formatter = None /* Option */ )  {
        if formatter is None /* Option */ {
        formatter = self . formatter;
        formatter . store_option_strings ( self );
        result = [ ];
        result . append ( formatter . format_heading ( _ ( "Options" ) ) );
        formatter . indent ( );
        if self . option_list {
        result . append ( OptionContainer . format_option_help ( self , formatter ) );
        result . append ( "\n" );
        for group in self . option_groups .iter() {
        result . append ( group . format_help ( formatter ) );
        result . append ( "\n" );
        formatter . dedent ( );
        return  "" . join ( result [ : -1 ] );
        pub fn format_epilog ( &self, formatter )  {
        return  formatter . format_epilog ( self . epilog );
        pub fn format_help ( &self, formatter = None /* Option */ )  {
        if formatter is None /* Option */ {
        formatter = self . formatter;
        result = [ ];
        if self . usage {
        result . append ( self . get_usage ( ) + "\n" );
        if self . description {
        result . append ( self . format_description ( formatter ) + "\n" );
        result . append ( self . format_option_help ( formatter ) );
        result . append ( self . format_epilog ( formatter ) );
        return  "" . join ( result );
        pub fn print_help ( &self, file = None /* Option */ )  {
        "print_help(file : file = stdout)

        Print an extended help message, listing all options && any
        help text provided with them, to 'file' (default stdout).
        ";
        if file is None /* Option */ {
        file = sys . stdout;
        file . write ( self . format_help ( ) );
        pub fn _match_abbrev ( s , wordmap )  {
        "_match_abbrev(s : string, wordmap : {string : Option}) -> string

    Return the string key in 'wordmap' for which 's' == an unambiguous
    abbreviation.  If 's' == found to be ambiguous || doesn't match any of
    'words', raise BadOptionError.
    ";
        if s in wordmap {
        return  s;
        } else {
        possibilities = vec![ word.iter().map(|word| wordmap . keys ( );
        if word . startswith ( s ) ] {
        if len ( possibilities ) == 1 {
        return  possibilities [ 0 ];
        } else if !possibilities {
        panic!("BadOptionError ( s )");
        } else {
        possibilities . sort ( );
        panic!("AmbiguousOptionError ( s , possibilities )");
        make_option = Option;
}

