//! windows_events.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::_overlapped;
// use crate::errno;
// use crate::msvcrt;
// use crate::struct;
// use crate::weakref;
// use crate::.::{events};

pub const __all__: f64 = (;
pub const NULL: f64 = _winapi . NULL;
pub const INFINITE: f64 = _winapi . INFINITE;
pub const ERROR_CONNECTION_REFUSED: u64 = 1225;
pub const ERROR_CONNECTION_ABORTED: u64 = 1236;
pub const CONNECT_PIPE_INIT_DELAY: f64 = 0.001;
pub const CONNECT_PIPE_MAX_DELAY: f64 = 0.100;
pub struct _OverlappedFuture {
    pub _ov: String, // TODO: infer type
    pub _handle: String, // TODO: infer type
    pub _wait_handle: String, // TODO: infer type
    pub _registered: String, // TODO: infer type
    pub _done_callback: String, // TODO: infer type
    pub _proactor: String, // TODO: infer type
    pub _unregister_proactor: String, // TODO: infer type
    pub _event: String, // TODO: infer type
    pub _event_fut: String, // TODO: infer type
    pub _address: String, // TODO: infer type
    pub _free_instances: String, // TODO: infer type
    pub _pipe: String, // TODO: infer type
    pub _accept_pipe_future: String, // TODO: infer type
    pub _self_reading_future: String, // TODO: infer type
    pub _loop: String, // TODO: infer type
    pub _results: String, // TODO: infer type
    pub _iocp: String, // TODO: infer type
    pub _cache: String, // TODO: infer type
    pub _unregistered: String, // TODO: infer type
    pub _stopped_serving: String, // TODO: infer type
    pub _proc: String, // TODO: infer type
}

impl _OverlappedFuture {
}

pub struct _BaseWaitHandleFuture {
    pub _ov: String, // TODO: infer type
    pub _handle: String, // TODO: infer type
    pub _wait_handle: String, // TODO: infer type
    pub _registered: String, // TODO: infer type
    pub _done_callback: String, // TODO: infer type
    pub _proactor: String, // TODO: infer type
    pub _unregister_proactor: String, // TODO: infer type
    pub _event: String, // TODO: infer type
    pub _event_fut: String, // TODO: infer type
    pub _address: String, // TODO: infer type
    pub _free_instances: String, // TODO: infer type
    pub _pipe: String, // TODO: infer type
    pub _accept_pipe_future: String, // TODO: infer type
    pub _self_reading_future: String, // TODO: infer type
    pub _loop: String, // TODO: infer type
    pub _results: String, // TODO: infer type
    pub _iocp: String, // TODO: infer type
    pub _cache: String, // TODO: infer type
    pub _unregistered: String, // TODO: infer type
    pub _stopped_serving: String, // TODO: infer type
    pub _proc: String, // TODO: infer type
}

impl _BaseWaitHandleFuture {
}

pub struct _WaitCancelFuture {
    pub _done_callback: String, // TODO: infer type
    pub _proactor: String, // TODO: infer type
    pub _unregister_proactor: String, // TODO: infer type
    pub _event: String, // TODO: infer type
    pub _event_fut: String, // TODO: infer type
    pub _registered: String, // TODO: infer type
    pub _wait_handle: String, // TODO: infer type
    pub _address: String, // TODO: infer type
    pub _free_instances: String, // TODO: infer type
    pub _pipe: String, // TODO: infer type
    pub _accept_pipe_future: String, // TODO: infer type
    pub _self_reading_future: String, // TODO: infer type
    pub _loop: String, // TODO: infer type
    pub _results: String, // TODO: infer type
    pub _iocp: String, // TODO: infer type
    pub _cache: String, // TODO: infer type
    pub _unregistered: String, // TODO: infer type
    pub _stopped_serving: String, // TODO: infer type
    pub _proc: String, // TODO: infer type
}

impl _WaitCancelFuture {
}

pub struct _WaitHandleFuture {
    pub _proactor: String, // TODO: infer type
    pub _unregister_proactor: String, // TODO: infer type
    pub _event: String, // TODO: infer type
    pub _event_fut: String, // TODO: infer type
    pub _registered: String, // TODO: infer type
    pub _wait_handle: String, // TODO: infer type
    pub _address: String, // TODO: infer type
    pub _free_instances: String, // TODO: infer type
    pub _pipe: String, // TODO: infer type
    pub _accept_pipe_future: String, // TODO: infer type
    pub _self_reading_future: String, // TODO: infer type
    pub _loop: String, // TODO: infer type
    pub _results: String, // TODO: infer type
    pub _iocp: String, // TODO: infer type
    pub _cache: String, // TODO: infer type
    pub _unregistered: String, // TODO: infer type
    pub _stopped_serving: String, // TODO: infer type
    pub _proc: String, // TODO: infer type
}

impl _WaitHandleFuture {
    pub fn new(ov: &str, handle: &str, wait_handle: &str, proactor: &str, loop: &str) -> Self {
        super ( ) . __init__ ( ov , handle , wait_handle , loop = loop );
        self . _proactor = proactor;
        self . _unregister_proactor = true;
        self . _event = _overlapped . CreateEvent ( None /* Option */ , true , false , None /* Option */ );
        self . _event_fut = None /* Option */;
    }

}

