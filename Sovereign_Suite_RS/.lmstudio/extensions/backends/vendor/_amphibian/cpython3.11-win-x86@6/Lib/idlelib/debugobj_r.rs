//! debugobj_r.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::idlelib::{rpc};
// use crate::unittest::{main};

pub fn remote_object_tree_item(item: &str) {
        wrapper = WrappedObjectTreeItem ( item );
        oid = id ( wrapper );
        rpc . objecttable [ oid ] = wrapper;
        return  oid;
        class WrappedObjectTreeItem ;
        pub fn __init__ ( &self, item )  {
        self . __item = item;
        pub fn __getattr__ ( &self, name )  {
        value = getattr ( self . __item , name );
        return  value;
        pub fn _GetSubList ( self )  {
        sub_list = self . __item . _GetSubList ( );
        return  list ( map ( remote_object_tree_item , sub_list ) );
        class StubObjectTreeItem ;
        pub fn __init__ ( &self, sockio , oid )  {
        self . sockio = sockio;
        self . oid = oid;
        pub fn __getattr__ ( &self, name )  {
        value = rpc . MethodProxy ( self . sockio , self . oid , name );
        return  value;
        pub fn _GetSubList ( self )  {
        sub_list = self . sockio . remotecall ( self . oid , "_GetSubList" , ( ) , { } );
        return  [ StubObjectTreeItem ( self . sockio , oid ) for oid in sub_list ];
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_debugobj_r" , verbosity = 2 );
}

