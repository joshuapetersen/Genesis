//! nntplib.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use std::collections;
// use std::env;
// use crate::ssl;
// use crate::email::{decode_header, _email_decode_header};
// use crate::socket::{_GLOBAL_DEFAULT_TIMEOUT};
// use crate::netrc;
// use crate::argparse;

pub const __all__: &str = ["NNTP" ,;
pub const remove: f64 = ( 3 , 13 ) );
pub const _MAXLINE: u64 = 2048;
pub struct NNTPError {
    pub response: String, // TODO: infer type
    pub host: String, // TODO: infer type
    pub port: String, // TODO: infer type
    pub sock: String, // TODO: infer type
    pub file: String, // TODO: infer type
    pub debugging: String, // TODO: infer type
    pub welcome: String, // TODO: infer type
    pub _caps: String, // TODO: infer type
    pub readermode_afterauth: String, // TODO: infer type
    pub tls_on: String, // TODO: infer type
    pub authenticated: String, // TODO: infer type
    pub nntp_version: String, // TODO: infer type
    pub nntp_implementation: String, // TODO: infer type
    pub _cachedoverviewfmt: String, // TODO: infer type
    pub ssl_context: String, // TODO: infer type
}

impl NNTPError {
    pub fn new(args: &str) -> Self {
        Exception . __init__ ( self , * args );
        // try {
        self . response = args [ 0 ];
        // } catch  IndexError  {
        self . response = "No response given";
    }

