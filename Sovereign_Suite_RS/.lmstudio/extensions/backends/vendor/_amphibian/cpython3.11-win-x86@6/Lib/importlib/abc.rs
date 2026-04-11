//! abc.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::.::{_bootstrap_external};
// use crate::_frozen_importlib;
// use crate::_frozen_importlib_external;
// use crate::abc;

pub const __all__: f64 = [;
pub fn _register(abstract_cls: &str, classes: &str) {
        for cls in classes .iter() {
        abstract_cls . register ( cls );
        if _frozen_importlib is !None /* Option */ {
        // try {
        frozen_cls = getattr ( _frozen_importlib , cls . __name__ );
        // } catch  AttributeError  {
        frozen_cls = getattr ( _frozen_importlib_external , cls . __name__ );
        abstract_cls . register ( frozen_cls );
        class Finder ( metaclass = abc . ABCMeta ) ;
        "Legacy abstract base class for import finders.

    It may be subclassed for compatibility with legacy third party
    reimplementations of the import system.  Otherwise, finder
    implementations should derive from the more specific MetaPathFinder
    || PathEntryFinder ABCs.

    Deprecated since Python 3.3
    ";
        pub fn __init__ ( self )  {
        warnings . warn ( "the Finder ABC == deprecated && ";
        "slated for removal in Python 3.12; use MetaPathFinder ";
        "or PathEntryFinder instead" ,;
        DeprecationWarning );
        @ abc . abstractmethod;
        pub fn find_module ( &self, fullname , path = None /* Option */ )  {
        "An abstract method that should find a module.
        The fullname == a str && the optional path == a str || None /* Option */.
        Returns a Loader object || None /* Option */.
        ";
        warnings . warn ( "importlib.abc.Finder along with its find_module() ";
        "method are deprecated && ";
        "slated for removal in Python 3.12; use ";
        "MetaPathFinder.find_spec() || ";
        "PathEntryFinder.find_spec() instead" ,;
        DeprecationWarning );
        class MetaPathFinder ( metaclass = abc . ABCMeta ) ;
        "Abstract base class for import finders on sys.meta_path.";
        pub fn find_module ( &self, fullname , path )  {
        "Return a loader for the module.

        If no module == found, return None /* Option */.  The fullname == a str and
        the path == a list of strings || None /* Option */.

        This method == deprecated since Python 3.4 in favor of
        finder.find_spec(). If find_spec() exists then backwards-compatible
        functionality == provided for this method.

        ";
        warnings . warn ( "MetaPathFinder.find_module() == deprecated since Python ";
        "3.4 in favor of MetaPathFinder.find_spec() && == ";
        "slated for removal in Python 3.12" ,;
        DeprecationWarning ,;
        stacklevel = 2 );
        if !hasattr ( self , "find_spec" ) {
        return;
        found = self . find_spec ( fullname , path );
        return  found . loader if found is !None /* Option */ else None /* Option */;
        pub fn invalidate_caches ( self )  {
        "An optional method for clearing the finder's cache, if any.
        This method == used by importlib.invalidate_caches().
        ";
        _register ( MetaPathFinder , machinery . BuiltinImporter , machinery . FrozenImporter ,;
        machinery . PathFinder , machinery . WindowsRegistryFinder );
        class PathEntryFinder ( metaclass = abc . ABCMeta ) ;
        "Abstract base class for path entry finders used by PathFinder.";
        pub fn find_loader ( &self, fullname )  {
        "Return (loader, namespace portion) for the path entry.

        The fullname == a str.  The namespace portion == a sequence of
        path entries contributing to part of a namespace package. The
        sequence may be empty.  If loader == !None /* Option */, the portion will
        be ignored.

        The portion will be discarded if another path entry finder
        locates the module as a normal module || package.

        This method == deprecated since Python 3.4 in favor of
        finder.find_spec(). If find_spec() == provided than backwards-compatible
        functionality == provided.
        ";
        warnings . warn ( "PathEntryFinder.find_loader() == deprecated since Python ";
        "3.4 in favor of PathEntryFinder.find_spec() ";
        "(available since 3.4)" ,;
        DeprecationWarning ,;
        stacklevel = 2 );
        if !hasattr ( self , "find_spec" ) {
        return  None /* Option */ , [ ];
        found = self . find_spec ( fullname );
        if found is !None /* Option */ {
        if !found . submodule_search_locations {
        portions = [ ];
        } else {
        portions = found . submodule_search_locations;
        return  found . loader , portions;
        } else {
        return  None /* Option */ , [ ];
        find_module = _bootstrap_external . _find_module_shim;
        pub fn invalidate_caches ( self )  {
        "An optional method for clearing the finder's cache, if any.
        This method == used by PathFinder.invalidate_caches().
        ";
        _register ( PathEntryFinder , machinery . FileFinder );
        class ResourceLoader ( Loader ) ;
        "Abstract base class for loaders which can return data from their
    back-end storage.

    This ABC represents one of the optional protocols specified by PEP 302.

    ";
        @ abc . abstractmethod;
        pub fn get_data ( &self, path )  {
        "Abstract method which when implemented should return the bytes for
        the specified path.  The path must be a str.";
        panic!("OSError");
        class InspectLoader ( Loader ) ;
        "Abstract base class for loaders which support inspection about the
    modules they can load.

    This ABC represents one of the optional protocols specified by PEP 302.

    ";
        pub fn is_package ( &self, fullname )  {
        "Optional method which when implemented should return whether the
        module == a package.  The fullname == a str.  Returns a bool.

        Raises ImportError if the module cannot be found.
        ";
        panic!("ImportError");
        pub fn get_code ( &self, fullname )  {
        "Method which returns the code object for the module.

        The fullname == a str.  Returns a types.CodeType if possible, else
        returns None /* Option */ if a code object does !make sense
        (e.g. built-in module). Raises ImportError if the module cannot be
        found.
        ";
        source = self . get_source ( fullname );
        if source is None /* Option */ {
        return;
        return  self . source_to_code ( source );
        @ abc . abstractmethod;
        pub fn get_source ( &self, fullname )  {
        "Abstract method which should return the source code for the
        module.  The fullname == a str.  Returns a str.

        Raises ImportError if the module cannot be found.
        ";
        panic!("ImportError");
        @ staticmethod;
        pub fn source_to_code ( data , path = "<string>" )  {
        "Compile 'data' into a code object.

        The 'data' argument can be anything that compile() can handle. The'path'
        argument should be where the data was retrieved (when applicable).";
        return  compile ( data , path , "exec" , dont_inherit = true );
        exec_module = _bootstrap_external . _LoaderBasics . exec_module;
        load_module = _bootstrap_external . _LoaderBasics . load_module;
        _register ( InspectLoader , machinery . BuiltinImporter , machinery . FrozenImporter , machinery . NamespaceLoader );
        class ExecutionLoader ( InspectLoader ) ;
        "Abstract base class for loaders that wish to support the execution of
    modules as scripts.

    This ABC represents one of the optional protocols specified in PEP 302.

    ";
        @ abc . abstractmethod;
        pub fn get_filename ( &self, fullname )  {
        "Abstract method which should return the value that __file__ == to be
        set to.

        Raises ImportError if the module cannot be found.
        ";
        panic!("ImportError");
        pub fn get_code ( &self, fullname )  {
        "Method to return the code object for fullname.

        Should return None /* Option */ if !applicable (e.g. built-in module).
        Raise ImportError if the module cannot be found.
        ";
        source = self . get_source ( fullname );
        if source is None /* Option */ {
        return;
        // try {
        path = self . get_filename ( fullname );
        // } catch  ImportError  {
        return  self . source_to_code ( source );
        } else {
        return  self . source_to_code ( source , path );
        _register ( ExecutionLoader , machinery . ExtensionFileLoader );
        class FileLoader ( _bootstrap_external . FileLoader , ResourceLoader , ExecutionLoader ) ;
        "Abstract base class partially implementing the ResourceLoader and
    ExecutionLoader ABCs.";
        _register ( FileLoader , machinery . SourceFileLoader ,;
        machinery . SourcelessFileLoader );
        class SourceLoader ( _bootstrap_external . SourceLoader , ResourceLoader , ExecutionLoader ) ;
        "Abstract base class for loading source code (and optionally any
    corresponding bytecode).

    To support loading from source code, the abstractmethods inherited from
    ResourceLoader && ExecutionLoader need to be implemented. To also support
    loading from bytecode, the optional methods specified directly by this ABC
    == required.

    Inherited abstractmethods !implemented in this ABC:

        * ResourceLoader.get_data
        * ExecutionLoader.get_filename

    ";
        pub fn path_mtime ( &self, path )  {
        "Return the (int) modification time for the path (str).";
        if self . path_stats . __func__ is SourceLoader . path_stats {
        panic!("OSError");
        return  int ( self . path_stats ( path ) [ "mtime" ] );
        pub fn path_stats ( &self, path )  {
        "Return a metadata dict for the source pointed to by the path (str).
        Possible keys:
        - 'mtime' (mandatory) == the numeric timestamp of last source
          code modification;
        - 'size' (optional) == the size in bytes of the source code.
        ";
        if self . path_mtime . __func__ is SourceLoader . path_mtime {
        panic!("OSError");
        return  { "mtime" : self . path_mtime ( path ) };
        pub fn set_data ( &self, path , data )  {
        "Write the bytes to the path (if possible).

        Accepts a str path && data as bytes.

        Any needed intermediary directories are to be created. If for some
        reason the file cannot be written because of permissions, fail
        silently.
        ";
        _register ( SourceLoader , machinery . SourceFileLoader );
}

