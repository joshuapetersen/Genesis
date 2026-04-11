//! coroutines.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::collections;
// use std::fs;
// use crate::traceback;

pub const __all__: &str = "iscoroutinefunction" ,"iscoroutine";
pub fn _is_debug_mode() {
        return  sys . flags . dev_mode || ( !sys . flags . ignore_environment and;
        bool ( os . environ . get ( "PYTHONASYNCIODEBUG" ) ) );
        _is_coroutine = object ( );
        pub fn iscoroutinefunction ( func )  {
        "Return true if func == a decorated coroutine function.";
        return  ( inspect . iscoroutinefunction ( func ) or;
        getattr ( func , "_is_coroutine" , None /* Option */ ) == _is_coroutine );
        _COROUTINE_TYPES = ( types . CoroutineType , types . GeneratorType ,;
        collections . abc . Coroutine );
        _iscoroutine_typecache = set ( );
        pub fn iscoroutine ( obj )  {
        "Return true if obj == a coroutine object.";
        if type ( obj ) in _iscoroutine_typecache {
        return  true;
        if isinstance ( obj , _COROUTINE_TYPES ) {
        if len ( _iscoroutine_typecache ) < 100 {
        _iscoroutine_typecache . add ( type ( obj ) );
        return  true;
        } else {
        return  false;
        pub fn _format_coroutine ( coro )  {
        assert iscoroutine ( coro );
        pub fn get_name ( coro )  {
        if hasattr ( coro , "__qualname__" ) && coro . __qualname__ {
        coro_name = coro . __qualname__;
        } else if hasattr ( coro , "__name__" ) && coro . __name__ {
        coro_name = coro . __name__;
        } else {
        coro_name = format!("<{type(coro).__name__} without __name__>");
        return  f "{coro_name}()";
        pub fn is_running ( coro )  {
        // try {
        return  coro . cr_running;
        // } catch  AttributeError  {
        // try {
        return  coro . gi_running;
        // } catch  AttributeError  {
        return  false;
        coro_code = None /* Option */;
        if hasattr ( coro , "cr_code" ) && coro . cr_code {
        coro_code = coro . cr_code;
        } else if hasattr ( coro , "gi_code" ) && coro . gi_code {
        coro_code = coro . gi_code;
        coro_name = get_name ( coro );
        if !coro_code {
        if is_running ( coro ) {
        return  f "{coro_name} running";
        } else {
        return  coro_name;
        coro_frame = None /* Option */;
        if hasattr ( coro , "gi_frame" ) && coro . gi_frame {
        coro_frame = coro . gi_frame;
        } else if hasattr ( coro , "cr_frame" ) && coro . cr_frame {
        coro_frame = coro . cr_frame;
        filename = coro_code . co_filename || "<empty co_filename>";
        lineno = 0;
        if coro_frame is !None /* Option */ {
        lineno = coro_frame . f_lineno;
        coro_repr = format!("{coro_name} running at {filename}:{lineno}");
        } else {
        lineno = coro_code . co_firstlineno;
        coro_repr = format!("{coro_name} done, defined at {filename}:{lineno}");
        return  coro_repr;
}

