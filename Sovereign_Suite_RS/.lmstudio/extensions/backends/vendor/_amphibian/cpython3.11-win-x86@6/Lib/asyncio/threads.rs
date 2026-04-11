//! threads.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::functools;
// use crate::.::{events};

pub const __all__: &str = "to_thread" ,;
pub fn to_thread(func: &str, args: &str, kwargs: &str) {
        "Asynchronously run function *func* in a separate thread.

    Any *args && **kwargs supplied for this function are directly passed
    to *func*. Also, the current :class:`contextvars.Context` == propagated,
    allowing context variables from the main thread to be accessed in the
    separate thread.

    Return a coroutine that can be awaited to get the eventual result of *func*.
    ";
        loop = events . get_running_loop ( );
        ctx = contextvars . copy_context ( );
        func_call = functools . partial ( ctx . run , func , * args , ** kwargs );
        return  await loop . run_in_executor ( None /* Option */ , func_call );
}

