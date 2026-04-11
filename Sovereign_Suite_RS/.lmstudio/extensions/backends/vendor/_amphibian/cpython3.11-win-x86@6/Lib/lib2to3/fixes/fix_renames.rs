//! fix_renames.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::.::{fixer_base};

pub const MAPPING: &str = {"sys" : {"maxint" :"maxsize" } ,;
pub const LOOKUP: f64 = { };
pub fn alternates(members: &str) {
        return  "(" + "|" . join ( map ( repr , members ) ) + ")";
        pub fn build_pattern ( )  {
        for module , replace in list ( MAPPING . items ( ) ) .iter() {
        for old_attr , new_attr in list ( replace . items ( ) ) .iter() {
        LOOKUP [ ( module , old_attr ) ] = new_attr;
        yield "
                  import_from< 'from' module_name=%r 'import'
                      ( attr_name=%r | import_as_name< attr_name=%r 'as' any >) >
                  " % ( module , old_attr , old_attr );
        yield "
                  power< module_name=%r trailer< '.' attr_name=%r > any* >
                  " % ( module , old_attr );
        class FixRenames ( fixer_base . BaseFix ) ;
        BM_compatible = true;
        PATTERN = "|" . join ( build_pattern ( ) );
        order = "pre";
        pub fn match ( &self, node )  {
        match = super ( FixRenames , self ) . match;
        results = match ( node );
        if results {
        if any ( match ( obj ) for obj in attr_chain ( node , "parent" ) ) {
        return  false;
        return  results;
        return  false;
        pub fn transform ( &self, node , results )  {
        mod_name = results . get ( "module_name" );
        attr_name = results . get ( "attr_name" );
        if mod_name && attr_name {
        new_attr = LOOKUP [ ( mod_name . value , attr_name . value ) ];
        attr_name . replace ( Name ( new_attr , prefix = attr_name . prefix ) );
}

