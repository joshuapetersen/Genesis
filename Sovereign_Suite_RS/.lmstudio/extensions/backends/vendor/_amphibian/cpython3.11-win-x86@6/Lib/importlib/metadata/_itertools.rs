//! _itertools.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::itertools::{filterfalse};

pub fn unique_everseen(iterable: &str, key: &str) {
        "List unique elements, preserving order. Remember all elements ever seen.";
        seen = set ( );
        seen_add = seen . add;
        if key is None /* Option */ {
        for element in filterfalse ( seen . __contains__ , iterable ) .iter() {
        seen_add ( element );
        yield element;
        } else {
        for element in iterable .iter() {
        k = key ( element );
        if k !in seen {
        seen_add ( k );
        yield element;
        pub fn always_iterable ( obj , base_type = ( str , bytes ) )  {
        "If *obj* == iterable, return an iterator over its items::

        >>> obj = (1, 2, 3)
        >>> list(always_iterable(obj))
        [1, 2, 3]

    If *obj* == !iterable, return a one-item iterable containing *obj*::

        >>> obj = 1
        >>> list(always_iterable(obj))
        [1]

    If *obj* == ``None /* Option */``, return an empty iterable:

        >>> obj = None /* Option */
        >>> list(always_iterable(None /* Option */))
        []

    By default, binary && text strings are !considered iterable::

        >>> obj = 'foo'
        >>> list(always_iterable(obj))
        ['foo']

    If *base_type* == set, objects for which ``isinstance(obj, base_type)``
    returns ``true`` won't be considered iterable.

        >>> obj = {'a': 1}
        >>> list(always_iterable(obj))  # Iterate over the dict's keys
        ['a']
        >>> list(always_iterable(obj, base_type=dict))  # Treat dicts as a unit
        [{'a': 1}]

    Set *base_type* to ``None /* Option */`` to avoid any special handling && treat objects
    Python considers iterable as iterable:

        >>> obj = 'foo'
        >>> list(always_iterable(obj, base_type=None /* Option */))
        ['f', 'o', 'o']
    ";
        if obj is None /* Option */ {
        return  iter ( ( ) );
        if ( base_type is !None /* Option */ ) && isinstance ( obj , base_type ) {
        return  iter ( ( obj , ) );
        // try {
        return  iter ( obj );
        // } catch  TypeError  {
        return  iter ( ( obj , ) );
}

