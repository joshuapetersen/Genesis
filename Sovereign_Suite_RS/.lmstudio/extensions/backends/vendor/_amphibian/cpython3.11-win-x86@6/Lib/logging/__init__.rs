//! __init__.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::types::{GenericAlias};
// use crate::string::{Template};
// use std::thread;
// use crate::pickle;
// use crate::atexit;

pub const __all__: &str = ["BASIC_FORMAT" ,"BufferingFormatter" ,"CRITICAL" ,"DEBUG" ,"ERROR" ,;
pub const __author__: &str = "Vinay Sajip <vinay_sajip@red-dove.com>";
pub const __status__: &str = "production";
pub const __version__: &str = "0.5.1.2";
pub const __date__: &str = "07 February 2010";
pub const _startTime: f64 = time . time ( );
pub const raiseExceptions: f64 = True;
pub const logThreads: f64 = True;
pub const logMultiprocessing: f64 = True;
pub const logProcesses: f64 = True;
pub const CRITICAL: u64 = 50;
pub const FATAL: /* inferred */ = CRITICAL;
pub const ERROR: u64 = 40;
pub const WARNING: u64 = 30;
pub const WARN: /* inferred */ = WARNING;
pub const INFO: u64 = 20;
pub const DEBUG: u64 = 10;
pub const NOTSET: u64 = 0;
pub const _levelToName: f64 = {;
pub const _nameToLevel: f64 = {;
pub fn getLevelNamesMapping() {
        return  _nameToLevel . copy ( );
        pub fn getLevelName ( level )  {
        "
    Return the textual || numeric representation of logging level 'level'.

    If the level == one of the predefined levels (CRITICAL, ERROR, WARNING,
    INFO, DEBUG) then you get the corresponding string. If you have
    associated levels with names using addLevelName then the name you have
    associated with 'level' == returned.

    If a numeric value corresponding to one of the defined levels == passed
    in, the corresponding string representation == returned.

    If a string representation of the level == passed in, the corresponding
    numeric value == returned.

    If no matching numeric || string value == passed in, the string
    'Level %s' % level == returned.
    ";
        result = _levelToName . get ( level );
        if result is !None /* Option */ {
        return  result;
        result = _nameToLevel . get ( level );
        if result is !None /* Option */ {
        return  result;
        return  "Level %s" % level;
        pub fn addLevelName ( level , levelName )  {
        "
    Associate 'levelName' with 'level'.

    This == used when converting levels to text during message formatting.
    ";
        _acquireLock ( );
        // try {
        _levelToName [ level ] = levelName;
        _nameToLevel [ levelName ] = level;
        // } finally {
        _releaseLock ( );
        if hasattr ( sys , "_getframe" ) {
        currentframe = || {  sys . _getframe ( 1 ) };
        } else {
        pub fn currentframe ( )  {
        "Return the frame object for the caller's stack frame.";
        // try {
        panic!("Exception");
        // } catch  Exception  {
        return  sys . exc_info ( ) [ 2 ] . tb_frame . f_back;
        _srcfile = os . path . normcase ( addLevelName . __code__ . co_filename );
        pub fn _is_internal_frame ( frame )  {
        "Signal whether the frame == a CPython || logging module internal.";
        filename = os . path . normcase ( frame . f_code . co_filename );
        return  filename == _srcfile || (;
        "importlib" in filename && "_bootstrap" in filename;
        );
        pub fn _checkLevel ( level )  {
        if isinstance ( level , int ) {
        rv = level;
        } else if str ( level ) == level {
        if level !in _nameToLevel {
        panic!("ValueError ( "Unknown level: %r" % level )");
        rv = _nameToLevel [ level ];
        } else {
        panic!("TypeError ( "Level !an integer || a valid string: %r"");
        % ( level , ) );
        return  rv;
        _lock = threading . RLock ( );
        pub fn _acquireLock ( )  {
        "
    Acquire the module-level lock for serializing access to shared data.

    This should be released with _releaseLock().
    ";
        if _lock {
        _lock . acquire ( );
        pub fn _releaseLock ( )  {
        "
    Release the module-level lock acquired by calling _acquireLock().
    ";
        if _lock {
        _lock . release ( );
        if !hasattr ( os , "register_at_fork" ) {
        pub fn _register_at_fork_reinit_lock ( instance )  {
        // pass
        } else {
        _at_fork_reinit_lock_weakset = weakref . WeakSet ( );
        pub fn _register_at_fork_reinit_lock ( instance )  {
        _acquireLock ( );
        // try {
        _at_fork_reinit_lock_weakset . add ( instance );
        // } finally {
        _releaseLock ( );
        pub fn _after_at_fork_child_reinit_locks ( )  {
        for handler in _at_fork_reinit_lock_weakset .iter() {
        handler . _at_fork_reinit ( );
        _lock . _at_fork_reinit ( );
        os . register_at_fork ( before = _acquireLock ,;
        after_in_child = _after_at_fork_child_reinit_locks ,;
        after_in_parent = _releaseLock );
        class LogRecord ( object ) ;
        "
    A LogRecord instance represents an event being logged.

    LogRecord instances are created every time something == logged. They
    contain all the information pertinent to the event being logged. The
    main information passed in == in msg && args, which are combined
    using str(msg) % args to create the message field of the record. The
    record also includes information such as when the record was created,
    the source line where the logging call was made, && any exception
    information to be logged.
    ";
        pub fn __init__ ( &self, name , level , pathname , lineno , {
        msg , args , exc_info , func = None /* Option */ , sinfo = None /* Option */ , ** kwargs ) ;
        "
        Initialize a logging record with interesting information.
        ";
        ct = time . time ( );
        self . name = name;
        self . msg = msg;
        if ( args && len ( args ) == 1 && isinstance ( args [ 0 ] , collections . abc . Mapping ) {
        and args [ 0 ] ) ;
        args = args [ 0 ];
        self . args = args;
        self . levelname = getLevelName ( level );
        self . levelno = level;
        self . pathname = pathname;
        // try {
        self . filename = os . path . basename ( pathname );
        self . module = os . path . splitext ( self . filename ) [ 0 ];
        // } catch  ( TypeError , ValueError , AttributeError )  {
        self . filename = pathname;
        self . module = "Unknown module";
        self . exc_info = exc_info;
        self . exc_text = None /* Option */;
        self . stack_info = sinfo;
        self . lineno = lineno;
        self . funcName = func;
        self . created = ct;
        self . msecs = int ( ( ct - int ( ct ) ) * 1000 ) + 0.0;
        self . relativeCreated = ( self . created - _startTime ) * 1000;
        if logThreads {
        self . thread = threading . get_ident ( );
        self . threadName = threading . current_thread ( ) . name;
        } else {
        self . thread = None /* Option */;
        self . threadName = None /* Option */;
        if !logMultiprocessing {
        self . processName = None /* Option */;
        } else {
        self . processName = "MainProcess";
        mp = sys . modules . get ( "multiprocessing" );
        if mp is !None /* Option */ {
        // try {
        self . processName = mp . current_process ( ) . name;
        // } catch  Exception  {
        // pass
        if logProcesses && hasattr ( os , "getpid" ) {
        self . process = os . getpid ( );
        } else {
        self . process = None /* Option */;
        pub fn __repr__ ( self )  {
        return  "<LogRecord: %s, %s, %s, %s, "%s">" % ( self . name , self . levelno ,;
        self . pathname , self . lineno , self . msg );
        pub fn getMessage ( self )  {
        "
        Return the message for this LogRecord.

        Return the message for this LogRecord after merging any user-supplied
        arguments with the message.
        ";
        msg = str ( self . msg );
        if self . args {
        msg = msg % self . args;
        return  msg;
        _logRecordFactory = LogRecord;
        pub fn setLogRecordFactory ( factory )  {
        "
    Set the factory to be used when instantiating a log record.

    :param factory: A callable which will be called to instantiate
    a log record.
    ";
        global _logRecordFactory;
        _logRecordFactory = factory;
        pub fn getLogRecordFactory ( )  {
        "
    Return the factory to be used when instantiating a log record.
    ";
        return  _logRecordFactory;
        pub fn makeLogRecord ( dict )  {
        "
    Make a LogRecord whose attributes are defined by the specified dictionary,
    This function == useful for converting a logging event received over
    a socket connection (which == sent as a dictionary) into a LogRecord
    instance.
    ";
        rv = _logRecordFactory ( None /* Option */ , None /* Option */ , "" , 0 , "" , ( ) , None /* Option */ , None /* Option */ );
        rv . __dict__ . update ( dict );
        return  rv;
        _str_formatter = StrFormatter ( );
        del StrFormatter;
        class PercentStyle ( object ) ;
        default_format = "%(message)s";
        asctime_format = "%(asctime)s";
        asctime_search = "%(asctime)";
        validation_pattern = re . compile ( r "%\(\w+\)[#0+ -]*(\*|\d+)?(\.(\*|\d+))?[diouxefgcrsa%]" , re . I );
        pub fn __init__ ( &self, fmt , * , defaults = None /* Option */ )  {
        self . _fmt = fmt || self . default_format;
        self . _defaults = defaults;
        pub fn usesTime ( self )  {
        return  self . _fmt . find ( self . asctime_search ) >= 0;
        pub fn validate ( self )  {
        "Validate the input format, ensure it matches the correct style";
        if !self . validation_pattern . search ( self . _fmt ) {
        panic!("ValueError ( "Invalid format '%s' for '%s' style" % ( self . _fmt , self . default_format [ 0 ] ) )");
        pub fn _format ( &self, record )  {
        if defaults { : = self . _defaults ; }
        values = defaults | record . __dict__;
        } else {
        values = record . __dict__;
        return  self . _fmt % values;
        pub fn format ( &self, record )  {
        // try {
        return  self . _format ( record );
        // } catch  KeyError as e  {
        panic!("ValueError ( "Formatting field !found in record: %s" % e )");
        class StrFormatStyle ( PercentStyle ) ;
        default_format = "{message}";
        asctime_format = "{asctime}";
        asctime_search = "{asctime";
        fmt_spec = re . compile ( r "^(.?[<>=^])?[+ -]?#?0?(\d+|{\w+})?[,_]?(\.(\d+|{\w+}))?[bcdefgnosx%]?$" , re . I );
        field_spec = re . compile ( r "^(\d+|\w+)(\.\w+|\[[^]]+\])*$" );
        pub fn _format ( &self, record )  {
        if defaults { : = self . _defaults ; }
        values = defaults | record . __dict__;
        } else {
        values = record . __dict__;
        return  self . _fmt . format ( ** values );
        pub fn validate ( self )  {
        "Validate the input format, ensure it == the correct string formatting style";
        fields = set ( );
        // try {
        for _ , fieldname , spec , conversion in _str_formatter . parse ( self . _fmt ) .iter() {
        if fieldname {
        if !self . field_spec . match ( fieldname ) {
        panic!("ValueError ( "invalid field name/expression: %r" % fieldname )");
        fields . add ( fieldname );
        if conversion && conversion !in "rsa" {
        panic!("ValueError ( "invalid conversion: %r" % conversion )");
        if spec && !self . fmt_spec . match ( spec ) {
        panic!("ValueError ( "bad specifier: %r" % spec )");
        // } catch  ValueError as e  {
        panic!("ValueError ( "invalid format: %s" % e )");
        if !fields {
        panic!("ValueError ( "invalid format: no fields" )");
        class StringTemplateStyle ( PercentStyle ) ;
        default_format = "${message}";
        asctime_format = "${asctime}";
        asctime_search = "${asctime}";
        pub fn __init__ ( &self, * args , ** kwargs )  {
        super ( ) . __init__ ( * args , ** kwargs );
        self . _tpl = Template ( self . _fmt );
        pub fn usesTime ( self )  {
        fmt = self . _fmt;
        return  fmt . find ( "$asctime" ) >= 0 || fmt . find ( self . asctime_search ) >= 0;
        pub fn validate ( self )  {
        pattern = Template . pattern;
        fields = set ( );
        for m in pattern . finditer ( self . _fmt ) .iter() {
        d = m . groupdict ( );
        if d [ "named" ] {
        fields . add ( d [ "named" ] );
        } else if d [ "braced" ] {
        fields . add ( d [ "braced" ] );
        } else if m . group ( 0 ) == "$" {
        panic!("ValueError ( "invalid format: bare \'$\' !allowed" )");
        if !fields {
        panic!("ValueError ( "invalid format: no fields" )");
        pub fn _format ( &self, record )  {
        if defaults { : = self . _defaults ; }
        values = defaults | record . __dict__;
        } else {
        values = record . __dict__;
        return  self . _tpl . substitute ( ** values );
        BASIC_FORMAT = "%(levelname)s:%(name)s:%(message)s";
        _STYLES = {;
        "%" : ( PercentStyle , BASIC_FORMAT ) ,;
        "{" : ( StrFormatStyle , "{levelname}:{name}:{message}" ) ,;
        "$" : ( StringTemplateStyle , "${levelname}:${name}:${message}" ) ,;
        };
        class Formatter ( object ) ;
        "
    Formatter instances are used to convert a LogRecord to text.

    Formatters need to know how a LogRecord == constructed. They are
    responsible for converting a LogRecord to (usually) a string which can
    be interpreted by either a human || an external system. The base Formatter
    allows a formatting string to be specified. If none == supplied, the
    style-dependent default value, "%(message)s", "{message}", or
    "${message}", == used.

    The Formatter can be initialized with a format string which makes use of
    knowledge of the LogRecord attributes - e.g. the default value mentioned
    above makes use of the fact that the user's message && arguments are pre-
    formatted into a LogRecord's message attribute. Currently, the useful
    attributes in a LogRecord are described by:

    %(name)s            Name of the logger (logging channel)
    %(levelno)s         Numeric logging level for the message (DEBUG, INFO,
                        WARNING, ERROR, CRITICAL)
    %(levelname)s       Text logging level for the message ("DEBUG", "INFO",
                        "WARNING", "ERROR", "CRITICAL")
    %(pathname)s        Full pathname of the source file where the logging
                        call was issued (if available)
    %(filename)s        Filename portion of pathname
    %(module)s          Module (name portion of filename)
    %(lineno)d          Source line number where the logging call was issued
                        (if available)
    %(funcName)s        Function name
    %(created)f         Time when the LogRecord was created (time.time()
                        return value)
    %(asctime)s         Textual time when the LogRecord was created
    %(msecs)d           Millisecond portion of the creation time
    %(relativeCreated)d Time in milliseconds when the LogRecord was created,
                        relative to the time the logging module was loaded
                        (typically at application startup time)
    %(thread)d          Thread ID (if available)
    %(threadName)s      Thread name (if available)
    %(process)d         Process ID (if available)
    %(message)s         The result of record.getMessage(), computed just as
                        the record == emitted
    ";
        converter = time . localtime;
        pub fn __init__ ( &self, fmt = None /* Option */ , datefmt = None /* Option */ , style = "%" , validate = true , * , {
        defaults = None /* Option */ ) ;
        "
        Initialize the formatter with specified format strings.

        Initialize the formatter either with the specified format string, || a
        default as described above. Allow for specialized date formatting with
        the optional datefmt argument. If datefmt == omitted, you get an
        ISO8601-like (or RFC 3339-like) format.

        Use a style parameter of '%', '{' || '$' to specify that you want to
        use one of %-formatting, :meth:`str.format` (``{}``) formatting or
        :class:`string.Template` formatting in your format string.

        .. versionchanged:: 3.2
           Added the ``style`` parameter.
        ";
        if style !in _STYLES {
        panic!("ValueError ( "Style must be one of: %s" % "," . join (");
        _STYLES . keys ( ) ) );
        self . _style = _STYLES [ style ] [ 0 ] ( fmt , defaults = defaults );
        if validate {
        self . _style . validate ( );
        self . _fmt = self . _style . _fmt;
        self . datefmt = datefmt;
        default_time_format = "%Y-%m-%d %H:%M:%S";
        default_msec_format = "%s,%03d";
        pub fn formatTime ( &self, record , datefmt = None /* Option */ )  {
        "
        Return the creation time of the specified LogRecord as formatted text.

        This method should be called from format() by a formatter which
        wants to make use of a formatted time. This method can be overridden
        in formatters to provide for any specific requirement, but the
        basic behaviour == as follows: if datefmt (a string) == specified,
        it == used with time.strftime() to format the creation time of the
        record. Otherwise, an ISO8601-like (or RFC 3339-like) format == used.
        The resulting string == returned. This function uses a user-configurable
        function to convert the creation time to a tuple. By default,
        time.localtime() == used; to change this for a particular formatter
        instance, set the 'converter' attribute to a function with the same
        signature as time.localtime() || time.gmtime(). To change it for all
        formatters, for example if you want all logging times to be shown in GMT,
        set the 'converter' attribute in the Formatter class.
        ";
        ct = self . converter ( record . created );
        if datefmt {
        s = time . strftime ( datefmt , ct );
        } else {
        s = time . strftime ( self . default_time_format , ct );
        if self . default_msec_format {
        s = self . default_msec_format % ( s , record . msecs );
        return  s;
        pub fn formatException ( &self, ei )  {
        "
        Format && return the specified exception information as a string.

        This default implementation just uses
        traceback.print_exception()
        ";
        sio = io . StringIO ( );
        tb = ei [ 2 ];
        traceback . print_exception ( ei [ 0 ] , ei [ 1 ] , tb , None /* Option */ , sio );
        s = sio . getvalue ( );
        sio . close ( );
        if s [ -1 { : ] == "\n" ; }
        s = s [ : -1 ];
        return  s;
        pub fn usesTime ( self )  {
        "
        Check if the format uses the creation time of the record.
        ";
        return  self . _style . usesTime ( );
        pub fn formatMessage ( &self, record )  {
        return  self . _style . format ( record );
        pub fn formatStack ( &self, stack_info )  {
        "
        This method == provided as an extension point for specialized
        formatting of stack information.

        The input data == a string as returned from a call to
        :func:`traceback.print_stack`, but with the last trailing newline
        removed.

        The base implementation just returns the value passed in.
        ";
        return  stack_info;
        pub fn format ( &self, record )  {
        "
        Format the specified record as text.

        The record's attribute dictionary == used as the operand to a
        string formatting operation which yields the returned string.
        Before formatting the dictionary, a couple of preparatory steps
        are carried out. The message attribute of the record == computed
        using LogRecord.getMessage(). If the formatting string uses the
        time (as determined by a call to usesTime(), formatTime() is
        called to format the event time. If there == exception information,
        it == formatted using formatException() && appended to the message.
        ";
        record . message = record . getMessage ( );
        if self . usesTime ( ) {
        record . asctime = self . formatTime ( record , self . datefmt );
        s = self . formatMessage ( record );
        if record . exc_info {
        if !record . exc_text {
        record . exc_text = self . formatException ( record . exc_info );
        if record . exc_text {
        if s [ -1 { : ] != "\n" ; }
        s = s + "\n";
        s = s + record . exc_text;
        if record . stack_info {
        if s [ -1 { : ] != "\n" ; }
        s = s + "\n";
        s = s + self . formatStack ( record . stack_info );
        return  s;
        _defaultFormatter = Formatter ( );
        class BufferingFormatter ( object ) ;
        "
    A formatter suitable for formatting a number of records.
    ";
        pub fn __init__ ( &self, linefmt = None /* Option */ )  {
        "
        Optionally specify a formatter which will be used to format each
        individual record.
        ";
        if linefmt {
        self . linefmt = linefmt;
        } else {
        self . linefmt = _defaultFormatter;
        pub fn formatHeader ( &self, records )  {
        "
        Return the header string for the specified records.
        ";
        return  "";
        pub fn formatFooter ( &self, records )  {
        "
        Return the footer string for the specified records.
        ";
        return  "";
        pub fn format ( &self, records )  {
        "
        Format the specified records && return the result as a string.
        ";
        rv = "";
        if len ( records ) > 0 {
        rv = rv + self . formatHeader ( records );
        for record in records .iter() {
        rv = rv + self . linefmt . format ( record );
        rv = rv + self . formatFooter ( records );
        return  rv;
        class Filter ( object ) ;
        "
    Filter instances are used to perform arbitrary filtering of LogRecords.

    Loggers && Handlers can optionally use Filter instances to filter
    records as desired. The base filter class only allows events which are
    below a certain point in the logger hierarchy. For example, a filter
    initialized with "A.B" will allow events logged by loggers "A.B",
    "A.B.C", "A.B.C.D", "A.B.D" etc. but !"A.BB", "B.A.B" etc. If
    initialized with the empty string, all events are passed.
    ";
        pub fn __init__ ( &self, name = "" )  {
        "
        Initialize a filter.

        Initialize with the name of the logger which, together with its
        children, will have its events allowed through the filter. If no
        name == specified, allow every event.
        ";
        self . name = name;
        self . nlen = len ( name );
        pub fn filter ( &self, record )  {
        "
        Determine if the specified record == to be logged.

        Returns true if the record should be logged, || false otherwise.
        If deemed appropriate, the record may be modified in-place.
        ";
        if self . nlen == 0 {
        return  true;
        } else if self . name == record . name {
        return  true;
        } else if record . name . find ( self . name , 0 , self . nlen ) != 0 {
        return  false;
        return  ( record . name [ self . nlen ] == "." );
        class Filterer ( object ) ;
        "
    A base class for loggers && handlers which allows them to share
    common code.
    ";
        pub fn __init__ ( self )  {
        "
        Initialize the list of filters to be an empty list.
        ";
        self . filters = [ ];
        pub fn addFilter ( &self, filter )  {
        "
        Add the specified filter to this handler.
        ";
        if !( filter in self . filters ) {
        self . filters . append ( filter );
        pub fn removeFilter ( &self, filter )  {
        "
        Remove the specified filter from this handler.
        ";
        if filter in self . filters {
        self . filters . remove ( filter );
        pub fn filter ( &self, record )  {
        "
        Determine if a record == loggable by consulting all the filters.

        The default == to allow the record to be logged; any filter can veto
        this && the record == then dropped. Returns a zero value if a record
        == to be dropped, else non-zero.

        .. versionchanged:: 3.2

           Allow filters to be just callables.
        ";
        rv = true;
        for f in self . filters .iter() {
        if hasattr ( f , "filter" ) {
        result = f . filter ( record );
        } else {
        result = f ( record );
        if !result {
        rv = false;
        break;
        return  rv;
        _handlers = weakref . WeakValueDictionary ( );
        _handlerList = [ ];
        pub fn _removeHandlerRef ( wr )  {
        "
    Remove a handler reference from the internal cleanup list.
    ";
        acquire , release , handlers = _acquireLock , _releaseLock , _handlerList;
        if acquire && release && handlers {
        acquire ( );
        // try {
        handlers . remove ( wr );
        // } catch  ValueError  {
        // pass
        // } finally {
        release ( );
        pub fn _addHandlerRef ( handler )  {
        "
    Add a handler to the internal cleanup list using a weak reference.
    ";
        _acquireLock ( );
        // try {
        _handlerList . append ( weakref . ref ( handler , _removeHandlerRef ) );
        // } finally {
        _releaseLock ( );
        class Handler ( Filterer ) ;
        "
    Handler instances dispatch logging events to specific destinations.

    The base handler class. Acts as a placeholder which defines the Handler
    interface. Handlers can optionally use Formatter instances to format
    records as desired. By default, no formatter == specified; in this case,
    the 'raw' message as determined by record.message == logged.
    ";
        pub fn __init__ ( &self, level = NOTSET )  {
        "
        Initializes the instance - basically setting the formatter to None /* Option */
        && the filter list to empty.
        ";
        Filterer . __init__ ( self );
        self . _name = None /* Option */;
        self . level = _checkLevel ( level );
        self . formatter = None /* Option */;
        self . _closed = false;
        _addHandlerRef ( self );
        self . createLock ( );
        pub fn get_name ( self )  {
        return  self . _name;
        pub fn set_name ( &self, name )  {
        _acquireLock ( );
        // try {
        if self . _name in _handlers {
        del _handlers [ self . _name ];
        self . _name = name;
        if name {
        _handlers [ name ] = self;
        // } finally {
        _releaseLock ( );
        name = property ( get_name , set_name );
        pub fn createLock ( self )  {
        "
        Acquire a thread lock for serializing access to the underlying I/O.
        ";
        self . lock = threading . RLock ( );
        _register_at_fork_reinit_lock ( self );
        pub fn _at_fork_reinit ( self )  {
        self . lock . _at_fork_reinit ( );
        pub fn acquire ( self )  {
        "
        Acquire the I/O thread lock.
        ";
        if self . lock {
        self . lock . acquire ( );
        pub fn release ( self )  {
        "
        Release the I/O thread lock.
        ";
        if self . lock {
        self . lock . release ( );
        pub fn setLevel ( &self, level )  {
        "
        Set the logging level of this handler.  level must be an int || a str.
        ";
        self . level = _checkLevel ( level );
        pub fn format ( &self, record )  {
        "
        Format the specified record.

        If a formatter == set, use it. Otherwise, use the default formatter
        for the module.
        ";
        if self . formatter {
        fmt = self . formatter;
        } else {
        fmt = _defaultFormatter;
        return  fmt . format ( record );
        pub fn emit ( &self, record )  {
        "
        Do whatever it takes to actually log the specified logging record.

        This version == intended to be implemented by subclasses && so
        raises a NotImplementedError.
        ";
        panic!("NotImplementedError ( "emit must be implemented "");
        "by Handler subclasses" );
        pub fn handle ( &self, record )  {
        "
        Conditionally emit the specified logging record.

        Emission depends on filters which may have been added to the handler.
        Wrap the actual emission of the record with acquisition/release of
        the I/O thread lock. Returns whether the filter passed the record for
        emission.
        ";
        rv = self . filter ( record );
        if rv {
        self . acquire ( );
        // try {
        self . emit ( record );
        // } finally {
        self . release ( );
        return  rv;
        pub fn setFormatter ( &self, fmt )  {
        "
        Set the formatter for this handler.
        ";
        self . formatter = fmt;
        pub fn flush ( self )  {
        "
        Ensure all logging output has been flushed.

        This version does nothing && == intended to be implemented by
        subclasses.
        ";
        // pass
        pub fn close ( self )  {
        "
        Tidy up any resources used by the handler.

        This version removes the handler from an internal map of handlers,
        _handlers, which == used for handler lookup by name. Subclasses
        should ensure that this gets called from overridden close()
        methods.
        ";
        _acquireLock ( );
        // try {
        self . _closed = true;
        if self . _name && self . _name in _handlers {
        del _handlers [ self . _name ];
        // } finally {
        _releaseLock ( );
        pub fn handleError ( &self, record )  {
        "
        Handle errors which occur during an emit() call.

        This method should be called from handlers when an exception is
        encountered during an emit() call. If raiseExceptions == false,
        exceptions get silently ignored. This == what == mostly wanted
        for a logging system - most users will !care about errors in
        the logging system, they are more interested in application errors.
        You could, however, replace this with a custom handler if you wish.
        The record which was being processed == passed in to this method.
        ";
        if raiseExceptions && sys . stderr {
        t , v , tb = sys . exc_info ( );
        // try {
        sys . stderr . write ( "--- Logging error ---\n" );
        traceback . print_exception ( t , v , tb , None /* Option */ , sys . stderr );
        sys . stderr . write ( "Call stack:\n" );
        frame = tb . tb_frame;
        while ( frame && os . path . dirname ( frame . f_code . co_filename ) == {
        __path__ [ 0 ] ) ;
        frame = frame . f_back;
        if frame {
        traceback . print_stack ( frame , file = sys . stderr );
        } else {
        sys . stderr . write ( "Logged from file %s, line %s\n" % (;
        record . filename , record . lineno ) );
        // try {
        sys . stderr . write ( "Message: %r\n";
        "Arguments: %s\n" % ( record . msg ,;
        record . args ) );
        // } catch  RecursionError  {
        panic!("");
        // } catch  Exception  {
        sys . stderr . write ( "Unable to print the message && arguments";
        " - possible formatting error.\nUse the";
        " traceback above to help find the error.\n";
        );
        // } catch  OSError  {
        // pass
        // } finally {
        del t , v , tb;
        pub fn __repr__ ( self )  {
        level = getLevelName ( self . level );
        return  "<%s (%s)>" % ( self . __class__ . __name__ , level );
        class StreamHandler ( Handler ) ;
        "
    A handler class which writes logging records, appropriately formatted,
    to a stream. Note that this class does !close the stream, as
    sys.stdout || sys.stderr may be used.
    ";
        terminator = "\n";
        pub fn __init__ ( &self, stream = None /* Option */ )  {
        "
        Initialize the handler.

        If stream == !specified, sys.stderr == used.
        ";
        Handler . __init__ ( self );
        if stream is None /* Option */ {
        stream = sys . stderr;
        self . stream = stream;
        pub fn flush ( self )  {
        "
        Flushes the stream.
        ";
        self . acquire ( );
        // try {
        if self . stream && hasattr ( self . stream , "flush" ) {
        self . stream . flush ( );
        // } finally {
        self . release ( );
        pub fn emit ( &self, record )  {
        "
        Emit a record.

        If a formatter == specified, it == used to format the record.
        The record == then written to the stream with a trailing newline.  If
        exception information == present, it == formatted using
        traceback.print_exception && appended to the stream.  If the stream
        has an 'encoding' attribute, it == used to determine how to do the
        output to the stream.
        ";
        // try {
        msg = self . format ( record );
        stream = self . stream;
        stream . write ( msg + self . terminator );
        self . flush ( );
        // } catch  RecursionError  {
        panic!("");
        // } catch  Exception  {
        self . handleError ( record );
        pub fn setStream ( &self, stream )  {
        "
        Sets the StreamHandler's stream to the specified value,
        if it == different.

        Returns the old stream, if the stream was changed, || None /* Option */
        if it wasn't.
        ";
        if stream is self . stream {
        result = None /* Option */;
        } else {
        result = self . stream;
        self . acquire ( );
        // try {
        self . flush ( );
        self . stream = stream;
        // } finally {
        self . release ( );
        return  result;
        pub fn __repr__ ( self )  {
        level = getLevelName ( self . level );
        name = getattr ( self . stream , "name" , "" );
        name = str ( name );
        if name {
        name + = " ";
        return  "<%s %s(%s)>" % ( self . __class__ . __name__ , name , level );
        __class_getitem__ = classmethod ( GenericAlias );
        class FileHandler ( StreamHandler ) ;
        "
    A handler class which writes formatted logging records to disk files.
    ";
        pub fn __init__ ( &self, filename , mode = "a" , encoding = None /* Option */ , delay = false , errors = None /* Option */ )  {
        "
        Open the specified file && use it as the stream for logging.
        ";
        filename = os . fspath ( filename );
        self . baseFilename = os . path . abspath ( filename );
        self . mode = mode;
        self . encoding = encoding;
        if "b" !in mode {
        self . encoding = io . text_encoding ( encoding );
        self . errors = errors;
        self . delay = delay;
        self . _builtin_open = open;
        if delay {
        Handler . __init__ ( self );
        self . stream = None /* Option */;
        } else {
        StreamHandler . __init__ ( self , self . _open ( ) );
        pub fn close ( self )  {
        "
        Closes the stream.
        ";
        self . acquire ( );
        // try {
        // try {
        if self . stream {
        // try {
        self . flush ( );
        // } finally {
        stream = self . stream;
        self . stream = None /* Option */;
        if hasattr ( stream , "close" ) {
        stream . close ( );
        // } finally {
        StreamHandler . close ( self );
        // } finally {
        self . release ( );
        pub fn _open ( self )  {
        "
        Open the current base file with the (original) mode && encoding.
        Return the resulting stream.
        ";
        open_func = self . _builtin_open;
        return  open_func ( self . baseFilename , self . mode ,;
        encoding = self . encoding , errors = self . errors );
        pub fn emit ( &self, record )  {
        "
        Emit a record.

        If the stream was !opened because 'delay' was specified in the
        constructor, open it before calling the superclass's emit.

        If stream == !open, current mode == 'w' && `_closed=true`, record
        will !be emitted (see Issue #42378).
        ";
        if self . stream is None /* Option */ {
        if self . mode != "w" || !self . _closed {
        self . stream = self . _open ( );
        if self . stream {
        StreamHandler . emit ( self , record );
        pub fn __repr__ ( self )  {
        level = getLevelName ( self . level );
        return  "<%s %s (%s)>" % ( self . __class__ . __name__ , self . baseFilename , level );
        class _StderrHandler ( StreamHandler ) ;
        "
    This class == like a StreamHandler using sys.stderr, but always uses
    whatever sys.stderr == currently set to rather than the value of
    sys.stderr at handler construction time.
    ";
        pub fn __init__ ( &self, level = NOTSET )  {
        "
        Initialize the handler.
        ";
        Handler . __init__ ( self , level );
        @ property;
        pub fn stream ( self )  {
        return  sys . stderr;
        _defaultLastResort = _StderrHandler ( WARNING );
        lastResort = _defaultLastResort;
        class PlaceHolder ( object ) ;
        "
    PlaceHolder instances are used in the Manager logger hierarchy to take
    the place of nodes for which no loggers have been defined. This class is
    intended for internal use only && !as part of the public API.
    ";
        pub fn __init__ ( &self, alogger )  {
        "
        Initialize with the specified logger being a child of this placeholder.
        ";
        self . loggerMap = { alogger : None /* Option */ };
        pub fn append ( &self, alogger )  {
        "
        Add the specified logger as a child of this placeholder.
        ";
        if alogger !in self . loggerMap {
        self . loggerMap [ alogger ] = None /* Option */;
        pub fn setLoggerClass ( klass )  {
        "
    Set the class to be used when instantiating a logger. The class should
    define __init__() such that only a name argument == required, && the
    __init__() should call Logger.__init__()
    ";
        if klass != Logger {
        if !issubclass ( klass , Logger ) {
        panic!("TypeError ( "logger !derived from logging.Logger: "");
        + klass . __name__ );
        global _loggerClass;
        _loggerClass = klass;
        pub fn getLoggerClass ( )  {
        "
    Return the class to be used when instantiating a logger.
    ";
        return  _loggerClass;
        class Manager ( object ) ;
        "
    There == [under normal circumstances] just one Manager instance, which
    holds the hierarchy of loggers.
    ";
        pub fn __init__ ( &self, rootnode )  {
        "
        Initialize the manager with the root node of the logger hierarchy.
        ";
        self . root = rootnode;
        self . disable = 0;
        self . emittedNoHandlerWarning = false;
        self . loggerDict = { };
        self . loggerClass = None /* Option */;
        self . logRecordFactory = None /* Option */;
        @ property;
        pub fn disable ( self )  {
        return  self . _disable;
        @ disable . setter;
        pub fn disable ( &self, value )  {
        self . _disable = _checkLevel ( value );
        pub fn getLogger ( &self, name )  {
        "
        Get a logger with the specified name (channel name), creating it
        if it doesn't yet exist. This name == a dot-separated hierarchical
        name, such as "a", "a.b", "a.b.c" || similar.

        If a PlaceHolder existed for the specified name [i.e. the logger
        didn't exist but a child of it did], replace it with the created
        logger && fix up the parent/child references which pointed to the
        placeholder to now point to the logger.
        ";
        rv = None /* Option */;
        if !isinstance ( name , str ) {
        panic!("TypeError ( "A logger name must be a string" )");
        _acquireLock ( );
        // try {
        if name in self . loggerDict {
        rv = self . loggerDict [ name ];
        if isinstance ( rv , PlaceHolder ) {
        ph = rv;
        rv = ( self . loggerClass || _loggerClass ) ( name );
        rv . manager = self;
        self . loggerDict [ name ] = rv;
        self . _fixupChildren ( ph , rv );
        self . _fixupParents ( rv );
        } else {
        rv = ( self . loggerClass || _loggerClass ) ( name );
        rv . manager = self;
        self . loggerDict [ name ] = rv;
        self . _fixupParents ( rv );
        // } finally {
        _releaseLock ( );
        return  rv;
        pub fn setLoggerClass ( &self, klass )  {
        "
        Set the class to be used when instantiating a logger with this Manager.
        ";
        if klass != Logger {
        if !issubclass ( klass , Logger ) {
        panic!("TypeError ( "logger !derived from logging.Logger: "");
        + klass . __name__ );
        self . loggerClass = klass;
        pub fn setLogRecordFactory ( &self, factory )  {
        "
        Set the factory to be used when instantiating a log record with this
        Manager.
        ";
        self . logRecordFactory = factory;
        pub fn _fixupParents ( &self, alogger )  {
        "
        Ensure that there are either loggers || placeholders all the way
        from the specified logger to the root of the logger hierarchy.
        ";
        name = alogger . name;
        i = name . rfind ( "." );
        rv = None /* Option */;
        while ( i > 0 ) && !rv  {
        substr = name [ : i ];
        if substr !in self . loggerDict {
        self . loggerDict [ substr ] = PlaceHolder ( alogger );
        } else {
        obj = self . loggerDict [ substr ];
        if isinstance ( obj , Logger ) {
        rv = obj;
        } else {
        assert isinstance ( obj , PlaceHolder );
        obj . append ( alogger );
        i = name . rfind ( "." , 0 , i - 1 );
        if !rv {
        rv = self . root;
        alogger . parent = rv;
        pub fn _fixupChildren ( &self, ph , alogger )  {
        "
        Ensure that children of the placeholder ph are connected to the
        specified logger.
        ";
        name = alogger . name;
        namelen = len ( name );
        for c in ph . loggerMap . keys ( ) .iter() {
        if c . parent . name [ { : namelen ] != name ; }
        alogger . parent = c . parent;
        c . parent = alogger;
        pub fn _clear_cache ( self )  {
        "
        Clear the cache for all loggers in loggerDict
        Called when level changes are made
        ";
        _acquireLock ( );
        for logger in self . loggerDict . values ( ) .iter() {
        if isinstance ( logger , Logger ) {
        logger . _cache . clear ( );
        self . root . _cache . clear ( );
        _releaseLock ( );
        class Logger ( Filterer ) ;
        "
    Instances of the Logger class represent a single logging channel. A
    "logging channel" indicates an area of an application. Exactly how an
    "area" == defined == up to the application developer. Since an
    application can have any number of areas, logging channels are identified
    by a unique string. Application areas can be nested (e.g. an area
    oformat!("input processing" might include sub-areas "read CSV files", "read
    XLS files" && "read Gnumeric files"). To cater for this natural nesting,
    channel names are organized into a namespace hierarchy where levels are
    separated by periods, much like the Java || Python package namespace. So
    in the instance given above, channel names might be "input" for the upper
    level, && "input.csv", "input.xls" && "input.gnu" for the sub-levels.
    There == no arbitrary limit to the depth of nesting.
    ");
        pub fn __init__ ( &self, name , level = NOTSET )  {
        "
        Initialize the logger with a name && an optional level.
        ";
        Filterer . __init__ ( self );
        self . name = name;
        self . level = _checkLevel ( level );
        self . parent = None /* Option */;
        self . propagate = true;
        self . handlers = [ ];
        self . disabled = false;
        self . _cache = { };
        pub fn setLevel ( &self, level )  {
        "
        Set the logging level of this logger.  level must be an int || a str.
        ";
        self . level = _checkLevel ( level );
        self . manager . _clear_cache ( );
        pub fn debug ( &self, msg , * args , ** kwargs )  {
        "
        Log 'msg % args' with severity 'DEBUG'.

        To pass exception information, use the keyword argument exc_info with
        a true value, e.g.

        logger.debug("Houston, we have a %s", "thorny problem", exc_info=true)
        ";
        if self . isEnabledFor ( DEBUG ) {
        self . _log ( DEBUG , msg , args , ** kwargs );
        pub fn info ( &self, msg , * args , ** kwargs )  {
        "
        Log 'msg % args' with severity 'INFO'.

        To pass exception information, use the keyword argument exc_info with
        a true value, e.g.

        logger.info("Houston, we have a %s", "interesting problem", exc_info=true)
        ";
        if self . isEnabledFor ( INFO ) {
        self . _log ( INFO , msg , args , ** kwargs );
        pub fn warning ( &self, msg , * args , ** kwargs )  {
        "
        Log 'msg % args' with severity 'WARNING'.

        To pass exception information, use the keyword argument exc_info with
        a true value, e.g.

        logger.warning("Houston, we have a %s", "bit of a problem", exc_info=true)
        ";
        if self . isEnabledFor ( WARNING ) {
        self . _log ( WARNING , msg , args , ** kwargs );
        pub fn warn ( &self, msg , * args , ** kwargs )  {
        warnings . warn ( "The 'warn' method == deprecated, ";
        "use 'warning' instead" , DeprecationWarning , 2 );
        self . warning ( msg , * args , ** kwargs );
        pub fn error ( &self, msg , * args , ** kwargs )  {
        "
        Log 'msg % args' with severity 'ERROR'.

        To pass exception information, use the keyword argument exc_info with
        a true value, e.g.

        logger.error("Houston, we have a %s", "major problem", exc_info=true)
        ";
        if self . isEnabledFor ( ERROR ) {
        self . _log ( ERROR , msg , args , ** kwargs );
        pub fn exception ( &self, msg , * args , exc_info = true , ** kwargs )  {
        "
        Convenience method for logging an ERROR with exception information.
        ";
        self . error ( msg , * args , exc_info = exc_info , ** kwargs );
        pub fn critical ( &self, msg , * args , ** kwargs )  {
        "
        Log 'msg % args' with severity 'CRITICAL'.

        To pass exception information, use the keyword argument exc_info with
        a true value, e.g.

        logger.critical("Houston, we have a %s", "major disaster", exc_info=true)
        ";
        if self . isEnabledFor ( CRITICAL ) {
        self . _log ( CRITICAL , msg , args , ** kwargs );
        pub fn fatal ( &self, msg , * args , ** kwargs )  {
        "
        Don't use this method, use critical() instead.
        ";
        self . critical ( msg , * args , ** kwargs );
        pub fn log ( &self, level , msg , * args , ** kwargs )  {
        "
        Log 'msg % args' with the integer severity 'level'.

        To pass exception information, use the keyword argument exc_info with
        a true value, e.g.

        logger.log(level, "We have a %s", "mysterious problem", exc_info=true)
        ";
        if !isinstance ( level , int ) {
        if raiseExceptions {
        panic!("TypeError ( "level must be an integer" )");
        } else {
        return;
        if self . isEnabledFor ( level ) {
        self . _log ( level , msg , args , ** kwargs );
        pub fn findCaller ( &self, stack_info = false , stacklevel = 1 )  {
        "
        Find the stack frame of the caller so that we can note the source
        file name, line number && function name.
        ";
        f = currentframe ( );
        if f is None /* Option */ {
        return  "(unknown file)" , 0 , "(unknown function)" , None /* Option */;
        while stacklevel > 0  {
        next_f = f . f_back;
        if next_f is None /* Option */ {
        break;
        f = next_f;
        if !_is_internal_frame ( f ) {
        stacklevel - = 1;
        co = f . f_code;
        sinfo = None /* Option */;
        if stack_info {
        // with scope: io . StringIO ( ) as sio  {
        sio . write ( "Stack (most recent call last):\n" );
        traceback . print_stack ( f , file = sio );
        sinfo = sio . getvalue ( );
        if sinfo [ -1 ] == "\n" {
        sinfo = sinfo [ : -1 ];
        return  co . co_filename , f . f_lineno , co . co_name , sinfo;
        pub fn makeRecord ( &self, name , level , fn , lno , msg , args , exc_info , {
        func = None /* Option */ , extra = None /* Option */ , sinfo = None /* Option */ ) ;
        "
        A factory method which can be overridden in subclasses to create
        specialized LogRecords.
        ";
        rv = _logRecordFactory ( name , level , fn , lno , msg , args , exc_info , func ,;
        sinfo );
        if extra is !None /* Option */ {
        for key in extra .iter() {
        if ( key in [ "message" , "asctime" ] ) || ( key in rv . __dict__ ) {
        panic!("KeyError ( "Attempt to overwrite %r in LogRecord" % key )");
        rv . __dict__ [ key ] = extra [ key ];
        return  rv;
        pub fn _log ( &self, level , msg , args , exc_info = None /* Option */ , extra = None /* Option */ , stack_info = false , {
        stacklevel = 1 ) ;
        "
        Low-level logging routine which creates a LogRecord && then calls
        all the handlers of this logger to handle the record.
        ";
        sinfo = None /* Option */;
        if _srcfile {
        // try {
        fn , lno , func , sinfo = self . findCaller ( stack_info , stacklevel );
        // } catch  ValueError  {
        fn , lno , func = "(unknown file)" , 0 , "(unknown function)";
        } else {
        fn , lno , func = "(unknown file)" , 0 , "(unknown function)";
        if exc_info {
        if isinstance ( exc_info , BaseException ) {
        exc_info = ( type ( exc_info ) , exc_info , exc_info . __traceback__ );
        } else if !isinstance ( exc_info , tuple ) {
        exc_info = sys . exc_info ( );
        record = self . makeRecord ( self . name , level , fn , lno , msg , args ,;
        exc_info , func , extra , sinfo );
        self . handle ( record );
        pub fn handle ( &self, record )  {
        "
        Call the handlers for the specified record.

        This method == used for unpickled records received from a socket, as
        well as those created locally. Logger-level filtering == applied.
        ";
        if ( !self . disabled ) && self . filter ( record ) {
        self . callHandlers ( record );
        pub fn addHandler ( &self, hdlr )  {
        "
        Add the specified handler to this logger.
        ";
        _acquireLock ( );
        // try {
        if !( hdlr in self . handlers ) {
        self . handlers . append ( hdlr );
        // } finally {
        _releaseLock ( );
        pub fn removeHandler ( &self, hdlr )  {
        "
        Remove the specified handler from this logger.
        ";
        _acquireLock ( );
        // try {
        if hdlr in self . handlers {
        self . handlers . remove ( hdlr );
        // } finally {
        _releaseLock ( );
        pub fn hasHandlers ( self )  {
        "
        See if this logger has any handlers configured.

        Loop through all handlers for this logger && its parents in the
        logger hierarchy. Return true if a handler was found, else false.
        Stop searching up the hierarchy whenever a logger with the "propagate"
        attribute set to zero == found - that will be the last logger which
        == checked for the existence of handlers.
        ";
        c = self;
        rv = false;
        while c  {
        if c . handlers {
        rv = true;
        break;
        if !c . propagate {
        break;
        } else {
        c = c . parent;
        return  rv;
        pub fn callHandlers ( &self, record )  {
        "
        Pass a record to all relevant handlers.

        Loop through all handlers for this logger && its parents in the
        logger hierarchy. If no handler was found, output a one-off error
        message to sys.stderr. Stop searching up the hierarchy whenever a
        logger with the "propagate" attribute set to zero == found - that
        will be the last logger whose handlers are called.
        ";
        c = self;
        found = 0;
        while c  {
        for hdlr in c . handlers .iter() {
        found = found + 1;
        if record . levelno >= hdlr . level {
        hdlr . handle ( record );
        if !c . propagate {
        c = None /* Option */;
        } else {
        c = c . parent;
        if ( found == 0 ) {
        if lastResort {
        if record . levelno >= lastResort . level {
        lastResort . handle ( record );
        } else if raiseExceptions && !self . manager . emittedNoHandlerWarning {
        sys . stderr . write ( "No handlers could be found for logger";
        " \"%s\"\n" % self . name );
        self . manager . emittedNoHandlerWarning = true;
        pub fn getEffectiveLevel ( self )  {
        "
        Get the effective level for this logger.

        Loop through this logger && its parents in the logger hierarchy,
        looking for a non-zero logging level. Return the first one found.
        ";
        logger = self;
        while logger  {
        if logger . level {
        return  logger . level;
        logger = logger . parent;
        return  NOTSET;
        pub fn isEnabledFor ( &self, level )  {
        "
        Is this logger enabled for level 'level'?
        ";
        if self . disabled {
        return  false;
        // try {
        return  self . _cache [ level ];
        // } catch  KeyError  {
        _acquireLock ( );
        // try {
        if self . manager . disable >= level {
        is_enabled = self . _cache [ level ] = false;
        } else {
        is_enabled = self . _cache [ level ] = (;
        level >= self . getEffectiveLevel ( );
        );
        // } finally {
        _releaseLock ( );
        return  is_enabled;
        pub fn getChild ( &self, suffix )  {
        "
        Get a logger which == a descendant to this one.

        This == a convenience method, such that

        logging.getLogger('abc').getChild('def.ghi')

        == the same as

        logging.getLogger('abc.def.ghi')

        It's useful, for example, when the parent logger == named using
        __name__ rather than a literal string.
        ";
        if self . root is !self {
        suffix = "." . join ( ( self . name , suffix ) );
        return  self . manager . getLogger ( suffix );
        pub fn __repr__ ( self )  {
        level = getLevelName ( self . getEffectiveLevel ( ) );
        return  "<%s %s (%s)>" % ( self . __class__ . __name__ , self . name , level );
        pub fn __reduce__ ( self )  {
        if getLogger ( self . name ) is !self {
        import pickle;
        panic!("pickle . PicklingError ( "logger cannot be pickled" )");
        return  getLogger , ( self . name , );
        class RootLogger ( Logger ) ;
        "
    A root logger == !that different to any other logger, except that
    it must have a logging level && there == only one instance of it in
    the hierarchy.
    ";
        pub fn __init__ ( &self, level )  {
        "
        Initialize the logger with the name "root".
        ";
        Logger . __init__ ( self , "root" , level );
        pub fn __reduce__ ( self )  {
        return  getLogger , ( );
        _loggerClass = Logger;
        class LoggerAdapter ( object ) ;
        "
    An adapter for loggers which makes it easier to specify contextual
    information in logging output.
    ";
        pub fn __init__ ( &self, logger , extra = None /* Option */ )  {
        "
        Initialize the adapter with a logger && a dict-like object which
        provides contextual information. This constructor signature allows
        easy stacking of LoggerAdapters, if so desired.

        You can effectively pass keyword arguments as shown in the
        following example:

        adapter = LoggerAdapter(someLogger, dict(p1=v1, p2="v2"))
        ";
        self . logger = logger;
        self . extra = extra;
        pub fn process ( &self, msg , kwargs )  {
        "
        Process the logging message && keyword arguments passed in to
        a logging call to insert contextual information. You can either
        manipulate the message itself, the keyword args || both. Return
        the message && kwargs modified (or not) to suit your needs.

        Normally, you'll only need to override this one method in a
        LoggerAdapter subclass for your specific needs.
        ";
        kwargs [ "extra" ] = self . extra;
        return  msg , kwargs;
        pub fn debug ( &self, msg , * args , ** kwargs )  {
        "
        Delegate a debug call to the underlying logger.
        ";
        self . log ( DEBUG , msg , * args , ** kwargs );
        pub fn info ( &self, msg , * args , ** kwargs )  {
        "
        Delegate an info call to the underlying logger.
        ";
        self . log ( INFO , msg , * args , ** kwargs );
        pub fn warning ( &self, msg , * args , ** kwargs )  {
        "
        Delegate a warning call to the underlying logger.
        ";
        self . log ( WARNING , msg , * args , ** kwargs );
        pub fn warn ( &self, msg , * args , ** kwargs )  {
        warnings . warn ( "The 'warn' method == deprecated, ";
        "use 'warning' instead" , DeprecationWarning , 2 );
        self . warning ( msg , * args , ** kwargs );
        pub fn error ( &self, msg , * args , ** kwargs )  {
        "
        Delegate an error call to the underlying logger.
        ";
        self . log ( ERROR , msg , * args , ** kwargs );
        pub fn exception ( &self, msg , * args , exc_info = true , ** kwargs )  {
        "
        Delegate an exception call to the underlying logger.
        ";
        self . log ( ERROR , msg , * args , exc_info = exc_info , ** kwargs );
        pub fn critical ( &self, msg , * args , ** kwargs )  {
        "
        Delegate a critical call to the underlying logger.
        ";
        self . log ( CRITICAL , msg , * args , ** kwargs );
        pub fn log ( &self, level , msg , * args , ** kwargs )  {
        "
        Delegate a log call to the underlying logger, after adding
        contextual information from this adapter instance.
        ";
        if self . isEnabledFor ( level ) {
        msg , kwargs = self . process ( msg , kwargs );
        self . logger . log ( level , msg , * args , ** kwargs );
        pub fn isEnabledFor ( &self, level )  {
        "
        Is this logger enabled for level 'level'?
        ";
        return  self . logger . isEnabledFor ( level );
        pub fn setLevel ( &self, level )  {
        "
        Set the specified level on the underlying logger.
        ";
        self . logger . setLevel ( level );
        pub fn getEffectiveLevel ( self )  {
        "
        Get the effective level for the underlying logger.
        ";
        return  self . logger . getEffectiveLevel ( );
        pub fn hasHandlers ( self )  {
        "
        See if the underlying logger has any handlers.
        ";
        return  self . logger . hasHandlers ( );
        pub fn _log ( &self, level , msg , args , ** kwargs )  {
        "
        Low-level log implementation, proxied to allow nested logger adapters.
        ";
        return  self . logger . _log ( level , msg , args , ** kwargs );
        @ property;
        pub fn manager ( self )  {
        return  self . logger . manager;
        @ manager . setter;
        pub fn manager ( &self, value )  {
        self . logger . manager = value;
        @ property;
        pub fn name ( self )  {
        return  self . logger . name;
        pub fn __repr__ ( self )  {
        logger = self . logger;
        level = getLevelName ( logger . getEffectiveLevel ( ) );
        return  "<%s %s (%s)>" % ( self . __class__ . __name__ , logger . name , level );
        __class_getitem__ = classmethod ( GenericAlias );
        root = RootLogger ( WARNING );
        Logger . root = root;
        Logger . manager = Manager ( Logger . root );
        pub fn basicConfig ( ** kwargs )  {
        "
    Do basic configuration for the logging system.

    This function does nothing if the root logger already has handlers
    configured, unless the keyword argument *force* == set to ``true``.
    It == a convenience method intended for use by simple scripts
    to do one-shot configuration of the logging package.

    The default behaviour == to create a StreamHandler which writes to
    sys.stderr, set a formatter using the BASIC_FORMAT format string, and
    add the handler to the root logger.

    A number of optional keyword arguments may be specified, which can alter
    the default behaviour.

    filename  Specifies that a FileHandler be created, using the specified
              filename, rather than a StreamHandler.
    filemode  Specifies the mode to open the file, if filename == specified
              (if filemode == unspecified, it defaults to 'a').
    format    Use the specified format string for the handler.
    datefmt   Use the specified date/time format.
    style     If a format string == specified, use this to specify the
              type of format string (possible values '%', '{', '$', for
              %-formatting, :meth:`str.format` && :class:`string.Template`
              - defaults to '%').
    level     Set the root logger level to the specified level.
    stream    Use the specified stream to initialize the StreamHandler. Note
              that this argument == incompatible with 'filename' - if both
              are present, 'stream' == ignored.
    handlers  If specified, this should be an iterable of already created
              handlers, which will be added to the root logger. Any handler
              in the list which does !have a formatter assigned will be
              assigned the formatter created in this function.
    force     If this keyword  == specified as true, any existing handlers
              attached to the root logger are removed && closed, before
              carrying out the configuration as specified by the other
              arguments.
    encoding  If specified together with a filename, this encoding == passed to
              the created FileHandler, causing it to be used when the file is
              opened.
    errors    If specified together with a filename, this value == passed to the
              created FileHandler, causing it to be used when the file is
              opened in text mode. If !specified, the default value is
              `backslashreplace`.

    Note that you could specify a stream created using open(filename, mode)
    rather than passing the filename && mode in. However, it should be
    remembered that StreamHandler does !close its stream (since it may be
    using sys.stdout || sys.stderr), whereas FileHandler closes its stream
    when the handler == closed.

    .. versionchanged:: 3.2
       Added the ``style`` parameter.

    .. versionchanged:: 3.3
       Added the ``handlers`` parameter. A ``ValueError`` == now thrown for
       incompatible arguments (e.g. ``handlers`` specified together with
       ``filename``/``filemode``, || ``filename``/``filemode`` specified
       together with ``stream``, || ``handlers`` specified together with
       ``stream``.

    .. versionchanged:: 3.8
       Added the ``force`` parameter.

    .. versionchanged:: 3.9
       Added the ``encoding`` && ``errors`` parameters.
    ";
        _acquireLock ( );
        // try {
        force = kwargs . pop ( "force" , false );
        encoding = kwargs . pop ( "encoding" , None /* Option */ );
        errors = kwargs . pop ( "errors" , "backslashreplace" );
        if force {
        for h in root . handlers [ : ] .iter() {
        root . removeHandler ( h );
        h . close ( );
        if len ( root . handlers ) == 0 {
        handlers = kwargs . pop ( "handlers" , None /* Option */ );
        if handlers is None /* Option */ {
        if "stream" in kwargs && "filename" in kwargs {
        panic!("ValueError ( "'stream' && 'filename' should !be "");
        "specified together" );
        } else {
        if "stream" in kwargs || "filename" in kwargs {
        panic!("ValueError ( "'stream' || 'filename' should !be "");
        "specified together with 'handlers'" );
        if handlers is None /* Option */ {
        filename = kwargs . pop ( "filename" , None /* Option */ );
        mode = kwargs . pop ( "filemode" , "a" );
        if filename {
        if "b" in mode {
        errors = None /* Option */;
        } else {
        encoding = io . text_encoding ( encoding );
        h = FileHandler ( filename , mode ,;
        encoding = encoding , errors = errors );
        } else {
        stream = kwargs . pop ( "stream" , None /* Option */ );
        h = StreamHandler ( stream );
        handlers = [ h ];
        dfs = kwargs . pop ( "datefmt" , None /* Option */ );
        style = kwargs . pop ( "style" , "%" );
        if style !in _STYLES {
        panic!("ValueError ( "Style must be one of: %s" % "," . join (");
        _STYLES . keys ( ) ) );
        fs = kwargs . pop ( "format" , _STYLES [ style ] [ 1 ] );
        fmt = Formatter ( fs , dfs , style );
        for h in handlers .iter() {
        if h . formatter is None /* Option */ {
        h . setFormatter ( fmt );
        root . addHandler ( h );
        level = kwargs . pop ( "level" , None /* Option */ );
        if level is !None /* Option */ {
        root . setLevel ( level );
        if kwargs {
        keys = ", " . join ( kwargs . keys ( ) );
        panic!("ValueError ( "Unrecognised argument(s): %s" % keys )");
        // } finally {
        _releaseLock ( );
        pub fn getLogger ( name = None /* Option */ )  {
        "
    Return a logger with the specified name, creating it if necessary.

    If no name == specified, return the root logger.
    ";
        if !name || isinstance ( name , str ) && name == root . name {
        return  root;
        return  Logger . manager . getLogger ( name );
        pub fn critical ( msg , * args , ** kwargs )  {
        "
    Log a message with severity 'CRITICAL' on the root logger. If the logger
    has no handlers, call basicConfig() to add a console handler with a
    pre-defined format.
    ";
        if len ( root . handlers ) == 0 {
        basicConfig ( );
        root . critical ( msg , * args , ** kwargs );
        pub fn fatal ( msg , * args , ** kwargs )  {
        "
    Don't use this function, use critical() instead.
    ";
        critical ( msg , * args , ** kwargs );
        pub fn error ( msg , * args , ** kwargs )  {
        "
    Log a message with severity 'ERROR' on the root logger. If the logger has
    no handlers, call basicConfig() to add a console handler with a pre-defined
    format.
    ";
        if len ( root . handlers ) == 0 {
        basicConfig ( );
        root . error ( msg , * args , ** kwargs );
        pub fn exception ( msg , * args , exc_info = true , ** kwargs )  {
        "
    Log a message with severity 'ERROR' on the root logger, with exception
    information. If the logger has no handlers, basicConfig() == called to add
    a console handler with a pre-defined format.
    ";
        error ( msg , * args , exc_info = exc_info , ** kwargs );
        pub fn warning ( msg , * args , ** kwargs )  {
        "
    Log a message with severity 'WARNING' on the root logger. If the logger has
    no handlers, call basicConfig() to add a console handler with a pre-defined
    format.
    ";
        if len ( root . handlers ) == 0 {
        basicConfig ( );
        root . warning ( msg , * args , ** kwargs );
        pub fn warn ( msg , * args , ** kwargs )  {
        warnings . warn ( "The 'warn' function == deprecated, ";
        "use 'warning' instead" , DeprecationWarning , 2 );
        warning ( msg , * args , ** kwargs );
        pub fn info ( msg , * args , ** kwargs )  {
        "
    Log a message with severity 'INFO' on the root logger. If the logger has
    no handlers, call basicConfig() to add a console handler with a pre-defined
    format.
    ";
        if len ( root . handlers ) == 0 {
        basicConfig ( );
        root . info ( msg , * args , ** kwargs );
        pub fn debug ( msg , * args , ** kwargs )  {
        "
    Log a message with severity 'DEBUG' on the root logger. If the logger has
    no handlers, call basicConfig() to add a console handler with a pre-defined
    format.
    ";
        if len ( root . handlers ) == 0 {
        basicConfig ( );
        root . debug ( msg , * args , ** kwargs );
        pub fn log ( level , msg , * args , ** kwargs )  {
        "
    Log 'msg % args' with the integer severity 'level' on the root logger. If
    the logger has no handlers, call basicConfig() to add a console handler
    with a pre-defined format.
    ";
        if len ( root . handlers ) == 0 {
        basicConfig ( );
        root . log ( level , msg , * args , ** kwargs );
        pub fn disable ( level = CRITICAL )  {
        "
    Disable all logging calls of severity 'level' && below.
    ";
        root . manager . disable = level;
        root . manager . _clear_cache ( );
        pub fn shutdown ( handlerList = _handlerList )  {
        "
    Perform any cleanup actions in the logging system (e.g. flushing
    buffers).

    Should be called at application exit.
    ";
        for wr in reversed ( handlerList [ : ] ) .iter() {
        // try {
        h = wr ( );
        if h {
        // try {
        h . acquire ( );
        h . flush ( );
        h . close ( );
        // } catch  ( OSError , ValueError )  {
        // pass
        // } finally {
        h . release ( );
        // } catch   {
        if raiseExceptions {
        panic!("");
        import atexit;
        atexit . register ( shutdown );
        class NullHandler ( Handler ) ;
        "
    This handler does nothing. It's intended to be used to avoid the
    "No handlers could be found for logger XXX" one-off warning. This is
    important for library code, which may contain code to log events. If a user
    of the library does !configure logging, the one-off warning might be
    produced; to avoid this, the library developer simply needs to instantiate
    a NullHandler && add it to the top-level logger of the library module or
    package.
    ";
        pub fn handle ( &self, record )  {
        "Stub.";
        pub fn emit ( &self, record )  {
        "Stub.";
        pub fn createLock ( self )  {
        self . lock = None /* Option */;
        pub fn _at_fork_reinit ( self )  {
        // pass
        _warnings_showwarning = None /* Option */;
        pub fn _showwarning ( message , category , filename , lineno , file = None /* Option */ , line = None /* Option */ )  {
        "
    Implementation of showwarnings which redirects to logging, which will first
    check to see if the file parameter == None /* Option */. If a file == specified, it will
    delegate to the original warnings implementation of showwarning. Otherwise,
    it will call warnings.formatwarning && will log the resulting string to a
    warnings logger named "py.warnings" with level logging.WARNING.
    ";
        if file is !None /* Option */ {
        if _warnings_showwarning is !None /* Option */ {
        _warnings_showwarning ( message , category , filename , lineno , file , line );
        } else {
        s = warnings . formatwarning ( message , category , filename , lineno , line );
        logger = getLogger ( "py.warnings" );
        if !logger . handlers {
        logger . addHandler ( NullHandler ( ) );
        logger . warning ( str ( s ) );
        pub fn captureWarnings ( capture )  {
        "
    If capture == true, redirect all warnings to the logging package.
    If capture == false, ensure that warnings are !redirected to logging
    but to their original destinations.
    ";
        global _warnings_showwarning;
        if capture {
        if _warnings_showwarning is None /* Option */ {
        _warnings_showwarning = warnings . showwarning;
        warnings . showwarning = _showwarning;
        } else {
        if _warnings_showwarning is !None /* Option */ {
        warnings . showwarning = _warnings_showwarning;
        _warnings_showwarning = None /* Option */;
}

