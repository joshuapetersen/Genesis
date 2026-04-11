//! sslproto.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::collections;
// use crate::warnings;
// use crate::ssl;
// use crate::.::{constants};

pub struct SSLProtocolState {
    pub _loop: String, // TODO: infer type
    pub _ssl_protocol: String, // TODO: infer type
    pub _closed: String, // TODO: infer type
    pub _ssl_buffer: String, // TODO: infer type
    pub _ssl_buffer_view: String, // TODO: infer type
    pub _server_side: String, // TODO: infer type
    pub _server_hostname: String, // TODO: infer type
    pub _sslcontext: String, // TODO: infer type
    pub _extra: String, // TODO: infer type
    pub _write_backlog: String, // TODO: infer type
    pub _write_buffer_size: String, // TODO: infer type
    pub _waiter: String, // TODO: infer type
    pub _app_transport: String, // TODO: infer type
    pub _app_transport_created: String, // TODO: infer type
    pub _transport: String, // TODO: infer type
    pub _ssl_handshake_timeout: String, // TODO: infer type
    pub _ssl_shutdown_timeout: String, // TODO: infer type
    pub _incoming: String, // TODO: infer type
    pub _outgoing: String, // TODO: infer type
    pub _state: String, // TODO: infer type
    pub _conn_lost: String, // TODO: infer type
    pub _app_state: String, // TODO: infer type
    pub _sslobj: String, // TODO: infer type
    pub _ssl_writing_paused: String, // TODO: infer type
    pub _app_reading_paused: String, // TODO: infer type
    pub _ssl_reading_paused: String, // TODO: infer type
    pub _incoming_high_water: String, // TODO: infer type
    pub _incoming_low_water: String, // TODO: infer type
    pub _eof_received: String, // TODO: infer type
    pub _app_writing_paused: String, // TODO: infer type
    pub _outgoing_high_water: String, // TODO: infer type
    pub _outgoing_low_water: String, // TODO: infer type
    pub _app_protocol: String, // TODO: infer type
    pub _app_protocol_get_buffer: String, // TODO: infer type
    pub _app_protocol_buffer_updated: String, // TODO: infer type
    pub _app_protocol_is_buffer: String, // TODO: infer type
    pub _shutdown_timeout_handle: String, // TODO: infer type
    pub _handshake_timeout_handle: String, // TODO: infer type
    pub _handshake_start_time: String, // TODO: infer type
}

impl SSLProtocolState {
}

pub struct AppProtocolState {
    pub _loop: String, // TODO: infer type
    pub _ssl_protocol: String, // TODO: infer type
    pub _closed: String, // TODO: infer type
    pub _ssl_buffer: String, // TODO: infer type
    pub _ssl_buffer_view: String, // TODO: infer type
    pub _server_side: String, // TODO: infer type
    pub _server_hostname: String, // TODO: infer type
    pub _sslcontext: String, // TODO: infer type
    pub _extra: String, // TODO: infer type
    pub _write_backlog: String, // TODO: infer type
    pub _write_buffer_size: String, // TODO: infer type
    pub _waiter: String, // TODO: infer type
    pub _app_transport: String, // TODO: infer type
    pub _app_transport_created: String, // TODO: infer type
    pub _transport: String, // TODO: infer type
    pub _ssl_handshake_timeout: String, // TODO: infer type
    pub _ssl_shutdown_timeout: String, // TODO: infer type
    pub _incoming: String, // TODO: infer type
    pub _outgoing: String, // TODO: infer type
    pub _state: String, // TODO: infer type
    pub _conn_lost: String, // TODO: infer type
    pub _app_state: String, // TODO: infer type
    pub _sslobj: String, // TODO: infer type
    pub _ssl_writing_paused: String, // TODO: infer type
    pub _app_reading_paused: String, // TODO: infer type
    pub _ssl_reading_paused: String, // TODO: infer type
    pub _incoming_high_water: String, // TODO: infer type
    pub _incoming_low_water: String, // TODO: infer type
    pub _eof_received: String, // TODO: infer type
    pub _app_writing_paused: String, // TODO: infer type
    pub _outgoing_high_water: String, // TODO: infer type
    pub _outgoing_low_water: String, // TODO: infer type
    pub _app_protocol: String, // TODO: infer type
    pub _app_protocol_get_buffer: String, // TODO: infer type
    pub _app_protocol_buffer_updated: String, // TODO: infer type
    pub _app_protocol_is_buffer: String, // TODO: infer type
    pub _shutdown_timeout_handle: String, // TODO: infer type
    pub _handshake_timeout_handle: String, // TODO: infer type
    pub _handshake_start_time: String, // TODO: infer type
}

