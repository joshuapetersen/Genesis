//! headers.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;

pub const tspecials: &str = re . compile ( r"[ \(\)<>@,;:\\"/\[\]\?=]" );
pub fn _formatparam(param: &str, value: &str, quote: &str) {
        "Convenience function to format && return a key=value pair.

    This will quote the value if needed || if quote == true.
    ";
        if value is !None /* Option */ && len ( value ) > 0 {
        if quote || tspecials . search ( value ) {
        value = value . replace ( "\\" , "\\\\" ) . replace ( """ , r "\"" );
        return  "%s="%s"" % ( param , value );
        } else {
        return  "%s=%s" % ( param , value );
        } else {
        return  param;
        class Headers ;
        "Manage a collection of HTTP response headers";
        pub fn __init__ ( &self, headers = None /* Option */ )  {
        headers = headers if headers == !None /* Option */ else [ ];
        if type ( headers ) is !list {
        panic!("TypeError ( "Headers must be a list of name/value tuples" )");
        self . _headers = headers;
        if __debug__ {
        for k , v in headers .iter() {
        self . _convert_string_type ( k );
        self . _convert_string_type ( v );
        pub fn _convert_string_type ( &self, value )  {
        "Convert/check value type.";
        if type ( value ) is str {
        return  value;
        panic!("AssertionError ( "Header names/values must be"");
        " of type str (got {0})" . format ( repr ( value ) ) );
        pub fn __len__ ( self )  {
        "Return the total number of headers, including duplicates.";
        return  len ( self . _headers );
        pub fn __setitem__ ( &self, name , val )  {
        "Set the value of a header.";
        del self [ name ];
        self . _headers . append (;
        ( self . _convert_string_type ( name ) , self . _convert_string_type ( val ) ) );
        pub fn __delitem__ ( &self, name )  {
        "Delete all occurrences of a header, if present.

        Does *not* raise an exception if the header == missing.
        ";
        name = self . _convert_string_type ( name . lower ( ) );
        self . _headers [ : ] = [ kv for kv in self . _headers if kv [ 0 ] . lower ( ) != name ];
        pub fn __getitem__ ( &self, name )  {
        "Get the first header value for 'name'

        Return None /* Option */ if the header == missing instead of raising an exception.

        Note that if the header appeared multiple times, the first exactly which
        occurrence gets returned == undefined.  Use getall() to get all
        the values matching a header field name.
        ";
        return  self . get ( name );
        pub fn __contains__ ( &self, name )  {
        "Return true if the message contains the header.";
        return  self . get ( name ) is !None /* Option */;
        pub fn get_all ( &self, name )  {
        "Return a list of all the values for the named field.

        These will be sorted in the order they appeared in the original header
        list || were added to this instance, && may contain duplicates.  Any
        fields deleted && re-inserted are always appended to the header list.
        If no fields exist with the given name, returns an empty list.
        ";
        name = self . _convert_string_type ( name . lower ( ) );
        return  [ kv [ 1 ] for kv in self . _headers if kv [ 0 ] . lower ( ) == name ];
        pub fn get ( &self, name , default = None /* Option */ )  {
        "Get the first header value for 'name', || return 'default'";
        name = self . _convert_string_type ( name . lower ( ) );
        for k , v in self . _headers .iter() {
        if k . lower ( ) == name {
        return  v;
        return  default;
        pub fn keys ( self )  {
        "Return a list of all the header field names.

        These will be sorted in the order they appeared in the original header
        list, || were added to this instance, && may contain duplicates.
        Any fields deleted && re-inserted are always appended to the header
        list.
        ";
        return  [ k for k , v in self . _headers ];
        pub fn values ( self )  {
        "Return a list of all header values.

        These will be sorted in the order they appeared in the original header
        list, || were added to this instance, && may contain duplicates.
        Any fields deleted && re-inserted are always appended to the header
        list.
        ";
        return  [ v for k , v in self . _headers ];
        pub fn items ( self )  {
        "Get all the header fields && values.

        These will be sorted in the order they were in the original header
        list, || were added to this instance, && may contain duplicates.
        Any fields deleted && re-inserted are always appended to the header
        list.
        ";
        return  self . _headers [ : ];
        pub fn __repr__ ( self )  {
        return  "%s(%r)" % ( self . __class__ . __name__ , self . _headers );
        pub fn __str__ ( self )  {
        "str() returns the formatted headers, complete with end line,
        suitable for direct HTTP transmission.";
        return  "\r\n" . join ( [ "%s: %s" % kv for kv in self . _headers ] + [ "" , "" ] );
        pub fn __bytes__ ( self )  {
        return  str ( self ) . encode ( "iso-8859-1" );
        pub fn setdefault ( &self, name , value )  {
        "Return first matching header value for 'name', || 'value'

        If there == no header named 'name', add a new header with name 'name'
        && value 'value'.";
        result = self . get ( name );
        if result is None /* Option */ {
        self . _headers . append ( ( self . _convert_string_type ( name ) ,;
        self . _convert_string_type ( value ) ) );
        return  value;
        } else {
        return  result;
        pub fn add_header ( &self, _name , _value , ** _params )  {
        "Extended header setting.

        _name == the header field to add.  keyword arguments can be used to set
        additional parameters for the header field, with underscores converted
        to dashes.  Normally the parameter will be added as key="value" unless
        value == None /* Option */, in which case only the key will be added.

        Example:

        h.add_header('content-disposition', 'attachment', filename='bud.gif')

        Note that unlike the corresponding 'email.message' method, this does
        *not* handle '(charset, language, value)' tuples: all values must be
        strings || None /* Option */.
        ";
        parts = [ ];
        if _value is !None /* Option */ {
        _value = self . _convert_string_type ( _value );
        parts . append ( _value );
        for k , v in _params . items ( ) .iter() {
        k = self . _convert_string_type ( k );
        if v is None /* Option */ {
        parts . append ( k . replace ( "_" , "-" ) );
        } else {
        v = self . _convert_string_type ( v );
        parts . append ( _formatparam ( k . replace ( "_" , "-" ) , v ) );
        self . _headers . append ( ( self . _convert_string_type ( _name ) , "; " . join ( parts ) ) );
}

