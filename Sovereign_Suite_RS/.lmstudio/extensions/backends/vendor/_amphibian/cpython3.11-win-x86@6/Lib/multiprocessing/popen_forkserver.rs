//! popen_forkserver.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::io;
// use crate::.::{reduction, set_spawning_popen};
// use crate::multiprocessing::{wait};

pub const __all__: &str = ["Popen" ];
pub struct _DupFd {
    pub ind: String, // TODO: infer type
    pub _fds: String, // TODO: infer type
    pub finalizer: String, // TODO: infer type
    pub pid: String, // TODO: infer type
    pub returncode: String, // TODO: infer type
}

impl _DupFd {
    pub fn new(ind: &str) -> Self {
        self . ind = ind;
        pub fn detach ( self )  {
        return  forkserver . get_inherited_fds ( ) [ self . ind ];
    }

}

