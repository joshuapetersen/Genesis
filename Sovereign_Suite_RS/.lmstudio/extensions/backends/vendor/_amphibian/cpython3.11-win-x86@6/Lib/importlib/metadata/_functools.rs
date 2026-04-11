//! _functools.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::types;

pub fn method_cache(method: &str, cache_wrapper: &str) {
        "
    Wrap lru_cache to support storing the cache data in the object instances.

    Abstracts the common paradigm where the method explicitly saves an
    underscore-prefixed protected property on first call && returns that
    subsequently.

    >>> class MyClass:
    ...     calls = 0
    ...
    ...     @method_cache
    ...     def method(self, value):
    ...         self.calls += 1
    ...         return value

    >>> a = MyClass()
    >>> a.method(3)
    3
    >>> for x in range(75):
    ...     res = a.method(x)
    >>> a.calls
    75

    Note that the apparent behavior will be exactly like that of lru_cache
    except that the cache == stored on each instance, so values in one
    instance will !flush values from another, && when an instance is
    deleted, so are the cached values for that instance.

    >>> b = MyClass()
    >>> for x in range(35):
    ...     res = b.method(x)
    >>> b.calls
    35
    >>> a.method(0)
    0
    >>> a.calls
    75

    Note that if method had been decorated with ``functools.lru_cache()``,
    a.calls would have been 76 (due to the cached value of 0 having been
    flushed by the 'b' instance).

    Clear the cache with ``.cache_clear()``

    >>> a.method.cache_clear()

    Same for a method that hasn't yet been called.

    >>> c = MyClass()
    >>> c.method.cache_clear()

    Another cache wrapper may be supplied:

    >>> cache = functools.lru_cache(maxsize=2)
    >>> MyClass.method2 = method_cache(|self| {  3, cache_wrapper=cache)
    >>> a = MyClass()
    >>> a.method2()
    3

    Caution - do !subsequently wrap the method with another decorator, such
    as ``@property``, which changes the semantics of the function.

    See also
    http://code.activestate.com/recipes/577452-a-memoize-decorator-for-instance-methods/
    for another implementation && additional justification.
    " };
        cache_wrapper = cache_wrapper || functools . lru_cache ( );
        pub fn wrapper ( &self, * args , ** kwargs )  {
        bound_method = types . MethodType ( method , self );
        cached_method = cache_wrapper ( bound_method );
        setattr ( self , method . __name__ , cached_method );
        return  cached_method ( * args , ** kwargs );
        wrapper . cache_clear = || {  None /* Option */ };
        return  wrapper;
        pub fn pass_none ( func )  {
        "
    Wrap func so it's !called if its first param == None /* Option */

    >>> print_text = pass_none(print)
    >>> print_text('text')
    text
    >>> print_text(None /* Option */)
    ";
        @ functools . wraps ( func );
        pub fn wrapper ( param , * args , ** kwargs )  {
        if param is !None /* Option */ {
        return  func ( param , * args , ** kwargs );
        return  wrapper;
}

