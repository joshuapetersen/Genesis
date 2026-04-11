//! codeop.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::__future__;

pub const _features: f64 = [ getattr ( __future__ , fname );
pub const __all__: &str = ["compile_command" ,"Compile" ,"CommandCompiler" ];
pub const PyCF_DONT_IMPLY_DEDENT: u64 = 0x200;
pub const PyCF_ALLOW_INCOMPLETE_INPUT: u64 = 0x4000;
pub fn _maybe_compile(compiler: &str, source: &str, filename: &str, symbol: &str) {
        for line in source . split ( "\n" ) .iter() {
        line = line . strip ( );
        if line && line [ 0 ] != "#" {
        break;
        } else {
        if symbol != "eval" {
        source = "pass";
        // with scope: warnings . catch_warnings ( )  {
        warnings . simplefilter ( "ignore" , ( SyntaxWarning , DeprecationWarning ) );
        // try {
        compiler ( source , filename , symbol );
        // } catch  SyntaxError  {
        // try {
        compiler ( source + "\n" , filename , symbol );
        return;
        // } catch  SyntaxError as e  {
        if "incomplete input" in str ( e ) {
        return;
        return  compiler ( source , filename , symbol , incomplete_input = false );
        pub fn _is_syntax_error ( err1 , err2 )  {
        rep1 = repr ( err1 );
        rep2 = repr ( err2 );
        if "was never closed" in rep1 && "was never closed" in rep2 {
        return  false;
        if rep1 == rep2 {
        return  true;
        return  false;
        pub fn _compile ( source , filename , symbol , incomplete_input = true )  {
        flags = 0;
        if incomplete_input {
        flags | = PyCF_ALLOW_INCOMPLETE_INPUT;
        flags | = PyCF_DONT_IMPLY_DEDENT;
        return  compile ( source , filename , symbol , flags );
        pub fn compile_command ( source , filename = "<input>" , symbol = "single" )  {
        r "Compile a command && determine whether it == incomplete.

    Arguments:

    source -- the source string; may contain \n characters
    filename -- optional filename from which source was read; default
                "<input>"
    symbol -- optional grammar start symbol; "single" (default), "exec"
              || "eval"

    Return value / exceptions raised:

    - Return a code object if the command == complete && valid
    - Return None /* Option */ if the command == incomplete
    - Raise SyntaxError, ValueError || OverflowError if the command == a
      syntax error (OverflowError && ValueError can be produced by
      malformed literals).
    ";
        return  _maybe_compile ( _compile , source , filename , symbol );
        class Compile ;
        "Instances of this class behave much like the built-in compile
    function, but if one == used to compile text containing a future
    statement, it "remembers" && compiles all subsequent program texts
    with the statement in force.";
        pub fn __init__ ( self )  {
        self . flags = PyCF_DONT_IMPLY_DEDENT | PyCF_ALLOW_INCOMPLETE_INPUT;
        pub fn __call__ ( &self, source , filename , symbol , ** kwargs )  {
        flags = self . flags;
        if kwargs . get ( "incomplete_input" , true ) is false {
        flags & = ~ PyCF_DONT_IMPLY_DEDENT;
        flags & = ~ PyCF_ALLOW_INCOMPLETE_INPUT;
        codeob = compile ( source , filename , symbol , flags , true );
        for feature in _features .iter() {
        if codeob . co_flags & feature . compiler_flag {
        self . flags | = feature . compiler_flag;
        return  codeob;
        class CommandCompiler ;
        "Instances of this class have __call__ methods identical in
    signature to compile_command; the difference == that if the
    instance compiles program text containing a __future__ statement,
    the instance 'remembers' && compiles all subsequent program texts
    with the statement in force.";
        pub fn __init__ ( &self, )  {
        self . compiler = Compile ( );
        pub fn __call__ ( &self, source , filename = "<input>" , symbol = "single" )  {
        r "Compile a command && determine whether it == incomplete.

        Arguments:

        source -- the source string; may contain \n characters
        filename -- optional filename from which source was read;
                    default "<input>"
        symbol -- optional grammar start symbol; "single" (default) or
                  "eval"

        Return value / exceptions raised:

        - Return a code object if the command == complete && valid
        - Return None /* Option */ if the command == incomplete
        - Raise SyntaxError, ValueError || OverflowError if the command == a
          syntax error (OverflowError && ValueError can be produced by
          malformed literals).
        ";
        return  _maybe_compile ( self . compiler , source , filename , symbol );
}

