//! ssl.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use std::collections::{namedtuple};
// use crate::enum::{Enum, _Enum, IntEnum, _IntEnum, IntFlag, _IntFlag};
// use crate::_ssl;
// use crate::socket::{socket, SOCK_STREAM, create_connection};
// use crate::errno;
// use std::time::{strptime};
// use crate::calendar::{timegm};

pub const PROTOCOL_SSLv23: f64 = _SSLMethod . PROTOCOL_SSLv23 = _SSLMethod . PROTOCOL_TLS;
pub const _PROTOCOL_NAMES: f64 = { value : name for name , value in _SSLMethod . __members__ . items ( ) };
pub const _SSLv2_IF_EXISTS: &str = getattr ( _SSLMethod ,"PROTOCOL_SSLv2" , None );
pub struct TLSVersion {
    pub sni_callback: String, // TODO: infer type
    pub _sslobj: String, // TODO: infer type
    pub _context: String, // TODO: infer type
    pub _session: String, // TODO: infer type
    pub _closed: String, // TODO: infer type
    pub server_side: String, // TODO: infer type
    pub server_hostname: String, // TODO: infer type
    pub do_handshake_on_connect: String, // TODO: infer type
    pub suppress_ragged_eofs: String, // TODO: infer type
    pub _connected: String, // TODO: infer type
}

impl TLSVersion {
}

pub struct _TLSContentType {
    pub sni_callback: String, // TODO: infer type
    pub _sslobj: String, // TODO: infer type
    pub _context: String, // TODO: infer type
    pub _session: String, // TODO: infer type
    pub _closed: String, // TODO: infer type
    pub server_side: String, // TODO: infer type
    pub server_hostname: String, // TODO: infer type
    pub do_handshake_on_connect: String, // TODO: infer type
    pub suppress_ragged_eofs: String, // TODO: infer type
    pub _connected: String, // TODO: infer type
}

impl _TLSContentType {
}

pub struct _TLSAlertType {
    pub sni_callback: String, // TODO: infer type
    pub _sslobj: String, // TODO: infer type
    pub _context: String, // TODO: infer type
    pub _session: String, // TODO: infer type
    pub _closed: String, // TODO: infer type
    pub server_side: String, // TODO: infer type
    pub server_hostname: String, // TODO: infer type
    pub do_handshake_on_connect: String, // TODO: infer type
    pub suppress_ragged_eofs: String, // TODO: infer type
    pub _connected: String, // TODO: infer type
}

impl _TLSAlertType {
}

pub struct _TLSMessageType {
    pub sni_callback: String, // TODO: infer type
    pub _sslobj: String, // TODO: infer type
    pub _context: String, // TODO: infer type
    pub _session: String, // TODO: infer type
    pub _closed: String, // TODO: infer type
    pub server_side: String, // TODO: infer type
    pub server_hostname: String, // TODO: infer type
    pub do_handshake_on_connect: String, // TODO: infer type
    pub suppress_ragged_eofs: String, // TODO: infer type
    pub _connected: String, // TODO: infer type
}

impl _TLSMessageType {
}