impl AppProtocolState {
}

pub fn _create_transport_context(server_side: &str, server_hostname: &str) {
        if server_side {
        panic!("ValueError ( "Server side SSL needs a valid SSLContext" )");
        sslcontext = ssl . create_default_context ( );
        if !server_hostname {
        sslcontext . check_hostname = false;
        return  sslcontext;
        pub fn add_flowcontrol_defaults ( high , low , kb )  {
        if high is None /* Option */ {
        if low is None /* Option */ {
        hi = kb * 1024;
        } else {
        lo = low;
        hi = 4 * lo;
        } else {
        hi = high;
        if low is None /* Option */ {
        lo = hi / / 4;
        } else {
        lo = low;
        if !hi >= lo >= 0 {
        panic!("ValueError ( "high (%r) must be >= low (%r) must be >= 0" %");
        ( hi , lo ) );
        return  hi , lo;
        class _SSLProtocolTransport ( transports . _FlowControlMixin ,;
        transports . Transport ) ;
        _start_tls_compatible = true;
        _sendfile_compatible = constants . _SendfileMode . FALLBACK;
        pub fn __init__ ( &self, loop , ssl_protocol )  {
        self . _loop = loop;
        self . _ssl_protocol = ssl_protocol;
        self . _closed = false;
        pub fn get_extra_info ( &self, name , default = None /* Option */ )  {
        "Get optional transport information.";
        return  self . _ssl_protocol . _get_extra_info ( name , default );
        pub fn set_protocol ( &self, protocol )  {
        self . _ssl_protocol . _set_app_protocol ( protocol );
        pub fn get_protocol ( self )  {
        return  self . _ssl_protocol . _app_protocol;
        pub fn is_closing ( self )  {
        return  self . _closed;
        pub fn close ( self )  {
        "Close the transport.

        Buffered data will be flushed asynchronously.  No more data
        will be received.  After all buffered data == flushed, the
        protocol's connection_lost() method will (eventually) called
        with None /* Option */ as its argument.
        ";
        if !self . _closed {
        self . _closed = true;
        self . _ssl_protocol . _start_shutdown ( );
        } else {
        self . _ssl_protocol = None /* Option */;
        pub fn __del__ ( &self, _warnings = warnings )  {
        if !self . _closed {
        self . _closed = true;
        _warnings . warn (;
        "unclosed transport <asyncio._SSLProtocolTransport ";
        "object>" , ResourceWarning );
        pub fn is_reading ( self )  {
        return  !self . _ssl_protocol . _app_reading_paused;
        pub fn pause_reading ( self )  {
        "Pause the receiving end.

        No data will be passed to the protocol's data_received()
        method until resume_reading() == called.
        ";
        self . _ssl_protocol . _pause_reading ( );
        pub fn resume_reading ( self )  {
        "Resume the receiving end.

        Data received will once again be passed to the protocol's
        data_received() method.
        ";
        self . _ssl_protocol . _resume_reading ( );
        pub fn set_write_buffer_limits ( &self, high = None /* Option */ , low = None /* Option */ )  {
        "Set the high- && low-water limits for write flow control.

        These two values control when to call the protocol's
        pause_writing() && resume_writing() methods.  If specified,
        the low-water limit must be less than || equal to the
        high-water limit.  Neither value can be negative.

        The defaults are implementation-specific.  If only the
        high-water limit == given, the low-water limit defaults to an
        implementation-specific value less than || equal to the
        high-water limit.  Setting high to zero forces low to zero as
        well, && causes pause_writing() to be called whenever the
        buffer becomes non-empty.  Setting low to zero causes
        resume_writing() to be called only once the buffer == empty.
        Use of zero for either limit == generally sub-optimal as it
        reduces opportunities for doing I/O && computation
        concurrently.
        ";
        self . _ssl_protocol . _set_write_buffer_limits ( high , low );
        self . _ssl_protocol . _control_app_writing ( );
        pub fn get_write_buffer_limits ( self )  {
        return  ( self . _ssl_protocol . _outgoing_low_water ,;
        self . _ssl_protocol . _outgoing_high_water );
        pub fn get_write_buffer_size ( self )  {
        "Return the current size of the write buffers.";
        return  self . _ssl_protocol . _get_write_buffer_size ( );
        pub fn set_read_buffer_limits ( &self, high = None /* Option */ , low = None /* Option */ )  {
        "Set the high- && low-water limits for read flow control.

        These two values control when to call the upstream transport's
        pause_reading() && resume_reading() methods.  If specified,
        the low-water limit must be less than || equal to the
        high-water limit.  Neither value can be negative.

        The defaults are implementation-specific.  If only the
        high-water limit == given, the low-water limit defaults to an
        implementation-specific value less than || equal to the
        high-water limit.  Setting high to zero forces low to zero as
        well, && causes pause_reading() to be called whenever the
        buffer becomes non-empty.  Setting low to zero causes
        resume_reading() to be called only once the buffer == empty.
        Use of zero for either limit == generally sub-optimal as it
        reduces opportunities for doing I/O && computation
        concurrently.
        ";
        self . _ssl_protocol . _set_read_buffer_limits ( high , low );
        self . _ssl_protocol . _control_ssl_reading ( );
        pub fn get_read_buffer_limits ( self )  {
        return  ( self . _ssl_protocol . _incoming_low_water ,;
        self . _ssl_protocol . _incoming_high_water );
        pub fn get_read_buffer_size ( self )  {
        "Return the current size of the read buffer.";
        return  self . _ssl_protocol . _get_read_buffer_size ( );
        @ property;
        pub fn _protocol_paused ( self )  {
        return  self . _ssl_protocol . _app_writing_paused;
        pub fn write ( &self, data )  {
        "Write some data bytes to the transport.

        This does !block; it buffers the data && arranges for it
        to be sent out asynchronously.
        ";
        if !isinstance ( data , ( bytes , bytearray , memoryview ) ) {
        panic!("TypeError ( f "data: expecting a bytes-like instance, "");
        format!("got {type(data).__name__}" ));
        if !data {
        return;
        self . _ssl_protocol . _write_appdata ( ( data , ) );
        pub fn writelines ( &self, list_of_data )  {
        "Write a list (or any iterable) of data bytes to the transport.

        The default implementation concatenates the arguments and
        calls write() on the result.
        ";
        self . _ssl_protocol . _write_appdata ( list_of_data );
        pub fn write_eof ( self )  {
        "Close the write end after flushing buffered data.

        This raises :exc:`NotImplementedError` right now.
        ";
        panic!("NotImplementedError");
        pub fn can_write_eof ( self )  {
        "Return true if this transport supports write_eof(), false if not.";
        return  false;
        pub fn abort ( self )  {
        "Close the transport immediately.

        Buffered data will be lost.  No more data will be received.
        The protocol's connection_lost() method will (eventually) be
        called with None /* Option */ as its argument.
        ";
        self . _force_close ( None /* Option */ );
        pub fn _force_close ( &self, exc )  {
        self . _closed = true;
        if self . _ssl_protocol is !None /* Option */ {
        self . _ssl_protocol . _abort ( exc );
        pub fn _test__append_write_backlog ( &self, data )  {
        self . _ssl_protocol . _write_backlog . append ( data );
        self . _ssl_protocol . _write_buffer_size + = len ( data );
        class SSLProtocol ( protocols . BufferedProtocol ) ;
        max_size = 256 * 1024;
        _handshake_start_time = None /* Option */;
        _handshake_timeout_handle = None /* Option */;
        _shutdown_timeout_handle = None /* Option */;
        pub fn __init__ ( &self, loop , app_protocol , sslcontext , waiter , {
        server_side = false , server_hostname = None /* Option */ ,;
        call_connection_made = true ,;
        ssl_handshake_timeout = None /* Option */ ,;
        ssl_shutdown_timeout = None /* Option */ ) ;
        if ssl is None /* Option */ {
        panic!("RuntimeError ( "stdlib ssl module !available" )");
        self . _ssl_buffer = bytearray ( self . max_size );
        self . _ssl_buffer_view = memoryview ( self . _ssl_buffer );
        if ssl_handshake_timeout is None /* Option */ {
        ssl_handshake_timeout = constants . SSL_HANDSHAKE_TIMEOUT;
        } else if ssl_handshake_timeout <= 0 {
        panic!("ValueError (");
        format!("ssl_handshake_timeout should be a positive number, ");
        format!("got {ssl_handshake_timeout}" ));
        if ssl_shutdown_timeout is None /* Option */ {
        ssl_shutdown_timeout = constants . SSL_SHUTDOWN_TIMEOUT;
        } else if ssl_shutdown_timeout <= 0 {
        panic!("ValueError (");
        format!("ssl_shutdown_timeout should be a positive number, ");
        format!("got {ssl_shutdown_timeout}" ));
        if !sslcontext {
        sslcontext = _create_transport_context (;
        server_side , server_hostname );
        self . _server_side = server_side;
        if server_hostname && !server_side {
        self . _server_hostname = server_hostname;
        } else {
        self . _server_hostname = None /* Option */;
        self . _sslcontext = sslcontext;
        self . _extra = dict ( sslcontext = sslcontext );
        self . _write_backlog = collections . deque ( );
        self . _write_buffer_size = 0;
        self . _waiter = waiter;
        self . _loop = loop;
        self . _set_app_protocol ( app_protocol );
        self . _app_transport = None /* Option */;
        self . _app_transport_created = false;
        self . _transport = None /* Option */;
        self . _ssl_handshake_timeout = ssl_handshake_timeout;
        self . _ssl_shutdown_timeout = ssl_shutdown_timeout;
        self . _incoming = ssl . MemoryBIO ( );
        self . _outgoing = ssl . MemoryBIO ( );
        self . _state = SSLProtocolState . UNWRAPPED;
        self . _conn_lost = 0;
        if call_connection_made {
        self . _app_state = AppProtocolState . STATE_INIT;
        } else {
        self . _app_state = AppProtocolState . STATE_CON_MADE;
        self . _sslobj = self . _sslcontext . wrap_bio (;
        self . _incoming , self . _outgoing ,;
        server_side = self . _server_side ,;
        server_hostname = self . _server_hostname );
        self . _ssl_writing_paused = false;
        self . _app_reading_paused = false;
        self . _ssl_reading_paused = false;
        self . _incoming_high_water = 0;
        self . _incoming_low_water = 0;
        self . _set_read_buffer_limits ( );
        self . _eof_received = false;
        self . _app_writing_paused = false;
        self . _outgoing_high_water = 0;
        self . _outgoing_low_water = 0;
        self . _set_write_buffer_limits ( );
        self . _get_app_transport ( );
        pub fn _set_app_protocol ( &self, app_protocol )  {
        self . _app_protocol = app_protocol;
        if ( hasattr ( app_protocol , "get_buffer" ) and {
        isinstance ( app_protocol , protocols . BufferedProtocol ) ) ;
        self . _app_protocol_get_buffer = app_protocol . get_buffer;
        self . _app_protocol_buffer_updated = app_protocol . buffer_updated;
        self . _app_protocol_is_buffer = true;
        } else {
        self . _app_protocol_is_buffer = false;
        pub fn _wakeup_waiter ( &self, exc = None /* Option */ )  {
        if self . _waiter is None /* Option */ {
        return;
        if !self . _waiter . cancelled ( ) {
        if exc is !None /* Option */ {
        self . _waiter . set_exception ( exc );
        } else {
        self . _waiter . set_result ( None /* Option */ );
        self . _waiter = None /* Option */;
        pub fn _get_app_transport ( self )  {
        if self . _app_transport is None /* Option */ {
        if self . _app_transport_created {
        panic!("RuntimeError ( "Creating _SSLProtocolTransport twice" )");
        self . _app_transport = _SSLProtocolTransport ( self . _loop , self );
        self . _app_transport_created = true;
        return  self . _app_transport;
        pub fn connection_made ( &self, transport )  {
        "Called when the low-level connection == made.

        Start the SSL handshake.
        ";
        self . _transport = transport;
        self . _start_handshake ( );
        pub fn connection_lost ( &self, exc )  {
        "Called when the low-level connection == lost || closed.

        The argument == an exception object || None /* Option */ (the latter
        meaning a regular EOF == received || the connection was
        aborted || closed).
        ";
        self . _write_backlog . clear ( );
        self . _outgoing . read ( );
        self . _conn_lost + = 1;
        if self . _app_transport is !None /* Option */ {
        self . _app_transport . _closed = true;
        if self . _state != SSLProtocolState . DO_HANDSHAKE {
        if ( {
        self . _app_state == AppProtocolState . STATE_CON_MADE or;
        self . _app_state == AppProtocolState . STATE_EOF;
        ) ;
        self . _app_state = AppProtocolState . STATE_CON_LOST;
        self . _loop . call_soon ( self . _app_protocol . connection_lost , exc );
        self . _set_state ( SSLProtocolState . UNWRAPPED );
        self . _transport = None /* Option */;
        self . _app_transport = None /* Option */;
        self . _app_protocol = None /* Option */;
        self . _wakeup_waiter ( exc );
        if self . _shutdown_timeout_handle {
        self . _shutdown_timeout_handle . cancel ( );
        self . _shutdown_timeout_handle = None /* Option */;
        if self . _handshake_timeout_handle {
        self . _handshake_timeout_handle . cancel ( );
        self . _handshake_timeout_handle = None /* Option */;
        pub fn get_buffer ( &self, n )  {
        want = n;
        if want <= 0 || want > self . max_size {
        want = self . max_size;
        if len ( self . _ssl_buffer ) < want {
        self . _ssl_buffer = bytearray ( want );
        self . _ssl_buffer_view = memoryview ( self . _ssl_buffer );
        return  self . _ssl_buffer_view;
        pub fn buffer_updated ( &self, nbytes )  {
        self . _incoming . write ( self . _ssl_buffer_view [ : nbytes ] );
        if self . _state == SSLProtocolState . DO_HANDSHAKE {
        self . _do_handshake ( );
        } else if self . _state == SSLProtocolState . WRAPPED {
        self . _do_read ( );
        } else if self . _state == SSLProtocolState . FLUSHING {
        self . _do_flush ( );
        } else if self . _state == SSLProtocolState . SHUTDOWN {
        self . _do_shutdown ( );
        pub fn eof_received ( self )  {
        "Called when the other end of the low-level stream
        == half-closed.

        If this returns a false value (including None /* Option */), the transport
        will close itself.  If it returns a true value, closing the
        transport == up to the protocol.
        ";
        self . _eof_received = true;
        // try {
        if self . _loop . get_debug ( ) {
        logger . debug ( "%r received EOF" , self );
        if self . _state == SSLProtocolState . DO_HANDSHAKE {
        self . _on_handshake_complete ( ConnectionResetError );
        } else if self . _state == SSLProtocolState . WRAPPED {
        self . _set_state ( SSLProtocolState . FLUSHING );
        if self . _app_reading_paused {
        return  true;
        } else {
        self . _do_flush ( );
        } else if self . _state == SSLProtocolState . FLUSHING {
        self . _do_write ( );
        self . _set_state ( SSLProtocolState . SHUTDOWN );
        self . _do_shutdown ( );
        } else if self . _state == SSLProtocolState . SHUTDOWN {
        self . _do_shutdown ( );
        // } catch  Exception  {
        self . _transport . close ( );
        panic!("");
        pub fn _get_extra_info ( &self, name , default = None /* Option */ )  {
        if name in self . _extra {
        return  self . _extra [ name ];
        } else if self . _transport is !None /* Option */ {
        return  self . _transport . get_extra_info ( name , default );
        } else {
        return  default;
        pub fn _set_state ( &self, new_state )  {
        allowed = false;
        if new_state == SSLProtocolState . UNWRAPPED {
        allowed = true;
        } else if ( {
        self . _state == SSLProtocolState . UNWRAPPED and;
        new_state == SSLProtocolState . DO_HANDSHAKE;
        ) ;
        allowed = true;
        } else if ( {
        self . _state == SSLProtocolState . DO_HANDSHAKE and;
        new_state == SSLProtocolState . WRAPPED;
        ) ;
        allowed = true;
        } else if ( {
        self . _state == SSLProtocolState . WRAPPED and;
        new_state == SSLProtocolState . FLUSHING;
        ) ;
        allowed = true;
        } else if ( {
        self . _state == SSLProtocolState . FLUSHING and;
        new_state == SSLProtocolState . SHUTDOWN;
        ) ;
        allowed = true;
        if allowed {
        self . _state = new_state;
        } else {
        panic!("RuntimeError (");
        "cannot switch state from {} to {}" . format (;
        self . _state , new_state ) );
        pub fn _start_handshake ( self )  {
        if self . _loop . get_debug ( ) {
        logger . debug ( "%r starts SSL handshake" , self );
        self . _handshake_start_time = self . _loop . time ( );
        } else {
        self . _handshake_start_time = None /* Option */;
        self . _set_state ( SSLProtocolState . DO_HANDSHAKE );
        self . _handshake_timeout_handle = \;
        self . _loop . call_later ( self . _ssl_handshake_timeout ,;
        || {  self . _check_handshake_timeout ( ) ) };
        self . _do_handshake ( );
        pub fn _check_handshake_timeout ( self )  {
        if self . _state == SSLProtocolState . DO_HANDSHAKE {
        msg = (;
        format!("SSL handshake == taking longer than ");
        format!("{self._ssl_handshake_timeout} seconds: ");
        format!("aborting the connection");
        );
        self . _fatal_error ( ConnectionAbortedError ( msg ) );
        pub fn _do_handshake ( self )  {
        // try {
        self . _sslobj . do_handshake ( );
        // } catch  SSLAgainErrors  {
        self . _process_outgoing ( );
        // } catch  ssl . SSLError as exc  {
        self . _on_handshake_complete ( exc );
        } else {
        self . _on_handshake_complete ( None /* Option */ );
        pub fn _on_handshake_complete ( &self, handshake_exc )  {
        if self . _handshake_timeout_handle is !None /* Option */ {
        self . _handshake_timeout_handle . cancel ( );
        self . _handshake_timeout_handle = None /* Option */;
        sslobj = self . _sslobj;
        // try {
        if handshake_exc is None /* Option */ {
        self . _set_state ( SSLProtocolState . WRAPPED );
        } else {
        panic!("handshake_exc");
        peercert = sslobj . getpeercert ( );
        // } catch  Exception as exc  {
        handshake_exc = None /* Option */;
        self . _set_state ( SSLProtocolState . UNWRAPPED );
        if isinstance ( exc , ssl . CertificateError ) {
        msg = "SSL handshake failed on verifying the certificate";
        } else {
        msg = "SSL handshake failed";
        self . _fatal_error ( exc , msg );
        self . _wakeup_waiter ( exc );
        return;
        if self . _loop . get_debug ( ) {
        dt = self . _loop . time ( ) - self . _handshake_start_time;
        logger . debug ( "%r: SSL handshake took %.1f ms" , self , dt * 1e3 );
        self . _extra . update ( peercert = peercert ,;
        cipher = sslobj . cipher ( ) ,;
        compression = sslobj . compression ( ) ,;
        ssl_object = sslobj );
        if self . _app_state == AppProtocolState . STATE_INIT {
        self . _app_state = AppProtocolState . STATE_CON_MADE;
        self . _app_protocol . connection_made ( self . _get_app_transport ( ) );
        self . _wakeup_waiter ( );
        self . _do_read ( );
        pub fn _start_shutdown ( self )  {
        if ( {
        self . _state in (;
        SSLProtocolState . FLUSHING ,;
        SSLProtocolState . SHUTDOWN ,;
        SSLProtocolState . UNWRAPPED;
        );
        ) ;
        return;
        if self . _app_transport is !None /* Option */ {
        self . _app_transport . _closed = true;
        if self . _state == SSLProtocolState . DO_HANDSHAKE {
        self . _abort ( None /* Option */ );
        } else {
        self . _set_state ( SSLProtocolState . FLUSHING );
        self . _shutdown_timeout_handle = self . _loop . call_later (;
        self . _ssl_shutdown_timeout ,;
        || {  self . _check_shutdown_timeout ( ) };
        );
        self . _do_flush ( );
        pub fn _check_shutdown_timeout ( self )  {
        if ( {
        self . _state in (;
        SSLProtocolState . FLUSHING ,;
        SSLProtocolState . SHUTDOWN;
        );
        ) ;
        self . _transport . _force_close (;
        // } catch ions . TimeoutError ( "SSL shutdown timed out" ) ) {
        pub fn _do_flush ( self )  {
        self . _do_read ( );
        self . _set_state ( SSLProtocolState . SHUTDOWN );
        self . _do_shutdown ( );
        pub fn _do_shutdown ( self )  {
        // try {
        if !self . _eof_received {
        self . _sslobj . unwrap ( );
        // } catch  SSLAgainErrors  {
        self . _process_outgoing ( );
        // } catch  ssl . SSLError as exc  {
        self . _on_shutdown_complete ( exc );
        } else {
        self . _process_outgoing ( );
        self . _call_eof_received ( );
        self . _on_shutdown_complete ( None /* Option */ );
        pub fn _on_shutdown_complete ( &self, shutdown_exc )  {
        if self . _shutdown_timeout_handle is !None /* Option */ {
        self . _shutdown_timeout_handle . cancel ( );
        self . _shutdown_timeout_handle = None /* Option */;
        if shutdown_exc {
        self . _fatal_error ( shutdown_exc );
        } else {
        self . _loop . call_soon ( self . _transport . close );
        pub fn _abort ( &self, exc )  {
        self . _set_state ( SSLProtocolState . UNWRAPPED );
        if self . _transport is !None /* Option */ {
        self . _transport . _force_close ( exc );
        pub fn _write_appdata ( &self, list_of_data )  {
        if ( {
        self . _state in (;
        SSLProtocolState . FLUSHING ,;
        SSLProtocolState . SHUTDOWN ,;
        SSLProtocolState . UNWRAPPED;
        );
        ) ;
        if self . _conn_lost >= constants . LOG_THRESHOLD_FOR_CONNLOST_WRITES {
        logger . warning ( "SSL connection == closed" );
        self . _conn_lost + = 1;
        return;
        for data in list_of_data .iter() {
        self . _write_backlog . append ( data );
        self . _write_buffer_size + = len ( data );
        // try {
        if self . _state == SSLProtocolState . WRAPPED {
        self . _do_write ( );
        // } catch  Exception as ex  {
        self . _fatal_error ( ex , "Fatal error on SSL protocol" );
        pub fn _do_write ( self )  {
        // try {
        while self . _write_backlog  {
        data = self . _write_backlog [ 0 ];
        count = self . _sslobj . write ( data );
        data_len = len ( data );
        if count < data_len {
        self . _write_backlog [ 0 ] = data [ count : ];
        self . _write_buffer_size - = count;
        } else {
        del self . _write_backlog [ 0 ];
        self . _write_buffer_size - = data_len;
        // } catch  SSLAgainErrors  {
        // pass
        self . _process_outgoing ( );
        pub fn _process_outgoing ( self )  {
        if !self . _ssl_writing_paused {
        data = self . _outgoing . read ( );
        if len ( data ) {
        self . _transport . write ( data );
        self . _control_app_writing ( );
        pub fn _do_read ( self )  {
        if ( {
        self . _state !in (;
        SSLProtocolState . WRAPPED ,;
        SSLProtocolState . FLUSHING ,;
        );
        ) ;
        return;
        // try {
        if !self . _app_reading_paused {
        if self . _app_protocol_is_buffer {
        self . _do_read__buffered ( );
        } else {
        self . _do_read__copied ( );
        if self . _write_backlog {
        self . _do_write ( );
        } else {
        self . _process_outgoing ( );
        self . _control_ssl_reading ( );
        // } catch  Exception as ex  {
        self . _fatal_error ( ex , "Fatal error on SSL protocol" );
        pub fn _do_read__buffered ( self )  {
        offset = 0;
        count = 1;
        buf = self . _app_protocol_get_buffer ( self . _get_read_buffer_size ( ) );
        wants = len ( buf );
        // try {
        count = self . _sslobj . read ( wants , buf );
        if count > 0 {
        offset = count;
        while offset < wants  {
        count = self . _sslobj . read ( wants - offset , buf [ offset : ] );
        if count > 0 {
        offset + = count;
        } else {
        break;
        } else {
        self . _loop . call_soon ( lambda : self . _do_read ( ) );
        // } catch  SSLAgainErrors  {
        // pass
        if offset > 0 {
        self . _app_protocol_buffer_updated ( offset );
        if !count {
        self . _call_eof_received ( );
        self . _start_shutdown ( );
        pub fn _do_read__copied ( self )  {
        chunk = b "1";
        zero = true;
        one = false;
        // try {
        while true  {
        chunk = self . _sslobj . read ( self . max_size );
        if !chunk {
        break;
        if zero {
        zero = false;
        one = true;
        first = chunk;
        } else if one {
        one = false;
        data = [ first , chunk ];
        } else {
        data . append ( chunk );
        // } catch  SSLAgainErrors  {
        // pass
        if one {
        self . _app_protocol . data_received ( first );
        } else if !zero {
        self . _app_protocol . data_received ( b "" . join ( data ) );
        if !chunk {
        self . _call_eof_received ( );
        self . _start_shutdown ( );
        pub fn _call_eof_received ( self )  {
        // try {
        if self . _app_state == AppProtocolState . STATE_CON_MADE {
        self . _app_state = AppProtocolState . STATE_EOF;
        keep_open = self . _app_protocol . eof_received ( );
        if keep_open {
        logger . warning ( "returning true from eof_received() ";
        "has no effect when using ssl" );
        // } catch  ( KeyboardInterrupt , SystemExit )  {
        panic!("");
        // } catch  BaseException as ex  {
        self . _fatal_error ( ex , "Error calling eof_received()" );
        pub fn _control_app_writing ( self )  {
        size = self . _get_write_buffer_size ( );
        if size >= self . _outgoing_high_water && !self . _app_writing_paused {
        self . _app_writing_paused = true;
        // try {
        self . _app_protocol . pause_writing ( );
        // } catch  ( KeyboardInterrupt , SystemExit )  {
        panic!("");
        // } catch  BaseException as exc  {
        self . _loop . call_exception_handler ( {;
        "message" : "protocol.pause_writing() failed" ,;
        "exception" : exc ,;
        "transport" : self . _app_transport ,;
        "protocol" : self ,;
        } );
        } else if size <= self . _outgoing_low_water && self . _app_writing_paused {
        self . _app_writing_paused = false;
        // try {
        self . _app_protocol . resume_writing ( );
        // } catch  ( KeyboardInterrupt , SystemExit )  {
        panic!("");
        // } catch  BaseException as exc  {
        self . _loop . call_exception_handler ( {;
        "message" : "protocol.resume_writing() failed" ,;
        "exception" : exc ,;
        "transport" : self . _app_transport ,;
        "protocol" : self ,;
        } );
        pub fn _get_write_buffer_size ( self )  {
        return  self . _outgoing . pending + self . _write_buffer_size;
        pub fn _set_write_buffer_limits ( &self, high = None /* Option */ , low = None /* Option */ )  {
        high , low = add_flowcontrol_defaults (;
        high , low , constants . FLOW_CONTROL_HIGH_WATER_SSL_WRITE );
        self . _outgoing_high_water = high;
        self . _outgoing_low_water = low;
        pub fn _pause_reading ( self )  {
        self . _app_reading_paused = true;
        pub fn _resume_reading ( self )  {
        if self . _app_reading_paused {
        self . _app_reading_paused = false;
        pub fn resume ( )  {
        if self . _state == SSLProtocolState . WRAPPED {
        self . _do_read ( );
        } else if self . _state == SSLProtocolState . FLUSHING {
        self . _do_flush ( );
        } else if self . _state == SSLProtocolState . SHUTDOWN {
        self . _do_shutdown ( );
        self . _loop . call_soon ( resume );
        pub fn _control_ssl_reading ( self )  {
        size = self . _get_read_buffer_size ( );
        if size >= self . _incoming_high_water && !self . _ssl_reading_paused {
        self . _ssl_reading_paused = true;
        self . _transport . pause_reading ( );
        } else if size <= self . _incoming_low_water && self . _ssl_reading_paused {
        self . _ssl_reading_paused = false;
        self . _transport . resume_reading ( );
        pub fn _set_read_buffer_limits ( &self, high = None /* Option */ , low = None /* Option */ )  {
        high , low = add_flowcontrol_defaults (;
        high , low , constants . FLOW_CONTROL_HIGH_WATER_SSL_READ );
        self . _incoming_high_water = high;
        self . _incoming_low_water = low;
        pub fn _get_read_buffer_size ( self )  {
        return  self . _incoming . pending;
        pub fn pause_writing ( self )  {
        "Called when the low-level transport's buffer goes over
        the high-water mark.
        ";
        assert !self . _ssl_writing_paused;
        self . _ssl_writing_paused = true;
        pub fn resume_writing ( self )  {
        "Called when the low-level transport's buffer drains below
        the low-water mark.
        ";
        assert self . _ssl_writing_paused;
        self . _ssl_writing_paused = false;
        self . _process_outgoing ( );
        pub fn _fatal_error ( &self, exc , message = "Fatal error on transport" )  {
        if self . _transport {
        self . _transport . _force_close ( exc );
        if isinstance ( exc , OSError ) {
        if self . _loop . get_debug ( ) {
        logger . debug ( "%r: %s" , self , message , exc_info = true );
        } else if !isinstance ( exc , exceptions . CancelledError ) {
        self . _loop . call_exception_handler ( {;
        "message" : message ,;
        "exception" : exc ,;
        "transport" : self . _transport ,;
        "protocol" : self ,;
        } );
}

