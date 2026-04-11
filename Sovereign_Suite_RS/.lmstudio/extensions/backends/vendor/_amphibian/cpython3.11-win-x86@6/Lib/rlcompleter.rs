//! rlcompleter.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::atexit;
// use crate::inspect;
// use regex::Regex;
// use crate::readline;

pub const __all__: &str = ["Completer" ];
pub struct Completer {
    pub use_main_ns: String, // TODO: infer type
    pub namespace: String, // TODO: infer type
    pub matches: String, // TODO: infer type
}

impl Completer {
    pub fn new(namespace: &str) -> Self {
        "Create a new completer for the command line.

        Completer([namespace]) -> completer instance.

        If unspecified, the default namespace where completions are performed
        == __main__ (technically, __main__.__dict__). Namespaces should be
        given as dictionaries.

        Completer instances should be used as the completion mechanism of
        readline via the set_completer() call:

        readline.set_completer(Completer(my_namespace).complete)
        ";
    }

    pub fn get_class_members(&self, klass: &str) {
        ret = dir ( klass );
        if hasattr ( klass , "__bases__" ) {
        for base in klass . __bases__ .iter() {
        ret = ret + get_class_members ( base );
        return  ret;
        // try {
        import readline;
        // } catch  ImportError  {
        _readline_available = false;
        } else {
        readline . set_completer ( Completer ( ) . complete );
        atexit . register ( || {  readline . set_completer ( None /* Option */ ) ) };
        _readline_available = true;
    }

}

