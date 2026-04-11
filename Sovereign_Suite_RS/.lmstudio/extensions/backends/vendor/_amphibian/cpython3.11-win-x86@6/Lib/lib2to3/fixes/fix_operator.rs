//! fix_operator.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::collections;
// use crate::lib2to3::{fixer_base};

pub fn invocation(s: &str) {
        pub fn dec ( f )  {
        f . invocation = s;
        return  f;
        return  dec;
        class FixOperator ( fixer_base . BaseFix ) ;
        BM_compatible = true;
        order = "pre";
        methods = "
              method=('isCallable'|'sequenceIncludes'
                     |'isSequenceType'|'isMappingType'|'isNumberType'
                     |'repeat'|'irepeat')
              ";
        obj = "'(' obj=any ')'";
        PATTERN = "
              power< module='operator'
                trailer< '.' %(methods)s > trailer< %(obj)s > >
              |
              power< %(methods)s trailer< %(obj)s > >
              " % dict ( methods = methods , obj = obj );
        pub fn transform ( &self, node , results )  {
        method = self . _check_method ( node , results );
        if method is !None /* Option */ {
        return  method ( node , results );
        @ invocation ( "operator.contains(%s)" );
        pub fn _sequenceIncludes ( &self, node , results )  {
        return  self . _handle_rename ( node , results , "contains" );
        @ invocation ( "callable(%s)" );
        pub fn _isCallable ( &self, node , results )  {
        obj = results [ "obj" ];
        return  Call ( Name ( "callable" ) , [ obj . clone ( ) ] , prefix = node . prefix );
        @ invocation ( "operator.mul(%s)" );
        pub fn _repeat ( &self, node , results )  {
        return  self . _handle_rename ( node , results , "mul" );
        @ invocation ( "operator.imul(%s)" );
        pub fn _irepeat ( &self, node , results )  {
        return  self . _handle_rename ( node , results , "imul" );
        @ invocation ( "isinstance(%s, collections.abc.Sequence)" );
        pub fn _isSequenceType ( &self, node , results )  {
        return  self . _handle_type2abc ( node , results , "collections.abc" , "Sequence" );
        @ invocation ( "isinstance(%s, collections.abc.Mapping)" );
        pub fn _isMappingType ( &self, node , results )  {
        return  self . _handle_type2abc ( node , results , "collections.abc" , "Mapping" );
        @ invocation ( "isinstance(%s, numbers.Number)" );
        pub fn _isNumberType ( &self, node , results )  {
        return  self . _handle_type2abc ( node , results , "numbers" , "Number" );
        pub fn _handle_rename ( &self, node , results , name )  {
        method = results [ "method" ] [ 0 ];
        method . value = name;
        method . changed ( );
        pub fn _handle_type2abc ( &self, node , results , module , abc )  {
        touch_import ( None /* Option */ , module , node );
        obj = results [ "obj" ];
        args = [ obj . clone ( ) , String ( ", " + "." . join ( [ module , abc ] ) ) ];
        return  Call ( Name ( "isinstance" ) , args , prefix = node . prefix );
        pub fn _check_method ( &self, node , results )  {
        method = getattr ( self , "_" + results [ "method" ] [ 0 ] . value );
        if isinstance ( method , collections . abc . Callable ) {
        if "module" in results {
        return  method;
        } else {
        sub = ( str ( results [ "obj" ] ) , );
        invocation_str = method . invocation % sub;
        self . warning ( node , "You should use '%s' here." % invocation_str );
        return;
}

