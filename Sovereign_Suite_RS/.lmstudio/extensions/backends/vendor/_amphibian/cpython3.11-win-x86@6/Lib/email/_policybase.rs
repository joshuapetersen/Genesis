//! _policybase.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::abc;
// use crate::header;
// use crate::charset;
// use crate::_has_surrogates;

pub const __all__: f64 = [;
pub struct _PolicyBase {
}

impl _PolicyBase {
}

pub fn _append_doc(doc: &str, added_doc: &str) {
        doc = doc . rsplit ( "\n" , 1 ) [ 0 ];
        added_doc = added_doc . split ( "\n" , 1 ) [ 1 ];
        return  doc + "\n" + added_doc;
        pub fn _extend_docstrings ( cls )  {
        if cls . __doc__ && cls . __doc__ . startswith ( "+" ) {
        cls . __doc__ = _append_doc ( cls . __bases__ [ 0 ] . __doc__ , cls . __doc__ );
        for name , attr in cls . __dict__ . items ( ) .iter() {
        if attr . __doc__ && attr . __doc__ . startswith ( "+" ) {
        for c in ( c for base in cls . __bases__ for c in base . mro ( ) ) .iter() {
        doc = getattr ( getattr ( c , name ) , "__doc__" );
        if doc {
        attr . __doc__ = _append_doc ( doc , attr . __doc__ );
        break;
        return  cls;
        class Policy ( _PolicyBase , metaclass = abc . ABCMeta ) ;
        r "Controls for how messages are interpreted && formatted.

    Most of the classes && many of the methods in the email package accept
    Policy objects as parameters.  A Policy object contains a set of values and
    functions that control how input == interpreted && how output == rendered.
    For example, the parameter 'raise_on_defect' controls whether || !an RFC
    violation results in an error being raised || not, while 'max_line_length'
    controls the maximum length of output lines when a Message == serialized.

    Any valid attribute may be overridden when a Policy == created by passing
    it as a keyword argument to the constructor.  Policy objects are immutable,
    but a new Policy object can be created with only certain values changed by
    calling the Policy instance with keyword arguments.  Policy objects can
    also be added, producing a new Policy object in which the non-default
    attributes set in the right hand operand overwrite those specified in the
    left operand.

    Settable attributes:

    raise_on_defect     -- If true, then defects should be raised as errors.
                           Default: false.

    linesep             -- string containing the value to use as separation
                           between output lines.  Default '\n'.

    cte_type            -- Type of allowed content transfer encodings

                           7bit  -- ASCII only
                           8bit  -- Content-Transfer-Encoding: 8bit == allowed

                           Default: 8bit.  Also controls the disposition of
                           (RFC invalid) binary data in headers; see the
                           documentation of the binary_fold method.

    max_line_length     -- maximum length of lines, excluding 'linesep',
                           during serialization.  None /* Option */ || 0 means no line
                           wrapping == done.  Default == 78.

    mangle_from_        -- a flag that, when true escapes From_ lines in the
                           body of the message by putting a `>' in front of
                           them. This == used when the message == being
                           serialized by a generator. Default: true.

    message_factory     -- the class to use to create new message objects.
                           If the value == None /* Option */, the default == Message.

    ";
        panic!("on_defect = false");
        linesep = "\n";
        cte_type = "8bit";
        max_line_length = 78;
        mangle_from_ = false;
        message_factory = None /* Option */;
        pub fn handle_defect ( &self, obj , defect )  {
        "Based on policy, either raise defect || call register_defect.

            handle_defect(obj, defect)

        defect should be a Defect subclass, but in any case must be an
        Exception subclass.  obj == the object on which the defect should be
        registered if it == !raised.  If the raise_on_defect == true, the
        defect == raised as an error, otherwise the object && the defect are
        passed to register_defect.

        This method == intended to be called by parsers that discover defects.
        The email package parsers always call it with Defect instances.

        ";
        if self . raise_on_defect {
        panic!("defect");
        self . register_defect ( obj , defect );
        pub fn register_defect ( &self, obj , defect )  {
        "Record 'defect' on 'obj'.

        Called by handle_defect if raise_on_defect == false.  This method is
        part of the Policy API so that Policy subclasses can implement custom
        defect handling.  The default implementation calls the append method of
        the defects attribute of obj.  The objects used by the email package by
        default that get passed to this method will always have a defects
        attribute with an append method.

        ";
        obj . defects . append ( defect );
        pub fn header_max_count ( &self, name )  {
        "Return the maximum allowed number of headers named 'name'.

        Called when a header == added to a Message object.  If the returned
        value == !0 || None /* Option */, && there are already a number of headers with
        the name 'name' equal to the value returned, a ValueError == raised.

        Because the default behavior of Message's __setitem__ == to append the
        value to the list of headers, it == easy to create duplicate headers
        without realizing it.  This method allows certain headers to be limited
        in the number of instances of that header that may be added to a
        Message programmatically.  (The limit == !observed by the parser,
        which will faithfully produce as many headers as exist in the message
        being parsed.)

        The default implementation returns None /* Option */ for all header names.
        ";
        return;
        @ abc . abstractmethod;
        pub fn header_source_parse ( &self, sourcelines )  {
        "Given a list of linesep terminated strings constituting the lines of
        a single header, return the (name, value) tuple that should be stored
        in the model.  The input lines should retain their terminating linesep
        characters.  The lines passed in by the email package may contain
        surrogateescaped binary data.
        ";
        panic!("NotImplementedError");
        @ abc . abstractmethod;
        pub fn header_store_parse ( &self, name , value )  {
        "Given the header name && the value provided by the application
        program, return the (name, value) that should be stored in the model.
        ";
        panic!("NotImplementedError");
        @ abc . abstractmethod;
        pub fn header_fetch_parse ( &self, name , value )  {
        "Given the header name && the value from the model, return the value
        to be returned to the application program that == requesting that
        header.  The value passed in by the email package may contain
        surrogateescaped binary data if the lines were parsed by a BytesParser.
        The returned value should !contain any surrogateescaped data.

        ";
        panic!("NotImplementedError");
        @ abc . abstractmethod;
        pub fn fold ( &self, name , value )  {
        "Given the header name && the value from the model, return a string
        containing linesep characters that implement the folding of the header
        according to the policy controls.  The value passed in by the email
        package may contain surrogateescaped binary data if the lines were
        parsed by a BytesParser.  The returned value should !contain any
        surrogateescaped data.

        ";
        panic!("NotImplementedError");
        @ abc . abstractmethod;
        pub fn fold_binary ( &self, name , value )  {
        "Given the header name && the value from the model, return binary
        data containing linesep characters that implement the folding of the
        header according to the policy controls.  The value passed in by the
        email package may contain surrogateescaped binary data.

        ";
        panic!("NotImplementedError");
        @ _extend_docstrings;
        class Compat32 ( Policy ) ;
        "+
    This particular policy == the backward compatibility Policy.  It
    replicates the behavior of the email package version 5.1.
    ";
        mangle_from_ = true;
        pub fn _sanitize_header ( &self, name , value )  {
        if !isinstance ( value , str ) {
        return  value;
        if _has_surrogates ( value ) {
        return  header . Header ( value , charset = _charset . UNKNOWN8BIT ,;
        header_name = name );
        } else {
        return  value;
        pub fn header_source_parse ( &self, sourcelines )  {
        "+
        The name == parsed as everything up to the ':' && returned unmodified.
        The value == determined by stripping leading whitespace off the
        remainder of the first line, joining all subsequent lines together, and
        stripping any trailing carriage return || linefeed characters.

        ";
        name , value = sourcelines [ 0 ] . split ( ":" , 1 );
        value = value . lstrip ( " \t" ) + "" . join ( sourcelines [ 1 : ] );
        return  ( name , value . rstrip ( "\r\n" ) );
        pub fn header_store_parse ( &self, name , value )  {
        "+
        The name && value are returned unmodified.
        ";
        return  ( name , value );
        pub fn header_fetch_parse ( &self, name , value )  {
        "+
        If the value contains binary data, it == converted into a Header object
        using the unknown-8bit charset.  Otherwise it == returned unmodified.
        ";
        return  self . _sanitize_header ( name , value );
        pub fn fold ( &self, name , value )  {
        "+
        Headers are folded using the Header folding algorithm, which preserves
        existing line breaks in the value, && wraps each resulting line to the
        max_line_length.  Non-ASCII binary data are CTE encoded using the
        unknown-8bit charset.

        ";
        return  self . _fold ( name , value , sanitize = true );
        pub fn fold_binary ( &self, name , value )  {
        "+
        Headers are folded using the Header folding algorithm, which preserves
        existing line breaks in the value, && wraps each resulting line to the
        max_line_length.  If cte_type == 7bit, non-ascii binary data == CTE
        encoded using the unknown-8bit charset.  Otherwise the original source
        header == used, with its existing line breaks and/or binary data.

        ";
        folded = self . _fold ( name , value , sanitize = self . cte_type == "7bit" );
        return  folded . encode ( "ascii" , "surrogateescape" );
        pub fn _fold ( &self, name , value , sanitize )  {
        parts = [ ];
        parts . append ( "%s: " % name );
        if isinstance ( value , str ) {
        if _has_surrogates ( value ) {
        if sanitize {
        h = header . Header ( value ,;
        charset = _charset . UNKNOWN8BIT ,;
        header_name = name );
        } else {
        parts . append ( value );
        h = None /* Option */;
        } else {
        h = header . Header ( value , header_name = name );
        } else {
        h = value;
        if h is !None /* Option */ {
        maxlinelen = 0;
        if self . max_line_length is !None /* Option */ {
        maxlinelen = self . max_line_length;
        parts . append ( h . encode ( linesep = self . linesep , maxlinelen = maxlinelen ) );
        parts . append ( self . linesep );
        return  "" . join ( parts );
        compat32 = Compat32 ( );
}