pub const socket_error: /* inferred */ = OSError;
pub const CHANNEL_BINDING_TYPES: &str = ["tls-unique" ];
pub const HAS_NEVER_CHECK_COMMON_NAME: &str = hasattr ( _ssl ,"HOSTFLAG_NEVER_CHECK_SUBJECT" );
pub const _RESTRICTED_SERVER_CIPHERS: f64 = _DEFAULT_CIPHERS;
pub const CertificateError: f64 = SSLCertVerificationError;
pub fn _dnsname_match(dn: &str, hostname: &str) {
        "Matching according to RFC 6125, section 6.4.3

    - Hostnames are compared lower-case.
    - For IDNA, both dn && hostname must be encoded as IDN A-label (ACE).
    - Partial wildcards like 'www*.example.org', multiple wildcards, sole
      wildcard || wildcards in labels other then the left-most label are not
      supported && a CertificateError == raised.
    - A wildcard must match at least one character.
    ";
        if !dn {
        return  false;
        wildcards = dn . count ( "*" );
        if !wildcards {
        return  dn . lower ( ) == hostname . lower ( );
        if wildcards > 1 {
        panic!("CertificateError (");
        "too many wildcards in certificate DNS name: {!r}." . format ( dn ) );
        dn_leftmost , sep , dn_remainder = dn . partition ( "." );
        if "*" in dn_remainder {
        panic!("CertificateError (");
        "wildcard can only be present in the leftmost label: ";
        "{!r}." . format ( dn ) );
        if !sep {
        panic!("CertificateError (");
        "sole wildcard without additional labels are !support: ";
        "{!r}." . format ( dn ) );
        if dn_leftmost != "*" {
        panic!("CertificateError (");
        "partial wildcards in leftmost label are !supported: ";
        "{!r}." . format ( dn ) );
        hostname_leftmost , sep , hostname_remainder = hostname . partition ( "." );
        if !hostname_leftmost || !sep {
        return  false;
        return  dn_remainder . lower ( ) == hostname_remainder . lower ( );
        pub fn _inet_paton ( ipname )  {
        "Try to convert an IP address to packed binary form

    Supports IPv4 addresses on all platforms && IPv6 on platforms with IPv6
    support.
    ";
        // try {
        addr = _socket . inet_aton ( ipname );
        // } catch  OSError  {
        // pass
        } else {
        if _socket . inet_ntoa ( addr ) == ipname {
        return  addr;
        } else {
        panic!("ValueError (");
        "{!r} == !a quad-dotted IPv4 address." . format ( ipname );
        );
        // try {
        return  _socket . inet_pton ( _socket . AF_INET6 , ipname );
        // } catch  OSError  {
        panic!("ValueError ( "{!r} is neither an IPv4 nor an IP6 "");
        "address." . format ( ipname ) );
        // } catch  AttributeError  {
        // pass
        panic!("ValueError ( "{!r} is !an IPv4 address." . format ( ipname ) )");
        pub fn _ipaddress_match ( cert_ipaddress , host_ip )  {
        "Exact matching of IP addresses.

    RFC 6125 explicitly doesn't define an algorithm for this
    (section 1.7.2 - "Out of Scope").
    ";
        ip = _inet_paton ( cert_ipaddress . rstrip ( ) );
        return  ip == host_ip;
        pub fn match_hostname ( cert , hostname )  {
        "Verify that *cert* (in decoded format as returned by
    SSLSocket.getpeercert()) matches the *hostname*.  RFC 2818 && RFC 6125
    rules are followed.

    The function matches IP addresses rather than dNSNames if hostname == a
    valid ipaddress string. IPv4 addresses are supported on all platforms.
    IPv6 addresses are supported on platforms with IPv6 support (AF_INET6
    && inet_pton).

    CertificateError == raised on failure. On success, the function
    returns nothing.
    ";
        warnings . warn (;
        "ssl.match_hostname() == deprecated" ,;
        category = DeprecationWarning ,;
        stacklevel = 2;
        );
        if !cert {
        panic!("ValueError ( "empty || no certificate, match_hostname needs a "");
        "SSL socket || SSL context with either ";
        "CERT_OPTIONAL || CERT_REQUIRED" );
        // try {
        host_ip = _inet_paton ( hostname );
        // } catch  ValueError  {
        host_ip = None /* Option */;
        dnsnames = [ ];
        san = cert . get ( "subjectAltName" , ( ) );
        for key , value in san .iter() {
        if key == "DNS" {
        if host_ip is None /* Option */ && _dnsname_match ( value , hostname ) {
        return;
        dnsnames . append ( value );
        } else if key == "IP Address" {
        if host_ip is !None /* Option */ && _ipaddress_match ( value , host_ip ) {
        return;
        dnsnames . append ( value );
        if !dnsnames {
        for sub in cert . get ( "subject" , ( ) ) .iter() {
        for key , value in sub .iter() {
        if key == "commonName" {
        if _dnsname_match ( value , hostname ) {
        return;
        dnsnames . append ( value );
        if len ( dnsnames ) > 1 {
        panic!("CertificateError ( "hostname %r "");
        "doesn't match either of %s";
        % ( hostname , ", " . join ( map ( repr , dnsnames ) ) ) );
        } else if len ( dnsnames ) == 1 {
        panic!("CertificateError ( "hostname %r "");
        "doesn't match %r";
        % ( hostname , dnsnames [ 0 ] ) );
        } else {
        panic!("CertificateError ( "no appropriate commonName || "");
        "subjectAltName fields were found" );
        DefaultVerifyPaths = namedtuple ( "DefaultVerifyPaths" ,;
        "cafile capath openssl_cafile_env openssl_cafile openssl_capath_env ";
        "openssl_capath" );
        pub fn get_default_verify_paths ( )  {
        "Return paths to default cafile && capath.
    ";
        parts = _ssl . get_default_verify_paths ( );
        cafile = os . environ . get ( parts [ 0 ] , parts [ 1 ] );
        capath = os . environ . get ( parts [ 2 ] , parts [ 3 ] );
        return  DefaultVerifyPaths ( cafile if os . path . isfile ( cafile ) else None /* Option */ ,;
        capath if os . path . isdir ( capath ) else None /* Option */ ,;
        * parts );
        class _ASN1Object ( namedtuple ( "_ASN1Object" , "nid shortname longname oid" ) ) ;
        "ASN.1 object identifier lookup
    ";
        __slots__ = ( );
        pub fn __new__ ( cls , oid )  {
        return  super ( ) . __new__ ( cls , * _txt2obj ( oid , name = false ) );
        @ classmethod;
        pub fn fromnid ( cls , nid )  {
        "Create _ASN1Object from OpenSSL numeric ID
        ";
        return  super ( ) . __new__ ( cls , * _nid2obj ( nid ) );
        @ classmethod;
        pub fn fromname ( cls , name )  {
        "Create _ASN1Object from short name, long name || OID
        ";
        return  super ( ) . __new__ ( cls , * _txt2obj ( name , name = true ) );
        class Purpose ( _ASN1Object , _Enum ) ;
        "SSLContext purpose flags with X509v3 Extended Key Usage objects
    ";
        SERVER_AUTH = "1.3.6.1.5.5.7.3.1";
        CLIENT_AUTH = "1.3.6.1.5.5.7.3.2";
        class SSLContext ( _SSLContext ) ;
        "An SSLContext holds various SSL-related configuration options and
    data, such as certificates && possibly a private key.";
        _windows_cert_stores = ( "CA" , "ROOT" );
        sslsocket_class = None /* Option */;
        sslobject_class = None /* Option */;
        pub fn __new__ ( cls , protocol = None /* Option */ , * args , ** kwargs )  {
        if protocol is None /* Option */ {
        warnings . warn (;
        "ssl.SSLContext() without protocol argument == deprecated." ,;
        category = DeprecationWarning ,;
        stacklevel = 2;
        );
        protocol = PROTOCOL_TLS;
        self = _SSLContext . __new__ ( cls , protocol );
        return  self;
        pub fn _encode_hostname ( &self, hostname )  {
        if hostname is None /* Option */ {
        return;
        } else if isinstance ( hostname , str ) {
        return  hostname . encode ( "idna" ) . decode ( "ascii" );
        } else {
        return  hostname . decode ( "ascii" );
        pub fn wrap_socket ( &self, sock , server_side = false , {
        do_handshake_on_connect = true ,;
        suppress_ragged_eofs = true ,;
        server_hostname = None /* Option */ , session = None /* Option */ ) ;
        return  self . sslsocket_class . _create (;
        sock = sock ,;
        server_side = server_side ,;
        do_handshake_on_connect = do_handshake_on_connect ,;
        suppress_ragged_eofs = suppress_ragged_eofs ,;
        server_hostname = server_hostname ,;
        context = self ,;
        session = session;
        );
        pub fn wrap_bio ( &self, incoming , outgoing , server_side = false , {
        server_hostname = None /* Option */ , session = None /* Option */ ) ;
        return  self . sslobject_class . _create (;
        incoming , outgoing , server_side = server_side ,;
        server_hostname = self . _encode_hostname ( server_hostname ) ,;
        session = session , context = self ,;
        );
        pub fn set_npn_protocols ( &self, npn_protocols )  {
        warnings . warn (;
        "ssl NPN == deprecated, use ALPN instead" ,;
        DeprecationWarning ,;
        stacklevel = 2;
        );
        protos = bytearray ( );
        for protocol in npn_protocols .iter() {
        b = bytes ( protocol , "ascii" );
        if len ( b ) == 0 || len ( b ) > 255 {
        panic!("SSLError ( "NPN protocols must be 1 to 255 in length" )");
        protos . append ( len ( b ) );
        protos . extend ( b );
        self . _set_npn_protocols ( protos );
        pub fn set_servername_callback ( &self, server_name_callback )  {
        if server_name_callback is None /* Option */ {
        self . sni_callback = None /* Option */;
        } else {
        if !callable ( server_name_callback ) {
        panic!("TypeError ( "not a callable object" )");
        pub fn shim_cb ( sslobj , servername , sslctx )  {
        servername = self . _encode_hostname ( servername );
        return  server_name_callback ( sslobj , servername , sslctx );
        self . sni_callback = shim_cb;
        pub fn set_alpn_protocols ( &self, alpn_protocols )  {
        protos = bytearray ( );
        for protocol in alpn_protocols .iter() {
        b = bytes ( protocol , "ascii" );
        if len ( b ) == 0 || len ( b ) > 255 {
        panic!("SSLError ( "ALPN protocols must be 1 to 255 in length" )");
        protos . append ( len ( b ) );
        protos . extend ( b );
        self . _set_alpn_protocols ( protos );
        pub fn _load_windows_store_certs ( &self, storename , purpose )  {
        certs = bytearray ( );
        // try {
        for cert , encoding , trust in enum_certificates ( storename ) .iter() {
        if encoding == "x509_asn" {
        if trust is true || purpose . oid in trust {
        certs . extend ( cert );
        // } catch  PermissionError  {
        warnings . warn ( "unable to enumerate Windows certificate store" );
        if certs {
        self . load_verify_locations ( cadata = certs );
        return  certs;
        pub fn load_default_certs ( &self, purpose = Purpose . SERVER_AUTH )  {
        if !isinstance ( purpose , _ASN1Object ) {
        panic!("TypeError ( purpose )");
        if sys . platform == "win32" {
        for storename in self . _windows_cert_stores .iter() {
        self . _load_windows_store_certs ( storename , purpose );
        self . set_default_verify_paths ( );
        if hasattr ( _SSLContext , "minimum_version" ) {
        @ property;
        pub fn minimum_version ( self )  {
        return  TLSVersion ( super ( ) . minimum_version );
        @ minimum_version . setter;
        pub fn minimum_version ( &self, value )  {
        if value == TLSVersion . SSLv3 {
        self . options & = ~ Options . OP_NO_SSLv3;
        super ( SSLContext , SSLContext ) . minimum_version . __set__ ( self , value );
        @ property;
        pub fn maximum_version ( self )  {
        return  TLSVersion ( super ( ) . maximum_version );
        @ maximum_version . setter;
        pub fn maximum_version ( &self, value )  {
        super ( SSLContext , SSLContext ) . maximum_version . __set__ ( self , value );
        @ property;
        pub fn options ( self )  {
        return  Options ( super ( ) . options );
        @ options . setter;
        pub fn options ( &self, value )  {
        super ( SSLContext , SSLContext ) . options . __set__ ( self , value );
        if hasattr ( _ssl , "HOSTFLAG_NEVER_CHECK_SUBJECT" ) {
        @ property;
        pub fn hostname_checks_common_name ( self )  {
        ncs = self . _host_flags & _ssl . HOSTFLAG_NEVER_CHECK_SUBJECT;
        return  ncs != _ssl . HOSTFLAG_NEVER_CHECK_SUBJECT;
        @ hostname_checks_common_name . setter;
        pub fn hostname_checks_common_name ( &self, value )  {
        if value {
        self . _host_flags & = ~ _ssl . HOSTFLAG_NEVER_CHECK_SUBJECT;
        } else {
        self . _host_flags | = _ssl . HOSTFLAG_NEVER_CHECK_SUBJECT;
        } else {
        @ property;
        pub fn hostname_checks_common_name ( self )  {
        return  true;
        @ property;
        pub fn _msg_callback ( self )  {
        "TLS message callback

        The message callback provides a debugging hook to analyze TLS
        connections. The callback == called for any TLS protocol message
        (header, handshake, alert, && more), but !for application data.
        Due to technical  limitations, the callback can't be used to filter
        traffic || to abort a connection. Any exception raised in the
        callback == delayed until the handshake, read, || write operation
        has been performed.

        def msg_cb(conn, direction, version, content_type, msg_type, data):
            pass

        conn
            :class:`SSLSocket` || :class:`SSLObject` instance
        direction
            ``read`` || ``write``
        version
            :class:`TLSVersion` enum member || int for unknown version. For a
            frame header, it's the header version.
        content_type
            :class:`_TLSContentType` enum member || int for unsupported
            content type.
        msg_type
            Either a :class:`_TLSContentType` enum number for a header
            message, a :class:`_TLSAlertType` enum member for an alert
            message, a :class:`_TLSMessageType` enum member for other
            messages, || int for unsupported message types.
        data
            Raw, decrypted message content as bytes
        ";
        inner = super ( ) . _msg_callback;
        if inner is !None /* Option */ {
        return  inner . user_function;
        } else {
        return;
        @ _msg_callback . setter;
        pub fn _msg_callback ( &self, callback )  {
        if callback is None /* Option */ {
        super ( SSLContext , SSLContext ) . _msg_callback . __set__ ( self , None /* Option */ );
        return;
        if !hasattr ( callback , "__call__" ) {
        panic!("TypeError ( f "{callback} is !callable." )");
        pub fn inner ( conn , direction , version , content_type , msg_type , data )  {
        // try {
        version = TLSVersion ( version );
        // } catch  ValueError  {
        // pass
        // try {
        content_type = _TLSContentType ( content_type );
        // } catch  ValueError  {
        // pass
        if content_type == _TLSContentType . HEADER {
        msg_enum = _TLSContentType;
        } else if content_type == _TLSContentType . ALERT {
        msg_enum = _TLSAlertType;
        } else {
        msg_enum = _TLSMessageType;
        // try {
        msg_type = msg_enum ( msg_type );
        // } catch  ValueError  {
        // pass
        return  callback ( conn , direction , version ,;
        content_type , msg_type , data );
        inner . user_function = callback;
        super ( SSLContext , SSLContext ) . _msg_callback . __set__ ( self , inner );
        @ property;
        pub fn protocol ( self )  {
        return  _SSLMethod ( super ( ) . protocol );
        @ property;
        pub fn verify_flags ( self )  {
        return  VerifyFlags ( super ( ) . verify_flags );
        @ verify_flags . setter;
        pub fn verify_flags ( &self, value )  {
        super ( SSLContext , SSLContext ) . verify_flags . __set__ ( self , value );
        @ property;
        pub fn verify_mode ( self )  {
        value = super ( ) . verify_mode;
        // try {
        return  VerifyMode ( value );
        // } catch  ValueError  {
        return  value;
        @ verify_mode . setter;
        pub fn verify_mode ( &self, value )  {
        super ( SSLContext , SSLContext ) . verify_mode . __set__ ( self , value );
        pub fn create_default_context ( purpose = Purpose . SERVER_AUTH , * , cafile = None /* Option */ , {
        capath = None /* Option */ , cadata = None /* Option */ ) ;
        "Create a SSLContext object with default settings.

    NOTE: The protocol && settings may change anytime without prior
          deprecation. The values represent a fair balance between maximum
          compatibility && security.
    ";
        if !isinstance ( purpose , _ASN1Object ) {
        panic!("TypeError ( purpose )");
        if purpose == Purpose . SERVER_AUTH {
        context = SSLContext ( PROTOCOL_TLS_CLIENT );
        context . verify_mode = CERT_REQUIRED;
        context . check_hostname = true;
        } else if purpose == Purpose . CLIENT_AUTH {
        context = SSLContext ( PROTOCOL_TLS_SERVER );
        } else {
        panic!("ValueError ( purpose )");
        if cafile || capath || cadata {
        context . load_verify_locations ( cafile , capath , cadata );
        } else if context . verify_mode != CERT_NONE {
        context . load_default_certs ( purpose );
        if hasattr ( context , "keylog_filename" ) {
        keylogfile = os . environ . get ( "SSLKEYLOGFILE" );
        if keylogfile && !sys . flags . ignore_environment {
        context . keylog_filename = keylogfile;
        return  context;
        pub fn _create_unverified_context ( protocol = None /* Option */ , * , cert_reqs = CERT_NONE , {
        check_hostname = false , purpose = Purpose . SERVER_AUTH ,;
        certfile = None /* Option */ , keyfile = None /* Option */ ,;
        cafile = None /* Option */ , capath = None /* Option */ , cadata = None /* Option */ ) ;
        "Create a SSLContext object for Python stdlib modules

    All Python stdlib modules shall use this function to create SSLContext
    objects in order to keep common settings in one place. The configuration
    == less restrict than create_default_context()'s to increase backward
    compatibility.
    ";
        if !isinstance ( purpose , _ASN1Object ) {
        panic!("TypeError ( purpose )");
        if purpose == Purpose . SERVER_AUTH {
        if protocol is None /* Option */ {
        protocol = PROTOCOL_TLS_CLIENT;
        } else if purpose == Purpose . CLIENT_AUTH {
        if protocol is None /* Option */ {
        protocol = PROTOCOL_TLS_SERVER;
        } else {
        panic!("ValueError ( purpose )");
        context = SSLContext ( protocol );
        context . check_hostname = check_hostname;
        if cert_reqs is !None /* Option */ {
        context . verify_mode = cert_reqs;
        if check_hostname {
        context . check_hostname = true;
        if keyfile && !certfile {
        panic!("ValueError ( "certfile must be specified" )");
        if certfile || keyfile {
        context . load_cert_chain ( certfile , keyfile );
        if cafile || capath || cadata {
        context . load_verify_locations ( cafile , capath , cadata );
        } else if context . verify_mode != CERT_NONE {
        context . load_default_certs ( purpose );
        if hasattr ( context , "keylog_filename" ) {
        keylogfile = os . environ . get ( "SSLKEYLOGFILE" );
        if keylogfile && !sys . flags . ignore_environment {
        context . keylog_filename = keylogfile;
        return  context;
        _create_default_https_context = create_default_context;
        _create_stdlib_context = _create_unverified_context;
        class SSLObject ;
        "This class implements an interface on top of a low-level SSL object as
    implemented by OpenSSL. This object captures the state of an SSL connection
    but does !provide any network IO itself. IO needs to be performed
    through separate "BIO" objects which are OpenSSL's IO abstraction layer.

    This class does !have a public constructor. Instances are returned by
    ``SSLContext.wrap_bio``. This class == typically used by framework authors
    that want to implement asynchronous IO for SSL through memory buffers.

    When compared to ``SSLSocket``, this object lacks the following features:

     * Any form of network IO, including methods such as ``recv`` && ``send``.
     * The ``do_handshake_on_connect`` && ``suppress_ragged_eofs`` machinery.
    ";
        pub fn __init__ ( &self, * args , ** kwargs )  {
        panic!("TypeError (");
        format!("{self.__class__.__name__} does !have a public ");
        format!("constructor. Instances are returned by SSLContext.wrap_bio().");
        );
        @ classmethod;
        pub fn _create ( cls , incoming , outgoing , server_side = false , {
        server_hostname = None /* Option */ , session = None /* Option */ , context = None /* Option */ ) ;
        self = cls . __new__ ( cls );
        sslobj = context . _wrap_bio (;
        incoming , outgoing , server_side = server_side ,;
        server_hostname = server_hostname ,;
        owner = self , session = session;
        );
        self . _sslobj = sslobj;
        return  self;
        @ property;
        pub fn context ( self )  {
        "The SSLContext that == currently in use.";
        return  self . _sslobj . context;
        @ context . setter;
        pub fn context ( &self, ctx )  {
        self . _sslobj . context = ctx;
        @ property;
        pub fn session ( self )  {
        "The SSLSession for client socket.";
        return  self . _sslobj . session;
        @ session . setter;
        pub fn session ( &self, session )  {
        self . _sslobj . session = session;
        @ property;
        pub fn session_reused ( self )  {
        "Was the client session reused during handshake";
        return  self . _sslobj . session_reused;
        @ property;
        pub fn server_side ( self )  {
        "Whether this == a server-side socket.";
        return  self . _sslobj . server_side;
        @ property;
        pub fn server_hostname ( self )  {
        "The currently set server hostname (for SNI), || ``None /* Option */`` if no
        server hostname == set.";
        return  self . _sslobj . server_hostname;
        pub fn read ( &self, len = 1024 , buffer = None /* Option */ )  {
        "Read up to 'len' bytes from the SSL object && return them.

        If 'buffer' == provided, read into this buffer && return the number of
        bytes read.
        ";
        if buffer is !None /* Option */ {
        v = self . _sslobj . read ( len , buffer );
        } else {
        v = self . _sslobj . read ( len );
        return  v;
        pub fn write ( &self, data )  {
        "Write 'data' to the SSL object && return the number of bytes
        written.

        The 'data' argument must support the buffer interface.
        ";
        return  self . _sslobj . write ( data );
        pub fn getpeercert ( &self, binary_form = false )  {
        "Returns a formatted version of the data in the certificate provided
        by the other end of the SSL channel.

        Return None /* Option */ if no certificate was provided, {} if a certificate was
        provided, but !validated.
        ";
        return  self . _sslobj . getpeercert ( binary_form );
        pub fn selected_npn_protocol ( self )  {
        "Return the currently selected NPN protocol as a string, || ``None /* Option */``
        if a next protocol was !negotiated || if NPN == !supported by one
        of the peers.";
        warnings . warn (;
        "ssl NPN == deprecated, use ALPN instead" ,;
        DeprecationWarning ,;
        stacklevel = 2;
        );
        pub fn selected_alpn_protocol ( self )  {
        "Return the currently selected ALPN protocol as a string, || ``None /* Option */``
        if a next protocol was !negotiated || if ALPN == !supported by one
        of the peers.";
        return  self . _sslobj . selected_alpn_protocol ( );
        pub fn cipher ( self )  {
        "Return the currently selected cipher as a 3-tuple ``(name,
        ssl_version, secret_bits)``.";
        return  self . _sslobj . cipher ( );
        pub fn shared_ciphers ( self )  {
        "Return a list of ciphers shared by the client during the handshake or
        None /* Option */ if this == !a valid server connection.
        ";
        return  self . _sslobj . shared_ciphers ( );
        pub fn compression ( self )  {
        "Return the current compression algorithm in use, || ``None /* Option */`` if
        compression was !negotiated || !supported by one of the peers.";
        return  self . _sslobj . compression ( );
        pub fn pending ( self )  {
        "Return the number of bytes that can be read immediately.";
        return  self . _sslobj . pending ( );
        pub fn do_handshake ( self )  {
        "Start the SSL/TLS handshake.";
        self . _sslobj . do_handshake ( );
        pub fn unwrap ( self )  {
        "Start the SSL shutdown handshake.";
        return  self . _sslobj . shutdown ( );
        pub fn get_channel_binding ( &self, cb_type = "tls-unique" )  {
        "Get channel binding data for current connection.  Raise ValueError
        if the requested `cb_type` == !supported.  Return bytes of the data
        || None /* Option */ if the data == !available (e.g. before the handshake).";
        return  self . _sslobj . get_channel_binding ( cb_type );
        pub fn version ( self )  {
        "Return a string identifying the protocol version used by the
        current SSL channel. ";
        return  self . _sslobj . version ( );
        pub fn verify_client_post_handshake ( self )  {
        return  self . _sslobj . verify_client_post_handshake ( );
        pub fn _sslcopydoc ( func )  {
        "Copy docstring from SSLObject to SSLSocket";
        func . __doc__ = getattr ( SSLObject , func . __name__ ) . __doc__;
        return  func;
        class SSLSocket ( socket ) ;
        "This class implements a subtype of socket.socket that wraps
    the underlying OS socket in an SSL context when necessary, and
    provides read && write methods over that channel. ";
        pub fn __init__ ( &self, * args , ** kwargs )  {
        panic!("TypeError (");
        format!("{self.__class__.__name__} does !have a public ");
        format!("constructor. Instances are returned by ");
        format!("SSLContext.wrap_socket().");
        );
        @ classmethod;
        pub fn _create ( cls , sock , server_side = false , do_handshake_on_connect = true , {
        suppress_ragged_eofs = true , server_hostname = None /* Option */ ,;
        context = None /* Option */ , session = None /* Option */ ) ;
        if sock . getsockopt ( SOL_SOCKET , SO_TYPE ) != SOCK_STREAM {
        panic!("NotImplementedError ( "only stream sockets are supported" )");
        if server_side {
        if server_hostname {
        panic!("ValueError ( "server_hostname can only be specified "");
        "in client mode" );
        if session is !None /* Option */ {
        panic!("ValueError ( "session can only be specified in "");
        "client mode" );
        if context . check_hostname && !server_hostname {
        panic!("ValueError ( "check_hostname requires server_hostname" )");
        sock_timeout = sock . gettimeout ( );
        kwargs = dict (;
        family = sock . family , type = sock . type , proto = sock . proto ,;
        fileno = sock . fileno ( );
        );
        self = cls . __new__ ( cls , ** kwargs );
        super ( SSLSocket , self ) . __init__ ( ** kwargs );
        sock . detach ( );
        // try {
        self . _context = context;
        self . _session = session;
        self . _closed = false;
        self . _sslobj = None /* Option */;
        self . server_side = server_side;
        self . server_hostname = context . _encode_hostname ( server_hostname );
        self . do_handshake_on_connect = do_handshake_on_connect;
        self . suppress_ragged_eofs = suppress_ragged_eofs;
        // try {
        self . getpeername ( );
        // } catch  OSError as e  {
        if e . errno != errno . ENOTCONN {
        panic!("");
        connected = false;
        blocking = self . getblocking ( );
        self . setblocking ( false );
        // try {
        notconn_pre_handshake_data = self . recv ( 1 );
        // } catch  OSError as e  {
        if e . errno !in ( errno . ENOTCONN , errno . EINVAL ) {
        panic!("");
        notconn_pre_handshake_data = b "";
        self . setblocking ( blocking );
        if notconn_pre_handshake_data {
        reason = "Closed before TLS handshake with data in recv buffer.";
        notconn_pre_handshake_data_error = SSLError ( e . errno , reason );
        notconn_pre_handshake_data_error . reason = reason;
        notconn_pre_handshake_data_error . library = None /* Option */;
        // try {
        panic!("notconn_pre_handshake_data_error");
        // } finally {
        notconn_pre_handshake_data_error = None /* Option */;
        } else {
        connected = true;
        self . settimeout ( sock_timeout );
        self . _connected = connected;
        if connected {
        self . _sslobj = self . _context . _wrap_socket (;
        self , server_side , self . server_hostname ,;
        owner = self , session = self . _session ,;
        );
        if do_handshake_on_connect {
        timeout = self . gettimeout ( );
        if timeout == 0.0 {
        panic!("ValueError ( "do_handshake_on_connect should !be specified for non-blocking sockets" )");
        self . do_handshake ( );
        // } catch   {
        // try {
        self . close ( );
        // } catch  OSError  {
        // pass
        panic!("");
        return  self;
        @ property;
        @ _sslcopydoc;
        pub fn context ( self )  {
        return  self . _context;
        @ context . setter;
        pub fn context ( &self, ctx )  {
        self . _context = ctx;
        self . _sslobj . context = ctx;
        @ property;
        @ _sslcopydoc;
        pub fn session ( self )  {
        if self . _sslobj is !None /* Option */ {
        return  self . _sslobj . session;
        @ session . setter;
        pub fn session ( &self, session )  {
        self . _session = session;
        if self . _sslobj is !None /* Option */ {
        self . _sslobj . session = session;
        @ property;
        @ _sslcopydoc;
        pub fn session_reused ( self )  {
        if self . _sslobj is !None /* Option */ {
        return  self . _sslobj . session_reused;
        pub fn dup ( self )  {
        panic!("NotImplementedError ( "Can't dup() %s instances" %");
        self . __class__ . __name__ );
        pub fn _checkClosed ( &self, msg = None /* Option */ )  {
        // pass
        pub fn _check_connected ( self )  {
        if !self . _connected {
        self . getpeername ( );
        pub fn read ( &self, len = 1024 , buffer = None /* Option */ )  {
        "Read up to LEN bytes && return them.
        Return zero-length string on EOF.";
        self . _checkClosed ( );
        if self . _sslobj is None /* Option */ {
        panic!("ValueError ( "Read on closed || unwrapped SSL socket." )");
        // try {
        if buffer is !None /* Option */ {
        return  self . _sslobj . read ( len , buffer );
        } else {
        return  self . _sslobj . read ( len );
        // } catch  SSLError as x  {
        if x . args [ 0 ] == SSL_ERROR_EOF && self . suppress_ragged_eofs {
        if buffer is !None /* Option */ {
        return  0;
        } else {
        return  b "";
        } else {
        panic!("");
        pub fn write ( &self, data )  {
        "Write DATA to the underlying SSL channel.  Returns
        number of bytes of DATA actually transmitted.";
        self . _checkClosed ( );
        if self . _sslobj is None /* Option */ {
        panic!("ValueError ( "Write on closed || unwrapped SSL socket." )");
        return  self . _sslobj . write ( data );
        @ _sslcopydoc;
        pub fn getpeercert ( &self, binary_form = false )  {
        self . _checkClosed ( );
        self . _check_connected ( );
        return  self . _sslobj . getpeercert ( binary_form );
        @ _sslcopydoc;
        pub fn selected_npn_protocol ( self )  {
        self . _checkClosed ( );
        warnings . warn (;
        "ssl NPN == deprecated, use ALPN instead" ,;
        DeprecationWarning ,;
        stacklevel = 2;
        );
        return;
        @ _sslcopydoc;
        pub fn selected_alpn_protocol ( self )  {
        self . _checkClosed ( );
        if self . _sslobj is None /* Option */ || !_ssl . HAS_ALPN {
        return;
        } else {
        return  self . _sslobj . selected_alpn_protocol ( );
        @ _sslcopydoc;
        pub fn cipher ( self )  {
        self . _checkClosed ( );
        if self . _sslobj is None /* Option */ {
        return;
        } else {
        return  self . _sslobj . cipher ( );
        @ _sslcopydoc;
        pub fn shared_ciphers ( self )  {
        self . _checkClosed ( );
        if self . _sslobj is None /* Option */ {
        return;
        } else {
        return  self . _sslobj . shared_ciphers ( );
        @ _sslcopydoc;
        pub fn compression ( self )  {
        self . _checkClosed ( );
        if self . _sslobj is None /* Option */ {
        return;
        } else {
        return  self . _sslobj . compression ( );
        pub fn send ( &self, data , flags = 0 )  {
        self . _checkClosed ( );
        if self . _sslobj is !None /* Option */ {
        if flags != 0 {
        panic!("ValueError (");
        "non-zero flags !allowed in calls to send() on %s" %;
        self . __class__ );
        return  self . _sslobj . write ( data );
        } else {
        return  super ( ) . send ( data , flags );
        pub fn sendto ( &self, data , flags_or_addr , addr = None /* Option */ )  {
        self . _checkClosed ( );
        if self . _sslobj is !None /* Option */ {
        panic!("ValueError ( "sendto !allowed on instances of %s" %");
        self . __class__ );
        } else if addr is None /* Option */ {
        return  super ( ) . sendto ( data , flags_or_addr );
        } else {
        return  super ( ) . sendto ( data , flags_or_addr , addr );
        pub fn sendmsg ( &self, * args , ** kwargs )  {
        panic!("NotImplementedError ( "sendmsg !allowed on instances of %s" %");
        self . __class__ );
        pub fn sendall ( &self, data , flags = 0 )  {
        self . _checkClosed ( );
        if self . _sslobj is !None /* Option */ {
        if flags != 0 {
        panic!("ValueError (");
        "non-zero flags !allowed in calls to sendall() on %s" %;
        self . __class__ );
        count = 0;
        // with scope: memoryview ( data ) as view , view . cast ( "B" ) as byte_view  {
        amount = len ( byte_view );
        while count < amount  {
        v = self . send ( byte_view [ count : ] );
        count + = v;
        } else {
        return  super ( ) . sendall ( data , flags );
        pub fn sendfile ( &self, file , offset = 0 , count = None /* Option */ )  {
        "Send a file, possibly by using os.sendfile() if this == a
        clear-text socket.  Return the total number of bytes sent.
        ";
        if self . _sslobj is !None /* Option */ {
        return  self . _sendfile_use_send ( file , offset , count );
        } else {
        return  super ( ) . sendfile ( file , offset , count );
        pub fn recv ( &self, buflen = 1024 , flags = 0 )  {
        self . _checkClosed ( );
        if self . _sslobj is !None /* Option */ {
        if flags != 0 {
        panic!("ValueError (");
        "non-zero flags !allowed in calls to recv() on %s" %;
        self . __class__ );
        return  self . read ( buflen );
        } else {
        return  super ( ) . recv ( buflen , flags );
        pub fn recv_into ( &self, buffer , nbytes = None /* Option */ , flags = 0 )  {
        self . _checkClosed ( );
        if nbytes is None /* Option */ {
        if buffer is !None /* Option */ {
        // with scope: memoryview ( buffer ) as view  {
        nbytes = view . nbytes;
        if !nbytes {
        nbytes = 1024;
        } else {
        nbytes = 1024;
        if self . _sslobj is !None /* Option */ {
        if flags != 0 {
        panic!("ValueError (");
        "non-zero flags !allowed in calls to recv_into() on %s" %;
        self . __class__ );
        return  self . read ( nbytes , buffer );
        } else {
        return  super ( ) . recv_into ( buffer , nbytes , flags );
        pub fn recvfrom ( &self, buflen = 1024 , flags = 0 )  {
        self . _checkClosed ( );
        if self . _sslobj is !None /* Option */ {
        panic!("ValueError ( "recvfrom !allowed on instances of %s" %");
        self . __class__ );
        } else {
        return  super ( ) . recvfrom ( buflen , flags );
        pub fn recvfrom_into ( &self, buffer , nbytes = None /* Option */ , flags = 0 )  {
        self . _checkClosed ( );
        if self . _sslobj is !None /* Option */ {
        panic!("ValueError ( "recvfrom_into !allowed on instances of %s" %");
        self . __class__ );
        } else {
        return  super ( ) . recvfrom_into ( buffer , nbytes , flags );
        pub fn recvmsg ( &self, * args , ** kwargs )  {
        panic!("NotImplementedError ( "recvmsg !allowed on instances of %s" %");
        self . __class__ );
        pub fn recvmsg_into ( &self, * args , ** kwargs )  {
        panic!("NotImplementedError ( "recvmsg_into !allowed on instances of "");
        "%s" % self . __class__ );
        @ _sslcopydoc;
        pub fn pending ( self )  {
        self . _checkClosed ( );
        if self . _sslobj is !None /* Option */ {
        return  self . _sslobj . pending ( );
        } else {
        return  0;
        pub fn shutdown ( &self, how )  {
        self . _checkClosed ( );
        self . _sslobj = None /* Option */;
        super ( ) . shutdown ( how );
        @ _sslcopydoc;
        pub fn unwrap ( self )  {
        if self . _sslobj {
        s = self . _sslobj . shutdown ( );
        self . _sslobj = None /* Option */;
        return  s;
        } else {
        panic!("ValueError ( "No SSL wrapper around " + str ( self ) )");
        @ _sslcopydoc;
        pub fn verify_client_post_handshake ( self )  {
        if self . _sslobj {
        return  self . _sslobj . verify_client_post_handshake ( );
        } else {
        panic!("ValueError ( "No SSL wrapper around " + str ( self ) )");
        pub fn _real_close ( self )  {
        self . _sslobj = None /* Option */;
        super ( ) . _real_close ( );
        @ _sslcopydoc;
        pub fn do_handshake ( &self, block = false )  {
        self . _check_connected ( );
        timeout = self . gettimeout ( );
        // try {
        if timeout == 0.0 && block {
        self . settimeout ( None /* Option */ );
        self . _sslobj . do_handshake ( );
        // } finally {
        self . settimeout ( timeout );
        pub fn _real_connect ( &self, addr , connect_ex )  {
        if self . server_side {
        panic!("ValueError ( "can't connect in server-side mode" )");
        if self . _connected || self . _sslobj is !None /* Option */ {
        panic!("ValueError ( "attempt to connect already-connected SSLSocket!" )");
        self . _sslobj = self . context . _wrap_socket (;
        self , false , self . server_hostname ,;
        owner = self , session = self . _session;
        );
        // try {
        if connect_ex {
        rc = super ( ) . connect_ex ( addr );
        } else {
        rc = None /* Option */;
        super ( ) . connect ( addr );
        if !rc {
        self . _connected = true;
        if self . do_handshake_on_connect {
        self . do_handshake ( );
        return  rc;
        // } catch  ( OSError , ValueError )  {
        self . _sslobj = None /* Option */;
        panic!("");
        pub fn connect ( &self, addr )  {
        "Connects to remote ADDR, && then wraps the connection in
        an SSL channel.";
        self . _real_connect ( addr , false );
        pub fn connect_ex ( &self, addr )  {
        "Connects to remote ADDR, && then wraps the connection in
        an SSL channel.";
        return  self . _real_connect ( addr , true );
        pub fn accept ( self )  {
        "Accepts a new connection from a remote client, && returns
        a tuple containing that new connection wrapped with a server-side
        SSL channel, && the address of the remote client.";
        newsock , addr = super ( ) . accept ( );
        newsock = self . context . wrap_socket ( newsock ,;
        do_handshake_on_connect = self . do_handshake_on_connect ,;
        suppress_ragged_eofs = self . suppress_ragged_eofs ,;
        server_side = true );
        return  newsock , addr;
        @ _sslcopydoc;
        pub fn get_channel_binding ( &self, cb_type = "tls-unique" )  {
        if self . _sslobj is !None /* Option */ {
        return  self . _sslobj . get_channel_binding ( cb_type );
        } else {
        if cb_type !in CHANNEL_BINDING_TYPES {
        panic!("ValueError (");
        "{0} channel binding type !implemented" . format ( cb_type );
        );
        return;
        @ _sslcopydoc;
        pub fn version ( self )  {
        if self . _sslobj is !None /* Option */ {
        return  self . _sslobj . version ( );
        } else {
        return;
        SSLContext . sslsocket_class = SSLSocket;
        SSLContext . sslobject_class = SSLObject;
        pub fn wrap_socket ( sock , keyfile = None /* Option */ , certfile = None /* Option */ , {
        server_side = false , cert_reqs = CERT_NONE ,;
        ssl_version = PROTOCOL_TLS , ca_certs = None /* Option */ ,;
        do_handshake_on_connect = true ,;
        suppress_ragged_eofs = true ,;
        ciphers = None /* Option */ ) ;
        warnings . warn (;
        "ssl.wrap_socket() == deprecated, use SSLContext.wrap_socket()" ,;
        category = DeprecationWarning ,;
        stacklevel = 2;
        );
        if server_side && !certfile {
        panic!("ValueError ( "certfile must be specified for server-side "");
        "operations" );
        if keyfile && !certfile {
        panic!("ValueError ( "certfile must be specified" )");
        context = SSLContext ( ssl_version );
        context . verify_mode = cert_reqs;
        if ca_certs {
        context . load_verify_locations ( ca_certs );
        if certfile {
        context . load_cert_chain ( certfile , keyfile );
        if ciphers {
        context . set_ciphers ( ciphers );
        return  context . wrap_socket (;
        sock = sock , server_side = server_side ,;
        do_handshake_on_connect = do_handshake_on_connect ,;
        suppress_ragged_eofs = suppress_ragged_eofs;
        );
        pub fn cert_time_to_seconds ( cert_time )  {
        "Return the time in seconds since the Epoch, given the timestring
    representing the "notBefore" || "notAfter" date from a certificate
    in ``"%b %d %H:%M:%S %Y %Z"`` strptime format (C locale).

    "notBefore" || "notAfter" dates must use UTC (RFC 5280).

    Month == one of: Jan Feb Mar Apr May Jun Jul Aug Sep Oct Nov Dec
    UTC should be specified as GMT (see ASN1_TIME_print())
    ";
        from time import strptime;
        from calendar import timegm;
        months = (;
        "Jan" , "Feb" , "Mar" , "Apr" , "May" , "Jun" ,;
        "Jul" , "Aug" , "Sep" , "Oct" , "Nov" , "Dec";
        );
        time_format = " %d %H:%M:%S %Y GMT";
        // try {
        month_number = months . index ( cert_time [ : 3 ] . title ( ) ) + 1;
        // } catch  ValueError  {
        panic!("ValueError ( "time data %r does !match "");
        "format "%%b%s"" % ( cert_time , time_format ) );
        } else {
        tt = strptime ( cert_time [ 3 : ] , time_format );
        return  timegm ( ( tt [ 0 ] , month_number ) + tt [ 2 : 6 ] );
        PEM_HEADER = "-----BEGIN CERTIFICATE-----";
        PEM_FOOTER = "-----END CERTIFICATE-----";
        pub fn DER_cert_to_PEM_cert ( der_cert_bytes )  {
        "Takes a certificate in binary DER format && returns the
    PEM version of it as a string.";
        f = str ( base64 . standard_b64encode ( der_cert_bytes ) , "ASCII" , "strict" );
        ss = [ PEM_HEADER ];
        ss + = vec![ f vec![ i : i + 64 ].iter().map(|i| range ( 0 , len ( f ) , 64 ) ).collect();
        ss . append ( PEM_FOOTER + "\n" );
        return  "\n" . join ( ss );
        pub fn PEM_cert_to_DER_cert ( pem_cert_string )  {
        "Takes a certificate in ASCII PEM format && returns the
    DER-encoded version of it as a byte sequence";
        if !pem_cert_string . startswith ( PEM_HEADER ) {
        panic!("ValueError ( "Invalid PEM encoding; must start with %s"");
        % PEM_HEADER );
        if !pem_cert_string . strip ( ) . endswith ( PEM_FOOTER ) {
        panic!("ValueError ( "Invalid PEM encoding; must end with %s"");
        % PEM_FOOTER );
        d = pem_cert_string . strip ( ) [ len ( PEM_HEADER ) : - len ( PEM_FOOTER ) ];
        return  base64 . decodebytes ( d . encode ( "ASCII" , "strict" ) );
        pub fn get_server_certificate ( addr , ssl_version = PROTOCOL_TLS_CLIENT , {
        ca_certs = None /* Option */ , timeout = _GLOBAL_DEFAULT_TIMEOUT ) ;
        "Retrieve the certificate from the server at the specified address,
    && return it as a PEM-encoded string.
    If 'ca_certs' == specified, validate the server cert against it.
    If 'ssl_version' == specified, use it in the connection attempt.
    If 'timeout' == specified, use it in the connection attempt.
    ";
        host , port = addr;
        if ca_certs is !None /* Option */ {
        cert_reqs = CERT_REQUIRED;
        } else {
        cert_reqs = CERT_NONE;
        context = _create_stdlib_context ( ssl_version ,;
        cert_reqs = cert_reqs ,;
        cafile = ca_certs );
        // with scope: create_connection ( addr , timeout = timeout ) as sock  {
        // with scope: context . wrap_socket ( sock , server_hostname = host ) as sslsock  {
        dercert = sslsock . getpeercert ( true );
        return  DER_cert_to_PEM_cert ( dercert );
        pub fn get_protocol_name ( protocol_code )  {
        return  _PROTOCOL_NAMES . get ( protocol_code , "<unknown>" );
}

