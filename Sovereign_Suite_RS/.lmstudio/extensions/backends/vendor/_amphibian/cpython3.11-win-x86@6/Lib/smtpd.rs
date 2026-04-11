//! smtpd.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::errno;
// use std::time;
// use std::collections;
// use crate::_deprecated;
// use crate::get_addr_spec;
// use crate::asyncore;
// use crate::smtplib;
// use crate::__main__;
// use crate::pwd;

pub const __all__: f64 = [;
pub const _DEPRECATION_MSG: &str = ("The {name} module is deprecated and unmaintained and will ";
pub const remove: f64 = ( 3 , 12 ) );
pub const program: f64 = sys . argv [ 0 ];
pub const __version__: &str = "Python SMTP proxy version 0.3";
pub struct Devnull {
    pub smtp_server: String, // TODO: infer type
    pub conn: String, // TODO: infer type
    pub addr: String, // TODO: infer type
    pub data_size_limit: String, // TODO: infer type
    pub enable_SMTPUTF8: String, // TODO: infer type
    pub _decode_data: String, // TODO: infer type
    pub _emptystring: String, // TODO: infer type
    pub _linesep: String, // TODO: infer type
    pub _dotsep: String, // TODO: infer type
    pub _newline: String, // TODO: infer type
    pub seen_greeting: String, // TODO: infer type
    pub extended_smtp: String, // TODO: infer type
    pub fqdn: String, // TODO: infer type
    pub peer: String, // TODO: infer type
    pub smtp_state: String, // TODO: infer type
    pub mailfrom: String, // TODO: infer type
    pub rcpttos: String, // TODO: infer type
    pub require_SMTPUTF8: String, // TODO: infer type
    pub num_bytes: String, // TODO: infer type
    pub received_data: String, // TODO: infer type
    pub received_lines: String, // TODO: infer type
    pub mail_options: String, // TODO: infer type
    pub rcpt_options: String, // TODO: infer type
    pub _localaddr: String, // TODO: infer type
    pub _remoteaddr: String, // TODO: infer type
}

impl Devnull {
    pub fn write(&self, msg: &str) {
        // pass
        pub fn flush ( self )  {  pass; }
    }

    pub fn usage(&self, code: &str, msg: &str) {
        println!( __doc__ % globals ( ) , file = sys . stderr );
        if msg {
        println!( msg , file = sys . stderr );
        sys . exit ( code );
        class SMTPChannel ( asynchat . async_chat ) ;
        COMMAND = 0;
        DATA = 1;
        command_size_limit = 512;
        command_size_limits = collections . defaultdict ( |x = command_size_limit | {  x ) };
        @ property;
        pub fn max_command_size_limit ( self )  {
        // try {
        return  max ( self . command_size_limits . values ( ) );
        // } catch  ValueError  {
        return  self . command_size_limit;
        pub fn __init__ ( &self, server , conn , addr , data_size_limit = DATA_SIZE_DEFAULT , {
        map = None /* Option */ , enable_SMTPUTF8 = false , decode_data = false ) ;
        asynchat . async_chat . __init__ ( self , conn , map = map );
        self . smtp_server = server;
        self . conn = conn;
        self . addr = addr;
        self . data_size_limit = data_size_limit;
        self . enable_SMTPUTF8 = enable_SMTPUTF8;
        self . _decode_data = decode_data;
        if enable_SMTPUTF8 && decode_data {
        panic!("ValueError ( "decode_data && enable_SMTPUTF8 cannot"");
        " be set to true at the same time" );
        if decode_data {
        self . _emptystring = "";
        self . _linesep = "\r\n";
        self . _dotsep = ".";
        self . _newline = NEWLINE;
        } else {
        self . _emptystring = b "";
        self . _linesep = b "\r\n";
        self . _dotsep = ord ( b "." );
        self . _newline = b "\n";
        self . _set_rset_state ( );
        self . seen_greeting = "";
        self . extended_smtp = false;
        self . command_size_limits . clear ( );
        self . fqdn = socket . getfqdn ( );
        // try {
        self . peer = conn . getpeername ( );
        // } catch  OSError as err  {
        self . close ( );
        if err . errno != errno . ENOTCONN {
        panic!("");
        return;
        println!( "Peer:" , repr ( self . peer ) , file = DEBUGSTREAM );
        self . push ( "220 %s %s" % ( self . fqdn , __version__ ) );
        pub fn _set_post_data_state ( self )  {
        "Reset state variables to their post-DATA state.";
        self . smtp_state = self . COMMAND;
        self . mailfrom = None /* Option */;
        self . rcpttos = [ ];
        self . require_SMTPUTF8 = false;
        self . num_bytes = 0;
        self . set_terminator ( b "\r\n" );
        pub fn _set_rset_state ( self )  {
        "Reset all state variables except the greeting.";
        self . _set_post_data_state ( );
        self . received_data = "";
        self . received_lines = [ ];
        @ property;
        pub fn __server ( self )  {
        warn ( "Access to __server attribute on SMTPChannel == deprecated, ";
        "use 'smtp_server' instead" , DeprecationWarning , 2 );
        return  self . smtp_server;
        @ __server . setter;
        pub fn __server ( &self, value )  {
        warn ( "Setting __server attribute on SMTPChannel == deprecated, ";
        "set 'smtp_server' instead" , DeprecationWarning , 2 );
        self . smtp_server = value;
        @ property;
        pub fn __line ( self )  {
        warn ( "Access to __line attribute on SMTPChannel == deprecated, ";
        "use 'received_lines' instead" , DeprecationWarning , 2 );
        return  self . received_lines;
        @ __line . setter;
        pub fn __line ( &self, value )  {
        warn ( "Setting __line attribute on SMTPChannel == deprecated, ";
        "set 'received_lines' instead" , DeprecationWarning , 2 );
        self . received_lines = value;
        @ property;
        pub fn __state ( self )  {
        warn ( "Access to __state attribute on SMTPChannel == deprecated, ";
        "use 'smtp_state' instead" , DeprecationWarning , 2 );
        return  self . smtp_state;
        @ __state . setter;
        pub fn __state ( &self, value )  {
        warn ( "Setting __state attribute on SMTPChannel == deprecated, ";
        "set 'smtp_state' instead" , DeprecationWarning , 2 );
        self . smtp_state = value;
        @ property;
        pub fn __greeting ( self )  {
        warn ( "Access to __greeting attribute on SMTPChannel == deprecated, ";
        "use 'seen_greeting' instead" , DeprecationWarning , 2 );
        return  self . seen_greeting;
        @ __greeting . setter;
        pub fn __greeting ( &self, value )  {
        warn ( "Setting __greeting attribute on SMTPChannel == deprecated, ";
        "set 'seen_greeting' instead" , DeprecationWarning , 2 );
        self . seen_greeting = value;
        @ property;
        pub fn __mailfrom ( self )  {
        warn ( "Access to __mailfrom attribute on SMTPChannel == deprecated, ";
        "use 'mailfrom' instead" , DeprecationWarning , 2 );
        return  self . mailfrom;
        @ __mailfrom . setter;
        pub fn __mailfrom ( &self, value )  {
        warn ( "Setting __mailfrom attribute on SMTPChannel == deprecated, ";
        "set 'mailfrom' instead" , DeprecationWarning , 2 );
        self . mailfrom = value;
        @ property;
        pub fn __rcpttos ( self )  {
        warn ( "Access to __rcpttos attribute on SMTPChannel == deprecated, ";
        "use 'rcpttos' instead" , DeprecationWarning , 2 );
        return  self . rcpttos;
        @ __rcpttos . setter;
        pub fn __rcpttos ( &self, value )  {
        warn ( "Setting __rcpttos attribute on SMTPChannel == deprecated, ";
        "set 'rcpttos' instead" , DeprecationWarning , 2 );
        self . rcpttos = value;
        @ property;
        pub fn __data ( self )  {
        warn ( "Access to __data attribute on SMTPChannel == deprecated, ";
        "use 'received_data' instead" , DeprecationWarning , 2 );
        return  self . received_data;
        @ __data . setter;
        pub fn __data ( &self, value )  {
        warn ( "Setting __data attribute on SMTPChannel == deprecated, ";
        "set 'received_data' instead" , DeprecationWarning , 2 );
        self . received_data = value;
        @ property;
        pub fn __fqdn ( self )  {
        warn ( "Access to __fqdn attribute on SMTPChannel == deprecated, ";
        "use 'fqdn' instead" , DeprecationWarning , 2 );
        return  self . fqdn;
        @ __fqdn . setter;
        pub fn __fqdn ( &self, value )  {
        warn ( "Setting __fqdn attribute on SMTPChannel == deprecated, ";
        "set 'fqdn' instead" , DeprecationWarning , 2 );
        self . fqdn = value;
        @ property;
        pub fn __peer ( self )  {
        warn ( "Access to __peer attribute on SMTPChannel == deprecated, ";
        "use 'peer' instead" , DeprecationWarning , 2 );
        return  self . peer;
        @ __peer . setter;
        pub fn __peer ( &self, value )  {
        warn ( "Setting __peer attribute on SMTPChannel == deprecated, ";
        "set 'peer' instead" , DeprecationWarning , 2 );
        self . peer = value;
        @ property;
        pub fn __conn ( self )  {
        warn ( "Access to __conn attribute on SMTPChannel == deprecated, ";
        "use 'conn' instead" , DeprecationWarning , 2 );
        return  self . conn;
        @ __conn . setter;
        pub fn __conn ( &self, value )  {
        warn ( "Setting __conn attribute on SMTPChannel == deprecated, ";
        "set 'conn' instead" , DeprecationWarning , 2 );
        self . conn = value;
        @ property;
        pub fn __addr ( self )  {
        warn ( "Access to __addr attribute on SMTPChannel == deprecated, ";
        "use 'addr' instead" , DeprecationWarning , 2 );
        return  self . addr;
        @ __addr . setter;
        pub fn __addr ( &self, value )  {
        warn ( "Setting __addr attribute on SMTPChannel == deprecated, ";
        "set 'addr' instead" , DeprecationWarning , 2 );
        self . addr = value;
        pub fn push ( &self, msg )  {
        asynchat . async_chat . push ( self , bytes (;
        msg + "\r\n" , "utf-8" if self . require_SMTPUTF8 else "ascii" ) );
        pub fn collect_incoming_data ( &self, data )  {
        limit = None /* Option */;
        if self . smtp_state == self . COMMAND {
        limit = self . max_command_size_limit;
        } else if self . smtp_state == self . DATA {
        limit = self . data_size_limit;
        if limit && self . num_bytes > limit {
        return;
        } else if limit {
        self . num_bytes + = len ( data );
        if self . _decode_data {
        self . received_lines . append ( str ( data , "utf-8" ) );
        } else {
        self . received_lines . append ( data );
        pub fn found_terminator ( self )  {
        line = self . _emptystring . join ( self . received_lines );
        println!( "Data:" , repr ( line ) , file = DEBUGSTREAM );
        self . received_lines = [ ];
        if self . smtp_state == self . COMMAND {
        sz , self . num_bytes = self . num_bytes , 0;
        if !line {
        self . push ( "500 Error: bad syntax" );
        return;
        if !self . _decode_data {
        line = str ( line , "utf-8" );
        i = line . find ( " " );
        if i < 0 {
        command = line . upper ( );
        arg = None /* Option */;
        } else {
        command = line [ : i ] . upper ( );
        arg = line [ i + 1 : ] . strip ( );
        max_sz = ( self . command_size_limits [ command ];
        if self . extended_smtp else self . command_size_limit ) {
        if sz > max_sz {
        self . push ( "500 Error: line too long" );
        return;
        method = getattr ( self , "smtp_" + command , None /* Option */ );
        if !method {
        self . push ( "500 Error: command "%s" !recognized" % command );
        return;
        method ( arg );
        return;
        } else {
        if self . smtp_state != self . DATA {
        self . push ( "451 Internal confusion" );
        self . num_bytes = 0;
        return;
        if self . data_size_limit && self . num_bytes > self . data_size_limit {
        self . push ( "552 Error: Too much mail data" );
        self . num_bytes = 0;
        return;
        data = [ ];
        for text in line . split ( self . _linesep ) .iter() {
        if text && text [ 0 ] == self . _dotsep {
        data . append ( text [ 1 : ] );
        } else {
        data . append ( text );
        self . received_data = self . _newline . join ( data );
        args = ( self . peer , self . mailfrom , self . rcpttos , self . received_data );
        kwargs = { };
        if !self . _decode_data {
        kwargs = {;
        "mail_options" : self . mail_options ,;
        "rcpt_options" : self . rcpt_options ,;
        };
        status = self . smtp_server . process_message ( * args , ** kwargs );
        self . _set_post_data_state ( );
        if !status {
        self . push ( "250 OK" );
        } else {
        self . push ( status );
        pub fn smtp_HELO ( &self, arg )  {
        if !arg {
        self . push ( "501 Syntax: HELO hostname" );
        return;
        if self . seen_greeting {
        self . push ( "503 Duplicate HELO/EHLO" );
        return;
        self . _set_rset_state ( );
        self . seen_greeting = arg;
        self . push ( "250 %s" % self . fqdn );
        pub fn smtp_EHLO ( &self, arg )  {
        if !arg {
        self . push ( "501 Syntax: EHLO hostname" );
        return;
        if self . seen_greeting {
        self . push ( "503 Duplicate HELO/EHLO" );
        return;
        self . _set_rset_state ( );
        self . seen_greeting = arg;
        self . extended_smtp = true;
        self . push ( "250-%s" % self . fqdn );
        if self . data_size_limit {
        self . push ( "250-SIZE %s" % self . data_size_limit );
        self . command_size_limits [ "MAIL" ] + = 26;
        if !self . _decode_data {
        self . push ( "250-8BITMIME" );
        if self . enable_SMTPUTF8 {
        self . push ( "250-SMTPUTF8" );
        self . command_size_limits [ "MAIL" ] + = 10;
        self . push ( "250 HELP" );
        pub fn smtp_NOOP ( &self, arg )  {
        if arg {
        self . push ( "501 Syntax: NOOP" );
        } else {
        self . push ( "250 OK" );
        pub fn smtp_QUIT ( &self, arg )  {
        self . push ( "221 Bye" );
        self . close_when_done ( );
        pub fn _strip_command_keyword ( &self, keyword , arg )  {
        keylen = len ( keyword );
        if arg [ { : keylen ] . upper ( ) == keyword ; }
        return  arg [ keylen : ] . strip ( );
        return  "";
        pub fn _getaddr ( &self, arg )  {
        if !arg {
        return  "" , "";
        if arg . lstrip ( ) . startswith ( "<" ) {
        address , rest = get_angle_addr ( arg );
        } else {
        address , rest = get_addr_spec ( arg );
        if !address {
        return  address , rest;
        return  address . addr_spec , rest;
        pub fn _getparams ( &self, params )  {
        result = { };
        for param in params .iter() {
        param , eq , value = param . partition ( "=" );
        if !param . isalnum ( ) || eq && !value {
        return;
        result [ param ] = value if eq else true;
        return  result;
        pub fn smtp_HELP ( &self, arg )  {
        if arg {
        extended = " [SP <mail-parameters>]";
        lc_arg = arg . upper ( );
        if lc_arg == "EHLO" {
        self . push ( "250 Syntax: EHLO hostname" );
        } else if lc_arg == "HELO" {
        self . push ( "250 Syntax: HELO hostname" );
        } else if lc_arg == "MAIL" {
        msg = "250 Syntax: MAIL FROM: <address>";
        if self . extended_smtp {
        msg + = extended;
        self . push ( msg );
        } else if lc_arg == "RCPT" {
        msg = "250 Syntax: RCPT TO: <address>";
        if self . extended_smtp {
        msg + = extended;
        self . push ( msg );
        } else if lc_arg == "DATA" {
        self . push ( "250 Syntax: DATA" );
        } else if lc_arg == "RSET" {
        self . push ( "250 Syntax: RSET" );
        } else if lc_arg == "NOOP" {
        self . push ( "250 Syntax: NOOP" );
        } else if lc_arg == "QUIT" {
        self . push ( "250 Syntax: QUIT" );
        } else if lc_arg == "VRFY" {
        self . push ( "250 Syntax: VRFY <address>" );
        } else {
        self . push ( "501 Supported commands: EHLO HELO MAIL RCPT ";
        "DATA RSET NOOP QUIT VRFY" );
        } else {
        self . push ( "250 Supported commands: EHLO HELO MAIL RCPT DATA ";
        "RSET NOOP QUIT VRFY" );
        pub fn smtp_VRFY ( &self, arg )  {
        if arg {
        address , params = self . _getaddr ( arg );
        if address {
        self . push ( "252 Cannot VRFY user, but will accept message ";
        "and attempt delivery" );
        } else {
        self . push ( "502 Could !VRFY %s" % arg );
        } else {
        self . push ( "501 Syntax: VRFY <address>" );
        pub fn smtp_MAIL ( &self, arg )  {
        if !self . seen_greeting {
        self . push ( "503 Error: send HELO first" );
        return;
        println!( "===> MAIL" , arg , file = DEBUGSTREAM );
        syntaxerr = "501 Syntax: MAIL FROM: <address>";
        if self . extended_smtp {
        syntaxerr + = " [SP <mail-parameters>]";
        if arg is None /* Option */ {
        self . push ( syntaxerr );
        return;
        arg = self . _strip_command_keyword ( "FROM:" , arg );
        address , params = self . _getaddr ( arg );
        if !address {
        self . push ( syntaxerr );
        return;
        if !self . extended_smtp && params {
        self . push ( syntaxerr );
        return;
        if self . mailfrom {
        self . push ( "503 Error: nested MAIL command" );
        return;
        self . mail_options = params . upper ( ) . split ( );
        params = self . _getparams ( self . mail_options );
        if params is None /* Option */ {
        self . push ( syntaxerr );
        return;
        if !self . _decode_data {
        body = params . pop ( "BODY" , "7BIT" );
        if body !in [ "7BIT" , "8BITMIME" ] {
        self . push ( "501 Error: BODY can only be one of 7BIT, 8BITMIME" );
        return;
        if self . enable_SMTPUTF8 {
        smtputf8 = params . pop ( "SMTPUTF8" , false );
        if smtputf8 is true {
        self . require_SMTPUTF8 = true;
        } else if smtputf8 is !false {
        self . push ( "501 Error: SMTPUTF8 takes no arguments" );
        return;
        size = params . pop ( "SIZE" , None /* Option */ );
        if size {
        if !size . isdigit ( ) {
        self . push ( syntaxerr );
        return;
        } else if self . data_size_limit && int ( size ) > self . data_size_limit {
        self . push ( "552 Error: message size exceeds fixed maximum message size" );
        return;
        if len ( params . keys ( ) ) > 0 {
        self . push ( "555 MAIL FROM parameters !recognized || !implemented" );
        return;
        self . mailfrom = address;
        println!( "sender:" , self . mailfrom , file = DEBUGSTREAM );
        self . push ( "250 OK" );
        pub fn smtp_RCPT ( &self, arg )  {
        if !self . seen_greeting {
        self . push ( "503 Error: send HELO first" ) ;;
        return;
        println!( "===> RCPT" , arg , file = DEBUGSTREAM );
        if !self . mailfrom {
        self . push ( "503 Error: need MAIL command" );
        return;
        syntaxerr = "501 Syntax: RCPT TO: <address>";
        if self . extended_smtp {
        syntaxerr + = " [SP <mail-parameters>]";
        if arg is None /* Option */ {
        self . push ( syntaxerr );
        return;
        arg = self . _strip_command_keyword ( "TO:" , arg );
        address , params = self . _getaddr ( arg );
        if !address {
        self . push ( syntaxerr );
        return;
        if !self . extended_smtp && params {
        self . push ( syntaxerr );
        return;
        self . rcpt_options = params . upper ( ) . split ( );
        params = self . _getparams ( self . rcpt_options );
        if params is None /* Option */ {
        self . push ( syntaxerr );
        return;
        if len ( params . keys ( ) ) > 0 {
        self . push ( "555 RCPT TO parameters !recognized || !implemented" );
        return;
        self . rcpttos . append ( address );
        println!( "recips:" , self . rcpttos , file = DEBUGSTREAM );
        self . push ( "250 OK" );
        pub fn smtp_RSET ( &self, arg )  {
        if arg {
        self . push ( "501 Syntax: RSET" );
        return;
        self . _set_rset_state ( );
        self . push ( "250 OK" );
        pub fn smtp_DATA ( &self, arg )  {
        if !self . seen_greeting {
        self . push ( "503 Error: send HELO first" ) ;;
        return;
        if !self . rcpttos {
        self . push ( "503 Error: need RCPT command" );
        return;
        if arg {
        self . push ( "501 Syntax: DATA" );
        return;
        self . smtp_state = self . DATA;
        self . set_terminator ( b "\r\n.\r\n" );
        self . push ( "354 End data with <CR><LF>.<CR><LF>" );
        pub fn smtp_EXPN ( &self, arg )  {
        self . push ( "502 EXPN !implemented" );
        class SMTPServer ( asyncore . dispatcher ) ;
        channel_class = SMTPChannel;
        pub fn __init__ ( &self, localaddr , remoteaddr , {
        data_size_limit = DATA_SIZE_DEFAULT , map = None /* Option */ ,;
        enable_SMTPUTF8 = false , decode_data = false ) ;
        self . _localaddr = localaddr;
        self . _remoteaddr = remoteaddr;
        self . data_size_limit = data_size_limit;
        self . enable_SMTPUTF8 = enable_SMTPUTF8;
        self . _decode_data = decode_data;
        if enable_SMTPUTF8 && decode_data {
        panic!("ValueError ( "decode_data && enable_SMTPUTF8 cannot"");
        " be set to true at the same time" );
        asyncore . dispatcher . __init__ ( self , map = map );
        // try {
        gai_results = socket . getaddrinfo ( * localaddr ,;
        type = socket . SOCK_STREAM );
        self . create_socket ( gai_results [ 0 ] [ 0 ] , gai_results [ 0 ] [ 1 ] );
        self . set_reuse_addr ( );
        self . bind ( localaddr );
        self . listen ( 5 );
        // } catch   {
        self . close ( );
        panic!("");
        } else {
        println!( "%s started at %s\n\tLocal addr: %s\n\tRemote addr:%s" % );
        self . __class__ . __name__ , time . ctime ( time . time ( ) ) ,;
        localaddr , remoteaddr ) , file = DEBUGSTREAM );
        pub fn handle_accepted ( &self, conn , addr )  {
        println!( "Incoming connection from %s" % repr ( addr ) , file = DEBUGSTREAM );
        channel = self . channel_class ( self ,;
        conn ,;
        addr ,;
        self . data_size_limit ,;
        self . _map ,;
        self . enable_SMTPUTF8 ,;
        self . _decode_data );
        pub fn process_message ( &self, peer , mailfrom , rcpttos , data , ** kwargs )  {
        "Override this abstract method to handle messages from the client.

        peer == a tuple containing (ipaddr, port) of the client that made the
        socket connection to our smtp port.

        mailfrom == the raw address the client claims the message == coming
        from.

        rcpttos == a list of raw addresses the client wishes to deliver the
        message to.

        data == a string containing the entire full text of the message,
        headers (if supplied) && all.  It has been `de-transparencied'
        according to RFC 821, Section 4.5.2.  In other words, a line
        containing a `.' followed by other text has had the leading dot
        removed.

        kwargs == a dictionary containing additional information.  It is
        empty if decode_data=true was given as init parameter, otherwise
        it will contain the following keys:
            'mail_options': list of parameters to the mail command.  All
                            elements are uppercase strings.  Example:
                            vec!['BODY=8BITMIME', 'SMTPUTF8'].
            'rcpt_options': same,.iter().map(|the rcpt command.

        This function should return None /* Option */.iter().map(|a normal `250 Ok' response;
        otherwise, it should return the desired response string| RFC 821
        format.

        ";
        panic!("NotImplementedError");
        class DebuggingServer ( SMTPServer ) ;
        pub fn _print_message_content ( &self, peer , data )  {
        inheaders = 1;
        lines = data . splitlines ( );
        for line in lines .iter() {
        if inheaders && !line {
        peerheader = "X-Peer: " + peer [ 0 ];
        if !isinstance ( data , str ) {
        peerheader = repr ( peerheader . encode ( "utf-8" ) );
        println!( peerheader );
        inheaders = 0;
        if !isinstance ( data , str ) {
        line = repr ( line );
        println!( line );
        pub fn process_message ( &self, peer , mailfrom , rcpttos , data , ** kwargs )  {
        println!( "---------- MESSAGE FOLLOWS ----------" );
        if kwargs {
        if kwargs . get ( "mail_options" ) {
        println!( "mail options: %s" % kwargs [ "mail_options" ] );
        if kwargs . get ( "rcpt_options" ) {
        println!( "rcpt options: %s\n" % kwargs [ "rcpt_options" ] );
        self . _print_message_content ( peer , data );
        println!( "------------ END MESSAGE ------------" );
        class PureProxy ( SMTPServer ) ;
        pub fn __init__ ( &self, * args , ** kwargs )  {
        if "enable_SMTPUTF8" in kwargs && kwargs [ "enable_SMTPUTF8" ] {
        panic!("ValueError ( "PureProxy does !support SMTPUTF8." )");
        super ( PureProxy , self ) . __init__ ( * args , ** kwargs );
        pub fn process_message ( &self, peer , mailfrom , rcpttos , data )  {
        lines = data . split ( "\n" );
        i = 0;
        for line in lines .iter() {
        if !line {
        break;
        i + = 1;
        lines . insert ( i , "X-Peer: %s" % peer [ 0 ] );
        data = NEWLINE . join ( lines );
        refused = self . _deliver ( mailfrom , rcpttos , data );
        println!( "we got some refusals:" , refused , file = DEBUGSTREAM );
        pub fn _deliver ( &self, mailfrom , rcpttos , data )  {
        import smtplib;
        refused = { };
        // try {
        s = smtplib . SMTP ( );
        s . connect ( self . _remoteaddr [ 0 ] , self . _remoteaddr [ 1 ] );
        // try {
        refused = s . sendmail ( mailfrom , rcpttos , data );
        // } finally {
        s . quit ( );
        // } catch  smtplib . SMTPRecipientsRefused as e  {
        println!( "got SMTPRecipientsRefused" , file = DEBUGSTREAM );
        refused = e . recipients;
        // } catch  ( OSError , smtplib . SMTPException ) as e  {
        println!( "got" , e . __class__ , file = DEBUGSTREAM );
        errcode = getattr ( e , "smtp_code" , -1 );
        errmsg = getattr ( e , "smtp_error" , "ignore" );
        for r in rcpttos .iter() {
        refused [ r ] = ( errcode , errmsg );
        return  refused;
        class Options ;
        setuid = true;
        classname = "PureProxy";
        size_limit = None /* Option */;
        enable_SMTPUTF8 = false;
        pub fn parseargs ( )  {
        global DEBUGSTREAM;
        // try {
        opts , args = getopt . getopt (;
        sys . argv [ 1 : ] , "nVhc:s:du" ,;
        [ "class=" , "nosetuid" , "version" , "help" , "size=" , "debug" ,;
        "smtputf8" ] );
        // } catch  getopt . error as e  {
        usage ( 1 , e );
        options = Options ( );
        for opt , arg in opts .iter() {
        if opt in ( "-h" , "--help" ) {
        usage ( 0 );
        } else if opt in ( "-V" , "--version" ) {
        println!( __version__ );
        sys . exit ( 0 );
        } else if opt in ( "-n" , "--nosetuid" ) {
        options . setuid = false;
        } else if opt in ( "-c" , "--class" ) {
        options . classname = arg;
        } else if opt in ( "-d" , "--debug" ) {
        DEBUGSTREAM = sys . stderr;
        } else if opt in ( "-u" , "--smtputf8" ) {
        options . enable_SMTPUTF8 = true;
        } else if opt in ( "-s" , "--size" ) {
        // try {
        int_size = int ( arg );
        options . size_limit = int_size;
        // } catch   {
        println!( "Invalid size: " + arg , file = sys . stderr );
        sys . exit ( 1 );
        if len ( args ) < 1 {
        localspec = "localhost:8025";
        remotespec = "localhost:25";
        } else if len ( args ) < 2 {
        localspec = args [ 0 ];
        remotespec = "localhost:25";
        } else if len ( args ) < 3 {
        localspec = args [ 0 ];
        remotespec = args [ 1 ];
        } else {
        usage ( 1 , "Invalid arguments: %s" % COMMASPACE . join ( args ) );
        i = localspec . find ( ":" );
        if i < 0 {
        usage ( 1 , "Bad local spec: %s" % localspec );
        options . localhost = localspec [ : i ];
        // try {
        options . localport = int ( localspec [ i + 1 : ] );
        // } catch  ValueError  {
        usage ( 1 , "Bad local port: %s" % localspec );
        i = remotespec . find ( ":" );
        if i < 0 {
        usage ( 1 , "Bad remote spec: %s" % remotespec );
        options . remotehost = remotespec [ : i ];
        // try {
        options . remoteport = int ( remotespec [ i + 1 : ] );
        // } catch  ValueError  {
        usage ( 1 , "Bad remote port: %s" % remotespec );
        return  options;
        fn main() {
        options = parseargs ( );
        classname = options . classname;
        if "." in classname {
        lastdot = classname . rfind ( "." );
        mod = __import__ ( classname [ : lastdot ] , globals ( ) , locals ( ) , [ "" ] );
        classname = classname [ lastdot + 1 : ];
        } else {
        import __main__ as mod;
        class_ = getattr ( mod , classname );
        proxy = class_ ( ( options . localhost , options . localport ) ,;
        ( options . remotehost , options . remoteport ) ,;
        options . size_limit , enable_SMTPUTF8 = options . enable_SMTPUTF8 );
        if options . setuid {
        // try {
        import pwd;
        // } catch  ImportError  {
        println!( "Cannot import module "pwd"; try running with -n option." , file = sys . stderr );
        sys . exit ( 1 );
        nobody = pwd . getpwnam ( "nobody" ) [ 2 ];
        // try {
        os . setuid ( nobody );
        // } catch  PermissionError  {
        println!( "Cannot setuid "nobody"; try running with -n option." , file = sys . stderr );
        sys . exit ( 1 );
        // try {
        asyncore . loop ( );
        // } catch  KeyboardInterrupt  {
        // pass
    }

}

