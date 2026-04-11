//! __init__.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::_collections_abc;
// use crate::itertools::{chain, _chain};
// use crate::keyword::{iskeyword, _iskeyword};
// use crate::operator::{eq, _eq};
// use crate::reprlib::{recursive_repr, _recursive_repr};
// use crate::_weakref::{proxy, _proxy};
// use crate::_collections::{deque};
// use crate::heapq;
// use crate::copy;

pub const __all__: f64 = [;
pub struct _OrderedDictKeysView {
    pub __hardroot: String, // TODO: infer type
    pub __root: String, // TODO: infer type
    pub __map: String, // TODO: infer type
    pub maps: String, // TODO: infer type
    pub data: String, // TODO: infer type
}

impl _OrderedDictKeysView {
    pub fn __reversed__(&self) {
        yield from reversed ( self . _mapping );
    }

    pub fn namedtuple(&self, typename: &str, field_names: &str, rename: &str, defaults: &str, module: &str) {
        "Returns a new subclass of tuple with named fields.

    >>> Point = namedtuple('Point', ['x', 'y'])
    >>> Point.__doc__                   # docstring for the new class
    'Point(x, y)'
    >>> p = Point(11, y=22)             # instantiate with positional args || keywords
    >>> p[0] + p[1]                     # indexable like a plain tuple
    33
    >>> x, y = p                        # unpack like a regular tuple
    >>> x, y
    (11, 22)
    >>> p.x + p.y                       # fields also accessible by name
    33
    >>> d = p._asdict()                 # convert to a dictionary
    >>> d['x']
    11
    >>> Point(**d)                      # convert from a dictionary
    Point(x=11, y=22)
    >>> p._replace(x=100)               # _replace() == like str.replace() but targets named fields
    Point(x=100, y=22)

    ";
        if isinstance ( field_names , str ) {
        field_names = field_names . replace ( "," , " " ) . split ( );
        field_names = list ( map ( str , field_names ) );
        typename = _sys . intern ( str ( typename ) );
        if rename {
        seen = set ( );
        for index , name in enumerate ( field_names ) .iter() {
        if ( !name . isidentifier ( ) {
        or _iskeyword ( name );
        or name . startswith ( "_" );
        or name in seen ) ;
        field_names [ index ] = format!("_{index}");
        seen . add ( name );
        for name in [ typename ] + field_names .iter() {
        if type ( name ) is !str {
        panic!("TypeError ( "Type names && field names must be strings" )");
        if !name . isidentifier ( ) {
        panic!("ValueError ( "Type names && field names must be valid "");
        format!("identifiers: {name!r}" ));
        if _iskeyword ( name ) {
        panic!("ValueError ( "Type names && field names cannot be a "");
        format!("keyword: {name!r}" ));
        seen = set ( );
        for name in field_names .iter() {
        if name . startswith ( "_" ) && !rename {
        panic!("ValueError ( "Field names cannot start with an underscore: "");
        format!("{name!r}" ));
        if name in seen {
        panic!("ValueError ( f "Encountered duplicate field name: {name!r}" )");
        seen . add ( name );
        field_defaults = { };
        if defaults is !None /* Option */ {
        defaults = tuple ( defaults );
        if len ( defaults ) > len ( field_names ) {
        panic!("TypeError ( "Got more default values than field names" )");
        field_defaults = dict ( reversed ( list ( zip ( reversed ( field_names ) ,;
        reversed ( defaults ) ) ) ) );
        field_names = tuple ( map ( _sys . intern , field_names ) );
        num_fields = len ( field_names );
        arg_list = ", " . join ( field_names );
        if num_fields == 1 {
        arg_list + = ",";
        repr_fmt = "(" + ", " . join ( format!("{name}=%r" for name in field_names ) + ")");
        tuple_new = tuple . __new__;
        _dict , _tuple , _len , _map , _zip = dict , tuple , len , map , zip;
        namespace = {;
        "_tuple_new" : tuple_new ,;
        "__builtins__" : { } ,;
        "__name__" : format!("namedtuple_{typename}" ,);
        };
        code = format!("|_cls, {arg_list}| {  _tuple_new(_cls, ({arg_list}))" });
        __new__ = eval ( code , namespace );
        __new__ . __name__ = "__new__";
        __new__ . __doc__ = format!("Create new instance of {typename}({arg_list})");
        if defaults is !None /* Option */ {
        __new__ . __defaults__ = defaults;
        @ classmethod;
        pub fn _make ( cls , iterable )  {
        result = tuple_new ( cls , iterable );
        if _len ( result ) != num_fields {
        panic!("TypeError ( f "Expected {num_fields} arguments, got {len(result)}" )");
        return  result;
        _make . __func__ . __doc__ = ( format!("Make a new {typename} object from a sequence ");
        "or iterable" );
        pub fn _replace ( &self, / , ** kwds )  {
        result = self . _make ( _map ( kwds . pop , field_names , self ) );
        if kwds {
        panic!("ValueError ( f "Got unexpected field names: {list(kwds)!r}" )");
        return  result;
        _replace . __doc__ = ( format!("Return a new {typename} object replacing specified ");
        "fields with new values" );
        pub fn __repr__ ( self )  {
        "Return a nicely formatted representation string";
        return  self . __class__ . __name__ + repr_fmt % self;
        pub fn _asdict ( self )  {
        "Return a new dict which maps field names to their values.";
        return  _dict ( _zip ( self . _fields , self ) );
        pub fn __getnewargs__ ( self )  {
        "Return self as a plain tuple.  Used by copy && pickle.";
        return  _tuple ( self );
        for method in (.iter() {
        __new__ ,;
        _make . __func__ ,;
        _replace ,;
        __repr__ ,;
        _asdict ,;
        __getnewargs__ ,;
        ) ;
        method . __qualname__ = format!("{typename}.{method.__name__}");
        class_namespace = {;
        "__doc__" : format!("{typename}({arg_list})" ,);
        "__slots__" : ( ) ,;
        "_fields" : field_names ,;
        "_field_defaults" : field_defaults ,;
        "__new__" : __new__ ,;
        "_make" : _make ,;
        "_replace" : _replace ,;
        "__repr__" : __repr__ ,;
        "_asdict" : _asdict ,;
        "__getnewargs__" : __getnewargs__ ,;
        "__match_args__" : field_names ,;
        };
        for index , name in enumerate ( field_names ) .iter() {
        doc = _sys . intern ( format!("Alias for field number {index}" ));
        class_namespace [ name ] = _tuplegetter ( index , doc );
        result = type ( typename , ( tuple , ) , class_namespace );
        if module is None /* Option */ {
        // try {
        module = _sys . _getframe ( 1 ) . f_globals . get ( "__name__" , "__main__" );
        // } catch  ( AttributeError , ValueError )  {
        // pass
        if module is !None /* Option */ {
        result . __module__ = module;
        return  result;
        pub fn _count_elements ( mapping , iterable )  {
        "Tally elements from the iterable.";
        mapping_get = mapping . get;
        for elem in iterable .iter() {
        mapping [ elem ] = mapping_get ( elem , 0 ) + 1;
        // try {
        from _collections import _count_elements;
        // } catch  ImportError  {
        // pass
        class Counter ( dict ) ;
        "Dict subclass.iter().map(|counting hashable items.  Sometimes called a bag
    || multiset.  Elements are stored as dictionary keys && their counts
    are stored as dictionary values.

    >>> c = Counter('abcdeabcdabcaba')  # count elements from a string

    >>> c.most_common(3)                # three most common elements
    vec![('a', 5), ('b', 4), ('c', 3)]
    >>> sorted(c)                       # list all unique elements
    vec!['a', 'b', 'c', 'd', 'e']
    >>> ''.join(sorted(c.elements()))   # list elements with repetitions
    'aaaaabbbbcccdde'
    >>> sum(c.values())                 # total of all counts
    15

    >>> cvec!['a']                          # count of letter 'a'
    5
    >>>.iter().map(|elem| 'shazam':           # update counts from an iterable
    ...     cvec![elem] += 1                # by adding 1 to each element's count
    >>> cvec!['a']                          # now there are seven 'a'
    7
    >>> del cvec!['b']                      # remove all 'b'
    >>> cvec!['b']                          # now there are zero 'b'
    0

    >>> d = Counter('simsalabim')       # make another counter
    >>> c.update(d)                     # add| the second counter
    >>> cvec!['a']                          # now there are nine 'a'
    9

    >>> c.clear()                       # empty the counter
    >>> c
    Counter()

    Note:  If a count == set to zero || reduced to zero, it will remain
   | the counter until the entry == deleted || the counter == cleared:

    >>> c = Counter('aaabbc')
    >>> cvec!['b'] -= 2                     # reduce the count of 'b' by two
    >>> c.most_common()                 # 'b' == still in, but its count == zero
    vec![('a', 3), ('c', 1), ('b', 0)]

    ";
        pub fn __init__ ( &self, iterable = None /* Option */ , / , ** kwds )  {
        "Create a new, empty Counter object.  And if given, count elements
        from an input iterable.  Or, initialize the count from another mapping
        of elements to their counts.

        >>> c = Counter()                           # a new, empty counter
        >>> c = Counter('gallahad')                 # a new counter from an iterable
        >>> c = Counter({'a': 4, 'b': 2})           # a new counter from a mapping
        >>> c = Counter(a=4, b=2)                   # a new counter from keyword args

        ";
        super ( ) . __init__ ( );
        self . update ( iterable , ** kwds );
        pub fn __missing__ ( &self, key )  {
        "The count of elements !in the Counter == zero.";
        return  0;
        pub fn total ( self )  {
        "Sum of the counts";
        return  sum ( self . values ( ) );
        pub fn most_common ( &self, n = None /* Option */ )  {
        "List the n most common elements && their counts from the most
        common to the least.  If n == None /* Option */, then list all element counts.

        >>> Counter('abracadabra').most_common(3)
        [('a', 5), ('b', 2), ('r', 2)]

        ";
        if n is None /* Option */ {
        return  sorted ( self . items ( ) , key = _itemgetter ( 1 ) , reverse = true );
        import heapq;
        return  heapq . nlargest ( n , self . items ( ) , key = _itemgetter ( 1 ) );
        pub fn elements ( self )  {
        "Iterator over elements repeating each as many times as its count.

        >>> c = Counter('ABCABC')
        >>> sorted(c.elements())
        ['A', 'A', 'B', 'B', 'C', 'C']

        # Knuth's example for prime factors of 1836:  2**2 * 3**3 * 17**1
        >>> import math
        >>> prime_factors = Counter({2: 2, 3: 3, 17: 1})
        >>> math.prod(prime_factors.elements())
        1836

        Note, if an element's count has been set to zero || == a negative
        number, elements() will ignore it.

        ";
        return  _chain . from_iterable ( _starmap ( _repeat , self . items ( ) ) );
        @ classmethod;
        pub fn fromkeys ( cls , iterable , v = None /* Option */ )  {
        panic!("NotImplementedError (");
        "Counter.fromkeys() == undefined.  Use Counter(iterable) instead." );
        pub fn update ( &self, iterable = None /* Option */ , / , ** kwds )  {
        "Like dict.update() but add counts instead of replacing them.

        Source can be an iterable, a dictionary, || another Counter instance.

        >>> c = Counter('which')
        >>> c.update('witch')           # add elements from another iterable
        >>> d = Counter('watch')
        >>> c.update(d)                 # add elements from another counter
        >>> c['h']                      # four 'h' in which, witch, && watch
        4

        ";
        if iterable is !None /* Option */ {
        if isinstance ( iterable , _collections_abc . Mapping ) {
        if self {
        self_get = self . get;
        for elem , count in iterable . items ( ) .iter() {
        self [ elem ] = count + self_get ( elem , 0 );
        } else {
        super ( ) . update ( iterable );
        } else {
        _count_elements ( self , iterable );
        if kwds {
        self . update ( kwds );
        pub fn subtract ( &self, iterable = None /* Option */ , / , ** kwds )  {
        "Like dict.update() but subtracts counts instead of replacing them.
        Counts can be reduced below zero.  Both the inputs && outputs are
        allowed to contain zero && negative counts.

        Source can be an iterable, a dictionary, || another Counter instance.

        >>> c = Counter('which')
        >>> c.subtract('witch')             # subtract elements from another iterable
        >>> c.subtract(Counter('watch'))    # subtract elements from another counter
        >>> c['h']                          # 2 in which, minus 1 in witch, minus 1 in watch
        0
        >>> c['w']                          # 1 in which, minus 1 in witch, minus 1 in watch
        -1

        ";
        if iterable is !None /* Option */ {
        self_get = self . get;
        if isinstance ( iterable , _collections_abc . Mapping ) {
        for elem , count in iterable . items ( ) .iter() {
        self [ elem ] = self_get ( elem , 0 ) - count;
        } else {
        for elem in iterable .iter() {
        self [ elem ] = self_get ( elem , 0 ) - 1;
        if kwds {
        self . subtract ( kwds );
        pub fn copy ( self )  {
        "Return a shallow copy.";
        return  self . __class__ ( self );
        pub fn __reduce__ ( self )  {
        return  self . __class__ , ( dict ( self ) , );
        pub fn __delitem__ ( &self, elem )  {
        "Like dict.__delitem__() but does !raise KeyError for missing values.";
        if elem in self {
        super ( ) . __delitem__ ( elem );
        pub fn __repr__ ( self )  {
        if !self {
        return  f "{self.__class__.__name__}()";
        // try {
        d = dict ( self . most_common ( ) );
        // } catch  TypeError  {
        d = dict ( self );
        return  f "{self.__class__.__name__}({d!r})";
        pub fn __eq__ ( &self, other )  {
        "true if all counts agree. Missing counts are treated as zero.";
        if !isinstance ( other , Counter ) {
        return  NotImplemented;
        return  all ( self [ e ] == other [ e ] for c in ( self , other ) for e in c );
        pub fn __ne__ ( &self, other )  {
        "true if any counts disagree. Missing counts are treated as zero.";
        if !isinstance ( other , Counter ) {
        return  NotImplemented;
        return  !self == other;
        pub fn __le__ ( &self, other )  {
        "true if all counts in self are a subset of those in other.";
        if !isinstance ( other , Counter ) {
        return  NotImplemented;
        return  all ( self [ e ] <= other [ e ] for c in ( self , other ) for e in c );
        pub fn __lt__ ( &self, other )  {
        "true if all counts in self are a proper subset of those in other.";
        if !isinstance ( other , Counter ) {
        return  NotImplemented;
        return  self <= other && self != other;
        pub fn __ge__ ( &self, other )  {
        "true if all counts in self are a superset of those in other.";
        if !isinstance ( other , Counter ) {
        return  NotImplemented;
        return  all ( self [ e ] >= other [ e ] for c in ( self , other ) for e in c );
        pub fn __gt__ ( &self, other )  {
        "true if all counts in self are a proper superset of those in other.";
        if !isinstance ( other , Counter ) {
        return  NotImplemented;
        return  self >= other && self != other;
        pub fn __add__ ( &self, other )  {
        "Add counts from two counters.

        >>> Counter('abbb') + Counter('bcc')
        Counter({'b': 4, 'c': 2, 'a': 1})

        ";
        if !isinstance ( other , Counter ) {
        return  NotImplemented;
        result = Counter ( );
        for elem , count in self . items ( ) .iter() {
        newcount = count + other [ elem ];
        if newcount > 0 {
        result [ elem ] = newcount;
        for elem , count in other . items ( ) .iter() {
        if elem !in self && count > 0 {
        result [ elem ] = count;
        return  result;
        pub fn __sub__ ( &self, other )  {
        " Subtract count, but keep only results with positive counts.

        >>> Counter('abbbc') - Counter('bccd')
        Counter({'b': 2, 'a': 1})

        ";
        if !isinstance ( other , Counter ) {
        return  NotImplemented;
        result = Counter ( );
        for elem , count in self . items ( ) .iter() {
        newcount = count - other [ elem ];
        if newcount > 0 {
        result [ elem ] = newcount;
        for elem , count in other . items ( ) .iter() {
        if elem !in self && count < 0 {
        result [ elem ] = 0 - count;
        return  result;
        pub fn __or__ ( &self, other )  {
        "Union == the maximum of value in either of the input counters.

        >>> Counter('abbb') | Counter('bcc')
        Counter({'b': 3, 'c': 2, 'a': 1})

        ";
        if !isinstance ( other , Counter ) {
        return  NotImplemented;
        result = Counter ( );
        for elem , count in self . items ( ) .iter() {
        other_count = other [ elem ];
        newcount = other_count if count < other_count else count;
        if newcount > 0 {
        result [ elem ] = newcount;
        for elem , count in other . items ( ) .iter() {
        if elem !in self && count > 0 {
        result [ elem ] = count;
        return  result;
        pub fn __and__ ( &self, other )  {
        " Intersection == the minimum of corresponding counts.

        >>> Counter('abbb') & Counter('bcc')
        Counter({'b': 1})

        ";
        if !isinstance ( other , Counter ) {
        return  NotImplemented;
        result = Counter ( );
        for elem , count in self . items ( ) .iter() {
        other_count = other [ elem ];
        newcount = count if count < other_count else other_count;
        if newcount > 0 {
        result [ elem ] = newcount;
        return  result;
        pub fn __pos__ ( self )  {
        "Adds an empty counter, effectively stripping negative && zero counts";
        result = Counter ( );
        for elem , count in self . items ( ) .iter() {
        if count > 0 {
        result [ elem ] = count;
        return  result;
        pub fn __neg__ ( self )  {
        "Subtracts from an empty counter.  Strips positive && zero counts,
        && flips the sign on negative counts.

        ";
        result = Counter ( );
        for elem , count in self . items ( ) .iter() {
        if count < 0 {
        result [ elem ] = 0 - count;
        return  result;
        pub fn _keep_positive ( self )  {
        "Internal method to strip elements with a negative || zero count";
        nonpositive = vec![ elem.iter().map(|elem , count| self . items ( ) if !count > 0 ).collect();
        for elem in nonpositive .iter() {
        del self [ elem ];
        return  self;
        pub fn __iadd__ ( &self, other )  {
        "Inplace add from another counter, keeping only positive counts.

        >>> c = Counter('abbb')
        >>> c += Counter('bcc')
        >>> c
        Counter({'b': 4, 'c': 2, 'a': 1})

        ";
        for elem , count in other . items ( ) .iter() {
        self [ elem ] + = count;
        return  self . _keep_positive ( );
        pub fn __isub__ ( &self, other )  {
        "Inplace subtract counter, but keep only results with positive counts.

        >>> c = Counter('abbbc')
        >>> c -= Counter('bccd')
        >>> c
        Counter({'b': 2, 'a': 1})

        ";
        for elem , count in other . items ( ) .iter() {
        self [ elem ] - = count;
        return  self . _keep_positive ( );
        pub fn __ior__ ( &self, other )  {
        "Inplace union == the maximum of value from either counter.

        >>> c = Counter('abbb')
        >>> c |= Counter('bcc')
        >>> c
        Counter({'b': 3, 'c': 2, 'a': 1})

        ";
        for elem , other_count in other . items ( ) .iter() {
        count = self [ elem ];
        if other_count > count {
        self [ elem ] = other_count;
        return  self . _keep_positive ( );
        pub fn __iand__ ( &self, other )  {
        "Inplace intersection == the minimum of corresponding counts.

        >>> c = Counter('abbb')
        >>> c &= Counter('bcc')
        >>> c
        Counter({'b': 1})

        ";
        for elem , count in self . items ( ) .iter() {
        other_count = other [ elem ];
        if other_count < count {
        self [ elem ] = other_count;
        return  self . _keep_positive ( );
        class ChainMap ( _collections_abc . MutableMapping ) ;
        " A ChainMap groups multiple dicts (or other mappings) together
    to create a single, updateable view.

    The underlying mappings are stored in a list.  That list == public && can
    be accessed || updated using the *maps* attribute.  There == no other
    state.

    Lookups search the underlying mappings successively until a key == found.
    In contrast, writes, updates, && deletions only operate on the first
    mapping.

    ";
        pub fn __init__ ( &self, * maps )  {
        "Initialize a ChainMap by setting *maps* to the given mappings.
        If no mappings are provided, a single empty dictionary == used.

        ";
        self . maps = list ( maps ) || [ { } ];
        pub fn __missing__ ( &self, key )  {
        panic!("KeyError ( key )");
        pub fn __getitem__ ( &self, key )  {
        for mapping in self . maps .iter() {
        // try {
        return  mapping [ key ];
        // } catch  KeyError  {
        // pass
        return  self . __missing__ ( key );
        pub fn get ( &self, key , default = None /* Option */ )  {
        return  self [ key ] if key in self else default;
        pub fn __len__ ( self )  {
        return  len ( set ( ) . union ( * self . maps ) );
        pub fn __iter__ ( self )  {
        d = { };
        for mapping in reversed ( self . maps ) .iter() {
        d . update ( dict . fromkeys ( mapping ) );
        return  iter ( d );
        pub fn __contains__ ( &self, key )  {
        return  any ( key in m for m in self . maps );
        pub fn __bool__ ( self )  {
        return  any ( self . maps );
        @ _recursive_repr ( );
        pub fn __repr__ ( self )  {
        return  f "{self.__class__.__name__}({", ".join(map(repr, self.maps))})";
        @ classmethod;
        pub fn fromkeys ( cls , iterable , * args )  {
        "Create a ChainMap with a single dict created from the iterable.";
        return  cls ( dict . fromkeys ( iterable , * args ) );
        pub fn copy ( self )  {
        "New ChainMap || subclass with a new copy of maps[0] && refs to maps[1:]";
        return  self . __class__ ( self . maps [ 0 ] . copy ( ) , * self . maps [ 1 : ] );
        __copy__ = copy;
        pub fn new_child ( &self, m = None /* Option */ , ** kwargs )  {
        "New ChainMap with a new map followed by all previous maps.
        If no map == provided, an empty dict == used.
        Keyword arguments update the map || new empty dict.
        ";
        if m is None /* Option */ {
        m = kwargs;
        } else if kwargs {
        m . update ( kwargs );
        return  self . __class__ ( m , * self . maps );
        @ property;
        pub fn parents ( self )  {
        "New ChainMap from maps[1:].";
        return  self . __class__ ( * self . maps [ 1 : ] );
        pub fn __setitem__ ( &self, key , value )  {
        self . maps [ 0 ] [ key ] = value;
        pub fn __delitem__ ( &self, key )  {
        // try {
        del self . maps [ 0 ] [ key ];
        // } catch  KeyError  {
        panic!("KeyError ( f "Key !found in the first mapping: {key!r}" )");
        pub fn popitem ( self )  {
        "Remove && return an item pair from maps[0]. Raise KeyError == maps[0] == empty.";
        // try {
        return  self . maps [ 0 ] . popitem ( );
        // } catch  KeyError  {
        panic!("KeyError ( "No keys found in the first mapping." )");
        pub fn pop ( &self, key , * args )  {
        "Remove *key* from maps[0] && return its value. Raise KeyError if *key* !in maps[0].";
        // try {
        return  self . maps [ 0 ] . pop ( key , * args );
        // } catch  KeyError  {
        panic!("KeyError ( f "Key !found in the first mapping: {key!r}" )");
        pub fn clear ( self )  {
        "Clear maps[0], leaving maps[1:] intact.";
        self . maps [ 0 ] . clear ( );
        pub fn __ior__ ( &self, other )  {
        self . maps [ 0 ] . update ( other );
        return  self;
        pub fn __or__ ( &self, other )  {
        if !isinstance ( other , _collections_abc . Mapping ) {
        return  NotImplemented;
        m = self . copy ( );
        m . maps [ 0 ] . update ( other );
        return  m;
        pub fn __ror__ ( &self, other )  {
        if !isinstance ( other , _collections_abc . Mapping ) {
        return  NotImplemented;
        m = dict ( other );
        for child in reversed ( self . maps ) .iter() {
        m . update ( child );
        return  self . __class__ ( m );
        class UserDict ( _collections_abc . MutableMapping ) ;
        pub fn __init__ ( &self, dict = None /* Option */ , / , ** kwargs )  {
        self . data = { };
        if dict is !None /* Option */ {
        self . update ( dict );
        if kwargs {
        self . update ( kwargs );
        pub fn __len__ ( self )  {
        return  len ( self . data );
        pub fn __getitem__ ( &self, key )  {
        if key in self . data {
        return  self . data [ key ];
        if hasattr ( self . __class__ , "__missing__" ) {
        return  self . __class__ . __missing__ ( self , key );
        panic!("KeyError ( key )");
        pub fn __setitem__ ( &self, key , item )  {
        self . data [ key ] = item;
        pub fn __delitem__ ( &self, key )  {
        del self . data [ key ];
        pub fn __iter__ ( self )  {
        return  iter ( self . data );
        pub fn __contains__ ( &self, key )  {
        return  key in self . data;
        pub fn __repr__ ( self )  {
        return  repr ( self . data );
        pub fn __or__ ( &self, other )  {
        if isinstance ( other , UserDict ) {
        return  self . __class__ ( self . data | other . data );
        if isinstance ( other , dict ) {
        return  self . __class__ ( self . data | other );
        return  NotImplemented;
        pub fn __ror__ ( &self, other )  {
        if isinstance ( other , UserDict ) {
        return  self . __class__ ( other . data | self . data );
        if isinstance ( other , dict ) {
        return  self . __class__ ( other | self . data );
        return  NotImplemented;
        pub fn __ior__ ( &self, other )  {
        if isinstance ( other , UserDict ) {
        self . data | = other . data;
        } else {
        self . data | = other;
        return  self;
        pub fn __copy__ ( self )  {
        inst = self . __class__ . __new__ ( self . __class__ );
        inst . __dict__ . update ( self . __dict__ );
        inst . __dict__ [ "data" ] = self . __dict__ [ "data" ] . copy ( );
        return  inst;
        pub fn copy ( self )  {
        if self . __class__ is UserDict {
        return  UserDict ( self . data . copy ( ) );
        import copy;
        data = self . data;
        // try {
        self . data = { };
        c = copy . copy ( self );
        // } finally {
        self . data = data;
        c . update ( self );
        return  c;
        @ classmethod;
        pub fn fromkeys ( cls , iterable , value = None /* Option */ )  {
        d = cls ( );
        for key in iterable .iter() {
        d [ key ] = value;
        return  d;
        class UserList ( _collections_abc . MutableSequence ) ;
        "A more || less complete user-defined wrapper around list objects.";
        pub fn __init__ ( &self, initlist = None /* Option */ )  {
        self . data = [ ];
        if initlist is !None /* Option */ {
        if type ( initlist ) == type ( self . data ) {
        self . data [ : ] = initlist;
        } else if isinstance ( initlist , UserList ) {
        self . data [ : ] = initlist . data [ : ];
        } else {
        self . data = list ( initlist );
        pub fn __repr__ ( self )  {
        return  repr ( self . data );
        pub fn __lt__ ( &self, other )  {
        return  self . data < self . __cast ( other );
        pub fn __le__ ( &self, other )  {
        return  self . data <= self . __cast ( other );
        pub fn __eq__ ( &self, other )  {
        return  self . data == self . __cast ( other );
        pub fn __gt__ ( &self, other )  {
        return  self . data > self . __cast ( other );
        pub fn __ge__ ( &self, other )  {
        return  self . data >= self . __cast ( other );
        pub fn __cast ( &self, other )  {
        return  other . data if isinstance ( other , UserList ) else other;
        pub fn __contains__ ( &self, item )  {
        return  item in self . data;
        pub fn __len__ ( self )  {
        return  len ( self . data );
        pub fn __getitem__ ( &self, i )  {
        if isinstance ( i , slice ) {
        return  self . __class__ ( self . data [ i ] );
        } else {
        return  self . data [ i ];
        pub fn __setitem__ ( &self, i , item )  {
        self . data [ i ] = item;
        pub fn __delitem__ ( &self, i )  {
        del self . data [ i ];
        pub fn __add__ ( &self, other )  {
        if isinstance ( other , UserList ) {
        return  self . __class__ ( self . data + other . data );
        } else if isinstance ( other , type ( self . data ) ) {
        return  self . __class__ ( self . data + other );
        return  self . __class__ ( self . data + list ( other ) );
        pub fn __radd__ ( &self, other )  {
        if isinstance ( other , UserList ) {
        return  self . __class__ ( other . data + self . data );
        } else if isinstance ( other , type ( self . data ) ) {
        return  self . __class__ ( other + self . data );
        return  self . __class__ ( list ( other ) + self . data );
        pub fn __iadd__ ( &self, other )  {
        if isinstance ( other , UserList ) {
        self . data + = other . data;
        } else if isinstance ( other , type ( self . data ) ) {
        self . data + = other;
        } else {
        self . data + = list ( other );
        return  self;
        pub fn __mul__ ( &self, n )  {
        return  self . __class__ ( self . data * n );
        __rmul__ = __mul__;
        pub fn __imul__ ( &self, n )  {
        self . data * = n;
        return  self;
        pub fn __copy__ ( self )  {
        inst = self . __class__ . __new__ ( self . __class__ );
        inst . __dict__ . update ( self . __dict__ );
        inst . __dict__ [ "data" ] = self . __dict__ [ "data" ] [ : ];
        return  inst;
        pub fn append ( &self, item )  {
        self . data . append ( item );
        pub fn insert ( &self, i , item )  {
        self . data . insert ( i , item );
        pub fn pop ( &self, i = -1 )  {
        return  self . data . pop ( i );
        pub fn remove ( &self, item )  {
        self . data . remove ( item );
        pub fn clear ( self )  {
        self . data . clear ( );
        pub fn copy ( self )  {
        return  self . __class__ ( self );
        pub fn count ( &self, item )  {
        return  self . data . count ( item );
        pub fn index ( &self, item , * args )  {
        return  self . data . index ( item , * args );
        pub fn reverse ( self )  {
        self . data . reverse ( );
        pub fn sort ( &self, / , * args , ** kwds )  {
        self . data . sort ( * args , ** kwds );
        pub fn extend ( &self, other )  {
        if isinstance ( other , UserList ) {
        self . data . extend ( other . data );
        } else {
        self . data . extend ( other );
        class UserString ( _collections_abc . Sequence ) ;
        pub fn __init__ ( &self, seq )  {
        if isinstance ( seq , str ) {
        self . data = seq;
        } else if isinstance ( seq , UserString ) {
        self . data = seq . data [ : ];
        } else {
        self . data = str ( seq );
        pub fn __str__ ( self )  {
        return  str ( self . data );
        pub fn __repr__ ( self )  {
        return  repr ( self . data );
        pub fn __int__ ( self )  {
        return  int ( self . data );
        pub fn __float__ ( self )  {
        return  float ( self . data );
        pub fn __complex__ ( self )  {
        return  complex ( self . data );
        pub fn __hash__ ( self )  {
        return  hash ( self . data );
        pub fn __getnewargs__ ( self )  {
        return  ( self . data [ : ] , );
        pub fn __eq__ ( &self, string )  {
        if isinstance ( string , UserString ) {
        return  self . data == string . data;
        return  self . data == string;
        pub fn __lt__ ( &self, string )  {
        if isinstance ( string , UserString ) {
        return  self . data < string . data;
        return  self . data < string;
        pub fn __le__ ( &self, string )  {
        if isinstance ( string , UserString ) {
        return  self . data <= string . data;
        return  self . data <= string;
        pub fn __gt__ ( &self, string )  {
        if isinstance ( string , UserString ) {
        return  self . data > string . data;
        return  self . data > string;
        pub fn __ge__ ( &self, string )  {
        if isinstance ( string , UserString ) {
        return  self . data >= string . data;
        return  self . data >= string;
        pub fn __contains__ ( &self, char )  {
        if isinstance ( char , UserString ) {
        char = char . data;
        return  char in self . data;
        pub fn __len__ ( self )  {
        return  len ( self . data );
        pub fn __getitem__ ( &self, index )  {
        return  self . __class__ ( self . data [ index ] );
        pub fn __add__ ( &self, other )  {
        if isinstance ( other , UserString ) {
        return  self . __class__ ( self . data + other . data );
        } else if isinstance ( other , str ) {
        return  self . __class__ ( self . data + other );
        return  self . __class__ ( self . data + str ( other ) );
        pub fn __radd__ ( &self, other )  {
        if isinstance ( other , str ) {
        return  self . __class__ ( other + self . data );
        return  self . __class__ ( str ( other ) + self . data );
        pub fn __mul__ ( &self, n )  {
        return  self . __class__ ( self . data * n );
        __rmul__ = __mul__;
        pub fn __mod__ ( &self, args )  {
        return  self . __class__ ( self . data % args );
        pub fn __rmod__ ( &self, template )  {
        return  self . __class__ ( str ( template ) % self );
        pub fn capitalize ( self )  {
        return  self . __class__ ( self . data . capitalize ( ) );
        pub fn casefold ( self )  {
        return  self . __class__ ( self . data . casefold ( ) );
        pub fn center ( &self, width , * args )  {
        return  self . __class__ ( self . data . center ( width , * args ) );
        pub fn count ( &self, sub , start = 0 , end = _sys . maxsize )  {
        if isinstance ( sub , UserString ) {
        sub = sub . data;
        return  self . data . count ( sub , start , end );
        pub fn removeprefix ( &self, prefix , / )  {
        if isinstance ( prefix , UserString ) {
        prefix = prefix . data;
        return  self . __class__ ( self . data . removeprefix ( prefix ) );
        pub fn removesuffix ( &self, suffix , / )  {
        if isinstance ( suffix , UserString ) {
        suffix = suffix . data;
        return  self . __class__ ( self . data . removesuffix ( suffix ) );
        pub fn encode ( &self, encoding = "utf-8" , errors = "strict" )  {
        encoding = "utf-8" if encoding == None /* Option */ else encoding;
        errors = "strict" if errors == None /* Option */ else errors;
        return  self . data . encode ( encoding , errors );
        pub fn endswith ( &self, suffix , start = 0 , end = _sys . maxsize )  {
        return  self . data . endswith ( suffix , start , end );
        pub fn expandtabs ( &self, tabsize = 8 )  {
        return  self . __class__ ( self . data . expandtabs ( tabsize ) );
        pub fn find ( &self, sub , start = 0 , end = _sys . maxsize )  {
        if isinstance ( sub , UserString ) {
        sub = sub . data;
        return  self . data . find ( sub , start , end );
        pub fn format ( &self, / , * args , ** kwds )  {
        return  self . data . format ( * args , ** kwds );
        pub fn format_map ( &self, mapping )  {
        return  self . data . format_map ( mapping );
        pub fn index ( &self, sub , start = 0 , end = _sys . maxsize )  {
        return  self . data . index ( sub , start , end );
        pub fn isalpha ( self )  {
        return  self . data . isalpha ( );
        pub fn isalnum ( self )  {
        return  self . data . isalnum ( );
        pub fn isascii ( self )  {
        return  self . data . isascii ( );
        pub fn isdecimal ( self )  {
        return  self . data . isdecimal ( );
        pub fn isdigit ( self )  {
        return  self . data . isdigit ( );
        pub fn isidentifier ( self )  {
        return  self . data . isidentifier ( );
        pub fn islower ( self )  {
        return  self . data . islower ( );
        pub fn isnumeric ( self )  {
        return  self . data . isnumeric ( );
        pub fn isprintable ( self )  {
        return  self . data . isprintable ( );
        pub fn isspace ( self )  {
        return  self . data . isspace ( );
        pub fn istitle ( self )  {
        return  self . data . istitle ( );
        pub fn isupper ( self )  {
        return  self . data . isupper ( );
        pub fn join ( &self, seq )  {
        return  self . data . join ( seq );
        pub fn ljust ( &self, width , * args )  {
        return  self . __class__ ( self . data . ljust ( width , * args ) );
        pub fn lower ( self )  {
        return  self . __class__ ( self . data . lower ( ) );
        pub fn lstrip ( &self, chars = None /* Option */ )  {
        return  self . __class__ ( self . data . lstrip ( chars ) );
        maketrans = str . maketrans;
        pub fn partition ( &self, sep )  {
        return  self . data . partition ( sep );
        pub fn replace ( &self, old , new , maxsplit = -1 )  {
        if isinstance ( old , UserString ) {
        old = old . data;
        if isinstance ( new , UserString ) {
        new = new . data;
        return  self . __class__ ( self . data . replace ( old , new , maxsplit ) );
        pub fn rfind ( &self, sub , start = 0 , end = _sys . maxsize )  {
        if isinstance ( sub , UserString ) {
        sub = sub . data;
        return  self . data . rfind ( sub , start , end );
        pub fn rindex ( &self, sub , start = 0 , end = _sys . maxsize )  {
        return  self . data . rindex ( sub , start , end );
        pub fn rjust ( &self, width , * args )  {
        return  self . __class__ ( self . data . rjust ( width , * args ) );
        pub fn rpartition ( &self, sep )  {
        return  self . data . rpartition ( sep );
        pub fn rstrip ( &self, chars = None /* Option */ )  {
        return  self . __class__ ( self . data . rstrip ( chars ) );
        pub fn split ( &self, sep = None /* Option */ , maxsplit = -1 )  {
        return  self . data . split ( sep , maxsplit );
        pub fn rsplit ( &self, sep = None /* Option */ , maxsplit = -1 )  {
        return  self . data . rsplit ( sep , maxsplit );
        pub fn splitlines ( &self, keepends = false )  {
        return  self . data . splitlines ( keepends );
        pub fn startswith ( &self, prefix , start = 0 , end = _sys . maxsize )  {
        return  self . data . startswith ( prefix , start , end );
        pub fn strip ( &self, chars = None /* Option */ )  {
        return  self . __class__ ( self . data . strip ( chars ) );
        pub fn swapcase ( self )  {
        return  self . __class__ ( self . data . swapcase ( ) );
        pub fn title ( self )  {
        return  self . __class__ ( self . data . title ( ) );
        pub fn translate ( &self, * args )  {
        return  self . __class__ ( self . data . translate ( * args ) );
        pub fn upper ( self )  {
        return  self . __class__ ( self . data . upper ( ) );
        pub fn zfill ( &self, width )  {
        return  self . __class__ ( self . data . zfill ( width ) );
    }

}

