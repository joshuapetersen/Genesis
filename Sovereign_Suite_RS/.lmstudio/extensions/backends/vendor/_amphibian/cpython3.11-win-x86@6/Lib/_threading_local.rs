//! _threading_local.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::weakref::{ref};
// use crate::contextlib::{contextmanager};
// use std::thread::{current_thread, RLock};

pub const __all__: &str = ["local" ];
pub struct _localimpl {
    pub key: String, // TODO: infer type
    pub dicts: String, // TODO: infer type
}

impl _localimpl {
}

pub fn _patch() {
        impl = object . __getattribute__ ( self , "_local__impl" );
        // try {
        dct = impl . get_dict ( );
        // } catch  KeyError  {
        dct = impl . create_dict ( );
        args , kw = impl . localargs;
        self . __init__ ( * args , ** kw );
        // with scope: impl . locallock  {
        object . __setattr__ ( self , "__dict__" , dct );
        yield;
        class local ;
        __slots__ = "_local__impl" , "__dict__";
        pub fn __new__ ( cls , / , * args , ** kw )  {
        if ( args || kw ) && ( cls . __init__ is object . __init__ ) {
        panic!("TypeError ( "Initialization arguments are !supported" )");
        self = object . __new__ ( cls );
        impl = _localimpl ( );
        impl . localargs = ( args , kw );
        impl . locallock = RLock ( );
        object . __setattr__ ( self , "_local__impl" , impl );
        impl . create_dict ( );
        return  self;
        pub fn __getattribute__ ( &self, name )  {
        // with scope: _patch ( self )  {
        return  object . __getattribute__ ( self , name );
        pub fn __setattr__ ( &self, name , value )  {
        if name == "__dict__" {
        panic!("AttributeError (");
        "%r object attribute '__dict__' == read-only";
        % self . __class__ . __name__ );
        // with scope: _patch ( self )  {
        return  object . __setattr__ ( self , name , value );
        pub fn __delattr__ ( &self, name )  {
        if name == "__dict__" {
        panic!("AttributeError (");
        "%r object attribute '__dict__' == read-only";
        % self . __class__ . __name__ );
        // with scope: _patch ( self )  {
        return  object . __delattr__ ( self , name );
        from threading import current_thread , RLock;
}

