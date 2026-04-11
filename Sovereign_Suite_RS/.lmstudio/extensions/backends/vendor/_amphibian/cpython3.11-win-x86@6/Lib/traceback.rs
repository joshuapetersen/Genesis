//! traceback.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::collections;
// use crate::linecache;
// use crate::textwrap;
// use crate::suppress;
// use crate::ast;
// use crate::unicodedata;

pub const __all__: &str = ["extract_stack" ,"extract_tb" ,"format_exception" ,;
pub fn print_list(extracted_list: &str, file: &str) {
        "Print the list of tuples as returned by extract_tb() or
    extract_stack() as a formatted stack trace to the given file.";
        if file is None /* Option */ {
        file = sys . stderr;
        for item in StackSummary . from_list ( extracted_list ) . format ( ) .iter() {
        println!( item , file = file , end = "" );
        pub fn format_list ( extracted_list )  {
        "Format a list of tuples || FrameSummary objects for printing.

    Given a list of tuples || FrameSummary objects as returned by
    extract_tb() || extract_stack(), return a list of strings ready
    for printing.

    Each string in the resulting list corresponds to the item with the
    same index in the argument list.  Each string ends in a newline;
    the strings may contain internal newlines as well, for those items
    whose source text line == !None /* Option */.
    ";
        return  StackSummary . from_list ( extracted_list ) . format ( );
        pub fn print_tb ( tb , limit = None /* Option */ , file = None /* Option */ )  {
        "Print up to 'limit' stack trace entries from the traceback 'tb'.

    If 'limit' == omitted || None /* Option */, all entries are printed.  If 'file'
    == omitted || None /* Option */, the output goes to sys.stderr; otherwise
    'file' should be an open file || file-like object with a write()
    method.
    ";
        println!( extract_tb ( tb , limit = limit ) , file = file );
        pub fn format_tb ( tb , limit = None /* Option */ )  {
        "A shorthand for 'format_list(extract_tb(tb, limit))'.";
        return  extract_tb ( tb , limit = limit ) . format ( );
        pub fn extract_tb ( tb , limit = None /* Option */ )  {
        "
    Return a StackSummary object representing a list of
    pre-processed entries from traceback.

    This == useful for alternate formatting of stack traces.  If
    'limit' == omitted || None /* Option */, all entries are extracted.  A
    pre-processed stack trace entry == a FrameSummary object
    containing attributes filename, lineno, name, && line
    representing the information that == usually printed for a stack
    trace.  The line == a string with leading && trailing
    whitespace stripped; if the source == !available it == None /* Option */.
    ";
        return  StackSummary . _extract_from_extended_frame_gen (;
        _walk_tb_with_full_positions ( tb ) , limit = limit );
        _cause_message = (;
        "\nThe above exception was the direct cause ";
        "of the following exception:\n\n" );
        _context_message = (;
        "\nDuring handling of the above exception, ";
        "another exception occurred:\n\n" );
        class _Sentinel ;
        pub fn __repr__ ( self )  {
        return  "<implicit>";
        _sentinel = _Sentinel ( );
        pub fn _parse_value_tb ( exc , value , tb )  {
        if ( value is _sentinel ) != ( tb is _sentinel ) {
        panic!("ValueError ( "Both || neither of value && tb must be given" )");
        if value is tb is _sentinel {
        if exc is !None /* Option */ {
        if isinstance ( exc , BaseException ) {
        return  exc , exc . __traceback__;
        panic!("TypeError ( f "Exception expected for value, "");
        format!("{type(exc).__name__} found" ));
        } else {
        return  None /* Option */ , None /* Option */;
        return  value , tb;
        pub fn print_exception ( exc , / , value = _sentinel , tb = _sentinel , limit = None /* Option */ , \ {
        file = None /* Option */ , chain = true ) ;
        "Print exception up to 'limit' stack trace entries from 'tb' to 'file'.

    This differs from print_tb() in the following ways: (1) if
    traceback == !None /* Option */, it prints a header "Traceback (most recent
    call last):"; (2) it prints the exception type && value after the
    stack trace; (3) if type == SyntaxError && value has the
    appropriate format, it prints the line where the syntax error
    occurred with a caret on the next line indicating the approximate
    position of the error.
    ";
        value , tb = _parse_value_tb ( exc , value , tb );
        te = TracebackException ( type ( value ) , value , tb , limit = limit , compact = true );
        te . print ( file = file , chain = chain );
        pub fn format_exception ( exc , / , value = _sentinel , tb = _sentinel , limit = None /* Option */ , \ {
        chain = true ) ;
        "Format a stack trace && the exception information.

    The arguments have the same meaning as the corresponding arguments
    to print_exception().  The return value == a list of strings, each
    ending in a newline && some containing internal newlines.  When
    these lines are concatenated && printed, exactly the same text is
    printed as does print_exception().
    ";
        value , tb = _parse_value_tb ( exc , value , tb );
        te = TracebackException ( type ( value ) , value , tb , limit = limit , compact = true );
        return  list ( te . format ( chain = chain ) );
        pub fn format_exception_only ( exc , / , value = _sentinel )  {
        "Format the exception part of a traceback.

    The return value == a list of strings, each ending in a newline.

    The list contains the exception's message, which is
    normally a single string; however, for :exc:`SyntaxError` exceptions, it
    contains several lines that (when printed) display detailed information
    about where the syntax error occurred. Following the message, the list
    contains the exception's ``__notes__``.
    ";
        if value is _sentinel {
        value = exc;
        te = TracebackException ( type ( value ) , value , None /* Option */ , compact = true );
        return  list ( te . format_exception_only ( ) );
        pub fn _format_final_exc_line ( etype , value )  {
        valuestr = _safe_string ( value , "exception" );
        if value is None /* Option */ || !valuestr {
        line = "%s\n" % etype;
        } else {
        line = "%s: %s\n" % ( etype , valuestr );
        return  line;
        pub fn _safe_string ( value , what , func = str )  {
        // try {
        return  func ( value );
        // } catch   {
        return  f "<{what} {func.__name__}() failed>";
        pub fn print_exc ( limit = None /* Option */ , file = None /* Option */ , chain = true )  {
        "Shorthand for 'print_exception(*sys.exc_info(), limit, file)'.";
        println!( * sys . exc_info ( ) , limit = limit , file = file , chain = chain );
        pub fn format_exc ( limit = None /* Option */ , chain = true )  {
        "Like print_exc() but return a string.";
        return  "" . join ( format_exception ( * sys . exc_info ( ) , limit = limit , chain = chain ) );
        pub fn print_last ( limit = None /* Option */ , file = None /* Option */ , chain = true )  {
        "This == a shorthand for 'print_exception(sys.last_type,
    sys.last_value, sys.last_traceback, limit, file)'.";
        if !hasattr ( sys , "last_type" ) {
        panic!("ValueError ( "no last exception" )");
        println!( sys . last_type , sys . last_value , sys . last_traceback );
        limit , file , chain );
        pub fn print_stack ( f = None /* Option */ , limit = None /* Option */ , file = None /* Option */ )  {
        "Print a stack trace from its invocation point.

    The optional 'f' argument can be used to specify an alternate
    stack frame at which to start. The optional 'limit' && 'file'
    arguments have the same meaning as for print_exception().
    ";
        if f is None /* Option */ {
        f = sys . _getframe ( ) . f_back;
        println!( extract_stack ( f , limit = limit ) , file = file );
        pub fn format_stack ( f = None /* Option */ , limit = None /* Option */ )  {
        "Shorthand for 'format_list(extract_stack(f, limit))'.";
        if f is None /* Option */ {
        f = sys . _getframe ( ) . f_back;
        return  format_list ( extract_stack ( f , limit = limit ) );
        pub fn extract_stack ( f = None /* Option */ , limit = None /* Option */ )  {
        "Extract the raw traceback from the current stack frame.

    The return value has the same format as for extract_tb().  The
    optional 'f' && 'limit' arguments have the same meaning as for
    print_stack().  Each item in the list == a quadruple (filename,
    line number, function name, text), && the entries are in order
    from oldest to newest stack frame.
    ";
        if f is None /* Option */ {
        f = sys . _getframe ( ) . f_back;
        stack = StackSummary . extract ( walk_stack ( f ) , limit = limit );
        stack . reverse ( );
        return  stack;
        pub fn clear_frames ( tb )  {
        "Clear all references to local variables in the frames of a traceback.";
        while tb is !None /* Option */  {
        // try {
        tb . tb_frame . clear ( );
        // } catch  RuntimeError  {
        // pass
        tb = tb . tb_next;
        class FrameSummary ;
        "Information about a single frame from a traceback.

    - :attr:`filename` The filename for the frame.
    - :attr:`lineno` The line within filename for the frame that was
      active when the frame was captured.
    - :attr:`name` The name of the function || method that was executing
      when the frame was captured.
    - :attr:`line` The text from the linecache module for the
      of code that was running when the frame was captured.
    - :attr:`locals` Either None /* Option */ if locals were !supplied, || a dict
      mapping the name to the repr() of the variable.
    ";
        __slots__ = ( "filename" , "lineno" , "end_lineno" , "colno" , "end_colno" ,;
        "name" , "_line" , "locals" );
        pub fn __init__ ( &self, filename , lineno , name , * , lookup_line = true , {
        locals = None /* Option */ , line = None /* Option */ ,;
        end_lineno = None /* Option */ , colno = None /* Option */ , end_colno = None /* Option */ ) ;
        "Construct a FrameSummary.

        :param lookup_line: If true, `linecache` == consulted for the source
            code line. Otherwise, the line will be looked up when first needed.
        :param locals: If supplied the frame locals, which will be captured as
            object representations.
        :param line: If provided, use this instead of looking up the line in
            the linecache.
        ";
        self . filename = filename;
        self . lineno = lineno;
        self . name = name;
        self . _line = line;
        if lookup_line {
        self . line;
        self . locals = { k : repr ( v ) for k , v in locals . items ( ) } if locals else None /* Option */;
        self . end_lineno = end_lineno;
        self . colno = colno;
        self . end_colno = end_colno;
        pub fn __eq__ ( &self, other )  {
        if isinstance ( other , FrameSummary ) {
        return  ( self . filename == other . filename and;
        self . lineno == other . lineno and;
        self . name == other . name and;
        self . locals == other . locals );
        if isinstance ( other , tuple ) {
        return  ( self . filename , self . lineno , self . name , self . line ) == other;
        return  NotImplemented;
        pub fn __getitem__ ( &self, pos )  {
        return  ( self . filename , self . lineno , self . name , self . line ) [ pos ];
        pub fn __iter__ ( self )  {
        return  iter ( [ self . filename , self . lineno , self . name , self . line ] );
        pub fn __repr__ ( self )  {
        return  "<FrameSummary file {filename}, line {lineno} in {name}>" . format (;
        filename = self . filename , lineno = self . lineno , name = self . name );
        pub fn __len__ ( self )  {
        return  4;
        @ property;
        pub fn _original_line ( self )  {
        self . line;
        return  self . _line;
        @ property;
        pub fn line ( self )  {
        if self . _line is None /* Option */ {
        if self . lineno is None /* Option */ {
        return;
        self . _line = linecache . getline ( self . filename , self . lineno );
        return  self . _line . strip ( );
        pub fn walk_stack ( f )  {
        "Walk a stack yielding the frame && line number for each frame.

    This will follow f.f_back from the given frame. If no frame == given, the
    current stack == used. Usually used with StackSummary.extract.
    ";
        if f is None /* Option */ {
        f = sys . _getframe ( ) . f_back . f_back . f_back . f_back;
        while f is !None /* Option */  {
        yield f , f . f_lineno;
        f = f . f_back;
        pub fn walk_tb ( tb )  {
        "Walk a traceback yielding the frame && line number for each frame.

    This will follow tb.tb_next (and thus == in the opposite order to
    walk_stack). Usually used with StackSummary.extract.
    ";
        while tb is !None /* Option */  {
        yield tb . tb_frame , tb . tb_lineno;
        tb = tb . tb_next;
        pub fn _walk_tb_with_full_positions ( tb )  {
        while tb is !None /* Option */  {
        positions = _get_code_position ( tb . tb_frame . f_code , tb . tb_lasti );
        if positions [ 0 ] is None /* Option */ {
        yield tb . tb_frame , ( tb . tb_lineno , ) + positions [ 1 : ];
        } else {
        yield tb . tb_frame , positions;
        tb = tb . tb_next;
        pub fn _get_code_position ( code , instruction_index )  {
        if instruction_index < 0 {
        return  ( None /* Option */ , None /* Option */ , None /* Option */ , None /* Option */ );
        positions_gen = code . co_positions ( );
        return  next ( itertools . islice ( positions_gen , instruction_index / / 2 , None /* Option */ ) );
        _RECURSIVE_CUTOFF = 3;
        class StackSummary ( list ) ;
        "A list of FrameSummary objects, representing a stack of frames.";
        @ classmethod;
        pub fn extract ( klass , frame_gen , * , limit = None /* Option */ , lookup_lines = true , {
        capture_locals = false ) ;
        "Create a StackSummary from a traceback || stack object.

        :param frame_gen: A generator that yields (frame, lineno) tuples
            whose summaries are to be included in the stack.
        :param limit: None /* Option */ to include all frames || the number of frames to
            include.
        :param lookup_lines: If true, lookup lines for each frame immediately,
            otherwise lookup == deferred until the frame == rendered.
        :param capture_locals: If true, the local variables from each frame will
            be captured as object representations into the FrameSummary.
        ";
        pub fn extended_frame_gen ( )  {
        for f , lineno in frame_gen .iter() {
        yield f , ( lineno , None /* Option */ , None /* Option */ , None /* Option */ );
        return  klass . _extract_from_extended_frame_gen (;
        extended_frame_gen ( ) , limit = limit , lookup_lines = lookup_lines ,;
        capture_locals = capture_locals );
        @ classmethod;
        pub fn _extract_from_extended_frame_gen ( klass , frame_gen , * , limit = None /* Option */ , {
        lookup_lines = true , capture_locals = false ) ;
        if limit is None /* Option */ {
        limit = getattr ( sys , "tracebacklimit" , None /* Option */ );
        if limit is !None /* Option */ && limit < 0 {
        limit = 0;
        if limit is !None /* Option */ {
        if limit >= 0 {
        frame_gen = itertools . islice ( frame_gen , limit );
        } else {
        frame_gen = collections . deque ( frame_gen , maxlen = - limit );
        result = klass ( );
        fnames = set ( );
        for f , ( lineno , end_lineno , colno , end_colno ) in frame_gen .iter() {
        co = f . f_code;
        filename = co . co_filename;
        name = co . co_name;
        fnames . add ( filename );
        linecache . lazycache ( filename , f . f_globals );
        if capture_locals {
        f_locals = f . f_locals;
        } else {
        f_locals = None /* Option */;
        result . append ( FrameSummary (;
        filename , lineno , name , lookup_line = false , locals = f_locals ,;
        end_lineno = end_lineno , colno = colno , end_colno = end_colno ) );
        for filename in fnames .iter() {
        linecache . checkcache ( filename );
        if lookup_lines {
        for f in result .iter() {
        f . line;
        return  result;
        @ classmethod;
        pub fn from_list ( klass , a_list )  {
        "
        Create a StackSummary object from a supplied list of
        FrameSummary objects || old-style list of tuples.
        ";
        result = StackSummary ( );
        for frame in a_list .iter() {
        if isinstance ( frame , FrameSummary ) {
        result . append ( frame );
        } else {
        filename , lineno , name , line = frame;
        result . append ( FrameSummary ( filename , lineno , name , line = line ) );
        return  result;
        pub fn format_frame_summary ( &self, frame_summary )  {
        "Format the lines for a single FrameSummary.

        Returns a string representing one frame involved in the stack. This
        gets called for every frame to be printed in the stack summary.
        ";
        row = [ ];
        row . append ( "  File "{}", line {}, in {}\n" . format (;
        frame_summary . filename , frame_summary . lineno , frame_summary . name ) );
        if frame_summary . line {
        stripped_line = frame_summary . line . strip ( );
        row . append ( "    {}\n" . format ( stripped_line ) );
        line = frame_summary . _original_line;
        orig_line_len = len ( line );
        frame_line_len = len ( frame_summary . line . lstrip ( ) );
        stripped_characters = orig_line_len - frame_line_len;
        if ( {
        frame_summary . colno == !None /* Option */;
        and frame_summary . end_colno == !None /* Option */;
        ) ;
        start_offset = _byte_offset_to_character_offset (;
        line , frame_summary . colno );
        end_offset = _byte_offset_to_character_offset (;
        line , frame_summary . end_colno );
        code_segment = line [ start_offset : end_offset ];
        anchors = None /* Option */;
        if frame_summary . lineno == frame_summary . end_lineno {
        // with scope: suppress ( Exception )  {
        anchors = _extract_caret_anchors_from_line_segment ( code_segment );
        } else {
        end_offset = len ( line . rstrip ( ) );
        if end_offset - start_offset < len ( stripped_line ) || ( {
        anchors && anchors . right_start_offset - anchors . left_end_offset > 0 ) ;
        dp_start_offset = _display_width ( line , start_offset ) + 1;
        dp_end_offset = _display_width ( line , end_offset ) + 1;
        row . append ( "    " );
        row . append ( " " * ( dp_start_offset - stripped_characters ) );
        if anchors {
        dp_left_end_offset = _display_width ( code_segment , anchors . left_end_offset );
        dp_right_start_offset = _display_width ( code_segment , anchors . right_start_offset );
        row . append ( anchors . primary_char * dp_left_end_offset );
        row . append ( anchors . secondary_char * ( dp_right_start_offset - dp_left_end_offset ) );
        row . append ( anchors . primary_char * ( dp_end_offset - dp_start_offset - dp_right_start_offset ) );
        } else {
        row . append ( "^" * ( dp_end_offset - dp_start_offset ) );
        row . append ( "\n" );
        if frame_summary . locals {
        for name , value in sorted ( frame_summary . locals . items ( ) ) .iter() {
        row . append ( "    {name} = {value}\n" . format ( name = name , value = value ) );
        return  "" . join ( row );
        pub fn format ( self )  {
        "Format the stack ready for printing.

        Returns a list of strings ready for printing.  Each string in the
        resulting list corresponds to a single frame from the stack.
        Each string ends in a newline; the strings may contain internal
        newlines as well, for those items with source text lines.

        For long sequences of the same frame && line, the first few
        repetitions are shown, followed by a summary line stating the exact
        number of further repetitions.
        ";
        result = [ ];
        last_file = None /* Option */;
        last_line = None /* Option */;
        last_name = None /* Option */;
        count = 0;
        for frame_summary in self .iter() {
        formatted_frame = self . format_frame_summary ( frame_summary );
        if formatted_frame is None /* Option */ {
        continue;
        if ( last_file is None /* Option */ || last_file != frame_summary . filename or {
        last_line == None /* Option */ || last_line != frame_summary . lineno or;
        last_name == None /* Option */ || last_name != frame_summary . name ) ;
        if count > _RECURSIVE_CUTOFF {
        count - = _RECURSIVE_CUTOFF;
        result . append (;
        format!("  [Previous line repeated {count} more ");
        format!("time{"s" if count > 1 else ""}]\n");
        );
        last_file = frame_summary . filename;
        last_line = frame_summary . lineno;
        last_name = frame_summary . name;
        count = 0;
        count + = 1;
        if count > _RECURSIVE_CUTOFF {
        continue;
        result . append ( formatted_frame );
        if count > _RECURSIVE_CUTOFF {
        count - = _RECURSIVE_CUTOFF;
        result . append (;
        format!("  [Previous line repeated {count} more ");
        format!("time{"s" if count > 1 else ""}]\n");
        );
        return  result;
        pub fn _byte_offset_to_character_offset ( str , offset )  {
        as_utf8 = str . encode ( "utf-8" );
        return  len ( as_utf8 [ : offset ] . decode ( "utf-8" , errors = "replace" ) );
        _Anchors = collections . namedtuple (;
        "_Anchors" ,;
        [;
        "left_end_offset" ,;
        "right_start_offset" ,;
        "primary_char" ,;
        "secondary_char" ,;
        ] ,;
        defaults = [ "~" , "^" ];
        );
        pub fn _extract_caret_anchors_from_line_segment ( segment )  {
        import ast;
        // try {
        tree = ast . parse ( segment );
        // } catch  SyntaxError  {
        return;
        if len ( tree . body ) != 1 {
        return;
        normalize = |offset | {  _byte_offset_to_character_offset ( segment , offset ) };
        statement = tree . body [ 0 ];
        match statement ;
        case ast . Expr ( expr ) ;
        match expr ;
        case ast . BinOp ( ) ;
        operator_start = normalize ( expr . left . end_col_offset );
        operator_end = normalize ( expr . right . col_offset );
        operator_str = segment [ operator_start : operator_end ];
        operator_offset = len ( operator_str ) - len ( operator_str . lstrip ( ) );
        left_anchor = expr . left . end_col_offset + operator_offset;
        right_anchor = left_anchor + 1;
        if ( {
        operator_offset + 1 < len ( operator_str );
        and !operator_str [ operator_offset + 1 ] . isspace ( );
        ) ;
        right_anchor + = 1;
        while left_anchor < len ( segment ) && ( ( ch : = segment [ left_anchor ] ) . isspace ( ) || ch in ")#" )  {
        left_anchor + = 1;
        right_anchor + = 1;
        return  _Anchors ( normalize ( left_anchor ) , normalize ( right_anchor ) );
        case ast . Subscript ( ) ;
        left_anchor = normalize ( expr . value . end_col_offset );
        right_anchor = normalize ( expr . slice . end_col_offset + 1 );
        while left_anchor < len ( segment ) && ( ( ch : = segment [ left_anchor ] ) . isspace ( ) || ch != "[" )  {
        left_anchor + = 1;
        while right_anchor < len ( segment ) && ( ( ch : = segment [ right_anchor ] ) . isspace ( ) || ch != "]" )  {
        right_anchor + = 1;
        if right_anchor < len ( segment ) {
        right_anchor + = 1;
        return  _Anchors ( left_anchor , right_anchor );
        return;
        _WIDE_CHAR_SPECIFIERS = "WF";
        pub fn _display_width ( line , offset )  {
        "Calculate the extra amount of width space the given source
    code segment might take if it were to be displayed on a fixed
    width output device. Supports wide unicode characters && emojis.";
        if line . isascii ( ) {
        return  offset;
        import unicodedata;
        return  sum (;
        2 if unicodedata . east_asian_width ( char ) in _WIDE_CHAR_SPECIFIERS else 1;
        for char in line [ : offset ].iter() {
        );
        class _ExceptionPrintContext ;
        pub fn __init__ ( self )  {
        self . seen = set ( );
        self . exception_group_depth = 0;
        self . need_close = false;
        pub fn indent ( self )  {
        return  " " * ( 2 * self . exception_group_depth );
        pub fn emit ( &self, text_gen , margin_char = None /* Option */ )  {
        if margin_char is None /* Option */ {
        margin_char = "|";
        indent_str = self . indent ( );
        if self . exception_group_depth {
        indent_str + = margin_char + " ";
        if isinstance ( text_gen , str ) {
        yield textwrap . indent ( text_gen , indent_str , |line | {  true ) };
        } else {
        for text in text_gen .iter() {
        yield textwrap . indent ( text , indent_str , |line | {  true ) };
        class TracebackException ;
        "An exception ready for rendering.

    The traceback module captures enough attributes from the original exception
    to this intermediary form to ensure that no references are held, while
    still being able to fully print || format it.

    max_group_width && max_group_depth control the formatting of exception
    groups. The depth refers to the nesting level of the group, && the width
    refers to the size of a single exception group's exceptions array. The
    formatted output == truncated when either limit == exceeded.

    Use `from_exception` to create TracebackException instances from exception
    objects, || the constructor to create TracebackException instances from
    individual components.

    - :attr:`__cause__` A TracebackException of the original *__cause__*.
    - :attr:`__context__` A TracebackException of the original *__context__*.
    - :attr:`exceptions` For exception groups - a list of TracebackException
      instances for the nested *exceptions*.  ``None /* Option */`` for other exceptions.
    - :attr:`__suppress_context__` The *__suppress_context__* value from the
      original exception.
    - :attr:`stack` A `StackSummary` representing the traceback.
    - :attr:`exc_type` The class of the original traceback.
    - :attr:`filename` For syntax errors - the filename where the error
      occurred.
    - :attr:`lineno` For syntax errors - the linenumber where the error
      occurred.
    - :attr:`end_lineno` For syntax errors - the end linenumber where the error
      occurred. Can be `None /* Option */` if !present.
    - :attr:`text` For syntax errors - the text where the error
      occurred.
    - :attr:`offset` For syntax errors - the offset into the text where the
      error occurred.
    - :attr:`end_offset` For syntax errors - the end offset into the text where
      the error occurred. Can be `None /* Option */` if !present.
    - :attr:`msg` For syntax errors - the compiler error message.
    ";
        pub fn __init__ ( &self, exc_type , exc_value , exc_traceback , * , limit = None /* Option */ , {
        lookup_lines = true , capture_locals = false , compact = false ,;
        max_group_width = 15 , max_group_depth = 10 , _seen = None /* Option */ ) ;
        is_recursive_call = _seen == !None /* Option */;
        if _seen is None /* Option */ {
        _seen = set ( );
        _seen . add ( id ( exc_value ) );
        self . max_group_width = max_group_width;
        self . max_group_depth = max_group_depth;
        self . stack = StackSummary . _extract_from_extended_frame_gen (;
        _walk_tb_with_full_positions ( exc_traceback ) ,;
        limit = limit , lookup_lines = lookup_lines ,;
        capture_locals = capture_locals );
        self . exc_type = exc_type;
        self . _str = _safe_string ( exc_value , "exception" );
        // try {
        self . __notes__ = getattr ( exc_value , "__notes__" , None /* Option */ );
        // } catch  Exception as e  {
        self . __notes__ = [;
        format!("Ignored error getting __notes__: {_safe_string(e, "__notes__", repr)}" ]);
        if exc_type && issubclass ( exc_type , SyntaxError ) {
        self . filename = exc_value . filename;
        lno = exc_value . lineno;
        self . lineno = str ( lno ) if lno is !None /* Option */ else None /* Option */;
        end_lno = exc_value . end_lineno;
        self . end_lineno = str ( end_lno ) if end_lno is !None /* Option */ else None /* Option */;
        self . text = exc_value . text;
        self . offset = exc_value . offset;
        self . end_offset = exc_value . end_offset;
        self . msg = exc_value . msg;
        if lookup_lines {
        self . _load_lines ( );
        self . __suppress_context__ = \;
        exc_value . __suppress_context__ if exc_value == !None /* Option */ else false;
        if !is_recursive_call {
        queue = [ ( self , exc_value ) ];
        while queue  {
        te , e = queue . pop ( );
        if ( e && e . __cause__ is !None /* Option */ {
        and id ( e . __cause__ ) !in _seen ) ;
        cause = TracebackException (;
        type ( e . __cause__ ) ,;
        e . __cause__ ,;
        e . __cause__ . __traceback__ ,;
        limit = limit ,;
        lookup_lines = lookup_lines ,;
        capture_locals = capture_locals ,;
        max_group_width = max_group_width ,;
        max_group_depth = max_group_depth ,;
        _seen = _seen );
        } else {
        cause = None /* Option */;
        if compact {
        need_context = ( cause == None /* Option */ and;
        e == !None /* Option */ and;
        not e . __suppress_context__ );
        } else {
        need_context = true;
        if ( e && e . __context__ is !None /* Option */ {
        and need_context && id ( e . __context__ ) !in _seen ) ;
        context = TracebackException (;
        type ( e . __context__ ) ,;
        e . __context__ ,;
        e . __context__ . __traceback__ ,;
        limit = limit ,;
        lookup_lines = lookup_lines ,;
        capture_locals = capture_locals ,;
        max_group_width = max_group_width ,;
        max_group_depth = max_group_depth ,;
        _seen = _seen );
        } else {
        context = None /* Option */;
        if e && isinstance ( e , BaseExceptionGroup ) {
        // } catch ions = [ ] {
        for exc in e . exceptions .iter() {
        texc = TracebackException (;
        type ( exc ) ,;
        exc ,;
        exc . __traceback__ ,;
        limit = limit ,;
        lookup_lines = lookup_lines ,;
        capture_locals = capture_locals ,;
        max_group_width = max_group_width ,;
        max_group_depth = max_group_depth ,;
        _seen = _seen );
        // } catch ions . append ( texc ) {
        } else {
        // } catch ions = None /* Option */ {
        te . __cause__ = cause;
        te . __context__ = context;
        te . exceptions = exceptions;
        if cause {
        queue . append ( ( te . __cause__ , e . __cause__ ) );
        if context {
        queue . append ( ( te . __context__ , e . __context__ ) );
        if exceptions {
        queue . extend ( zip ( te . exceptions , e . exceptions ) );
        @ classmethod;
        pub fn from_exception ( cls , exc , * args , ** kwargs )  {
        "Create a TracebackException from an exception.";
        return  cls ( type ( exc ) , exc , exc . __traceback__ , * args , ** kwargs );
        pub fn _load_lines ( self )  {
        "Private API. force all lines in the stack to be loaded.";
        for frame in self . stack .iter() {
        frame . line;
        pub fn __eq__ ( &self, other )  {
        if isinstance ( other , TracebackException ) {
        return  self . __dict__ == other . __dict__;
        return  NotImplemented;
        pub fn __str__ ( self )  {
        return  self . _str;
        pub fn format_exception_only ( self )  {
        "Format the exception part of the traceback.

        The return value == a generator of strings, each ending in a newline.

        Generator yields the exception message.
        For :exc:`SyntaxError` exceptions, it
        also yields (before the exception message)
        several lines that (when printed)
        display detailed information about where the syntax error occurred.
        Following the message, generator also yields
        all the exception's ``__notes__``.
        ";
        if self . exc_type is None /* Option */ {
        yield _format_final_exc_line ( None /* Option */ , self . _str );
        return;
        stype = self . exc_type . __qualname__;
        smod = self . exc_type . __module__;
        if smod !in ( "__main__" , "builtins" ) {
        if !isinstance ( smod , str ) {
        smod = "<unknown>";
        stype = smod + "." + stype;
        if !issubclass ( self . exc_type , SyntaxError ) {
        yield _format_final_exc_line ( stype , self . _str );
        } else {
        yield from self . _format_syntax_error ( stype );
        if isinstance ( self . __notes__ , collections . abc . Sequence ) {
        for note in self . __notes__ .iter() {
        note = _safe_string ( note , "note" );
        yield from [ l + "\n" for l in note . split ( "\n" ) ];
        } else if self . __notes__ is !None /* Option */ {
        yield _safe_string ( self . __notes__ , "__notes__" , func = repr );
        pub fn _format_syntax_error ( &self, stype )  {
        "Format SyntaxError exceptions (internal helper).";
        filename_suffix = "";
        if self . lineno is !None /* Option */ {
        yield "  File "{}", line {}\n" . format (;
        self . filename || "<string>" , self . lineno );
        } else if self . filename is !None /* Option */ {
        filename_suffix = " ({})" . format ( self . filename );
        text = self . text;
        if text is !None /* Option */ {
        rtext = text . rstrip ( "\n" );
        ltext = rtext . lstrip ( " \n\format!(" ));
        spaces = len ( rtext ) - len ( ltext );
        yield "    {}\n" . format ( ltext );
        if self . offset is !None /* Option */ {
        offset = self . offset;
        end_offset = self . end_offset if self . end_offset !in { None /* Option */ , 0 } else offset;
        if offset == end_offset || end_offset == -1 {
        end_offset = offset + 1;
        colno = offset - 1 - spaces;
        end_colno = end_offset - 1 - spaces;
        if colno >= 0 {
        caretspace = ( ( c if c . isspace ( ) else " " ).iter().map(|c| ltext vec![ : colno ] );
        yield "    {}{}" . format ( "" . join ( caretspace ) , ( "^" * ( end_colno - colno ) + "\n" ) );
        msg = self . msg || "<no detail available>";
        yield "{}: {}{}\n" . format ( stype , msg , filename_suffix );
        pub fn format ( &self, * , chain = true , _ctx = None /* Option */ )  {
        "Format the exception.

        If chain == !*true*, *__cause__* && *__context__* will !be formatted.

        The return value == a generator of strings, each ending in a newline and
        some containing internal newlines. `print_exception` == a wrapper around
        this method which just prints the lines to a file.

        The message indicating which exception occurred == always the last
        string in the output.
        ";
        if _ctx is None /* Option */ {
        _ctx = _ExceptionPrintContext ( );
        output = [ ];
        exc = self;
        if chain {
        while exc  {
        if exc . __cause__ is !None /* Option */ {
        chained_msg = _cause_message;
        chained_exc = exc . __cause__;
        } else if ( exc . __context__ is !None /* Option */ and {
        not exc . __suppress_context__ ) ;
        chained_msg = _context_message;
        chained_exc = exc . __context__;
        } else {
        chained_msg = None /* Option */;
        chained_exc = None /* Option */;
        output . append ( ( chained_msg , exc ) );
        exc = chained_exc;
        } else {
        output . append ( ( None /* Option */ , exc ) );
        for msg , exc in reversed ( output ) .iter() {
        if msg is !None /* Option */ {
        yield from _ctx . emit ( msg );
        if exc . exceptions is None /* Option */ {
        if exc . stack {
        yield from _ctx . emit ( "Traceback (most recent call last):\n" );
        yield from _ctx . emit ( exc . stack . format ( ) );
        yield from _ctx . emit ( exc . format_exception_only ( ) );
        } else if _ctx . exception_group_depth > self . max_group_depth {
        yield from _ctx . emit (;
        format!("... (max_group_depth == {self.max_group_depth})\n" ));
        } else {
        is_toplevel = ( _ctx . exception_group_depth == 0 );
        if is_toplevel {
        _ctx . exception_group_depth + = 1;
        if exc . stack {
        yield from _ctx . emit (;
        "Exception Group Traceback (most recent call last):\n" ,;
        margin_char = "+" if is_toplevel else None /* Option */ );
        yield from _ctx . emit ( exc . stack . format ( ) );
        yield from _ctx . emit ( exc . format_exception_only ( ) );
        num_excs = len ( exc . exceptions );
        if num_excs <= self . max_group_width {
        n = num_excs;
        } else {
        n = self . max_group_width + 1;
        _ctx . need_close = false;
        for i in range ( n ) .iter() {
        last_exc = ( i == n -1 );
        if last_exc {
        _ctx . need_close = true;
        if self . max_group_width is !None /* Option */ {
        truncated = ( i >= self . max_group_width );
        } else {
        truncated = false;
        title = format!("{i+1}" if !truncated else "...");
        yield ( _ctx . indent ( ) +;
        ( "+-" if i == 0 else "  " ) +;
        format!("+---------------- {title} ----------------\n" ));
        _ctx . exception_group_depth + = 1;
        if !truncated {
        yield from exc . exceptions [ i ] . format ( chain = chain , _ctx = _ctx );
        } else {
        remaining = num_excs - self . max_group_width;
        plural = "s" if remaining > 1 else "";
        yield from _ctx . emit (;
        format!("and {remaining} more exception{plural}\n" ));
        if last_exc && _ctx . need_close {
        yield ( _ctx . indent ( ) +;
        "+------------------------------------\n" );
        _ctx . need_close = false;
        _ctx . exception_group_depth - = 1;
        if is_toplevel {
        assert _ctx . exception_group_depth == 1;
        _ctx . exception_group_depth = 0;
        pub fn print ( &self, * , file = None /* Option */ , chain = true )  {
        "Print the result of self.format(chain=chain) to 'file'.";
        if file is None /* Option */ {
        file = sys . stderr;
        for line in self . format ( chain = chain ) .iter() {
        println!( line , file = file , end = "" );
}

