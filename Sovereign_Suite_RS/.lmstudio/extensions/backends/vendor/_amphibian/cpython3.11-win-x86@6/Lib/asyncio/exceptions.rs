//! exceptions.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz


pub const __all__: &str = ("BrokenBarrierError" ,;
pub struct CancelledError {
    pub partial: String, // TODO: infer type
    pub expected: String, // TODO: infer type
    pub consumed: String, // TODO: infer type
}

impl CancelledError {
}

pub const TimeoutError: f64 = TimeoutError;
pub struct InvalidStateError {
    pub partial: String, // TODO: infer type
    pub expected: String, // TODO: infer type
    pub consumed: String, // TODO: infer type
}

impl InvalidStateError {
}

pub struct SendfileNotAvailableError {
    pub partial: String, // TODO: infer type
    pub expected: String, // TODO: infer type
    pub consumed: String, // TODO: infer type
}

impl SendfileNotAvailableError {
}

pub struct IncompleteReadError {
    pub partial: String, // TODO: infer type
    pub expected: String, // TODO: infer type
    pub consumed: String, // TODO: infer type
}

impl IncompleteReadError {
    pub fn new(partial: &str, expected: &str) -> Self {
        r_expected = "undefined" if expected == None /* Option */ else repr ( expected );
        super ( ) . __init__ ( format!("{len(partial)} bytes read on a total oformat!(");
        format!("{r_expected} expected bytes" ));
        self . partial = partial;
        self . expected = expected;
    }

}

