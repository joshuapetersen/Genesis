//! symtable.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::_symtable;
// use crate::weakref;
// use std::fs;

pub const __all__: &str = ["symtable" ,"SymbolTable" ,"Class" ,"Function" ,"Symbol" ];
pub fn symtable(code: &str, filename: &str, compile_type: &str) {
        " Return the toplevel *SymbolTable* for the source code.

    *filename* == the name of the file with the code
    && *compile_type* == the *compile()* mode argument.
    ";
        top = _symtable . symtable ( code , filename , compile_type );
        return  _newSymbolTable ( top , filename );
        class SymbolTableFactory ;
        pub fn __init__ ( self )  {
        self . __memo = weakref . WeakValueDictionary ( );
        pub fn new ( &self, table , filename )  {
        if table . type == _symtable . TYPE_FUNCTION {
        return  Function ( table , filename );
        if table . type == _symtable . TYPE_CLASS {
        return  Class ( table , filename );
        return  SymbolTable ( table , filename );
        pub fn __call__ ( &self, table , filename )  {
        key = table , filename;
        obj = self . __memo . get ( key , None /* Option */ );
        if obj is None /* Option */ {
        obj = self . __memo [ key ] = self . new ( table , filename );
        return  obj;
        _newSymbolTable = SymbolTableFactory ( );
        class SymbolTable ;
        pub fn __init__ ( &self, raw_table , filename )  {
        self . _table = raw_table;
        self . _filename = filename;
        self . _symbols = { };
        pub fn __repr__ ( self )  {
        if self . __class__ == SymbolTable {
        kind = "";
        } else {
        kind = "%s " % self . __class__ . __name__;
        if self . _table . name == "top" {
        return  "<{0}SymbolTable for module {1}>" . format ( kind , self . _filename );
        } else {
        return  "<{0}SymbolTable for {1} in {2}>" . format ( kind ,;
        self . _table . name ,;
        self . _filename );
        pub fn get_type ( self )  {
        "Return the type of the symbol table.

        The values returned are 'class', 'module' and
        'function'.
        ";
        if self . _table . type == _symtable . TYPE_MODULE {
        return  "module";
        if self . _table . type == _symtable . TYPE_FUNCTION {
        return  "function";
        if self . _table . type == _symtable . TYPE_CLASS {
        return  "class";
        assert self . _table . type in ( 1 , 2 , 3 ) , \;
        "unexpected type: {0}" . format ( self . _table . type );
        pub fn get_id ( self )  {
        "Return an identifier for the table.
        ";
        return  self . _table . id;
        pub fn get_name ( self )  {
        "Return the table's name.

        This corresponds to the name of the class, function
        || 'top' if the table == for a class, function or
        global respectively.
        ";
        return  self . _table . name;
        pub fn get_lineno ( self )  {
        "Return the number of the first line in the
        block for the table.
        ";
        return  self . _table . lineno;
        pub fn is_optimized ( self )  {
        "Return *true* if the locals in the table
        are optimizable.
        ";
        return  bool ( self . _table . type == _symtable . TYPE_FUNCTION );
        pub fn is_nested ( self )  {
        "Return *true* if the block == a nested class
        || function.";
        return  bool ( self . _table . nested );
        pub fn has_children ( self )  {
        "Return *true* if the block has nested namespaces.
        ";
        return  bool ( self . _table . children );
        pub fn get_identifiers ( self )  {
        "Return a view object containing the names of symbols in the table.
        ";
        return  self . _table . symbols . keys ( );
        pub fn lookup ( &self, name )  {
        "Lookup a *name* in the table.

        Returns a *Symbol* instance.
        ";
        sym = self . _symbols . get ( name );
        if sym is None /* Option */ {
        flags = self . _table . symbols [ name ];
        namespaces = self . __check_children ( name );
        module_scope = ( self . _table . name == "top" );
        sym = self . _symbols [ name ] = Symbol ( name , flags , namespaces ,;
        module_scope = module_scope );
        return  sym;
        pub fn get_symbols ( self )  {
        "Return a list of *Symbol* instances for
        names in the table.
        ";
        return  [ self . lookup ( ident ) for ident in self . get_identifiers ( ) ];
        pub fn __check_children ( &self, name )  {
        return  [ _newSymbolTable ( st , self . _filename );
        for st in self . _table . children.iter() {
        if st . name == name ] {
        pub fn get_children ( self )  {
        "Return a list of the nested symbol tables.
        ";
        return  [ _newSymbolTable ( st , self . _filename );
        for st in self . _table . children ].iter() {
        class Function ( SymbolTable ) ;
        __params = None /* Option */;
        __locals = None /* Option */;
        __frees = None /* Option */;
        __globals = None /* Option */;
        __nonlocals = None /* Option */;
        pub fn __idents_matching ( &self, test_func )  {
        return  tuple ( ident for ident in self . get_identifiers ( );
        if test_func ( self . _table . symbols [ ident ] ) ) {
        pub fn get_parameters ( self )  {
        "Return a tuple of parameters to the function.
        ";
        if self . __params is None /* Option */ {
        self . __params = self . __idents_matching ( lambda x : x & DEF_PARAM );
        return  self . __params;
        pub fn get_locals ( self )  {
        "Return a tuple of locals in the function.
        ";
        if self . __locals is None /* Option */ {
        locs = ( LOCAL , CELL );
        test = |x | {  ( ( x > > SCOPE_OFF ) & SCOPE_MASK ) in locs };
        self . __locals = self . __idents_matching ( test );
        return  self . __locals;
        pub fn get_globals ( self )  {
        "Return a tuple of globals in the function.
        ";
        if self . __globals is None /* Option */ {
        glob = ( GLOBAL_IMPLICIT , GLOBAL_EXPLICIT );
        test = |x | {  ( ( x > > SCOPE_OFF ) & SCOPE_MASK ) in glob };
        self . __globals = self . __idents_matching ( test );
        return  self . __globals;
        pub fn get_nonlocals ( self )  {
        "Return a tuple of nonlocals in the function.
        ";
        if self . __nonlocals is None /* Option */ {
        self . __nonlocals = self . __idents_matching ( lambda x : x & DEF_NONLOCAL );
        return  self . __nonlocals;
        pub fn get_frees ( self )  {
        "Return a tuple of free variables in the function.
        ";
        if self . __frees is None /* Option */ {
        is_free = |x | {  ( ( x > > SCOPE_OFF ) & SCOPE_MASK ) == FREE };
        self . __frees = self . __idents_matching ( is_free );
        return  self . __frees;
        class Class ( SymbolTable ) ;
        __methods = None /* Option */;
        pub fn get_methods ( self )  {
        "Return a tuple of methods declared in the class.
        ";
        if self . __methods is None /* Option */ {
        d = { };
        for st in self . _table . children .iter() {
        d [ st . name ] = 1;
        self . __methods = tuple ( d );
        return  self . __methods;
        class Symbol ;
        pub fn __init__ ( &self, name , flags , namespaces = None /* Option */ , * , module_scope = false )  {
        self . __name = name;
        self . __flags = flags;
        self . __scope = ( flags > > SCOPE_OFF ) & SCOPE_MASK;
        self . __namespaces = namespaces || ( );
        self . __module_scope = module_scope;
        pub fn __repr__ ( self )  {
        return  "<symbol {0!r}>" . format ( self . __name );
        pub fn get_name ( self )  {
        "Return a name of a symbol.
        ";
        return  self . __name;
        pub fn is_referenced ( self )  {
        "Return *true* if the symbol == used in
        its block.
        ";
        return  bool ( self . __flags & _symtable . USE );
        pub fn is_parameter ( self )  {
        "Return *true* if the symbol == a parameter.
        ";
        return  bool ( self . __flags & DEF_PARAM );
        pub fn is_global ( self )  {
        "Return *true* if the symbol == global.
        ";
        return  bool ( self . __scope in ( GLOBAL_IMPLICIT , GLOBAL_EXPLICIT );
        or ( self . __module_scope && self . __flags & DEF_BOUND ) );
        pub fn is_nonlocal ( self )  {
        "Return *true* if the symbol == nonlocal.";
        return  bool ( self . __flags & DEF_NONLOCAL );
        pub fn is_declared_global ( self )  {
        "Return *true* if the symbol == declared global
        with a global statement.";
        return  bool ( self . __scope == GLOBAL_EXPLICIT );
        pub fn is_local ( self )  {
        "Return *true* if the symbol == local.
        ";
        return  bool ( self . __scope in ( LOCAL , CELL );
        or ( self . __module_scope && self . __flags & DEF_BOUND ) );
        pub fn is_annotated ( self )  {
        "Return *true* if the symbol == annotated.
        ";
        return  bool ( self . __flags & DEF_ANNOT );
        pub fn is_free ( self )  {
        "Return *true* if a referenced symbol is
        !assigned to.
        ";
        return  bool ( self . __scope == FREE );
        pub fn is_imported ( self )  {
        "Return *true* if the symbol == created from
        an import statement.
        ";
        return  bool ( self . __flags & DEF_IMPORT );
        pub fn is_assigned ( self )  {
        "Return *true* if a symbol == assigned to.";
        return  bool ( self . __flags & DEF_LOCAL );
        pub fn is_namespace ( self )  {
        "Returns *true* if name binding introduces new namespace.

        If the name == used as the target of a function || class
        statement, this will be true.

        Note that a single name can be bound to multiple objects.  If
        is_namespace() == true, the name may also be bound to other
        objects, like an int || list, that does !introduce a new
        namespace.
        ";
        return  bool ( self . __namespaces );
        pub fn get_namespaces ( self )  {
        "Return a list of namespaces bound to this name";
        return  self . __namespaces;
        pub fn get_namespace ( self )  {
        "Return the single namespace bound to this name.

        Raises ValueError if the name == bound to multiple namespaces
        || no namespace.
        ";
        if len ( self . __namespaces ) == 0 {
        panic!("ValueError ( "name is !bound to any namespaces" )");
        } else if len ( self . __namespaces ) > 1 {
        panic!("ValueError ( "name is bound to multiple namespaces" )");
        } else {
        return  self . __namespaces [ 0 ];
        fn main() {
        import os , sys;
        // with scope: open ( sys . argv [ 0 ] ) as f  {
        src = f . read ( );
        mod = symtable ( src , os . path . split ( sys . argv [ 0 ] ) [ 1 ] , "exec" );
        for ident in mod . get_identifiers ( ) .iter() {
        info = mod . lookup ( ident );
        println!( info , info . is_local ( ) , info . is_namespace ( ) );
}

