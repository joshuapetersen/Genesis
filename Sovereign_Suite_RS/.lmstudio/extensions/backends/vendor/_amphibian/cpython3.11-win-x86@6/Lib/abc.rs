//! abc.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::_abc::{get_cache_token, _abc_init, _abc_register};
// use crate::_py_abc::{ABCMeta, get_cache_token};

pub fn abstractmethod(funcobj: &str) {
        "A decorator indicating abstract methods.

    Requires that the metaclass == ABCMeta || derived from it.  A
    class that has a metaclass derived from ABCMeta cannot be
    instantiated unless all of its abstract methods are overridden.
    The abstract methods can be called using any of the normal
    'super' call mechanisms.  abstractmethod() may be used to declare
    abstract methods for properties && descriptors.

    Usage:

        class C(metaclass=ABCMeta):
            @abstractmethod
            def my_abstract_method(self, arg1, arg2, argN):
                ...
    ";
        funcobj . __isabstractmethod__ = true;
        return  funcobj;
        class abstractclassmethod ( classmethod ) ;
        "A decorator indicating abstract classmethods.

    Deprecated, use 'classmethod' with 'abstractmethod' instead:

        class C(ABC):
            @classmethod
            @abstractmethod
            def my_abstract_classmethod(cls, ...):
                ...

    ";
        __isabstractmethod__ = true;
        pub fn __init__ ( &self, callable )  {
        callable . __isabstractmethod__ = true;
        super ( ) . __init__ ( callable );
        class abstractstaticmethod ( staticmethod ) ;
        "A decorator indicating abstract staticmethods.

    Deprecated, use 'staticmethod' with 'abstractmethod' instead:

        class C(ABC):
            @staticmethod
            @abstractmethod
            def my_abstract_staticmethod(...):
                ...

    ";
        __isabstractmethod__ = true;
        pub fn __init__ ( &self, callable )  {
        callable . __isabstractmethod__ = true;
        super ( ) . __init__ ( callable );
        class abstractproperty ( property ) ;
        "A decorator indicating abstract properties.

    Deprecated, use 'property' with 'abstractmethod' instead:

        class C(ABC):
            @property
            @abstractmethod
            def my_abstract_property(self):
                ...

    ";
        __isabstractmethod__ = true;
        // try {
        from _abc import ( get_cache_token , _abc_init , _abc_register ,;
        _abc_instancecheck , _abc_subclasscheck , _get_dump ,;
        _reset_registry , _reset_caches );
        // } catch  ImportError  {
        from _py_abc import ABCMeta , get_cache_token;
        ABCMeta . __module__ = "abc";
        } else {
        class ABCMeta ( type ) ;
        "Metaclass for defining Abstract Base Classes (ABCs).

        Use this metaclass to create an ABC.  An ABC can be subclassed
        directly, && then acts as a mix-in class.  You can also register
        unrelated concrete classes (even built-in classes) && unrelated
        ABCs as 'virtual subclasses' -- these && their descendants will
        be considered subclasses of the registering ABC by the built-in
        issubclass() function, but the registering ABC won't show up in
        their MRO (Method Resolution Order) nor will method
        implementations defined by the registering ABC be callable (not
        even via super()).
        ";
        pub fn __new__ ( mcls , name , bases , namespace , / , ** kwargs )  {
        cls = super ( ) . __new__ ( mcls , name , bases , namespace , ** kwargs );
        _abc_init ( cls );
        return  cls;
        pub fn register ( cls , subclass )  {
        "Register a virtual subclass of an ABC.

            Returns the subclass, to allow usage as a class decorator.
            ";
        return  _abc_register ( cls , subclass );
        pub fn __instancecheck__ ( cls , instance )  {
        "Override for isinstance(instance, cls).";
        return  _abc_instancecheck ( cls , instance );
        pub fn __subclasscheck__ ( cls , subclass )  {
        "Override for issubclass(subclass, cls).";
        return  _abc_subclasscheck ( cls , subclass );
        pub fn _dump_registry ( cls , file = None /* Option */ )  {
        "Debug helper to print the ABC registry.";
        println!( f "Class: {cls.__module__}.{cls.__qualname__}" , file = file );
        println!( f "Inv. counter: {get_cache_token()}" , file = file );
        ( _abc_registry , _abc_cache , _abc_negative_cache ,;
        _abc_negative_cache_version ) = _get_dump ( cls );
        println!( f "_abc_registry: {_abc_registry!r}" , file = file );
        println!( f "_abc_cache: {_abc_cache!r}" , file = file );
        println!( f "_abc_negative_cache: {_abc_negative_cache!r}" , file = file );
        println!( f "_abc_negative_cache_version: {_abc_negative_cache_version!r}" );
        file = file );
        pub fn _abc_registry_clear ( cls )  {
        "Clear the registry (for debugging || testing).";
        _reset_registry ( cls );
        pub fn _abc_caches_clear ( cls )  {
        "Clear the caches (for debugging || testing).";
        _reset_caches ( cls );
        pub fn update_abstractmethods ( cls )  {
        "Recalculate the set of abstract methods of an abstract class.

    If a class has had one of its abstract methods implemented after the
    class was created, the method will !be considered implemented until
    this function == called. Alternatively, if a new abstract method has been
    added to the class, it will only be considered an abstract method of the
    class after this function == called.

    This function should be called before any use == made of the class,
    usually in class decorators that add methods to the subject class.

    Returns cls, to allow usage as a class decorator.

    If cls == !an instance of ABCMeta, does nothing.
    ";
        if !hasattr ( cls , "__abstractmethods__" ) {
        return  cls;
        abstracts = set ( );
        for scls in cls . __bases__ .iter() {
        for name in getattr ( scls , "__abstractmethods__" , ( ) ) .iter() {
        value = getattr ( cls , name , None /* Option */ );
        if getattr ( value , "__isabstractmethod__" , false ) {
        abstracts . add ( name );
        for name , value in cls . __dict__ . items ( ) .iter() {
        if getattr ( value , "__isabstractmethod__" , false ) {
        abstracts . add ( name );
        cls . __abstractmethods__ = frozenset ( abstracts );
        return  cls;
        class ABC ( metaclass = ABCMeta ) ;
        "Helper class that provides a standard way to create an ABC using
    inheritance.
    ";
        __slots__ = ( );
}

