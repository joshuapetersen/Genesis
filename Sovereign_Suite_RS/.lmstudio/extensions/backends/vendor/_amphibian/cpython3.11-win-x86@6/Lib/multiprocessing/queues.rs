//! queues.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use std::thread;
// use std::time;
// use crate::weakref;
// use crate::queue::{Empty, Full};
// use crate::_multiprocessing;
// use crate::.::{connection};
// use crate::traceback;

pub const __all__: &str = ["Queue" ,"SimpleQueue" ,"JoinableQueue" ];
pub const _ForkingPickler: f64 = context . reduction . ForkingPickler;
pub struct Queue {
    pub _maxsize: String, // TODO: infer type
    pub _writer: String, // TODO: infer type
    pub _rlock: String, // TODO: infer type
    pub _opid: String, // TODO: infer type
    pub _wlock: String, // TODO: infer type
    pub _sem: String, // TODO: infer type
    pub _ignore_epipe: String, // TODO: infer type
    pub _notempty: String, // TODO: infer type
    pub _buffer: String, // TODO: infer type
    pub _thread: String, // TODO: infer type
    pub _jointhread: String, // TODO: infer type
    pub _joincancelled: String, // TODO: infer type
    pub _closed: String, // TODO: infer type
    pub _close: String, // TODO: infer type
    pub _send_bytes: String, // TODO: infer type
    pub _recv_bytes: String, // TODO: infer type
    pub _poll: String, // TODO: infer type
    pub _unfinished_tasks: String, // TODO: infer type
    pub _cond: String, // TODO: infer type
}

impl Queue {
    pub fn new(maxsize: &str, ctx: &str) -> Self {
        if maxsize <= 0 {
        from . synchronize import SEM_VALUE_MAX as maxsize;
        self . _maxsize = maxsize;
        self . _reader , self . _writer = connection . Pipe ( duplex = false );
        self . _rlock = ctx . Lock ( );
        self . _opid = os . getpid ( );
        if sys . platform == "win32" {
        self . _wlock = None /* Option */;
        } else {
        self . _wlock = ctx . Lock ( );
        self . _sem = ctx . BoundedSemaphore ( maxsize );
        self . _ignore_epipe = false;
        self . _reset ( );
    }

}

