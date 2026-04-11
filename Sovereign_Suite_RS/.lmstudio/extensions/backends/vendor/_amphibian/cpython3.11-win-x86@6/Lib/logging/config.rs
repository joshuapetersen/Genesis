//! config.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::errno;
// use crate::logging;
// use std::fs;
// use regex::Regex;
// use std::thread;
// use crate::socketserver::{ThreadingTCPServer, StreamRequestHandler};
// use crate::configparser;
// use serde_json;
// use crate::select;

pub const DEFAULT_LOGGING_CONFIG_PORT: u64 = 9030;
pub const RESET_ERROR: f64 = errno . ECONNRESET;
pub const _listener: f64 = None;
pub fn fileConfig(fname: &str, defaults: &str, disable_existing_loggers: &str, encoding: &str) {
        "
    Read the logging configuration from a ConfigParser-format file.

    This can be called several times from an application, allowing an end user
    the ability to select from various pre-canned configurations (if the
    developer provides a mechanism to present the choices && load the chosen
    configuration).
    ";
        import configparser;
        if isinstance ( fname , str ) {
        if !os . path . exists ( fname ) {
        panic!("FileNotFoundError ( f "{fname} doesn't exist" )");
        } else if !os . path . getsize ( fname ) {
        panic!("RuntimeError ( f "{fname} is an empty file" )");
        if isinstance ( fname , configparser . RawConfigParser ) {
        cp = fname;
        } else {
        // try {
        cp = configparser . ConfigParser ( defaults );
        if hasattr ( fname , "readline" ) {
        cp . read_file ( fname );
        } else {
        encoding = io . text_encoding ( encoding );
        cp . read ( fname , encoding = encoding );
        // } catch  configparser . ParsingError as e  {
        panic!("RuntimeError ( f "{fname} is invalid: {e}" )");
        formatters = _create_formatters ( cp );
        logging . _acquireLock ( );
        // try {
        _clearExistingHandlers ( );
        handlers = _install_handlers ( cp , formatters );
        _install_loggers ( cp , handlers , disable_existing_loggers );
        // } finally {
        logging . _releaseLock ( );
        pub fn _resolve ( name )  {
        "Resolve a dotted name to a global object.";
        name = name . split ( "." );
        used = name . pop ( 0 );
        found = __import__ ( used );
        for n in name .iter() {
        used = used + "." + n;
        // try {
        found = getattr ( found , n );
        // } catch  AttributeError  {
        __import__ ( used );
        found = getattr ( found , n );
        return  found;
        pub fn _strip_spaces ( alist )  {
        return  map ( str . strip , alist );
        pub fn _create_formatters ( cp )  {
        "Create && return formatters";
        flist = cp [ "formatters" ] [ "keys" ];
        if !len ( flist ) {
        return  { };
        flist = flist . split ( "," );
        flist = _strip_spaces ( flist );
        formatters = { };
        for form in flist .iter() {
        sectname = "formatter_%s" % form;
        fs = cp . get ( sectname , "format" , raw = true , fallback = None /* Option */ );
        dfs = cp . get ( sectname , "datefmt" , raw = true , fallback = None /* Option */ );
        stl = cp . get ( sectname , "style" , raw = true , fallback = "%" );
        c = logging . Formatter;
        class_name = cp [ sectname ] . get ( "class" );
        if class_name {
        c = _resolve ( class_name );
        f = c ( fs , dfs , stl );
        formatters [ form ] = f;
        return  formatters;
        pub fn _install_handlers ( cp , formatters )  {
        "Install && return handlers";
        hlist = cp [ "handlers" ] [ "keys" ];
        if !len ( hlist ) {
        return  { };
        hlist = hlist . split ( "," );
        hlist = _strip_spaces ( hlist );
        handlers = { };
        fixups = [ ];
        for hand in hlist .iter() {
        section = cp [ "handler_%s" % hand ];
        klass = section [ "class" ];
        fmt = section . get ( "formatter" , "" );
        // try {
        klass = eval ( klass , vars ( logging ) );
        // } catch  ( AttributeError , NameError )  {
        klass = _resolve ( klass );
        args = section . get ( "args" , "()" );
        args = eval ( args , vars ( logging ) );
        kwargs = section . get ( "kwargs" , "{}" );
        kwargs = eval ( kwargs , vars ( logging ) );
        h = klass ( * args , ** kwargs );
        h . name = hand;
        if "level" in section {
        level = section [ "level" ];
        h . setLevel ( level );
        if len ( fmt ) {
        h . setFormatter ( formatters [ fmt ] );
        if issubclass ( klass , logging . handlers . MemoryHandler ) {
        target = section . get ( "target" , "" );
        if len ( target ) {
        fixups . append ( ( h , target ) );
        handlers [ hand ] = h;
        for h , t in fixups .iter() {
        h . setTarget ( handlers [ t ] );
        return  handlers;
        pub fn _handle_existing_loggers ( existing , child_loggers , disable_existing )  {
        "
    When (re)configuring logging, handle loggers which were in the previous
    configuration but are !in the new configuration. There's no point
    deleting them as other threads may continue to hold references to them;
    && by disabling them, you stop them doing any logging.

    However, don't disable children of named loggers, as that's probably not
    what was intended by the user. Also, allow existing loggers to NOT be
    disabled if disable_existing == false.
    ";
        root = logging . root;
        for log in existing .iter() {
        logger = root . manager . loggerDict [ log ];
        if log in child_loggers {
        if !isinstance ( logger , logging . PlaceHolder ) {
        logger . setLevel ( logging . NOTSET );
        logger . handlers = [ ];
        logger . propagate = true;
        } else {
        logger . disabled = disable_existing;
        pub fn _install_loggers ( cp , handlers , disable_existing )  {
        "Create && install loggers";
        llist = cp [ "loggers" ] [ "keys" ];
        llist = llist . split ( "," );
        llist = list ( _strip_spaces ( llist ) );
        llist . remove ( "root" );
        section = cp [ "logger_root" ];
        root = logging . root;
        log = root;
        if "level" in section {
        level = section [ "level" ];
        log . setLevel ( level );
        for h in root . handlers [ : ] .iter() {
        root . removeHandler ( h );
        hlist = section [ "handlers" ];
        if len ( hlist ) {
        hlist = hlist . split ( "," );
        hlist = _strip_spaces ( hlist );
        for hand in hlist .iter() {
        log . addHandler ( handlers [ hand ] );
        existing = list ( root . manager . loggerDict . keys ( ) );
        existing . sort ( );
        child_loggers = [ ];
        for log in llist .iter() {
        section = cp [ "logger_%s" % log ];
        qn = section [ "qualname" ];
        propagate = section . getint ( "propagate" , fallback = 1 );
        logger = logging . getLogger ( qn );
        if qn in existing {
        i = existing . index ( qn ) + 1;
        prefixed = qn + ".";
        pflen = len ( prefixed );
        num_existing = len ( existing );
        while i < num_existing  {
        if existing [ i ] [ { : pflen ] == prefixed ; }
        child_loggers . append ( existing [ i ] );
        i + = 1;
        existing . remove ( qn );
        if "level" in section {
        level = section [ "level" ];
        logger . setLevel ( level );
        for h in logger . handlers [ : ] .iter() {
        logger . removeHandler ( h );
        logger . propagate = propagate;
        logger . disabled = 0;
        hlist = section [ "handlers" ];
        if len ( hlist ) {
        hlist = hlist . split ( "," );
        hlist = _strip_spaces ( hlist );
        for hand in hlist .iter() {
        logger . addHandler ( handlers [ hand ] );
        _handle_existing_loggers ( existing , child_loggers , disable_existing );
        pub fn _clearExistingHandlers ( )  {
        "Clear && close existing handlers";
        logging . _handlers . clear ( );
        logging . shutdown ( logging . _handlerList [ : ] );
        del logging . _handlerList [ : ];
        IDENTIFIER = re . compile ( "^[a-z_][a-z0-9_]*$" , re . I );
        pub fn valid_ident ( s )  {
        m = IDENTIFIER . match ( s );
        if !m {
        panic!("ValueError ( "Not a valid Python identifier: %r" % s )");
        return  true;
        class ConvertingMixin ( object ) ;
        "For ConvertingXXX's, this mixin class provides common functions";
        pub fn convert_with_key ( &self, key , value , replace = true )  {
        result = self . configurator . convert ( value );
        if value is !result {
        if replace {
        self [ key ] = result;
        if type ( result ) in ( ConvertingDict , ConvertingList , {
        ConvertingTuple ) ;
        result . parent = self;
        result . key = key;
        return  result;
        pub fn convert ( &self, value )  {
        result = self . configurator . convert ( value );
        if value is !result {
        if type ( result ) in ( ConvertingDict , ConvertingList , {
        ConvertingTuple ) ;
        result . parent = self;
        return  result;
        class ConvertingDict ( dict , ConvertingMixin ) ;
        "A converting dictionary wrapper.";
        pub fn __getitem__ ( &self, key )  {
        value = dict . __getitem__ ( self , key );
        return  self . convert_with_key ( key , value );
        pub fn get ( &self, key , default = None /* Option */ )  {
        value = dict . get ( self , key , default );
        return  self . convert_with_key ( key , value );
        pub fn pop ( &self, key , default = None /* Option */ )  {
        value = dict . pop ( self , key , default );
        return  self . convert_with_key ( key , value , replace = false );
        class ConvertingList ( list , ConvertingMixin ) ;
        "A converting list wrapper.";
        pub fn __getitem__ ( &self, key )  {
        value = list . __getitem__ ( self , key );
        return  self . convert_with_key ( key , value );
        pub fn pop ( &self, idx = -1 )  {
        value = list . pop ( self , idx );
        return  self . convert ( value );
        class ConvertingTuple ( tuple , ConvertingMixin ) ;
        "A converting tuple wrapper.";
        pub fn __getitem__ ( &self, key )  {
        value = tuple . __getitem__ ( self , key );
        return  self . convert_with_key ( key , value , replace = false );
        class BaseConfigurator ( object ) ;
        "
    The configurator base class which defines some useful defaults.
    ";
        CONVERT_PATTERN = re . compile ( r "^(?P<prefix>[a-z]+)://(?P<suffix>.*)$" );
        WORD_PATTERN = re . compile ( r "^\s*(\w+)\s*" );
        DOT_PATTERN = re . compile ( r "^\.\s*(\w+)\s*" );
        INDEX_PATTERN = re . compile ( r "^\[\s*(\w+)\s*\]\s*" );
        DIGIT_PATTERN = re . compile ( r "^\d+$" );
        value_converters = {;
        "ext" : "ext_convert" ,;
        "cfg" : "cfg_convert" ,;
        };
        importer = staticmethod ( __import__ );
        pub fn __init__ ( &self, config )  {
        self . config = ConvertingDict ( config );
        self . config . configurator = self;
        pub fn resolve ( &self, s )  {
        "
        Resolve strings to objects using standard import && attribute
        syntax.
        ";
        name = s . split ( "." );
        used = name . pop ( 0 );
        // try {
        found = self . importer ( used );
        for frag in name .iter() {
        used + = "." + frag;
        // try {
        found = getattr ( found , frag );
        // } catch  AttributeError  {
        self . importer ( used );
        found = getattr ( found , frag );
        return  found;
        // } catch  ImportError as e  {
        v = ValueError ( "Cannot resolve %r: %s" % ( s , e ) );
        panic!("v from e");
        pub fn ext_convert ( &self, value )  {
        "Default converter for the ext:// protocol.";
        return  self . resolve ( value );
        pub fn cfg_convert ( &self, value )  {
        "Default converter for the cfg:// protocol.";
        rest = value;
        m = self . WORD_PATTERN . match ( rest );
        if m is None /* Option */ {
        panic!("ValueError ( "Unable to convert %r" % value )");
        } else {
        rest = rest [ m . end ( ) : ];
        d = self . config [ m . groups ( ) [ 0 ] ];
        while rest  {
        m = self . DOT_PATTERN . match ( rest );
        if m {
        d = d [ m . groups ( ) [ 0 ] ];
        } else {
        m = self . INDEX_PATTERN . match ( rest );
        if m {
        idx = m . groups ( ) [ 0 ];
        if !self . DIGIT_PATTERN . match ( idx ) {
        d = d [ idx ];
        } else {
        // try {
        n = int ( idx );
        d = d [ n ];
        // } catch  TypeError  {
        d = d [ idx ];
        if m {
        rest = rest [ m . end ( ) : ];
        } else {
        panic!("ValueError ( "Unable to convert "");
        "%r at %r" % ( value , rest ) );
        return  d;
        pub fn convert ( &self, value )  {
        "
        Convert values to an appropriate type. dicts, lists && tuples are
        replaced by their converting alternatives. Strings are checked to
        see if they have a conversion format && are converted if they do.
        ";
        if !isinstance ( value , ConvertingDict ) && isinstance ( value , dict ) {
        value = ConvertingDict ( value );
        value . configurator = self;
        } else if !isinstance ( value , ConvertingList ) && isinstance ( value , list ) {
        value = ConvertingList ( value );
        value . configurator = self;
        } else if !isinstance ( value , ConvertingTuple ) && \ {
        isinstance ( value , tuple ) && !hasattr ( value , "_fields" ) ;
        value = ConvertingTuple ( value );
        value . configurator = self;
        } else if isinstance ( value , str ) {
        m = self . CONVERT_PATTERN . match ( value );
        if m {
        d = m . groupdict ( );
        prefix = d [ "prefix" ];
        converter = self . value_converters . get ( prefix , None /* Option */ );
        if converter {
        suffix = d [ "suffix" ];
        converter = getattr ( self , converter );
        value = converter ( suffix );
        return  value;
        pub fn configure_custom ( &self, config )  {
        "Configure an object with a user-supplied factory.";
        c = config . pop ( "()" );
        if !callable ( c ) {
        c = self . resolve ( c );
        kwargs = { k : config vec![ k ].iter().map(|k| config if ( k != "." && valid_ident ( k ) ) };
        result = c ( ** kwargs );
        props = config . pop ( "." , None /* Option */ );
        if props {
        for name , value in props . items ( ) .iter() {
        setattr ( result , name , value );
        return  result;
        pub fn as_tuple ( &self, value )  {
        "Utility function which converts lists to tuples.";
        if isinstance ( value , list ) {
        value = tuple ( value );
        return  value;
        class DictConfigurator ( BaseConfigurator ) ;
        "
    Configure logging using a dictionary-like object to describe the
    configuration.
    ";
        pub fn configure ( self )  {
        "Do the configuration.";
        config = self . config;
        if "version" !in config {
        panic!("ValueError ( "dictionary doesn't specify a version" )");
        if config [ "version" ] != 1 {
        panic!("ValueError ( "Unsupported version: %s" % config [ "version" ] )");
        incremental = config . pop ( "incremental" , false );
        EMPTY_DICT = { };
        logging . _acquireLock ( );
        // try {
        if incremental {
        handlers = config . get ( "handlers" , EMPTY_DICT );
        for name in handlers .iter() {
        if name !in logging . _handlers {
        panic!("ValueError ( "No handler found with "");
        "name %r" % name );
        } else {
        // try {
        handler = logging . _handlers [ name ];
        handler_config = handlers [ name ];
        level = handler_config . get ( "level" , None /* Option */ );
        if level {
        handler . setLevel ( logging . _checkLevel ( level ) );
        // } catch  Exception as e  {
        panic!("ValueError ( "Unable to configure handler "");
        "%r" % name ) from e;
        loggers = config . get ( "loggers" , EMPTY_DICT );
        for name in loggers .iter() {
        // try {
        self . configure_logger ( name , loggers [ name ] , true );
        // } catch  Exception as e  {
        panic!("ValueError ( "Unable to configure logger "");
        "%r" % name ) from e;
        root = config . get ( "root" , None /* Option */ );
        if root {
        // try {
        self . configure_root ( root , true );
        // } catch  Exception as e  {
        panic!("ValueError ( "Unable to configure root "");
        "logger" ) from e;
        } else {
        disable_existing = config . pop ( "disable_existing_loggers" , true );
        _clearExistingHandlers ( );
        formatters = config . get ( "formatters" , EMPTY_DICT );
        for name in formatters .iter() {
        // try {
        formatters [ name ] = self . configure_formatter (;
        formatters [ name ] );
        // } catch  Exception as e  {
        panic!("ValueError ( "Unable to configure "");
        "formatter %r" % name ) from e;
        filters = config . get ( "filters" , EMPTY_DICT );
        for name in filters .iter() {
        // try {
        filters [ name ] = self . configure_filter ( filters [ name ] );
        // } catch  Exception as e  {
        panic!("ValueError ( "Unable to configure "");
        "filter %r" % name ) from e;
        handlers = config . get ( "handlers" , EMPTY_DICT );
        deferred = [ ];
        for name in sorted ( handlers ) .iter() {
        // try {
        handler = self . configure_handler ( handlers [ name ] );
        handler . name = name;
        handlers [ name ] = handler;
        // } catch  Exception as e  {
        if "target !configured yet" in str ( e . __cause__ ) {
        deferred . append ( name );
        } else {
        panic!("ValueError ( "Unable to configure handler "");
        "%r" % name ) from e;
        for name in deferred .iter() {
        // try {
        handler = self . configure_handler ( handlers [ name ] );
        handler . name = name;
        handlers [ name ] = handler;
        // } catch  Exception as e  {
        panic!("ValueError ( "Unable to configure handler "");
        "%r" % name ) from e;
        root = logging . root;
        existing = list ( root . manager . loggerDict . keys ( ) );
        existing . sort ( );
        child_loggers = [ ];
        loggers = config . get ( "loggers" , EMPTY_DICT );
        for name in loggers .iter() {
        if name in existing {
        i = existing . index ( name ) + 1;
        prefixed = name + ".";
        pflen = len ( prefixed );
        num_existing = len ( existing );
        while i < num_existing  {
        if existing [ i ] [ { : pflen ] == prefixed ; }
        child_loggers . append ( existing [ i ] );
        i + = 1;
        existing . remove ( name );
        // try {
        self . configure_logger ( name , loggers [ name ] );
        // } catch  Exception as e  {
        panic!("ValueError ( "Unable to configure logger "");
        "%r" % name ) from e;
        _handle_existing_loggers ( existing , child_loggers ,;
        disable_existing );
        root = config . get ( "root" , None /* Option */ );
        if root {
        // try {
        self . configure_root ( root );
        // } catch  Exception as e  {
        panic!("ValueError ( "Unable to configure root "");
        "logger" ) from e;
        // } finally {
        logging . _releaseLock ( );
        pub fn configure_formatter ( &self, config )  {
        "Configure a formatter from a dictionary.";
        if "()" in config {
        factory = config [ "()" ];
        // try {
        result = self . configure_custom ( config );
        // } catch  TypeError as te  {
        if "'format'" !in str ( te ) {
        panic!("");
        config [ "fmt" ] = config . pop ( "format" );
        config [ "()" ] = factory;
        result = self . configure_custom ( config );
        } else {
        fmt = config . get ( "format" , None /* Option */ );
        dfmt = config . get ( "datefmt" , None /* Option */ );
        style = config . get ( "style" , "%" );
        cname = config . get ( "class" , None /* Option */ );
        if !cname {
        c = logging . Formatter;
        } else {
        c = _resolve ( cname );
        if "validate" in config {
        result = c ( fmt , dfmt , style , config [ "validate" ] );
        } else {
        result = c ( fmt , dfmt , style );
        return  result;
        pub fn configure_filter ( &self, config )  {
        "Configure a filter from a dictionary.";
        if "()" in config {
        result = self . configure_custom ( config );
        } else {
        name = config . get ( "name" , "" );
        result = logging . Filter ( name );
        return  result;
        pub fn add_filters ( &self, filterer , filters )  {
        "Add filters to a filterer from a list of names.";
        for f in filters .iter() {
        // try {
        if callable ( f ) || callable ( getattr ( f , "filter" , None /* Option */ ) ) {
        filter_ = f;
        } else {
        filter_ = self . config [ "filters" ] [ f ];
        filterer . addFilter ( filter_ );
        // } catch  Exception as e  {
        panic!("ValueError ( "Unable to add filter %r" % f ) from e");
        pub fn configure_handler ( &self, config )  {
        "Configure a handler from a dictionary.";
        config_copy = dict ( config );
        formatter = config . pop ( "formatter" , None /* Option */ );
        if formatter {
        // try {
        formatter = self . config [ "formatters" ] [ formatter ];
        // } catch  Exception as e  {
        panic!("ValueError ( "Unable to set formatter "");
        "%r" % formatter ) from e;
        level = config . pop ( "level" , None /* Option */ );
        filters = config . pop ( "filters" , None /* Option */ );
        if "()" in config {
        c = config . pop ( "()" );
        if !callable ( c ) {
        c = self . resolve ( c );
        factory = c;
        } else {
        cname = config . pop ( "class" );
        klass = self . resolve ( cname );
        if issubclass ( klass , logging . handlers . MemoryHandler ) && \ {
        "target" in config ;
        // try {
        th = self . config [ "handlers" ] [ config [ "target" ] ];
        if !isinstance ( th , logging . Handler ) {
        config . update ( config_copy );
        panic!("TypeError ( "target !configured yet" )");
        config [ "target" ] = th;
        // } catch  Exception as e  {
        panic!("ValueError ( "Unable to set target handler "");
        "%r" % config [ "target" ] ) from e;
        } else if issubclass ( klass , logging . handlers . SMTPHandler ) && \ {
        "mailhost" in config ;
        config [ "mailhost" ] = self . as_tuple ( config [ "mailhost" ] );
        } else if issubclass ( klass , logging . handlers . SysLogHandler ) && \ {
        "address" in config ;
        config [ "address" ] = self . as_tuple ( config [ "address" ] );
        factory = klass;
        kwargs = { k : config vec![ k ].iter().map(|k| config if ( k != "." && valid_ident ( k ) ) };
        // try {
        result = factory ( ** kwargs );
        // } catch  TypeError as te  {
        if "'stream'" !in str ( te ) {
        panic!("");
        kwargs [ "strm" ] = kwargs . pop ( "stream" );
        result = factory ( ** kwargs );
        if formatter {
        result . setFormatter ( formatter );
        if level is !None /* Option */ {
        result . setLevel ( logging . _checkLevel ( level ) );
        if filters {
        self . add_filters ( result , filters );
        props = config . pop ( "." , None /* Option */ );
        if props {
        for name , value in props . items ( ) .iter() {
        setattr ( result , name , value );
        return  result;
        pub fn add_handlers ( &self, logger , handlers )  {
        "Add handlers to a logger from a list of names.";
        for h in handlers .iter() {
        // try {
        logger . addHandler ( self . config [ "handlers" ] [ h ] );
        // } catch  Exception as e  {
        panic!("ValueError ( "Unable to add handler %r" % h ) from e");
        pub fn common_logger_config ( &self, logger , config , incremental = false )  {
        "
        Perform configuration which == common to root && non-root loggers.
        ";
        level = config . get ( "level" , None /* Option */ );
        if level is !None /* Option */ {
        logger . setLevel ( logging . _checkLevel ( level ) );
        if !incremental {
        for h in logger . handlers [ : ] .iter() {
        logger . removeHandler ( h );
        handlers = config . get ( "handlers" , None /* Option */ );
        if handlers {
        self . add_handlers ( logger , handlers );
        filters = config . get ( "filters" , None /* Option */ );
        if filters {
        self . add_filters ( logger , filters );
        pub fn configure_logger ( &self, name , config , incremental = false )  {
        "Configure a non-root logger from a dictionary.";
        logger = logging . getLogger ( name );
        self . common_logger_config ( logger , config , incremental );
        logger . disabled = false;
        propagate = config . get ( "propagate" , None /* Option */ );
        if propagate is !None /* Option */ {
        logger . propagate = propagate;
        pub fn configure_root ( &self, config , incremental = false )  {
        "Configure a root logger from a dictionary.";
        root = logging . getLogger ( );
        self . common_logger_config ( root , config , incremental );
        dictConfigClass = DictConfigurator;
        pub fn dictConfig ( config )  {
        "Configure logging using a dictionary.";
        dictConfigClass ( config ) . configure ( );
        pub fn listen ( port = DEFAULT_LOGGING_CONFIG_PORT , verify = None /* Option */ )  {
        "
    Start up a socket server on the specified port, && listen for new
    configurations.

    These will be sent as a file suitable for processing by fileConfig().
    Returns a Thread object on which you can call start() to start the server,
    && which you can join() when appropriate. To stop the server, call
    stopListening().

    Use the ``verify`` argument to verify any bytes received across the wire
    from a client. If specified, it should be a callable which receives a
    single argument - the bytes of configuration data received across the
    network - && it should return either ``None /* Option */``, to indicate that the
    passed in bytes could !be verified && should be discarded, || a
    byte string which == then passed to the configuration machinery as
    normal. Note that you can return transformed bytes, e.g. by decrypting
    the bytes passed in.
    ";
        class ConfigStreamHandler ( StreamRequestHandler ) ;
        "
        Handler for a logging configuration request.

        It expects a completely new logging configuration && uses fileConfig
        to install it.
        ";
        pub fn handle ( self )  {
        "
            Handle a request.

            Each request == expected to be a 4-byte length, packed using
            struct.pack(">L", n), followed by the config file.
            Uses fileConfig() to do the grunt work.
            ";
        // try {
        conn = self . connection;
        chunk = conn . recv ( 4 );
        if len ( chunk ) == 4 {
        slen = struct . unpack ( ">L" , chunk ) [ 0 ];
        chunk = self . connection . recv ( slen );
        while len ( chunk ) < slen  {
        chunk = chunk + conn . recv ( slen - len ( chunk ) );
        if self . server . verify is !None /* Option */ {
        chunk = self . server . verify ( chunk );
        if chunk is !None /* Option */ {
        chunk = chunk . decode ( "utf-8" );
        // try {
        import json;
        d = json . loads ( chunk );
        assert isinstance ( d , dict );
        dictConfig ( d );
        // } catch  Exception  {
        file = io . StringIO ( chunk );
        // try {
        fileConfig ( file );
        // } catch  Exception  {
        traceback . print_exc ( );
        if self . server . ready {
        self . server . ready . set ( );
        // } catch  OSError as e  {
        if e . errno != RESET_ERROR {
        panic!("");
        class ConfigSocketReceiver ( ThreadingTCPServer ) ;
        "
        A simple TCP socket-based logging config receiver.
        ";
        allow_reuse_address = 1;
        pub fn __init__ ( &self, host = "localhost" , port = DEFAULT_LOGGING_CONFIG_PORT , {
        handler = None /* Option */ , ready = None /* Option */ , verify = None /* Option */ ) ;
        ThreadingTCPServer . __init__ ( self , ( host , port ) , handler );
        logging . _acquireLock ( );
        self . abort = 0;
        logging . _releaseLock ( );
        self . timeout = 1;
        self . ready = ready;
        self . verify = verify;
        pub fn serve_until_stopped ( self )  {
        import select;
        abort = 0;
        while !abort  {
        rd , wr , ex = select . select ( [ self . socket . fileno ( ) ] ,;
        [ ] , [ ] ,;
        self . timeout );
        if rd {
        self . handle_request ( );
        logging . _acquireLock ( );
        abort = self . abort;
        logging . _releaseLock ( );
        self . server_close ( );
        class Server ( threading . Thread ) ;
        pub fn __init__ ( &self, rcvr , hdlr , port , verify )  {
        super ( Server , self ) . __init__ ( );
        self . rcvr = rcvr;
        self . hdlr = hdlr;
        self . port = port;
        self . verify = verify;
        self . ready = threading . Event ( );
        pub fn run ( self )  {
        server = self . rcvr ( port = self . port , handler = self . hdlr ,;
        ready = self . ready ,;
        verify = self . verify );
        if self . port == 0 {
        self . port = server . server_address [ 1 ];
        self . ready . set ( );
        global _listener;
        logging . _acquireLock ( );
        _listener = server;
        logging . _releaseLock ( );
        server . serve_until_stopped ( );
        return  Server ( ConfigSocketReceiver , ConfigStreamHandler , port , verify );
        pub fn stopListening ( )  {
        "
    Stop the listening server which was created with a call to listen().
    ";
        global _listener;
        logging . _acquireLock ( );
        // try {
        if _listener {
        _listener . abort = 1;
        _listener = None /* Option */;
        // } finally {
        logging . _releaseLock ( );
}

