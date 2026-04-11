//! dis.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use std::collections;
// use crate::opcode::{};
// use crate::argparse;

pub const __all__: &str = ["code_info" ,"dis" ,"disassemble" ,"distb" ,"disco" ,;
pub const _have_code: f64 = ( types . MethodType , types . FunctionType , types . CodeType ,;
pub const FORMAT_VALUE: &str = opmap ["FORMAT_VALUE" ];
pub const FORMAT_VALUE_CONVERTERS: f64 = (;
pub const MAKE_FUNCTION: &str = opmap ["MAKE_FUNCTION" ];
pub const MAKE_FUNCTION_FLAGS: &str = ("defaults" ,"kwdefaults" ,"annotations" ,"closure" );
pub const LOAD_CONST: &str = opmap ["LOAD_CONST" ];
pub const LOAD_GLOBAL: &str = opmap ["LOAD_GLOBAL" ];
pub const BINARY_OP: &str = opmap ["BINARY_OP" ];
pub const JUMP_BACKWARD: &str = opmap ["JUMP_BACKWARD" ];
pub const CACHE: &str = opmap ["CACHE" ];
pub const _all_opname: f64 = list ( opname );
pub const _all_opmap: /* inferred */ = dict ( opmap );
pub const _empty_slot: &str = [ slot for slot , name in enumerate ( _all_opname ) if name . startswith ("<" ) ];
pub const deoptmap: f64 = {;
pub fn _try_compile(source: &str, name: &str) {
        "Attempts to compile the given source, first as an expression and
       then as a statement if the first approach fails.

       Utility function to accept strings in functions that otherwise
       expect code objects
    ";
        // try {
        c = compile ( source , name , "eval" );
        // } catch  SyntaxError  {
        c = compile ( source , name , "exec" );
        return  c;
        pub fn dis ( x = None /* Option */ , * , file = None /* Option */ , depth = None /* Option */ , show_caches = false , adaptive = false )  {
        "Disassemble classes, methods, functions, && other compiled objects.

    With no argument, disassemble the last traceback.

    Compiled objects currently include generator objects, async generator
    objects, && coroutine objects, all of which store their code object
    in a special attribute.
    ";
        if x is None /* Option */ {
        distb ( file = file , show_caches = show_caches , adaptive = adaptive );
        return;
        if hasattr ( x , "__func__" ) {
        x = x . __func__;
        if hasattr ( x , "__code__" ) {
        x = x . __code__;
        } else if hasattr ( x , "gi_code" ) {
        x = x . gi_code;
        } else if hasattr ( x , "ag_code" ) {
        x = x . ag_code;
        } else if hasattr ( x , "cr_code" ) {
        x = x . cr_code;
        if hasattr ( x , "__dict__" ) {
        items = sorted ( x . __dict__ . items ( ) );
        for name , x1 in items .iter() {
        if isinstance ( x1 , _have_code ) {
        println!( "Disassembly of %s:" % name , file = file );
        // try {
        dis ( x1 , file = file , depth = depth , show_caches = show_caches , adaptive = adaptive );
        // } catch  TypeError as msg  {
        println!( "Sorry:" , msg , file = file );
        println!( file = file );
        } else if hasattr ( x , "co_code" ) {
        _disassemble_recursive ( x , file = file , depth = depth , show_caches = show_caches , adaptive = adaptive );
        } else if isinstance ( x , ( bytes , bytearray ) ) {
        _disassemble_bytes ( x , file = file , show_caches = show_caches );
        } else if isinstance ( x , str ) {
        _disassemble_str ( x , file = file , depth = depth , show_caches = show_caches , adaptive = adaptive );
        } else {
        panic!("TypeError ( "don't know how to disassemble %s objects" %");
        type ( x ) . __name__ );
        pub fn distb ( tb = None /* Option */ , * , file = None /* Option */ , show_caches = false , adaptive = false )  {
        "Disassemble a traceback (default: last traceback).";
        if tb is None /* Option */ {
        // try {
        tb = sys . last_traceback;
        // } catch  AttributeError  {
        panic!("RuntimeError ( "no last traceback to disassemble" ) from None /* Option */");
        while tb . tb_next : tb = tb . tb_next {
        disassemble ( tb . tb_frame . f_code , tb . tb_lasti , file = file , show_caches = show_caches , adaptive = adaptive );
        COMPILER_FLAG_NAMES = {;
        1 : "OPTIMIZED" ,;
        2 : "NEWLOCALS" ,;
        4 : "VARARGS" ,;
        8 : "VARKEYWORDS" ,;
        16 : "NESTED" ,;
        32 : "GENERATOR" ,;
        64 : "NOFREE" ,;
        128 : "COROUTINE" ,;
        256 : "ITERABLE_COROUTINE" ,;
        512 : "ASYNC_GENERATOR" ,;
        };
        pub fn pretty_flags ( flags )  {
        "Return pretty representation of code flags.";
        names = [ ];
        for i in range ( 32 ) .iter() {
        flag = 1 < < i;
        if flags & flag {
        names . append ( COMPILER_FLAG_NAMES . get ( flag , hex ( flag ) ) );
        flags ^ = flag;
        if !flags {
        break;
        } else {
        names . append ( hex ( flags ) );
        return  ", " . join ( names );
        class _Unknown ;
        pub fn __repr__ ( self )  {
        return  "<unknown>";
        UNKNOWN = _Unknown ( );
        pub fn _get_code_object ( x )  {
        "Helper to handle methods, compiled || raw code objects, && strings.";
        if hasattr ( x , "__func__" ) {
        x = x . __func__;
        if hasattr ( x , "__code__" ) {
        x = x . __code__;
        } else if hasattr ( x , "gi_code" ) {
        x = x . gi_code;
        } else if hasattr ( x , "ag_code" ) {
        x = x . ag_code;
        } else if hasattr ( x , "cr_code" ) {
        x = x . cr_code;
        if isinstance ( x , str ) {
        x = _try_compile ( x , "<disassembly>" );
        if hasattr ( x , "co_code" ) {
        return  x;
        panic!("TypeError ( "don't know how to disassemble %s objects" %");
        type ( x ) . __name__ );
        pub fn _deoptop ( op )  {
        name = _all_opname [ op ];
        return  _all_opmap [ deoptmap [ name ] ] if name in deoptmap else op;
        pub fn _get_code_array ( co , adaptive )  {
        return  co . _co_code_adaptive if adaptive else co . co_code;
        pub fn code_info ( x )  {
        "Formatted details of methods, functions, || code.";
        return  _format_code_info ( _get_code_object ( x ) );
        pub fn _format_code_info ( co )  {
        lines = [ ];
        lines . append ( "Name:              %s" % co . co_name );
        lines . append ( "Filename:          %s" % co . co_filename );
        lines . append ( "Argument count:    %s" % co . co_argcount );
        lines . append ( "Positional-only arguments: %s" % co . co_posonlyargcount );
        lines . append ( "Kw-only arguments: %s" % co . co_kwonlyargcount );
        lines . append ( "Number of locals:  %s" % co . co_nlocals );
        lines . append ( "Stack size:        %s" % co . co_stacksize );
        lines . append ( "Flags:             %s" % pretty_flags ( co . co_flags ) );
        if co . co_consts {
        lines . append ( "Constants:" );
        for i_c in enumerate ( co . co_consts ) .iter() {
        lines . append ( "%4d: %r" % i_c );
        if co . co_names {
        lines . append ( "Names:" );
        for i_n in enumerate ( co . co_names ) .iter() {
        lines . append ( "%4d: %s" % i_n );
        if co . co_varnames {
        lines . append ( "Variable names:" );
        for i_n in enumerate ( co . co_varnames ) .iter() {
        lines . append ( "%4d: %s" % i_n );
        if co . co_freevars {
        lines . append ( "Free variables:" );
        for i_n in enumerate ( co . co_freevars ) .iter() {
        lines . append ( "%4d: %s" % i_n );
        if co . co_cellvars {
        lines . append ( "Cell variables:" );
        for i_n in enumerate ( co . co_cellvars ) .iter() {
        lines . append ( "%4d: %s" % i_n );
        return  "\n" . join ( lines );
        pub fn show_code ( co , * , file = None /* Option */ )  {
        "Print details of methods, functions, || code to *file*.

    If *file* == !provided, the output == printed on stdout.
    ";
        println!( code_info ( co ) , file = file );
        Positions = collections . namedtuple (;
        "Positions" ,;
        [;
        "lineno" ,;
        "end_lineno" ,;
        "col_offset" ,;
        "end_col_offset" ,;
        ] ,;
        defaults = [ None /* Option */ ] * 4;
        );
        _Instruction = collections . namedtuple (;
        "_Instruction" ,;
        [;
        "opname" ,;
        "opcode" ,;
        "arg" ,;
        "argval" ,;
        "argrepr" ,;
        "offset" ,;
        "starts_line" ,;
        "is_jump_target" ,;
        "positions";
        ] ,;
        defaults = [ None /* Option */ ];
        );
        _Instruction . opname . __doc__ = "Human readable name for operation";
        _Instruction . opcode . __doc__ = "Numeric code for operation";
        _Instruction . arg . __doc__ = "Numeric argument to operation (if any), otherwise None /* Option */";
        _Instruction . argval . __doc__ = "Resolved arg value (if known), otherwise same as arg";
        _Instruction . argrepr . __doc__ = "Human readable description of operation argument";
        _Instruction . offset . __doc__ = "Start index of operation within bytecode sequence";
        _Instruction . starts_line . __doc__ = "Line started by this opcode (if any), otherwise None /* Option */";
        _Instruction . is_jump_target . __doc__ = "true if other code jumps to here, otherwise false";
        _Instruction . positions . __doc__ = "dis.Positions object holding the span of source code covered by this instruction";
        _ExceptionTableEntry = collections . namedtuple ( "_ExceptionTableEntry" ,;
        "start end target depth lasti" );
        _OPNAME_WIDTH = 20;
        _OPARG_WIDTH = 5;
        class Instruction ( _Instruction ) ;
        "Details for a bytecode operation

       Defined fields:
         opname - human readable name for operation
         opcode - numeric code for operation
         arg - numeric argument to operation (if any), otherwise None /* Option */
         argval - resolved arg value (if known), otherwise same as arg
         argrepr - human readable description of operation argument
         offset - start index of operation within bytecode sequence
         starts_line - line started by this opcode (if any), otherwise None /* Option */
         is_jump_target - true if other code jumps to here, otherwise false
         positions - Optional dis.Positions object holding the span of source code
                     covered by this instruction
    ";
        pub fn _disassemble ( &self, lineno_width = 3 , mark_as_current = false , offset_width = 4 )  {
        "Format instruction details for inclusion in disassembly output

        *lineno_width* sets the width of the line number field (0 omits it)
        *mark_as_current* inserts a '-->' marker arrow as part of the line
        *offset_width* sets the width of the instruction offset field
        ";
        fields = [ ];
        if lineno_width {
        if self . starts_line is !None /* Option */ {
        lineno_fmt = "%%%dd" % lineno_width;
        fields . append ( lineno_fmt % self . starts_line );
        } else {
        fields . append ( " " * lineno_width );
        if mark_as_current {
        fields . append ( "-->" );
        } else {
        fields . append ( "   " );
        if self . is_jump_target {
        fields . append ( ">>" );
        } else {
        fields . append ( "  " );
        fields . append ( repr ( self . offset ) . rjust ( offset_width ) );
        fields . append ( self . opname . ljust ( _OPNAME_WIDTH ) );
        if self . arg is !None /* Option */ {
        fields . append ( repr ( self . arg ) . rjust ( _OPARG_WIDTH ) );
        if self . argrepr {
        fields . append ( "(" + self . argrepr + ")" );
        return  " " . join ( fields ) . rstrip ( );
        pub fn get_instructions ( x , * , first_line = None /* Option */ , show_caches = false , adaptive = false )  {
        "Iterator for the opcodes in methods, functions || code

    Generates a series of Instruction named tuples giving the details of
    each operations in the supplied code.

    If *first_line* == !None /* Option */, it indicates the line number that should
    be reported for the first source line in the disassembled code.
    Otherwise, the source line information (if any) == taken directly from
    the disassembled code object.
    ";
        co = _get_code_object ( x );
        linestarts = dict ( findlinestarts ( co ) );
        if first_line is !None /* Option */ {
        line_offset = first_line - co . co_firstlineno;
        } else {
        line_offset = 0;
        return  _get_instructions_bytes ( _get_code_array ( co , adaptive ) ,;
        co . _varname_from_oparg ,;
        co . co_names , co . co_consts ,;
        linestarts , line_offset ,;
        co_positions = co . co_positions ( ) ,;
        show_caches = show_caches );
        pub fn _get_const_value ( op , arg , co_consts )  {
        "Helper to get the value of the const in a hasconst op.

       Returns the dereferenced constant if this == possible.
       Otherwise (if it == a LOAD_CONST && co_consts == not
       provided) returns the dis.UNKNOWN sentinel.
    ";
        assert op in hasconst;
        argval = UNKNOWN;
        if op == LOAD_CONST {
        if co_consts is !None /* Option */ {
        argval = co_consts [ arg ];
        return  argval;
        pub fn _get_const_info ( op , arg , co_consts )  {
        "Helper to get optional details about const references

       Returns the dereferenced constant && its repr if the value
       can be calculated.
       Otherwise returns the sentinel value dis.UNKNOWN for the value
       && an empty string for its repr.
    ";
        argval = _get_const_value ( op , arg , co_consts );
        argrepr = repr ( argval ) if argval == !UNKNOWN else "";
        return  argval , argrepr;
        pub fn _get_name_info ( name_index , get_name , ** extrainfo )  {
        "Helper to get optional details about named references

       Returns the dereferenced name as both value && repr if the name
       list == defined.
       Otherwise returns the sentinel value dis.UNKNOWN for the value
       && an empty string for its repr.
    ";
        if get_name is !None /* Option */ {
        argval = get_name ( name_index , ** extrainfo );
        return  argval , argval;
        } else {
        return  UNKNOWN , "";
        pub fn _parse_varint ( iterator )  {
        b = next ( iterator );
        val = b & 63;
        while b & 64  {
        val < <= 6;
        b = next ( iterator );
        val | = b & 63;
        return  val;
        pub fn _parse_exception_table ( code )  {
        iterator = iter ( code . co_exceptiontable );
        entries = [ ];
        // try {
        while true  {
        start = _parse_varint ( iterator ) * 2;
        length = _parse_varint ( iterator ) * 2;
        end = start + length;
        target = _parse_varint ( iterator ) * 2;
        dl = _parse_varint ( iterator );
        depth = dl > > 1;
        lasti = bool ( dl & 1 );
        entries . append ( _ExceptionTableEntry ( start , end , target , depth , lasti ) );
        // } catch  StopIteration  {
        return  entries;
        pub fn _is_backward_jump ( op )  {
        return  "JUMP_BACKWARD" in opname [ op ];
        pub fn _get_instructions_bytes ( code , varname_from_oparg = None /* Option */ , {
        names = None /* Option */ , co_consts = None /* Option */ ,;
        linestarts = None /* Option */ , line_offset = 0 ,;
        // } catch ion_entries = ( ) , co_positions = None /* Option */ , {
        show_caches = false ) ;
        "Iterate over the instructions in a bytecode string.

    Generates a sequence of Instruction namedtuples giving the details of each
    opcode.  Additional information about the code's runtime environment
    (e.g. variable names, co_consts) can be specified using optional
    arguments.

    ";
        co_positions = co_positions || iter ( ( ) );
        get_name = None /* Option */ if names == None /* Option */ else names . __getitem__;
        labels = set ( findlabels ( code ) );
        for start , end , target , _ , _ in exception_entries .iter() {
        for i in range ( start , end ) .iter() {
        labels . add ( target );
        starts_line = None /* Option */;
        for offset , op , arg in _unpack_opargs ( code ) .iter() {
        if linestarts is !None /* Option */ {
        starts_line = linestarts . get ( offset , None /* Option */ );
        if starts_line is !None /* Option */ {
        starts_line + = line_offset;
        is_jump_target = offset in labels;
        argval = None /* Option */;
        argrepr = "";
        positions = Positions ( * next ( co_positions , ( ) ) );
        deop = _deoptop ( op );
        if arg is !None /* Option */ {
        argval = arg;
        if deop in hasconst {
        argval , argrepr = _get_const_info ( deop , arg , co_consts );
        } else if deop in hasname {
        if deop == LOAD_GLOBAL {
        argval , argrepr = _get_name_info ( arg / / 2 , get_name );
        if ( arg & 1 ) && argrepr {
        argrepr = "NULL + " + argrepr;
        } else {
        argval , argrepr = _get_name_info ( arg , get_name );
        } else if deop in hasjabs {
        argval = arg * 2;
        argrepr = "to " + repr ( argval );
        } else if deop in hasjrel {
        signed_arg = - arg if _is_backward_jump ( deop ) else arg;
        argval = offset + 2 + signed_arg * 2;
        argrepr = "to " + repr ( argval );
        } else if deop in haslocal || deop in hasfree {
        argval , argrepr = _get_name_info ( arg , varname_from_oparg );
        } else if deop in hascompare {
        argval = cmp_op [ arg ];
        argrepr = argval;
        } else if deop == FORMAT_VALUE {
        argval , argrepr = FORMAT_VALUE_CONVERTERS [ arg & 0x3 ];
        argval = ( argval , bool ( arg & 0x4 ) );
        if argval [ 1 ] {
        if argrepr {
        argrepr + = ", ";
        argrepr + = "with format";
        } else if deop == MAKE_FUNCTION {
        argrepr = ", " . join ( s for i , s in enumerate ( MAKE_FUNCTION_FLAGS );
        if arg & ( 1 < < i ) ) {
        } else if deop == BINARY_OP {
        _ , argrepr = _nb_ops [ arg ];
        yield Instruction ( _all_opname [ op ] , op ,;
        arg , argval , argrepr ,;
        offset , starts_line , is_jump_target , positions );
        caches = _inline_cache_entries [ deop ];
        if !caches {
        continue;
        if !show_caches {
        for _ in range ( caches ) .iter() {
        next ( co_positions , ( ) );
        continue;
        for name , size in _cache_format [ opname [ deop ] ] . items ( ) .iter() {
        for i in range ( size ) .iter() {
        offset + = 2;
        if i == 0 && op != deop {
        data = code [ offset : offset + 2 * size ];
        argrepr = format!("{name}: {int.from_bytes(data, sys.byteorder)}");
        } else {
        argrepr = "";
        yield Instruction (;
        "CACHE" , CACHE , 0 , None /* Option */ , argrepr , offset , None /* Option */ , false ,;
        Positions ( * next ( co_positions , ( ) ) );
        );
        pub fn disassemble ( co , lasti = -1 , * , file = None /* Option */ , show_caches = false , adaptive = false )  {
        "Disassemble a code object.";
        linestarts = dict ( findlinestarts ( co ) );
        // } catch ion_entries = _parse_exception_table ( co ) {
        _disassemble_bytes ( _get_code_array ( co , adaptive ) ,;
        lasti , co . _varname_from_oparg ,;
        co . co_names , co . co_consts , linestarts , file = file ,;
        // } catch ion_entries = exception_entries , {
        co_positions = co . co_positions ( ) , show_caches = show_caches );
        pub fn _disassemble_recursive ( co , * , file = None /* Option */ , depth = None /* Option */ , show_caches = false , adaptive = false )  {
        disassemble ( co , file = file , show_caches = show_caches , adaptive = adaptive );
        if depth is None /* Option */ || depth > 0 {
        if depth is !None /* Option */ {
        depth = depth - 1;
        for x in co . co_consts .iter() {
        if hasattr ( x , "co_code" ) {
        println!( file = file );
        println!( "Disassembly of %r:" % ( x , ) , file = file );
        _disassemble_recursive (;
        x , file = file , depth = depth , show_caches = show_caches , adaptive = adaptive;
        );
        pub fn _disassemble_bytes ( code , lasti = -1 , varname_from_oparg = None /* Option */ , {
        names = None /* Option */ , co_consts = None /* Option */ , linestarts = None /* Option */ ,;
        * , file = None /* Option */ , line_offset = 0 , exception_entries = ( ) ,;
        co_positions = None /* Option */ , show_caches = false ) ;
        show_lineno = bool ( linestarts );
        if show_lineno {
        maxlineno = max ( linestarts . values ( ) ) + line_offset;
        if maxlineno >= 1000 {
        lineno_width = len ( str ( maxlineno ) );
        } else {
        lineno_width = 3;
        } else {
        lineno_width = 0;
        maxoffset = len ( code ) - 2;
        if maxoffset >= 10000 {
        offset_width = len ( str ( maxoffset ) );
        } else {
        offset_width = 4;
        for instr in _get_instructions_bytes ( code , varname_from_oparg , names ,.iter() {
        co_consts , linestarts ,;
        line_offset = line_offset ,;
        // } catch ion_entries = exception_entries , {
        co_positions = co_positions ,;
        show_caches = show_caches ) ;
        new_source_line = ( show_lineno and;
        instr . starts_line == !None /* Option */ and;
        instr . offset > 0 );
        if new_source_line {
        println!( file = file );
        is_current_instr = instr . offset == lasti;
        println!( instr . _disassemble ( lineno_width , is_current_instr , offset_width ) );
        file = file );
        if exception_entries {
        println!( "ExceptionTable:" , file = file );
        for entry in exception_entries .iter() {
        lasti = " lasti" if entry . lasti else "";
        end = entry . end -2;
        println!( f "  {entry.start} to {end} -> {entry.target} [{entry.depth}]{lasti}" , file = file );
        pub fn _disassemble_str ( source , ** kwargs )  {
        "Compile the source string, then disassemble the code object.";
        _disassemble_recursive ( _try_compile ( source , "<dis>" ) , ** kwargs );
        disco = disassemble;
        _INT_BITS = 32;
        _INT_OVERFLOW = 2 ** ( _INT_BITS - 1 );
        pub fn _unpack_opargs ( code )  {
        extended_arg = 0;
        caches = 0;
        for i in range ( 0 , len ( code ) , 2 ) .iter() {
        if caches {
        caches - = 1;
        continue;
        op = code [ i ];
        deop = _deoptop ( op );
        caches = _inline_cache_entries [ deop ];
        if deop >= HAVE_ARGUMENT {
        arg = code [ i + 1 ] | extended_arg;
        extended_arg = ( arg < < 8 ) if deop == EXTENDED_ARG else 0;
        if extended_arg >= _INT_OVERFLOW {
        extended_arg - = 2 * _INT_OVERFLOW;
        } else {
        arg = None /* Option */;
        extended_arg = 0;
        yield ( i , op , arg );
        pub fn findlabels ( code )  {
        "Detect all offsets in a byte code which are jump targets.

    Return the list of offsets.

    ";
        labels = [ ];
        for offset , op , arg in _unpack_opargs ( code ) .iter() {
        if arg is !None /* Option */ {
        if op in hasjrel {
        if _is_backward_jump ( op ) {
        arg = - arg;
        label = offset + 2 + arg * 2;
        } else if op in hasjabs {
        label = arg * 2;
        } else {
        continue;
        if label !in labels {
        labels . append ( label );
        return  labels;
        pub fn findlinestarts ( code )  {
        "Find the offsets in a byte code which are start of lines in the source.

    Generate pairs (offset, lineno)
    ";
        lastline = None /* Option */;
        for start , end , line in code . co_lines ( ) .iter() {
        if line is !None /* Option */ && line != lastline {
        lastline = line;
        yield start , line;
        return;
        pub fn _find_imports ( co )  {
        "Find import statements in the code

    Generate triplets (name, level, fromlist) where
    name == the imported module && level, fromlist are
    the corresponding args to __import__.
    ";
        IMPORT_NAME = opmap [ "IMPORT_NAME" ];
        LOAD_CONST = opmap [ "LOAD_CONST" ];
        consts = co . co_consts;
        names = co . co_names;
        opargs = vec![ ( op , arg ).iter().map(|_ , op , arg| _unpack_opargs ( co . co_code );
        if op != EXTENDED_ARG ] {
        for i , ( op , oparg ) in enumerate ( opargs ) .iter() {
        if op == IMPORT_NAME && i >= 2 {
        from_op = opargs [ i -1 ];
        level_op = opargs [ i -2 ];
        if ( from_op [ 0 ] in hasconst && level_op [ 0 ] in hasconst ) {
        level = _get_const_value ( level_op [ 0 ] , level_op [ 1 ] , consts );
        fromlist = _get_const_value ( from_op [ 0 ] , from_op [ 1 ] , consts );
        yield ( names [ oparg ] , level , fromlist );
        pub fn _find_store_names ( co )  {
        "Find names of variables which are written in the code

    Generate sequence of strings
    ";
        STORE_OPS = {;
        opmap [ "STORE_NAME" ] ,;
        opmap [ "STORE_GLOBAL" ];
        };
        names = co . co_names;
        for _ , op , arg in _unpack_opargs ( co . co_code ) .iter() {
        if op in STORE_OPS {
        yield names [ arg ];
        class Bytecode ;
        "The bytecode operations of a piece of code

    Instantiate this with a function, method, other compiled object, string of
    code, || a code object (as returned by compile()).

    Iterating over this yields the bytecode operations as Instruction instances.
    ";
        pub fn __init__ ( &self, x , * , first_line = None /* Option */ , current_offset = None /* Option */ , show_caches = false , adaptive = false )  {
        self . codeobj = co = _get_code_object ( x );
        if first_line is None /* Option */ {
        self . first_line = co . co_firstlineno;
        self . _line_offset = 0;
        } else {
        self . first_line = first_line;
        self . _line_offset = first_line - co . co_firstlineno;
        self . _linestarts = dict ( findlinestarts ( co ) );
        self . _original_object = x;
        self . current_offset = current_offset;
        self . exception_entries = _parse_exception_table ( co );
        self . show_caches = show_caches;
        self . adaptive = adaptive;
        pub fn __iter__ ( self )  {
        co = self . codeobj;
        return  _get_instructions_bytes ( _get_code_array ( co , self . adaptive ) ,;
        co . _varname_from_oparg ,;
        co . co_names , co . co_consts ,;
        self . _linestarts ,;
        line_offset = self . _line_offset ,;
        // } catch ion_entries = self . exception_entries , {
        co_positions = co . co_positions ( ) ,;
        show_caches = self . show_caches );
        pub fn __repr__ ( self )  {
        return  "{}({!r})" . format ( self . __class__ . __name__ ,;
        self . _original_object );
        @ classmethod;
        pub fn from_traceback ( cls , tb , * , show_caches = false , adaptive = false )  {
        " Construct a Bytecode from the given traceback ";
        while tb . tb_next  {
        tb = tb . tb_next;
        return  cls (;
        tb . tb_frame . f_code , current_offset = tb . tb_lasti , show_caches = show_caches , adaptive = adaptive;
        );
        pub fn info ( self )  {
        "Return formatted information about the code object.";
        return  _format_code_info ( self . codeobj );
        pub fn dis ( self )  {
        "Return a formatted view of the bytecode operations.";
        co = self . codeobj;
        if self . current_offset is !None /* Option */ {
        offset = self . current_offset;
        } else {
        offset = -1;
        // with scope: io . StringIO ( ) as output  {
        _disassemble_bytes ( _get_code_array ( co , self . adaptive ) ,;
        varname_from_oparg = co . _varname_from_oparg ,;
        names = co . co_names , co_consts = co . co_consts ,;
        linestarts = self . _linestarts ,;
        line_offset = self . _line_offset ,;
        file = output ,;
        lasti = offset ,;
        // } catch ion_entries = self . exception_entries , {
        co_positions = co . co_positions ( ) ,;
        show_caches = self . show_caches );
        return  output . getvalue ( );
        pub fn main ( )  {
        import argparse;
        parser = argparse . ArgumentParser ( );
        parser . add_argument ( "infile" , type = argparse . FileType ( "rb" ) , nargs = "?" , default = "-" );
        args = parser . parse_args ( );
        // with scope: args . infile as infile  {
        source = infile . read ( );
        code = compile ( source , args . infile . name , "exec" );
        dis ( code );
        fn main() {
        main ( );
}

