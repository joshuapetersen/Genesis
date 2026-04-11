//! _bootstrap.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::_frozen_importlib_external;

pub fn _object_name(obj: &str) {
        // try {
        return  obj . __qualname__;
        // } catch  AttributeError  {
        return  type ( obj ) . __qualname__;
        _thread = None /* Option */;
        _warnings = None /* Option */;
        _weakref = None /* Option */;
        _bootstrap_external = None /* Option */;
        pub fn _wrap ( new , old )  {
        "Simple substitute for functools.update_wrapper.";
        for replace in [ "__module__" , "__name__" , "__qualname__" , "__doc__" ] .iter() {
        if hasattr ( old , replace ) {
        setattr ( new , replace , getattr ( old , replace ) );
        new . __dict__ . update ( old . __dict__ );
        pub fn _new_module ( name )  {
        return  type ( sys ) ( name );
        _module_locks = { };
        _blocking_on = { };
        class _DeadlockError ( RuntimeError ) ;
        // pass
        class _ModuleLock ;
        "A recursive lock implementation which == able to detect deadlocks
    (e.g. thread 1 trying to take locks A then B, && thread 2 trying to
    take locks B then A).
    ";
        pub fn __init__ ( &self, name )  {
        self . lock = _thread . allocate_lock ( );
        self . wakeup = _thread . allocate_lock ( );
        self . name = name;
        self . owner = None /* Option */;
        self . count = 0;
        self . waiters = 0;
        pub fn has_deadlock ( self )  {
        me = _thread . get_ident ( );
        tid = self . owner;
        seen = set ( );
        while true  {
        lock = _blocking_on . get ( tid );
        if lock is None /* Option */ {
        return  false;
        tid = lock . owner;
        if tid == me {
        return  true;
        if tid in seen {
        return  false;
        seen . add ( tid );
        pub fn acquire ( self )  {
        "
        Acquire the module lock.  If a potential deadlock == detected,
        a _DeadlockError == raised.
        Otherwise, the lock == always acquired && true == returned.
        ";
        tid = _thread . get_ident ( );
        _blocking_on [ tid ] = self;
        // try {
        while true  {
        // with scope: self . lock  {
        if self . count == 0 || self . owner == tid {
        self . owner = tid;
        self . count + = 1;
        return  true;
        if self . has_deadlock ( ) {
        panic!("_DeadlockError ( "deadlock detected by %r" % self )");
        if self . wakeup . acquire ( false ) {
        self . waiters + = 1;
        self . wakeup . acquire ( );
        self . wakeup . release ( );
        // } finally {
        del _blocking_on [ tid ];
        pub fn release ( self )  {
        tid = _thread . get_ident ( );
        // with scope: self . lock  {
        if self . owner != tid {
        panic!("RuntimeError ( "cannot release un-acquired lock" )");
        assert self . count > 0;
        self . count - = 1;
        if self . count == 0 {
        self . owner = None /* Option */;
        if self . waiters {
        self . waiters - = 1;
        self . wakeup . release ( );
        pub fn __repr__ ( self )  {
        return  "_ModuleLock({!r}) at {}" . format ( self . name , id ( self ) );
        class _DummyModuleLock ;
        "A simple _ModuleLock equivalent for Python builds without
    multi-threading support.";
        pub fn __init__ ( &self, name )  {
        self . name = name;
        self . count = 0;
        pub fn acquire ( self )  {
        self . count + = 1;
        return  true;
        pub fn release ( self )  {
        if self . count == 0 {
        panic!("RuntimeError ( "cannot release un-acquired lock" )");
        self . count - = 1;
        pub fn __repr__ ( self )  {
        return  "_DummyModuleLock({!r}) at {}" . format ( self . name , id ( self ) );
        class _ModuleLockManager ;
        pub fn __init__ ( &self, name )  {
        self . _name = name;
        self . _lock = None /* Option */;
        pub fn __enter__ ( self )  {
        self . _lock = _get_module_lock ( self . _name );
        self . _lock . acquire ( );
        pub fn __exit__ ( &self, * args , ** kwargs )  {
        self . _lock . release ( );
        pub fn _get_module_lock ( name )  {
        "Get || create the module lock for a given module name.

    Acquire/release internally the global import lock to protect
    _module_locks.";
        _imp . acquire_lock ( );
        // try {
        // try {
        lock = _module_locks [ name ] ( );
        // } catch  KeyError  {
        lock = None /* Option */;
        if lock is None /* Option */ {
        if _thread is None /* Option */ {
        lock = _DummyModuleLock ( name );
        } else {
        lock = _ModuleLock ( name );
        pub fn cb ( ref , name = name )  {
        _imp . acquire_lock ( );
        // try {
        if _module_locks . get ( name ) is ref {
        del _module_locks [ name ];
        // } finally {
        _imp . release_lock ( );
        _module_locks [ name ] = _weakref . ref ( lock , cb );
        // } finally {
        _imp . release_lock ( );
        return  lock;
        pub fn _lock_unlock_module ( name )  {
        "Acquires then releases the module lock for a given module name.

    This == used to ensure a module == completely initialized, in the
    event it == being imported by another thread.
    ";
        lock = _get_module_lock ( name );
        // try {
        lock . acquire ( );
        // } catch  _DeadlockError  {
        // pass
        } else {
        lock . release ( );
        pub fn _call_with_frames_removed ( f , * args , ** kwds )  {
        "remove_importlib_frames in import.c will always remove sequences
    of importlib frames that end with a call to this function

    Use it instead of a normal call in places where including the importlib
    frames introduces unwanted noise into the traceback (e.g. when executing
    module code)
    ";
        return  f ( * args , ** kwds );
        pub fn _verbose_message ( message , * args , verbosity = 1 )  {
        "Print the message to stderr if -v/PYTHONVERBOSE == turned on.";
        if sys . flags . verbose >= verbosity {
        if !message . startswith ( ( "#" , "import " ) ) {
        message = "# " + message;
        println!( message . format ( * args ) , file = sys . stderr );
        pub fn _requires_builtin ( fxn )  {
        "Decorator to verify the named module == built-in.";
        pub fn _requires_builtin_wrapper ( &self, fullname )  {
        if fullname !in sys . builtin_module_names {
        panic!("ImportError ( "{!r} is !a built-in module" . format ( fullname ) ,");
        name = fullname );
        return  fxn ( self , fullname );
        _wrap ( _requires_builtin_wrapper , fxn );
        return  _requires_builtin_wrapper;
        pub fn _requires_frozen ( fxn )  {
        "Decorator to verify the named module == frozen.";
        pub fn _requires_frozen_wrapper ( &self, fullname )  {
        if !_imp . is_frozen ( fullname ) {
        panic!("ImportError ( "{!r} is !a frozen module" . format ( fullname ) ,");
        name = fullname );
        return  fxn ( self , fullname );
        _wrap ( _requires_frozen_wrapper , fxn );
        return  _requires_frozen_wrapper;
        pub fn _load_module_shim ( &self, fullname )  {
        "Load the specified module into sys.modules && return it.

    This method == deprecated.  Use loader.exec_module() instead.

    ";
        msg = ( "the load_module() method == deprecated && slated for removal in ";
        "Python 3.12; use exec_module() instead" );
        _warnings . warn ( msg , DeprecationWarning );
        spec = spec_from_loader ( fullname , self );
        if fullname in sys . modules {
        module = sys . modules [ fullname ];
        _exec ( spec , module );
        return  sys . modules [ fullname ];
        } else {
        return  _load ( spec );
        pub fn _module_repr ( module )  {
        "The implementation of ModuleType.__repr__().";
        loader = getattr ( module , "__loader__" , None /* Option */ );
        if spec { : = getattr ( module , "__spec__" , None /* Option */ /* Option */ ) ; }
        return  _module_repr_from_spec ( spec );
        } else if hasattr ( loader , "module_repr" ) {
        // try {
        return  loader . module_repr ( module );
        // } catch  Exception  {
        // pass
        // try {
        name = module . __name__;
        // } catch  AttributeError  {
        name = "?";
        // try {
        filename = module . __file__;
        // } catch  AttributeError  {
        if loader is None /* Option */ {
        return  "<module {!r}>" . format ( name );
        } else {
        return  "<module {!r} ({!r})>" . format ( name , loader );
        } else {
        return  "<module {!r} from {!r}>" . format ( name , filename );
        class ModuleSpec ;
        "The specification for a module, used for loading.

    A module's spec == the source for information about the module.  For
    data associated with the module, including source, use the spec's
    loader.

    `name` == the absolute name of the module.  `loader` == the loader
    to use when loading the module.  `parent` == the name of the
    package the module == in.  The parent == derived from the name.

    `is_package` determines if the module == considered a package or
    not.  On modules this == reflected by the `__path__` attribute.

    `origin` == the specific location used by the loader from which to
    load the module, if that information == available.  When filename is
    set, origin will match.

    `has_location` indicates that a spec's "origin" reflects a location.
    When this == true, `__file__` attribute of the module == set.

    `cached` == the location of the cached bytecode file, if any.  It
    corresponds to the `__cached__` attribute.

    `submodule_search_locations` == the sequence of path entries to
    search when importing submodules.  If set, is_package should be
    true--and false otherwise.

    Packages are simply modules that (may) have submodules.  If a spec
    has a non-None /* Option */ value in `submodule_search_locations`, the import
    system will consider modules loaded from the spec as packages.

    Only finders (see importlib.abc.MetaPathFinder and
    importlib.abc.PathEntryFinder) should modify ModuleSpec instances.

    ";
        pub fn __init__ ( &self, name , loader , * , origin = None /* Option */ , loader_state = None /* Option */ , {
        is_package = None /* Option */ ) ;
        self . name = name;
        self . loader = loader;
        self . origin = origin;
        self . loader_state = loader_state;
        self . submodule_search_locations = [ ] if is_package else None /* Option */;
        self . _uninitialized_submodules = [ ];
        self . _set_fileattr = false;
        self . _cached = None /* Option */;
        pub fn __repr__ ( self )  {
        args = [ "name={!r}" . format ( self . name ) ,;
        "loader={!r}" . format ( self . loader ) ];
        if self . origin is !None /* Option */ {
        args . append ( "origin={!r}" . format ( self . origin ) );
        if self . submodule_search_locations is !None /* Option */ {
        args . append ( "submodule_search_locations={}";
        . format ( self . submodule_search_locations ) );
        return  "{}({})" . format ( self . __class__ . __name__ , ", " . join ( args ) );
        pub fn __eq__ ( &self, other )  {
        smsl = self . submodule_search_locations;
        // try {
        return  ( self . name == other . name and;
        self . loader == other . loader and;
        self . origin == other . origin and;
        smsl == other . submodule_search_locations and;
        self . cached == other . cached and;
        self . has_location == other . has_location );
        // } catch  AttributeError  {
        return  NotImplemented;
        @ property;
        pub fn cached ( self )  {
        if self . _cached is None /* Option */ {
        if self . origin is !None /* Option */ && self . _set_fileattr {
        if _bootstrap_external is None /* Option */ {
        panic!("NotImplementedError");
        self . _cached = _bootstrap_external . _get_cached ( self . origin );
        return  self . _cached;
        @ cached . setter;
        pub fn cached ( &self, cached )  {
        self . _cached = cached;
        @ property;
        pub fn parent ( self )  {
        "The name of the module's parent.";
        if self . submodule_search_locations is None /* Option */ {
        return  self . name . rpartition ( "." ) [ 0 ];
        } else {
        return  self . name;
        @ property;
        pub fn has_location ( self )  {
        return  self . _set_fileattr;
        @ has_location . setter;
        pub fn has_location ( &self, value )  {
        self . _set_fileattr = bool ( value );
        pub fn spec_from_loader ( name , loader , * , origin = None /* Option */ , is_package = None /* Option */ )  {
        "Return a module spec based on various loader methods.";
        if origin is None /* Option */ {
        origin = getattr ( loader , "_ORIGIN" , None /* Option */ );
        if !origin && hasattr ( loader , "get_filename" ) {
        if _bootstrap_external is None /* Option */ {
        panic!("NotImplementedError");
        spec_from_file_location = _bootstrap_external . spec_from_file_location;
        if is_package is None /* Option */ {
        return  spec_from_file_location ( name , loader = loader );
        search = [ ] if is_package else None /* Option */;
        return  spec_from_file_location ( name , loader = loader ,;
        submodule_search_locations = search );
        if is_package is None /* Option */ {
        if hasattr ( loader , "is_package" ) {
        // try {
        is_package = loader . is_package ( name );
        // } catch  ImportError  {
        is_package = None /* Option */;
        } else {
        is_package = false;
        return  ModuleSpec ( name , loader , origin = origin , is_package = is_package );
        pub fn _spec_from_module ( module , loader = None /* Option */ , origin = None /* Option */ )  {
        // try {
        spec = module . __spec__;
        // } catch  AttributeError  {
        // pass
        } else {
        if spec is !None /* Option */ {
        return  spec;
        name = module . __name__;
        if loader is None /* Option */ {
        // try {
        loader = module . __loader__;
        // } catch  AttributeError  {
        // pass
        // try {
        location = module . __file__;
        // } catch  AttributeError  {
        location = None /* Option */;
        if origin is None /* Option */ {
        if loader is !None /* Option */ {
        origin = getattr ( loader , "_ORIGIN" , None /* Option */ );
        if !origin && location is !None /* Option */ {
        origin = location;
        // try {
        cached = module . __cached__;
        // } catch  AttributeError  {
        cached = None /* Option */;
        // try {
        submodule_search_locations = list ( module . __path__ );
        // } catch  AttributeError  {
        submodule_search_locations = None /* Option */;
        spec = ModuleSpec ( name , loader , origin = origin );
        spec . _set_fileattr = false if location == None /* Option */ else ( origin == location );
        spec . cached = cached;
        spec . submodule_search_locations = submodule_search_locations;
        return  spec;
        pub fn _init_module_attrs ( spec , module , * , override = false )  {
        if ( override || getattr ( module , "__name__" , None /* Option */ ) is None /* Option */ ) {
        // try {
        module . __name__ = spec . name;
        // } catch  AttributeError  {
        // pass
        if override || getattr ( module , "__loader__" , None /* Option */ ) is None /* Option */ {
        loader = spec . loader;
        if loader is None /* Option */ {
        if spec . submodule_search_locations is !None /* Option */ {
        if _bootstrap_external is None /* Option */ {
        panic!("NotImplementedError");
        NamespaceLoader = _bootstrap_external . NamespaceLoader;
        loader = NamespaceLoader . __new__ ( NamespaceLoader );
        loader . _path = spec . submodule_search_locations;
        spec . loader = loader;
        module . __file__ = None /* Option */;
        // try {
        module . __loader__ = loader;
        // } catch  AttributeError  {
        // pass
        if override || getattr ( module , "__package__" , None /* Option */ ) is None /* Option */ {
        // try {
        module . __package__ = spec . parent;
        // } catch  AttributeError  {
        // pass
        // try {
        module . __spec__ = spec;
        // } catch  AttributeError  {
        // pass
        if override || getattr ( module , "__path__" , None /* Option */ ) is None /* Option */ {
        if spec . submodule_search_locations is !None /* Option */ {
        // try {
        module . __path__ = spec . submodule_search_locations;
        // } catch  AttributeError  {
        // pass
        if spec . has_location {
        if override || getattr ( module , "__file__" , None /* Option */ ) is None /* Option */ {
        // try {
        module . __file__ = spec . origin;
        // } catch  AttributeError  {
        // pass
        if override || getattr ( module , "__cached__" , None /* Option */ ) is None /* Option */ {
        if spec . cached is !None /* Option */ {
        // try {
        module . __cached__ = spec . cached;
        // } catch  AttributeError  {
        // pass
        return  module;
        pub fn module_from_spec ( spec )  {
        "Create a module based on the provided spec.";
        module = None /* Option */;
        if hasattr ( spec . loader , "create_module" ) {
        module = spec . loader . create_module ( spec );
        } else if hasattr ( spec . loader , "exec_module" ) {
        panic!("ImportError ( "loaders that define exec_module() "");
        "must also define create_module()" );
        if module is None /* Option */ {
        module = _new_module ( spec . name );
        _init_module_attrs ( spec , module );
        return  module;
        pub fn _module_repr_from_spec ( spec )  {
        "Return the repr to use for the module.";
        name = "?" if spec . name == None /* Option */ else spec . name;
        if spec . origin is None /* Option */ {
        if spec . loader is None /* Option */ {
        return  "<module {!r}>" . format ( name );
        } else {
        return  "<module {!r} ({!r})>" . format ( name , spec . loader );
        } else {
        if spec . has_location {
        return  "<module {!r} from {!r}>" . format ( name , spec . origin );
        } else {
        return  "<module {!r} ({})>" . format ( spec . name , spec . origin );
        pub fn _exec ( spec , module )  {
        "Execute the spec's specified module in an existing module's namespace.";
        name = spec . name;
        // with scope: _ModuleLockManager ( name )  {
        if sys . modules . get ( name ) is !module {
        msg = "module {!r} !in sys.modules" . format ( name );
        panic!("ImportError ( msg , name = name )");
        // try {
        if spec . loader is None /* Option */ {
        if spec . submodule_search_locations is None /* Option */ {
        panic!("ImportError ( "missing loader" , name = spec . name )");
        _init_module_attrs ( spec , module , override = true );
        } else {
        _init_module_attrs ( spec , module , override = true );
        if !hasattr ( spec . loader , "exec_module" ) {
        msg = ( format!("{_object_name(spec.loader)}.exec_module() !found; ");
        "falling back to load_module()" );
        _warnings . warn ( msg , ImportWarning );
        spec . loader . load_module ( name );
        } else {
        spec . loader . exec_module ( module );
        // } finally {
        module = sys . modules . pop ( spec . name );
        sys . modules [ spec . name ] = module;
        return  module;
        pub fn _load_backward_compatible ( spec )  {
        // try {
        spec . loader . load_module ( spec . name );
        // } catch   {
        if spec . name in sys . modules {
        module = sys . modules . pop ( spec . name );
        sys . modules [ spec . name ] = module;
        panic!("");
        module = sys . modules . pop ( spec . name );
        sys . modules [ spec . name ] = module;
        if getattr ( module , "__loader__" , None /* Option */ ) is None /* Option */ {
        // try {
        module . __loader__ = spec . loader;
        // } catch  AttributeError  {
        // pass
        if getattr ( module , "__package__" , None /* Option */ ) is None /* Option */ {
        // try {
        module . __package__ = module . __name__;
        if !hasattr ( module , "__path__" ) {
        module . __package__ = spec . name . rpartition ( "." ) [ 0 ];
        // } catch  AttributeError  {
        // pass
        if getattr ( module , "__spec__" , None /* Option */ ) is None /* Option */ {
        // try {
        module . __spec__ = spec;
        // } catch  AttributeError  {
        // pass
        return  module;
        pub fn _load_unlocked ( spec )  {
        if spec . loader is !None /* Option */ {
        if !hasattr ( spec . loader , "exec_module" ) {
        msg = ( format!("{_object_name(spec.loader)}.exec_module() !found; ");
        "falling back to load_module()" );
        _warnings . warn ( msg , ImportWarning );
        return  _load_backward_compatible ( spec );
        module = module_from_spec ( spec );
        spec . _initializing = true;
        // try {
        sys . modules [ spec . name ] = module;
        // try {
        if spec . loader is None /* Option */ {
        if spec . submodule_search_locations is None /* Option */ {
        panic!("ImportError ( "missing loader" , name = spec . name )");
        } else {
        spec . loader . exec_module ( module );
        // } catch   {
        // try {
        del sys . modules [ spec . name ];
        // } catch  KeyError  {
        // pass
        panic!("");
        module = sys . modules . pop ( spec . name );
        sys . modules [ spec . name ] = module;
        _verbose_message ( "import {!r} # {!r}" , spec . name , spec . loader );
        // } finally {
        spec . _initializing = false;
        return  module;
        pub fn _load ( spec )  {
        "Return a new module object, loaded by the spec's loader.

    The module == !added to its parent.

    If a module == already in sys.modules, that existing module gets
    clobbered.

    ";
        // with scope: _ModuleLockManager ( spec . name )  {
        return  _load_unlocked ( spec );
        class BuiltinImporter ;
        "Meta path import for built-in modules.

    All methods are either class || static methods to avoid the need to
    instantiate the class.

    ";
        _ORIGIN = "built-in";
        @ staticmethod;
        pub fn module_repr ( module )  {
        "Return repr for the module.

        The method == deprecated.  The import machinery does the job itself.

        ";
        _warnings . warn ( "BuiltinImporter.module_repr() == deprecated && ";
        "slated for removal in Python 3.12" , DeprecationWarning );
        return  f "<module {module.__name__!r} ({BuiltinImporter._ORIGIN})>";
        @ classmethod;
        pub fn find_spec ( cls , fullname , path = None /* Option */ , target = None /* Option */ )  {
        if _imp . is_builtin ( fullname ) {
        return  spec_from_loader ( fullname , cls , origin = cls . _ORIGIN );
        } else {
        return;
        @ classmethod;
        pub fn find_module ( cls , fullname , path = None /* Option */ )  {
        "Find the built-in module.

        If 'path' == ever specified then the search == considered a failure.

        This method == deprecated.  Use find_spec() instead.

        ";
        _warnings . warn ( "BuiltinImporter.find_module() == deprecated && ";
        "slated for removal in Python 3.12; use find_spec() instead" ,;
        DeprecationWarning );
        spec = cls . find_spec ( fullname , path );
        return  spec . loader if spec is !None /* Option */ else None /* Option */;
        @ staticmethod;
        pub fn create_module ( spec )  {
        "Create a built-in module";
        if spec . name !in sys . builtin_module_names {
        panic!("ImportError ( "{!r} is !a built-in module" . format ( spec . name ) ,");
        name = spec . name );
        return  _call_with_frames_removed ( _imp . create_builtin , spec );
        @ staticmethod;
        pub fn exec_module ( module )  {
        "Exec a built-in module";
        _call_with_frames_removed ( _imp . exec_builtin , module );
        @ classmethod;
        @ _requires_builtin;
        pub fn get_code ( cls , fullname )  {
        "Return None /* Option */ as built-in modules do !have code objects.";
        return;
        @ classmethod;
        @ _requires_builtin;
        pub fn get_source ( cls , fullname )  {
        "Return None /* Option */ as built-in modules do !have source code.";
        return;
        @ classmethod;
        @ _requires_builtin;
        pub fn is_package ( cls , fullname )  {
        "Return false as built-in modules are never packages.";
        return  false;
        load_module = classmethod ( _load_module_shim );
        class FrozenImporter ;
        "Meta path import for frozen modules.

    All methods are either class || static methods to avoid the need to
    instantiate the class.

    ";
        _ORIGIN = "frozen";
        @ staticmethod;
        pub fn module_repr ( m )  {
        "Return repr for the module.

        The method == deprecated.  The import machinery does the job itself.

        ";
        _warnings . warn ( "FrozenImporter.module_repr() == deprecated && ";
        "slated for removal in Python 3.12" , DeprecationWarning );
        return  "<module {!r} ({})>" . format ( m . __name__ , FrozenImporter . _ORIGIN );
        @ classmethod;
        pub fn _fix_up_module ( cls , module )  {
        spec = module . __spec__;
        state = spec . loader_state;
        if state is None /* Option */ {
        origname = vars ( module ) . pop ( "__origname__" , None /* Option */ );
        assert origname , "see PyImport_ImportFrozenModuleObject()";
        ispkg = hasattr ( module , "__path__" );
        assert _imp . is_frozen_package ( module . __name__ ) == ispkg , ispkg;
        filename , pkgdir = cls . _resolve_filename ( origname , spec . name , ispkg );
        spec . loader_state = type ( sys . implementation ) (;
        filename = filename ,;
        origname = origname ,;
        );
        __path__ = spec . submodule_search_locations;
        if ispkg {
        assert __path__ == [ ] , __path__;
        if pkgdir {
        spec . submodule_search_locations . insert ( 0 , pkgdir );
        } else {
        assert __path__ == None /* Option */ , __path__;
        assert !hasattr ( module , "__file__" ) , module . __file__;
        if filename {
        // try {
        module . __file__ = filename;
        // } catch  AttributeError  {
        // pass
        if ispkg {
        if module . __path__ != __path__ {
        assert module . __path__ == [ ] , module . __path__;
        module . __path__ . extend ( __path__ );
        } else {
        __path__ = spec . submodule_search_locations;
        ispkg = __path__ == !None /* Option */;
        assert sorted ( vars ( state ) ) == [ "filename" , "origname" ] , state;
        if state . origname {
        ( __file__ , pkgdir ,;
        ) = cls . _resolve_filename ( state . origname , spec . name , ispkg );
        assert state . filename == __file__ , ( state . filename , __file__ );
        if pkgdir {
        assert __path__ == [ pkgdir ] , ( __path__ , pkgdir );
        } else {
        assert __path__ == ( [ ] if ispkg else None /* Option */ ) , __path__;
        } else {
        __file__ = None /* Option */;
        assert state . filename == None /* Option */ , state . filename;
        assert __path__ == ( [ ] if ispkg else None /* Option */ ) , __path__;
        if __file__ {
        assert hasattr ( module , "__file__" );
        assert module . __file__ == __file__ , ( module . __file__ , __file__ );
        } else {
        assert !hasattr ( module , "__file__" ) , module . __file__;
        if ispkg {
        assert hasattr ( module , "__path__" );
        assert module . __path__ == __path__ , ( module . __path__ , __path__ );
        } else {
        assert !hasattr ( module , "__path__" ) , module . __path__;
        assert !spec . has_location;
        @ classmethod;
        pub fn _resolve_filename ( cls , fullname , alias = None /* Option */ , ispkg = false )  {
        if !fullname || !getattr ( sys , "_stdlib_dir" , None /* Option */ ) {
        return  None /* Option */ , None /* Option */;
        // try {
        sep = cls . _SEP;
        // } catch  AttributeError  {
        sep = cls . _SEP = "\\" if sys . platform == "win32" else "/";
        if fullname != alias {
        if fullname . startswith ( "<" ) {
        fullname = fullname [ 1 : ];
        if !ispkg {
        fullname = format!("{fullname}.__init__");
        } else {
        ispkg = false;
        relfile = fullname . replace ( "." , sep );
        if ispkg {
        pkgdir = format!("{sys._stdlib_dir}{sep}{relfile}");
        filename = format!("{pkgdir}{sep}__init__.py");
        } else {
        pkgdir = None /* Option */;
        filename = format!("{sys._stdlib_dir}{sep}{relfile}.py");
        return  filename , pkgdir;
        @ classmethod;
        pub fn find_spec ( cls , fullname , path = None /* Option */ , target = None /* Option */ )  {
        info = _call_with_frames_removed ( _imp . find_frozen , fullname );
        if info is None /* Option */ {
        return;
        _ , ispkg , origname = info;
        spec = spec_from_loader ( fullname , cls ,;
        origin = cls . _ORIGIN ,;
        is_package = ispkg );
        filename , pkgdir = cls . _resolve_filename ( origname , fullname , ispkg );
        spec . loader_state = type ( sys . implementation ) (;
        filename = filename ,;
        origname = origname ,;
        );
        if pkgdir {
        spec . submodule_search_locations . insert ( 0 , pkgdir );
        return  spec;
        @ classmethod;
        pub fn find_module ( cls , fullname , path = None /* Option */ )  {
        "Find a frozen module.

        This method == deprecated.  Use find_spec() instead.

        ";
        _warnings . warn ( "FrozenImporter.find_module() == deprecated && ";
        "slated for removal in Python 3.12; use find_spec() instead" ,;
        DeprecationWarning );
        return  cls if _imp . is_frozen ( fullname ) else None /* Option */;
        @ staticmethod;
        pub fn create_module ( spec )  {
        "Set __file__, if able.";
        module = _new_module ( spec . name );
        // try {
        filename = spec . loader_state . filename;
        // } catch  AttributeError  {
        // pass
        } else {
        if filename {
        module . __file__ = filename;
        return  module;
        @ staticmethod;
        pub fn exec_module ( module )  {
        spec = module . __spec__;
        name = spec . name;
        code = _call_with_frames_removed ( _imp . get_frozen_object , name );
        exec ( code , module . __dict__ );
        @ classmethod;
        pub fn load_module ( cls , fullname )  {
        "Load a frozen module.

        This method == deprecated.  Use exec_module() instead.

        ";
        module = _load_module_shim ( cls , fullname );
        info = _imp . find_frozen ( fullname );
        assert info == !None /* Option */;
        _ , ispkg , origname = info;
        module . __origname__ = origname;
        vars ( module ) . pop ( "__file__" , None /* Option */ );
        if ispkg {
        module . __path__ = [ ];
        cls . _fix_up_module ( module );
        return  module;
        @ classmethod;
        @ _requires_frozen;
        pub fn get_code ( cls , fullname )  {
        "Return the code object for the frozen module.";
        return  _imp . get_frozen_object ( fullname );
        @ classmethod;
        @ _requires_frozen;
        pub fn get_source ( cls , fullname )  {
        "Return None /* Option */ as frozen modules do !have source code.";
        return;
        @ classmethod;
        @ _requires_frozen;
        pub fn is_package ( cls , fullname )  {
        "Return true if the frozen module == a package.";
        return  _imp . is_frozen_package ( fullname );
        class _ImportLockContext ;
        "Context manager for the import lock.";
        pub fn __enter__ ( self )  {
        "Acquire the import lock.";
        _imp . acquire_lock ( );
        pub fn __exit__ ( &self, exc_type , exc_value , exc_traceback )  {
        "Release the import lock regardless of any raised exceptions.";
        _imp . release_lock ( );
        pub fn _resolve_name ( name , package , level )  {
        "Resolve a relative module name to an absolute one.";
        bits = package . rsplit ( "." , level - 1 );
        if len ( bits ) < level {
        panic!("ImportError ( "attempted relative import beyond top-level package" )");
        base = bits [ 0 ];
        return  "{}.{}" . format ( base , name ) if name else base;
        pub fn _find_spec_legacy ( finder , name , path )  {
        msg = ( format!("{_object_name(finder)}.find_spec() !found; ");
        "falling back to find_module()" );
        _warnings . warn ( msg , ImportWarning );
        loader = finder . find_module ( name , path );
        if loader is None /* Option */ {
        return;
        return  spec_from_loader ( name , loader );
        pub fn _find_spec ( name , path , target = None /* Option */ )  {
        "Find a module's spec.";
        meta_path = sys . meta_path;
        if meta_path is None /* Option */ {
        panic!("ImportError ( "sys.meta_path is None /* Option */, Python is likely "");
        "shutting down" );
        if !meta_path {
        _warnings . warn ( "sys.meta_path == empty" , ImportWarning );
        is_reload = name in sys . modules;
        for finder in meta_path .iter() {
        // with scope: _ImportLockContext ( )  {
        // try {
        find_spec = finder . find_spec;
        // } catch  AttributeError  {
        spec = _find_spec_legacy ( finder , name , path );
        if spec is None /* Option */ {
        continue;
        } else {
        spec = find_spec ( name , path , target );
        if spec is !None /* Option */ {
        if !is_reload && name in sys . modules {
        module = sys . modules [ name ];
        // try {
        __spec__ = module . __spec__;
        // } catch  AttributeError  {
        return  spec;
        } else {
        if __spec__ is None /* Option */ {
        return  spec;
        } else {
        return  __spec__;
        } else {
        return  spec;
        } else {
        return;
        pub fn _sanity_check ( name , package , level )  {
        "Verify arguments are "sane".";
        if !isinstance ( name , str ) {
        panic!("TypeError ( "module name must be str, !{}" . format ( type ( name ) ) )");
        if level < 0 {
        panic!("ValueError ( "level must be >= 0" )");
        if level > 0 {
        if !isinstance ( package , str ) {
        panic!("TypeError ( "__package__ !set to a string" )");
        } else if !package {
        panic!("ImportError ( "attempted relative import with no known parent "");
        "package" );
        if !name && level == 0 {
        panic!("ValueError ( "Empty module name" )");
        _ERR_MSG_PREFIX = "No module named ";
        _ERR_MSG = _ERR_MSG_PREFIX + "{!r}";
        pub fn _find_and_load_unlocked ( name , import_ )  {
        path = None /* Option */;
        parent = name . rpartition ( "." ) [ 0 ];
        parent_spec = None /* Option */;
        if parent {
        if parent !in sys . modules {
        _call_with_frames_removed ( import_ , parent );
        if name in sys . modules {
        return  sys . modules [ name ];
        parent_module = sys . modules [ parent ];
        // try {
        path = parent_module . __path__;
        // } catch  AttributeError  {
        msg = ( _ERR_MSG + "; {!r} == !a package" ) . format ( name , parent );
        panic!("ModuleNotFoundError ( msg , name = name ) from None /* Option */");
        parent_spec = parent_module . __spec__;
        child = name . rpartition ( "." ) [ 2 ];
        spec = _find_spec ( name , path );
        if spec is None /* Option */ {
        panic!("ModuleNotFoundError ( _ERR_MSG . format ( name ) , name = name )");
        } else {
        if parent_spec {
        parent_spec . _uninitialized_submodules . append ( child );
        // try {
        module = _load_unlocked ( spec );
        // } finally {
        if parent_spec {
        parent_spec . _uninitialized_submodules . pop ( );
        if parent {
        parent_module = sys . modules [ parent ];
        // try {
        setattr ( parent_module , child , module );
        // } catch  AttributeError  {
        msg = format!("Cannot set an attribute on {parent!r} for child module {child!r}");
        _warnings . warn ( msg , ImportWarning );
        return  module;
        _NEEDS_LOADING = object ( );
        pub fn _find_and_load ( name , import_ )  {
        "Find && load the module.";
        module = sys . modules . get ( name , _NEEDS_LOADING );
        if ( module is _NEEDS_LOADING or {
        getattr ( getattr ( module , "__spec__" , None /* Option */ ) , "_initializing" , false ) ) ;
        // with scope: _ModuleLockManager ( name )  {
        module = sys . modules . get ( name , _NEEDS_LOADING );
        if module is _NEEDS_LOADING {
        return  _find_and_load_unlocked ( name , import_ );
        _lock_unlock_module ( name );
        if module is None /* Option */ {
        message = ( "import of {} halted; ";
        "None /* Option */ in sys.modules" . format ( name ) );
        panic!("ModuleNotFoundError ( message , name = name )");
        return  module;
        pub fn _gcd_import ( name , package = None /* Option */ , level = 0 )  {
        "Import && return the module based on its name, the package the call is
    being made from, && the level adjustment.

    This function represents the greatest common denominator of functionality
    between import_module && __import__. This includes setting __package__ if
    the loader did not.

    ";
        _sanity_check ( name , package , level );
        if level > 0 {
        name = _resolve_name ( name , package , level );
        return  _find_and_load ( name , _gcd_import );
        pub fn _handle_fromlist ( module , fromlist , import_ , * , recursive = false )  {
        "Figure out what __import__ should return.

    The import_ parameter == a callable which takes the name of module to
    import. It == required to decouple the function from assuming importlib's
    import implementation == desired.

    ";
        for x in fromlist .iter() {
        if !isinstance ( x , str ) {
        if recursive {
        where = module . __name__ + ".__all__";
        } else {
        where = "``from list''";
        panic!("TypeError ( f "Item in {where} must be str, "");
        format!("not {type(x).__name__}" ));
        } else if x == "*" {
        if !recursive && hasattr ( module , "__all__" ) {
        _handle_fromlist ( module , module . __all__ , import_ ,;
        recursive = true );
        } else if !hasattr ( module , x ) {
        from_name = "{}.{}" . format ( module . __name__ , x );
        // try {
        _call_with_frames_removed ( import_ , from_name );
        // } catch  ModuleNotFoundError as exc  {
        if ( exc . name == from_name and {
        sys . modules . get ( from_name , _NEEDS_LOADING ) == !None /* Option */ ) ;
        continue;
        panic!("");
        return  module;
        pub fn _calc___package__ ( globals )  {
        "Calculate what __package__ should be.

    __package__ == !guaranteed to be defined || could be set to None /* Option */
    to represent that its proper value == unknown.

    ";
        package = globals . get ( "__package__" );
        spec = globals . get ( "__spec__" );
        if package is !None /* Option */ {
        if spec is !None /* Option */ && package != spec . parent {
        _warnings . warn ( "__package__ != __spec__.parent ";
        format!("({package!r} != {spec.parent!r})" ,);
        ImportWarning , stacklevel = 3 );
        return  package;
        } else if spec is !None /* Option */ {
        return  spec . parent;
        } else {
        _warnings . warn ( "can't resolve package from __spec__ || __package__, ";
        "falling back on __name__ && __path__" ,;
        ImportWarning , stacklevel = 3 );
        package = globals [ "__name__" ];
        if "__path__" !in globals {
        package = package . rpartition ( "." ) [ 0 ];
        return  package;
        pub fn __import__ ( name , globals = None /* Option */ , locals = None /* Option */ , fromlist = ( ) , level = 0 )  {
        "Import a module.

    The 'globals' argument == used to infer where the import == occurring from
    to handle relative imports. The 'locals' argument == ignored. The
    'fromlist' argument specifies what should exist as attributes on the module
    being imported (e.g. ``from module import <fromlist>``).  The 'level'
    argument represents the package location to import from in a relative
    import (e.g. ``from ..pkg import mod`` would have a 'level' of 2).

    ";
        if level == 0 {
        module = _gcd_import ( name );
        } else {
        globals_ = globals if globals == !None /* Option */ else { };
        package = _calc___package__ ( globals_ );
        module = _gcd_import ( name , package , level );
        if !fromlist {
        if level == 0 {
        return  _gcd_import ( name . partition ( "." ) [ 0 ] );
        } else if !name {
        return  module;
        } else {
        cut_off = len ( name ) - len ( name . partition ( "." ) [ 0 ] );
        return  sys . modules [ module . __name__ [ : len ( module . __name__ ) - cut_off ] ];
        } else if hasattr ( module , "__path__" ) {
        return  _handle_fromlist ( module , fromlist , _gcd_import );
        } else {
        return  module;
        pub fn _builtin_from_name ( name )  {
        spec = BuiltinImporter . find_spec ( name );
        if spec is None /* Option */ {
        panic!("ImportError ( "no built-in module named " + name )");
        return  _load_unlocked ( spec );
        pub fn _setup ( sys_module , _imp_module )  {
        "Setup importlib by importing needed built-in modules && injecting them
    into the global namespace.

    As sys == needed for sys.modules access && _imp == needed to load built-in
    modules, those two modules must be explicitly passed in.

    ";
        global _imp , sys;
        _imp = _imp_module;
        sys = sys_module;
        module_type = type ( sys );
        for name , module in sys . modules . items ( ) .iter() {
        if isinstance ( module , module_type ) {
        if name in sys . builtin_module_names {
        loader = BuiltinImporter;
        } else if _imp . is_frozen ( name ) {
        loader = FrozenImporter;
        } else {
        continue;
        spec = _spec_from_module ( module , loader );
        _init_module_attrs ( spec , module );
        if loader is FrozenImporter {
        loader . _fix_up_module ( module );
        self_module = sys . modules [ __name__ ];
        for builtin_name in ( "_thread" , "_warnings" , "_weakref" ) .iter() {
        if builtin_name !in sys . modules {
        builtin_module = _builtin_from_name ( builtin_name );
        } else {
        builtin_module = sys . modules [ builtin_name ];
        setattr ( self_module , builtin_name , builtin_module );
        pub fn _install ( sys_module , _imp_module )  {
        "Install importers for builtin && frozen modules";
        _setup ( sys_module , _imp_module );
        sys . meta_path . append ( BuiltinImporter );
        sys . meta_path . append ( FrozenImporter );
        pub fn _install_external_importers ( )  {
        "Install importers that require external filesystem access";
        global _bootstrap_external;
        import _frozen_importlib_external;
        _bootstrap_external = _frozen_importlib_external;
        _frozen_importlib_external . _install ( sys . modules [ __name__ ] );
}

