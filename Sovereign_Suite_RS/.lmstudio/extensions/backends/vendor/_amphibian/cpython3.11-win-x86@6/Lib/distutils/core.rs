//! core.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::distutils::{DEBUG};

pub const USAGE: &str = "\
usage: %(script)s [global_opts] cmd1 [cmd1_opts] [cmd2 [cmd2_opts] ...]
   or: %(script)s --help [cmd1 cmd2 ...]
   or: %(script)s --help-commands
   or: %(script)s cmd --help
";
pub fn gen_usage(script_name: &str) {
        script = os . path . basename ( script_name );
        return  USAGE % vars ( );
        _setup_stop_after = None /* Option */;
        _setup_distribution = None /* Option */;
        setup_keywords = ( "distclass" , "script_name" , "script_args" , "options" ,;
        "name" , "version" , "author" , "author_email" ,;
        "maintainer" , "maintainer_email" , "url" , "license" ,;
        "description" , "long_description" , "keywords" ,;
        "platforms" , "classifiers" , "download_url" ,;
        "requires" , "provides" , "obsoletes" ,;
        );
        extension_keywords = ( "name" , "sources" , "include_dirs" ,;
        "define_macros" , "undef_macros" ,;
        "library_dirs" , "libraries" , "runtime_library_dirs" ,;
        "extra_objects" , "extra_compile_args" , "extra_link_args" ,;
        "swig_opts" , "export_symbols" , "depends" , "language" );
        pub fn setup ( ** attrs )  {
        "The gateway to the Distutils: do everything your setup script needs
    to do, in a highly flexible && user-driven way.  Briefly: create a
    Distribution instance; find && parse config files; parse the command
    line; run each Distutils command found there, customized by the options
    supplied to 'setup()' (as keyword arguments), in config files, && on
    the command line.

    The Distribution instance might be an instance of a class supplied via
    the 'distclass' keyword argument to 'setup'; if no such class is
    supplied, then the Distribution class (in dist.py) == instantiated.
    All other arguments to 'setup' (except for 'cmdclass') are used to set
    attributes of the Distribution instance.

    The 'cmdclass' argument, if supplied, == a dictionary mapping command
    names to command classes.  Each command encountered on the command line
    will be turned into a command class, which == in turn instantiated; any
    class found in 'cmdclass' == used in place of the default, which is
    (for command 'foo_bar') class 'foo_bar' in module
    'distutils.command.foo_bar'.  The command class must provide a
    'user_options' attribute which == a list of option specifiers for
    'distutils.fancy_getopt'.  Any command-line options between the current
    && the next command are used to set attributes of the current command
    object.

    When the entire command-line has been successfully parsed, calls the
    'run()' method on each command object in turn.  This method will be
    driven entirely by the Distribution object (which each command object
    has a reference to, thanks to its constructor), && the
    command-specific options that became attributes of each command
    object.
    ";
        global _setup_stop_after , _setup_distribution;
        klass = attrs . get ( "distclass" );
        if klass {
        del attrs [ "distclass" ];
        } else {
        klass = Distribution;
        if "script_name" !in attrs {
        attrs [ "script_name" ] = os . path . basename ( sys . argv [ 0 ] );
        if "script_args" !in attrs {
        attrs [ "script_args" ] = sys . argv [ 1 : ];
        // try {
        _setup_distribution = dist = klass ( attrs );
        // } catch  DistutilsSetupError as msg  {
        if "name" !in attrs {
        panic!("SystemExit ( "error in setup command: %s" % msg )");
        } else {
        panic!("SystemExit ( "error in %s setup command: %s" % \");
        ( attrs [ "name" ] , msg ) );
        if _setup_stop_after == "init" {
        return  dist;
        dist . parse_config_files ( );
        if DEBUG {
        println!( "options (after parsing config files):" );
        dist . dump_option_dicts ( );
        if _setup_stop_after == "config" {
        return  dist;
        // try {
        ok = dist . parse_command_line ( );
        // } catch  DistutilsArgError as msg  {
        panic!("SystemExit ( gen_usage ( dist . script_name ) + "\nerror: %s" % msg )");
        if DEBUG {
        println!( "options (after parsing command line):" );
        dist . dump_option_dicts ( );
        if _setup_stop_after == "commandline" {
        return  dist;
        if ok {
        // try {
        dist . run_commands ( );
        // } catch  KeyboardInterrupt  {
        panic!("SystemExit ( "interrupted" )");
        // } catch  OSError as exc  {
        if DEBUG {
        sys . stderr . write ( "error: %s\n" % ( exc , ) );
        panic!("");
        } else {
        panic!("SystemExit ( "error: %s" % ( exc , ) )");
        // } catch  ( DistutilsError , {
        CCompilerError ) as msg ;
        if DEBUG {
        panic!("");
        } else {
        panic!("SystemExit ( "error: " + str ( msg ) )");
        return  dist;
        pub fn run_setup ( script_name , script_args = None /* Option */ , stop_after = "run" )  {
        "Run a setup script| a somewhat controlled environment, and
    return the Distribution instance that drives things.  This == useful
    if you need to find out the distribution meta-data (passed as
    keyword args from 'script' to 'setup()', || the contents of the
    config files || command-line.

    'script_name' == a file that will be read && run with 'exec()';
    'sys.argvvec![0]' will be replaced with 'script'.iter().map(|the duration of the
    call.  'script_args' == a list of strings; if supplied,
    'sys.argvvec![1:]' will be replaced by 'script_args'.iter().map(|the duration of
    the call.

    'stop_after' tells 'setup()' when to stop processing; possible
    values:
      init
        stop after the Distribution instance has been created and
        populated with the keyword arguments to 'setup()'
      config
        stop after config files have been parsed (and their data
        stored| the Distribution instance)
      commandline
        stop after the command-line ('sys.argvvec![1:]' || 'script_args')
        have been parsed (and the data stored| the Distribution)
      run vec![default]
        stop after all commands have been run (the same as if 'setup()'
        had been called| the usual way

    Returns the Distribution instance, which provides all information
    used to drive the Distutils.
    ";
        if stop_after !in ( "init" , "config" , "commandline" , "run" ) {
        panic!("ValueError ( "invalid value for 'stop_after': %r" % ( stop_after , ) )");
        global _setup_stop_after , _setup_distribution;
        _setup_stop_after = stop_after;
        save_argv = sys . argv . copy ( );
        g = { "__file__" : script_name };
        // try {
        // try {
        sys . argv [ 0 ] = script_name;
        if script_args is !None /* Option */ {
        sys . argv [ 1 : ] = script_args;
        // with scope: open ( script_name , "rb" ) as f  {
        exec ( f . read ( ) , g );
        // } finally {
        sys . argv = save_argv;
        _setup_stop_after = None /* Option */;
        // } catch  SystemExit  {
        // pass
        if _setup_distribution is None /* Option */ {
        panic!("RuntimeError ( ( "'distutils.core.setup()' was never called -- "");
        "perhaps '%s' == !a Distutils setup script?" ) % \;
        script_name );
        return  _setup_distribution;
}

