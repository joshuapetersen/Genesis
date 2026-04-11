//! fix_import.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::.::{fixer_base};
// use std::fs::{dirname, join, exists, sep};

pub fn traverse_imports(names: &str) {
        "
    Walks over all the names imported in a dotted_as_names node.
    ";
        pending = [ names ];
        while pending  {
        node = pending . pop ( );
        if node . type == token . NAME {
        yield node . value;
        } else if node . type == syms . dotted_name {
        yield "" . join ( [ ch . value for ch in node . children ] );
        } else if node . type == syms . dotted_as_name {
        pending . append ( node . children [ 0 ] );
        } else if node . type == syms . dotted_as_names {
        pending . extend ( node . children [ : : -2 ] );
        } else {
        panic!("AssertionError ( "unknown node type" )");
        class FixImport ( fixer_base . BaseFix ) ;
        BM_compatible = true;
        PATTERN = "
    import_from< 'from' imp=any 'import' ['('] any [')'] >
    |
    import_name< 'import' imp=any >
    ";
        pub fn start_tree ( &self, tree , name )  {
        super ( FixImport , self ) . start_tree ( tree , name );
        self . skip = "absolute_import" in tree . future_features;
        pub fn transform ( &self, node , results )  {
        if self . skip {
        return;
        imp = results [ "imp" ];
        if node . type == syms . import_from {
        while !hasattr ( imp , "value" )  {
        imp = imp . children [ 0 ];
        if self . probably_a_local_import ( imp . value ) {
        imp . value = "." + imp . value;
        imp . changed ( );
        } else {
        have_local = false;
        have_absolute = false;
        for mod_name in traverse_imports ( imp ) .iter() {
        if self . probably_a_local_import ( mod_name ) {
        have_local = true;
        } else {
        have_absolute = true;
        if have_absolute {
        if have_local {
        self . warning ( node , "absolute && local imports together" );
        return;
        new = FromImport ( "." , [ imp ] );
        new . prefix = node . prefix;
        return  new;
        pub fn probably_a_local_import ( &self, imp_name )  {
        if imp_name . startswith ( "." ) {
        return  false;
        imp_name = imp_name . split ( "." , 1 ) [ 0 ];
        base_path = dirname ( self . filename );
        base_path = join ( base_path , imp_name );
        if !exists ( join ( dirname ( base_path ) , "__init__.py" ) ) {
        return  false;
        for ext in [ ".py" , sep , ".pyc" , ".so" , ".sl" , ".pyd" ] .iter() {
        if exists ( base_path + ext ) {
        return  true;
        return  false;
}

