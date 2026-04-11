//! signal.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::_signal;
// use crate::IntEnum;

pub const _globals: /* inferred */ = globals ( );
pub fn _int_to_enum(value: &str, enum_klass: &str) {
        "Convert a possible numeric value to an IntEnum member.
    If it's !a known member, return the value itself.
    ";
        if !isinstance ( value , int ) {
        return  value;
        // try {
        return  enum_klass ( value );
        // } catch  ValueError  {
        return  value;
        pub fn _enum_to_int ( value )  {
        "Convert an IntEnum member to a numeric value.
    If it's !an IntEnum member return the value itself.
    ";
        // try {
        return  int ( value );
        // } catch  ( ValueError , TypeError )  {
        return  value;
        pub fn _wraps ( wrapped )  {
        pub fn decorator ( wrapper )  {
        wrapper . __doc__ = wrapped . __doc__;
        return  wrapper;
        return  decorator;
        @ _wraps ( _signal . signal );
        pub fn signal ( signalnum , handler )  {
        handler = _signal . signal ( _enum_to_int ( signalnum ) , _enum_to_int ( handler ) );
        return  _int_to_enum ( handler , Handlers );
        @ _wraps ( _signal . getsignal );
        pub fn getsignal ( signalnum )  {
        handler = _signal . getsignal ( signalnum );
        return  _int_to_enum ( handler , Handlers );
        if "pthread_sigmask" in _globals {
        @ _wraps ( _signal . pthread_sigmask );
        pub fn pthread_sigmask ( how , mask )  {
        sigs_set = _signal . pthread_sigmask ( how , mask );
        return  set ( _int_to_enum ( x , Signals ) for x in sigs_set );
        if "sigpending" in _globals {
        @ _wraps ( _signal . sigpending );
        pub fn sigpending ( )  {
        return  { _int_to_enum ( x , Signals ) for x in _signal . sigpending ( ) };
        if "sigwait" in _globals {
        @ _wraps ( _signal . sigwait );
        pub fn sigwait ( sigset )  {
        retsig = _signal . sigwait ( sigset );
        return  _int_to_enum ( retsig , Signals );
        if "valid_signals" in _globals {
        @ _wraps ( _signal . valid_signals );
        pub fn valid_signals ( )  {
        return  { _int_to_enum ( x , Signals ) for x in _signal . valid_signals ( ) };
        del _globals , _wraps;
}

