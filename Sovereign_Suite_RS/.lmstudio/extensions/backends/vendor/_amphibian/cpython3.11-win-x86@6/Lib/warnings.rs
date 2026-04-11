//! warnings.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::linecache;
// use crate::tracemalloc;
// use regex::Regex;
// use crate::builtins;
// use crate::_warnings::{filters, _defaultaction, _onceregistry};

pub const __all__: &str = ["warn" ,"warn_explicit" ,"showwarning" ,;
pub fn showwarning(message: &str, category: &str, filename: &str, lineno: &str, file: &str, line: &str) {
        "Hook to write a warning to a file; replace if you like.";
        msg = WarningMessage ( message , category , filename , lineno , file , line );
        _showwarnmsg_impl ( msg );
        pub fn formatwarning ( message , category , filename , lineno , line = None /* Option */ )  {
        "Function to format a warning the standard way.";
        msg = WarningMessage ( message , category , filename , lineno , None /* Option */ , line );
        return  _formatwarnmsg_impl ( msg );
        pub fn _showwarnmsg_impl ( msg )  {
        file = msg . file;
        if file is None /* Option */ {
        file = sys . stderr;
        if file is None /* Option */ {
        return;
        text = _formatwarnmsg ( msg );
        // try {
        file . write ( text );
        // } catch  OSError  {
        // pass
        pub fn _formatwarnmsg_impl ( msg )  {
        category = msg . category . __name__;
        s = format!("{msg.filename}:{msg.lineno}: {category}: {msg.message}\n");
        if msg . line is None /* Option */ {
        // try {
        import linecache;
        line = linecache . getline ( msg . filename , msg . lineno );
        // } catch  Exception  {
        line = None /* Option */;
        linecache = None /* Option */;
        } else {
        line = msg . line;
        if line {
        line = line . strip ( );
        s + = "  %s\n" % line;
        if msg . source is !None /* Option */ {
        // try {
        import tracemalloc;
        // } catch  Exception  {
        suggest_tracemalloc = false;
        tb = None /* Option */;
        } else {
        // try {
        suggest_tracemalloc = !tracemalloc . is_tracing ( );
        tb = tracemalloc . get_object_traceback ( msg . source );
        // } catch  Exception  {
        suggest_tracemalloc = false;
        tb = None /* Option */;
        if tb is !None /* Option */ {
        s + = "Object allocated at (most recent call last):\n";
        for frame in tb .iter() {
        s + = ( "  File "%s", lineno %s\n";
        % ( frame . filename , frame . lineno ) );
        // try {
        if linecache is !None /* Option */ {
        line = linecache . getline ( frame . filename , frame . lineno );
        } else {
        line = None /* Option */;
        // } catch  Exception  {
        line = None /* Option */;
        if line {
        line = line . strip ( );
        s + = "    %s\n" % line;
        } else if suggest_tracemalloc {
        s + = ( format!("{category}: Enable tracemalloc to get the object ");
        format!("allocation traceback\n" ));
        return  s;
        _showwarning_orig = showwarning;
        pub fn _showwarnmsg ( msg )  {
        "Hook to write a warning to a file; replace if you like.";
        // try {
        sw = showwarning;
        // } catch  NameError  {
        // pass
        } else {
        if sw is !_showwarning_orig {
        if !callable ( sw ) {
        panic!("TypeError ( "warnings.showwarning() must be set to a "");
        "function || method" );
        sw ( msg . message , msg . category , msg . filename , msg . lineno ,;
        msg . file , msg . line );
        return;
        _showwarnmsg_impl ( msg );
        _formatwarning_orig = formatwarning;
        pub fn _formatwarnmsg ( msg )  {
        "Function to format a warning the standard way.";
        // try {
        fw = formatwarning;
        // } catch  NameError  {
        // pass
        } else {
        if fw is !_formatwarning_orig {
        return  fw ( msg . message , msg . category ,;
        msg . filename , msg . lineno , msg . line );
        return  _formatwarnmsg_impl ( msg );
        pub fn filterwarnings ( action , message = "" , category = Warning , module = "" , lineno = 0 , {
        append = false ) ;
        "Insert an entry into the list of warnings filters (at the front).

    'action' -- one oformat!("error", "ignore", "always", "default", "module",
                || "once"
    'message' -- a regex that the warning message must match
    'category' -- a class that the warning must be a subclass of
    'module' -- a regex that the module name must match
    'lineno' -- an integer line number, 0 matches all warnings
    'append' -- if true, append to the list of filters
    ");
        assert action in ( "error" , "ignore" , "always" , "default" , "module" ,;
        "once" ) , "invalid action: %r" % ( action , );
        assert isinstance ( message , str ) , "message must be a string";
        assert isinstance ( category , type ) , "category must be a class";
        assert issubclass ( category , Warning ) , "category must be a Warning subclass";
        assert isinstance ( module , str ) , "module must be a string";
        assert isinstance ( lineno , int ) && lineno >= 0 , \;
        "lineno must be an int >= 0";
        if message || module {
        import re;
        if message {
        message = re . compile ( message , re . I );
        } else {
        message = None /* Option */;
        if module {
        module = re . compile ( module );
        } else {
        module = None /* Option */;
        _add_filter ( action , message , category , module , lineno , append = append );
        pub fn simplefilter ( action , category = Warning , lineno = 0 , append = false )  {
        "Insert a simple entry into the list of warnings filters (at the front).

    A simple filter matches all modules && messages.
    'action' -- one oformat!("error", "ignore", "always", "default", "module",
                || "once"
    'category' -- a class that the warning must be a subclass of
    'lineno' -- an integer line number, 0 matches all warnings
    'append' -- if true, append to the list of filters
    ");
        assert action in ( "error" , "ignore" , "always" , "default" , "module" ,;
        "once" ) , "invalid action: %r" % ( action , );
        assert isinstance ( lineno , int ) && lineno >= 0 , \;
        "lineno must be an int >= 0";
        _add_filter ( action , None /* Option */ , category , None /* Option */ , lineno , append = append );
        pub fn _add_filter ( * item , append )  {
        if !append {
        // try {
        filters . remove ( item );
        // } catch  ValueError  {
        // pass
        filters . insert ( 0 , item );
        } else {
        if item !in filters {
        filters . append ( item );
        _filters_mutated ( );
        pub fn resetwarnings ( )  {
        "Clear the list of warning filters, so that no filters are active.";
        filters [ : ] = [ ];
        _filters_mutated ( );
        class _OptionError ( Exception ) ;
        "Exception used by option processing helpers.";
        // pass
        pub fn _processoptions ( args )  {
        for arg in args .iter() {
        // try {
        _setoption ( arg );
        // } catch  _OptionError as msg  {
        println!( "Invalid -W option ignored:" , msg , file = sys . stderr );
        pub fn _setoption ( arg )  {
        parts = arg . split ( ":" );
        if len ( parts ) > 5 {
        panic!("_OptionError ( "too many fields (max 5): %r" % ( arg , ) )");
        while len ( parts ) < 5  {
        parts . append ( "" );
        action , message , category , module , lineno = [ s . strip ( );
        for s in parts ].iter() {
        action = _getaction ( action );
        category = _getcategory ( category );
        if message || module {
        import re;
        if message {
        message = re . escape ( message );
        if module {
        module = re . escape ( module ) + r "\Z";
        if lineno {
        // try {
        lineno = int ( lineno );
        if lineno < 0 {
        panic!("ValueError");
        // } catch  ( ValueError , OverflowError )  {
        panic!("_OptionError ( "invalid lineno %r" % ( lineno , ) ) from None /* Option */");
        } else {
        lineno = 0;
        filterwarnings ( action , message , category , module , lineno );
        pub fn _getaction ( action )  {
        if !action {
        return  "default";
        if action == "all" { : return "always"; }
        for a in ( "default" , "always" , "ignore" , "module" , "once" , "error" ) .iter() {
        if a . startswith ( action ) {
        return  a;
        panic!("_OptionError ( "invalid action: %r" % ( action , ) )");
        pub fn _getcategory ( category )  {
        if !category {
        return  Warning;
        if "." !in category {
        import builtins as m;
        klass = category;
        } else {
        module , _ , klass = category . rpartition ( "." );
        // try {
        m = __import__ ( module , None /* Option */ , None /* Option */ , [ klass ] );
        // } catch  ImportError  {
        panic!("_OptionError ( "invalid module name: %r" % ( module , ) ) from None /* Option */");
        // try {
        cat = getattr ( m , klass );
        // } catch  AttributeError  {
        panic!("_OptionError ( "unknown warning category: %r" % ( category , ) ) from None /* Option */");
        if !issubclass ( cat , Warning ) {
        panic!("_OptionError ( "invalid warning category: %r" % ( category , ) )");
        return  cat;
        pub fn _is_internal_frame ( frame )  {
        "Signal whether the frame == an internal CPython implementation detail.";
        filename = frame . f_code . co_filename;
        return  "importlib" in filename && "_bootstrap" in filename;
        pub fn _next_external_frame ( frame )  {
        "Find the next frame that doesn't involve CPython internals.";
        frame = frame . f_back;
        while frame is !None /* Option */ && _is_internal_frame ( frame )  {
        frame = frame . f_back;
        return  frame;
        pub fn warn ( message , category = None /* Option */ , stacklevel = 1 , source = None /* Option */ )  {
        "Issue a warning, || maybe ignore it || raise an exception.";
        if isinstance ( message , Warning ) {
        category = message . __class__;
        if category is None /* Option */ {
        category = UserWarning;
        if !( isinstance ( category , type ) && issubclass ( category , Warning ) ) {
        panic!("TypeError ( "category must be a Warning subclass, "");
        "not '{:s}'" . format ( type ( category ) . __name__ ) );
        // try {
        if stacklevel <= 1 || _is_internal_frame ( sys . _getframe ( 1 ) ) {
        frame = sys . _getframe ( stacklevel );
        } else {
        frame = sys . _getframe ( 1 );
        for x in range ( stacklevel -1 ) .iter() {
        frame = _next_external_frame ( frame );
        if frame is None /* Option */ {
        panic!("ValueError");
        // } catch  ValueError  {
        globals = sys . __dict__;
        filename = "sys";
        lineno = 1;
        } else {
        globals = frame . f_globals;
        filename = frame . f_code . co_filename;
        lineno = frame . f_lineno;
        if "__name__" in globals {
        module = globals [ "__name__" ];
        } else {
        module = "<string>";
        registry = globals . setdefault ( "__warningregistry__" , { } );
        warn_explicit ( message , category , filename , lineno , module , registry ,;
        globals , source );
        pub fn warn_explicit ( message , category , filename , lineno , {
        module = None /* Option */ , registry = None /* Option */ , module_globals = None /* Option */ ,;
        source = None /* Option */ ) ;
        lineno = int ( lineno );
        if module is None /* Option */ {
        module = filename || "<unknown>";
        if module [ -3 { : ] . lower ( ) == ".py" ; }
        module = module [ : -3 ];
        if registry is None /* Option */ {
        registry = { };
        if registry . get ( "version" , 0 ) != _filters_version {
        registry . clear ( );
        registry [ "version" ] = _filters_version;
        if isinstance ( message , Warning ) {
        text = str ( message );
        category = message . __class__;
        } else {
        text = message;
        message = category ( message );
        key = ( text , category , lineno );
        if registry . get ( key ) {
        return;
        for item in filters .iter() {
        action , msg , cat , mod , ln = item;
        if ( ( msg is None /* Option */ || msg . match ( text ) ) and {
        issubclass ( category , cat ) and;
        ( mod == None /* Option */ || mod . match ( module ) ) and;
        ( ln == 0 || lineno == ln ) ) ;
        break;
        } else {
        action = defaultaction;
        if action == "ignore" {
        return;
        import linecache;
        linecache . getlines ( filename , module_globals );
        if action == "error" {
        panic!("message");
        if action == "once" {
        registry [ key ] = 1;
        oncekey = ( text , category );
        if onceregistry . get ( oncekey ) {
        return;
        onceregistry [ oncekey ] = 1;
        } else if action == "always" {
        // pass
        } else if action == "module" {
        registry [ key ] = 1;
        altkey = ( text , category , 0 );
        if registry . get ( altkey ) {
        return;
        registry [ altkey ] = 1;
        } else if action == "default" {
        registry [ key ] = 1;
        } else {
        panic!("RuntimeError (");
        "Unrecognized action (%r) in warnings.filters:\n %s" %;
        ( action , item ) );
        msg = WarningMessage ( message , category , filename , lineno , source );
        _showwarnmsg ( msg );
        class WarningMessage ( object ) ;
        _WARNING_DETAILS = ( "message" , "category" , "filename" , "lineno" , "file" ,;
        "line" , "source" );
        pub fn __init__ ( &self, message , category , filename , lineno , file = None /* Option */ , {
        line = None /* Option */ , source = None /* Option */ ) ;
        self . message = message;
        self . category = category;
        self . filename = filename;
        self . lineno = lineno;
        self . file = file;
        self . line = line;
        self . source = source;
        self . _category_name = category . __name__ if category else None /* Option */;
        pub fn __str__ ( self )  {
        return  ( "{message : %r, category : %r, filename : %r, lineno : %s, ";
        "line : %r}" % ( self . message , self . _category_name ,;
        self . filename , self . lineno , self . line ) );
        class catch_warnings ( object ) ;
        "A context manager that copies && restores the warnings filter upon
    exiting the context.

    The 'record' argument specifies whether warnings should be captured by a
    custom implementation of warnings.showwarning() && be appended to a list
    returned by the context manager. Otherwise None /* Option */ == returned by the context
    manager. The objects appended to the list are arguments whose attributes
    mirror the arguments to showwarning().

    The 'module' argument == to specify an alternative module to the module
    named 'warnings' && imported under that name. This argument == only useful
    when testing the warnings module itself.

    If the 'action' argument == !None /* Option */, the remaining arguments are passed
    to warnings.simplefilter() as if it were called immediately on entering the
    context.
    ";
        pub fn __init__ ( &self, * , record = false , module = None /* Option */ , {
        action = None /* Option */ , category = Warning , lineno = 0 , append = false ) ;
        "Specify whether to record warnings && if an alternative module
        should be used other than sys.modules['warnings'].

        For compatibility with Python 3.0, please consider all arguments to be
        keyword-only.

        ";
        self . _record = record;
        self . _module = sys . modules [ "warnings" ] if module is None /* Option */ else module;
        self . _entered = false;
        if action is None /* Option */ {
        self . _filter = None /* Option */;
        } else {
        self . _filter = ( action , category , lineno , append );
        pub fn __repr__ ( self )  {
        args = [ ];
        if self . _record {
        args . append ( "record=true" );
        if self . _module is !sys . modules [ "warnings" ] {
        args . append ( "module=%r" % self . _module );
        name = type ( self ) . __name__;
        return  "%s(%s)" % ( name , ", " . join ( args ) );
        pub fn __enter__ ( self )  {
        if self . _entered {
        panic!("RuntimeError ( "Cannot enter %r twice" % self )");
        self . _entered = true;
        self . _filters = self . _module . filters;
        self . _module . filters = self . _filters [ : ];
        self . _module . _filters_mutated ( );
        self . _showwarning = self . _module . showwarning;
        self . _showwarnmsg_impl = self . _module . _showwarnmsg_impl;
        if self . _filter is !None /* Option */ {
        simplefilter ( * self . _filter );
        if self . _record {
        log = [ ];
        self . _module . _showwarnmsg_impl = log . append;
        self . _module . showwarning = self . _module . _showwarning_orig;
        return  log;
        } else {
        return;
        pub fn __exit__ ( &self, * exc_info )  {
        if !self . _entered {
        panic!("RuntimeError ( "Cannot exit %r without entering first" % self )");
        self . _module . filters = self . _filters;
        self . _module . _filters_mutated ( );
        self . _module . showwarning = self . _showwarning;
        self . _module . _showwarnmsg_impl = self . _showwarnmsg_impl;
        _DEPRECATED_MSG = "{name!r} == deprecated && slated for removal in Python {remove}";
        pub fn _deprecated ( name , message = _DEPRECATED_MSG , * , remove , _version = sys . version_info )  {
        "Warn that *name* == deprecated || should be removed.

    RuntimeError == raised if *remove* specifies a major/minor tuple older than
    the current Python version || the same version but past the alpha.

    The *message* argument == formatted with *name* && *remove* as a Python
    version (e.g. "3.11").

    ";
        remove_formatted = format!("{remove[0]}.{remove[1]}");
        if ( _version [ { : 2 ] > remove ) || ( _version [ : 2 ] == remove && _version [ 3 ] != "alpha" ) ; }
        msg = format!("{name!r} was slated for removal after Python {remove_formatted} alpha");
        panic!("RuntimeError ( msg )");
        } else {
        msg = message . format ( name = name , remove = remove_formatted );
        warn ( msg , DeprecationWarning , stacklevel = 3 );
        pub fn _warn_unawaited_coroutine ( coro )  {
        msg_lines = [;
        format!("coroutine '{coro.__qualname__}' was never awaited\n");
        ];
        if coro . cr_origin is !None /* Option */ {
        import linecache , traceback;
        pub fn extract ( )  {
        for filename , lineno , funcname in reversed ( coro . cr_origin ) .iter() {
        line = linecache . getline ( filename , lineno );
        yield ( filename , lineno , funcname , line );
        msg_lines . append ( "Coroutine created at (most recent call last)\n" );
        msg_lines + = traceback . format_list ( list ( extract ( ) ) );
        msg = "" . join ( msg_lines ) . rstrip ( "\n" );
        warn ( msg , category = RuntimeWarning , stacklevel = 2 , source = coro );
        // try {
        from _warnings import ( filters , _defaultaction , _onceregistry ,;
        warn , warn_explicit , _filters_mutated );
        defaultaction = _defaultaction;
        onceregistry = _onceregistry;
        _warnings_defaults = true;
        // } catch  ImportError  {
        filters = [ ];
        defaultaction = "default";
        onceregistry = { };
        _filters_version = 1;
        pub fn _filters_mutated ( )  {
        global _filters_version;
        _filters_version + = 1;
        _warnings_defaults = false;
        _processoptions ( sys . warnoptions );
        if !_warnings_defaults {
        if !hasattr ( sys , "gettotalrefcount" ) {
        filterwarnings ( "default" , category = DeprecationWarning ,;
        module = "__main__" , append = 1 );
        simplefilter ( "ignore" , category = DeprecationWarning , append = 1 );
        simplefilter ( "ignore" , category = PendingDeprecationWarning , append = 1 );
        simplefilter ( "ignore" , category = ImportWarning , append = 1 );
        simplefilter ( "ignore" , category = ResourceWarning , append = 1 );
        del _warnings_defaults;
}

