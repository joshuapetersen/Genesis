//! fix_urllib.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::lib2to3::{alternates, FixImports};

pub const MAPPING: &str = {"urllib" : [;
pub fn build_pattern() {
        bare = set ( );
        for old_module , changes in MAPPING . items ( ) .iter() {
        for change in changes .iter() {
        new_module , members = change;
        members = alternates ( members );
        yield "import_name< 'import' (module=%r
                                  | dotted_as_names< any* module=%r any* >) >
                  " % ( old_module , old_module );
        yield "import_from< 'from' mod_member=%r 'import'
                       ( member=%s | import_as_name< member=%s 'as' any > |
                         import_as_names< members=any*  >) >
                  " % ( old_module , members , members );
        yield "import_from< 'from' module_star=%r 'import' star='*' >
                  " % old_module;
        yield "import_name< 'import'
                                  dotted_as_name< module_as=%r 'as' any > >
                  " % old_module;
        yield "power< bare_with_attr=%r trailer< '.' member=%s > any* >
                  " % ( old_module , members );
        class FixUrllib ( FixImports ) ;
        pub fn build_pattern ( self )  {
        return  "|" . join ( build_pattern ( ) );
        pub fn transform_import ( &self, node , results )  {
        "Transform for the basic import case. Replaces the old
           import name with a comma separated list of its
           replacements.
        ";
        import_mod = results . get ( "module" );
        pref = import_mod . prefix;
        names = [ ];
        for name in MAPPING [ import_mod . value ] [ : -1 ] .iter() {
        names . extend ( [ Name ( name [ 0 ] , prefix = pref ) , Comma ( ) ] );
        names . append ( Name ( MAPPING [ import_mod . value ] [ -1 ] [ 0 ] , prefix = pref ) );
        import_mod . replace ( names );
        pub fn transform_member ( &self, node , results )  {
        "Transform for imports of specific module elements. Replaces
           the module to be imported from with the appropriate new
           module.
        ";
        mod_member = results . get ( "mod_member" );
        pref = mod_member . prefix;
        member = results . get ( "member" );
        if member {
        if isinstance ( member , list ) {
        member = member [ 0 ];
        new_name = None /* Option */;
        for change in MAPPING [ mod_member . value ] .iter() {
        if member . value in change [ 1 ] {
        new_name = change [ 0 ];
        break;
        if new_name {
        mod_member . replace ( Name ( new_name , prefix = pref ) );
        } else {
        self . cannot_convert ( node , "This is an invalid module element" );
        } else {
        modules = [ ];
        mod_dict = { };
        members = results [ "members" ];
        for member in members .iter() {
        if member . type == syms . import_as_name {
        as_name = member . children [ 2 ] . value;
        member_name = member . children [ 0 ] . value;
        } else {
        member_name = member . value;
        as_name = None /* Option */;
        if member_name != "," {
        for change in MAPPING [ mod_member . value ] .iter() {
        if member_name in change [ 1 ] {
        if change [ 0 ] !in mod_dict {
        modules . append ( change [ 0 ] );
        mod_dict . setdefault ( change [ 0 ] , [ ] ) . append ( member );
        new_nodes = [ ];
        indentation = find_indentation ( node );
        first = true;
        pub fn handle_name ( name , prefix )  {
        if name . type == syms . import_as_name {
        kids = [ Name ( name . children [ 0 ] . value , prefix = prefix ) ,;
        name . children [ 1 ] . clone ( ) ,;
        name . children [ 2 ] . clone ( ) ];
        return  [ Node ( syms . import_as_name , kids ) ];
        return  [ Name ( name . value , prefix = prefix ) ];
        for module in modules .iter() {
        elts = mod_dict [ module ];
        names = [ ];
        for elt in elts [ : -1 ] .iter() {
        names . extend ( handle_name ( elt , pref ) );
        names . append ( Comma ( ) );
        names . extend ( handle_name ( elts [ -1 ] , pref ) );
        new = FromImport ( module , names );
        if !first || node . parent . prefix . endswith ( indentation ) {
        new . prefix = indentation;
        new_nodes . append ( new );
        first = false;
        if new_nodes {
        nodes = [ ];
        for new_node in new_nodes [ : -1 ] .iter() {
        nodes . extend ( [ new_node , Newline ( ) ] );
        nodes . append ( new_nodes [ -1 ] );
        node . replace ( nodes );
        } else {
        self . cannot_convert ( node , "All module elements are invalid" );
        pub fn transform_dot ( &self, node , results )  {
        "Transform for calls to module members in code.";
        module_dot = results . get ( "bare_with_attr" );
        member = results . get ( "member" );
        new_name = None /* Option */;
        if isinstance ( member , list ) {
        member = member [ 0 ];
        for change in MAPPING [ module_dot . value ] .iter() {
        if member . value in change [ 1 ] {
        new_name = change [ 0 ];
        break;
        if new_name {
        module_dot . replace ( Name ( new_name ,;
        prefix = module_dot . prefix ) );
        } else {
        self . cannot_convert ( node , "This is an invalid module element" );
        pub fn transform ( &self, node , results )  {
        if results . get ( "module" ) {
        self . transform_import ( node , results );
        } else if results . get ( "mod_member" ) {
        self . transform_member ( node , results );
        } else if results . get ( "bare_with_attr" ) {
        self . transform_dot ( node , results );
        } else if results . get ( "module_star" ) {
        self . cannot_convert ( node , "Cannot handle star imports." );
        } else if results . get ( "module_as" ) {
        self . cannot_convert ( node , "This module is now multiple modules" );
}

