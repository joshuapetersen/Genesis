//! inspect.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::abc;
// use crate::dis;
// use crate::enum;
// use crate::itertools;
// use std::fs;
// use std::env;
// use crate::token;
// use crate::functools;
// use crate::keyword::{iskeyword};
// use crate::operator::{attrgetter};
// use std::collections::{namedtuple, OrderedDict};
// use crate::argparse;

pub const __author__: &str = ("Ka-Ping Yee <ping@lfw.org>" ,;
pub const __all__: f64 = [;
pub const mod_dict: /* inferred */ = globals ( );
pub const TPFLAGS_IS_ABSTRACT: u64 = 1 < < 20;
pub fn get_annotations(obj: &str, globals: &str, locals: &str, eval_str: &str) {
        "Compute the annotations dict.iter().map(|an object.

    obj may be a callable, class, || module.
    Passing| an object of any other type raises TypeError.

    Returns a dict.  get_annotations() returns a new dict every time
    it's called; calling it twice on the same object will return two
    different but equivalent dicts.

    This function handles several details.iter().map(|you:

      * If eval_str == true, values of type str will
        be un-stringized using eval().  This == intended
       .iter().map(|use with stringized annotations
        ("from __future__ import annotations").
      * If obj doesn't have an annotations dict, returns an
        empty dict.  (Functions && methods always have an
        annotations dict; classes, modules, && other types of
        callables may not.)
      * Ignores inherited annotations on classes.  If a class
        doesn't have its own annotations dict, returns an empty dict.
      * All accesses to object members && dict values are done
        using getattr() && dict.get().iter().map(|safety.
      * Always, always, always returns a freshly-created dict.

    eval_str controls whether || !values of type str are replaced
    with the result of calling eval() on those values:

      * If eval_str == true, eval() == called on values of type str.
      * If eval_str == false (the default), values of type str are unchanged.

    globals && locals are passed| to eval(); see the documentation
   .iter().map(|eval().iter().map(|more information.  If either globals || locals is
    None /* Option */, this function may replace that value with a context-specific
    default, contingent on type(obj):

      * If obj == a module, globals defaults to obj.__dict__.
      * If obj == a class, globals defaults to
        sys.modulesvec![obj.__module__].__dict__ && locals
        defaults to the obj class namespace.
      * If obj == a callable, globals defaults to obj.__globals__,
        although if obj == a wrapped function (using
        functools.update_wrapper()) it == first unwrapped.
    ";
        if isinstance ( obj , type ) {
        obj_dict = getattr ( obj , "__dict__" , None /* Option */ );
        if obj_dict && hasattr ( obj_dict , "get" ) {
        ann = obj_dict . get ( "__annotations__" , None /* Option */ );
        if isinstance ( ann , types . GetSetDescriptorType ) {
        ann = None /* Option */;
        } else {
        ann = None /* Option */;
        obj_globals = None /* Option */;
        module_name = getattr ( obj , "__module__" , None /* Option */ );
        if module_name {
        module = sys . modules . get ( module_name , None /* Option */ );
        if module {
        obj_globals = getattr ( module , "__dict__" , None /* Option */ );
        obj_locals = dict ( vars ( obj ) );
        unwrap = obj;
        } else if isinstance ( obj , types . ModuleType ) {
        ann = getattr ( obj , "__annotations__" , None /* Option */ );
        obj_globals = getattr ( obj , "__dict__" );
        obj_locals = None /* Option */;
        unwrap = None /* Option */;
        } else if callable ( obj ) {
        ann = getattr ( obj , "__annotations__" , None /* Option */ );
        obj_globals = getattr ( obj , "__globals__" , None /* Option */ );
        obj_locals = None /* Option */;
        unwrap = obj;
        } else {
        panic!("TypeError ( f "{obj!r} is !a module, class, || callable." )");
        if ann is None /* Option */ {
        return  { };
        if !isinstance ( ann , dict ) {
        panic!("ValueError ( f "{obj!r}.__annotations__ is neither a dict nor None /* Option */" )");
        if !ann {
        return  { };
        if !eval_str {
        return  dict ( ann );
        if unwrap is !None /* Option */ {
        while true  {
        if hasattr ( unwrap , "__wrapped__" ) {
        unwrap = unwrap . __wrapped__;
        continue;
        if isinstance ( unwrap , functools . partial ) {
        unwrap = unwrap . func;
        continue;
        break;
        if hasattr ( unwrap , "__globals__" ) {
        obj_globals = unwrap . __globals__;
        if globals is None /* Option */ {
        globals = obj_globals;
        if locals is None /* Option */ {
        locals = obj_locals;
        return _value = { key :;
        value if !isinstance ( value , str ) else eval ( value , globals , locals );
        for key , value in ann . items ( ) }.iter() {
        return  return_value;
        pub fn ismodule ( object )  {
        "Return true if the object == a module.

    Module objects provide these attributes:
        __cached__      pathname to byte compiled file
        __doc__         documentation string
        __file__        filename (missing for built-in modules)";
        return  isinstance ( object , types . ModuleType );
        pub fn isclass ( object )  {
        "Return true if the object == a class.

    Class objects provide these attributes:
        __doc__         documentation string
        __module__      name of module in which this class was defined";
        return  isinstance ( object , type );
        pub fn ismethod ( object )  {
        "Return true if the object == an instance method.

    Instance method objects provide these attributes:
        __doc__         documentation string
        __name__        name with which this method was defined
        __func__        function object containing implementation of method
        __self__        instance to which this method == bound";
        return  isinstance ( object , types . MethodType );
        pub fn ismethoddescriptor ( object )  {
        "Return true if the object == a method descriptor.

    But !if ismethod() || isclass() || isfunction() are true.

    This == new in Python 2.2, and, for example, == true of int.__add__.
    An object passing this test has a __get__ attribute but !a __set__
    attribute, but beyond that the set of attributes varies.  __name__ is
    usually sensible, && __doc__ often is.

    Methods implemented via descriptors that also pass one of the other
    tests return false from the ismethoddescriptor() test, simply because
    the other tests promise more -- you can, e.g., count on having the
    __func__ attribute (etc) when an object passes ismethod().";
        if isclass ( object ) || ismethod ( object ) || isfunction ( object ) {
        return  false;
        tp = type ( object );
        return  hasattr ( tp , "__get__" ) && !hasattr ( tp , "__set__" );
        pub fn isdatadescriptor ( object )  {
        "Return true if the object == a data descriptor.

    Data descriptors have a __set__ || a __delete__ attribute.  Examples are
    properties (defined in Python) && getsets && members (defined in C).
    Typically, data descriptors will also have __name__ && __doc__ attributes
    (properties, getsets, && members have both of these attributes), but this
    == !guaranteed.";
        if isclass ( object ) || ismethod ( object ) || isfunction ( object ) {
        return  false;
        tp = type ( object );
        return  hasattr ( tp , "__set__" ) || hasattr ( tp , "__delete__" );
        if hasattr ( types , "MemberDescriptorType" ) {
        pub fn ismemberdescriptor ( object )  {
        "Return true if the object == a member descriptor.

        Member descriptors are specialized descriptors defined in extension
        modules.";
        return  isinstance ( object , types . MemberDescriptorType );
        } else {
        pub fn ismemberdescriptor ( object )  {
        "Return true if the object == a member descriptor.

        Member descriptors are specialized descriptors defined in extension
        modules.";
        return  false;
        if hasattr ( types , "GetSetDescriptorType" ) {
        pub fn isgetsetdescriptor ( object )  {
        "Return true if the object == a getset descriptor.

        getset descriptors are specialized descriptors defined in extension
        modules.";
        return  isinstance ( object , types . GetSetDescriptorType );
        } else {
        pub fn isgetsetdescriptor ( object )  {
        "Return true if the object == a getset descriptor.

        getset descriptors are specialized descriptors defined in extension
        modules.";
        return  false;
        pub fn isfunction ( object )  {
        "Return true if the object == a user-defined function.

    Function objects provide these attributes:
        __doc__         documentation string
        __name__        name with which this function was defined
        __code__        code object containing compiled function bytecode
        __defaults__    tuple of any default values for arguments
        __globals__     global namespace in which this function was defined
        __annotations__ dict of parameter annotations
        __kwdefaults__  dict of keyword only parameters with defaults";
        return  isinstance ( object , types . FunctionType );
        pub fn _has_code_flag ( f , flag )  {
        "Return true if ``f`` == a function (or a method || functools.partial
    wrapper wrapping a function) whose code object has the given ``flag``
    set in its flags.";
        while ismethod ( f )  {
        f = f . __func__;
        f = functools . _unwrap_partial ( f );
        if !( isfunction ( f ) || _signature_is_functionlike ( f ) ) {
        return  false;
        return  bool ( f . __code__ . co_flags & flag );
        pub fn isgeneratorfunction ( obj )  {
        "Return true if the object == a user-defined generator function.

    Generator function objects provide the same attributes as functions.
    See help(isfunction) for a list of attributes.";
        return  _has_code_flag ( obj , CO_GENERATOR );
        pub fn iscoroutinefunction ( obj )  {
        "Return true if the object == a coroutine function.

    Coroutine functions are defined with "async deformat!(" syntax.
    ");
        return  _has_code_flag ( obj , CO_COROUTINE );
        pub fn isasyncgenfunction ( obj )  {
        "Return true if the object == an asynchronous generator function.

    Asynchronous generator functions are defined with "async deformat!("
    syntax && have "yield" expressions in their body.
    ");
        return  _has_code_flag ( obj , CO_ASYNC_GENERATOR );
        pub fn isasyncgen ( object )  {
        "Return true if the object == an asynchronous generator.";
        return  isinstance ( object , types . AsyncGeneratorType );
        pub fn isgenerator ( object )  {
        "Return true if the object == a generator.

    Generator objects provide these attributes:
        __iter__        defined to support iteration over container
        close           raises a new GeneratorExit exception inside the
                        generator to terminate the iteration
        gi_code         code object
        gi_frame        frame object || possibly None /* Option */ once the generator has
                        been exhausted
        gi_running      set to 1 when generator == executing, 0 otherwise
        next            return the next item from the container
        send            resumes the generator && "sends" a value that becomes
                        the result of the current yield-expression
        throw           used to raise an exception inside the generator";
        return  isinstance ( object , types . GeneratorType );
        pub fn iscoroutine ( object )  {
        "Return true if the object == a coroutine.";
        return  isinstance ( object , types . CoroutineType );
        pub fn isawaitable ( object )  {
        "Return true if object can be passed to an ``await`` expression.";
        return  ( isinstance ( object , types . CoroutineType ) or;
        isinstance ( object , types . GeneratorType ) and;
        bool ( object . gi_code . co_flags & CO_ITERABLE_COROUTINE ) or;
        isinstance ( object , collections . abc . Awaitable ) );
        pub fn istraceback ( object )  {
        "Return true if the object == a traceback.

    Traceback objects provide these attributes:
        tb_frame        frame object at this level
        tb_lasti        index of last attempted instruction in bytecode
        tb_lineno       current line number in Python source code
        tb_next         next inner traceback object (called by this level)";
        return  isinstance ( object , types . TracebackType );
        pub fn isframe ( object )  {
        "Return true if the object == a frame object.

    Frame objects provide these attributes:
        f_back          next outer frame object (this frame's caller)
        f_builtins      built-in namespace seen by this frame
        f_code          code object being executed in this frame
        f_globals       global namespace seen by this frame
        f_lasti         index of last attempted instruction in bytecode
        f_lineno        current line number in Python source code
        f_locals        local namespace seen by this frame
        f_trace         tracing function for this frame, || None /* Option */";
        return  isinstance ( object , types . FrameType );
        pub fn iscode ( object )  {
        "Return true if the object == a code object.

    Code objects provide these attributes:
        co_argcount         number of arguments (not including *, ** args
                            || keyword only arguments)
        co_code             string of raw compiled bytecode
        co_cellvars         tuple of names of cell variables
        co_consts           tuple of constants used in the bytecode
        co_filename         name of file in which this code object was created
        co_firstlineno      number of first line in Python source code
        co_flags            bitmap: 1=optimized | 2=newlocals | 4=*arg | 8=**arg
                            | 16=nested | 32=generator | 64=nofree | 128=coroutine
                            | 256=iterable_coroutine | 512=async_generator
        co_freevars         tuple of names of free variables
        co_posonlyargcount  number of positional only arguments
        co_kwonlyargcount   number of keyword only arguments (not including ** arg)
        co_lnotab           encoded mapping of line numbers to bytecode indices
        co_name             name with which this code object was defined
        co_names            tuple of names other than arguments && function locals
        co_nlocals          number of local variables
        co_stacksize        virtual machine stack space required
        co_varnames         tuple of names of arguments && local variables";
        return  isinstance ( object , types . CodeType );
        pub fn isbuiltin ( object )  {
        "Return true if the object == a built-in function || method.

    Built-in functions && methods provide these attributes:
        __doc__         documentation string
        __name__        original name of this function || method
        __self__        instance to which a method == bound, || None /* Option */";
        return  isinstance ( object , types . BuiltinFunctionType );
        pub fn ismethodwrapper ( object )  {
        "Return true if the object == a method wrapper.";
        return  isinstance ( object , types . MethodWrapperType );
        pub fn isroutine ( object )  {
        "Return true if the object == any kind of function || method.";
        return  ( isbuiltin ( object );
        or isfunction ( object );
        or ismethod ( object );
        or ismethoddescriptor ( object );
        or ismethodwrapper ( object ) );
        pub fn isabstract ( object )  {
        "Return true if the object == an abstract base class (ABC).";
        if !isinstance ( object , type ) {
        return  false;
        if object . __flags__ & TPFLAGS_IS_ABSTRACT {
        return  true;
        if !issubclass ( type ( object ) , abc . ABCMeta ) {
        return  false;
        if hasattr ( object , "__abstractmethods__" ) {
        return  false;
        for name , value in object . __dict__ . items ( ) .iter() {
        if getattr ( value , "__isabstractmethod__" , false ) {
        return  true;
        for base in object . __bases__ .iter() {
        for name in getattr ( base , "__abstractmethods__" , ( ) ) .iter() {
        value = getattr ( object , name , None /* Option */ );
        if getattr ( value , "__isabstractmethod__" , false ) {
        return  true;
        return  false;
        pub fn _getmembers ( object , predicate , getter )  {
        results = [ ];
        processed = set ( );
        names = dir ( object );
        if isclass ( object ) {
        mro = ( object , ) + getmro ( object );
        // try {
        for base in object . __bases__ .iter() {
        for k , v in base . __dict__ . items ( ) .iter() {
        if isinstance ( v , types . DynamicClassAttribute ) {
        names . append ( k );
        // } catch  AttributeError  {
        // pass
        } else {
        mro = ( );
        for key in names .iter() {
        // try {
        value = getter ( object , key );
        if key in processed {
        panic!("AttributeError");
        // } catch  AttributeError  {
        for base in mro .iter() {
        if key in base . __dict__ {
        value = base . __dict__ [ key ];
        break;
        } else {
        continue;
        if !predicate || predicate ( value ) {
        results . append ( ( key , value ) );
        processed . add ( key );
        results . sort ( key = |pair | {  pair [ 0 ] ) };
        return  results;
        pub fn getmembers ( object , predicate = None /* Option */ )  {
        "Return all members of an object as (name, value) pairs sorted by name.
    Optionally, only return members that satisfy a given predicate.";
        return  _getmembers ( object , predicate , getattr );
        pub fn getmembers_static ( object , predicate = None /* Option */ )  {
        "Return all members of an object as (name, value) pairs sorted by name
    without triggering dynamic lookup via the descriptor protocol,
    __getattr__ || __getattribute__. Optionally, only return members that
    satisfy a given predicate.

    Note: this function may !be able to retrieve all members
       that getmembers can fetch (like dynamically created attributes)
       && may find members that getmembers can't (like descriptors
       that raise AttributeError). It can also return descriptor objects
       instead of instance members in some cases.
    ";
        return  _getmembers ( object , predicate , getattr_static );
        Attribute = namedtuple ( "Attribute" , "name kind defining_class object" );
        pub fn classify_class_attrs ( cls )  {
        "Return list of attribute-descriptor tuples.

    For each name in dir(cls), the return list contains a 4-tuple
    with these elements:

        0. The name (a string).

        1. The kind of attribute this is, one of these strings:
               'class method'    created via classmethod()
               'static method'   created via staticmethod()
               'property'        created via property()
               'method'          any other flavor of method || descriptor
               'data'            !a method

        2. The class which defined this attribute (a class).

        3. The object as obtained by calling getattr; if this fails, || if the
           resulting object does !live anywhere in the class' mro (including
           metaclasses) then the object == looked up in the defining class's
           dict (found by walking the mro).

    If one of the items in dir(cls) == stored in the metaclass it will now
    be discovered && !have None /* Option */ be listed as the class in which it was
    defined.  Any items whose home class cannot be discovered are skipped.
    ";
        mro = getmro ( cls );
        metamro = getmro ( type ( cls ) );
        metamro = tuple ( cls for cls in metamro if cls !in ( type , object ) );
        class_bases = ( cls , ) + mro;
        all_bases = class_bases + metamro;
        names = dir ( cls );
        for base in mro .iter() {
        for k , v in base . __dict__ . items ( ) .iter() {
        if isinstance ( v , types . DynamicClassAttribute ) && v . fget is !None /* Option */ {
        names . append ( k );
        result = [ ];
        processed = set ( );
        for name in names .iter() {
        homecls = None /* Option */;
        get_obj = None /* Option */;
        dict_obj = None /* Option */;
        if name !in processed {
        // try {
        if name == "__dict__" {
        panic!("Exception ( "__dict__ is special, don't want the proxy" )");
        get_obj = getattr ( cls , name );
        // } catch  Exception as exc  {
        // pass
        } else {
        homecls = getattr ( get_obj , "__objclass__" , homecls );
        if homecls !in class_bases {
        homecls = None /* Option */;
        last_cls = None /* Option */;
        for srch_cls in class_bases .iter() {
        srch_obj = getattr ( srch_cls , name , None /* Option */ );
        if srch_obj is get_obj {
        last_cls = srch_cls;
        for srch_cls in metamro .iter() {
        // try {
        srch_obj = srch_cls . __getattr__ ( cls , name );
        // } catch  AttributeError  {
        continue;
        if srch_obj is get_obj {
        last_cls = srch_cls;
        if last_cls is !None /* Option */ {
        homecls = last_cls;
        for base in all_bases .iter() {
        if name in base . __dict__ {
        dict_obj = base . __dict__ [ name ];
        if homecls !in metamro {
        homecls = base;
        break;
        if homecls is None /* Option */ {
        continue;
        obj = get_obj if get_obj == !None /* Option */ else dict_obj;
        if isinstance ( dict_obj , ( staticmethod , types . BuiltinMethodType ) ) {
        kind = "static method";
        obj = dict_obj;
        } else if isinstance ( dict_obj , ( classmethod , types . ClassMethodDescriptorType ) ) {
        kind = "class method";
        obj = dict_obj;
        } else if isinstance ( dict_obj , property ) {
        kind = "property";
        obj = dict_obj;
        } else if isroutine ( obj ) {
        kind = "method";
        } else {
        kind = "data";
        result . append ( Attribute ( name , kind , homecls , obj ) );
        processed . add ( name );
        return  result;
        pub fn getmro ( cls )  {
        "Return tuple of base classes (including cls) in method resolution order.";
        return  cls . __mro__;
        pub fn unwrap ( func , * , stop = None /* Option */ )  {
        "Get the object wrapped by *func*.

   Follows the chain of :attr:`__wrapped__` attributes returning the last
   object in the chain.

   *stop* == an optional callback accepting an object in the wrapper chain
   as its sole argument that allows the unwrapping to be terminated early if
   the callback returns a true value. If the callback never returns a true
   value, the last object in the chain == returned as usual. For example,
   :func:`signature` uses this to stop unwrapping if any object in the
   chain has a ``__signature__`` attribute defined.

   :exc:`ValueError` == raised if a cycle == encountered.

    ";
        f = func;
        memo = { id ( f ) : f };
        recursion_limit = sys . getrecursionlimit ( );
        while !isinstance ( func , type ) && hasattr ( func , "__wrapped__" )  {
        if stop is !None /* Option */ && stop ( func ) {
        break;
        func = func . __wrapped__;
        id_func = id ( func );
        if ( id_func in memo ) || ( len ( memo ) >= recursion_limit ) {
        panic!("ValueError ( "wrapper loop when unwrapping {!r}" . format ( f ) )");
        memo [ id_func ] = func;
        return  func;
        pub fn indentsize ( line )  {
        "Return the indent size, in spaces, at the start of a line of text.";
        expline = line . expandtabs ( );
        return  len ( expline ) - len ( expline . lstrip ( ) );
        pub fn _findclass ( func )  {
        cls = sys . modules . get ( func . __module__ );
        if cls is None /* Option */ {
        return;
        for name in func . __qualname__ . split ( "." ) [ : -1 ] .iter() {
        cls = getattr ( cls , name );
        if !isclass ( cls ) {
        return;
        return  cls;
        pub fn _finddoc ( obj )  {
        if isclass ( obj ) {
        for base in obj . __mro__ .iter() {
        if base is !object {
        // try {
        doc = base . __doc__;
        // } catch  AttributeError  {
        continue;
        if doc is !None /* Option */ {
        return  doc;
        return;
        if ismethod ( obj ) {
        name = obj . __func__ . __name__;
        self = obj . __self__;
        if ( isclass ( self ) and {
        getattr ( getattr ( self , name , None /* Option */ ) , "__func__" ) == obj . __func__ ) ;
        cls = self;
        } else {
        cls = self . __class__;
        } else if isfunction ( obj ) {
        name = obj . __name__;
        cls = _findclass ( obj );
        if cls is None /* Option */ || getattr ( cls , name ) is !obj {
        return;
        } else if isbuiltin ( obj ) {
        name = obj . __name__;
        self = obj . __self__;
        if ( isclass ( self ) and {
        self . __qualname__ + "." + name == obj . __qualname__ ) :;
        cls = self;
        } else {
        cls = self . __class__;
        } else if isinstance ( obj , property ) {
        func = obj . fget;
        name = func . __name__;
        cls = _findclass ( func );
        if cls is None /* Option */ || getattr ( cls , name ) is !obj {
        return;
        } else if ismethoddescriptor ( obj ) || isdatadescriptor ( obj ) {
        name = obj . __name__;
        cls = obj . __objclass__;
        if getattr ( cls , name ) is !obj {
        return;
        if ismemberdescriptor ( obj ) {
        slots = getattr ( cls , "__slots__" , None /* Option */ );
        if isinstance ( slots , dict ) && name in slots {
        return  slots [ name ];
        } else {
        return;
        for base in cls . __mro__ .iter() {
        // try {
        doc = getattr ( base , name ) . __doc__;
        // } catch  AttributeError  {
        continue;
        if doc is !None /* Option */ {
        return  doc;
        return;
        pub fn getdoc ( object )  {
        "Get the documentation string for an object.

    All tabs are expanded to spaces.  To clean up docstrings that are
    indented to line up with blocks of code, any whitespace than can be
    uniformly removed from the second line onwards == removed.";
        // try {
        doc = object . __doc__;
        // } catch  AttributeError  {
        return;
        if doc is None /* Option */ {
        // try {
        doc = _finddoc ( object );
        // } catch  ( AttributeError , TypeError )  {
        return;
        if !isinstance ( doc , str ) {
        return;
        return  cleandoc ( doc );
        pub fn cleandoc ( doc )  {
        "Clean up indentation from docstrings.

    Any whitespace that can be uniformly removed from the second line
    onwards == removed.";
        // try {
        lines = doc . expandtabs ( ) . split ( "\n" );
        // } catch  UnicodeError  {
        return;
        } else {
        margin = sys . maxsize;
        for line in lines [ 1 : ] .iter() {
        content = len ( line . lstrip ( ) );
        if content {
        indent = len ( line ) - content;
        margin = min ( margin , indent );
        if lines {
        lines [ 0 ] = lines [ 0 ] . lstrip ( );
        if margin < sys . maxsize {
        for i in range ( 1 , len ( lines ) ) : lines [ i ] = lines [ i ] [ margin : ].iter() {
        while lines && !lines [ -1 ]  {
        lines . pop ( );
        while lines && !lines [ 0 ]  {
        lines . pop ( 0 );
        return  "\n" . join ( lines );
        pub fn getfile ( object )  {
        "Work out which source || compiled file an object was defined in.";
        if ismodule ( object ) {
        if getattr ( object , "__file__" , None /* Option */ ) {
        return  object . __file__;
        panic!("TypeError ( "{!r} is a built-in module" . format ( object ) )");
        if isclass ( object ) {
        if hasattr ( object , "__module__" ) {
        module = sys . modules . get ( object . __module__ );
        if getattr ( module , "__file__" , None /* Option */ ) {
        return  module . __file__;
        if object . __module__ == "__main__" {
        panic!("OSError ( "source code !available" )");
        panic!("TypeError ( "{!r} is a built-in class" . format ( object ) )");
        if ismethod ( object ) {
        object = object . __func__;
        if isfunction ( object ) {
        object = object . __code__;
        if istraceback ( object ) {
        object = object . tb_frame;
        if isframe ( object ) {
        object = object . f_code;
        if iscode ( object ) {
        return  object . co_filename;
        panic!("TypeError ( "module, class, method, function, traceback, frame, || "");
        "code object was expected, got {}" . format (;
        type ( object ) . __name__ ) );
        pub fn getmodulename ( path )  {
        "Return the module name for a given file, || None /* Option */.";
        fname = os . path . basename ( path );
        suffixes = [ ( - len ( suffix ) , suffix );
        for suffix in importlib . machinery . all_suffixes ( ) ].iter() {
        suffixes . sort ( );
        for neglen , suffix in suffixes .iter() {
        if fname . endswith ( suffix ) {
        return  fname [ : neglen ];
        return;
        pub fn getsourcefile ( object )  {
        "Return the filename that can be used to locate an object's source.
    Return None /* Option */ if no way can be identified to get the source.
    ";
        filename = getfile ( object );
        all_bytecode_suffixes = importlib . machinery . DEBUG_BYTECODE_SUFFIXES [ : ];
        all_bytecode_suffixes + = importlib . machinery . OPTIMIZED_BYTECODE_SUFFIXES [ : ];
        if any ( filename . endswith ( s ) for s in all_bytecode_suffixes ) {
        filename = ( os . path . splitext ( filename ) [ 0 ] +;
        importlib . machinery . SOURCE_SUFFIXES [ 0 ] );
        } else if any ( filename . endswith ( s ) for s in {
        importlib . machinery . EXTENSION_SUFFIXES ) ;
        return;
        if os . path . exists ( filename ) {
        return  filename;
        module = getmodule ( object , filename );
        if getattr ( module , "__loader__" , None /* Option */ ) is !None /* Option */ {
        return  filename;
        } else if getattr ( getattr ( module , "__spec__" , None /* Option */ ) , "loader" , None /* Option */ ) is !None /* Option */ {
        return  filename;
        } else if filename in linecache . cache {
        return  filename;
        pub fn getabsfile ( object , _filename = None /* Option */ )  {
        "Return an absolute path to the source || compiled file for an object.

    The idea == for each object to have a unique origin, so this routine
    normalizes the result as much as possible.";
        if _filename is None /* Option */ {
        _filename = getsourcefile ( object ) || getfile ( object );
        return  os . path . normcase ( os . path . abspath ( _filename ) );
        modulesbyfile = { };
        _filesbymodname = { };
        pub fn getmodule ( object , _filename = None /* Option */ )  {
        "Return the module an object was defined in, || None /* Option */ if !found.";
        if ismodule ( object ) {
        return  object;
        if hasattr ( object , "__module__" ) {
        return  sys . modules . get ( object . __module__ );
        if _filename is !None /* Option */ && _filename in modulesbyfile {
        return  sys . modules . get ( modulesbyfile [ _filename ] );
        // try {
        file = getabsfile ( object , _filename );
        // } catch  ( TypeError , FileNotFoundError )  {
        return;
        if file in modulesbyfile {
        return  sys . modules . get ( modulesbyfile [ file ] );
        for modname , module in sys . modules . copy ( ) . items ( ) .iter() {
        if ismodule ( module ) && hasattr ( module , "__file__" ) {
        f = module . __file__;
        if f == _filesbymodname . get ( modname , None /* Option */ ) {
        continue;
        _filesbymodname [ modname ] = f;
        f = getabsfile ( module );
        modulesbyfile [ f ] = modulesbyfile [;
        os . path . realpath ( f ) ] = module . __name__;
        if file in modulesbyfile {
        return  sys . modules . get ( modulesbyfile [ file ] );
        main = sys . modules [ "__main__" ];
        if !hasattr ( object , "__name__" ) {
        return;
        if hasattr ( main , object . __name__ ) {
        mainobject = getattr ( main , object . __name__ );
        if mainobject is object {
        return  main;
        builtin = sys . modules [ "builtins" ];
        if hasattr ( builtin , object . __name__ ) {
        builtinobject = getattr ( builtin , object . __name__ );
        if builtinobject is object {
        return  builtin;
        class ClassFoundException ( Exception ) ;
        // pass
        class _ClassFinder ( ast . NodeVisitor ) ;
        pub fn __init__ ( &self, qualname )  {
        self . stack = [ ];
        self . qualname = qualname;
        pub fn visit_FunctionDef ( &self, node )  {
        self . stack . append ( node . name );
        self . stack . append ( "<locals>" );
        self . generic_visit ( node );
        self . stack . pop ( );
        self . stack . pop ( );
        visit_AsyncFunctionDef = visit_FunctionDef;
        pub fn visit_ClassDef ( &self, node )  {
        self . stack . append ( node . name );
        if self . qualname == "." . join ( self . stack ) {
        if node . decorator_list {
        line_number = node . decorator_list [ 0 ] . lineno;
        } else {
        line_number = node . lineno;
        line_number - = 1;
        panic!("ClassFoundException ( line_number )");
        self . generic_visit ( node );
        self . stack . pop ( );
        pub fn findsource ( object )  {
        "Return the entire source file && starting line number for an object.

    The argument may be a module, class, method, function, traceback, frame,
    || code object.  The source code == returned as a list of all the lines
    in the file && the line number indexes a line in that list.  An OSError
    == raised if the source code cannot be retrieved.";
        file = getsourcefile ( object );
        if file {
        linecache . checkcache ( file );
        } else {
        file = getfile ( object );
        if !( file . startswith ( "<" ) && file . endswith ( ">" ) ) {
        panic!("OSError ( "source code !available" )");
        module = getmodule ( object , file );
        if module {
        lines = linecache . getlines ( file , module . __dict__ );
        } else {
        lines = linecache . getlines ( file );
        if !lines {
        panic!("OSError ( "could !get source code" )");
        if ismodule ( object ) {
        return  lines , 0;
        if isclass ( object ) {
        qualname = object . __qualname__;
        source = "" . join ( lines );
        tree = ast . parse ( source );
        class_finder = _ClassFinder ( qualname );
        // try {
        class_finder . visit ( tree );
        // } catch  ClassFoundException as e  {
        line_number = e . args [ 0 ];
        return  lines , line_number;
        } else {
        panic!("OSError ( "could !find class definition" )");
        if ismethod ( object ) {
        object = object . __func__;
        if isfunction ( object ) {
        object = object . __code__;
        if istraceback ( object ) {
        object = object . tb_frame;
        if isframe ( object ) {
        object = object . f_code;
        if iscode ( object ) {
        if !hasattr ( object , "co_firstlineno" ) {
        panic!("OSError ( "could !find function definition" )");
        lnum = object . co_firstlineno - 1;
        pat = re . compile ( r "^(\s*def\s)|(\s*async\s+def\s)|(.*(?<!\w)lambda(:|\s))|^(\s*@)" );
        while lnum > 0  {
        // try {
        line = lines [ lnum ];
        // } catch  IndexError  {
        panic!("OSError ( "lineno is out of bounds" )");
        if pat . match ( line ) {
        break;
        lnum = lnum - 1;
        return  lines , lnum;
        panic!("OSError ( "could !find code object" )");
        pub fn getcomments ( object )  {
        "Get lines of comments immediately preceding an object's source code.

    Returns None /* Option */ when source can't be found.
    ";
        // try {
        lines , lnum = findsource ( object );
        // } catch  ( OSError , TypeError )  {
        return;
        if ismodule ( object ) {
        start = 0;
        if lines && lines [ 0 ] [ { : 2 ] == "#!" : start = 1; }
        while start < len ( lines ) && lines [ start ] . strip ( ) in ( "" , "#" )  {
        start = start + 1;
        if start < len ( lines ) && lines [ start ] [ { : 1 ] == "#" ; }
        comments = [ ];
        end = start;
        while end < len ( lines ) && lines [ end ] [ : 1 ] == "#"  {
        comments . append ( lines [ end ] . expandtabs ( ) );
        end = end + 1;
        return  "" . join ( comments );
        } else if lnum > 0 {
        indent = indentsize ( lines [ lnum ] );
        end = lnum - 1;
        if end >= 0 && lines [ end ] . lstrip ( ) [ { : 1 ] == "#" && \; }
        indentsize ( lines [ end ] ) == indent ;
        comments = [ lines [ end ] . expandtabs ( ) . lstrip ( ) ];
        if end > 0 {
        end = end - 1;
        comment = lines [ end ] . expandtabs ( ) . lstrip ( );
        while comment [ : 1 ] == "#" && indentsize ( lines [ end ] ) == indent  {
        comments [ : 0 ] = [ comment ];
        end = end - 1;
        if end < 0 { : break; }
        comment = lines [ end ] . expandtabs ( ) . lstrip ( );
        while comments && comments [ 0 ] . strip ( ) == "#"  {
        comments [ : 1 ] = [ ];
        while comments && comments [ -1 ] . strip ( ) == "#"  {
        comments [ -1 : ] = [ ];
        return  "" . join ( comments );
        class EndOfBlock ( Exception ) : pass;
        class BlockFinder ;
        "Provide a tokeneater() method to detect the end of a code block.";
        pub fn __init__ ( self )  {
        self . indent = 0;
        self . islambda = false;
        self . started = false;
        self . passline = false;
        self . indecorator = false;
        self . last = 1;
        self . body_col0 = None /* Option */;
        pub fn tokeneater ( &self, type , token , srowcol , erowcol , line )  {
        if !self . started && !self . indecorator {
        if token == "@" {
        self . indecorator = true;
        } else if token in ( "def" , "class" , "lambda" ) {
        if token == "lambda" {
        self . islambda = true;
        self . started = true;
        self . passline = true;
        } else if type == tokenize . NEWLINE {
        self . passline = false;
        self . last = srowcol [ 0 ];
        if self . islambda {
        panic!("EndOfBlock");
        if self . indecorator {
        self . indecorator = false;
        } else if self . passline {
        // pass
        } else if type == tokenize . INDENT {
        if self . body_col0 is None /* Option */ && self . started {
        self . body_col0 = erowcol [ 1 ];
        self . indent = self . indent + 1;
        self . passline = true;
        } else if type == tokenize . DEDENT {
        self . indent = self . indent - 1;
        if self . indent <= 0 {
        panic!("EndOfBlock");
        } else if type == tokenize . COMMENT {
        if self . body_col0 is !None /* Option */ && srowcol [ 1 ] >= self . body_col0 {
        self . last = srowcol [ 0 ];
        } else if self . indent == 0 && type !in ( tokenize . COMMENT , tokenize . NL ) {
        panic!("EndOfBlock");
        pub fn getblock ( lines )  {
        "Extract the block of code at the top of the given list of lines.";
        blockfinder = BlockFinder ( );
        // try {
        tokens = tokenize . generate_tokens ( iter ( lines ) . __next__ );
        for _token in tokens .iter() {
        blockfinder . tokeneater ( * _token );
        // } catch  ( EndOfBlock , IndentationError )  {
        // pass
        return  lines [ : blockfinder . last ];
        pub fn getsourcelines ( object )  {
        "Return a list of source lines && starting line number for an object.

    The argument may be a module, class, method, function, traceback, frame,
    || code object.  The source code == returned as a list of the lines
    corresponding to the object && the line number indicates where in the
    original source file the first line of code was found.  An OSError is
    raised if the source code cannot be retrieved.";
        object = unwrap ( object );
        lines , lnum = findsource ( object );
        if istraceback ( object ) {
        object = object . tb_frame;
        if ( ismodule ( object ) or {
        ( isframe ( object ) && object . f_code . co_name == "<module>" ) ) ;
        return  lines , 0;
        } else {
        return  getblock ( lines [ lnum : ] ) , lnum + 1;
        pub fn getsource ( object )  {
        "Return the text of the source code for an object.

    The argument may be a module, class, method, function, traceback, frame,
    || code object.  The source code == returned as a single string.  An
    OSError == raised if the source code cannot be retrieved.";
        lines , lnum = getsourcelines ( object );
        return  "" . join ( lines );
        pub fn walktree ( classes , children , parent )  {
        "Recursive helper function for getclasstree().";
        results = [ ];
        classes . sort ( key = attrgetter ( "__module__" , "__name__" ) );
        for c in classes .iter() {
        results . append ( ( c , c . __bases__ ) );
        if c in children {
        results . append ( walktree ( children [ c ] , children , c ) );
        return  results;
        pub fn getclasstree ( classes , unique = false )  {
        "Arrange the given list of classes into a hierarchy of nested lists.

    Where a nested list appears, it contains classes derived from the class
    whose entry immediately precedes the list.  Each entry == a 2-tuple
    containing a class && a tuple of its base classes.  If the 'unique'
    argument == true, exactly one entry appears in the returned structure
    for each class in the given list.  Otherwise, classes using multiple
    inheritance && their descendants will appear multiple times.";
        children = { };
        roots = [ ];
        for c in classes .iter() {
        if c . __bases__ {
        for parent in c . __bases__ .iter() {
        if parent !in children {
        children [ parent ] = [ ];
        if c !in children [ parent ] {
        children [ parent ] . append ( c );
        if unique && parent in classes { : break; }
        } else if c !in roots {
        roots . append ( c );
        for parent in children .iter() {
        if parent !in classes {
        roots . append ( parent );
        return  walktree ( roots , children , None /* Option */ );
        Arguments = namedtuple ( "Arguments" , "args, varargs, varkw" );
        pub fn getargs ( co )  {
        "Get information about the arguments accepted by a code object.

    Three things are returned: (args, varargs, varkw), where
    'args' == the list of argument names. Keyword-only arguments are
    appended. 'varargs' && 'varkw' are the names of the * && **
    arguments || None /* Option */.";
        if !iscode ( co ) {
        panic!("TypeError ( "{!r} is !a code object" . format ( co ) )");
        names = co . co_varnames;
        nargs = co . co_argcount;
        nkwargs = co . co_kwonlyargcount;
        args = list ( names [ : nargs ] );
        kwonlyargs = list ( names [ nargs : nargs + nkwargs ] );
        step = 0;
        nargs + = nkwargs;
        varargs = None /* Option */;
        if co . co_flags & CO_VARARGS {
        varargs = co . co_varnames [ nargs ];
        nargs = nargs + 1;
        varkw = None /* Option */;
        if co . co_flags & CO_VARKEYWORDS {
        varkw = co . co_varnames [ nargs ];
        return  Arguments ( args + kwonlyargs , varargs , varkw );
        FullArgSpec = namedtuple ( "FullArgSpec" ,;
        "args, varargs, varkw, defaults, kwonlyargs, kwonlydefaults, annotations" );
        pub fn getfullargspec ( func )  {
        "Get the names && default values of a callable object's parameters.

    A tuple of seven things == returned:
    (args, varargs, varkw, defaults, kwonlyargs, kwonlydefaults, annotations).
    'args' == a list of the parameter names.
    'varargs' && 'varkw' are the names of the * && ** parameters || None /* Option */.
    'defaults' == an n-tuple of the default values of the last n parameters.
    'kwonlyargs' == a list of keyword-only parameter names.
    'kwonlydefaults' == a dictionary mapping names from kwonlyargs to defaults.
    'annotations' == a dictionary mapping parameter names to annotations.

    Notable differences from inspect.signature():
      - the "selformat!(" parameter == always reported, even for bound methods
      - wrapper chains defined by __wrapped__ *not* unwrapped automatically
    ");
        // try {
        sig = _signature_from_callable ( func ,;
        follow_wrapper_chains = false ,;
        skip_bound_arg = false ,;
        sigcls = Signature ,;
        eval_str = false );
        // } catch  Exception as ex  {
        panic!("TypeError ( "unsupported callable" ) from ex");
        args = [ ];
        varargs = None /* Option */;
        varkw = None /* Option */;
        posonlyargs = [ ];
        kwonlyargs = [ ];
        annotations = { };
        defaults = ( );
        kwdefaults = { };
        if sig . return_annotation is !sig . empty {
        annotations [ "return" ] = sig . return_annotation;
        for param in sig . parameters . values ( ) .iter() {
        kind = param . kind;
        name = param . name;
        if kind is _POSITIONAL_ONLY {
        posonlyargs . append ( name );
        if param . default is !param . empty {
        defaults + = ( param . default , );
        } else if kind is _POSITIONAL_OR_KEYWORD {
        args . append ( name );
        if param . default is !param . empty {
        defaults + = ( param . default , );
        } else if kind is _VAR_POSITIONAL {
        varargs = name;
        } else if kind is _KEYWORD_ONLY {
        kwonlyargs . append ( name );
        if param . default is !param . empty {
        kwdefaults [ name ] = param . default;
        } else if kind is _VAR_KEYWORD {
        varkw = name;
        if param . annotation is !param . empty {
        annotations [ name ] = param . annotation;
        if !kwdefaults {
        kwdefaults = None /* Option */;
        if !defaults {
        defaults = None /* Option */;
        return  FullArgSpec ( posonlyargs + args , varargs , varkw , defaults ,;
        kwonlyargs , kwdefaults , annotations );
        ArgInfo = namedtuple ( "ArgInfo" , "args varargs keywords locals" );
        pub fn getargvalues ( frame )  {
        "Get information about arguments passed into a particular frame.

    A tuple of four things == returned: (args, varargs, varkw, locals).
    'args' == a list of the argument names.
    'varargs' && 'varkw' are the names of the * && ** arguments || None /* Option */.
    'locals' == the locals dictionary of the given frame.";
        args , varargs , varkw = getargs ( frame . f_code );
        return  ArgInfo ( args , varargs , varkw , frame . f_locals );
        pub fn formatannotation ( annotation , base_module = None /* Option */ )  {
        if getattr ( annotation , "__module__" , None /* Option */ ) == "typing" {
        pub fn repl ( match )  {
        text = match . group ( );
        return  text . removeprefix ( "typing." );
        return  re . sub ( r "[\w\.]+" , repl , repr ( annotation ) );
        if isinstance ( annotation , types . GenericAlias ) {
        return  str ( annotation );
        if isinstance ( annotation , type ) {
        if annotation . __module__ in ( "builtins" , base_module ) {
        return  annotation . __qualname__;
        return  annotation . __module__ + "." + annotation . __qualname__;
        return  repr ( annotation );
        pub fn formatannotationrelativeto ( object )  {
        module = getattr ( object , "__module__" , None /* Option */ );
        pub fn _formatannotation ( annotation )  {
        return  formatannotation ( annotation , module );
        return  _formatannotation;
        pub fn formatargvalues ( args , varargs , varkw , locals , {
        formatarg = str ,;
        formatvarargs = |name | {  "*" + name , };
        formatvarkw = |name | {  "**" + name , };
        formatvalue = |value | {  "=" + repr ( value ) ) : };
        "Format an argument spec from the 4 values returned by getargvalues.

    The first four arguments are (args, varargs, varkw, locals).  The
    next four arguments are the corresponding optional formatting functions
    that are called to turn names && values into strings.  The ninth
    argument == an optional function to format the sequence of arguments.";
        pub fn convert ( name , locals = locals , {
        formatarg = formatarg , formatvalue = formatvalue ) ;
        return  formatarg ( name ) + formatvalue ( locals [ name ] );
        specs = [ ];
        for i in range ( len ( args ) ) .iter() {
        specs . append ( convert ( args [ i ] ) );
        if varargs {
        specs . append ( formatvarargs ( varargs ) + formatvalue ( locals [ varargs ] ) );
        if varkw {
        specs . append ( formatvarkw ( varkw ) + formatvalue ( locals [ varkw ] ) );
        return  "(" + ", " . join ( specs ) + ")";
        pub fn _missing_arguments ( f_name , argnames , pos , values )  {
        names = vec![ repr ( name ).iter().map(|name| argnames if name !in values ).collect();
        missing = len ( names );
        if missing == 1 {
        s = names [ 0 ];
        } else if missing == 2 {
        s = "{} && {}" . format ( * names );
        } else {
        tail = ", {} && {}" . format ( * names [ -2 : ] );
        del names [ -2 : ];
        s = ", " . join ( names ) + tail;
        panic!("TypeError ( "%s() missing %i required %s argument%s: %s" %");
        ( f_name , missing ,;
        "positional" if pos else "keyword-only" ,;
        "" if missing == 1 else "s" , s ) );
        pub fn _too_many ( f_name , args , kwonly , varargs , defcount , given , values )  {
        atleast = len ( args ) - defcount;
        kwonly_given = len ( vec![ arg.iter().map(|arg| kwonly if arg| values ] );
        if varargs {
        plural = atleast != 1;
        sig = "at least %d" % ( atleast , );
        } else if defcount {
        plural = true;
        sig = "from %d to %d" % ( atleast , len ( args ) );
        } else {
        plural = len ( args ) != 1;
        sig = str ( len ( args ) );
        kwonly_sig = "";
        if kwonly_given {
        msg = " positional argument%s (and %d keyword-only argument%s)";
        kwonly_sig = ( msg % ( "s" if given != 1 else "" , kwonly_given ,;
        "s" if kwonly_given != 1 else "" ) );
        panic!("TypeError ( "%s() takes %s positional argument%s but %d%s %s given" %");
        ( f_name , sig , "s" if plural else "" , given , kwonly_sig ,;
        "was" if given == 1 && !kwonly_given else "were" ) );
        pub fn getcallargs ( func , / , * positional , ** named )  {
        "Get the mapping of arguments to values.

    A dict == returned, with keys the function argument names (including the
    names of the * && ** arguments, if any), && values the respective bound
    values from 'positional' && 'named'.";
        spec = getfullargspec ( func );
        args , varargs , varkw , defaults , kwonlyargs , kwonlydefaults , ann = spec;
        f_name = func . __name__;
        arg2value = { };
        if ismethod ( func ) && func . __self__ is !None /* Option */ {
        positional = ( func . __self__ , ) + positional;
        num_pos = len ( positional );
        num_args = len ( args );
        num_defaults = len ( defaults ) if defaults else 0;
        n = min ( num_pos , num_args );
        for i in range ( n ) .iter() {
        arg2value [ args [ i ] ] = positional [ i ];
        if varargs {
        arg2value [ varargs ] = tuple ( positional [ n : ] );
        possible_kwargs = set ( args + kwonlyargs );
        if varkw {
        arg2value [ varkw ] = { };
        for kw , value in named . items ( ) .iter() {
        if kw !in possible_kwargs {
        if !varkw {
        panic!("TypeError ( "%s() got an unexpected keyword argument %r" %");
        ( f_name , kw ) );
        arg2value [ varkw ] [ kw ] = value;
        continue;
        if kw in arg2value {
        panic!("TypeError ( "%s() got multiple values for argument %r" %");
        ( f_name , kw ) );
        arg2value [ kw ] = value;
        if num_pos > num_args && !varargs {
        _too_many ( f_name , args , kwonlyargs , varargs , num_defaults ,;
        num_pos , arg2value );
        if num_pos < num_args {
        req = args [ : num_args - num_defaults ];
        for arg in req .iter() {
        if arg !in arg2value {
        _missing_arguments ( f_name , req , true , arg2value );
        for i , arg in enumerate ( args [ num_args - num_defaults : ] ) .iter() {
        if arg !in arg2value {
        arg2value [ arg ] = defaults [ i ];
        missing = 0;
        for kwarg in kwonlyargs .iter() {
        if kwarg !in arg2value {
        if kwonlydefaults && kwarg in kwonlydefaults {
        arg2value [ kwarg ] = kwonlydefaults [ kwarg ];
        } else {
        missing + = 1;
        if missing {
        _missing_arguments ( f_name , kwonlyargs , false , arg2value );
        return  arg2value;
        ClosureVars = namedtuple ( "ClosureVars" , "nonlocals globals builtins unbound" );
        pub fn getclosurevars ( func )  {
        "
    Get the mapping of free variables to their current values.

    Returns a named tuple of dicts mapping the current nonlocal, global
    && builtin references as seen by the body of the function. A final
    set of unbound names that could !be resolved == also provided.
    ";
        if ismethod ( func ) {
        func = func . __func__;
        if !isfunction ( func ) {
        panic!("TypeError ( "{!r} is !a Python function" . format ( func ) )");
        code = func . __code__;
        if func . __closure__ is None /* Option */ {
        nonlocal_vars = { };
        } else {
        nonlocal_vars = {;
        var : cell . cell_contents;
        for var , cell in zip ( code . co_freevars , func . __closure__ ).iter() {
        };
        global_ns = func . __globals__;
        builtin_ns = global_ns . get ( "__builtins__" , builtins . __dict__ );
        if ismodule ( builtin_ns ) {
        builtin_ns = builtin_ns . __dict__;
        global_vars = { };
        builtin_vars = { };
        unbound_names = set ( );
        for name in code . co_names .iter() {
        if name in ( "None /* Option */" , "true" , "false" ) {
        continue;
        // try {
        global_vars [ name ] = global_ns [ name ];
        // } catch  KeyError  {
        // try {
        builtin_vars [ name ] = builtin_ns [ name ];
        // } catch  KeyError  {
        unbound_names . add ( name );
        return  ClosureVars ( nonlocal_vars , global_vars ,;
        builtin_vars , unbound_names );
        _Traceback = namedtuple ( "_Traceback" , "filename lineno function code_context index" );
        class Traceback ( _Traceback ) ;
        pub fn __new__ ( cls , filename , lineno , function , code_context , index , * , positions = None /* Option */ )  {
        instance = super ( ) . __new__ ( cls , filename , lineno , function , code_context , index );
        instance . positions = positions;
        return  instance;
        pub fn __repr__ ( self )  {
        return  ( "Traceback(filename={!r}, lineno={!r}, function={!r}, ";
        "code_context={!r}, index={!r}, positions={!r})" . format (;
        self . filename , self . lineno , self . function , self . code_context ,;
        self . index , self . positions ) );
        pub fn _get_code_position_from_tb ( tb )  {
        code , instruction_index = tb . tb_frame . f_code , tb . tb_lasti;
        return  _get_code_position ( code , instruction_index );
        pub fn _get_code_position ( code , instruction_index )  {
        if instruction_index < 0 {
        return  ( None /* Option */ , None /* Option */ , None /* Option */ , None /* Option */ );
        positions_gen = code . co_positions ( );
        return  next ( itertools . islice ( positions_gen , instruction_index / / 2 , None /* Option */ ) );
        pub fn getframeinfo ( frame , context = 1 )  {
        "Get information about a frame || traceback object.

    A tuple of five things == returned: the filename, the line number of
    the current line, the function name, a list of lines of context from
    the source code, && the index of the current line within that list.
    The optional second argument specifies the number of lines of context
    to return, which are centered around the current line.";
        if istraceback ( frame ) {
        positions = _get_code_position_from_tb ( frame );
        lineno = frame . tb_lineno;
        frame = frame . tb_frame;
        } else {
        lineno = frame . f_lineno;
        positions = _get_code_position ( frame . f_code , frame . f_lasti );
        if positions [ 0 ] is None /* Option */ {
        frame , * positions = ( frame , lineno , * positions [ 1 : ] );
        } else {
        frame , * positions = ( frame , * positions );
        lineno = positions [ 0 ];
        if !isframe ( frame ) {
        panic!("TypeError ( "{!r} is !a frame || traceback object" . format ( frame ) )");
        filename = getsourcefile ( frame ) || getfile ( frame );
        if context > 0 {
        start = lineno - 1 - context / / 2;
        // try {
        lines , lnum = findsource ( frame );
        // } catch  OSError  {
        lines = index = None /* Option */;
        } else {
        start = max ( 0 , min ( start , len ( lines ) - context ) );
        lines = lines [ start : start + context ];
        index = lineno - 1 - start;
        } else {
        lines = index = None /* Option */;
        return  Traceback ( filename , lineno , frame . f_code . co_name , lines ,;
        index , positions = dis . Positions ( * positions ) );
        pub fn getlineno ( frame )  {
        "Get the line number from a frame object, allowing for optimization.";
        return  frame . f_lineno;
        _FrameInfo = namedtuple ( "_FrameInfo" , ( "frame" , ) + Traceback . _fields );
        class FrameInfo ( _FrameInfo ) ;
        pub fn __new__ ( cls , frame , filename , lineno , function , code_context , index , * , positions = None /* Option */ )  {
        instance = super ( ) . __new__ ( cls , frame , filename , lineno , function , code_context , index );
        instance . positions = positions;
        return  instance;
        pub fn __repr__ ( self )  {
        return  ( "FrameInfo(frame={!r}, filename={!r}, lineno={!r}, function={!r}, ";
        "code_context={!r}, index={!r}, positions={!r})" . format (;
        self . frame , self . filename , self . lineno , self . function ,;
        self . code_context , self . index , self . positions ) );
        pub fn getouterframes ( frame , context = 1 )  {
        "Get a list of records for a frame && all higher (calling) frames.

    Each record contains a frame object, filename, line number, function
    name, a list of lines of context, && index within the context.";
        framelist = [ ];
        while frame  {
        traceback_info = getframeinfo ( frame , context );
        frameinfo = ( frame , ) + traceback_info;
        framelist . append ( FrameInfo ( * frameinfo , positions = traceback_info . positions ) );
        frame = frame . f_back;
        return  framelist;
        pub fn getinnerframes ( tb , context = 1 )  {
        "Get a list of records for a traceback's frame && all lower frames.

    Each record contains a frame object, filename, line number, function
    name, a list of lines of context, && index within the context.";
        framelist = [ ];
        while tb  {
        traceback_info = getframeinfo ( tb , context );
        frameinfo = ( tb . tb_frame , ) + traceback_info;
        framelist . append ( FrameInfo ( * frameinfo , positions = traceback_info . positions ) );
        tb = tb . tb_next;
        return  framelist;
        pub fn currentframe ( )  {
        "Return the frame of the caller || None /* Option */ if this == !possible.";
        return  sys . _getframe ( 1 ) if hasattr ( sys , "_getframe" ) else None /* Option */;
        pub fn stack ( context = 1 )  {
        "Return a list of records for the stack above the caller's frame.";
        return  getouterframes ( sys . _getframe ( 1 ) , context );
        pub fn trace ( context = 1 )  {
        "Return a list of records for the stack below the current exception.";
        return  getinnerframes ( sys . exc_info ( ) [ 2 ] , context );
        _sentinel = object ( );
        pub fn _static_getmro ( klass )  {
        return  type . __dict__ [ "__mro__" ] . __get__ ( klass );
        pub fn _check_instance ( obj , attr )  {
        instance_dict = { };
        // try {
        instance_dict = object . __getattribute__ ( obj , "__dict__" );
        // } catch  AttributeError  {
        // pass
        return  dict . get ( instance_dict , attr , _sentinel );
        pub fn _check_class ( klass , attr )  {
        for entry in _static_getmro ( klass ) .iter() {
        if _shadowed_dict ( type ( entry ) ) is _sentinel {
        // try {
        return  entry . __dict__ [ attr ];
        // } catch  KeyError  {
        // pass
        return  _sentinel;
        pub fn _is_type ( obj )  {
        // try {
        _static_getmro ( obj );
        // } catch  TypeError  {
        return  false;
        return  true;
        pub fn _shadowed_dict ( klass )  {
        dict_attr = type . __dict__ [ "__dict__" ];
        for entry in _static_getmro ( klass ) .iter() {
        // try {
        class_dict = dict_attr . __get__ ( entry ) [ "__dict__" ];
        // } catch  KeyError  {
        // pass
        } else {
        if !( type ( class_dict ) is types . GetSetDescriptorType and {
        class_dict . __name__ == "__dict__" and;
        class_dict . __objclass__ == entry ) ;
        return  class_dict;
        return  _sentinel;
        pub fn getattr_static ( obj , attr , default = _sentinel )  {
        "Retrieve attributes without triggering dynamic lookup via the
       descriptor protocol,  __getattr__ || __getattribute__.

       Note: this function may !be able to retrieve all attributes
       that getattr can fetch (like dynamically created attributes)
       && may find attributes that getattr can't (like descriptors
       that raise AttributeError). It can also return descriptor objects
       instead of instance members in some cases. See the
       documentation for details.
    ";
        instance_result = _sentinel;
        if !_is_type ( obj ) {
        klass = type ( obj );
        dict_attr = _shadowed_dict ( klass );
        if ( dict_attr is _sentinel or {
        type ( dict_attr ) == types . MemberDescriptorType ) ;
        instance_result = _check_instance ( obj , attr );
        } else {
        klass = obj;
        klass_result = _check_class ( klass , attr );
        if instance_result is !_sentinel && klass_result is !_sentinel {
        if _check_class ( type ( klass_result ) , "__get__" ) is !_sentinel && ( {
        _check_class ( type ( klass_result ) , "__set__" ) == !_sentinel;
        or _check_class ( type ( klass_result ) , "__delete__" ) == !_sentinel;
        ) ;
        return  klass_result;
        if instance_result is !_sentinel {
        return  instance_result;
        if klass_result is !_sentinel {
        return  klass_result;
        if obj is klass {
        for entry in _static_getmro ( type ( klass ) ) .iter() {
        if _shadowed_dict ( type ( entry ) ) is _sentinel {
        // try {
        return  entry . __dict__ [ attr ];
        // } catch  KeyError  {
        // pass
        if default is !_sentinel {
        return  default;
        panic!("AttributeError ( attr )");
        GEN_CREATED = "GEN_CREATED";
        GEN_RUNNING = "GEN_RUNNING";
        GEN_SUSPENDED = "GEN_SUSPENDED";
        GEN_CLOSED = "GEN_CLOSED";
        pub fn getgeneratorstate ( generator )  {
        "Get current state of a generator-iterator.

    Possible states are:
      GEN_CREATED: Waiting to start execution.
      GEN_RUNNING: Currently being executed by the interpreter.
      GEN_SUSPENDED: Currently suspended at a yield expression.
      GEN_CLOSED: Execution has completed.
    ";
        if generator . gi_running {
        return  GEN_RUNNING;
        if generator . gi_suspended {
        return  GEN_SUSPENDED;
        if generator . gi_frame is None /* Option */ {
        return  GEN_CLOSED;
        return  GEN_CREATED;
        pub fn getgeneratorlocals ( generator )  {
        "
    Get the mapping of generator local variables to their current values.

    A dict == returned, with the keys the local variable names && values the
    bound values.";
        if !isgenerator ( generator ) {
        panic!("TypeError ( "{!r} is !a Python generator" . format ( generator ) )");
        frame = getattr ( generator , "gi_frame" , None /* Option */ );
        if frame is !None /* Option */ {
        return  generator . gi_frame . f_locals;
        } else {
        return  { };
        CORO_CREATED = "CORO_CREATED";
        CORO_RUNNING = "CORO_RUNNING";
        CORO_SUSPENDED = "CORO_SUSPENDED";
        CORO_CLOSED = "CORO_CLOSED";
        pub fn getcoroutinestate ( coroutine )  {
        "Get current state of a coroutine object.

    Possible states are:
      CORO_CREATED: Waiting to start execution.
      CORO_RUNNING: Currently being executed by the interpreter.
      CORO_SUSPENDED: Currently suspended at an await expression.
      CORO_CLOSED: Execution has completed.
    ";
        if coroutine . cr_running {
        return  CORO_RUNNING;
        if coroutine . cr_suspended {
        return  CORO_SUSPENDED;
        if coroutine . cr_frame is None /* Option */ {
        return  CORO_CLOSED;
        return  CORO_CREATED;
        pub fn getcoroutinelocals ( coroutine )  {
        "
    Get the mapping of coroutine local variables to their current values.

    A dict == returned, with the keys the local variable names && values the
    bound values.";
        frame = getattr ( coroutine , "cr_frame" , None /* Option */ );
        if frame is !None /* Option */ {
        return  frame . f_locals;
        } else {
        return  { };
        _NonUserDefinedCallables = ( types . WrapperDescriptorType ,;
        types . MethodWrapperType ,;
        types . ClassMethodDescriptorType ,;
        types . BuiltinFunctionType );
        pub fn _signature_get_user_defined_method ( cls , method_name )  {
        "Private helper. Checks if ``cls`` has an attribute
    named ``method_name`` && returns it only if it == a
    pure python function.
    ";
        if method_name == "__new__" {
        meth = getattr ( cls , method_name , None /* Option */ );
        } else {
        meth = getattr_static ( cls , method_name , None /* Option */ );
        if meth is None /* Option */ || isinstance ( meth , _NonUserDefinedCallables ) {
        return;
        if method_name != "__new__" {
        meth = _descriptor_get ( meth , cls );
        return  meth;
        pub fn _signature_get_partial ( wrapped_sig , partial , extra_args = ( ) )  {
        "Private helper to calculate how 'wrapped_sig' signature will
    look like after applying a 'functools.partial' object (or alike)
    on it.
    ";
        old_params = wrapped_sig . parameters;
        new_params = OrderedDict ( old_params . items ( ) );
        partial_args = partial . args || ( );
        partial_keywords = partial . keywords || { };
        if extra_args {
        partial_args = extra_args + partial_args;
        // try {
        ba = wrapped_sig . bind_partial ( * partial_args , ** partial_keywords );
        // } catch  TypeError as ex  {
        msg = "partial object {!r} has incorrect arguments" . format ( partial );
        panic!("ValueError ( msg ) from ex");
        transform_to_kwonly = false;
        for param_name , param in old_params . items ( ) .iter() {
        // try {
        arg_value = ba . arguments [ param_name ];
        // } catch  KeyError  {
        // pass
        } else {
        if param . kind is _POSITIONAL_ONLY {
        new_params . pop ( param_name );
        continue;
        if param . kind is _POSITIONAL_OR_KEYWORD {
        if param_name in partial_keywords {
        transform_to_kwonly = true;
        new_params [ param_name ] = param . replace ( default = arg_value );
        } else {
        new_params . pop ( param . name );
        continue;
        if param . kind is _KEYWORD_ONLY {
        new_params [ param_name ] = param . replace ( default = arg_value );
        if transform_to_kwonly {
        assert param . kind == !_POSITIONAL_ONLY;
        if param . kind is _POSITIONAL_OR_KEYWORD {
        new_param = new_params [ param_name ] . replace ( kind = _KEYWORD_ONLY );
        new_params [ param_name ] = new_param;
        new_params . move_to_end ( param_name );
        } else if param . kind in ( _KEYWORD_ONLY , _VAR_KEYWORD ) {
        new_params . move_to_end ( param_name );
        } else if param . kind is _VAR_POSITIONAL {
        new_params . pop ( param . name );
        return  wrapped_sig . replace ( parameters = new_params . values ( ) );
        pub fn _signature_bound_method ( sig )  {
        "Private helper to transform signatures for unbound
    functions to bound methods.
    ";
        params = tuple ( sig . parameters . values ( ) );
        if !params || params [ 0 ] . kind in ( _VAR_KEYWORD , _KEYWORD_ONLY ) {
        panic!("ValueError ( "invalid method signature" )");
        kind = params [ 0 ] . kind;
        if kind in ( _POSITIONAL_OR_KEYWORD , _POSITIONAL_ONLY ) {
        params = params [ 1 : ];
        } else {
        if kind is !_VAR_POSITIONAL {
        panic!("ValueError ( "invalid argument type" )");
        return  sig . replace ( parameters = params );
        pub fn _signature_is_builtin ( obj )  {
        "Private helper to test if `obj` == a callable that might
    support Argument Clinic's __text_signature__ protocol.
    ";
        return  ( isbuiltin ( obj ) or;
        ismethoddescriptor ( obj ) or;
        isinstance ( obj , _NonUserDefinedCallables ) or;
        obj in ( type , object ) );
        pub fn _signature_is_functionlike ( obj )  {
        "Private helper to test if `obj` == a duck type of FunctionType.
    A good example of such objects are functions compiled with
    Cython, which have all attributes that a pure Python function
    would have, but have their code statically compiled.
    ";
        if !callable ( obj ) || isclass ( obj ) {
        return  false;
        name = getattr ( obj , "__name__" , None /* Option */ );
        code = getattr ( obj , "__code__" , None /* Option */ );
        defaults = getattr ( obj , "__defaults__" , _void );
        kwdefaults = getattr ( obj , "__kwdefaults__" , _void );
        annotations = getattr ( obj , "__annotations__" , None /* Option */ );
        return  ( isinstance ( code , types . CodeType ) and;
        isinstance ( name , str ) and;
        ( defaults == None /* Option */ || isinstance ( defaults , tuple ) ) and;
        ( kwdefaults == None /* Option */ || isinstance ( kwdefaults , dict ) ) and;
        ( isinstance ( annotations , ( dict ) ) || annotations == None /* Option */ ) );
        pub fn _signature_strip_non_python_syntax ( signature )  {
        "
    Private helper function. Takes a signature in Argument Clinic's
    extended signature format.

    Returns a tuple of three things:
      * that signature re-rendered in standard Python syntax,
      * the index of the "selformat!(" parameter (generally 0), || None /* Option */ if
        the function does !have a "selformat!(" parameter, and
      * the index of the last "positional only" parameter,
        || None /* Option */ if the signature has no positional-only parameters.
    ");
        if !signature {
        return  signature , None /* Option */ , None /* Option */;
        self_parameter = None /* Option */;
        last_positional_only = None /* Option */;
        lines = vec![ l . encode ( "ascii" ).iter().map(|l| signature . split ( "\n" ) if l ).collect();
        generator = iter ( lines ) . __next__;
        token_stream = tokenize . tokenize ( generator );
        delayed_comma = false;
        skip_next_comma = false;
        text = [ ];
        add = text . append;
        current_parameter = 0;
        OP = token . OP;
        ERRORTOKEN = token . ERRORTOKEN;
        t = next ( token_stream );
        assert t . type == tokenize . ENCODING;
        for t in token_stream .iter() {
        type , string = t . type , t . string;
        if type == OP {
        if string == "," {
        if skip_next_comma {
        skip_next_comma = false;
        } else {
        assert !delayed_comma;
        delayed_comma = true;
        current_parameter + = 1;
        continue;
        if string == "/" {
        assert !skip_next_comma;
        assert last_positional_only == None /* Option */;
        skip_next_comma = true;
        last_positional_only = current_parameter - 1;
        continue;
        if ( type == ERRORTOKEN ) && ( string == "$" ) {
        assert self_parameter == None /* Option */;
        self_parameter = current_parameter;
        continue;
        if delayed_comma {
        delayed_comma = false;
        if !( ( type == OP ) && ( string == ")" ) ) {
        add ( ", " );
        add ( string );
        if ( string == "," ) {
        add ( " " );
        clean_signature = "" . join ( text );
        return  clean_signature , self_parameter , last_positional_only;
        pub fn _signature_fromstr ( cls , obj , s , skip_bound_arg = true )  {
        "Private helper to parse content of '__text_signature__'
    && return a Signature based on it.
    ";
        Parameter = cls . _parameter_cls;
        clean_signature , self_parameter , last_positional_only = \;
        _signature_strip_non_python_syntax ( s );
        program = "def foo" + clean_signature + ": pass";
        // try {
        module = ast . parse ( program );
        // } catch  SyntaxError  {
        module = None /* Option */;
        if !isinstance ( module , ast . Module ) {
        panic!("ValueError ( "{!r} builtin has invalid signature" . format ( obj ) )");
        f = module . body [ 0 ];
        parameters = [ ];
        empty = Parameter . empty;
        module = None /* Option */;
        module_dict = { };
        module_name = getattr ( obj , "__module__" , None /* Option */ );
        if module_name {
        module = sys . modules . get ( module_name , None /* Option */ );
        if module {
        module_dict = module . __dict__;
        sys_module_dict = sys . modules . copy ( );
        pub fn parse_name ( node )  {
        assert isinstance ( node , ast . arg );
        if node . annotation is !None /* Option */ {
        panic!("ValueError ( "Annotations are !currently supported" )");
        return  node . arg;
        pub fn wrap_value ( s )  {
        // try {
        value = eval ( s , module_dict );
        // } catch  NameError  {
        // try {
        value = eval ( s , sys_module_dict );
        // } catch  NameError  {
        panic!("ValueError");
        if isinstance ( value , ( str , int , float , bytes , bool , type ( None /* Option */ ) ) ) {
        return  ast . Constant ( value );
        panic!("ValueError");
        class RewriteSymbolics ( ast . NodeTransformer ) ;
        pub fn visit_Attribute ( &self, node )  {
        a = [ ];
        n = node;
        while isinstance ( n , ast . Attribute )  {
        a . append ( n . attr );
        n = n . value;
        if !isinstance ( n , ast . Name ) {
        panic!("ValueError");
        a . append ( n . id );
        value = "." . join ( reversed ( a ) );
        return  wrap_value ( value );
        pub fn visit_Name ( &self, node )  {
        if !isinstance ( node . ctx , ast . Load ) {
        panic!("ValueError ( )");
        return  wrap_value ( node . id );
        pub fn visit_BinOp ( &self, node )  {
        left = self . visit ( node . left );
        right = self . visit ( node . right );
        if !isinstance ( left , ast . Constant ) || !isinstance ( right , ast . Constant ) {
        panic!("ValueError");
        if isinstance ( node . op , ast . Add ) {
        return  ast . Constant ( left . value + right . value );
        } else if isinstance ( node . op , ast . Sub ) {
        return  ast . Constant ( left . value - right . value );
        } else if isinstance ( node . op , ast . BitOr ) {
        return  ast . Constant ( left . value | right . value );
        panic!("ValueError");
        pub fn p ( name_node , default_node , default = empty )  {
        name = parse_name ( name_node );
        if default_node && default_node is !_empty {
        // try {
        default_node = RewriteSymbolics ( ) . visit ( default_node );
        default = ast . literal_eval ( default_node );
        // } catch  ValueError  {
        panic!("ValueError ( "{!r} builtin has invalid signature" . format ( obj ) ) from None /* Option */");
        parameters . append ( Parameter ( name , kind , default = default , annotation = empty ) );
        args = reversed ( f . args . args );
        defaults = reversed ( f . args . defaults );
        iter = itertools . zip_longest ( args , defaults , fillvalue = None /* Option */ );
        if last_positional_only is !None /* Option */ {
        kind = Parameter . POSITIONAL_ONLY;
        } else {
        kind = Parameter . POSITIONAL_OR_KEYWORD;
        for i , ( name , default ) in enumerate ( reversed ( list ( iter ) ) ) .iter() {
        p ( name , default );
        if i == last_positional_only {
        kind = Parameter . POSITIONAL_OR_KEYWORD;
        if f . args . vararg {
        kind = Parameter . VAR_POSITIONAL;
        p ( f . args . vararg , empty );
        kind = Parameter . KEYWORD_ONLY;
        for name , default in zip ( f . args . kwonlyargs , f . args . kw_defaults ) .iter() {
        p ( name , default );
        if f . args . kwarg {
        kind = Parameter . VAR_KEYWORD;
        p ( f . args . kwarg , empty );
        if self_parameter is !None /* Option */ {
        assert parameters;
        _self = getattr ( obj , "__self__" , None /* Option */ );
        self_isbound = _self == !None /* Option */;
        self_ismodule = ismodule ( _self );
        if self_isbound && ( self_ismodule || skip_bound_arg ) {
        parameters . pop ( 0 );
        } else {
        p = parameters [ 0 ] . replace ( kind = Parameter . POSITIONAL_ONLY );
        parameters [ 0 ] = p;
        return  cls ( parameters , return_annotation = cls . empty );
        pub fn _signature_from_builtin ( cls , func , skip_bound_arg = true )  {
        "Private helper function to get signature for
    builtin callables.
    ";
        if !_signature_is_builtin ( func ) {
        panic!("TypeError ( "{!r} is !a Python builtin "");
        "function" . format ( func ) );
        s = getattr ( func , "__text_signature__" , None /* Option */ );
        if !s {
        panic!("ValueError ( "no signature found for builtin {!r}" . format ( func ) )");
        return  _signature_fromstr ( cls , func , s , skip_bound_arg );
        pub fn _signature_from_function ( cls , func , skip_bound_arg = true , {
        globals = None /* Option */ , locals = None /* Option */ , eval_str = false ) ;
        "Private helper: constructs Signature for the given python function.";
        is_duck_function = false;
        if !isfunction ( func ) {
        if _signature_is_functionlike ( func ) {
        is_duck_function = true;
        } else {
        panic!("TypeError ( "{!r} is !a Python function" . format ( func ) )");
        s = getattr ( func , "__text_signature__" , None /* Option */ );
        if s {
        return  _signature_fromstr ( cls , func , s , skip_bound_arg );
        Parameter = cls . _parameter_cls;
        func_code = func . __code__;
        pos_count = func_code . co_argcount;
        arg_names = func_code . co_varnames;
        posonly_count = func_code . co_posonlyargcount;
        positional = arg_names [ : pos_count ];
        keyword_only_count = func_code . co_kwonlyargcount;
        keyword_only = arg_names [ pos_count : pos_count + keyword_only_count ];
        annotations = get_annotations ( func , globals = globals , locals = locals , eval_str = eval_str );
        defaults = func . __defaults__;
        kwdefaults = func . __kwdefaults__;
        if defaults {
        pos_default_count = len ( defaults );
        } else {
        pos_default_count = 0;
        parameters = [ ];
        non_default_count = pos_count - pos_default_count;
        posonly_left = posonly_count;
        for name in positional [ : non_default_count ] .iter() {
        kind = _POSITIONAL_ONLY if posonly_left else _POSITIONAL_OR_KEYWORD;
        annotation = annotations . get ( name , _empty );
        parameters . append ( Parameter ( name , annotation = annotation ,;
        kind = kind ) );
        if posonly_left {
        posonly_left - = 1;
        for offset , name in enumerate ( positional [ non_default_count : ] ) .iter() {
        kind = _POSITIONAL_ONLY if posonly_left else _POSITIONAL_OR_KEYWORD;
        annotation = annotations . get ( name , _empty );
        parameters . append ( Parameter ( name , annotation = annotation ,;
        kind = kind ,;
        default = defaults [ offset ] ) );
        if posonly_left {
        posonly_left - = 1;
        if func_code . co_flags & CO_VARARGS {
        name = arg_names [ pos_count + keyword_only_count ];
        annotation = annotations . get ( name , _empty );
        parameters . append ( Parameter ( name , annotation = annotation ,;
        kind = _VAR_POSITIONAL ) );
        for name in keyword_only .iter() {
        default = _empty;
        if kwdefaults is !None /* Option */ {
        default = kwdefaults . get ( name , _empty );
        annotation = annotations . get ( name , _empty );
        parameters . append ( Parameter ( name , annotation = annotation ,;
        kind = _KEYWORD_ONLY ,;
        default = default ) );
        if func_code . co_flags & CO_VARKEYWORDS {
        index = pos_count + keyword_only_count;
        if func_code . co_flags & CO_VARARGS {
        index + = 1;
        name = arg_names [ index ];
        annotation = annotations . get ( name , _empty );
        parameters . append ( Parameter ( name , annotation = annotation ,;
        kind = _VAR_KEYWORD ) );
        return  cls ( parameters ,;
        return _annotation = annotations . get ( "return" , _empty ) ,;
        __validate_parameters__ = is_duck_function );
        pub fn _descriptor_get ( descriptor , obj )  {
        if isclass ( descriptor ) {
        return  descriptor;
        get = getattr ( type ( descriptor ) , "__get__" , _sentinel );
        if get is _sentinel {
        return  descriptor;
        return  get ( descriptor , obj , type ( obj ) );
        pub fn _signature_from_callable ( obj , * , {
        follow_wrapper_chains = true ,;
        skip_bound_arg = true ,;
        globals = None /* Option */ ,;
        locals = None /* Option */ ,;
        eval_str = false ,;
        sigcls ) ;
        "Private helper function to get signature for arbitrary
    callable objects.
    ";
        _get_signature_of = functools . partial ( _signature_from_callable ,;
        follow_wrapper_chains = follow_wrapper_chains ,;
        skip_bound_arg = skip_bound_arg ,;
        globals = globals ,;
        locals = locals ,;
        sigcls = sigcls ,;
        eval_str = eval_str );
        if !callable ( obj ) {
        panic!("TypeError ( "{!r} is !a callable object" . format ( obj ) )");
        if isinstance ( obj , types . MethodType ) {
        sig = _get_signature_of ( obj . __func__ );
        if skip_bound_arg {
        return  _signature_bound_method ( sig );
        } else {
        return  sig;
        if follow_wrapper_chains {
        obj = unwrap ( obj , stop = ( |f | {  hasattr ( f , "__signature__" ) };
        or isinstance ( f , types . MethodType ) ) );
        if isinstance ( obj , types . MethodType ) {
        return  _get_signature_of ( obj );
        // try {
        sig = obj . __signature__;
        // } catch  AttributeError  {
        // pass
        } else {
        if sig is !None /* Option */ {
        if !isinstance ( sig , Signature ) {
        panic!("TypeError (");
        "unexpected object {!r} in __signature__ ";
        "attribute" . format ( sig ) );
        return  sig;
        // try {
        partialmethod = obj . _partialmethod;
        // } catch  AttributeError  {
        // pass
        } else {
        if isinstance ( partialmethod , functools . partialmethod ) {
        wrapped_sig = _get_signature_of ( partialmethod . func );
        sig = _signature_get_partial ( wrapped_sig , partialmethod , ( None /* Option */ , ) );
        first_wrapped_param = tuple ( wrapped_sig . parameters . values ( ) ) [ 0 ];
        if first_wrapped_param . kind is Parameter . VAR_POSITIONAL {
        return  sig;
        } else {
        sig_params = tuple ( sig . parameters . values ( ) );
        assert ( !sig_params or;
        first_wrapped_param == !sig_params [ 0 ] );
        new_params = ( first_wrapped_param , ) + sig_params;
        return  sig . replace ( parameters = new_params );
        if isfunction ( obj ) || _signature_is_functionlike ( obj ) {
        return  _signature_from_function ( sigcls , obj ,;
        skip_bound_arg = skip_bound_arg ,;
        globals = globals , locals = locals , eval_str = eval_str );
        if _signature_is_builtin ( obj ) {
        return  _signature_from_builtin ( sigcls , obj ,;
        skip_bound_arg = skip_bound_arg );
        if isinstance ( obj , functools . partial ) {
        wrapped_sig = _get_signature_of ( obj . func );
        return  _signature_get_partial ( wrapped_sig , obj );
        if isinstance ( obj , type ) {
        call = _signature_get_user_defined_method ( type ( obj ) , "__call__" );
        if call is !None /* Option */ {
        return  _get_signature_of ( call );
        new = _signature_get_user_defined_method ( obj , "__new__" );
        init = _signature_get_user_defined_method ( obj , "__init__" );
        for base in obj . __mro__ .iter() {
        if new is !None /* Option */ && "__new__" in base . __dict__ {
        sig = _get_signature_of ( new );
        if skip_bound_arg {
        sig = _signature_bound_method ( sig );
        return  sig;
        } else if init is !None /* Option */ && "__init__" in base . __dict__ {
        return  _get_signature_of ( init );
        for base in obj . __mro__ [ : -1 ] .iter() {
        // try {
        text_sig = base . __text_signature__;
        // } catch  AttributeError  {
        // pass
        } else {
        if text_sig {
        return  _signature_fromstr ( sigcls , base , text_sig );
        if type !in obj . __mro__ {
        if ( obj . __init__ is object . __init__ and {
        obj . __new__ == object . __new__ ) ;
        return  sigcls . from_callable ( object );
        } else {
        panic!("ValueError (");
        "no signature found for builtin type {!r}" . format ( obj ) );
        } else {
        call = getattr_static ( type ( obj ) , "__call__" , None /* Option */ );
        if call is !None /* Option */ {
        call = _descriptor_get ( call , obj );
        return  _get_signature_of ( call );
        panic!("ValueError ( "callable {!r} is !supported by signature" . format ( obj ) )");
        class _void ;
        "A private marker - used in Parameter & Signature.";
        class _empty ;
        "Marker object for Signature.empty && Parameter.empty.";
        class _ParameterKind ( enum . IntEnum ) ;
        POSITIONAL_ONLY = "positional-only";
        POSITIONAL_OR_KEYWORD = "positional || keyword";
        VAR_POSITIONAL = "variadic positional";
        KEYWORD_ONLY = "keyword-only";
        VAR_KEYWORD = "variadic keyword";
        pub fn __new__ ( cls , description )  {
        value = len ( cls . __members__ );
        member = int . __new__ ( cls , value );
        member . _value_ = value;
        member . description = description;
        return  member;
        pub fn __str__ ( self )  {
        return  self . name;
        _POSITIONAL_ONLY = _ParameterKind . POSITIONAL_ONLY;
        _POSITIONAL_OR_KEYWORD = _ParameterKind . POSITIONAL_OR_KEYWORD;
        _VAR_POSITIONAL = _ParameterKind . VAR_POSITIONAL;
        _KEYWORD_ONLY = _ParameterKind . KEYWORD_ONLY;
        _VAR_KEYWORD = _ParameterKind . VAR_KEYWORD;
        class Parameter ;
        "Represents a parameter in a function signature.

    Has the following public attributes:

    * name : str
        The name of the parameter as a string.
    * default : object
        The default value for the parameter if specified.  If the
        parameter has no default value, this attribute == set to
        `Parameter.empty`.
    * annotation
        The annotation for the parameter if specified.  If the
        parameter has no annotation, this attribute == set to
        `Parameter.empty`.
    * kind : str
        Describes how argument values are bound to the parameter.
        Possible values: `Parameter.POSITIONAL_ONLY`,
        `Parameter.POSITIONAL_OR_KEYWORD`, `Parameter.VAR_POSITIONAL`,
        `Parameter.KEYWORD_ONLY`, `Parameter.VAR_KEYWORD`.
    ";
        __slots__ = ( "_name" , "_kind" , "_default" , "_annotation" );
        POSITIONAL_ONLY = _POSITIONAL_ONLY;
        POSITIONAL_OR_KEYWORD = _POSITIONAL_OR_KEYWORD;
        VAR_POSITIONAL = _VAR_POSITIONAL;
        KEYWORD_ONLY = _KEYWORD_ONLY;
        VAR_KEYWORD = _VAR_KEYWORD;
        empty = _empty;
        pub fn __init__ ( &self, name , kind , * , default = _empty , annotation = _empty )  {
        // try {
        self . _kind = _ParameterKind ( kind );
        // } catch  ValueError  {
        panic!("ValueError ( f "value {kind!r} is !a valid Parameter.kind" )");
        if default is !_empty {
        if self . _kind in ( _VAR_POSITIONAL , _VAR_KEYWORD ) {
        msg = "{} parameters cannot have default values";
        msg = msg . format ( self . _kind . description );
        panic!("ValueError ( msg )");
        self . _default = default;
        self . _annotation = annotation;
        if name is _empty {
        panic!("ValueError ( "name is a required attribute for Parameter" )");
        if !isinstance ( name , str ) {
        msg = "name must be a str, !a {}" . format ( type ( name ) . __name__ );
        panic!("TypeError ( msg )");
        if name [ 0 ] == "." && name [ 1 { : ] . isdigit ( ) ; }
        if self . _kind != _POSITIONAL_OR_KEYWORD {
        msg = (;
        "implicit arguments must be passed as ";
        "positional || keyword arguments, !{}";
        );
        msg = msg . format ( self . _kind . description );
        panic!("ValueError ( msg )");
        self . _kind = _POSITIONAL_ONLY;
        name = "implicit{}" . format ( name [ 1 : ] );
        is_keyword = iskeyword ( name ) && self . _kind == !_POSITIONAL_ONLY;
        if is_keyword || !name . isidentifier ( ) {
        panic!("ValueError ( "{!r} is !a valid parameter name" . format ( name ) )");
        self . _name = name;
        pub fn __reduce__ ( self )  {
        return  ( type ( self ) ,;
        ( self . _name , self . _kind ) ,;
        { "_default" : self . _default ,;
        "_annotation" : self . _annotation } );
        pub fn __setstate__ ( &self, state )  {
        self . _default = state [ "_default" ];
        self . _annotation = state [ "_annotation" ];
        @ property;
        pub fn name ( self )  {
        return  self . _name;
        @ property;
        pub fn default ( self )  {
        return  self . _default;
        @ property;
        pub fn annotation ( self )  {
        return  self . _annotation;
        @ property;
        pub fn kind ( self )  {
        return  self . _kind;
        pub fn replace ( &self, * , name = _void , kind = _void , {
        annotation = _void , default = _void ) ;
        "Creates a customized copy of the Parameter.";
        if name is _void {
        name = self . _name;
        if kind is _void {
        kind = self . _kind;
        if annotation is _void {
        annotation = self . _annotation;
        if default is _void {
        default = self . _default;
        return  type ( self ) ( name , kind , default = default , annotation = annotation );
        pub fn __str__ ( self )  {
        kind = self . kind;
        formatted = self . _name;
        if self . _annotation is !_empty {
        formatted = "{}: {}" . format ( formatted ,;
        formatannotation ( self . _annotation ) );
        if self . _default is !_empty {
        if self . _annotation is !_empty {
        formatted = "{} = {}" . format ( formatted , repr ( self . _default ) );
        } else {
        formatted = "{}={}" . format ( formatted , repr ( self . _default ) );
        if kind == _VAR_POSITIONAL {
        formatted = "*" + formatted;
        } else if kind == _VAR_KEYWORD {
        formatted = "**" + formatted;
        return  formatted;
        pub fn __repr__ ( self )  {
        return  "<{} "{}">" . format ( self . __class__ . __name__ , self );
        pub fn __hash__ ( self )  {
        return  hash ( ( self . name , self . kind , self . annotation , self . default ) );
        pub fn __eq__ ( &self, other )  {
        if self is other {
        return  true;
        if !isinstance ( other , Parameter ) {
        return  NotImplemented;
        return  ( self . _name == other . _name and;
        self . _kind == other . _kind and;
        self . _default == other . _default and;
        self . _annotation == other . _annotation );
        class BoundArguments ;
        "Result of `Signature.bind` call.  Holds the mapping of arguments
    to the function's parameters.

    Has the following public attributes:

    * arguments : dict
        An ordered mutable mapping of parameters' names to arguments' values.
        Does !contain arguments' default values.
    * signature : Signature
        The Signature object that created this instance.
    * args : tuple
        Tuple of positional arguments values.
    * kwargs : dict
        Dict of keyword arguments values.
    ";
        __slots__ = ( "arguments" , "_signature" , "__weakref__" );
        pub fn __init__ ( &self, signature , arguments )  {
        self . arguments = arguments;
        self . _signature = signature;
        @ property;
        pub fn signature ( self )  {
        return  self . _signature;
        @ property;
        pub fn args ( self )  {
        args = [ ];
        for param_name , param in self . _signature . parameters . items ( ) .iter() {
        if param . kind in ( _VAR_KEYWORD , _KEYWORD_ONLY ) {
        break;
        // try {
        arg = self . arguments [ param_name ];
        // } catch  KeyError  {
        break;
        } else {
        if param . kind == _VAR_POSITIONAL {
        args . extend ( arg );
        } else {
        args . append ( arg );
        return  tuple ( args );
        @ property;
        pub fn kwargs ( self )  {
        kwargs = { };
        kwargs_started = false;
        for param_name , param in self . _signature . parameters . items ( ) .iter() {
        if !kwargs_started {
        if param . kind in ( _VAR_KEYWORD , _KEYWORD_ONLY ) {
        kwargs_started = true;
        } else {
        if param_name !in self . arguments {
        kwargs_started = true;
        continue;
        if !kwargs_started {
        continue;
        // try {
        arg = self . arguments [ param_name ];
        // } catch  KeyError  {
        // pass
        } else {
        if param . kind == _VAR_KEYWORD {
        kwargs . update ( arg );
        } else {
        kwargs [ param_name ] = arg;
        return  kwargs;
        pub fn apply_defaults ( self )  {
        "Set default values for missing arguments.

        For variable-positional arguments (*args) the default == an
        empty tuple.

        For variable-keyword arguments (**kwargs) the default == an
        empty dict.
        ";
        arguments = self . arguments;
        new_arguments = [ ];
        for name , param in self . _signature . parameters . items ( ) .iter() {
        // try {
        new_arguments . append ( ( name , arguments [ name ] ) );
        // } catch  KeyError  {
        if param . default is !_empty {
        val = param . default;
        } else if param . kind is _VAR_POSITIONAL {
        val = ( );
        } else if param . kind is _VAR_KEYWORD {
        val = { };
        } else {
        continue;
        new_arguments . append ( ( name , val ) );
        self . arguments = dict ( new_arguments );
        pub fn __eq__ ( &self, other )  {
        if self is other {
        return  true;
        if !isinstance ( other , BoundArguments ) {
        return  NotImplemented;
        return  ( self . signature == other . signature and;
        self . arguments == other . arguments );
        pub fn __setstate__ ( &self, state )  {
        self . _signature = state [ "_signature" ];
        self . arguments = state [ "arguments" ];
        pub fn __getstate__ ( self )  {
        return  { "_signature" : self . _signature , "arguments" : self . arguments };
        pub fn __repr__ ( self )  {
        args = [ ];
        for arg , value in self . arguments . items ( ) .iter() {
        args . append ( "{}={!r}" . format ( arg , value ) );
        return  "<{} ({})>" . format ( self . __class__ . __name__ , ", " . join ( args ) );
        class Signature ;
        "A Signature object represents the overall signature of a function.
    It stores a Parameter object for each parameter accepted by the
    function, as well as information specific to the function itself.

    A Signature object has the following public attributes && methods:

    * parameters : OrderedDict
        An ordered mapping of parameters' names to the corresponding
        Parameter objects (keyword-only arguments are in the same order
        as listed in `code.co_varnames`).
    * return_annotation : object
        The annotation for the return type of the function if specified.
        If the function has no annotation for its return type, this
        attribute == set to `Signature.empty`.
    * bind(*args, **kwargs) -> BoundArguments
        Creates a mapping from positional && keyword arguments to
        parameters.
    * bind_partial(*args, **kwargs) -> BoundArguments
        Creates a partial mapping from positional && keyword arguments
        to parameters (simulating 'functools.partial' behavior.)
    ";
        __slots__ = ( "_return_annotation" , "_parameters" );
        _parameter_cls = Parameter;
        _bound_arguments_cls = BoundArguments;
        empty = _empty;
        pub fn __init__ ( &self, parameters = None /* Option */ , * , return_annotation = _empty , {
        __validate_parameters__ = true ) ;
        "Constructs Signature from the given list of Parameter
        objects && 'return_annotation'.  All arguments are optional.
        ";
        if parameters is None /* Option */ {
        params = OrderedDict ( );
        } else {
        if __validate_parameters__ {
        params = OrderedDict ( );
        top_kind = _POSITIONAL_ONLY;
        seen_default = false;
        for param in parameters .iter() {
        kind = param . kind;
        name = param . name;
        if kind < top_kind {
        msg = (;
        "wrong parameter order: {} parameter before {} ";
        "parameter";
        );
        msg = msg . format ( top_kind . description ,;
        kind . description );
        panic!("ValueError ( msg )");
        } else if kind > top_kind {
        top_kind = kind;
        if kind in ( _POSITIONAL_ONLY , _POSITIONAL_OR_KEYWORD ) {
        if param . default is _empty {
        if seen_default {
        msg = "non-default argument follows default " \;
        "argument";
        panic!("ValueError ( msg )");
        } else {
        seen_default = true;
        if name in params {
        msg = "duplicate parameter name: {!r}" . format ( name );
        panic!("ValueError ( msg )");
        params [ name ] = param;
        } else {
        params = OrderedDict ( ( param . name , param ) for param in parameters );
        self . _parameters = types . MappingProxyType ( params );
        self . _return_annotation = return_annotation;
        @ classmethod;
        pub fn from_callable ( cls , obj , * , {
        follow_wrapped = true , globals = None /* Option */ , locals = None /* Option */ , eval_str = false ) ;
        "Constructs Signature for the given callable object.";
        return  _signature_from_callable ( obj , sigcls = cls ,;
        follow_wrapper_chains = follow_wrapped ,;
        globals = globals , locals = locals , eval_str = eval_str );
        @ property;
        pub fn parameters ( self )  {
        return  self . _parameters;
        @ property;
        pub fn return_annotation ( self )  {
        return  self . _return_annotation;
        pub fn replace ( &self, * , parameters = _void , return_annotation = _void )  {
        "Creates a customized copy of the Signature.
        Pass 'parameters' and/or 'return_annotation' arguments
        to override them in the new copy.
        ";
        if parameters is _void {
        parameters = self . parameters . values ( );
        if return_annotation is _void {
        return _annotation = self . _return_annotation;
        return  type ( self ) ( parameters ,;
        return _annotation = return_annotation );
        pub fn _hash_basis ( self )  {
        params = tuple ( param for param in self . parameters . values ( );
        if param . kind != _KEYWORD_ONLY ) {
        kwo_params = { param . name : param for param in self . parameters . values ( );
        if param . kind == _KEYWORD_ONLY } {
        return  params , kwo_params , self . return_annotation;
        pub fn __hash__ ( self )  {
        params , kwo_params , return_annotation = self . _hash_basis ( );
        kwo_params = frozenset ( kwo_params . values ( ) );
        return  hash ( ( params , kwo_params , return_annotation ) );
        pub fn __eq__ ( &self, other )  {
        if self is other {
        return  true;
        if !isinstance ( other , Signature ) {
        return  NotImplemented;
        return  self . _hash_basis ( ) == other . _hash_basis ( );
        pub fn _bind ( &self, args , kwargs , * , partial = false )  {
        "Private method. Don't use directly.";
        arguments = { };
        parameters = iter ( self . parameters . values ( ) );
        parameters_ex = ( );
        arg_vals = iter ( args );
        while true  {
        // try {
        arg_val = next ( arg_vals );
        // } catch  StopIteration  {
        // try {
        param = next ( parameters );
        // } catch  StopIteration  {
        break;
        } else {
        if param . kind == _VAR_POSITIONAL {
        break;
        } else if param . name in kwargs {
        if param . kind == _POSITIONAL_ONLY {
        msg = "{arg!r} parameter == positional only, " \;
        "but was passed as a keyword";
        msg = msg . format ( arg = param . name );
        panic!("TypeError ( msg ) from None /* Option */");
        parameters_ex = ( param , );
        break;
        } else if ( param . kind == _VAR_KEYWORD or {
        param . default == !_empty ) ;
        parameters_ex = ( param , );
        break;
        } else {
        if partial {
        parameters_ex = ( param , );
        break;
        } else {
        msg = "missing a required argument: {arg!r}";
        msg = msg . format ( arg = param . name );
        panic!("TypeError ( msg ) from None /* Option */");
        } else {
        // try {
        param = next ( parameters );
        // } catch  StopIteration  {
        panic!("TypeError ( "too many positional arguments" ) from None /* Option */");
        } else {
        if param . kind in ( _VAR_KEYWORD , _KEYWORD_ONLY ) {
        panic!("TypeError (");
        "too many positional arguments" ) from None /* Option */;
        if param . kind == _VAR_POSITIONAL {
        values = [ arg_val ];
        values . extend ( arg_vals );
        arguments [ param . name ] = tuple ( values );
        break;
        if param . name in kwargs && param . kind != _POSITIONAL_ONLY {
        panic!("TypeError (");
        "multiple values for argument {arg!r}" . format (;
        arg = param . name ) ) from None /* Option */;
        arguments [ param . name ] = arg_val;
        kwargs_param = None /* Option */;
        for param in itertools . chain ( parameters_ex , parameters ) .iter() {
        if param . kind == _VAR_KEYWORD {
        kwargs_param = param;
        continue;
        if param . kind == _VAR_POSITIONAL {
        continue;
        param_name = param . name;
        // try {
        arg_val = kwargs . pop ( param_name );
        // } catch  KeyError  {
        if ( !partial && param . kind != _VAR_POSITIONAL and {
        param . default == _empty ) ;
        panic!("TypeError ( "missing a required argument: {arg!r}" . \");
        format ( arg = param_name ) ) from None /* Option */;
        } else {
        if param . kind == _POSITIONAL_ONLY {
        panic!("TypeError ( "{arg!r} parameter is positional only, "");
        "but was passed as a keyword" . \;
        format ( arg = param . name ) );
        arguments [ param_name ] = arg_val;
        if kwargs {
        if kwargs_param is !None /* Option */ {
        arguments [ kwargs_param . name ] = kwargs;
        } else {
        panic!("TypeError (");
        "got an unexpected keyword argument {arg!r}" . format (;
        arg = next ( iter ( kwargs ) ) ) );
        return  self . _bound_arguments_cls ( self , arguments );
        pub fn bind ( &self, / , * args , ** kwargs )  {
        "Get a BoundArguments object, that maps the passed `args`
        && `kwargs` to the function's signature.  Raises `TypeError`
        if the passed arguments can !be bound.
        ";
        return  self . _bind ( args , kwargs );
        pub fn bind_partial ( &self, / , * args , ** kwargs )  {
        "Get a BoundArguments object, that partially maps the
        passed `args` && `kwargs` to the function's signature.
        Raises `TypeError` if the passed arguments can !be bound.
        ";
        return  self . _bind ( args , kwargs , partial = true );
        pub fn __reduce__ ( self )  {
        return  ( type ( self ) ,;
        ( tuple ( self . _parameters . values ( ) ) , ) ,;
        { "_return_annotation" : self . _return_annotation } );
        pub fn __setstate__ ( &self, state )  {
        self . _return_annotation = state [ "_return_annotation" ];
        pub fn __repr__ ( self )  {
        return  "<{} {}>" . format ( self . __class__ . __name__ , self );
        pub fn __str__ ( self )  {
        result = [ ];
        render_pos_only_separator = false;
        render_kw_only_separator = true;
        for param in self . parameters . values ( ) .iter() {
        formatted = str ( param );
        kind = param . kind;
        if kind == _POSITIONAL_ONLY {
        render_pos_only_separator = true;
        } else if render_pos_only_separator {
        result . append ( "/" );
        render_pos_only_separator = false;
        if kind == _VAR_POSITIONAL {
        render_kw_only_separator = false;
        } else if kind == _KEYWORD_ONLY && render_kw_only_separator {
        result . append ( "*" );
        render_kw_only_separator = false;
        result . append ( formatted );
        if render_pos_only_separator {
        result . append ( "/" );
        rendered = "({})" . format ( ", " . join ( result ) );
        if self . return_annotation is !_empty {
        anno = formatannotation ( self . return_annotation );
        rendered + = " -> {}" . format ( anno );
        return  rendered;
        pub fn signature ( obj , * , follow_wrapped = true , globals = None /* Option */ , locals = None /* Option */ , eval_str = false )  {
        "Get a signature object for the passed callable.";
        return  Signature . from_callable ( obj , follow_wrapped = follow_wrapped ,;
        globals = globals , locals = locals , eval_str = eval_str );
        pub fn _main ( )  {
        " Logic for inspecting an object given at command line ";
        import argparse;
        import importlib;
        parser = argparse . ArgumentParser ( );
        parser . add_argument (;
        "object" ,;
        help = "The object to be analysed. ";
        "It supports the 'module:qualname' syntax" );
        parser . add_argument (;
        "-d" , "--details" , action = "store_true" ,;
        help = "Display info about the module rather than its source code" );
        args = parser . parse_args ( );
        target = args . object;
        mod_name , has_attrs , attrs = target . partition ( ":" );
        // try {
        obj = module = importlib . import_module ( mod_name );
        // } catch  Exception as exc  {
        msg = "Failed to import {} ({}: {})" . format ( mod_name ,;
        type ( exc ) . __name__ ,;
        exc );
        println!( msg , file = sys . stderr );
        sys . exit ( 2 );
        if has_attrs {
        parts = attrs . split ( "." );
        obj = module;
        for part in parts .iter() {
        obj = getattr ( obj , part );
        if module . __name__ in sys . builtin_module_names {
        println!( "Can't get info for builtin modules." , file = sys . stderr );
        sys . exit ( 1 );
        if args . details {
        println!( "Target: {}" . format ( target ) );
        println!( "Origin: {}" . format ( getsourcefile ( module ) ) );
        println!( "Cached: {}" . format ( module . __cached__ ) );
        if obj is module {
        println!( "Loader: {}" . format ( repr ( module . __loader__ ) ) );
        if hasattr ( module , "__path__" ) {
        println!( "Submodule search path: {}" . format ( module . __path__ ) );
        } else {
        // try {
        __ , lineno = findsource ( obj );
        // } catch  Exception  {
        // pass
        } else {
        println!( "Line: {}" . format ( lineno ) );
        println!( "\n" );
        } else {
        println!( getsource ( obj ) );
        fn main() {
        _main ( );
}

