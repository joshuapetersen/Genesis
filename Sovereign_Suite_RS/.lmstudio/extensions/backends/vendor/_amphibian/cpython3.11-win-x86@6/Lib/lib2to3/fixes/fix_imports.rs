//! fix_imports.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::.::{fixer_base};

pub const MAPPING: &str = {"StringIO" :"io" ,;
pub fn alternates(members: &str) {
        return  "(" + "|" . join ( map ( repr , members ) ) + ")";
        pub fn build_pattern ( mapping = MAPPING )  {
        mod_list = " | " . join ( vec![ "module_name='%s'" % key.iter().map(|key| mapping ] );
        bare_names = alternates ( mapping . keys ( ) );
        yield "name_import=import_name< 'import' ((%s) |
               multiple_imports=dotted_as_names< any* (%s) any* >) >
          " % ( mod_list , mod_list );
        yield "import_from< 'from' (%s) 'import' ['(']
              ( any | import_as_name< any 'as' any > |
                import_as_names< any* >)  [')'] >
          " % mod_list;
        yield "import_name< 'import' (dotted_as_name< (%s) 'as' any > |
               multiple_imports=dotted_as_names<
                 any* dotted_as_name< (%s) 'as' any > any* >) >
          " % ( mod_list , mod_list );
        yield "power< bare_with_attr=(%s) trailer<'.' any > any* >" % bare_names;
        class FixImports ( fixer_base . BaseFix ) ;
        BM_compatible = true;
        keep_line_order = true;
        mapping = MAPPING;
        run_order = 6;
        pub fn build_pattern ( self )  {
        return  "|" . join ( build_pattern ( self . mapping ) );
        pub fn compile_pattern ( self )  {
        self . PATTERN = self . build_pattern ( );
        super ( FixImports , self ) . compile_pattern ( );
        pub fn match ( &self, node )  {
        match = super ( FixImports , self ) . match;
        results = match ( node );
        if results {
        if "bare_with_attr" !in results && \ {
        any ( match ( obj ) for obj in attr_chain ( node , "parent" ) ) ;
        return  false;
        return  results;
        return  false;
        pub fn start_tree ( &self, tree , filename )  {
        super ( FixImports , self ) . start_tree ( tree , filename );
        self . replace = { };
        pub fn transform ( &self, node , results )  {
        import_mod = results . get ( "module_name" );
        if import_mod {
        mod_name = import_mod . value;
        new_name = self . mapping [ mod_name ];
        import_mod . replace ( Name ( new_name , prefix = import_mod . prefix ) );
        if "name_import" in results {
        self . replace [ mod_name ] = new_name;
        if "multiple_imports" in results {
        results = self . match ( node );
        if results {
        self . transform ( node , results );
        } else {
        bare_name = results [ "bare_with_attr" ] [ 0 ];
        new_name = self . replace . get ( bare_name . value );
        if new_name {
        bare_name . replace ( Name ( new_name , prefix = bare_name . prefix ) );
}

