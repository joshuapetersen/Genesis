//! taskgroups.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::.::{events};

pub const __all__: &str = ["TaskGroup" ];
pub struct TaskGroup {
    pub _entered: String, // TODO: infer type
    pub _exiting: String, // TODO: infer type
    pub _aborting: String, // TODO: infer type
    pub _loop: String, // TODO: infer type
    pub _parent_task: String, // TODO: infer type
    pub _parent_cancel_requested: String, // TODO: infer type
    pub _tasks: String, // TODO: infer type
    pub _errors: String, // TODO: infer type
    pub _base_error: String, // TODO: infer type
    pub _on_completed_fut: String, // TODO: infer type
}

impl TaskGroup {
    pub fn new() -> Self {
        self . _entered = false;
        self . _exiting = false;
        self . _aborting = false;
        self . _loop = None /* Option */;
        self . _parent_task = None /* Option */;
        self . _parent_cancel_requested = false;
        self . _tasks = set ( );
        self . _errors = [ ];
        self . _base_error = None /* Option */;
        self . _on_completed_fut = None /* Option */;
    }

}

