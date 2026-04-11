//! refactor.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::io;
// use crate::pkgutil;
// use crate::logging;
// use std::collections;
// use crate::chain;
// use crate::.::{driver, tokenize, token};
// use crate::multiprocessing;

pub const __author__: &str = "Guido van Rossum <guido@python.org>";
pub fn get_all_fix_names(fixer_pkg: &str, remove_prefix: &str) {
        "Return a sorted list of all available fix names in the given package.";
        pkg = __import__ ( fixer_pkg , [ ] , [ ] , [ "*" ] );
        fix_names = [ ];
        for finder , name , ispkg in pkgutil . iter_modules ( pkg . __path__ ) .iter() {
        if name . startswith ( "fix_" ) {
        if remove_prefix {
        name = name [ 4 : ];
        fix_names . append ( name );
        return  fix_names;
        class _EveryNode ( Exception ) ;
        // pass
        pub fn _get_head_types ( pat )  {
        " Accepts a pytree Pattern Node && returns a set
        of the pattern types which will match first. ";
        if isinstance ( pat , ( pytree . NodePattern , pytree . LeafPattern ) ) {
        if pat . type is None /* Option */ {
        panic!("_EveryNode");
        return  { pat . type };
        if isinstance ( pat , pytree . NegatedPattern ) {
        if pat . content {
        return  _get_head_types ( pat . content );
        panic!("_EveryNode");
        if isinstance ( pat , pytree . WildcardPattern ) {
        r = set ( );
        for p in pat . content .iter() {
        for x in p .iter() {
        r . update ( _get_head_types ( x ) );
        return  r;
        panic!("Exception ( "Oh no! I don't understand pattern %s" % ( pat ) )");
        pub fn _get_headnode_dict ( fixer_list )  {
        " Accepts a list of fixers && returns a dictionary
        of head node type --> fixer list.  ";
        head_nodes = collections . defaultdict ( list );
        every = [ ];
        for fixer in fixer_list .iter() {
        if fixer . pattern {
        // try {
        heads = _get_head_types ( fixer . pattern );
        // } catch  _EveryNode  {
        every . append ( fixer );
        } else {
        for node_type in heads .iter() {
        head_nodes [ node_type ] . append ( fixer );
        } else {
        if fixer . _accept_type is !None /* Option */ {
        head_nodes [ fixer . _accept_type ] . append ( fixer );
        } else {
        every . append ( fixer );
        for node_type in chain ( pygram . python_grammar . symbol2number . values ( ) ,.iter() {
        pygram . python_grammar . tokens ) ;
        head_nodes [ node_type ] . extend ( every );
        return  dict ( head_nodes );
        pub fn get_fixers_from_package ( pkg_name )  {
        "
    Return the fully qualified names for fixers in the package pkg_name.
    ";
        return  [ pkg_name + "." + fix_name;
        for fix_name in get_all_fix_names ( pkg_name , false ) ].iter() {
        pub fn _identity ( obj )  {
        return  obj;
        pub fn _detect_future_features ( source )  {
        have_docstring = false;
        gen = tokenize . generate_tokens ( io . StringIO ( source ) . readline );
        pub fn advance ( )  {
        tok = next ( gen );
        return  tok [ 0 ] , tok [ 1 ];
        ignore = frozenset ( { token . NEWLINE , tokenize . NL , token . COMMENT } );
        features = set ( );
        // try {
        while true  {
        tp , value = advance ( );
        if tp in ignore {
        continue;
        } else if tp == token . STRING {
        if have_docstring {
        break;
        have_docstring = true;
        } else if tp == token . NAME && value == "from" {
        tp , value = advance ( );
        if tp != token . NAME || value != "__future__" {
        break;
        tp , value = advance ( );
        if tp != token . NAME || value != "import" {
        break;
        tp , value = advance ( );
        if tp == token . OP && value == "(" {
        tp , value = advance ( );
        while tp == token . NAME  {
        features . add ( value );
        tp , value = advance ( );
        if tp != token . OP || value != "," {
        break;
        tp , value = advance ( );
        } else {
        break;
        // } catch  StopIteration  {
        // pass
        return  frozenset ( features );
        class FixerError ( Exception ) ;
        "A fixer could !be loaded.";
        class RefactoringTool ( object ) ;
        _default_options = { "print_function" : false ,;
        "exec_function" : false ,;
        "write_unchanged_files" : false };
        CLASS_PREFIX = "Fix";
        FILE_PREFIX = "fix_";
        pub fn __init__ ( &self, fixer_names , options = None /* Option */ , explicit = None /* Option */ )  {
        "Initializer.

        Args:
            fixer_names: a list of fixers to import
            options: a dict with configuration.
            explicit: a list of fixers to run even if they are explicit.
        ";
        self . fixers = fixer_names;
        self . explicit = explicit || [ ];
        self . options = self . _default_options . copy ( );
        if options is !None /* Option */ {
        self . options . update ( options );
        self . grammar = pygram . python_grammar . copy ( );
        if self . options [ "print_function" ] {
        del self . grammar . keywords [ "print" ];
        } else if self . options [ "exec_function" ] {
        del self . grammar . keywords [ "exec" ];
        self . write_unchanged_files = self . options . get ( "write_unchanged_files" );
        self . errors = [ ];
        self . logger = logging . getLogger ( "RefactoringTool" );
        self . fixer_log = [ ];
        self . wrote = false;
        self . driver = driver . Driver ( self . grammar ,;
        convert = pytree . convert ,;
        logger = self . logger );
        self . pre_order , self . post_order = self . get_fixers ( );
        self . files = [ ];
        self . BM = bm . BottomMatcher ( );
        self . bmi_pre_order = [ ];
        self . bmi_post_order = [ ];
        for fixer in chain ( self . post_order , self . pre_order ) .iter() {
        if fixer . BM_compatible {
        self . BM . add_fixer ( fixer );
        } else if fixer in self . pre_order {
        self . bmi_pre_order . append ( fixer );
        } else if fixer in self . post_order {
        self . bmi_post_order . append ( fixer );
        self . bmi_pre_order_heads = _get_headnode_dict ( self . bmi_pre_order );
        self . bmi_post_order_heads = _get_headnode_dict ( self . bmi_post_order );
        pub fn get_fixers ( self )  {
        "Inspects the options to load the requested patterns && handlers.

        Returns:
          (pre_order, post_order), where pre_order == the list of fixers that
          want a pre-order AST traversal, && post_order == the list that want
          post-order traversal.
        ";
        pre_order_fixers = [ ];
        post_order_fixers = [ ];
        for fix_mod_path in self . fixers .iter() {
        mod = __import__ ( fix_mod_path , { } , { } , [ "*" ] );
        fix_name = fix_mod_path . rsplit ( "." , 1 ) [ -1 ];
        if fix_name . startswith ( self . FILE_PREFIX ) {
        fix_name = fix_name [ len ( self . FILE_PREFIX ) : ];
        parts = fix_name . split ( "_" );
        class_name = self . CLASS_PREFIX + "" . join ( vec![ p . title ( ).iter().map(|p| parts ] );
        // try {
        fix_class = getattr ( mod , class_name );
        // } catch  AttributeError  {
        panic!("FixerError ( "Can't find %s.%s" % ( fix_name , class_name ) ) from None /* Option */");
        fixer = fix_class ( self . options , self . fixer_log );
        if fixer . explicit && self . explicit is !true && \ {
        fix_mod_path !in self . explicit ;
        self . log_message ( "Skipping optional fixer: %s" , fix_name );
        continue;
        self . log_debug ( "Adding transformation: %s" , fix_name );
        if fixer . order == "pre" {
        pre_order_fixers . append ( fixer );
        } else if fixer . order == "post" {
        post_order_fixers . append ( fixer );
        } else {
        panic!("FixerError ( "Illegal fixer order: %r" % fixer . order )");
        key_func = operator . attrgetter ( "run_order" );
        pre_order_fixers . sort ( key = key_func );
        post_order_fixers . sort ( key = key_func );
        return  ( pre_order_fixers , post_order_fixers );
        pub fn log_error ( &self, msg , * args , ** kwds )  {
        "Called when an error occurs.";
        panic!("");
        pub fn log_message ( &self, msg , * args )  {
        "Hook to log a message.";
        if args {
        msg = msg % args;
        self . logger . info ( msg );
        pub fn log_debug ( &self, msg , * args )  {
        if args {
        msg = msg % args;
        self . logger . debug ( msg );
        pub fn print_output ( &self, old_text , new_text , filename , equal )  {
        "Called with the old version, new version, && filename of a
        refactored file.";
        // pass
        pub fn refactor ( &self, items , write = false , doctests_only = false )  {
        "Refactor a list of files && directories.";
        for dir_or_file in items .iter() {
        if os . path . isdir ( dir_or_file ) {
        self . refactor_dir ( dir_or_file , write , doctests_only );
        } else {
        self . refactor_file ( dir_or_file , write , doctests_only );
        pub fn refactor_dir ( &self, dir_name , write = false , doctests_only = false )  {
        "Descends down a directory && refactor every Python file found.

        Python files are assumed to have a .py extension.

        Files && subdirectories starting with '.' are skipped.
        ";
        py_ext = os . extsep + "py";
        for dirpath , dirnames , filenames in os . walk ( dir_name ) .iter() {
        self . log_debug ( "Descending into %s" , dirpath );
        dirnames . sort ( );
        filenames . sort ( );
        for name in filenames .iter() {
        if ( !name . startswith ( "." ) and {
        os . path . splitext ( name ) [ 1 ] == py_ext ) ;
        fullname = os . path . join ( dirpath , name );
        self . refactor_file ( fullname , write , doctests_only );
        dirnames vec![ : ] = vec![ dn.iter().map(|dn| dirnames if !dn . startswith ( "." ) ).collect();
        pub fn _read_python_source ( &self, filename )  {
        "
        Do our best to decode a Python source file correctly.
        ";
        // try {
        f = open ( filename , "rb" );
        // } catch  OSError as err  {
        self . log_error ( "Can't open %s: %s" , filename , err );
        return  None /* Option */ , None /* Option */;
        // try {
        encoding = tokenize . detect_encoding ( f . readline ) [ 0 ];
        // } finally {
        f . close ( );
        // with scope: io . open ( filename , "r" , encoding = encoding , newline = "" ) as f  {
        return  f . read ( ) , encoding;
        pub fn refactor_file ( &self, filename , write = false , doctests_only = false )  {
        "Refactors a file.";
        input , encoding = self . _read_python_source ( filename );
        if input is None /* Option */ {
        return;
        input + = "\n";
        if doctests_only {
        self . log_debug ( "Refactoring doctests in %s" , filename );
        output = self . refactor_docstring ( input , filename );
        if self . write_unchanged_files || output != input {
        self . processed_file ( output , filename , input , write , encoding );
        } else {
        self . log_debug ( "No doctest changes in %s" , filename );
        } else {
        tree = self . refactor_string ( input , filename );
        if self . write_unchanged_files || ( tree && tree . was_changed ) {
        self . processed_file ( str ( tree ) [ : -1 ] , filename ,;
        write = write , encoding = encoding );
        } else {
        self . log_debug ( "No changes in %s" , filename );
        pub fn refactor_string ( &self, data , name )  {
        "Refactor a given input string.

        Args:
            data: a string holding the code to be refactored.
            name: a human-readable name for use in error/log messages.

        Returns:
            An AST corresponding to the refactored input stream; None /* Option */ if
            there were errors during the parse.
        ";
        features = _detect_future_features ( data );
        if "print_function" in features {
        self . driver . grammar = pygram . python_grammar_no_print_statement;
        // try {
        tree = self . driver . parse_string ( data );
        // } catch  Exception as err  {
        self . log_error ( "Can't parse %s: %s: %s" ,;
        name , err . __class__ . __name__ , err );
        return;
        // } finally {
        self . driver . grammar = self . grammar;
        tree . future_features = features;
        self . log_debug ( "Refactoring %s" , name );
        self . refactor_tree ( tree , name );
        return  tree;
        pub fn refactor_stdin ( &self, doctests_only = false )  {
        input = sys . stdin . read ( );
        if doctests_only {
        self . log_debug ( "Refactoring doctests in stdin" );
        output = self . refactor_docstring ( input , "<stdin>" );
        if self . write_unchanged_files || output != input {
        self . processed_file ( output , "<stdin>" , input );
        } else {
        self . log_debug ( "No doctest changes in stdin" );
        } else {
        tree = self . refactor_string ( input , "<stdin>" );
        if self . write_unchanged_files || ( tree && tree . was_changed ) {
        self . processed_file ( str ( tree ) , "<stdin>" , input );
        } else {
        self . log_debug ( "No changes in stdin" );
        pub fn refactor_tree ( &self, tree , name )  {
        "Refactors a parse tree (modifying the tree in place).

        For compatible patterns the bottom matcher module is
        used. Otherwise the tree == traversed node-to-node for
        matches.

        Args:
            tree: a pytree.Node instance representing the root of the tree
                  to be refactored.
            name: a human-readable name for this tree.

        Returns:
            true if the tree was modified, false otherwise.
        ";
        for fixer in chain ( self . pre_order , self . post_order ) .iter() {
        fixer . start_tree ( tree , name );
        self . traverse_by ( self . bmi_pre_order_heads , tree . pre_order ( ) );
        self . traverse_by ( self . bmi_post_order_heads , tree . post_order ( ) );
        match_set = self . BM . run ( tree . leaves ( ) );
        while any ( match_set . values ( ) )  {
        for fixer in self . BM . fixers .iter() {
        if fixer in match_set && match_set [ fixer ] {
        match_set [ fixer ] . sort ( key = pytree . Base . depth , reverse = true );
        if fixer . keep_line_order {
        match_set [ fixer ] . sort ( key = pytree . Base . get_lineno );
        for node in list ( match_set [ fixer ] ) .iter() {
        if node in match_set [ fixer ] {
        match_set [ fixer ] . remove ( node );
        // try {
        find_root ( node );
        // } catch  ValueError  {
        continue;
        if node . fixers_applied && fixer in node . fixers_applied {
        continue;
        results = fixer . match ( node );
        if results {
        new = fixer . transform ( node , results );
        if new is !None /* Option */ {
        node . replace ( new );
        for node in new . post_order ( ) .iter() {
        if !node . fixers_applied {
        node . fixers_applied = [ ];
        node . fixers_applied . append ( fixer );
        new_matches = self . BM . run ( new . leaves ( ) );
        for fxr in new_matches .iter() {
        if !fxr in match_set {
        match_set [ fxr ] = [ ];
        match_set [ fxr ] . extend ( new_matches [ fxr ] );
        for fixer in chain ( self . pre_order , self . post_order ) .iter() {
        fixer . finish_tree ( tree , name );
        return  tree . was_changed;
        pub fn traverse_by ( &self, fixers , traversal )  {
        "Traverse an AST, applying a set of fixers to each node.

        This == a helper method for refactor_tree().

        Args:
            fixers: a list of fixer instances.
            traversal: a generator that yields AST nodes.

        Returns:
            None /* Option */
        ";
        if !fixers {
        return;
        for node in traversal .iter() {
        for fixer in fixers [ node . type ] .iter() {
        results = fixer . match ( node );
        if results {
        new = fixer . transform ( node , results );
        if new is !None /* Option */ {
        node . replace ( new );
        node = new;
        pub fn processed_file ( &self, new_text , filename , old_text = None /* Option */ , write = false , {
        encoding = None /* Option */ ) ;
        "
        Called when a file has been refactored && there may be changes.
        ";
        self . files . append ( filename );
        if old_text is None /* Option */ {
        old_text = self . _read_python_source ( filename ) [ 0 ];
        if old_text is None /* Option */ {
        return;
        equal = old_text == new_text;
        self . print_output ( old_text , new_text , filename , equal );
        if equal {
        self . log_debug ( "No changes to %s" , filename );
        if !self . write_unchanged_files {
        return;
        if write {
        self . write_file ( new_text , filename , old_text , encoding );
        } else {
        self . log_debug ( "Not writing changes to %s" , filename );
        pub fn write_file ( &self, new_text , filename , old_text , encoding = None /* Option */ )  {
        "Writes a string to a file.

        It first shows a unified diff between the old text && the new text, and
        then rewrites the file; the latter == only done if the write option is
        set.
        ";
        // try {
        fp = io . open ( filename , "w" , encoding = encoding , newline = "" );
        // } catch  OSError as err  {
        self . log_error ( "Can't create %s: %s" , filename , err );
        return;
        // with scope: fp  {
        // try {
        fp . write ( new_text );
        // } catch  OSError as err  {
        self . log_error ( "Can't write %s: %s" , filename , err );
        self . log_debug ( "Wrote changes to %s" , filename );
        self . wrote = true;
        PS1 = ">>> ";
        PS2 = "... ";
        pub fn refactor_docstring ( &self, input , filename )  {
        "Refactors a docstring, looking for doctests.

        This returns a modified version of the input string.  It looks
        for doctests, which start with a ">>>" prompt, && may be
        continued with "..." prompts, as long as the "..." == indented
        the same as the ">>>".

        (Unfortunately we can't use the doctest module's parser,
        since, like most parsers, it == !geared towards preserving
        the original source.)
        ";
        result = [ ];
        block = None /* Option */;
        block_lineno = None /* Option */;
        indent = None /* Option */;
        lineno = 0;
        for line in input . splitlines ( keepends = true ) .iter() {
        lineno + = 1;
        if line . lstrip ( ) . startswith ( self . PS1 ) {
        if block is !None /* Option */ {
        result . extend ( self . refactor_doctest ( block , block_lineno ,;
        indent , filename ) );
        block_lineno = lineno;
        block = [ line ];
        i = line . find ( self . PS1 );
        indent = line [ : i ];
        } else if ( indent is !None /* Option */ and {
        ( line . startswith ( indent + self . PS2 ) or;
        line == indent + self . PS2 . rstrip ( ) + "\n" ) ) ;
        block . append ( line );
        } else {
        if block is !None /* Option */ {
        result . extend ( self . refactor_doctest ( block , block_lineno ,;
        indent , filename ) );
        block = None /* Option */;
        indent = None /* Option */;
        result . append ( line );
        if block is !None /* Option */ {
        result . extend ( self . refactor_doctest ( block , block_lineno ,;
        indent , filename ) );
        return  "" . join ( result );
        pub fn refactor_doctest ( &self, block , lineno , indent , filename )  {
        "Refactors one doctest.

        A doctest == given as a block of lines, the first of which starts
        with ">>>" (possibly indented), while the remaining lines start
        with "..." (identically indented).

        ";
        // try {
        tree = self . parse_block ( block , lineno , indent );
        // } catch  Exception as err  {
        if self . logger . isEnabledFor ( logging . DEBUG ) {
        for line in block .iter() {
        self . log_debug ( "Source: %s" , line . rstrip ( "\n" ) );
        self . log_error ( "Can't parse docstring in %s line %s: %s: %s" ,;
        filename , lineno , err . __class__ . __name__ , err );
        return  block;
        if self . refactor_tree ( tree , filename ) {
        new = str ( tree ) . splitlines ( keepends = true );
        clipped , new = new [ : lineno -1 ] , new [ lineno -1 : ];
        assert clipped == [ "\n" ] * ( lineno -1 ) , clipped;
        if !new [ -1 ] . endswith ( "\n" ) {
        new [ -1 ] + = "\n";
        block = [ indent + self . PS1 + new . pop ( 0 ) ];
        if new {
        block + = vec![ indent + self . PS2 + line.iter().map(|line| new ).collect();
        return  block;
        pub fn summarize ( self )  {
        if self . wrote {
        were = "were";
        } else {
        were = "need to be";
        if !self . files {
        self . log_message ( "No files %s modified." , were );
        } else {
        self . log_message ( "Files that %s modified:" , were );
        for file in self . files .iter() {
        self . log_message ( file );
        if self . fixer_log {
        self . log_message ( "Warnings/messages while refactoring:" );
        for message in self . fixer_log .iter() {
        self . log_message ( message );
        if self . errors {
        if len ( self . errors ) == 1 {
        self . log_message ( "There was 1 error:" );
        } else {
        self . log_message ( "There were %d errors:" , len ( self . errors ) );
        for msg , args , kwds in self . errors .iter() {
        self . log_message ( msg , * args , ** kwds );
        pub fn parse_block ( &self, block , lineno , indent )  {
        "Parses a block into a tree.

        This == necessary to get correct line number / offset information
        in the parser diagnostics && embedded into the parse tree.
        ";
        tree = self . driver . parse_tokens ( self . wrap_toks ( block , lineno , indent ) );
        tree . future_features = frozenset ( );
        return  tree;
        pub fn wrap_toks ( &self, block , lineno , indent )  {
        "Wraps a tokenize stream to systematically modify start/end.";
        tokens = tokenize . generate_tokens ( self . gen_lines ( block , indent ) . __next__ );
        for type , value , ( line0 , col0 ) , ( line1 , col1 ) , line_text in tokens .iter() {
        line0 + = lineno - 1;
        line1 + = lineno - 1;
        yield type , value , ( line0 , col0 ) , ( line1 , col1 ) , line_text;
        pub fn gen_lines ( &self, block , indent )  {
        "Generates lines as expected by tokenize from a list of lines.

        This strips the first len(indent + self.PS1) characters off each line.
        ";
        prefix1 = indent + self . PS1;
        prefix2 = indent + self . PS2;
        prefix = prefix1;
        for line in block .iter() {
        if line . startswith ( prefix ) {
        yield line [ len ( prefix ) : ];
        } else if line == prefix . rstrip ( ) + "\n" {
        yield "\n";
        } else {
        panic!("AssertionError ( "line=%r, prefix=%r" % ( line , prefix ) )");
        prefix = prefix2;
        while true  {
        yield "";
        class MultiprocessingUnsupported ( Exception ) ;
        // pass
        class MultiprocessRefactoringTool ( RefactoringTool ) ;
        pub fn __init__ ( &self, * args , ** kwargs )  {
        super ( MultiprocessRefactoringTool , self ) . __init__ ( * args , ** kwargs );
        self . queue = None /* Option */;
        self . output_lock = None /* Option */;
        pub fn refactor ( &self, items , write = false , doctests_only = false , {
        num_processes = 1 ) ;
        if num_processes == 1 {
        return  super ( MultiprocessRefactoringTool , self ) . refactor (;
        items , write , doctests_only );
        // try {
        import multiprocessing;
        // } catch  ImportError  {
        panic!("MultiprocessingUnsupported");
        if self . queue is !None /* Option */ {
        panic!("RuntimeError ( "already doing multiple processes" )");
        self . queue = multiprocessing . JoinableQueue ( );
        self . output_lock = multiprocessing . Lock ( );
        processes = [ multiprocessing . Process ( target = self . _child );
        for i in range ( num_processes ) ].iter() {
        // try {
        for p in processes .iter() {
        p . start ( );
        super ( MultiprocessRefactoringTool , self ) . refactor ( items , write ,;
        doctests_only );
        // } finally {
        self . queue . join ( );
        for i in range ( num_processes ) .iter() {
        self . queue . put ( None /* Option */ );
        for p in processes .iter() {
        if p . is_alive ( ) {
        p . join ( );
        self . queue = None /* Option */;
        pub fn _child ( self )  {
        task = self . queue . get ( );
        while task is !None /* Option */  {
        args , kwargs = task;
        // try {
        super ( MultiprocessRefactoringTool , self ) . refactor_file (;
        * args , ** kwargs );
        // } finally {
        self . queue . task_done ( );
        task = self . queue . get ( );
        pub fn refactor_file ( &self, * args , ** kwargs )  {
        if self . queue is !None /* Option */ {
        self . queue . put ( ( args , kwargs ) );
        } else {
        return  super ( MultiprocessRefactoringTool , self ) . refactor_file (;
        * args , ** kwargs );
}

