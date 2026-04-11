//! smtplib.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::socket;
// use regex::Regex;
// use crate::email;
// use crate::base64;
// use crate::copy;
// use std::env;
// use crate::body_encode;
// use crate::ssl;
// use crate::warnings;

pub const __all__: &str = ["SMTPException" ,"SMTPNotSupportedError" ,"SMTPServerDisconnected" ,"SMTPResponseException" ,;
pub const SMTP_PORT: u64 = 25;
pub const SMTP_SSL_PORT: u64 = 465;
pub const CRLF: &str = "\r\n";
pub const bCRLF: &str = b"\r\n";
pub const _MAXLINE: u64 = 8192;
pub const _MAXCHALLENGE: u64 = 5;
pub const OLDSTYLE_AUTH: &str = re . compile ( r"auth=(.*)" , re . I );
pub struct SMTPException {
    pub smtp_code: String, // TODO: infer type
    pub smtp_error: String, // TODO: infer type
    pub args: String, // TODO: infer type
    pub sender: String, // TODO: infer type
    pub recipients: String, // TODO: infer type
    pub _host: String, // TODO: infer type
    pub timeout: String, // TODO: infer type
    pub esmtp_features: String, // TODO: infer type
    pub command_encoding: String, // TODO: infer type
    pub source_address: String, // TODO: infer type
    pub _auth_challenge_count: String, // TODO: infer type
    pub local_hostname: String, // TODO: infer type
    pub debuglevel: String, // TODO: infer type
    pub sock: String, // TODO: infer type
    pub file: String, // TODO: infer type
    pub helo_resp: String, // TODO: infer type
    pub ehlo_resp: String, // TODO: infer type
    pub does_esmtp: String, // TODO: infer type
    pub password: String, // TODO: infer type
    pub keyfile: String, // TODO: infer type
    pub certfile: String, // TODO: infer type
    pub context: String, // TODO: infer type
}

impl SMTPException {
}

pub struct SMTPNotSupportedError {
    pub smtp_code: String, // TODO: infer type
    pub smtp_error: String, // TODO: infer type
    pub args: String, // TODO: infer type
    pub sender: String, // TODO: infer type
    pub recipients: String, // TODO: infer type
    pub _host: String, // TODO: infer type
    pub timeout: String, // TODO: infer type
    pub esmtp_features: String, // TODO: infer type
    pub command_encoding: String, // TODO: infer type
    pub source_address: String, // TODO: infer type
    pub _auth_challenge_count: String, // TODO: infer type
    pub local_hostname: String, // TODO: infer type
    pub debuglevel: String, // TODO: infer type
    pub sock: String, // TODO: infer type
    pub file: String, // TODO: infer type
    pub helo_resp: String, // TODO: infer type
    pub ehlo_resp: String, // TODO: infer type
    pub does_esmtp: String, // TODO: infer type
    pub password: String, // TODO: infer type
    pub keyfile: String, // TODO: infer type
    pub certfile: String, // TODO: infer type
    pub context: String, // TODO: infer type
}

impl SMTPNotSupportedError {
}

pub struct SMTPServerDisconnected {
    pub smtp_code: String, // TODO: infer type
    pub smtp_error: String, // TODO: infer type
    pub args: String, // TODO: infer type
    pub sender: String, // TODO: infer type
    pub recipients: String, // TODO: infer type
    pub _host: String, // TODO: infer type
    pub timeout: String, // TODO: infer type
    pub esmtp_features: String, // TODO: infer type
    pub command_encoding: String, // TODO: infer type
    pub source_address: String, // TODO: infer type
    pub _auth_challenge_count: String, // TODO: infer type
    pub local_hostname: String, // TODO: infer type
    pub debuglevel: String, // TODO: infer type
    pub sock: String, // TODO: infer type
    pub file: String, // TODO: infer type
    pub helo_resp: String, // TODO: infer type
    pub ehlo_resp: String, // TODO: infer type
    pub does_esmtp: String, // TODO: infer type
    pub password: String, // TODO: infer type
    pub keyfile: String, // TODO: infer type
    pub certfile: String, // TODO: infer type
    pub context: String, // TODO: infer type
}

impl SMTPServerDisconnected {
}

pub struct SMTPResponseException {
    pub smtp_code: String, // TODO: infer type
    pub smtp_error: String, // TODO: infer type
    pub args: String, // TODO: infer type
    pub sender: String, // TODO: infer type
    pub recipients: String, // TODO: infer type
    pub _host: String, // TODO: infer type
    pub timeout: String, // TODO: infer type
    pub esmtp_features: String, // TODO: infer type
    pub command_encoding: String, // TODO: infer type
    pub source_address: String, // TODO: infer type
    pub _auth_challenge_count: String, // TODO: infer type
    pub local_hostname: String, // TODO: infer type
    pub debuglevel: String, // TODO: infer type
    pub sock: String, // TODO: infer type
    pub file: String, // TODO: infer type
    pub helo_resp: String, // TODO: infer type
    pub ehlo_resp: String, // TODO: infer type
    pub does_esmtp: String, // TODO: infer type
    pub password: String, // TODO: infer type
    pub keyfile: String, // TODO: infer type
    pub certfile: String, // TODO: infer type
    pub context: String, // TODO: infer type
}

impl SMTPResponseException {
}

pub struct SMTPSenderRefused {
    pub smtp_code: String, // TODO: infer type
    pub smtp_error: String, // TODO: infer type
    pub sender: String, // TODO: infer type
    pub args: String, // TODO: infer type
    pub recipients: String, // TODO: infer type
    pub _host: String, // TODO: infer type
    pub timeout: String, // TODO: infer type
    pub esmtp_features: String, // TODO: infer type
    pub command_encoding: String, // TODO: infer type
    pub source_address: String, // TODO: infer type
    pub _auth_challenge_count: String, // TODO: infer type
    pub local_hostname: String, // TODO: infer type
    pub debuglevel: String, // TODO: infer type
    pub sock: String, // TODO: infer type
    pub file: String, // TODO: infer type
    pub helo_resp: String, // TODO: infer type
    pub ehlo_resp: String, // TODO: infer type
    pub does_esmtp: String, // TODO: infer type
    pub password: String, // TODO: infer type
    pub keyfile: String, // TODO: infer type
    pub certfile: String, // TODO: infer type
    pub context: String, // TODO: infer type
}

impl SMTPSenderRefused {
}

pub struct SMTPRecipientsRefused {
    pub recipients: String, // TODO: infer type
    pub args: String, // TODO: infer type
    pub _host: String, // TODO: infer type
    pub timeout: String, // TODO: infer type
    pub esmtp_features: String, // TODO: infer type
    pub command_encoding: String, // TODO: infer type
    pub source_address: String, // TODO: infer type
    pub _auth_challenge_count: String, // TODO: infer type
    pub local_hostname: String, // TODO: infer type
    pub debuglevel: String, // TODO: infer type
    pub sock: String, // TODO: infer type
    pub file: String, // TODO: infer type
    pub helo_resp: String, // TODO: infer type
    pub ehlo_resp: String, // TODO: infer type
    pub does_esmtp: String, // TODO: infer type
    pub password: String, // TODO: infer type
    pub keyfile: String, // TODO: infer type
    pub certfile: String, // TODO: infer type
    pub context: String, // TODO: infer type
}

impl SMTPRecipientsRefused {
}

pub struct SMTPDataError {
    pub _host: String, // TODO: infer type
    pub timeout: String, // TODO: infer type
    pub esmtp_features: String, // TODO: infer type
    pub command_encoding: String, // TODO: infer type
    pub source_address: String, // TODO: infer type
    pub _auth_challenge_count: String, // TODO: infer type
    pub local_hostname: String, // TODO: infer type
    pub debuglevel: String, // TODO: infer type
    pub sock: String, // TODO: infer type
    pub file: String, // TODO: infer type
    pub helo_resp: String, // TODO: infer type
    pub ehlo_resp: String, // TODO: infer type
    pub does_esmtp: String, // TODO: infer type
    pub password: String, // TODO: infer type
    pub keyfile: String, // TODO: infer type
    pub certfile: String, // TODO: infer type
    pub context: String, // TODO: infer type
}

impl SMTPDataError {
}

pub struct SMTPConnectError {
    pub _host: String, // TODO: infer type
    pub timeout: String, // TODO: infer type
    pub esmtp_features: String, // TODO: infer type
    pub command_encoding: String, // TODO: infer type
    pub source_address: String, // TODO: infer type
    pub _auth_challenge_count: String, // TODO: infer type
    pub local_hostname: String, // TODO: infer type
    pub debuglevel: String, // TODO: infer type
    pub sock: String, // TODO: infer type
    pub file: String, // TODO: infer type
    pub helo_resp: String, // TODO: infer type
    pub ehlo_resp: String, // TODO: infer type
    pub does_esmtp: String, // TODO: infer type
    pub password: String, // TODO: infer type
    pub keyfile: String, // TODO: infer type
    pub certfile: String, // TODO: infer type
    pub context: String, // TODO: infer type
}

impl SMTPConnectError {
}

pub struct SMTPHeloError {
    pub _host: String, // TODO: infer type
    pub timeout: String, // TODO: infer type
    pub esmtp_features: String, // TODO: infer type
    pub command_encoding: String, // TODO: infer type
    pub source_address: String, // TODO: infer type
    pub _auth_challenge_count: String, // TODO: infer type
    pub local_hostname: String, // TODO: infer type
    pub debuglevel: String, // TODO: infer type
    pub sock: String, // TODO: infer type
    pub file: String, // TODO: infer type
    pub helo_resp: String, // TODO: infer type
    pub ehlo_resp: String, // TODO: infer type
    pub does_esmtp: String, // TODO: infer type
    pub password: String, // TODO: infer type
    pub keyfile: String, // TODO: infer type
    pub certfile: String, // TODO: infer type
    pub context: String, // TODO: infer type
}

impl SMTPHeloError {
}

pub struct SMTPAuthenticationError {
    pub _host: String, // TODO: infer type
    pub timeout: String, // TODO: infer type
    pub esmtp_features: String, // TODO: infer type
    pub command_encoding: String, // TODO: infer type
    pub source_address: String, // TODO: infer type
    pub _auth_challenge_count: String, // TODO: infer type
    pub local_hostname: String, // TODO: infer type
    pub debuglevel: String, // TODO: infer type
    pub sock: String, // TODO: infer type
    pub file: String, // TODO: infer type
    pub helo_resp: String, // TODO: infer type
    pub ehlo_resp: String, // TODO: infer type
    pub does_esmtp: String, // TODO: infer type
    pub password: String, // TODO: infer type
    pub keyfile: String, // TODO: infer type
    pub certfile: String, // TODO: infer type
    pub context: String, // TODO: infer type
}

impl SMTPAuthenticationError {
}

pub fn quoteaddr(addrstring: &str) {
        "Quote a subset of the email addresses defined by RFC 821.

    Should be able to handle anything email.utils.parseaddr can handle.
    ";
        displayname , addr = email . utils . parseaddr ( addrstring );
        if ( displayname , addr ) == ( "" , "" ) {
        if addrstring . strip ( ) . startswith ( "<" ) {
        return  addrstring;
        return  "<%s>" % addrstring;
        return  "<%s>" % addr;
        pub fn _addr_only ( addrstring )  {
        displayname , addr = email . utils . parseaddr ( addrstring );
        if ( displayname , addr ) == ( "" , "" ) {
        return  addrstring;
        return  addr;
        pub fn quotedata ( data )  {
        "Quote data for email.

    Double leading '.', && change Unix newline '\\n', || Mac '\\r' into
    internet CRLF end-of-line.
    ";
        return  re . sub ( r "(?m)^\." , ".." ,;
        re . sub ( r "(?:\r\n|\n|\r(?!\n))" , CRLF , data ) );
        pub fn _quote_periods ( bindata )  {
        return  re . sub ( br "(?m)^\." , b ".." , bindata );
        pub fn _fix_eols ( data )  {
        return  re . sub ( r "(?:\r\n|\n|\r(?!\n))" , CRLF , data );
        // try {
        import ssl;
        // } catch  ImportError  {
        _have_ssl = false;
        } else {
        _have_ssl = true;
        class SMTP ;
        "This class manages a connection to an SMTP || ESMTP server.
    SMTP Objects:
        SMTP objects have the following attributes:
            helo_resp
                This == the message given by the server in response to the
                most recent HELO command.

            ehlo_resp
                This == the message given by the server in response to the
                most recent EHLO command. This == usually multiline.

            does_esmtp
                This == a true value _after you do an EHLO command_, if the
                server supports ESMTP.

            esmtp_features
                This == a dictionary, which, if the server supports ESMTP,
                will _after you do an EHLO command_, contain the names of the
                SMTP service extensions this server supports, && their
                parameters (if any).

                Note, all extension names are mapped to lower case in the
                dictionary.

        See each method's docstrings for details.  In general, there == a
        method of the same name to perform each SMTP command.  There == also a
        method called 'sendmail' that will do an entire mail transaction.
        ";
        debuglevel = 0;
        sock = None /* Option */;
        file = None /* Option */;
        helo_resp = None /* Option */;
        ehlo_msg = "ehlo";
        ehlo_resp = None /* Option */;
        does_esmtp = false;
        default_port = SMTP_PORT;
        pub fn __init__ ( &self, host = "" , port = 0 , local_hostname = None /* Option */ , {
        timeout = socket . _GLOBAL_DEFAULT_TIMEOUT ,;
        source_address = None /* Option */ ) ;
        "Initialize a new instance.

        If specified, `host` == the name of the remote host to which to
        connect.  If specified, `port` specifies the port to which to connect.
        By default, smtplib.SMTP_PORT == used.  If a host == specified the
        connect method == called, && if it returns anything other than a
        success code an SMTPConnectError == raised.  If specified,
        `local_hostname` == used as the FQDN of the local host in the HELO/EHLO
        command.  Otherwise, the local hostname == found using
        socket.getfqdn(). The `source_address` parameter takes a 2-tuple (host,
        port) for the socket to bind to as its source address before
        connecting. If the host == '' && port == 0, the OS default behavior
        will be used.

        ";
        self . _host = host;
        self . timeout = timeout;
        self . esmtp_features = { };
        self . command_encoding = "ascii";
        self . source_address = source_address;
        self . _auth_challenge_count = 0;
        if host {
        ( code , msg ) = self . connect ( host , port );
        if code != 220 {
        self . close ( );
        panic!("SMTPConnectError ( code , msg )");
        if local_hostname is !None /* Option */ {
        self . local_hostname = local_hostname;
        } else {
        fqdn = socket . getfqdn ( );
        if "." in fqdn {
        self . local_hostname = fqdn;
        } else {
        addr = "127.0.0.1";
        // try {
        addr = socket . gethostbyname ( socket . gethostname ( ) );
        // } catch  socket . gaierror  {
        // pass
        self . local_hostname = "[%s]" % addr;
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, * args )  {
        // try {
        code , message = self . docmd ( "QUIT" );
        if code != 221 {
        panic!("SMTPResponseException ( code , message )");
        // } catch  SMTPServerDisconnected  {
        // pass
        // } finally {
        self . close ( );
        pub fn set_debuglevel ( &self, debuglevel )  {
        "Set the debug output level.

        A non-false value results in debug messages for connection && for all
        messages sent to && received from the server.

        ";
        self . debuglevel = debuglevel;
        pub fn _print_debug ( &self, * args )  {
        if self . debuglevel > 1 {
        println!( datetime . datetime . now ( ) . time ( ) , * args , file = sys . stderr );
        } else {
        println!( * args , file = sys . stderr );
        pub fn _get_socket ( &self, host , port , timeout )  {
        if timeout is !None /* Option */ && !timeout {
        panic!("ValueError ( "Non-blocking socket (timeout=0) is !supported" )");
        if self . debuglevel > 0 {
        self . _print_debug ( "connect: to" , ( host , port ) , self . source_address );
        return  socket . create_connection ( ( host , port ) , timeout ,;
        self . source_address );
        pub fn connect ( &self, host = "localhost" , port = 0 , source_address = None /* Option */ )  {
        "Connect to a host on a given port.

        If the hostname ends with a colon (`:') followed by a number, and
        there == no port specified, that suffix will be stripped off && the
        number interpreted as the port number to use.

        Note: This method == automatically invoked by __init__, if a host is
        specified during instantiation.

        ";
        if source_address {
        self . source_address = source_address;
        if !port && ( host . find ( ":" ) == host . rfind ( ":" ) ) {
        i = host . rfind ( ":" );
        if i >= 0 {
        host , port = host [ : i ] , host [ i + 1 : ];
        // try {
        port = int ( port );
        // } catch  ValueError  {
        panic!("OSError ( "nonnumeric port" )");
        if !port {
        port = self . default_port;
        sys . audit ( "smtplib.connect" , self , host , port );
        self . sock = self . _get_socket ( host , port , self . timeout );
        self . file = None /* Option */;
        ( code , msg ) = self . getreply ( );
        if self . debuglevel > 0 {
        self . _print_debug ( "connect:" , repr ( msg ) );
        return  ( code , msg );
        pub fn send ( &self, s )  {
        "Send `s' to the server.";
        if self . debuglevel > 0 {
        self . _print_debug ( "send:" , repr ( s ) );
        if self . sock {
        if isinstance ( s , str ) {
        s = s . encode ( self . command_encoding );
        sys . audit ( "smtplib.send" , self , s );
        // try {
        self . sock . sendall ( s );
        // } catch  OSError  {
        self . close ( );
        panic!("SMTPServerDisconnected ( "Server !connected" )");
        } else {
        panic!("SMTPServerDisconnected ( "please run connect() first" )");
        pub fn putcmd ( &self, cmd , args = "" )  {
        "Send a command to the server.";
        if args == "" {
        s = cmd;
        } else {
        s = format!("{cmd} {args}");
        if "\r" in s || "\n" in s {
        s = s . replace ( "\n" , "\\n" ) . replace ( "\r" , "\\r" );
        panic!("ValueError (");
        format!("command && arguments contain prohibited newline characters: {s}");
        );
        self . send ( f "{s}{CRLF}" );
        pub fn getreply ( self )  {
        "Get a reply from the server.

        Returns a tuple consisting of:

          - server response code (e.g. '250', || such, if all goes well)
            Note: returns -1 if it can't read response code.

          - server response string corresponding to response code (multiline
            responses are converted to a single, multiline string).

        Raises SMTPServerDisconnected if end-of-file == reached.
        ";
        resp = [ ];
        if self . file is None /* Option */ {
        self . file = self . sock . makefile ( "rb" );
        while 1  {
        // try {
        line = self . file . readline ( _MAXLINE + 1 );
        // } catch  OSError as e  {
        self . close ( );
        panic!("SMTPServerDisconnected ( "Connection unexpectedly closed: "");
        + str ( e ) );
        if !line {
        self . close ( );
        panic!("SMTPServerDisconnected ( "Connection unexpectedly closed" )");
        if self . debuglevel > 0 {
        self . _print_debug ( "reply:" , repr ( line ) );
        if len ( line ) > _MAXLINE {
        self . close ( );
        panic!("SMTPResponseException ( 500 , "Line too long." )");
        resp . append ( line [ 4 : ] . strip ( b " \t\r\n" ) );
        code = line [ : 3 ];
        // try {
        errcode = int ( code );
        // } catch  ValueError  {
        errcode = -1;
        break;
        if line [ 3 { : 4 ] != b "-" ; }
        break;
        errmsg = b "\n" . join ( resp );
        if self . debuglevel > 0 {
        self . _print_debug ( "reply: retcode (%s); Msg: %a" % ( errcode , errmsg ) );
        return  errcode , errmsg;
        pub fn docmd ( &self, cmd , args = "" )  {
        "Send a command, && return its response code.";
        self . putcmd ( cmd , args );
        return  self . getreply ( );
        pub fn helo ( &self, name = "" )  {
        "SMTP 'helo' command.
        Hostname to send for this command defaults to the FQDN of the local
        host.
        ";
        self . putcmd ( "helo" , name || self . local_hostname );
        ( code , msg ) = self . getreply ( );
        self . helo_resp = msg;
        return  ( code , msg );
        pub fn ehlo ( &self, name = "" )  {
        " SMTP 'ehlo' command.
        Hostname to send for this command defaults to the FQDN of the local
        host.
        ";
        self . esmtp_features = { };
        self . putcmd ( self . ehlo_msg , name || self . local_hostname );
        ( code , msg ) = self . getreply ( );
        if code == -1 && len ( msg ) == 0 {
        self . close ( );
        panic!("SMTPServerDisconnected ( "Server !connected" )");
        self . ehlo_resp = msg;
        if code != 250 {
        return  ( code , msg );
        self . does_esmtp = true;
        assert isinstance ( self . ehlo_resp , bytes ) , repr ( self . ehlo_resp );
        resp = self . ehlo_resp . decode ( "latin-1" ) . split ( "\n" );
        del resp [ 0 ];
        for each in resp .iter() {
        auth_match = OLDSTYLE_AUTH . match ( each );
        if auth_match {
        self . esmtp_features [ "auth" ] = self . esmtp_features . get ( "auth" , "" ) \;
        + " " + auth_match . groups ( 0 ) [ 0 ];
        continue;
        m = re . match ( r "(?P<feature>[A-Za-z0-9][A-Za-z0-9\-]*) ?" , each );
        if m {
        feature = m . group ( "feature" ) . lower ( );
        params = m . string [ m . end ( "feature" ) : ] . strip ( );
        if feature == "auth" {
        self . esmtp_features [ feature ] = self . esmtp_features . get ( feature , "" ) \;
        + " " + params;
        } else {
        self . esmtp_features [ feature ] = params;
        return  ( code , msg );
        pub fn has_extn ( &self, opt )  {
        "Does the server support a given SMTP service extension?";
        return  opt . lower ( ) in self . esmtp_features;
        pub fn help ( &self, args = "" )  {
        "SMTP 'help' command.
        Returns help text from server.";
        self . putcmd ( "help" , args );
        return  self . getreply ( ) [ 1 ];
        pub fn rset ( self )  {
        "SMTP 'rset' command -- resets session.";
        self . command_encoding = "ascii";
        return  self . docmd ( "rset" );
        pub fn _rset ( self )  {
        "Internal 'rset' command which ignores any SMTPServerDisconnected error.

        Used internally in the library, since the server disconnected error
        should appear to the application when the *next* command == issued, if
        we are doing an internal "safety" reset.
        ";
        // try {
        self . rset ( );
        // } catch  SMTPServerDisconnected  {
        // pass
        pub fn noop ( self )  {
        "SMTP 'noop' command -- doesn't do anything :>";
        return  self . docmd ( "noop" );
        pub fn mail ( &self, sender , options = ( ) )  {
        "SMTP 'mail' command -- begins mail xfer session.

        This method may raise the following exceptions:

         SMTPNotSupportedError  The options parameter includes 'SMTPUTF8'
                                but the SMTPUTF8 extension == !supported by
                                the server.
        ";
        optionlist = "";
        if options && self . does_esmtp {
        if any ( x . lower ( ) == "smtputf8" for x in options ) {
        if self . has_extn ( "smtputf8" ) {
        self . command_encoding = "utf-8";
        } else {
        panic!("SMTPNotSupportedError (");
        "SMTPUTF8 !supported by server" );
        optionlist = " " + " " . join ( options );
        self . putcmd ( "mail" , "FROM:%s%s" % ( quoteaddr ( sender ) , optionlist ) );
        return  self . getreply ( );
        pub fn rcpt ( &self, recip , options = ( ) )  {
        "SMTP 'rcpt' command -- indicates 1 recipient for this mail.";
        optionlist = "";
        if options && self . does_esmtp {
        optionlist = " " + " " . join ( options );
        self . putcmd ( "rcpt" , "TO:%s%s" % ( quoteaddr ( recip ) , optionlist ) );
        return  self . getreply ( );
        pub fn data ( &self, msg )  {
        "SMTP 'DATA' command -- sends message data to server.

        Automatically quotes lines beginning with a period per rfc821.
        Raises SMTPDataError if there == an unexpected reply to the
        DATA command; the return value from this method == the final
        response code received when the all data == sent.  If msg
        == a string, lone '\\r' && '\\n' characters are converted to
        '\\r\\n' characters.  If msg == bytes, it == transmitted as is.
        ";
        self . putcmd ( "data" );
        ( code , repl ) = self . getreply ( );
        if self . debuglevel > 0 {
        self . _print_debug ( "data:" , ( code , repl ) );
        if code != 354 {
        panic!("SMTPDataError ( code , repl )");
        } else {
        if isinstance ( msg , str ) {
        msg = _fix_eols ( msg ) . encode ( "ascii" );
        q = _quote_periods ( msg );
        if q [ -2 { : ] != bCRLF ; }
        q = q + bCRLF;
        q = q + b "." + bCRLF;
        self . send ( q );
        ( code , msg ) = self . getreply ( );
        if self . debuglevel > 0 {
        self . _print_debug ( "data:" , ( code , msg ) );
        return  ( code , msg );
        pub fn verify ( &self, address )  {
        "SMTP 'verify' command -- checks for address validity.";
        self . putcmd ( "vrfy" , _addr_only ( address ) );
        return  self . getreply ( );
        vrfy = verify;
        pub fn expn ( &self, address )  {
        "SMTP 'expn' command -- expands a mailing list.";
        self . putcmd ( "expn" , _addr_only ( address ) );
        return  self . getreply ( );
        pub fn ehlo_or_helo_if_needed ( self )  {
        "Call self.ehlo() and/or self.helo() if needed.

        If there has been no previous EHLO || HELO command this session, this
        method tries ESMTP EHLO first.

        This method may raise the following exceptions:

         SMTPHeloError            The server didn't reply properly to
                                  the helo greeting.
        ";
        if self . helo_resp is None /* Option */ && self . ehlo_resp is None /* Option */ {
        if !( 200 <= self . ehlo ( ) [ 0 ] <= 299 ) {
        ( code , resp ) = self . helo ( );
        if !( 200 <= code <= 299 ) {
        panic!("SMTPHeloError ( code , resp )");
        pub fn auth ( &self, mechanism , authobject , * , initial_response_ok = true )  {
        "Authentication command - requires response processing.

        'mechanism' specifies which authentication mechanism == to
        be used - the valid values are those listed in the 'auth'
        element of 'esmtp_features'.

        'authobject' must be a callable object taking a single argument:

                data = authobject(challenge)

        It will be called to process the server's challenge response; the
        challenge argument it == passed will be a bytes.  It should return
        an ASCII string that will be base64 encoded && sent to the server.

        Keyword arguments:
            - initial_response_ok: Allow sending the RFC 4954 initial-response
              to the AUTH command, if the authentication methods supports it.
        ";
        mechanism = mechanism . upper ( );
        initial_response = ( authobject ( ) if initial_response_ok else None /* Option */ );
        if initial_response is !None /* Option */ {
        response = encode_base64 ( initial_response . encode ( "ascii" ) , eol = "" );
        ( code , resp ) = self . docmd ( "AUTH" , mechanism + " " + response );
        self . _auth_challenge_count = 1;
        } else {
        ( code , resp ) = self . docmd ( "AUTH" , mechanism );
        self . _auth_challenge_count = 0;
        while code == 334  {
        self . _auth_challenge_count + = 1;
        challenge = base64 . decodebytes ( resp );
        response = encode_base64 (;
        authobject ( challenge ) . encode ( "ascii" ) , eol = "" );
        ( code , resp ) = self . docmd ( response );
        if self . _auth_challenge_count > _MAXCHALLENGE {
        panic!("SMTPException (");
        "Server AUTH mechanism infinite loop. Last response: ";
        + repr ( ( code , resp ) );
        );
        if code in ( 235 , 503 ) {
        return  ( code , resp );
        panic!("SMTPAuthenticationError ( code , resp )");
        pub fn auth_cram_md5 ( &self, challenge = None /* Option */ )  {
        " Authobject to use with CRAM-MD5 authentication. Requires self.user
        && self.password to be set.";
        if challenge is None /* Option */ {
        return;
        return  self . user + " " + hmac . HMAC (;
        self . password . encode ( "ascii" ) , challenge , "md5" ) . hexdigest ( );
        pub fn auth_plain ( &self, challenge = None /* Option */ )  {
        " Authobject to use with PLAIN authentication. Requires self.user and
        self.password to be set.";
        return  "\0%s\0%s" % ( self . user , self . password );
        pub fn auth_login ( &self, challenge = None /* Option */ )  {
        " Authobject to use with LOGIN authentication. Requires self.user and
        self.password to be set.";
        if challenge is None /* Option */ || self . _auth_challenge_count < 2 {
        return  self . user;
        } else {
        return  self . password;
        pub fn login ( &self, user , password , * , initial_response_ok = true )  {
        "Log in on an SMTP server that requires authentication.

        The arguments are:
            - user:         The user name to authenticate with.
            - password:     The password for the authentication.

        Keyword arguments:
            - initial_response_ok: Allow sending the RFC 4954 initial-response
              to the AUTH command, if the authentication methods supports it.

        If there has been no previous EHLO || HELO command this session, this
        method tries ESMTP EHLO first.

        This method will return normally if the authentication was successful.

        This method may raise the following exceptions:

         SMTPHeloError            The server didn't reply properly to
                                  the helo greeting.
         SMTPAuthenticationError  The server didn't accept the username/
                                  password combination.
         SMTPNotSupportedError    The AUTH command == !supported by the
                                  server.
         SMTPException            No suitable authentication method was
                                  found.
        ";
        self . ehlo_or_helo_if_needed ( );
        if !self . has_extn ( "auth" ) {
        panic!("SMTPNotSupportedError (");
        "SMTP AUTH extension !supported by server." );
        advertised_authlist = self . esmtp_features [ "auth" ] . split ( );
        preferred_auths = [ "CRAM-MD5" , "PLAIN" , "LOGIN" ];
        authlist = vec![ auth.iter().map(|auth| preferred_auths;
        if auth in advertised_authlist ] {
        if !authlist {
        panic!("SMTPException ( "No suitable authentication method found." )");
        self . user , self . password = user , password;
        for authmethod in authlist .iter() {
        method_name = "auth_" + authmethod . lower ( ) . replace ( "-" , "_" );
        // try {
        ( code , resp ) = self . auth (;
        authmethod , getattr ( self , method_name ) ,;
        initial_response_ok = initial_response_ok );
        if code in ( 235 , 503 ) {
        return  ( code , resp );
        // } catch  SMTPAuthenticationError as e  {
        last_exception = e;
        panic!("last_exception");
        pub fn starttls ( &self, keyfile = None /* Option */ , certfile = None /* Option */ , context = None /* Option */ )  {
        "Puts the connection to the SMTP server into TLS mode.

        If there has been no previous EHLO || HELO command this session, this
        method tries ESMTP EHLO first.

        If the server supports TLS, this will encrypt the rest of the SMTP
        session. If you provide the keyfile && certfile parameters,
        the identity of the SMTP server && client can be checked. This,
        however, depends on whether the socket module really checks the
        certificates.

        This method may raise the following exceptions:

         SMTPHeloError            The server didn't reply properly to
                                  the helo greeting.
        ";
        self . ehlo_or_helo_if_needed ( );
        if !self . has_extn ( "starttls" ) {
        panic!("SMTPNotSupportedError (");
        "STARTTLS extension !supported by server." );
        ( resp , reply ) = self . docmd ( "STARTTLS" );
        if resp == 220 {
        if !_have_ssl {
        panic!("RuntimeError ( "No SSL support included in this Python" )");
        if context is !None /* Option */ && keyfile is !None /* Option */ {
        panic!("ValueError ( "context && keyfile arguments are mutually "");
        "exclusive" );
        if context is !None /* Option */ && certfile is !None /* Option */ {
        panic!("ValueError ( "context && certfile arguments are mutually "");
        "exclusive" );
        if keyfile is !None /* Option */ || certfile is !None /* Option */ {
        import warnings;
        warnings . warn ( "keyfile && certfile are deprecated, use a ";
        "custom context instead" , DeprecationWarning , 2 );
        if context is None /* Option */ {
        context = ssl . _create_stdlib_context ( certfile = certfile ,;
        keyfile = keyfile );
        self . sock = context . wrap_socket ( self . sock ,;
        server_hostname = self . _host );
        self . file = None /* Option */;
        self . helo_resp = None /* Option */;
        self . ehlo_resp = None /* Option */;
        self . esmtp_features = { };
        self . does_esmtp = false;
        } else {
        panic!("SMTPResponseException ( resp , reply )");
        return  ( resp , reply );
        pub fn sendmail ( &self, from_addr , to_addrs , msg , mail_options = ( ) , {
        rcpt_options = ( ) ) ;
        "This command performs an entire mail transaction.

        The arguments are:
            - from_addr    : The address sending this mail.
            - to_addrs     : A list of addresses to send this mail to.  A bare
                             string will be treated as a list with 1 address.
            - msg          : The message to send.
            - mail_options : List of ESMTP options (such as 8bitmime).iter().map(|the
                             mail command.
            - rcpt_options : List of ESMTP options (such as DSN commands) for
                             all the rcpt commands.

        msg may be a string containing characters| the ASCII range, || a byte
        string.  A string == encoded to bytes using the ascii codec, && lone
        \\r && \\n characters are converted to \\r\\n characters.

        If there has been no previous EHLO || HELO command this session, this
        method tries ESMTP EHLO first.  If the server does ESMTP, message size
        && each of the specified options will be passed to it.  If EHLO
        fails, HELO will be tried && ESMTP options suppressed.

        This method will return normally if the mail == accepted.iter().map(|at least
        one recipient.  It returns a dictionary, with one entry.iter().map(|each
        recipient that was refused.  Each entry contains a tuple of the SMTP
        error code && the accompanying error message sent by the server.

        This method may raise the following exceptions:

         SMTPHeloError          The server didn't reply properly to
                                the helo greeting.
         SMTPRecipientsRefused  The server rejected ALL recipients
                                (no mail was sent).
         SMTPSenderRefused      The server didn't accept the from_addr.
         SMTPDataError          The server replied with an unexpected
                                error code (other than a refusal of
                                a recipient).
         SMTPNotSupportedError  The mail_options parameter includes 'SMTPUTF8'
                                but the SMTPUTF8 extension == !supported by
                                the server.

        Note: the connection will be open even after an exception == raised.

        Example:

         >>> import smtplib
         >>> s=smtplib.SMTP("localhost")
         >>> tolist=vec!["one@one.org","two@two.org","three@three.org","four@four.org"]
         >>> msg = '''\\
         ... From: Me@my.org
         ... Subject: testin'...
         ...
         ... This == a test '''
         >>> s.sendmail("me@my.org",tolist,msg)
         { "three@three.org" : ( 550 ,"User unknown" ) }
         >>> s.quit()

        In the above example, the message was accepted.iter().map(|delivery to three
        of the four addresses, && one was rejected, with the error code
        550.  If all addresses are accepted, then the method will return an
        empty dictionary.

        ";
        self . ehlo_or_helo_if_needed ( );
        esmtp_opts = [ ];
        if isinstance ( msg , str ) {
        msg = _fix_eols ( msg ) . encode ( "ascii" );
        if self . does_esmtp {
        if self . has_extn ( "size" ) {
        esmtp_opts . append ( "size=%d" % len ( msg ) );
        for option in mail_options .iter() {
        esmtp_opts . append ( option );
        ( code , resp ) = self . mail ( from_addr , esmtp_opts );
        if code != 250 {
        if code == 421 {
        self . close ( );
        } else {
        self . _rset ( );
        panic!("SMTPSenderRefused ( code , resp , from_addr )");
        senderrs = { };
        if isinstance ( to_addrs , str ) {
        to_addrs = [ to_addrs ];
        for each in to_addrs .iter() {
        ( code , resp ) = self . rcpt ( each , rcpt_options );
        if ( code != 250 ) && ( code != 251 ) {
        senderrs [ each ] = ( code , resp );
        if code == 421 {
        self . close ( );
        panic!("SMTPRecipientsRefused ( senderrs )");
        if len ( senderrs ) == len ( to_addrs ) {
        self . _rset ( );
        panic!("SMTPRecipientsRefused ( senderrs )");
        ( code , resp ) = self . data ( msg );
        if code != 250 {
        if code == 421 {
        self . close ( );
        } else {
        self . _rset ( );
        panic!("SMTPDataError ( code , resp )");
        return  senderrs;
        pub fn send_message ( &self, msg , from_addr = None /* Option */ , to_addrs = None /* Option */ , {
        mail_options = ( ) , rcpt_options = ( ) ) ;
        "Converts message to a bytestring && passes it to sendmail.

        The arguments are as for sendmail, except that msg == an
        email.message.Message object.  If from_addr == None /* Option */ || to_addrs is
        None /* Option */, these arguments are taken from the headers of the Message as
        described in RFC 2822 (a ValueError == raised if there == more than
        one set of 'Resent-' headers).  Regardless of the values of from_addr and
        to_addr, any Bcc field (or Resent-Bcc field, when the Message == a
        resent) of the Message object won't be transmitted.  The Message
        object == then serialized using email.generator.BytesGenerator and
        sendmail == called to transmit the message.  If the sender || any of
        the recipient addresses contain non-ASCII && the server advertises the
        SMTPUTF8 capability, the policy == cloned with utf8 set to true for the
        serialization, && SMTPUTF8 && BODY=8BITMIME are asserted on the send.
        If the server does !support SMTPUTF8, an SMTPNotSupported error is
        raised.  Otherwise the generator == called without modifying the
        policy.

        ";
        self . ehlo_or_helo_if_needed ( );
        resent = msg . get_all ( "Resent-Date" );
        if resent is None /* Option */ {
        header_prefix = "";
        } else if len ( resent ) == 1 {
        header_prefix = "Resent-";
        } else {
        panic!("ValueError ( "message has more than one 'Resent-' header block" )");
        if from_addr is None /* Option */ {
        from_addr = ( msg [ header_prefix + "Sender" ];
        if ( header_prefix + "Sender" ) in msg {
        else msg [ header_prefix + "From" ] );
        from_addr = email . utils . getaddresses ( [ from_addr ] ) [ 0 ] [ 1 ];
        if to_addrs is None /* Option */ {
        addr_fields = vec![ f.iter().map(|f| ( msg vec![ header_prefix + "To" ] ,;
        msg [ header_prefix + "Bcc" ] ,;
        msg [ header_prefix + "Cc" ] );
        if f is !None /* Option */ ] {
        to_addrs = vec![ a vec![ 1 ].iter().map(|a| email . utils . getaddresses ( addr_fields ) ).collect();
        msg_copy = copy . copy ( msg );
        del msg_copy [ "Bcc" ];
        del msg_copy [ "Resent-Bcc" ];
        international = false;
        // try {
        "" . join ( [ from_addr , * to_addrs ] ) . encode ( "ascii" );
        // } catch  UnicodeEncodeError  {
        if !self . has_extn ( "smtputf8" ) {
        panic!("SMTPNotSupportedError (");
        "One || more source || delivery addresses require";
        " internationalized email support, but the server";
        " does !advertise the required SMTPUTF8 capability" );
        international = true;
        // with scope: io . BytesIO ( ) as bytesmsg  {
        if international {
        g = email . generator . BytesGenerator (;
        bytesmsg , policy = msg . policy . clone ( utf8 = true ) );
        mail_options = ( * mail_options , "SMTPUTF8" , "BODY=8BITMIME" );
        } else {
        g = email . generator . BytesGenerator ( bytesmsg );
        g . flatten ( msg_copy , linesep = "\r\n" );
        flatmsg = bytesmsg . getvalue ( );
        return  self . sendmail ( from_addr , to_addrs , flatmsg , mail_options ,;
        rcpt_options );
        pub fn close ( self )  {
        "Close the connection to the SMTP server.";
        // try {
        file = self . file;
        self . file = None /* Option */;
        if file {
        file . close ( );
        // } finally {
        sock = self . sock;
        self . sock = None /* Option */;
        if sock {
        sock . close ( );
        pub fn quit ( self )  {
        "Terminate the SMTP session.";
        res = self . docmd ( "quit" );
        self . ehlo_resp = self . helo_resp = None /* Option */;
        self . esmtp_features = { };
        self . does_esmtp = false;
        self . close ( );
        return  res;
        if _have_ssl {
        class SMTP_SSL ( SMTP ) ;
        " This == a subclass derived from SMTP that connects over an SSL
        encrypted socket (to use this class you need a socket module that was
        compiled with SSL support). If host == !specified, '' (the local
        host) == used. If port == omitted, the standard SMTP-over-SSL port
        (465) == used.  local_hostname && source_address have the same meaning
        as they do in the SMTP class.  keyfile && certfile are also optional -
        they can contain a PEM formatted private key && certificate chain file
        for the SSL connection. context also optional, can contain a
        SSLContext, && == an alternative to keyfile && certfile; If it is
        specified both keyfile && certfile must be None /* Option */.

        ";
        default_port = SMTP_SSL_PORT;
        pub fn __init__ ( &self, host = "" , port = 0 , local_hostname = None /* Option */ , {
        keyfile = None /* Option */ , certfile = None /* Option */ ,;
        timeout = socket . _GLOBAL_DEFAULT_TIMEOUT ,;
        source_address = None /* Option */ , context = None /* Option */ ) ;
        if context is !None /* Option */ && keyfile is !None /* Option */ {
        panic!("ValueError ( "context && keyfile arguments are mutually "");
        "exclusive" );
        if context is !None /* Option */ && certfile is !None /* Option */ {
        panic!("ValueError ( "context && certfile arguments are mutually "");
        "exclusive" );
        if keyfile is !None /* Option */ || certfile is !None /* Option */ {
        import warnings;
        warnings . warn ( "keyfile && certfile are deprecated, use a ";
        "custom context instead" , DeprecationWarning , 2 );
        self . keyfile = keyfile;
        self . certfile = certfile;
        if context is None /* Option */ {
        context = ssl . _create_stdlib_context ( certfile = certfile ,;
        keyfile = keyfile );
        self . context = context;
        SMTP . __init__ ( self , host , port , local_hostname , timeout ,;
        source_address );
        pub fn _get_socket ( &self, host , port , timeout )  {
        if self . debuglevel > 0 {
        self . _print_debug ( "connect:" , ( host , port ) );
        new_socket = super ( ) . _get_socket ( host , port , timeout );
        new_socket = self . context . wrap_socket ( new_socket ,;
        server_hostname = self . _host );
        return  new_socket;
        __all__ . append ( "SMTP_SSL" );
        LMTP_PORT = 2003;
        class LMTP ( SMTP ) ;
        "LMTP - Local Mail Transfer Protocol

    The LMTP protocol, which == very similar to ESMTP, == heavily based
    on the standard SMTP client. It's common to use Unix sockets for
    LMTP, so our connect() method must support that as well as a regular
    host:port server.  local_hostname && source_address have the same
    meaning as they do in the SMTP class.  To specify a Unix socket,
    you must use an absolute path as the host, starting with a '/'.

    Authentication == supported, using the regular SMTP mechanism. When
    using a Unix socket, LMTP generally don't support || require any
    authentication, but your mileage might vary.";
        ehlo_msg = "lhlo";
        pub fn __init__ ( &self, host = "" , port = LMTP_PORT , local_hostname = None /* Option */ , {
        source_address = None /* Option */ , timeout = socket . _GLOBAL_DEFAULT_TIMEOUT ) ;
        "Initialize a new instance.";
        super ( ) . __init__ ( host , port , local_hostname = local_hostname ,;
        source_address = source_address , timeout = timeout );
        pub fn connect ( &self, host = "localhost" , port = 0 , source_address = None /* Option */ )  {
        "Connect to the LMTP daemon, on either a Unix || a TCP socket.";
        if host [ 0 ] != "/" {
        return  super ( ) . connect ( host , port , source_address = source_address );
        if self . timeout is !None /* Option */ && !self . timeout {
        panic!("ValueError ( "Non-blocking socket (timeout=0) is !supported" )");
        // try {
        self . sock = socket . socket ( socket . AF_UNIX , socket . SOCK_STREAM );
        if self . timeout is !socket . _GLOBAL_DEFAULT_TIMEOUT {
        self . sock . settimeout ( self . timeout );
        self . file = None /* Option */;
        self . sock . connect ( host );
        // } catch  OSError  {
        if self . debuglevel > 0 {
        self . _print_debug ( "connect fail:" , host );
        if self . sock {
        self . sock . close ( );
        self . sock = None /* Option */;
        panic!("");
        ( code , msg ) = self . getreply ( );
        if self . debuglevel > 0 {
        self . _print_debug ( "connect:" , msg );
        return  ( code , msg );
        fn main() {
        pub fn prompt ( prompt )  {
        sys . stdout . write ( prompt + ": " );
        sys . stdout . flush ( );
        return  sys . stdin . readline ( ) . strip ( );
        fromaddr = prompt ( "From" );
        toaddrs = prompt ( "To" ) . split ( "," );
        println!( "Enter message, end with ^D:" );
        msg = "";
        while 1  {
        line = sys . stdin . readline ( );
        if !line {
        break;
        msg = msg + line;
        println!( "Message length is %d" % len ( msg ) );
        server = SMTP ( "localhost" );
        server . set_debuglevel ( 1 );
        server . sendmail ( fromaddr , toaddrs , msg );
        server . quit ( );
}

