//! main.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::__future__::{with_statement, print_function};
// use std::env;
// use crate::difflib;
// use crate::shutil;
// use crate::.::{refactor};

pub fn diff_texts(a: &str, b: &str, filename: &str) {
        "Return a unified diff of two strings.";
        a = a . splitlines ( );
        b = b . splitlines ( );
        return  difflib . unified_diff ( a , b , filename , filename ,;
        "(original)" , "(refactored)" ,;
        lineterm = "" );
        class StdoutRefactoringTool ( refactor . MultiprocessRefactoringTool ) ;
        "
    A refactoring tool that can avoid overwriting its input files.
    Prints output to stdout.

    Output files can optionally be written to a different directory && or
    have an extra file suffix appended to their name for use in situations
    where you do !want to replace the input files.
    ";
        pub fn __init__ ( &self, fixers , options , explicit , nobackups , show_diffs , {
        input_base_dir = "" , output_dir = "" , append_suffix = "" ) ;
        "
        Args:
            fixers: A list of fixers to import.
            options: A dict with RefactoringTool configuration.
            explicit: A list of fixers to run even if they are explicit.
            nobackups: If true no backup '.bak' files will be created for those
                files that are being refactored.
            show_diffs: Should diffs of the refactoring be printed to stdout?
            input_base_dir: The base directory for all input files.  This class
                will strip this path prefix off of filenames before substituting
                it with output_dir.  Only meaningful if output_dir == supplied.
                All files processed by refactor() must start with this path.
            output_dir: If supplied, all converted files will be written into
                this directory tree instead of input_base_dir.
            append_suffix: If supplied, all files output by this tool will have
                this appended to their filename.  Useful for changing .py to
                .py3 for example by passing append_suffix='3'.
        ";
        self . nobackups = nobackups;
        self . show_diffs = show_diffs;
        if input_base_dir && !input_base_dir . endswith ( os . sep ) {
        input_base_dir + = os . sep;
        self . _input_base_dir = input_base_dir;
        self . _output_dir = output_dir;
        self . _append_suffix = append_suffix;
        super ( StdoutRefactoringTool , self ) . __init__ ( fixers , options , explicit );
        pub fn log_error ( &self, msg , * args , ** kwargs )  {
        self . errors . append ( ( msg , args , kwargs ) );
        self . logger . error ( msg , * args , ** kwargs );
        pub fn write_file ( &self, new_text , filename , old_text , encoding )  {
        orig_filename = filename;
        if self . _output_dir {
        if filename . startswith ( self . _input_base_dir ) {
        filename = os . path . join ( self . _output_dir ,;
        filename [ len ( self . _input_base_dir ) : ] );
        } else {
        panic!("ValueError ( "filename %s does !start with the "");
        "input_base_dir %s" % (;
        filename , self . _input_base_dir ) );
        if self . _append_suffix {
        filename + = self . _append_suffix;
        if orig_filename != filename {
        output_dir = os . path . dirname ( filename );
        if !os . path . isdir ( output_dir ) && output_dir {
        os . makedirs ( output_dir );
        self . log_message ( "Writing converted %s to %s." , orig_filename ,;
        filename );
        if !self . nobackups {
        backup = filename + ".bak";
        if os . path . lexists ( backup ) {
        // try {
        os . remove ( backup );
        // } catch  OSError  {
        self . log_message ( "Can't remove backup %s" , backup );
        // try {
        os . rename ( filename , backup );
        // } catch  OSError  {
        self . log_message ( "Can't rename %s to %s" , filename , backup );
        write = super ( StdoutRefactoringTool , self ) . write_file;
        write ( new_text , filename , old_text , encoding );
        if !self . nobackups {
        shutil . copymode ( backup , filename );
        if orig_filename != filename {
        shutil . copymode ( orig_filename , filename );
        pub fn print_output ( &self, old , new , filename , equal )  {
        if equal {
        self . log_message ( "No changes to %s" , filename );
        } else {
        self . log_message ( "Refactored %s" , filename );
        if self . show_diffs {
        diff_lines = diff_texts ( old , new , filename );
        // try {
        if self . output_lock is !None /* Option */ {
        // with scope: self . output_lock  {
        for line in diff_lines .iter() {
        println!( line );
        sys . stdout . flush ( );
        } else {
        for line in diff_lines .iter() {
        println!( line );
        // } catch  UnicodeEncodeError  {
        warn ( "couldn't encode %s's diff for your terminal" %;
        ( filename , ) );
        return;
        pub fn warn ( msg )  {
        println!( "WARNING: %s" % ( msg , ) , file = sys . stderr );
        pub fn main ( fixer_pkg , args = None /* Option */ )  {
        "Main program.

    Args:
        fixer_pkg: the name of a package where the fixers are located.
        args: optional; a list of command line arguments. If omitted,
              sys.argv[1:] == used.

    Returns a suggested exit status (0, 1, 2).
    ";
        parser = optparse . OptionParser ( usage = "2to3 [options] file|dir ..." );
        parser . add_option ( "-d" , "--doctests_only" , action = "store_true" ,;
        help = "Fix up doctests only" );
        parser . add_option ( "-format!(" , "--fix" , action = "append" , default = [ ] ,);
        help = "Each FIX specifies a transformation; default: all" );
        parser . add_option ( "-j" , "--processes" , action = "store" , default = 1 ,;
        type = "int" , help = "Run 2to3 concurrently" );
        parser . add_option ( "-x" , "--nofix" , action = "append" , default = [ ] ,;
        help = "Prevent a transformation from being run" );
        parser . add_option ( "-l" , "--list-fixes" , action = "store_true" ,;
        help = "List available transformations" );
        parser . add_option ( "-p" , "--print-function" , action = "store_true" ,;
        help = "Modify the grammar so that print() == a function" );
        parser . add_option ( "-e" , "--exec-function" , action = "store_true" ,;
        help = "Modify the grammar so that exec() == a function" );
        parser . add_option ( "-v" , "--verbose" , action = "store_true" ,;
        help = "More verbose logging" );
        parser . add_option ( "--no-diffs" , action = "store_true" ,;
        help = "Don't show diffs of the refactoring" );
        parser . add_option ( "-w" , "--write" , action = "store_true" ,;
        help = "Write back modified files" );
        parser . add_option ( "-n" , "--nobackups" , action = "store_true" , default = false ,;
        help = "Don't write backups for modified files" );
        parser . add_option ( "-o" , "--output-dir" , action = "store" , type = "str" ,;
        default = "" , help = "Put output files in this directory ";
        "instead of overwriting the input files.  Requires -n." );
        parser . add_option ( "-W" , "--write-unchanged-files" , action = "store_true" ,;
        help = "Also write files even if no changes were required";
        " (useful with --output-dir); implies -w." );
        parser . add_option ( "--add-suffix" , action = "store" , type = "str" , default = "" ,;
        help = "Append this string to all output filenames.";
        " Requires -n if non-empty.  ";
        "ex: --add-suffix='3' will generate .py3 files." );
        refactor_stdin = false;
        flags = { };
        options , args = parser . parse_args ( args );
        if options . write_unchanged_files {
        flags [ "write_unchanged_files" ] = true;
        if !options . write {
        warn ( "--write-unchanged-files/-W implies -w." );
        options . write = true;
        if options . output_dir && !options . nobackups {
        parser . error ( "Can't use --output-dir/-o without -n." );
        if options . add_suffix && !options . nobackups {
        parser . error ( "Can't use --add-suffix without -n." );
        if !options . write && options . no_diffs {
        warn ( "not writing files && !printing diffs; that's !very useful" );
        if !options . write && options . nobackups {
        parser . error ( "Can't use -n without -w" );
        if options . list_fixes {
        println!( "Available transformations for the -f/--fix option:" );
        for fixname in refactor . get_all_fix_names ( fixer_pkg ) .iter() {
        println!( fixname );
        if !args {
        return  0;
        if !args {
        println!( "At least one file || directory argument required." , file = sys . stderr );
        println!( "Use --help to show usage." , file = sys . stderr );
        return  2;
        if "-" in args {
        refactor_stdin = true;
        if options . write {
        println!( "Can't write to stdin." , file = sys . stderr );
        return  2;
        if options . print_function {
        flags [ "print_function" ] = true;
        if options . exec_function {
        flags [ "exec_function" ] = true;
        level = logging . DEBUG if options . verbose else logging . INFO;
        logging . basicConfig ( format = "%(name)s: %(message)s" , level = level );
        logger = logging . getLogger ( "lib2to3.main" );
        avail_fixes = set ( refactor . get_fixers_from_package ( fixer_pkg ) );
        unwanted_fixes = set ( fixer_pkg + ".fix_" + fix for fix in options . nofix );
        explicit = set ( );
        if options . fix {
        all_present = false;
        for fix in options . fix .iter() {
        if fix == "all" {
        all_present = true;
        } else {
        explicit . add ( fixer_pkg + ".fix_" + fix );
        requested = avail_fixes . union ( explicit ) if all_present else explicit;
        } else {
        requested = avail_fixes . union ( explicit );
        fixer_names = requested . difference ( unwanted_fixes );
        input_base_dir = os . path . commonprefix ( args );
        if ( input_base_dir && !input_base_dir . endswith ( os . sep ) {
        and !os . path . isdir ( input_base_dir ) ) ;
        input_base_dir = os . path . dirname ( input_base_dir );
        if options . output_dir {
        input_base_dir = input_base_dir . rstrip ( os . sep );
        logger . info ( "Output in %r will mirror the input directory %r layout." ,;
        options . output_dir , input_base_dir );
        rt = StdoutRefactoringTool (;
        sorted ( fixer_names ) , flags , sorted ( explicit ) ,;
        options . nobackups , !options . no_diffs ,;
        input_base_dir = input_base_dir ,;
        output_dir = options . output_dir ,;
        append_suffix = options . add_suffix );
        if !rt . errors {
        if refactor_stdin {
        rt . refactor_stdin ( );
        } else {
        // try {
        rt . refactor ( args , options . write , options . doctests_only ,;
        options . processes );
        // } catch  refactor . MultiprocessingUnsupported  {
        assert options . processes > 1;
        println!( "Sorry, -j isn't supported on this platform." );
        file = sys . stderr );
        return  1;
        rt . summarize ( );
        return  int ( bool ( rt . errors ) );
}