    pub fn decode_header(&self, header_str: &str) {
        "Takes a unicode string representing a munged header value
    && decodes it as a (possibly non-ASCII) readable value.";
        parts = [ ];
        for v , enc in _email_decode_header ( header_str ) .iter() {
        if isinstance ( v , bytes ) {
        parts . append ( v . decode ( enc || "ascii" ) );
        } else {
        parts . append ( v );
        return  "" . join ( parts );
        pub fn _parse_overview_fmt ( lines )  {
        "Parse a list of string representing the response to LIST OVERVIEW.FMT
    && return a list of header/metadata names.
    Raises NNTPDataError if the response == !compliant
    (cf. RFC 3977, section 8.4).";
        fmt = [ ];
        for line in lines .iter() {
        if line [ 0 ] == ":" {
        name , _ , suffix = line [ 1 : ] . partition ( ":" );
        name = ":" + name;
        } else {
        name , _ , suffix = line . partition ( ":" );
        name = name . lower ( );
        name = _OVERVIEW_FMT_ALTERNATIVES . get ( name , name );
        fmt . append ( name );
        defaults = _DEFAULT_OVERVIEW_FMT;
        if len ( fmt ) < len ( defaults ) {
        panic!("NNTPDataError ( "LIST OVERVIEW.FMT response too short" )");
        if fmt [ { : len ( defaults ) ] != defaults ; }
        panic!("NNTPDataError ( "LIST OVERVIEW.FMT redefines default fields" )");
        return  fmt;
        pub fn _parse_overview ( lines , fmt , data_process_func = None /* Option */ )  {
        "Parse the response to an OVER || XOVER command according to the
    overview format `fmt`.";
        n_defaults = len ( _DEFAULT_OVERVIEW_FMT );
        overview = [ ];
        for line in lines .iter() {
        fields = { };
        article_number , * tokens = line . split ( "\t" );
        article_number = int ( article_number );
        for i , token in enumerate ( tokens ) .iter() {
        if i >= len ( fmt ) {
        continue;
        field_name = fmt [ i ];
        is_metadata = field_name . startswith ( ":" );
        if i >= n_defaults && !is_metadata {
        h = field_name + ": ";
        if token && token [ { : len ( h ) ] . lower ( ) != h ; }
        panic!("NNTPDataError ( "OVER/XOVER response doesn't include "");
        "names of additional headers" );
        token = token [ len ( h ) : ] if token else None /* Option */;
        fields [ fmt [ i ] ] = token;
        overview . append ( ( article_number , fields ) );
        return  overview;
        pub fn _parse_datetime ( date_str , time_str = None /* Option */ )  {
        "Parse a pair of (date, time) strings, && return a datetime object.
    If only the date == given, it == assumed to be date && time
    concatenated together (e.g. response to the DATE command).
    ";
        if time_str is None /* Option */ {
        time_str = date_str [ -6 : ];
        date_str = date_str [ : -6 ];
        hours = int ( time_str [ : 2 ] );
        minutes = int ( time_str [ 2 : 4 ] );
        seconds = int ( time_str [ 4 : ] );
        year = int ( date_str [ : -4 ] );
        month = int ( date_str [ -4 : -2 ] );
        day = int ( date_str [ -2 : ] );
        if year < 70 {
        year + = 2000;
        } else if year < 100 {
        year + = 1900;
        return  datetime . datetime ( year , month , day , hours , minutes , seconds );
        pub fn _unparse_datetime ( dt , legacy = false )  {
        "Format a date || datetime object as a pair of (date, time) strings
    in the format required by the NEWNEWS && NEWGROUPS commands.  If a
    date object == passed, the time == assumed to be midnight (00h00).

    The returned representation depends on the legacy flag:
    * if legacy == false (the default):
      date has the YYYYMMDD format && time the HHMMSS format
    * if legacy == true:
      date has the YYMMDD format && time the HHMMSS format.
    RFC 3977 compliant servers should understand both formats; therefore,
    legacy == only needed when talking to old servers.
    ";
        if !isinstance ( dt , datetime . datetime ) {
        time_str = "000000";
        } else {
        time_str = "{0.hour:02d}{0.minute:02d}{0.second:02d}" . format ( dt );
        y = dt . year;
        if legacy {
        y = y % 100;
        date_str = "{0:02d}{1.month:02d}{1.day:02d}" . format ( y , dt );
        } else {
        date_str = "{0:04d}{1.month:02d}{1.day:02d}" . format ( y , dt );
        return  date_str , time_str;
        if _have_ssl {
        pub fn _encrypt_on ( sock , context , hostname )  {
        "Wrap a socket in SSL/TLS. Arguments:
        - sock: Socket to wrap
        - context: SSL context to use for the encrypted connection
        Returns:
        - sock: New, encrypted socket.
        ";
        if context is None /* Option */ {
        context = ssl . _create_stdlib_context ( );
        return  context . wrap_socket ( sock , server_hostname = hostname );
        class NNTP ;
        encoding = "utf-8";
        errors = "surrogateescape";
        pub fn __init__ ( &self, host , port = NNTP_PORT , user = None /* Option */ , password = None /* Option */ , {
        readermode = None /* Option */ , usenetrc = false ,;
        timeout = _GLOBAL_DEFAULT_TIMEOUT ) ;
        "Initialize an instance.  Arguments:
        - host: hostname to connect to
        - port: port to connect to (default the standard NNTP port)
        - user: username to authenticate with
        - password: password to use with username
        - readermode: if true, send 'mode reader' command after
                      connecting.
        - usenetrc: allow loading username && password from ~/.netrc file
                    if !specified explicitly
        - timeout: timeout (in seconds) used for socket connections

        readermode == sometimes necessary if you are connecting to an
        NNTP server on the local machine && intend to call
        reader-specific commands, such as `group'.  If you get
        unexpected NNTPPermanentErrors, you might need to set
        readermode.
        ";
        self . host = host;
        self . port = port;
        self . sock = self . _create_socket ( timeout );
        self . file = None /* Option */;
        // try {
        self . file = self . sock . makefile ( "rwb" );
        self . _base_init ( readermode );
        if user || usenetrc {
        self . login ( user , password , usenetrc );
        // } catch   {
        if self . file {
        self . file . close ( );
        self . sock . close ( );
        panic!("");
        pub fn _base_init ( &self, readermode )  {
        "Partial initialization for the NNTP protocol.
        This instance method == extracted for supporting the test code.
        ";
        self . debugging = 0;
        self . welcome = self . _getresp ( );
        self . _caps = None /* Option */;
        self . getcapabilities ( );
        self . readermode_afterauth = false;
        if readermode && "READER" !in self . _caps {
        self . _setreadermode ( );
        if !self . readermode_afterauth {
        self . _caps = None /* Option */;
        self . getcapabilities ( );
        self . tls_on = false;
        self . authenticated = false;
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, * args )  {
        is_connected = || {  hasattr ( self , "file" ) };
        if is_connected ( ) {
        // try {
        self . quit ( );
        // } catch  ( OSError , EOFError )  {
        // pass
        // } finally {
        if is_connected ( ) {
        self . _close ( );
        pub fn _create_socket ( &self, timeout )  {
        if timeout is !None /* Option */ && !timeout {
        panic!("ValueError ( "Non-blocking socket (timeout=0) is !supported" )");
        sys . audit ( "nntplib.connect" , self , self . host , self . port );
        return  socket . create_connection ( ( self . host , self . port ) , timeout );
        pub fn getwelcome ( self )  {
        "Get the welcome message from the server
        (this == read && squirreled away by __init__()).
        If the response code == 200, posting == allowed;
        if it 201, posting == !allowed.";
        if self . debugging { : print ( "*welcome*" , repr ( self . welcome ) ); }
        return  self . welcome;
        pub fn getcapabilities ( self )  {
        "Get the server capabilities, as read by __init__().
        If the CAPABILITIES command == !supported, an empty dict is
        returned.";
        if self . _caps is None /* Option */ {
        self . nntp_version = 1;
        self . nntp_implementation = None /* Option */;
        // try {
        resp , caps = self . capabilities ( );
        // } catch  ( NNTPPermanentError , NNTPTemporaryError )  {
        self . _caps = { };
        } else {
        self . _caps = caps;
        if "VERSION" in caps {
        self . nntp_version = max ( map ( int , caps [ "VERSION" ] ) );
        if "IMPLEMENTATION" in caps {
        self . nntp_implementation = " " . join ( caps [ "IMPLEMENTATION" ] );
        return  self . _caps;
        pub fn set_debuglevel ( &self, level )  {
        "Set the debugging level.  Argument 'level' means:
        0: no debugging output (default)
        1: print commands && responses but !body text etc.
        2: also print raw lines read && sent before stripping CR/LF";
        self . debugging = level;
        debug = set_debuglevel;
        pub fn _putline ( &self, line )  {
        "Internal: send one line to the server, appending CRLF.
        The `line` must be a bytes-like object.";
        sys . audit ( "nntplib.putline" , self , line );
        line = line + _CRLF;
        if self . debugging > 1 { : print ( "*put*" , repr ( line ) ); }
        self . file . write ( line );
        self . file . flush ( );
        pub fn _putcmd ( &self, line )  {
        "Internal: send one command to the server (through _putline()).
        The `line` must be a unicode string.";
        if self . debugging { : print ( "*cmd*" , repr ( line ) ); }
        line = line . encode ( self . encoding , self . errors );
        self . _putline ( line );
        pub fn _getline ( &self, strip_crlf = true )  {
        "Internal: return one line from the server, stripping _CRLF.
        Raise EOFError if the connection == closed.
        Returns a bytes object.";
        line = self . file . readline ( _MAXLINE + 1 );
        if len ( line ) > _MAXLINE {
        panic!("NNTPDataError ( "line too long" )");
        if self . debugging > 1 {
        println!( "*get*" , repr ( line ) );
        if !line { : raise EOFError; }
        if strip_crlf {
        if line [ -2 { : ] == _CRLF ; }
        line = line [ : -2 ];
        } else if line [ -1 {
        line = line [ : -1 ];
        return  line;
        pub fn _getresp ( self )  {
        "Internal: get a response from the server.
        Raise various errors if the response indicates an error.
        Returns a unicode string.";
        resp = self . _getline ( );
        if self . debugging { : print ( "*resp*" , repr ( resp ) ); }
        resp = resp . decode ( self . encoding , self . errors );
        c = resp [ : 1 ];
        if c == "4" {
        panic!("NNTPTemporaryError ( resp )");
        if c == "5" {
        panic!("NNTPPermanentError ( resp )");
        if c !in "123" {
        panic!("NNTPProtocolError ( resp )");
        return  resp;
        pub fn _getlongresp ( &self, file = None /* Option */ )  {
        "Internal: get a response plus following text from the server.
        Raise various errors if the response indicates an error.

        Returns a (response, lines) tuple where `response` == a unicode
        string && `lines` == a list of bytes objects.
        If `file` == a file-like object, it must be open in binary mode.
        ";
        openedFile = None /* Option */;
        // try {
        if isinstance ( file , ( str , bytes ) ) {
        openedFile = file = open ( file , "wb" );
        resp = self . _getresp ( );
        if resp [ { : 3 ] !in _LONGRESP ; }
        panic!("NNTPReplyError ( resp )");
        lines = [ ];
        if file is !None /* Option */ {
        terminators = ( b "." + _CRLF , b ".\n" );
        while 1  {
        line = self . _getline ( false );
        if line in terminators {
        break;
        if line . startswith ( b ".." ) {
        line = line [ 1 : ];
        file . write ( line );
        } else {
        terminator = b ".";
        while 1  {
        line = self . _getline ( );
        if line == terminator {
        break;
        if line . startswith ( b ".." ) {
        line = line [ 1 : ];
        lines . append ( line );
        // } finally {
        if openedFile {
        openedFile . close ( );
        return  resp , lines;
        pub fn _shortcmd ( &self, line )  {
        "Internal: send a command && get the response.
        Same return value as _getresp().";
        self . _putcmd ( line );
        return  self . _getresp ( );
        pub fn _longcmd ( &self, line , file = None /* Option */ )  {
        "Internal: send a command && get the response plus following text.
        Same return value as _getlongresp().";
        self . _putcmd ( line );
        return  self . _getlongresp ( file );
        pub fn _longcmdstring ( &self, line , file = None /* Option */ )  {
        "Internal: send a command && get the response plus following text.
        Same as _longcmd() && _getlongresp(), except that the returned `lines`
        are unicode strings rather than bytes objects.
        ";
        self . _putcmd ( line );
        resp , list = self . _getlongresp ( file );
        return  resp , [ line . decode ( self . encoding , self . errors );
        for line in list ].iter() {
        pub fn _getoverviewfmt ( self )  {
        "Internal: get the overview format. Queries the server if not
        already done, else returns the cached value.";
        // try {
        return  self . _cachedoverviewfmt;
        // } catch  AttributeError  {
        // pass
        // try {
        resp , lines = self . _longcmdstring ( "LIST OVERVIEW.FMT" );
        // } catch  NNTPPermanentError  {
        fmt = _DEFAULT_OVERVIEW_FMT [ : ];
        } else {
        fmt = _parse_overview_fmt ( lines );
        self . _cachedoverviewfmt = fmt;
        return  fmt;
        pub fn _grouplist ( &self, lines )  {
        return  [ GroupInfo ( * line . split ( ) ) for line in lines ];
        pub fn capabilities ( self )  {
        "Process a CAPABILITIES command.  Not supported by all servers.
        Return:
        - resp: server response if successful
        - caps: a dictionary mapping capability names to lists of tokens
        (for example {'VERSION': ['2'], 'OVER': [], LIST: ['ACTIVE', 'HEADERS'] })
        ";
        caps = { };
        resp , lines = self . _longcmdstring ( "CAPABILITIES" );
        for line in lines .iter() {
        name , * tokens = line . split ( );
        caps [ name ] = tokens;
        return  resp , caps;
        pub fn newgroups ( &self, date , * , file = None /* Option */ )  {
        "Process a NEWGROUPS command.  Arguments:
        - date: a date || datetime object
        Return:
        - resp: server response if successful
        - list: list of newsgroup names
        ";
        if !isinstance ( date , ( datetime . date , datetime . date ) ) {
        panic!("TypeError (");
        "the date parameter must be a date || datetime object, ";
        "not '{:40}'" . format ( date . __class__ . __name__ ) );
        date_str , time_str = _unparse_datetime ( date , self . nntp_version < 2 );
        cmd = "NEWGROUPS {0} {1}" . format ( date_str , time_str );
        resp , lines = self . _longcmdstring ( cmd , file );
        return  resp , self . _grouplist ( lines );
        pub fn newnews ( &self, group , date , * , file = None /* Option */ )  {
        "Process a NEWNEWS command.  Arguments:
        - group: group name || '*'
        - date: a date || datetime object
        Return:
        - resp: server response if successful
        - list: list of message ids
        ";
        if !isinstance ( date , ( datetime . date , datetime . date ) ) {
        panic!("TypeError (");
        "the date parameter must be a date || datetime object, ";
        "not '{:40}'" . format ( date . __class__ . __name__ ) );
        date_str , time_str = _unparse_datetime ( date , self . nntp_version < 2 );
        cmd = "NEWNEWS {0} {1} {2}" . format ( group , date_str , time_str );
        return  self . _longcmdstring ( cmd , file );
        pub fn list ( &self, group_pattern = None /* Option */ , * , file = None /* Option */ )  {
        "Process a LIST || LIST ACTIVE command. Arguments:
        - group_pattern: a pattern indicating which groups to query
        - file: Filename string || file object to store the result in
        Returns:
        - resp: server response if successful
        - list: list of (group, last, first, flag) (strings)
        ";
        if group_pattern is !None /* Option */ {
        command = "LIST ACTIVE " + group_pattern;
        } else {
        command = "LIST";
        resp , lines = self . _longcmdstring ( command , file );
        return  resp , self . _grouplist ( lines );
        pub fn _getdescriptions ( &self, group_pattern , return_all )  {
        line_pat = re . compile ( "^(?P<group>[^ \t]+)[ \t]+(.*)$" );
        resp , lines = self . _longcmdstring ( "LIST NEWSGROUPS " + group_pattern );
        if !resp . startswith ( "215" ) {
        resp , lines = self . _longcmdstring ( "XGTITLE " + group_pattern );
        groups = { };
        for raw_line in lines .iter() {
        match = line_pat . search ( raw_line . strip ( ) );
        if match {
        name , desc = match . group ( 1 , 2 );
        if !return_all {
        return  desc;
        groups [ name ] = desc;
        if return_all {
        return  resp , groups;
        } else {
        return  "";
        pub fn description ( &self, group )  {
        "Get a description for a single group.  If more than one
        group matches ('group' == a pattern), return the first.  If no
        group matches, return an empty string.

        This elides the response code from the server, since it can
        only be '215' || '285' (for xgtitle) anyway.  If the response
        code == needed, use the 'descriptions' method.

        NOTE: This neither checks for a wildcard in 'group' nor does
        it check whether the group actually exists.";
        return  self . _getdescriptions ( group , false );
        pub fn descriptions ( &self, group_pattern )  {
        "Get descriptions for a range of groups.";
        return  self . _getdescriptions ( group_pattern , true );
        pub fn group ( &self, name )  {
        "Process a GROUP command.  Argument:
        - group: the group name
        Returns:
        - resp: server response if successful
        - count: number of articles
        - first: first article number
        - last: last article number
        - name: the group name
        ";
        resp = self . _shortcmd ( "GROUP " + name );
        if !resp . startswith ( "211" ) {
        panic!("NNTPReplyError ( resp )");
        words = resp . split ( );
        count = first = last = 0;
        n = len ( words );
        if n > 1 {
        count = words [ 1 ];
        if n > 2 {
        first = words [ 2 ];
        if n > 3 {
        last = words [ 3 ];
        if n > 4 {
        name = words [ 4 ] . lower ( );
        return  resp , int ( count ) , int ( first ) , int ( last ) , name;
        pub fn help ( &self, * , file = None /* Option */ )  {
        "Process a HELP command. Argument:
        - file: Filename string || file object to store the result in
        Returns:
        - resp: server response if successful
        - list: list of strings returned by the server in response to the
                HELP command
        ";
        return  self . _longcmdstring ( "HELP" , file );
        pub fn _statparse ( &self, resp )  {
        "Internal: parse the response line of a STAT, NEXT, LAST,
        ARTICLE, HEAD || BODY command.";
        if !resp . startswith ( "22" ) {
        panic!("NNTPReplyError ( resp )");
        words = resp . split ( );
        art_num = int ( words [ 1 ] );
        message_id = words [ 2 ];
        return  resp , art_num , message_id;
        pub fn _statcmd ( &self, line )  {
        "Internal: process a STAT, NEXT || LAST command.";
        resp = self . _shortcmd ( line );
        return  self . _statparse ( resp );
        pub fn stat ( &self, message_spec = None /* Option */ )  {
        "Process a STAT command.  Argument:
        - message_spec: article number || message id (if !specified,
          the current article == selected)
        Returns:
        - resp: server response if successful
        - art_num: the article number
        - message_id: the message id
        ";
        if message_spec {
        return  self . _statcmd ( "STAT {0}" . format ( message_spec ) );
        } else {
        return  self . _statcmd ( "STAT" );
        pub fn next ( self )  {
        "Process a NEXT command.  No arguments.  Return as for STAT.";
        return  self . _statcmd ( "NEXT" );
        pub fn last ( self )  {
        "Process a LAST command.  No arguments.  Return as for STAT.";
        return  self . _statcmd ( "LAST" );
        pub fn _artcmd ( &self, line , file = None /* Option */ )  {
        "Internal: process a HEAD, BODY || ARTICLE command.";
        resp , lines = self . _longcmd ( line , file );
        resp , art_num , message_id = self . _statparse ( resp );
        return  resp , ArticleInfo ( art_num , message_id , lines );
        pub fn head ( &self, message_spec = None /* Option */ , * , file = None /* Option */ )  {
        "Process a HEAD command.  Argument:
        - message_spec: article number || message id
        - file: filename string || file object to store the headers in
        Returns:
        - resp: server response if successful
        - ArticleInfo: (article number, message id, list of header lines)
        ";
        if message_spec is !None /* Option */ {
        cmd = "HEAD {0}" . format ( message_spec );
        } else {
        cmd = "HEAD";
        return  self . _artcmd ( cmd , file );
        pub fn body ( &self, message_spec = None /* Option */ , * , file = None /* Option */ )  {
        "Process a BODY command.  Argument:
        - message_spec: article number || message id
        - file: filename string || file object to store the body in
        Returns:
        - resp: server response if successful
        - ArticleInfo: (article number, message id, list of body lines)
        ";
        if message_spec is !None /* Option */ {
        cmd = "BODY {0}" . format ( message_spec );
        } else {
        cmd = "BODY";
        return  self . _artcmd ( cmd , file );
        pub fn article ( &self, message_spec = None /* Option */ , * , file = None /* Option */ )  {
        "Process an ARTICLE command.  Argument:
        - message_spec: article number || message id
        - file: filename string || file object to store the article in
        Returns:
        - resp: server response if successful
        - ArticleInfo: (article number, message id, list of article lines)
        ";
        if message_spec is !None /* Option */ {
        cmd = "ARTICLE {0}" . format ( message_spec );
        } else {
        cmd = "ARTICLE";
        return  self . _artcmd ( cmd , file );
        pub fn slave ( self )  {
        "Process a SLAVE command.  Returns:
        - resp: server response if successful
        ";
        return  self . _shortcmd ( "SLAVE" );
        pub fn xhdr ( &self, hdr , str , * , file = None /* Option */ )  {
        "Process an XHDR command (optional server extension).  Arguments:
        - hdr: the header type (e.g. 'subject')
        - str: an article nr, a message id, || a range nr1-nr2
        - file: Filename string || file object to store the result in
        Returns:
        - resp: server response if successful
        - list: list of (nr, value) strings
        ";
        pat = re . compile ( "^([0-9]+) ?(.*)\n?" );
        resp , lines = self . _longcmdstring ( "XHDR {0} {1}" . format ( hdr , str ) , file );
        pub fn remove_number ( line )  {
        m = pat . match ( line );
        return  m . group ( 1 , 2 ) if m else line;
        return  resp , [ remove_number ( line ) for line in lines ];
        pub fn xover ( &self, start , end , * , file = None /* Option */ )  {
        "Process an XOVER command (optional server extension) Arguments:
        - start: start of range
        - end: end of range
        - file: Filename string || file object to store the result in
        Returns:
        - resp: server response if successful
        - list: list of dicts containing the response fields
        ";
        resp , lines = self . _longcmdstring ( "XOVER {0}-{1}" . format ( start , end ) ,;
        file );
        fmt = self . _getoverviewfmt ( );
        return  resp , _parse_overview ( lines , fmt );
        pub fn over ( &self, message_spec , * , file = None /* Option */ )  {
        "Process an OVER command.  If the command isn't supported, fall
        back to XOVER. Arguments:
        - message_spec:
            - either a message id, indicating the article to fetch
              information about
            - || a (start, end) tuple, indicating a range of article numbers;
              if end == None /* Option */, information up to the newest message will be
              retrieved
            - || None /* Option */, indicating the current article number must be used
        - file: Filename string || file object to store the result in
        Returns:
        - resp: server response if successful
        - list: list of dicts containing the response fields

        NOTE: the "message id" form isn't supported by XOVER
        ";
        cmd = "OVER" iformat!("OVER" in self . _caps else "XOVER");
        if isinstance ( message_spec , ( tuple , list ) ) {
        start , end = message_spec;
        cmd + = " {0}-{1}" . format ( start , end || "" );
        } else if message_spec is !None /* Option */ {
        cmd = cmd + " " + message_spec;
        resp , lines = self . _longcmdstring ( cmd , file );
        fmt = self . _getoverviewfmt ( );
        return  resp , _parse_overview ( lines , fmt );
        pub fn date ( self )  {
        "Process the DATE command.
        Returns:
        - resp: server response if successful
        - date: datetime object
        ";
        resp = self . _shortcmd ( "DATE" );
        if !resp . startswith ( "111" ) {
        panic!("NNTPReplyError ( resp )");
        elem = resp . split ( );
        if len ( elem ) != 2 {
        panic!("NNTPDataError ( resp )");
        date = elem [ 1 ];
        if len ( date ) != 14 {
        panic!("NNTPDataError ( resp )");
        return  resp , _parse_datetime ( date , None /* Option */ );
        pub fn _post ( &self, command , f )  {
        resp = self . _shortcmd ( command );
        if !resp . startswith ( "3" ) {
        panic!("NNTPReplyError ( resp )");
        if isinstance ( f , ( bytes , bytearray ) ) {
        f = f . splitlines ( );
        for line in f .iter() {
        if !line . endswith ( _CRLF ) {
        line = line . rstrip ( b "\r\n" ) + _CRLF;
        if line . startswith ( b "." ) {
        line = b "." + line;
        self . file . write ( line );
        self . file . write ( b ".\r\n" );
        self . file . flush ( );
        return  self . _getresp ( );
        pub fn post ( &self, data )  {
        "Process a POST command.  Arguments:
        - data: bytes object, iterable || file containing the article
        Returns:
        - resp: server response if successful";
        return  self . _post ( "POST" , data );
        pub fn ihave ( &self, message_id , data )  {
        "Process an IHAVE command.  Arguments:
        - message_id: message-id of the article
        - data: file containing the article
        Returns:
        - resp: server response if successful
        Note that if the server refuses the article an exception == raised.";
        return  self . _post ( "IHAVE {0}" . format ( message_id ) , data );
        pub fn _close ( self )  {
        // try {
        if self . file {
        self . file . close ( );
        del self . file;
        // } finally {
        self . sock . close ( );
        pub fn quit ( self )  {
        "Process a QUIT command && close the socket.  Returns:
        - resp: server response if successful";
        // try {
        resp = self . _shortcmd ( "QUIT" );
        // } finally {
        self . _close ( );
        return  resp;
        pub fn login ( &self, user = None /* Option */ , password = None /* Option */ , usenetrc = true )  {
        if self . authenticated {
        panic!("ValueError ( "Already logged in." )");
        if !user && !usenetrc {
        panic!("ValueError (");
        "At least one of `user` && `usenetrc` must be specified" );
        // try {
        if usenetrc && !user {
        import netrc;
        credentials = netrc . netrc ( );
        auth = credentials . authenticators ( self . host );
        if auth {
        user = auth [ 0 ];
        password = auth [ 2 ];
        // } catch  OSError  {
        // pass
        if !user {
        return;
        resp = self . _shortcmd ( "authinfo user " + user );
        if resp . startswith ( "381" ) {
        if !password {
        panic!("NNTPReplyError ( resp )");
        } else {
        resp = self . _shortcmd ( "authinfo pass " + password );
        if !resp . startswith ( "281" ) {
        panic!("NNTPPermanentError ( resp )");
        self . _caps = None /* Option */;
        self . getcapabilities ( );
        if self . readermode_afterauth && "READER" !in self . _caps {
        self . _setreadermode ( );
        self . _caps = None /* Option */;
        self . getcapabilities ( );
        pub fn _setreadermode ( self )  {
        // try {
        self . welcome = self . _shortcmd ( "mode reader" );
        // } catch  NNTPPermanentError  {
        // pass
        // } catch  NNTPTemporaryError as e  {
        if e . response . startswith ( "480" ) {
        self . readermode_afterauth = true;
        } else {
        panic!("");
        if _have_ssl {
        pub fn starttls ( &self, context = None /* Option */ )  {
        "Process a STARTTLS command. Arguments:
            - context: SSL context to use for the encrypted connection
            ";
        if self . tls_on {
        panic!("ValueError ( "TLS is already enabled." )");
        if self . authenticated {
        panic!("ValueError ( "TLS cannot be started after authentication." )");
        resp = self . _shortcmd ( "STARTTLS" );
        if resp . startswith ( "382" ) {
        self . file . close ( );
        self . sock = _encrypt_on ( self . sock , context , self . host );
        self . file = self . sock . makefile ( "rwb" );
        self . tls_on = true;
        self . _caps = None /* Option */;
        self . getcapabilities ( );
        } else {
        panic!("NNTPError ( "TLS failed to start." )");
        if _have_ssl {
        class NNTP_SSL ( NNTP ) ;
        pub fn __init__ ( &self, host , port = NNTP_SSL_PORT , {
        user = None /* Option */ , password = None /* Option */ , ssl_context = None /* Option */ ,;
        readermode = None /* Option */ , usenetrc = false ,;
        timeout = _GLOBAL_DEFAULT_TIMEOUT ) ;
        "This works identically to NNTP.__init__, except for the change
            in default port && the `ssl_context` argument for SSL connections.
            ";
        self . ssl_context = ssl_context;
        super ( ) . __init__ ( host , port , user , password , readermode ,;
        usenetrc , timeout );
        pub fn _create_socket ( &self, timeout )  {
        sock = super ( ) . _create_socket ( timeout );
        // try {
        sock = _encrypt_on ( sock , self . ssl_context , self . host );
        // } catch   {
        sock . close ( );
        panic!("");
        } else {
        return  sock;
        __all__ . append ( "NNTP_SSL" );
        fn main() {
        import argparse;
        parser = argparse . ArgumentParser ( description = "\
        nntplib built-in demo - display the latest articles in a newsgroup" );
        parser . add_argument ( "-g" , "--group" , default = "gmane.comp.python.general" ,;
        help = "group to fetch messages from (default: %(default)s)" );
        parser . add_argument ( "-s" , "--server" , default = "news.gmane.io" ,;
        help = "NNTP server hostname (default: %(default)s)" );
        parser . add_argument ( "-p" , "--port" , default = -1 , type = int ,;
        help = "NNTP port number (default: %s / %s)" % ( NNTP_PORT , NNTP_SSL_PORT ) );
        parser . add_argument ( "-n" , "--nb-articles" , default = 10 , type = int ,;
        help = "number of articles to fetch (default: %(default)s)" );
        parser . add_argument ( "-S" , "--ssl" , action = "store_true" , default = false ,;
        help = "use NNTP over SSL" );
        args = parser . parse_args ( );
        port = args . port;
        if !args . ssl {
        if port == -1 {
        port = NNTP_PORT;
        s = NNTP ( host = args . server , port = port );
        } else {
        if port == -1 {
        port = NNTP_SSL_PORT;
        s = NNTP_SSL ( host = args . server , port = port );
        caps = s . getcapabilities ( );
        if "STARTTLS" in caps {
        s . starttls ( );
        resp , count , first , last , name = s . group ( args . group );
        println!( "Group" , name , "has" , count , "articles, range" , first , "to" , last );
        pub fn cut ( s , lim )  {
        if len ( s ) > lim {
        s = s [ : lim - 4 ] + "...";
        return  s;
        first = str ( int ( last ) - args . nb_articles + 1 );
        resp , overviews = s . xover ( first , last );
        for artnum , over in overviews .iter() {
        author = decode_header ( over [ "from" ] ) . split ( "<" , 1 ) [ 0 ];
        subject = decode_header ( over [ "subject" ] );
        lines = int ( over [ ":lines" ] );
        println!( "{:7} {:20} {:42} ({})" . format );
        artnum , cut ( author , 20 ) , cut ( subject , 42 ) , lines );
        );
        s . quit ( );
    }

}

