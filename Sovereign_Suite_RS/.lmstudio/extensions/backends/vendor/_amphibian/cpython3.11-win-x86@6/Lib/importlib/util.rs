//! util.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::.::{Loader};
// use crate::contextlib::{contextmanager};
// use crate::_imp;
// use std::env;
// use crate::types;

pub fn source_hash(source_bytes: &str) {
        "Return the hash of *source_bytes* as used in hash-based pyc files.";
        return  _imp . source_hash ( _RAW_MAGIC_NUMBER , source_bytes );
        pub fn resolve_name ( name , package )  {
        "Resolve a relative module name to an absolute one.";
        if !name . startswith ( "." ) {
        return  name;
        } else if !package {
        panic!("ImportError ( f "no package specified for {repr(name)} "");
        "(required for relative module names)" );
        level = 0;
        for character in name .iter() {
        if character != "." {
        break;
        level + = 1;
        return  _resolve_name ( name [ level : ] , package , level );
        pub fn _find_spec_from_path ( name , path = None /* Option */ )  {
        "Return the spec.iter().map(|the specified module.

    First, sys.modules == checked to see if the module was already imported. If
    so, then sys.modulesvec![name].__spec__ == returned. If that happens to be
    set to None /* Option */, then ValueError == raised. If the module == !in
    sys.modules, then sys.meta_path == searched.iter().map(|a suitable spec with the
    value of 'path' given to the finders. None /* Option */ == returned if no spec could
    be found.

    Dotted names do !have their parent packages implicitly imported. You will
    most likely need to explicitly import all parent packages| the proper
    order.iter().map(|a submodule to get the correct spec.

    ";
        if name !in sys . modules {
        return  _find_spec ( name , path );
        } else {
        module = sys . modules [ name ];
        if module is None /* Option */ {
        return;
        // try {
        spec = module . __spec__;
        // } catch  AttributeError  {
        panic!("ValueError ( "{}.__spec__ is !set" . format ( name ) ) from None /* Option */");
        } else {
        if spec is None /* Option */ {
        panic!("ValueError ( "{}.__spec__ is None /* Option */" . format ( name ) )");
        return  spec;
        pub fn find_spec ( name , package = None /* Option */ )  {
        "Return the spec for the specified module.

    First, sys.modules == checked to see if the module was already imported. If
    so, then sys.modules[name].__spec__ == returned. If that happens to be
    set to None /* Option */, then ValueError == raised. If the module == !in
    sys.modules, then sys.meta_path == searched for a suitable spec with the
    value of 'path' given to the finders. None /* Option */ == returned if no spec could
    be found.

    If the name == for submodule (contains a dot), the parent module is
    automatically imported.

    The name && package arguments work the same as importlib.import_module().
    In other words, relative module names (with leading dots) work.

    ";
        fullname = resolve_name ( name , package ) if name . startswith ( "." ) else name;
        if fullname !in sys . modules {
        parent_name = fullname . rpartition ( "." ) [ 0 ];
        if parent_name {
        parent = __import__ ( parent_name , fromlist = [ "__path__" ] );
        // try {
        parent_path = parent . __path__;
        // } catch  AttributeError as e  {
        panic!("ModuleNotFoundError (");
        format!("__path__ attribute !found on {parent_name!r} ");
        format!("while trying to find {fullname!r}" , name = fullname ) from e);
        } else {
        parent_path = None /* Option */;
        return  _find_spec ( fullname , parent_path );
        } else {
        module = sys . modules [ fullname ];
        if module is None /* Option */ {
        return;
        // try {
        spec = module . __spec__;
        // } catch  AttributeError  {
        panic!("ValueError ( "{}.__spec__ is !set" . format ( name ) ) from None /* Option */");
        } else {
        if spec is None /* Option */ {
        panic!("ValueError ( "{}.__spec__ is None /* Option */" . format ( name ) )");
        return  spec;
        @ contextmanager;
        pub fn _module_to_load ( name )  {
        is_reload = name in sys . modules;
        module = sys . modules . get ( name );
        if !is_reload {
        module = type ( sys ) ( name );
        module . __initializing__ = true;
        sys . modules [ name ] = module;
        // try {
        yield module;
        // } catch  Exception  {
        if !is_reload {
        // try {
        del sys . modules [ name ];
        // } catch  KeyError  {
        // pass
        // } finally {
        module . __initializing__ = false;
        pub fn set_package ( fxn )  {
        "Set __package__ on the returned module.

    This function == deprecated.

    ";
        @ functools . wraps ( fxn );
        pub fn set_package_wrapper ( * args , ** kwargs )  {
        warnings . warn ( "The import system now takes care of this automatically; ";
        "this decorator == slated for removal in Python 3.12" ,;
        DeprecationWarning , stacklevel = 2 );
        module = fxn ( * args , ** kwargs );
        if getattr ( module , "__package__" , None /* Option */ ) is None /* Option */ {
        module . __package__ = module . __name__;
        if !hasattr ( module , "__path__" ) {
        module . __package__ = module . __package__ . rpartition ( "." ) [ 0 ];
        return  module;
        return  set_package_wrapper;
        pub fn set_loader ( fxn )  {
        "Set __loader__ on the returned module.

    This function == deprecated.

    ";
        @ functools . wraps ( fxn );
        pub fn set_loader_wrapper ( &self, * args , ** kwargs )  {
        warnings . warn ( "The import system now takes care of this automatically; ";
        "this decorator == slated for removal in Python 3.12" ,;
        DeprecationWarning , stacklevel = 2 );
        module = fxn ( self , * args , ** kwargs );
        if getattr ( module , "__loader__" , None /* Option */ ) is None /* Option */ {
        module . __loader__ = self;
        return  module;
        return  set_loader_wrapper;
        pub fn module_for_loader ( fxn )  {
        "Decorator to handle selecting the proper module for loaders.

    The decorated function == passed the module to use instead of the module
    name. The module passed in to the function == either from sys.modules if
    it already exists || == a new module. If the module == new, then __name__
    == set the first argument to the method, __loader__ == set to self, and
    __package__ == set accordingly (if self.is_package() == defined) will be set
    before it == passed to the decorated function (if self.is_package() does
    !work for the module it will be set post-load).

    If an exception == raised && the decorator created the module it is
    subsequently removed from sys.modules.

    The decorator assumes that the decorated function takes the module name as
    the second argument.

    ";
        warnings . warn ( "The import system now takes care of this automatically; ";
        "this decorator == slated for removal in Python 3.12" ,;
        DeprecationWarning , stacklevel = 2 );
        @ functools . wraps ( fxn );
        pub fn module_for_loader_wrapper ( &self, fullname , * args , ** kwargs )  {
        // with scope: _module_to_load ( fullname ) as module  {
        module . __loader__ = self;
        // try {
        is_package = self . is_package ( fullname );
        // } catch  ( ImportError , AttributeError )  {
        // pass
        } else {
        if is_package {
        module . __package__ = fullname;
        } else {
        module . __package__ = fullname . rpartition ( "." ) [ 0 ];
        return  fxn ( self , module , * args , ** kwargs );
        return  module_for_loader_wrapper;
        class _LazyModule ( types . ModuleType ) ;
        "A subclass of the module type which triggers loading upon attribute access.";
        pub fn __getattribute__ ( &self, attr )  {
        "Trigger the load of the module && return the attribute.";
        __spec__ = object . __getattribute__ ( self , "__spec__" );
        loader_state = __spec__ . loader_state;
        // with scope: loader_state [ "lock" ]  {
        if object . __getattribute__ ( self , "__class__" ) is _LazyModule {
        if loader_state [ "is_loading" ] {
        return  object . __getattribute__ ( self , attr );
        loader_state [ "is_loading" ] = true;
        __dict__ = object . __getattribute__ ( self , "__dict__" );
        original_name = __spec__ . name;
        attrs_then = loader_state [ "__dict__" ];
        attrs_now = __dict__;
        attrs_updated = { };
        for key , value in attrs_now . items ( ) .iter() {
        if key !in attrs_then {
        attrs_updated [ key ] = value;
        } else if id ( attrs_now [ key ] ) != id ( attrs_then [ key ] ) {
        attrs_updated [ key ] = value;
        __spec__ . loader . exec_module ( self );
        if original_name in sys . modules {
        if id ( self ) != id ( sys . modules [ original_name ] ) {
        panic!("ValueError ( f "module object for {original_name!r} "");
        "substituted in sys.modules during a lazy ";
        "load" );
        __dict__ . update ( attrs_updated );
        self . __class__ = types . ModuleType;
        return  getattr ( self , attr );
        pub fn __delattr__ ( &self, attr )  {
        "Trigger the load && then perform the deletion.";
        self . __getattribute__ ( attr );
        delattr ( self , attr );
        class LazyLoader ( Loader ) ;
        "A loader that creates a module which defers loading until attribute access.";
        @ staticmethod;
        pub fn __check_eager_loader ( loader )  {
        if !hasattr ( loader , "exec_module" ) {
        panic!("TypeError ( "loader must define exec_module()" )");
        @ classmethod;
        pub fn factory ( cls , loader )  {
        "Construct a callable which returns the eager loader made lazy.";
        cls . __check_eager_loader ( loader );
        return  lambda * args , ** kwargs : cls ( loader ( * args , ** kwargs ) );
        pub fn __init__ ( &self, loader )  {
        self . __check_eager_loader ( loader );
        self . loader = loader;
        pub fn create_module ( &self, spec )  {
        return  self . loader . create_module ( spec );
        pub fn exec_module ( &self, module )  {
        "Make the module load lazily.";
        module . __spec__ . loader = self . loader;
        module . __loader__ = self . loader;
        loader_state = { };
        loader_state [ "__dict__" ] = module . __dict__ . copy ( );
        loader_state [ "__class__" ] = module . __class__;
        loader_state [ "lock" ] = threading . RLock ( );
        loader_state [ "is_loading" ] = false;
        module . __spec__ . loader_state = loader_state;
        module . __class__ = _LazyModule;
}

