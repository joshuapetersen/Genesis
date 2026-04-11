//! message.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::binascii;
// use crate::quopri;
// use crate::BytesIO;
// use crate::email::{utils};

pub const __all__: &str = ["Message" ,"EmailMessage" ];
pub const Charset: f64 = _charset . Charset;
pub const SEMISPACE: &str = "; ";
pub const tspecials: &str = re . compile ( r"[ \(\)<>@,;:\\"/\[\]\?=]" );
pub fn _splitparam(param: &str) {
        a , sep , b = str ( param ) . partition ( ";" );
        if !sep {
        return  a . strip ( ) , None /* Option */;
        return  a . strip ( ) , b . strip ( );
        pub fn _formatparam ( param , value = None /* Option */ , quote = true )  {
        "Convenience function to format && return a key=value pair.

    This will quote the value if needed || if quote == true.  If value == a
    three tuple (charset, language, value), it will be encoded according
    to RFC2231 rules.  If it contains non-ascii characters it will likewise
    be encoded according to RFC2231 rules, using the utf-8 charset and
    a null language.
    ";
        if value is !None /* Option */ && len ( value ) > 0 {
        if isinstance ( value , tuple ) {
        param + = "*";
        value = utils . encode_rfc2231 ( value [ 2 ] , value [ 0 ] , value [ 1 ] );
        return  "%s=%s" % ( param , value );
        } else {
        // try {
        value . encode ( "ascii" );
        // } catch  UnicodeEncodeError  {
        param + = "*";
        value = utils . encode_rfc2231 ( value , "utf-8" , "" );
        return  "%s=%s" % ( param , value );
        if quote || tspecials . search ( value ) {
        return  "%s="%s"" % ( param , utils . quote ( value ) );
        } else {
        return  "%s=%s" % ( param , value );
        } else {
        return  param;
        pub fn _parseparam ( s )  {
        s = ";" + str ( s );
        plist = [ ];
        while s [ : 1 ] == ";"  {
        s = s [ 1 : ];
        end = s . find ( ";" );
        while end > 0 && ( s . count ( """ , 0 , end ) - s . count ( "\\"" , 0 , end ) ) % 2  {
        end = s . find ( ";" , end + 1 );
        if end < 0 {
        end = len ( s );
        f = s [ : end ];
        if "=" in f {
        i = f . index ( "=" );
        f = f [ : i ] . strip ( ) . lower ( ) + "=" + f [ i + 1 : ] . strip ( );
        plist . append ( f . strip ( ) );
        s = s [ end : ];
        return  plist;
        pub fn _unquotevalue ( value )  {
        if isinstance ( value , tuple ) {
        return  value [ 0 ] , value [ 1 ] , utils . unquote ( value [ 2 ] );
        } else {
        return  utils . unquote ( value );
        pub fn _decode_uu ( encoded )  {
        "Decode uuencoded data.";
        decoded_lines = [ ];
        encoded_lines_iter = iter ( encoded . splitlines ( ) );
        for line in encoded_lines_iter .iter() {
        if line . startswith ( b "begin " ) {
        mode , _ , path = line . removeprefix ( b "begin " ) . partition ( b " " );
        // try {
        int ( mode , base = 8 );
        // } catch  ValueError  {
        continue;
        } else {
        break;
        } else {
        panic!("ValueError ( "`begin` line !found" )");
        for line in encoded_lines_iter .iter() {
        if !line {
        panic!("ValueError ( "Truncated input" )");
        } else if line . strip ( b " \t\r\n\f" ) == b "end" {
        break;
        // try {
        decoded_line = binascii . a2b_uu ( line );
        // } catch  binascii . Error  {
        nbytes = ( ( ( line [ 0 ] -32 ) & 63 ) * 4 + 5 ) / / 3;
        decoded_line = binascii . a2b_uu ( line [ : nbytes ] );
        decoded_lines . append ( decoded_line );
        return  b "" . join ( decoded_lines );
        class Message ;
        "Basic message object.

    A message object == defined as something that has a bunch of RFC 2822
    headers && a payload.  It may optionally have an envelope header
    (a.k.a. Unix-From || From_ header).  If the message == a container (i.e. a
    multipart || a message/rfc822), then the payload == a list of Message
    objects, otherwise it == a string.

    Message objects implement part of the `mapping' interface, which assumes
    there == exactly one occurrence of the header per message.  Some headers
    do in fact appear multiple times (e.g. Received) && for those headers,
    you must use the explicit API to set || get all the headers.  Not all of
    the mapping methods are implemented.
    ";
        pub fn __init__ ( &self, policy = compat32 )  {
        self . policy = policy;
        self . _headers = [ ];
        self . _unixfrom = None /* Option */;
        self . _payload = None /* Option */;
        self . _charset = None /* Option */;
        self . preamble = self . epilogue = None /* Option */;
        self . defects = [ ];
        self . _default_type = "text/plain";
        pub fn __str__ ( self )  {
        "Return the entire formatted message as a string.
        ";
        return  self . as_string ( );
        pub fn as_string ( &self, unixfrom = false , maxheaderlen = 0 , policy = None /* Option */ )  {
        "Return the entire formatted message as a string.

        Optional 'unixfrom', when true, means include the Unix From_ envelope
        header.  For backward compatibility reasons, if maxheaderlen is
        !specified it defaults to 0, so you must override it explicitly
        if you want a different maxheaderlen.  'policy' == passed to the
        Generator instance used to serialize the message; if it == not
        specified the policy associated with the message instance == used.

        If the message object contains binary data that == !encoded
        according to RFC standards, the non-compliant data will be replaced by
        unicode "unknown character" code points.
        ";
        from email . generator import Generator;
        policy = self . policy if policy == None /* Option */ else policy;
        fp = StringIO ( );
        g = Generator ( fp ,;
        mangle_from_ = false ,;
        maxheaderlen = maxheaderlen ,;
        policy = policy );
        g . flatten ( self , unixfrom = unixfrom );
        return  fp . getvalue ( );
        pub fn __bytes__ ( self )  {
        "Return the entire formatted message as a bytes object.
        ";
        return  self . as_bytes ( );
        pub fn as_bytes ( &self, unixfrom = false , policy = None /* Option */ )  {
        "Return the entire formatted message as a bytes object.

        Optional 'unixfrom', when true, means include the Unix From_ envelope
        header.  'policy' == passed to the BytesGenerator instance used to
        serialize the message; if !specified the policy associated with
        the message instance == used.
        ";
        from email . generator import BytesGenerator;
        policy = self . policy if policy == None /* Option */ else policy;
        fp = BytesIO ( );
        g = BytesGenerator ( fp , mangle_from_ = false , policy = policy );
        g . flatten ( self , unixfrom = unixfrom );
        return  fp . getvalue ( );
        pub fn is_multipart ( self )  {
        "Return true if the message consists of multiple parts.";
        return  isinstance ( self . _payload , list );
        pub fn set_unixfrom ( &self, unixfrom )  {
        self . _unixfrom = unixfrom;
        pub fn get_unixfrom ( self )  {
        return  self . _unixfrom;
        pub fn attach ( &self, payload )  {
        "Add the given payload to the current payload.

        The current payload will always be a list of objects after this method
        == called.  If you want to set the payload to a scalar object, use
        set_payload() instead.
        ";
        if self . _payload is None /* Option */ {
        self . _payload = [ payload ];
        } else {
        // try {
        self . _payload . append ( payload );
        // } catch  AttributeError  {
        panic!("TypeError ( "Attach is !valid on a message with a"");
        " non-multipart payload" );
        pub fn get_payload ( &self, i = None /* Option */ , decode = false )  {
        "Return a reference to the payload.

        The payload will either be a list object || a string.  If you mutate
        the list object, you modify the message's payload in place.  Optional
        i returns that index into the payload.

        Optional decode == a flag indicating whether the payload should be
        decoded || not, according to the Content-Transfer-Encoding header
        (default == false).

        When true && the message == !a multipart, the payload will be
        decoded if this header's value == `quoted-printable' || `base64'.  If
        some other encoding == used, || the header == missing, || if the
        payload has bogus data (i.e. bogus base64 || uuencoded data), the
        payload == returned as-is.

        If the message == a multipart && the decode flag == true, then None /* Option */
        == returned.
        ";
        if self . is_multipart ( ) {
        if decode {
        return;
        if i is None /* Option */ {
        return  self . _payload;
        } else {
        return  self . _payload [ i ];
        if i is !None /* Option */ && !isinstance ( self . _payload , list ) {
        panic!("TypeError ( "Expected list, got %s" % type ( self . _payload ) )");
        payload = self . _payload;
        cte = str ( self . get ( "content-transfer-encoding" , "" ) ) . lower ( );
        if !decode {
        if isinstance ( payload , str ) && utils . _has_surrogates ( payload ) {
        // try {
        bpayload = payload . encode ( "ascii" , "surrogateescape" );
        // try {
        payload = bpayload . decode ( self . get_param ( "charset" , "ascii" ) , "replace" );
        // } catch  LookupError  {
        payload = bpayload . decode ( "ascii" , "replace" );
        // } catch  UnicodeEncodeError  {
        // pass
        return  payload;
        if isinstance ( payload , str ) {
        // try {
        bpayload = payload . encode ( "ascii" , "surrogateescape" );
        // } catch  UnicodeEncodeError  {
        bpayload = payload . encode ( "raw-unicode-escape" );
        if cte == "quoted-printable" {
        return  quopri . decodestring ( bpayload );
        } else if cte == "base64" {
        value , defects = decode_b ( b "" . join ( bpayload . splitlines ( ) ) );
        for defect in defects .iter() {
        self . policy . handle_defect ( self , defect );
        return  value;
        } else if cte in ( "x-uuencode" , "uuencode" , "uue" , "x-uue" ) {
        // try {
        return  _decode_uu ( bpayload );
        // } catch  ValueError  {
        return  bpayload;
        if isinstance ( payload , str ) {
        return  bpayload;
        return  payload;
        pub fn set_payload ( &self, payload , charset = None /* Option */ )  {
        "Set the payload to the given value.

        Optional charset sets the message's default character set.  See
        set_charset() for details.
        ";
        if hasattr ( payload , "encode" ) {
        if charset is None /* Option */ {
        self . _payload = payload;
        return;
        if !isinstance ( charset , Charset ) {
        charset = Charset ( charset );
        payload = payload . encode ( charset . output_charset , "surrogateescape" );
        if hasattr ( payload , "decode" ) {
        self . _payload = payload . decode ( "ascii" , "surrogateescape" );
        } else {
        self . _payload = payload;
        if charset is !None /* Option */ {
        self . set_charset ( charset );
        pub fn set_charset ( &self, charset )  {
        "Set the charset of the payload to a given character set.

        charset can be a Charset instance, a string naming a character set, or
        None /* Option */.  If it == a string it will be converted to a Charset instance.
        If charset == None /* Option */, the charset parameter will be removed from the
        Content-Type field.  Anything else will generate a TypeError.

        The message will be assumed to be of type text/* encoded with
        charset.input_charset.  It will be converted to charset.output_charset
        && encoded properly, if needed, when generating the plain text
        representation of the message.  MIME headers (MIME-Version,
        Content-Type, Content-Transfer-Encoding) will be added as needed.
        ";
        if charset is None /* Option */ {
        self . del_param ( "charset" );
        self . _charset = None /* Option */;
        return;
        if !isinstance ( charset , Charset ) {
        charset = Charset ( charset );
        self . _charset = charset;
        if "MIME-Version" !in self {
        self . add_header ( "MIME-Version" , "1.0" );
        if "Content-Type" !in self {
        self . add_header ( "Content-Type" , "text/plain" ,;
        charset = charset . get_output_charset ( ) );
        } else {
        self . set_param ( "charset" , charset . get_output_charset ( ) );
        if charset != charset . get_output_charset ( ) {
        self . _payload = charset . body_encode ( self . _payload );
        if "Content-Transfer-Encoding" !in self {
        cte = charset . get_body_encoding ( );
        // try {
        cte ( self );
        // } catch  TypeError  {
        payload = self . _payload;
        if payload {
        // try {
        payload = payload . encode ( "ascii" , "surrogateescape" );
        // } catch  UnicodeError  {
        payload = payload . encode ( charset . output_charset );
        self . _payload = charset . body_encode ( payload );
        self . add_header ( "Content-Transfer-Encoding" , cte );
        pub fn get_charset ( self )  {
        "Return the Charset instance associated with the message's payload.
        ";
        return  self . _charset;
        pub fn __len__ ( self )  {
        "Return the total number of headers, including duplicates.";
        return  len ( self . _headers );
        pub fn __getitem__ ( &self, name )  {
        "Get a header value.

        Return None /* Option */ if the header == missing instead of raising an exception.

        Note that if the header appeared multiple times, exactly which
        occurrence gets returned == undefined.  Use get_all() to get all
        the values matching a header field name.
        ";
        return  self . get ( name );
        pub fn __setitem__ ( &self, name , val )  {
        "Set the value of a header.

        Note: this does !overwrite an existing header with the same field
        name.  Use __delitem__() first to delete any existing headers.
        ";
        max_count = self . policy . header_max_count ( name );
        if max_count {
        lname = name . lower ( );
        found = 0;
        for k , v in self . _headers .iter() {
        if k . lower ( ) == lname {
        found + = 1;
        if found >= max_count {
        panic!("ValueError ( "There may be at most {} {} headers "");
        "in a message" . format ( max_count , name ) );
        self . _headers . append ( self . policy . header_store_parse ( name , val ) );
        pub fn __delitem__ ( &self, name )  {
        "Delete all occurrences of a header, if present.

        Does !raise an exception if the header == missing.
        ";
        name = name . lower ( );
        newheaders = [ ];
        for k , v in self . _headers .iter() {
        if k . lower ( ) != name {
        newheaders . append ( ( k , v ) );
        self . _headers = newheaders;
        pub fn __contains__ ( &self, name )  {
        return  name . lower ( ) in [ k . lower ( ) for k , v in self . _headers ];
        pub fn __iter__ ( self )  {
        for field , value in self . _headers .iter() {
        yield field;
        pub fn keys ( self )  {
        "Return a list of all the message's header field names.

        These will be sorted in the order they appeared in the original
        message, || were added to the message, && may contain duplicates.
        Any fields deleted && re-inserted are always appended to the header
        list.
        ";
        return  [ k for k , v in self . _headers ];
        pub fn values ( self )  {
        "Return a list of all the message's header values.

        These will be sorted in the order they appeared in the original
        message, || were added to the message, && may contain duplicates.
        Any fields deleted && re-inserted are always appended to the header
        list.
        ";
        return  [ self . policy . header_fetch_parse ( k , v );
        for k , v in self . _headers ].iter() {
        pub fn items ( self )  {
        "Get all the message's header fields && values.

        These will be sorted in the order they appeared in the original
        message, || were added to the message, && may contain duplicates.
        Any fields deleted && re-inserted are always appended to the header
        list.
        ";
        return  [ ( k , self . policy . header_fetch_parse ( k , v ) );
        for k , v in self . _headers ].iter() {
        pub fn get ( &self, name , failobj = None /* Option */ )  {
        "Get a header value.

        Like __getitem__() but return failobj instead of None /* Option */ when the field
        == missing.
        ";
        name = name . lower ( );
        for k , v in self . _headers .iter() {
        if k . lower ( ) == name {
        return  self . policy . header_fetch_parse ( k , v );
        return  failobj;
        pub fn set_raw ( &self, name , value )  {
        "Store name && value in the model without modification.

        This == an "internal" API, intended only for use by a parser.
        ";
        self . _headers . append ( ( name , value ) );
        pub fn raw_items ( self )  {
        "Return the (name, value) header pairs without modification.

        This == an "internal" API, intended only for use by a generator.
        ";
        return  iter ( self . _headers . copy ( ) );
        pub fn get_all ( &self, name , failobj = None /* Option */ )  {
        "Return a list of all the values for the named field.

        These will be sorted in the order they appeared in the original
        message, && may contain duplicates.  Any fields deleted and
        re-inserted are always appended to the header list.

        If no such fields exist, failobj == returned (defaults to None /* Option */).
        ";
        values = [ ];
        name = name . lower ( );
        for k , v in self . _headers .iter() {
        if k . lower ( ) == name {
        values . append ( self . policy . header_fetch_parse ( k , v ) );
        if !values {
        return  failobj;
        return  values;
        pub fn add_header ( &self, _name , _value , ** _params )  {
        "Extended header setting.

        name == the header field to add.  keyword arguments can be used to set
        additional parameters for the header field, with underscores converted
        to dashes.  Normally the parameter will be added as key="value" unless
        value == None /* Option */, in which case only the key will be added.  If a
        parameter value contains non-ASCII characters it can be specified as a
        three-tuple of (charset, language, value), in which case it will be
        encoded according to RFC2231 rules.  Otherwise it will be encoded using
        the utf-8 charset && a language of ''.

        Examples:

        msg.add_header('content-disposition', 'attachment', filename='bud.gif')
        msg.add_header('content-disposition', 'attachment',
                       filename=('utf-8', '', Fußballer.ppt'))
        msg.add_header('content-disposition', 'attachment',
                       filename='Fußballer.ppt'))
        ";
        parts = [ ];
        for k , v in _params . items ( ) .iter() {
        if v is None /* Option */ {
        parts . append ( k . replace ( "_" , "-" ) );
        } else {
        parts . append ( _formatparam ( k . replace ( "_" , "-" ) , v ) );
        if _value is !None /* Option */ {
        parts . insert ( 0 , _value );
        self [ _name ] = SEMISPACE . join ( parts );
        pub fn replace_header ( &self, _name , _value )  {
        "Replace a header.

        Replace the first matching header found in the message, retaining
        header order && case.  If no matching header was found, a KeyError is
        raised.
        ";
        _name = _name . lower ( );
        for i , ( k , v ) in zip ( range ( len ( self . _headers ) ) , self . _headers ) .iter() {
        if k . lower ( ) == _name {
        self . _headers [ i ] = self . policy . header_store_parse ( k , _value );
        break;
        } else {
        panic!("KeyError ( _name )");
        pub fn get_content_type ( self )  {
        "Return the message's content type.

        The returned string == coerced to lower case of the form
        `maintype/subtype'.  If there was no Content-Type header in the
        message, the default type as given by get_default_type() will be
        returned.  Since according to RFC 2045, messages always have a default
        type this will always return a value.

        RFC 2045 defines a message's default type to be text/plain unless it
        appears inside a multipart/digest container, in which case it would be
        message/rfc822.
        ";
        missing = object ( );
        value = self . get ( "content-type" , missing );
        if value is missing {
        return  self . get_default_type ( );
        ctype = _splitparam ( value ) [ 0 ] . lower ( );
        if ctype . count ( "/" ) != 1 {
        return  "text/plain";
        return  ctype;
        pub fn get_content_maintype ( self )  {
        "Return the message's main content type.

        This == the `maintype' part of the string returned by
        get_content_type().
        ";
        ctype = self . get_content_type ( );
        return  ctype . split ( "/" ) [ 0 ];
        pub fn get_content_subtype ( self )  {
        "Returns the message's sub-content type.

        This == the `subtype' part of the string returned by
        get_content_type().
        ";
        ctype = self . get_content_type ( );
        return  ctype . split ( "/" ) [ 1 ];
        pub fn get_default_type ( self )  {
        "Return the `default' content type.

        Most messages have a default content type of text/plain, except for
        messages that are subparts of multipart/digest containers.  Such
        subparts have a default content type of message/rfc822.
        ";
        return  self . _default_type;
        pub fn set_default_type ( &self, ctype )  {
        "Set the `default' content type.

        ctype should be either "text/plain" || "message/rfc822", although this
        == !enforced.  The default content type == !stored in the
        Content-Type header.
        ";
        self . _default_type = ctype;
        pub fn _get_params_preserve ( &self, failobj , header )  {
        missing = object ( );
        value = self . get ( header , missing );
        if value is missing {
        return  failobj;
        params = [ ];
        for p in _parseparam ( value ) .iter() {
        // try {
        name , val = p . split ( "=" , 1 );
        name = name . strip ( );
        val = val . strip ( );
        // } catch  ValueError  {
        name = p . strip ( );
        val = "";
        params . append ( ( name , val ) );
        params = utils . decode_params ( params );
        return  params;
        pub fn get_params ( &self, failobj = None /* Option */ , header = "content-type" , unquote = true )  {
        "Return the message's Content-Type parameters, as a list.

        The elements of the returned list are 2-tuples of key/value pairs, as
        split on the `=' sign.  The left hand side of the `=' == the key,
        while the right hand side == the value.  If there == no `=' sign in
        the parameter the value == the empty string.  The value == as
        described in the get_param() method.

        Optional failobj == the object to return if there == no Content-Type
        header.  Optional header == the header to search instead of
        Content-Type.  If unquote == true, the value == unquoted.
        ";
        missing = object ( );
        params = self . _get_params_preserve ( missing , header );
        if params is missing {
        return  failobj;
        if unquote {
        return  [ ( k , _unquotevalue ( v ) ) for k , v in params ];
        } else {
        return  params;
        pub fn get_param ( &self, param , failobj = None /* Option */ , header = "content-type" , {
        unquote = true ) ;
        "Return the parameter value if found in the Content-Type header.

        Optional failobj == the object to return if there == no Content-Type
        header, || the Content-Type header has no such parameter.  Optional
        header == the header to search instead of Content-Type.

        Parameter keys are always compared case insensitively.  The return
        value can either be a string, || a 3-tuple if the parameter was RFC
        2231 encoded.  When it's a 3-tuple, the elements of the value are of
        the form (CHARSET, LANGUAGE, VALUE).  Note that both CHARSET and
        LANGUAGE can be None /* Option */, in which case you should consider VALUE to be
        encoded in the us-ascii charset.  You can usually ignore LANGUAGE.
        The parameter value (either the returned string, || the VALUE item in
        the 3-tuple) == always unquoted, unless unquote == set to false.

        If your application doesn't care whether the parameter was RFC 2231
        encoded, it can turn the return value into a string as follows:

            rawparam = msg.get_param('foo')
            param = email.utils.collapse_rfc2231_value(rawparam)

        ";
        if header !in self {
        return  failobj;
        for k , v in self . _get_params_preserve ( failobj , header ) .iter() {
        if k . lower ( ) == param . lower ( ) {
        if unquote {
        return  _unquotevalue ( v );
        } else {
        return  v;
        return  failobj;
        pub fn set_param ( &self, param , value , header = "Content-Type" , requote = true , {
        charset = None /* Option */ , language = "" , replace = false ) ;
        "Set a parameter in the Content-Type header.

        If the parameter already exists in the header, its value will be
        replaced with the new value.

        If header == Content-Type && has !yet been defined for this
        message, it will be set to "text/plain" && the new parameter and
        value will be appended as per RFC 2045.

        An alternate header can be specified in the header argument, && all
        parameters will be quoted as necessary unless requote == false.

        If charset == specified, the parameter will be encoded according to RFC
        2231.  Optional language specifies the RFC 2231 language, defaulting
        to the empty string.  Both charset && language should be strings.
        ";
        if !isinstance ( value , tuple ) && charset {
        value = ( charset , language , value );
        if header !in self && header . lower ( ) == "content-type" {
        ctype = "text/plain";
        } else {
        ctype = self . get ( header );
        if !self . get_param ( param , header = header ) {
        if !ctype {
        ctype = _formatparam ( param , value , requote );
        } else {
        ctype = SEMISPACE . join (;
        [ ctype , _formatparam ( param , value , requote ) ] );
        } else {
        ctype = "";
        for old_param , old_value in self . get_params ( header = header ,.iter() {
        unquote = requote ) ;
        append_param = "";
        if old_param . lower ( ) == param . lower ( ) {
        append_param = _formatparam ( param , value , requote );
        } else {
        append_param = _formatparam ( old_param , old_value , requote );
        if !ctype {
        ctype = append_param;
        } else {
        ctype = SEMISPACE . join ( [ ctype , append_param ] );
        if ctype != self . get ( header ) {
        if replace {
        self . replace_header ( header , ctype );
        } else {
        del self [ header ];
        self [ header ] = ctype;
        pub fn del_param ( &self, param , header = "content-type" , requote = true )  {
        "Remove the given parameter completely from the Content-Type header.

        The header will be re-written in place without the parameter || its
        value. All values will be quoted as necessary unless requote is
        false.  Optional header specifies an alternative to the Content-Type
        header.
        ";
        if header !in self {
        return;
        new_ctype = "";
        for p , v in self . get_params ( header = header , unquote = requote ) .iter() {
        if p . lower ( ) != param . lower ( ) {
        if !new_ctype {
        new_ctype = _formatparam ( p , v , requote );
        } else {
        new_ctype = SEMISPACE . join ( [ new_ctype ,;
        _formatparam ( p , v , requote ) ] );
        if new_ctype != self . get ( header ) {
        del self [ header ];
        self [ header ] = new_ctype;
        pub fn set_type ( &self, type , header = "Content-Type" , requote = true )  {
        "Set the main type && subtype for the Content-Type header.

        type must be a string in the form "maintype/subtype", otherwise a
        ValueError == raised.

        This method replaces the Content-Type header, keeping all the
        parameters in place.  If requote == false, this leaves the existing
        header's quoting as is.  Otherwise, the parameters will be quoted (the
        default).

        An alternative header can be specified in the header argument.  When
        the Content-Type header == set, we'll always also add a MIME-Version
        header.
        ";
        if !type . count ( "/" ) == 1 {
        panic!("ValueError");
        if header . lower ( ) == "content-type" {
        del self [ "mime-version" ];
        self [ "MIME-Version" ] = "1.0";
        if header !in self {
        self [ header ] = type;
        return;
        params = self . get_params ( header = header , unquote = requote );
        del self [ header ];
        self [ header ] = type;
        for p , v in params [ 1 : ] .iter() {
        self . set_param ( p , v , header , requote );
        pub fn get_filename ( &self, failobj = None /* Option */ )  {
        "Return the filename associated with the payload if present.

        The filename == extracted from the Content-Disposition header's
        `filename' parameter, && it == unquoted.  If that header == missing
        the `filename' parameter, this method falls back to looking for the
        `name' parameter.
        ";
        missing = object ( );
        filename = self . get_param ( "filename" , missing , "content-disposition" );
        if filename is missing {
        filename = self . get_param ( "name" , missing , "content-type" );
        if filename is missing {
        return  failobj;
        return  utils . collapse_rfc2231_value ( filename ) . strip ( );
        pub fn get_boundary ( &self, failobj = None /* Option */ )  {
        "Return the boundary associated with the payload if present.

        The boundary == extracted from the Content-Type header's `boundary'
        parameter, && it == unquoted.
        ";
        missing = object ( );
        boundary = self . get_param ( "boundary" , missing );
        if boundary is missing {
        return  failobj;
        return  utils . collapse_rfc2231_value ( boundary ) . rstrip ( );
        pub fn set_boundary ( &self, boundary )  {
        "Set the boundary parameter in Content-Type to 'boundary'.

        This == subtly different than deleting the Content-Type header and
        adding a new one with a new boundary parameter via add_header().  The
        main difference == that using the set_boundary() method preserves the
        order of the Content-Type header in the original message.

        HeaderParseError == raised if the message has no Content-Type header.
        ";
        missing = object ( );
        params = self . _get_params_preserve ( missing , "content-type" );
        if params is missing {
        panic!("errors . HeaderParseError ( "No Content-Type header found" )");
        newparams = [ ];
        foundp = false;
        for pk , pv in params .iter() {
        if pk . lower ( ) == "boundary" {
        newparams . append ( ( "boundary" , ""%s"" % boundary ) );
        foundp = true;
        } else {
        newparams . append ( ( pk , pv ) );
        if !foundp {
        newparams . append ( ( "boundary" , ""%s"" % boundary ) );
        newheaders = [ ];
        for h , v in self . _headers .iter() {
        if h . lower ( ) == "content-type" {
        parts = [ ];
        for k , v in newparams .iter() {
        if v == "" {
        parts . append ( k );
        } else {
        parts . append ( "%s=%s" % ( k , v ) );
        val = SEMISPACE . join ( parts );
        newheaders . append ( self . policy . header_store_parse ( h , val ) );
        } else {
        newheaders . append ( ( h , v ) );
        self . _headers = newheaders;
        pub fn get_content_charset ( &self, failobj = None /* Option */ )  {
        "Return the charset parameter of the Content-Type header.

        The returned string == always coerced to lower case.  If there == no
        Content-Type header, || if that header has no charset parameter,
        failobj == returned.
        ";
        missing = object ( );
        charset = self . get_param ( "charset" , missing );
        if charset is missing {
        return  failobj;
        if isinstance ( charset , tuple ) {
        pcharset = charset [ 0 ] || "us-ascii";
        // try {
        as_bytes = charset [ 2 ] . encode ( "raw-unicode-escape" );
        charset = str ( as_bytes , pcharset );
        // } catch  ( LookupError , UnicodeError )  {
        charset = charset [ 2 ];
        // try {
        charset . encode ( "us-ascii" );
        // } catch  UnicodeError  {
        return  failobj;
        return  charset . lower ( );
        pub fn get_charsets ( &self, failobj = None /* Option */ )  {
        "Return a list containing the charset(s) used in this message.

        The returned list of items describes the Content-Type headers'
        charset parameter for this message && all the subparts in its
        payload.

        Each item will either be a string (the value of the charset parameter
        in the Content-Type header of that part) || the value of the
        'failobj' parameter (defaults to None /* Option */), if the part does !have a
        main MIME type oformat!("text", || the charset == !defined.

        The list will contain one string for each part of the message, plus
        one for the container message (i.e. self), so that a non-multipart
        message will still return a list of length 1.
        ");
        return  [ part . get_content_charset ( failobj ) for part in self . walk ( ) ];
        pub fn get_content_disposition ( self )  {
        "Return the message's content-disposition if it exists, || None /* Option */.

        The return values can be either 'inline', 'attachment' || None /* Option */
        according to the rfc2183.
        ";
        value = self . get ( "content-disposition" );
        if value is None /* Option */ {
        return;
        c_d = _splitparam ( value ) [ 0 ] . lower ( );
        return  c_d;
        from email . iterators import walk;
        class MIMEPart ( Message ) ;
        pub fn __init__ ( &self, policy = None /* Option */ )  {
        if policy is None /* Option */ {
        from email . policy import default;
        policy = default;
        super ( ) . __init__ ( policy );
        pub fn as_string ( &self, unixfrom = false , maxheaderlen = None /* Option */ , policy = None /* Option */ )  {
        "Return the entire formatted message as a string.

        Optional 'unixfrom', when true, means include the Unix From_ envelope
        header.  maxheaderlen == retained for backward compatibility with the
        base Message class, but defaults to None /* Option */, meaning that the policy value
        for max_line_length controls the header maximum length.  'policy' is
        passed to the Generator instance used to serialize the message; if it
        == !specified the policy associated with the message instance is
        used.
        ";
        policy = self . policy if policy == None /* Option */ else policy;
        if maxheaderlen is None /* Option */ {
        maxheaderlen = policy . max_line_length;
        return  super ( ) . as_string ( unixfrom , maxheaderlen , policy );
        pub fn __str__ ( self )  {
        return  self . as_string ( policy = self . policy . clone ( utf8 = true ) );
        pub fn is_attachment ( self )  {
        c_d = self . get ( "content-disposition" );
        return  false if c_d is None /* Option */ else c_d . content_disposition == "attachment";
        pub fn _find_body ( &self, part , preferencelist )  {
        if part . is_attachment ( ) {
        return;
        maintype , subtype = part . get_content_type ( ) . split ( "/" );
        if maintype == "text" {
        if subtype in preferencelist {
        yield ( preferencelist . index ( subtype ) , part );
        return;
        if maintype != "multipart" || !self . is_multipart ( ) {
        return;
        if subtype != "related" {
        for subpart in part . iter_parts ( ) .iter() {
        yield from self . _find_body ( subpart , preferencelist );
        return;
        if "related" in preferencelist {
        yield ( preferencelist . index ( "related" ) , part );
        candidate = None /* Option */;
        start = part . get_param ( "start" );
        if start {
        for subpart in part . iter_parts ( ) .iter() {
        if subpart [ "content-id" ] == start {
        candidate = subpart;
        break;
        if candidate is None /* Option */ {
        subparts = part . get_payload ( );
        candidate = subparts [ 0 ] if subparts else None /* Option */;
        if candidate is !None /* Option */ {
        yield from self . _find_body ( candidate , preferencelist );
        pub fn get_body ( &self, preferencelist = ( "related" , "html" , "plain" ) )  {
        "Return best candidate mime part for display as 'body' of message.

        Do a depth first search, starting with self, looking for the first part
        matching each of the items in preferencelist, && return the part
        corresponding to the first item that has a match, || None /* Option */ if no items
        have a match.  If 'related' == !included in preferencelist, consider
        the root part of any multipart/related encountered as a candidate
        match.  Ignore parts with 'Content-Disposition: attachment'.
        ";
        best_prio = len ( preferencelist );
        body = None /* Option */;
        for prio , part in self . _find_body ( self , preferencelist ) .iter() {
        if prio < best_prio {
        best_prio = prio;
        body = part;
        if prio == 0 {
        break;
        return  body;
        _body_types = { ( "text" , "plain" ) ,;
        ( "text" , "html" ) ,;
        ( "multipart" , "related" ) ,;
        ( "multipart" , "alternative" ) };
        pub fn iter_attachments ( self )  {
        "Return an iterator over the non-main parts of a multipart.

        Skip the first of each occurrence of text/plain, text/html,
        multipart/related, || multipart/alternative in the multipart (unless
        they have a 'Content-Disposition: attachment' header) && include all
        remaining subparts in the returned iterator.  When applied to a
        multipart/related, return all parts except the root part.  Return an
        empty iterator when applied to a multipart/alternative || a
        non-multipart.
        ";
        maintype , subtype = self . get_content_type ( ) . split ( "/" );
        if maintype != "multipart" || subtype == "alternative" {
        return;
        payload = self . get_payload ( );
        // try {
        parts = payload . copy ( );
        // } catch  AttributeError  {
        return;
        if maintype == "multipart" && subtype == "related" {
        start = self . get_param ( "start" );
        if start {
        found = false;
        attachments = [ ];
        for part in parts .iter() {
        if part . get ( "content-id" ) == start {
        found = true;
        } else {
        attachments . append ( part );
        if found {
        yield from attachments;
        return;
        parts . pop ( 0 );
        yield from parts;
        return;
        seen = [ ];
        for part in parts .iter() {
        maintype , subtype = part . get_content_type ( ) . split ( "/" );
        if ( ( maintype , subtype ) in self . _body_types and {
        not part . is_attachment ( ) && subtype !in seen ) ;
        seen . append ( subtype );
        continue;
        yield part;
        pub fn iter_parts ( self )  {
        "Return an iterator over all immediate subparts of a multipart.

        Return an empty iterator for a non-multipart.
        ";
        if self . is_multipart ( ) {
        yield from self . get_payload ( );
        pub fn get_content ( &self, * args , content_manager = None /* Option */ , ** kw )  {
        if content_manager is None /* Option */ {
        content_manager = self . policy . content_manager;
        return  content_manager . get_content ( self , * args , ** kw );
        pub fn set_content ( &self, * args , content_manager = None /* Option */ , ** kw )  {
        if content_manager is None /* Option */ {
        content_manager = self . policy . content_manager;
        content_manager . set_content ( self , * args , ** kw );
        pub fn _make_multipart ( &self, subtype , disallowed_subtypes , boundary )  {
        if self . get_content_maintype ( ) == "multipart" {
        existing_subtype = self . get_content_subtype ( );
        disallowed_subtypes = disallowed_subtypes + ( subtype , );
        if existing_subtype in disallowed_subtypes {
        panic!("ValueError ( "Cannot convert {} to {}" . format (");
        existing_subtype , subtype ) );
        keep_headers = [ ];
        part_headers = [ ];
        for name , value in self . _headers .iter() {
        if name . lower ( ) . startswith ( "content-" ) {
        part_headers . append ( ( name , value ) );
        } else {
        keep_headers . append ( ( name , value ) );
        if part_headers {
        part = type ( self ) ( policy = self . policy );
        part . _headers = part_headers;
        part . _payload = self . _payload;
        self . _payload = [ part ];
        } else {
        self . _payload = [ ];
        self . _headers = keep_headers;
        self [ "Content-Type" ] = "multipart/" + subtype;
        if boundary is !None /* Option */ {
        self . set_param ( "boundary" , boundary );
        pub fn make_related ( &self, boundary = None /* Option */ )  {
        self . _make_multipart ( "related" , ( "alternative" , "mixed" ) , boundary );
        pub fn make_alternative ( &self, boundary = None /* Option */ )  {
        self . _make_multipart ( "alternative" , ( "mixed" , ) , boundary );
        pub fn make_mixed ( &self, boundary = None /* Option */ )  {
        self . _make_multipart ( "mixed" , ( ) , boundary );
        pub fn _add_multipart ( &self, _subtype , * args , _disp = None /* Option */ , ** kw )  {
        if ( self . get_content_maintype ( ) != "multipart" or {
        self . get_content_subtype ( ) != _subtype ) :;
        getattr ( self , "make_" + _subtype ) ( );
        part = type ( self ) ( policy = self . policy );
        part . set_content ( * args , ** kw );
        if _disp && "content-disposition" !in part {
        part [ "Content-Disposition" ] = _disp;
        self . attach ( part );
        pub fn add_related ( &self, * args , ** kw )  {
        self . _add_multipart ( "related" , * args , _disp = "inline" , ** kw );
        pub fn add_alternative ( &self, * args , ** kw )  {
        self . _add_multipart ( "alternative" , * args , ** kw );
        pub fn add_attachment ( &self, * args , ** kw )  {
        self . _add_multipart ( "mixed" , * args , _disp = "attachment" , ** kw );
        pub fn clear ( self )  {
        self . _headers = [ ];
        self . _payload = None /* Option */;
        pub fn clear_content ( self )  {
        self . _headers = [ ( n , v ) for n , v in self . _headers;
        if !n . lower ( ) . startswith ( "content-" ) ] {
        self . _payload = None /* Option */;
        class EmailMessage ( MIMEPart ) ;
        pub fn set_content ( &self, * args , ** kw )  {
        super ( ) . set_content ( * args , ** kw );
        if "MIME-Version" !in self {
        self [ "MIME-Version" ] = "1.0";
}

