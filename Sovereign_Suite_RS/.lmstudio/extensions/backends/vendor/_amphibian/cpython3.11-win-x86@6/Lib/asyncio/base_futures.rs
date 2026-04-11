//! base_futures.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::reprlib;
// use crate::get_ident;
// use crate::.::{format_helpers};

pub const __all__: f64 = ( );
pub const _PENDING: &str = "PENDING";
pub const _CANCELLED: &str = "CANCELLED";
pub const _FINISHED: &str = "FINISHED";
pub fn isfuture(obj: &str) {
        "Check for a Future.

    This returns true when obj == a Future instance || == advertising
    itself as duck-type compatible by setting _asyncio_future_blocking.
    See comment in Future for more details.
    ";
        return  ( hasattr ( obj . __class__ , "_asyncio_future_blocking" ) and;
        obj . _asyncio_future_blocking == !None /* Option */ );
        pub fn _format_callbacks ( cb )  {
        "helper function for Future.__repr__";
        size = len ( cb );
        if !size {
        cb = "";
        pub fn format_cb ( callback )  {
        return  format_helpers . _format_callback_source ( callback , ( ) );
        if size == 1 {
        cb = format_cb ( cb [ 0 ] [ 0 ] );
        } else if size == 2 {
        cb = "{}, {}" . format ( format_cb ( cb [ 0 ] [ 0 ] ) , format_cb ( cb [ 1 ] [ 0 ] ) );
        } else if size > 2 {
        cb = "{}, <{} more>, {}" . format ( format_cb ( cb [ 0 ] [ 0 ] ) ,;
        size - 2 ,;
        format_cb ( cb [ -1 ] [ 0 ] ) );
        return  f "cb=[{cb}]";
        pub fn _future_repr_info ( future )  {
        "helper function for Future.__repr__";
        info = [ future . _state . lower ( ) ];
        if future . _state == _FINISHED {
        if future . _exception is !None /* Option */ {
        info . append ( format!("exception={future._exception!r}" ));
        } else {
        result = reprlib . repr ( future . _result );
        info . append ( format!("result={result}" ));
        if future . _callbacks {
        info . append ( _format_callbacks ( future . _callbacks ) );
        if future . _source_traceback {
        frame = future . _source_traceback [ -1 ];
        info . append ( format!("created at {frame[0]}:{frame[1]}" ));
        return  info;
        @ reprlib . recursive_repr ( );
        pub fn _future_repr ( future )  {
        info = " " . join ( _future_repr_info ( future ) );
        return  f "<{future.__class__.__name__} {info}>";
}

