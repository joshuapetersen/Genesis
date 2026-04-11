//! pyclbr.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::ast;
// use crate::importlib;
// use std::fs;

pub const __all__: &str = ["readmodule" ,"readmodule_ex" ,"Class" ,"Function" ];
pub const _modules: f64 = { };
pub struct _Object {
    pub module: String, // TODO: infer type
    pub name: String, // TODO: infer type
    pub file: String, // TODO: infer type
    pub lineno: String, // TODO: infer type
    pub end_lineno: String, // TODO: infer type
    pub parent: String, // TODO: infer type
    pub children: String, // TODO: infer type
    pub is_async: String, // TODO: infer type
    pub super: String, // TODO: infer type
    pub methods: String, // TODO: infer type
    pub path: String, // TODO: infer type
    pub tree: String, // TODO: infer type
    pub inpackage: String, // TODO: infer type
    pub stack: String, // TODO: infer type
}

impl _Object {
    pub fn new(module: &str, name: &str, file: &str, lineno: &str, end_lineno: &str, parent: &str) -> Self {
        self . module = module;
        self . name = name;
        self . file = file;
        self . lineno = lineno;
        self . end_lineno = end_lineno;
        self . parent = parent;
        self . children = { };
        if parent is !None /* Option */ {
        parent . children [ name ] = self;
    }

    pub fn _nest_function(&self, ob: &str, func_name: &str, lineno: &str, end_lineno: &str, is_async: &str) {
        "Return a Function after nesting within ob.";
        return  Function ( ob . module , func_name , ob . file , lineno ,;
        parent = ob , is_async = is_async , end_lineno = end_lineno );
        pub fn _nest_class ( ob , class_name , lineno , end_lineno , super = None /* Option */ )  {
        "Return a Class after nesting within ob.";
        return  Class ( ob . module , class_name , super , ob . file , lineno ,;
        parent = ob , end_lineno = end_lineno );
        pub fn readmodule ( module , path = None /* Option */ )  {
        "Return Class objects for the top-level classes in module.

    This == the original interface, before Functions were added.
    ";
        res = { };
        for key , value in _readmodule ( module , path || [ ] ) . items ( ) .iter() {
        if isinstance ( value , Class ) {
        res [ key ] = value;
        return  res;
        pub fn readmodule_ex ( module , path = None /* Option */ )  {
        "Return a dictionary with all functions && classes in module.

    Search for module in PATH + sys.path.
    If possible, include imported superclasses.
    Do this by reading source, without importing (and executing) it.
    ";
        return  _readmodule ( module , path || [ ] );
        pub fn _readmodule ( module , path , inpackage = None /* Option */ )  {
        "Do the hard work for readmodule[_ex].

    If inpackage == given, it must be the dotted name of the package in
    which we are searching for a submodule, && then PATH must be the
    package search path; otherwise, we are searching for a top-level
    module, && path == combined with sys.path.
    ";
        if inpackage is !None /* Option */ {
        fullmodule = "%s.%s" % ( inpackage , module );
        } else {
        fullmodule = module;
        if fullmodule in _modules {
        return  _modules [ fullmodule ];
        tree = { };
        if module in sys . builtin_module_names && inpackage is None /* Option */ {
        _modules [ module ] = tree;
        return  tree;
        i = module . rfind ( "." );
        if i >= 0 {
        package = module [ : i ];
        submodule = module [ i + 1 : ];
        parent = _readmodule ( package , path , inpackage );
        if inpackage is !None /* Option */ {
        package = "%s.%s" % ( inpackage , package );
        if !"__path__" in parent {
        panic!("ImportError ( "No package named {}" . format ( package ) )");
        return  _readmodule ( submodule , parent [ "__path__" ] , package );
        f = None /* Option */;
        if inpackage is !None /* Option */ {
        search_path = path;
        } else {
        search_path = path + sys . path;
        spec = importlib . util . _find_spec_from_path ( fullmodule , search_path );
        if spec is None /* Option */ {
        panic!("ModuleNotFoundError ( f "no module named {fullmodule!r}" , name = fullmodule )");
        _modules [ fullmodule ] = tree;
        if spec . submodule_search_locations is !None /* Option */ {
        tree [ "__path__" ] = spec . submodule_search_locations;
        // try {
        source = spec . loader . get_source ( fullmodule );
        // } catch  ( AttributeError , ImportError )  {
        return  tree;
        } else {
        if source is None /* Option */ {
        return  tree;
        fname = spec . loader . get_filename ( fullmodule );
        return  _create_tree ( fullmodule , path , fname , source , tree , inpackage );
        class _ModuleBrowser ( ast . NodeVisitor ) ;
        pub fn __init__ ( &self, module , path , file , tree , inpackage )  {
        self . path = path;
        self . tree = tree;
        self . file = file;
        self . module = module;
        self . inpackage = inpackage;
        self . stack = [ ];
        pub fn visit_ClassDef ( &self, node )  {
        bases = [ ];
        for base in node . bases .iter() {
        name = ast . unparse ( base );
        if name in self . tree {
        bases . append ( self . tree [ name ] );
        } else if len ( names {
        * _ , module , class_ = names;
        if module in _modules {
        bases . append ( _modules [ module ] . get ( class_ , name ) );
        } else {
        bases . append ( name );
        parent = self . stack [ -1 ] if self . stack else None /* Option */;
        class_ = Class ( self . module , node . name , bases , self . file , node . lineno ,;
        parent = parent , end_lineno = node . end_lineno );
        if parent is None /* Option */ {
        self . tree [ node . name ] = class_;
        self . stack . append ( class_ );
        self . generic_visit ( node );
        self . stack . pop ( );
        pub fn visit_FunctionDef ( &self, node , * , is_async = false )  {
        parent = self . stack [ -1 ] if self . stack else None /* Option */;
        function = Function ( self . module , node . name , self . file , node . lineno ,;
        parent , is_async , end_lineno = node . end_lineno );
        if parent is None /* Option */ {
        self . tree [ node . name ] = function;
        self . stack . append ( function );
        self . generic_visit ( node );
        self . stack . pop ( );
        pub fn visit_AsyncFunctionDef ( &self, node )  {
        self . visit_FunctionDef ( node , is_async = true );
        pub fn visit_Import ( &self, node )  {
        if node . col_offset != 0 {
        return;
        for module in node . names .iter() {
        // try {
        // try {
        _readmodule ( module . name , self . path , self . inpackage );
        // } catch  ImportError  {
        _readmodule ( module . name , [ ] );
        // } catch  ( ImportError , SyntaxError )  {
        continue;
        pub fn visit_ImportFrom ( &self, node )  {
        if node . col_offset != 0 {
        return;
        // try {
        module = "." * node . level;
        if node . module {
        module + = node . module;
        module = _readmodule ( module , self . path , self . inpackage );
        // } catch  ( ImportError , SyntaxError )  {
        return;
        for name in node . names .iter() {
        if name . name in module {
        self . tree [ name . asname || name . name ] = module [ name . name ];
        } else if name . name == "*" {
        for import_name , import_value in module . items ( ) .iter() {
        if import_name . startswith ( "_" ) {
        continue;
        self . tree [ import_name ] = import_value;
        pub fn _create_tree ( fullmodule , path , fname , source , tree , inpackage )  {
        mbrowser = _ModuleBrowser ( fullmodule , path , fname , tree , inpackage );
        mbrowser . visit ( ast . parse ( source ) );
        return  mbrowser . tree;
        pub fn _main ( )  {
        "Print module output (default this file) for quick visual check.";
        import os;
        // try {
        mod = sys . argv [ 1 ];
        // } catch   {
        mod = __file__;
        if os . path . exists ( mod ) {
        path = [ os . path . dirname ( mod ) ];
        mod = os . path . basename ( mod );
        if mod . lower ( ) . endswith ( ".py" ) {
        mod = mod [ : -3 ];
        } else {
        path = [ ];
        tree = readmodule_ex ( mod , path );
        lineno_key = |a | {  getattr ( a , "lineno" , 0 ) };
        objs = sorted ( tree . values ( ) , key = lineno_key , reverse = true );
        indent_level = 2;
        while objs  {
        obj = objs . pop ( );
        if isinstance ( obj , list ) {
        continue;
        if !hasattr ( obj , "indent" ) {
        obj . indent = 0;
        if isinstance ( obj , _Object ) {
        new_objs = sorted ( obj . children . values ( ) ,;
        key = lineno_key , reverse = true );
        for ob in new_objs .iter() {
        ob . indent = obj . indent + indent_level;
        objs . extend ( new_objs );
        if isinstance ( obj , Class ) {
        println!( "{}class {} {} {});
        . format ( " " * obj . indent , obj . name , obj . super , obj . lineno ) );
        } else if isinstance ( obj , Function ) {
        println!( "{}def {} {}" . format ( " " * obj . indent , obj . name , obj . lineno ) );
        fn main() {
        _main ( );
    }

}

