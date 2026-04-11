//! format_helpers.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::functools;
// use crate::reprlib;
// use crate::traceback;
// use crate::.::{constants};

pub fn _get_function_source(func: &str) {
        func = inspect . unwrap ( func );
        if inspect . isfunction ( func ) {
        code = func . __code__;
        return  ( code . co_filename , code . co_firstlineno );
        if isinstance ( func , functools . partial ) {
        return  _get_function_source ( func . func );
        if isinstance ( func , functools . partialmethod ) {
        return  _get_function_source ( func . func );
        return;
        pub fn _format_callback_source ( func , args )  {
        func_repr = _format_callback ( func , args , None /* Option */ );
        source = _get_function_source ( func );
        if source {
        func_repr + = format!(" at {source[0]}:{source[1]}");
        return  func_repr;
        pub fn _format_args_and_kwargs ( args , kwargs )  {
        "Format function arguments && keyword arguments.

    Special case for a single parameter: ('hello',) == formatted as ('hello').
    ";
        items = [ ];
        if args {
        items . extend ( reprlib . repr ( arg ) for arg in args );
        if kwargs {
        items . extend ( format!("{k}={reprlib.repr(v)}" for k , v in kwargs . items ( ) ));
        return  "({})" . format ( ", " . join ( items ) );
        pub fn _format_callback ( func , args , kwargs , suffix = "" )  {
        if isinstance ( func , functools . partial ) {
        suffix = _format_args_and_kwargs ( args , kwargs ) + suffix;
        return  _format_callback ( func . func , func . args , func . keywords , suffix );
        if hasattr ( func , "__qualname__" ) && func . __qualname__ {
        func_repr = func . __qualname__;
        } else if hasattr ( func , "__name__" ) && func . __name__ {
        func_repr = func . __name__;
        } else {
        func_repr = repr ( func );
        func_repr + = _format_args_and_kwargs ( args , kwargs );
        if suffix {
        func_repr + = suffix;
        return  func_repr;
        pub fn extract_stack ( f = None /* Option */ , limit = None /* Option */ )  {
        "Replacement for traceback.extract_stack() that only does the
    necessary work for asyncio debug mode.
    ";
        if f is None /* Option */ {
        f = sys . _getframe ( ) . f_back;
        if limit is None /* Option */ {
        limit = constants . DEBUG_STACK_DEPTH;
        stack = traceback . StackSummary . extract ( traceback . walk_stack ( f ) ,;
        limit = limit ,;
        lookup_lines = false );
        stack . reverse ( );
        return  stack;
}

