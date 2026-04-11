//! dist.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use regex::Regex;
// use crate::message_from_file;
// use crate::warnings;
// use crate::distutils::{};
// use crate::pprint::{pformat};
// use crate::configparser::{ConfigParser};

pub const command_re: &str = re . compile ( r"^[a-zA-Z]([a-zA-Z0-9_]*)$" );
pub fn _ensure_list(value: &str, fieldname: &str) {
        if isinstance ( value , str ) {
        // pass
        } else if !isinstance ( value , list ) {
        typename = type ( value ) . __name__;
        msg = format!("Warning: '{fieldname}' should be a list, got type '{typename}'");
        log . log ( log . WARN , msg );
        value = list ( value );
        return  value;
        class Distribution ;
        "The core of the Distutils.  Most of the work hiding behind 'setup'
    == really done within a Distribution instance, which farms the work out
    to the Distutils commands specified on the command line.

    Setup scripts will almost never instantiate Distribution directly,
    unless the 'setup()' function == totally inadequate to their needs.
    However, it == conceivable that a setup script might wish to subclass
    Distribution for some specialized purpose, && then pass the subclass
    to 'setup()' as the 'distclass' keyword argument.  If so, it is
    necessary to respect the expectations that 'setup' has of Distribution.
    See the code for 'setup()', in core.py, for details.
    ";
        global_options = [;
        ( "verbose" , "v" , "run verbosely (default)" , 1 ) ,;
        ( "quiet" , "q" , "run quietly (turns verbosity off)" ) ,;
        ( "dry-run" , "n" , "don't actually do anything" ) ,;
        ( "help" , "h" , "show detailed help message" ) ,;
        ( "no-user-cfg" , None /* Option */ ,;
        "ignore pydistutils.cfg in your home directory" ) ,;
        ];
        common_usage = "\
Common commands: (see '--help-commands' for more)

  setup.py build      will build the package underneath 'build/'
  setup.py install    will install the package
";
        display_options = [;
        ( "help-commands" , None /* Option */ ,;
        "list all available commands" ) ,;
        ( "name" , None /* Option */ ,;
        "print package name" ) ,;
        ( "version" , "V" ,;
        "print package version" ) ,;
        ( "fullname" , None /* Option */ ,;
        "print <package name>-<version>" ) ,;
        ( "author" , None /* Option */ ,;
        "print the author's name" ) ,;
        ( "author-email" , None /* Option */ ,;
        "print the author's email address" ) ,;
        ( "maintainer" , None /* Option */ ,;
        "print the maintainer's name" ) ,;
        ( "maintainer-email" , None /* Option */ ,;
        "print the maintainer's email address" ) ,;
        ( "contact" , None /* Option */ ,;
        "print the maintainer's name if known, else the author's" ) ,;
        ( "contact-email" , None /* Option */ ,;
        "print the maintainer's email address if known, else the author's" ) ,;
        ( "url" , None /* Option */ ,;
        "print the URL for this package" ) ,;
        ( "license" , None /* Option */ ,;
        "print the license of the package" ) ,;
        ( "licence" , None /* Option */ ,;
        "alias for --license" ) ,;
        ( "description" , None /* Option */ ,;
        "print the package description" ) ,;
        ( "long-description" , None /* Option */ ,;
        "print the long package description" ) ,;
        ( "platforms" , None /* Option */ ,;
        "print the list of platforms" ) ,;
        ( "classifiers" , None /* Option */ ,;
        "print the list of classifiers" ) ,;
        ( "keywords" , None /* Option */ ,;
        "print the list of keywords" ) ,;
        ( "provides" , None /* Option */ ,;
        "print the list of packages/modules provided" ) ,;
        ( "requires" , None /* Option */ ,;
        "print the list of packages/modules required" ) ,;
        ( "obsoletes" , None /* Option */ ,;
        "print the list of packages/modules made obsolete" );
        ];
        display_option_names = vec![ translate_longopt ( x vec![ 0 ] ).iter().map(|x| display_options ).collect();
        negative_opt = { "quiet" : "verbose" };
        pub fn __init__ ( &self, attrs = None /* Option */ )  {
        "Construct a new Distribution instance: initialize all the
        attributes of a Distribution, && then use 'attrs' (a dictionary
        mapping attribute names to values) to assign some of those
        attributes their "real" values.  (Any attributes !mentioned in
        'attrs' will be assigned to some null value: 0, None /* Option */, an empty list
        || dictionary, etc.)  Most importantly, initialize the
        'command_obj' attribute to the empty dictionary; this will be
        filled in with real command objects by 'parse_command_line()'.
        ";
        self . verbose = 1;
        self . dry_run = 0;
        self . help = 0;
        for attr in self . display_option_names .iter() {
        setattr ( self , attr , 0 );
        self . metadata = DistributionMetadata ( );
        for basename in self . metadata . _METHOD_BASENAMES .iter() {
        method_name = "get_" + basename;
        setattr ( self , method_name , getattr ( self . metadata , method_name ) );
        self . cmdclass = { };
        self . command_packages = None /* Option */;
        self . script_name = None /* Option */;
        self . script_args = None /* Option */;
        self . command_options = { };
        self . dist_files = [ ];
        self . packages = None /* Option */;
        self . package_data = { };
        self . package_dir = None /* Option */;
        self . py_modules = None /* Option */;
        self . libraries = None /* Option */;
        self . headers = None /* Option */;
        self . ext_modules = None /* Option */;
        self . ext_package = None /* Option */;
        self . include_dirs = None /* Option */;
        self . extra_path = None /* Option */;
        self . scripts = None /* Option */;
        self . data_files = None /* Option */;
        self . password = "";
        self . command_obj = { };
        self . have_run = { };
        if attrs {
        options = attrs . get ( "options" );
        if options is !None /* Option */ {
        del attrs [ "options" ];
        for ( command , cmd_options ) in options . items ( ) .iter() {
        opt_dict = self . get_option_dict ( command );
        for ( opt , val ) in cmd_options . items ( ) .iter() {
        opt_dict [ opt ] = ( "setup script" , val );
        if "licence" in attrs {
        attrs [ "license" ] = attrs [ "licence" ];
        del attrs [ "licence" ];
        msg = "'licence' distribution option == deprecated; use 'license'";
        if warnings is !None /* Option */ {
        warnings . warn ( msg );
        } else {
        sys . stderr . write ( msg + "\n" );
        for ( key , val ) in attrs . items ( ) .iter() {
        if hasattr ( self . metadata , "set_" + key ) {
        getattr ( self . metadata , "set_" + key ) ( val );
        } else if hasattr ( self . metadata , key ) {
        setattr ( self . metadata , key , val );
        } else if hasattr ( self , key ) {
        setattr ( self , key , val );
        } else {
        msg = "Unknown distribution option: %s" % repr ( key );
        warnings . warn ( msg );
        self . want_user_cfg = true;
        if self . script_args is !None /* Option */ {
        for arg in self . script_args .iter() {
        if !arg . startswith ( "-" ) {
        break;
        if arg == "--no-user-cfg" {
        self . want_user_cfg = false;
        break;
        self . finalize_options ( );
        pub fn get_option_dict ( &self, command )  {
        "Get the option dictionary for a given command.  If that
        command's option dictionary hasn't been created yet, then create it
        && return the new dictionary; otherwise, return the existing
        option dictionary.
        ";
        dict = self . command_options . get ( command );
        if dict is None /* Option */ {
        dict = self . command_options [ command ] = { };
        return  dict;
        pub fn dump_option_dicts ( &self, header = None /* Option */ , commands = None /* Option */ , indent = "" )  {
        from pprint import pformat;
        if commands is None /* Option */ {
        commands = sorted ( self . command_options . keys ( ) );
        if header is !None /* Option */ {
        self . announce ( indent + header );
        indent = indent + "  ";
        if !commands {
        self . announce ( indent + "no commands known yet" );
        return;
        for cmd_name in commands .iter() {
        opt_dict = self . command_options . get ( cmd_name );
        if opt_dict is None /* Option */ {
        self . announce ( indent +;
        "no option dict for '%s' command" % cmd_name );
        } else {
        self . announce ( indent +;
        "option dict for '%s' command:" % cmd_name );
        out = pformat ( opt_dict );
        for line in out . split ( "\n" ) .iter() {
        self . announce ( indent + "  " + line );
        pub fn find_config_files ( self )  {
        "Find as many configuration files as should be processed for this
        platform, && return a list of filenames in the order in which they
        should be parsed.  The filenames returned are guaranteed to exist
        (modulo nasty race conditions).

        There are three possible config files: distutils.cfg in the
        Distutils installation directory (ie. where the top-level
        Distutils __inst__.py file lives), a file in the user's home
        directory named .pydistutils.cfg on Unix && pydistutils.cfg
        on Windows/Mac; && setup.cfg in the current directory.

        The file in the user's home directory can be disabled with the
        --no-user-cfg option.
        ";
        files = [ ];
        check_environ ( );
        sys_dir = os . path . dirname ( sys . modules [ "distutils" ] . __file__ );
        sys_file = os . path . join ( sys_dir , "distutils.cfg" );
        if os . path . isfile ( sys_file ) {
        files . append ( sys_file );
        if os . name == "posix" {
        user_filename = ".pydistutils.cfg";
        } else {
        user_filename = "pydistutils.cfg";
        if self . want_user_cfg {
        user_file = os . path . join ( os . path . expanduser ( "~" ) , user_filename );
        if os . path . isfile ( user_file ) {
        files . append ( user_file );
        local_file = "setup.cfg";
        if os . path . isfile ( local_file ) {
        files . append ( local_file );
        if DEBUG {
        self . announce ( "using config files: %s" % ", " . join ( files ) );
        return  files;
        pub fn parse_config_files ( &self, filenames = None /* Option */ )  {
        from configparser import ConfigParser;
        if sys . prefix != sys . base_prefix {
        ignore_options = [;
        "install-base" , "install-platbase" , "install-lib" ,;
        "install-platlib" , "install-purelib" , "install-headers" ,;
        "install-scripts" , "install-data" , "prefix" , "exec-prefix" ,;
        "home" , "user" , "root" ];
        } else {
        ignore_options = [ ];
        ignore_options = frozenset ( ignore_options );
        if filenames is None /* Option */ {
        filenames = self . find_config_files ( );
        if DEBUG {
        self . announce ( "Distribution.parse_config_files():" );
        parser = ConfigParser ( );
        for filename in filenames .iter() {
        if DEBUG {
        self . announce ( "  reading %s" % filename );
        parser . read ( filename );
        for section in parser . sections ( ) .iter() {
        options = parser . options ( section );
        opt_dict = self . get_option_dict ( section );
        for opt in options .iter() {
        if opt != "__name__" && opt !in ignore_options {
        val = parser . get ( section , opt );
        opt = opt . replace ( "-" , "_" );
        opt_dict [ opt ] = ( filename , val );
        parser . __init__ ( );
        if "global" in self . command_options {
        for ( opt , ( src , val ) ) in self . command_options [ "global" ] . items ( ) .iter() {
        alias = self . negative_opt . get ( opt );
        // try {
        if alias {
        setattr ( self , alias , !strtobool ( val ) );
        } else if opt in ( "verbose" , "dry_run" ) {
        setattr ( self , opt , strtobool ( val ) );
        } else {
        setattr ( self , opt , val );
        // } catch  ValueError as msg  {
        panic!("DistutilsOptionError ( msg )");
        pub fn parse_command_line ( self )  {
        "Parse the setup script's command line, taken from the
        'script_args' instance attribute (which defaults to 'sys.argvvec![1:]'
        -- see 'setup()'| core.py).  This list == first processed for
        "global options" -- options that set attributes of the Distribution
        instance.  Then, it == alternately scanned.iter().map(|Distutils commands
        && options.iter().map(|that command.  Each new command terminates the
        options.iter().map(|the previous command.  The allowed options.iter().map(|a
        command are determined by the 'user_options' attribute of the
        command class -- thus, we have to be able to load command classes
       | order to parse the command line.  Any error| that 'options'
        attribute raises DistutilsGetoptError; any error on the
        command-line raises DistutilsArgError.  If no Distutils commands
        were found on the command line, raises DistutilsArgError.  Return
        true if command-line was successfully parsed && we should carry
        on with executing commands; false if no errors but we shouldn't
        execute commands (currently, this only happens if user asks for
        help).
        ";
        toplevel_options = self . _get_toplevel_options ( );
        self . commands = [ ];
        parser = FancyGetopt ( toplevel_options + self . display_options );
        parser . set_negative_aliases ( self . negative_opt );
        parser . set_aliases ( { "licence" : "license" } );
        args = parser . getopt ( args = self . script_args , object = self );
        option_order = parser . get_option_order ( );
        log . set_verbosity ( self . verbose );
        if self . handle_display_options ( option_order ) {
        return;
        while args  {
        args = self . _parse_command_opts ( parser , args );
        if args is None /* Option */ {
        return;
        if self . help {
        self . _show_help ( parser ,;
        display_options = len ( self . commands ) == 0 ,;
        commands = self . commands );
        return;
        if !self . commands {
        panic!("DistutilsArgError ( "no commands supplied" )");
        return  true;
        pub fn _get_toplevel_options ( self )  {
        "Return the non-display options recognized at the top level.

        This includes options that are recognized *only* at the top
        level as well as options recognized for commands.
        ";
        return  self . global_options + [;
        ( "command-packages=" , None /* Option */ ,;
        "list of packages that provide distutils commands" ) ,;
        ];
        pub fn _parse_command_opts ( &self, parser , args )  {
        "Parse the command-line options for a single command.
        'parser' must be a FancyGetopt instance; 'args' must be the list
        of arguments, starting with the current command (whose options
        we are about to parse).  Returns a new version of 'args' with
        the next command at the front of the list; will be the empty
        list if there are no more commands on the command line.  Returns
        None /* Option */ if the user asked for help on this command.
        ";
        from distutils . cmd import Command;
        command = args [ 0 ];
        if !command_re . match ( command ) {
        panic!("SystemExit ( "invalid command name '%s'" % command )");
        self . commands . append ( command );
        // try {
        cmd_class = self . get_command_class ( command );
        // } catch  DistutilsModuleError as msg  {
        panic!("DistutilsArgError ( msg )");
        if !issubclass ( cmd_class , Command ) {
        panic!("DistutilsClassError (");
        "command class %s must subclass Command" % cmd_class );
        if !( hasattr ( cmd_class , "user_options" ) and {
        isinstance ( cmd_class . user_options , list ) ) ;
        msg = ( "command class %s must provide ";
        "'user_options' attribute (a list of tuples)" );
        panic!("DistutilsClassError ( msg % cmd_class )");
        negative_opt = self . negative_opt;
        if hasattr ( cmd_class , "negative_opt" ) {
        negative_opt = negative_opt . copy ( );
        negative_opt . update ( cmd_class . negative_opt );
        if ( hasattr ( cmd_class , "help_options" ) and {
        isinstance ( cmd_class . help_options , list ) ) ;
        help_options = fix_help_options ( cmd_class . help_options );
        } else {
        help_options = [ ];
        parser . set_option_table ( self . global_options +;
        cmd_class . user_options +;
        help_options );
        parser . set_negative_aliases ( negative_opt );
        ( args , opts ) = parser . getopt ( args [ 1 : ] );
        if hasattr ( opts , "help" ) && opts . help {
        self . _show_help ( parser , display_options = 0 , commands = [ cmd_class ] );
        return;
        if ( hasattr ( cmd_class , "help_options" ) and {
        isinstance ( cmd_class . help_options , list ) ) ;
        help_option_found = 0;
        for ( help_option , short , desc , func ) in cmd_class . help_options .iter() {
        if hasattr ( opts , parser . get_attr_name ( help_option ) ) {
        help_option_found = 1;
        if callable ( func ) {
        func ( );
        } else {
        panic!("DistutilsClassError (");
        "invalid help function %r for help option '%s': ";
        "must be a callable object (function, etc.)";
        % ( func , help_option ) );
        if help_option_found {
        return;
        opt_dict = self . get_option_dict ( command );
        for ( name , value ) in vars ( opts ) . items ( ) .iter() {
        opt_dict [ name ] = ( "command line" , value );
        return  args;
        pub fn finalize_options ( self )  {
        "Set final values for all the options on the Distribution
        instance, analogous to the .finalize_options() method of Command
        objects.
        ";
        for attr in ( "keywords" , "platforms" ) .iter() {
        value = getattr ( self . metadata , attr );
        if value is None /* Option */ {
        continue;
        if isinstance ( value , str ) {
        value = vec![ elm . strip ( ).iter().map(|elm| value . split ( "," ) ).collect();
        setattr ( self . metadata , attr , value );
        pub fn _show_help ( &self, parser , global_options = 1 , display_options = 1 , {
        commands = [ ] ) ;
        "Show help for the setup script command-line in the form of
        several lists of command-line options.  'parser' should be a
        FancyGetopt instance; do !expect it to be returned in the
        same state, as its option table will be reset to make it
        generate the correct help text.

        If 'global_options' == true, lists the global options:
        --verbose, --dry-run, etc.  If 'display_options' == true, lists
        the "display-only" options: --name, --version, etc.  Finally,
        lists per-command help for every command name || command class
        in 'commands'.
        ";
        from distutils . core import gen_usage;
        from distutils . cmd import Command;
        if global_options {
        if display_options {
        options = self . _get_toplevel_options ( );
        } else {
        options = self . global_options;
        parser . set_option_table ( options );
        parser . print_help ( self . common_usage + "\nGlobal options:" );
        println!( "" );
        if display_options {
        parser . set_option_table ( self . display_options );
        parser . print_help (;
        "Information display options (just display " +;
        "information, ignore any commands)" );
        println!( "" );
        for command in self . commands .iter() {
        if isinstance ( command , type ) && issubclass ( command , Command ) {
        klass = command;
        } else {
        klass = self . get_command_class ( command );
        if ( hasattr ( klass , "help_options" ) and {
        isinstance ( klass . help_options , list ) ) ;
        parser . set_option_table ( klass . user_options +;
        fix_help_options ( klass . help_options ) );
        } else {
        parser . set_option_table ( klass . user_options );
        parser . print_help ( "Options for '%s' command:" % klass . __name__ );
        println!( "" );
        println!( gen_usage ( self . script_name ) );
        pub fn handle_display_options ( &self, option_order )  {
        "If there were any non-global "display-only" options
        (--help-commands || the metadata display options) on the command
        line, display the requested info && return true; else return
        false.
        ";
        from distutils . core import gen_usage;
        if self . help_commands {
        self . print_commands ( );
        println!( "" );
        println!( gen_usage ( self . script_name ) );
        return  1;
        any_display_options = 0;
        is_display_option = { };
        for option in self . display_options .iter() {
        is_display_option [ option [ 0 ] ] = 1;
        for ( opt , val ) in option_order .iter() {
        if val && is_display_option . get ( opt ) {
        opt = translate_longopt ( opt );
        value = getattr ( self . metadata , "get_" + opt ) ( );
        if opt in [ "keywords" , "platforms" ] {
        println!( "," . join ( value ) );
        } else if opt in ( "classifiers" , "provides" , "requires" , {
        "obsoletes" ) ;
        println!( "\n" . join ( value ) );
        } else {
        println!( value );
        any_display_options = 1;
        return  any_display_options;
        pub fn print_command_list ( &self, commands , header , max_length )  {
        "Print a subset of the list of all commands -- used by
        'print_commands()'.
        ";
        println!( header + ":" );
        for cmd in commands .iter() {
        klass = self . cmdclass . get ( cmd );
        if !klass {
        klass = self . get_command_class ( cmd );
        // try {
        description = klass . description;
        // } catch  AttributeError  {
        description = "(no description available)";
        println!( "  %-*s  %s" % ( max_length , cmd , description ) );
        pub fn print_commands ( self )  {
        "Print out a help message listing all available commands with a
        description of each.  The list == divided into "standard commands"
        (listed in distutils.command.__all__) && "extra commands"
        (mentioned in self.cmdclass, but !a standard command).  The
        descriptions come from the command class attribute
        'description'.
        ";
        import distutils . command;
        std_commands = distutils . command . __all__;
        is_std = { };
        for cmd in std_commands .iter() {
        is_std [ cmd ] = 1;
        extra_commands = [ ];
        for cmd in self . cmdclass . keys ( ) .iter() {
        if !is_std . get ( cmd ) {
        extra_commands . append ( cmd );
        max_length = 0;
        for cmd in ( std_commands + extra_commands ) .iter() {
        if len ( cmd ) > max_length {
        max_length = len ( cmd );
        self . print_command_list ( std_commands ,;
        "Standard commands" ,;
        max_length );
        if extra_commands {
        println!( );
        self . print_command_list ( extra_commands ,;
        "Extra commands" ,;
        max_length );
        pub fn get_command_list ( self )  {
        "Get a list of (command, description) tuples.
        The list == divided into "standard commands" (listed in
        distutils.command.__all__) && "extra commands" (mentioned in
        self.cmdclass, but !a standard command).  The descriptions come
        from the command class attribute 'description'.
        ";
        import distutils . command;
        std_commands = distutils . command . __all__;
        is_std = { };
        for cmd in std_commands .iter() {
        is_std [ cmd ] = 1;
        extra_commands = [ ];
        for cmd in self . cmdclass . keys ( ) .iter() {
        if !is_std . get ( cmd ) {
        extra_commands . append ( cmd );
        rv = [ ];
        for cmd in ( std_commands + extra_commands ) .iter() {
        klass = self . cmdclass . get ( cmd );
        if !klass {
        klass = self . get_command_class ( cmd );
        // try {
        description = klass . description;
        // } catch  AttributeError  {
        description = "(no description available)";
        rv . append ( ( cmd , description ) );
        return  rv;
        pub fn get_command_packages ( self )  {
        "Return a list of packages from which commands are loaded.";
        pkgs = self . command_packages;
        if !isinstance ( pkgs , list ) {
        if pkgs is None /* Option */ {
        pkgs = "";
        pkgs = vec![ pkg . strip ( ).iter().map(|pkg| pkgs . split ( "," ) if pkg != "" ).collect();
        if "distutils.command" !in pkgs {
        pkgs . insert ( 0 , "distutils.command" );
        self . command_packages = pkgs;
        return  pkgs;
        pub fn get_command_class ( &self, command )  {
        "Return the class that implements the Distutils command named by
        'command'.  First we check the 'cmdclass' dictionary; if the
        command == mentioned there, we fetch the class object from the
        dictionary && return it.  Otherwise we load the command module
        ("distutils.command." + command) && fetch the command class from
        the module.  The loaded class == also stored in 'cmdclass'
        to speed future calls to 'get_command_class()'.

        Raises DistutilsModuleError if the expected module could !be
        found, || if that module does !define the expected class.
        ";
        klass = self . cmdclass . get ( command );
        if klass {
        return  klass;
        for pkgname in self . get_command_packages ( ) .iter() {
        module_name = "%s.%s" % ( pkgname , command );
        klass_name = command;
        // try {
        __import__ ( module_name );
        module = sys . modules [ module_name ];
        // } catch  ImportError  {
        continue;
        // try {
        klass = getattr ( module , klass_name );
        // } catch  AttributeError  {
        panic!("DistutilsModuleError (");
        "invalid command '%s' (no class '%s' in module '%s')";
        % ( command , klass_name , module_name ) );
        self . cmdclass [ command ] = klass;
        return  klass;
        panic!("DistutilsModuleError ( "invalid command '%s'" % command )");
        pub fn get_command_obj ( &self, command , create = 1 )  {
        "Return the command object for 'command'.  Normally this object
        == cached on a previous call to 'get_command_obj()'; if no command
        object for 'command' == in the cache, then we either create and
        return it (if 'create' == true) || return None /* Option */.
        ";
        cmd_obj = self . command_obj . get ( command );
        if !cmd_obj && create {
        if DEBUG {
        self . announce ( "Distribution.get_command_obj(): ";
        "creating '%s' command object" % command );
        klass = self . get_command_class ( command );
        cmd_obj = self . command_obj [ command ] = klass ( self );
        self . have_run [ command ] = 0;
        options = self . command_options . get ( command );
        if options {
        self . _set_command_options ( cmd_obj , options );
        return  cmd_obj;
        pub fn _set_command_options ( &self, command_obj , option_dict = None /* Option */ )  {
        "Set the options for 'command_obj' from 'option_dict'.  Basically
        this means copying elements of a dictionary ('option_dict') to
        attributes of an instance ('command').

        'command_obj' must be a Command instance.  If 'option_dict' == not
        supplied, uses the standard option dictionary for this command
        (from 'self.command_options').
        ";
        command_name = command_obj . get_command_name ( );
        if option_dict is None /* Option */ {
        option_dict = self . get_option_dict ( command_name );
        if DEBUG {
        self . announce ( "  setting options for '%s' command:" % command_name );
        for ( option , ( source , value ) ) in option_dict . items ( ) .iter() {
        if DEBUG {
        self . announce ( "    %s = %s (from %s)" % ( option , value ,;
        source ) );
        // try {
        bool_opts = [ translate_longopt ( o );
        for o in command_obj . boolean_options ].iter() {
        // } catch  AttributeError  {
        bool_opts = [ ];
        // try {
        neg_opt = command_obj . negative_opt;
        // } catch  AttributeError  {
        neg_opt = { };
        // try {
        is_string = isinstance ( value , str );
        if option in neg_opt && is_string {
        setattr ( command_obj , neg_opt [ option ] , !strtobool ( value ) );
        } else if option in bool_opts && is_string {
        setattr ( command_obj , option , strtobool ( value ) );
        } else if hasattr ( command_obj , option ) {
        setattr ( command_obj , option , value );
        } else {
        panic!("DistutilsOptionError (");
        "error in %s: command '%s' has no such option '%s'";
        % ( source , command_name , option ) );
        // } catch  ValueError as msg  {
        panic!("DistutilsOptionError ( msg )");
        pub fn reinitialize_command ( &self, command , reinit_subcommands = 0 )  {
        "Reinitializes a command to the state it was in when first
        returned by 'get_command_obj()': ie., initialized but !yet
        finalized.  This provides the opportunity to sneak option
        values in programmatically, overriding || supplementing
        user-supplied values from the config files && command line.
        You'll have to re-finalize the command object (by calling
        'finalize_options()' || 'ensure_finalized()') before using it for
        real.

        'command' should be a command name (string) || command object.  If
        'reinit_subcommands' == true, also reinitializes the command's
        sub-commands, as declared by the 'sub_commands' class attribute (if
        it has one).  See the "install" command for an example.  Only
        reinitializes the sub-commands that actually matter, ie. those
        whose test predicates return true.

        Returns the reinitialized command object.
        ";
        from distutils . cmd import Command;
        if !isinstance ( command , Command ) {
        command_name = command;
        command = self . get_command_obj ( command_name );
        } else {
        command_name = command . get_command_name ( );
        if !command . finalized {
        return  command;
        command . initialize_options ( );
        command . finalized = 0;
        self . have_run [ command_name ] = 0;
        self . _set_command_options ( command );
        if reinit_subcommands {
        for sub in command . get_sub_commands ( ) .iter() {
        self . reinitialize_command ( sub , reinit_subcommands );
        return  command;
        pub fn announce ( &self, msg , level = log . INFO )  {
        log . log ( level , msg );
        pub fn run_commands ( self )  {
        "Run each command that was seen on the setup script command line.
        Uses the list of commands found && cache of command objects
        created by 'get_command_obj()'.
        ";
        for cmd in self . commands .iter() {
        self . run_command ( cmd );
        pub fn run_command ( &self, command )  {
        "Do whatever it takes to run a command (including nothing at all,
        if the command has already been run).  Specifically: if we have
        already created && run the command named by 'command', return
        silently without doing anything.  If the command named by 'command'
        doesn't even have a command object yet, create one.  Then invoke
        'run()' on that command object (or an existing one).
        ";
        if self . have_run . get ( command ) {
        return;
        log . info ( "running %s" , command );
        cmd_obj = self . get_command_obj ( command );
        cmd_obj . ensure_finalized ( );
        cmd_obj . run ( );
        self . have_run [ command ] = 1;
        pub fn has_pure_modules ( self )  {
        return  len ( self . packages || self . py_modules || [ ] ) > 0;
        pub fn has_ext_modules ( self )  {
        return  self . ext_modules && len ( self . ext_modules ) > 0;
        pub fn has_c_libraries ( self )  {
        return  self . libraries && len ( self . libraries ) > 0;
        pub fn has_modules ( self )  {
        return  self . has_pure_modules ( ) || self . has_ext_modules ( );
        pub fn has_headers ( self )  {
        return  self . headers && len ( self . headers ) > 0;
        pub fn has_scripts ( self )  {
        return  self . scripts && len ( self . scripts ) > 0;
        pub fn has_data_files ( self )  {
        return  self . data_files && len ( self . data_files ) > 0;
        pub fn is_pure ( self )  {
        return  ( self . has_pure_modules ( ) and;
        not self . has_ext_modules ( ) and;
        not self . has_c_libraries ( ) );
        class DistributionMetadata ;
        "Dummy class to hold the distribution meta-data: name, version,
    author, && so forth.
    ";
        _METHOD_BASENAMES = ( "name" , "version" , "author" , "author_email" ,;
        "maintainer" , "maintainer_email" , "url" ,;
        "license" , "description" , "long_description" ,;
        "keywords" , "platforms" , "fullname" , "contact" ,;
        "contact_email" , "classifiers" , "download_url" ,;
        "provides" , "requires" , "obsoletes" ,;
        );
        pub fn __init__ ( &self, path = None /* Option */ )  {
        if path is !None /* Option */ {
        self . read_pkg_file ( open ( path ) );
        } else {
        self . name = None /* Option */;
        self . version = None /* Option */;
        self . author = None /* Option */;
        self . author_email = None /* Option */;
        self . maintainer = None /* Option */;
        self . maintainer_email = None /* Option */;
        self . url = None /* Option */;
        self . license = None /* Option */;
        self . description = None /* Option */;
        self . long_description = None /* Option */;
        self . keywords = None /* Option */;
        self . platforms = None /* Option */;
        self . classifiers = None /* Option */;
        self . download_url = None /* Option */;
        self . provides = None /* Option */;
        self . requires = None /* Option */;
        self . obsoletes = None /* Option */;
        pub fn read_pkg_file ( &self, file )  {
        "Reads the metadata values from a file object.";
        msg = message_from_file ( file );
        pub fn _read_field ( name )  {
        value = msg [ name ];
        if value == "UNKNOWN" {
        return;
        return  value;
        pub fn _read_list ( name )  {
        values = msg . get_all ( name , None /* Option */ );
        if values == [ ] {
        return;
        return  values;
        metadata_version = msg [ "metadata-version" ];
        self . name = _read_field ( "name" );
        self . version = _read_field ( "version" );
        self . description = _read_field ( "summary" );
        self . author = _read_field ( "author" );
        self . maintainer = None /* Option */;
        self . author_email = _read_field ( "author-email" );
        self . maintainer_email = None /* Option */;
        self . url = _read_field ( "home-page" );
        self . license = _read_field ( "license" );
        if "download-url" in msg {
        self . download_url = _read_field ( "download-url" );
        } else {
        self . download_url = None /* Option */;
        self . long_description = _read_field ( "description" );
        self . description = _read_field ( "summary" );
        if "keywords" in msg {
        self . keywords = _read_field ( "keywords" ) . split ( "," );
        self . platforms = _read_list ( "platform" );
        self . classifiers = _read_list ( "classifier" );
        if metadata_version == "1.1" {
        self . requires = _read_list ( "requires" );
        self . provides = _read_list ( "provides" );
        self . obsoletes = _read_list ( "obsoletes" );
        } else {
        self . requires = None /* Option */;
        self . provides = None /* Option */;
        self . obsoletes = None /* Option */;
        pub fn write_pkg_info ( &self, base_dir )  {
        "Write the PKG-INFO file into the release tree.
        ";
        // with scope: open ( os . path . join ( base_dir , "PKG-INFO" ) , "w" , {
        encoding = "UTF-8" ) as pkg_info ;
        self . write_pkg_file ( pkg_info );
        pub fn write_pkg_file ( &self, file )  {
        "Write the PKG-INFO format data to a file object.
        ";
        version = "1.0";
        if ( self . provides || self . requires || self . obsoletes or {
        self . classifiers || self . download_url ) :;
        version = "1.1";
        file . write ( "Metadata-Version: %s\n" % version );
        file . write ( "Name: %s\n" % self . get_name ( ) );
        file . write ( "Version: %s\n" % self . get_version ( ) );
        file . write ( "Summary: %s\n" % self . get_description ( ) );
        file . write ( "Home-page: %s\n" % self . get_url ( ) );
        file . write ( "Author: %s\n" % self . get_contact ( ) );
        file . write ( "Author-email: %s\n" % self . get_contact_email ( ) );
        file . write ( "License: %s\n" % self . get_license ( ) );
        if self . download_url {
        file . write ( "Download-URL: %s\n" % self . download_url );
        long_desc = rfc822_escape ( self . get_long_description ( ) );
        file . write ( "Description: %s\n" % long_desc );
        keywords = "," . join ( self . get_keywords ( ) );
        if keywords {
        file . write ( "Keywords: %s\n" % keywords );
        self . _write_list ( file , "Platform" , self . get_platforms ( ) );
        self . _write_list ( file , "Classifier" , self . get_classifiers ( ) );
        self . _write_list ( file , "Requires" , self . get_requires ( ) );
        self . _write_list ( file , "Provides" , self . get_provides ( ) );
        self . _write_list ( file , "Obsoletes" , self . get_obsoletes ( ) );
        pub fn _write_list ( &self, file , name , values )  {
        for value in values .iter() {
        file . write ( "%s: %s\n" % ( name , value ) );
        pub fn get_name ( self )  {
        return  self . name || "UNKNOWN";
        pub fn get_version ( self )  {
        return  self . version || "0.0.0";
        pub fn get_fullname ( self )  {
        return  "%s-%s" % ( self . get_name ( ) , self . get_version ( ) );
        pub fn get_author ( self )  {
        return  self . author || "UNKNOWN";
        pub fn get_author_email ( self )  {
        return  self . author_email || "UNKNOWN";
        pub fn get_maintainer ( self )  {
        return  self . maintainer || "UNKNOWN";
        pub fn get_maintainer_email ( self )  {
        return  self . maintainer_email || "UNKNOWN";
        pub fn get_contact ( self )  {
        return  self . maintainer || self . author || "UNKNOWN";
        pub fn get_contact_email ( self )  {
        return  self . maintainer_email || self . author_email || "UNKNOWN";
        pub fn get_url ( self )  {
        return  self . url || "UNKNOWN";
        pub fn get_license ( self )  {
        return  self . license || "UNKNOWN";
        get_licence = get_license;
        pub fn get_description ( self )  {
        return  self . description || "UNKNOWN";
        pub fn get_long_description ( self )  {
        return  self . long_description || "UNKNOWN";
        pub fn get_keywords ( self )  {
        return  self . keywords || [ ];
        pub fn set_keywords ( &self, value )  {
        self . keywords = _ensure_list ( value , "keywords" );
        pub fn get_platforms ( self )  {
        return  self . platforms || [ "UNKNOWN" ];
        pub fn set_platforms ( &self, value )  {
        self . platforms = _ensure_list ( value , "platforms" );
        pub fn get_classifiers ( self )  {
        return  self . classifiers || [ ];
        pub fn set_classifiers ( &self, value )  {
        self . classifiers = _ensure_list ( value , "classifiers" );
        pub fn get_download_url ( self )  {
        return  self . download_url || "UNKNOWN";
        pub fn get_requires ( self )  {
        return  self . requires || [ ];
        pub fn set_requires ( &self, value )  {
        import distutils . versionpredicate;
        for v in value .iter() {
        distutils . versionpredicate . VersionPredicate ( v );
        self . requires = list ( value );
        pub fn get_provides ( self )  {
        return  self . provides || [ ];
        pub fn set_provides ( &self, value )  {
        value = vec![ v . strip ( ).iter().map(|v| value ).collect();
        for v in value .iter() {
        import distutils . versionpredicate;
        distutils . versionpredicate . split_provision ( v );
        self . provides = value;
        pub fn get_obsoletes ( self )  {
        return  self . obsoletes || [ ];
        pub fn set_obsoletes ( &self, value )  {
        import distutils . versionpredicate;
        for v in value .iter() {
        distutils . versionpredicate . VersionPredicate ( v );
        self . obsoletes = list ( value );
        pub fn fix_help_options ( options )  {
        "Convert a 4-tuple 'help_options' list as found in various command
    classes to the 3-tuple form required by FancyGetopt.
    ";
        new_options = [ ];
        for help_tuple in options .iter() {
        new_options . append ( help_tuple [ 0 : 3 ] );
        return  new_options;
}

