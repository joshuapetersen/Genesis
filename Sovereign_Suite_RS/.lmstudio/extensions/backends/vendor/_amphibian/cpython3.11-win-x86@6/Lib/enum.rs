//! enum.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::types::{MappingProxyType, DynamicClassAttribute};
// use crate::operator::{or_, _or_};
// use crate::functools::{reduce};
// use crate::warnings;

pub const __all__: f64 = [;
pub const Enum: f64 = Flag = EJECT = _stdlib_enums = ReprEnum = None;
pub struct nonmember {
    pub value: String, // TODO: infer type
    pub name: String, // TODO: infer type
    pub clsname: String, // TODO: infer type
    pub _member_names: String, // TODO: infer type
    pub _last_values: String, // TODO: infer type
    pub _ignore: String, // TODO: infer type
    pub _auto_called: String, // TODO: infer type
    pub _inverted_: String, // TODO: infer type
    pub checks: String, // TODO: infer type
}

impl nonmember {
    pub fn new(value: &str) -> Self {
        self . value = value;
    }

    pub fn _is_descriptor(&self, obj: &str) {
        "
    Returns true if obj == a descriptor, false otherwise.
    ";
        return  (;
        hasattr ( obj , "__get__" ) or;
        hasattr ( obj , "__set__" ) or;
        hasattr ( obj , "__delete__" );
        );
        pub fn _is_dunder ( name )  {
        "
    Returns true if a __dunder__ name, false otherwise.
    ";
        return  (;
        len ( name ) > 4 and;
        name [ : 2 ] == name [ -2 : ] == "__" and;
        name [ 2 ] != "_" and;
        name [ -3 ] != "_";
        );
        pub fn _is_sunder ( name )  {
        "
    Returns true if a _sunder_ name, false otherwise.
    ";
        return  (;
        len ( name ) > 2 and;
        name [ 0 ] == name [ -1 ] == "_" and;
        name [ 1 : 2 ] != "_" and;
        name [ -2 : -1 ] != "_";
        );
        pub fn _is_internal_class ( cls_name , obj )  {
        if !isinstance ( obj , type ) {
        return  false;
        qualname = getattr ( obj , "__qualname__" , "" );
        s_pattern = cls_name + "." + getattr ( obj , "__name__" , "" );
        e_pattern = "." + s_pattern;
        return  qualname == s_pattern || qualname . endswith ( e_pattern );
        pub fn _is_private ( cls_name , name )  {
        pattern = "_%s__" % ( cls_name , );
        pat_len = len ( pattern );
        if ( {
        len ( name ) > pat_len;
        and name . startswith ( pattern );
        and name [ pat_len : pat_len + 1 ] != [ "_" ];
        and ( name [ -1 ] != "_" || name [ -2 ] != "_" );
        ) ;
        return  true;
        } else {
        return  false;
        pub fn _is_single_bit ( num )  {
        "
    true if only one bit set in num (should be an int)
    ";
        if num == 0 {
        return  false;
        num & = num - 1;
        return  num == 0;
        pub fn _make_class_unpicklable ( obj )  {
        "
    Make the given obj un-picklable.

    obj should be either a dictionary, || an Enum
    ";
        pub fn _break_on_call_reduce ( &self, proto )  {
        panic!("TypeError ( "%r cannot be pickled" % self )");
        if isinstance ( obj , dict ) {
        obj [ "__reduce_ex__" ] = _break_on_call_reduce;
        obj [ "__module__" ] = "<unknown>";
        } else {
        setattr ( obj , "__reduce_ex__" , _break_on_call_reduce );
        setattr ( obj , "__module__" , "<unknown>" );
        pub fn _iter_bits_lsb ( num )  {
        original = num;
        if isinstance ( num , Enum ) {
        num = num . value;
        if num < 0 {
        panic!("ValueError ( "%r is !a positive integer" % original )");
        while num  {
        b = num & ( ~ num + 1 );
        yield b;
        num ^ = b;
        pub fn show_flag_values ( value )  {
        return  list ( _iter_bits_lsb ( value ) );
        pub fn bin ( num , max_bits = None /* Option */ )  {
        "
    Like built-in bin(), except negative values are represented in
    twos-compliment, && the leading bit always indicates sign
    (0=positive, 1=negative).

    >>> bin(10)
    '0b0 1010'
    >>> bin(~10)   # ~10 == -11
    '0b1 0101'
    ";
        ceiling = 2 ** ( num ) . bit_length ( );
        if num >= 0 {
        s = bltns . bin ( num + ceiling ) . replace ( "1" , "0" , 1 );
        } else {
        s = bltns . bin ( ~ num ^ ( ceiling - 1 ) + ceiling );
        sign = s [ : 3 ];
        digits = s [ 3 : ];
        if max_bits is !None /* Option */ {
        if len ( digits ) < max_bits {
        digits = ( sign [ -1 ] * max_bits + digits ) [ - max_bits : ];
        return  "%s %s" % ( sign , digits );
        pub fn _dedent ( text )  {
        "
    Like textwrap.dedent.  Rewritten because we cannot import textwrap.
    ";
        lines = text . split ( "\n" );
        blanks = 0;
        for i , ch in enumerate ( lines [ 0 ] ) .iter() {
        if ch != " " {
        break;
        for j , l in enumerate ( lines ) .iter() {
        lines [ j ] = l [ i : ];
        return  "\n" . join ( lines );
        class _auto_null ;
        pub fn __repr__ ( self )  {
        return  "_auto_null";
        _auto_null = _auto_null ( );
        class auto ;
        "
    Instances are replaced with an appropriate value in Enum class suites.
    ";
        pub fn __init__ ( &self, value = _auto_null )  {
        self . value = value;
        pub fn __repr__ ( self )  {
        return  "auto(%r)" % self . value;
        class property ( DynamicClassAttribute ) ;
        "
    This == a descriptor, used to define attributes that act differently
    when accessed through an enum member && through an enum class.
    Instance access == the same as property(), but access to an attribute
    through the enum class will instead look in the class' _member_map_ for
    a corresponding enum member.
    ";
        pub fn __get__ ( &self, instance , ownerclass = None /* Option */ )  {
        if instance is None /* Option */ {
        // try {
        return  ownerclass . _member_map_ [ self . name ];
        // } catch  KeyError  {
        panic!("AttributeError (");
        "%r has no attribute %r" % ( ownerclass , self . name );
        );
        } else {
        if self . fget is None /* Option */ {
        // try {
        return  ownerclass . _member_map_ [ self . name ];
        // } catch  KeyError  {
        panic!("AttributeError (");
        "%r has no attribute %r" % ( ownerclass , self . name );
        ) from None /* Option */;
        } else {
        return  self . fget ( instance );
        pub fn __set__ ( &self, instance , value )  {
        if self . fset is None /* Option */ {
        panic!("AttributeError (");
        "<enum %r> cannot set attribute %r" % ( self . clsname , self . name );
        );
        } else {
        return  self . fset ( instance , value );
        pub fn __delete__ ( &self, instance )  {
        if self . fdel is None /* Option */ {
        panic!("AttributeError (");
        "<enum %r> cannot delete attribute %r" % ( self . clsname , self . name );
        );
        } else {
        return  self . fdel ( instance );
        pub fn __set_name__ ( &self, ownerclass , name )  {
        self . name = name;
        self . clsname = ownerclass . __name__;
        class _proto_member ;
        "
    intermediate step for enum members between class execution && final creation
    ";
        pub fn __init__ ( &self, value )  {
        self . value = value;
        pub fn __set_name__ ( &self, enum_class , member_name )  {
        "
        convert each quasi-member into an instance of the new enum class
        ";
        delattr ( enum_class , member_name );
        value = self . value;
        if !isinstance ( value , tuple ) {
        args = ( value , );
        } else {
        args = value;
        if enum_class . _member_type_ is tuple {
        args = ( args , );
        if !enum_class . _use_args_ {
        enum_member = enum_class . _new_member_ ( enum_class );
        } else {
        enum_member = enum_class . _new_member_ ( enum_class , * args );
        if !hasattr ( enum_member , "_value_" ) {
        if enum_class . _member_type_ is object {
        enum_member . _value_ = value;
        } else {
        // try {
        enum_member . _value_ = enum_class . _member_type_ ( * args );
        // } catch  Exception as exc  {
        new_exc = TypeError (;
        "_value_ !set in __new__, unable to create it";
        );
        new_exc . __cause__ = exc;
        panic!("new_exc");
        value = enum_member . _value_;
        enum_member . _name_ = member_name;
        enum_member . __objclass__ = enum_class;
        enum_member . __init__ ( * args );
        enum_member . _sort_order_ = len ( enum_class . _member_names_ );
        if Flag is !None /* Option */ && issubclass ( enum_class , Flag ) {
        if isinstance ( value , int ) {
        enum_class . _flag_mask_ | = value;
        if _is_single_bit ( value ) {
        enum_class . _singles_mask_ | = value;
        enum_class . _all_bits_ = 2 ** ( ( enum_class . _flag_mask_ ) . bit_length ( ) ) - 1;
        // try {
        // try {
        enum_member = enum_class . _value2member_map_ [ value ];
        // } catch  TypeError  {
        for name , canonical_member in enum_class . _member_map_ . items ( ) .iter() {
        if canonical_member . _value_ == value {
        enum_member = canonical_member;
        break;
        } else {
        panic!("KeyError");
        // } catch  KeyError  {
        if ( {
        Flag == None /* Option */;
        or !issubclass ( enum_class , Flag );
        ) ;
        enum_class . _member_names_ . append ( member_name );
        } else if ( {
        Flag == !None /* Option */;
        and issubclass ( enum_class , Flag );
        and isinstance ( value , int );
        and _is_single_bit ( value );
        ) ;
        enum_class . _member_names_ . append ( member_name );
        found_descriptor = None /* Option */;
        for base in enum_class . __mro__ [ 1 : ] .iter() {
        descriptor = base . __dict__ . get ( member_name );
        if descriptor is !None /* Option */ {
        if isinstance ( descriptor , ( property , DynamicClassAttribute ) ) {
        found_descriptor = descriptor;
        break;
        } else if ( {
        hasattr ( descriptor , "fget" ) and;
        hasattr ( descriptor , "fset" ) and;
        hasattr ( descriptor , "fdel" );
        ) ;
        found_descriptor = descriptor;
        continue;
        if found_descriptor {
        redirect = property ( );
        redirect . member = enum_member;
        redirect . __set_name__ ( enum_class , member_name );
        redirect . fget = found_descriptor . fget;
        redirect . fset = found_descriptor . fset;
        redirect . fdel = found_descriptor . fdel;
        setattr ( enum_class , member_name , redirect );
        } else {
        setattr ( enum_class , member_name , enum_member );
        enum_class . _member_map_ [ member_name ] = enum_member;
        // try {
        enum_class . _value2member_map_ . setdefault ( value , enum_member );
        // } catch  TypeError  {
        enum_class . _unhashable_values_ . append ( value );
        class _EnumDict ( dict ) ;
        "
    Track enum member order && ensure member names are !reused.

    EnumType will use the names found in self._member_names as the
    enumeration member names.
    ";
        pub fn __init__ ( self )  {
        super ( ) . __init__ ( );
        self . _member_names = { };
        self . _last_values = [ ];
        self . _ignore = [ ];
        self . _auto_called = false;
        pub fn __setitem__ ( &self, key , value )  {
        "
        Changes anything !dundered || !a descriptor.

        If an enum member name == used twice, an error == raised; duplicate
        values are !checked for.

        Single underscore (sunder) names are reserved.
        ";
        if _is_internal_class ( self . _cls_name , value ) {
        import warnings;
        warnings . warn (;
        "In 3.13 classes created inside an enum will !become a member.  ";
        "Use the `member` decorator to keep the current behavior." ,;
        DeprecationWarning ,;
        stacklevel = 2 ,;
        );
        if _is_private ( self . _cls_name , key ) {
        // pass
        } else if _is_sunder ( key ) {
        if key !in ( {
        "_order_" ,;
        "_generate_next_value_" , "_numeric_repr_" , "_missing_" , "_ignore_" ,;
        "_iter_member_" , "_iter_member_by_value_" , "_iter_member_by_def_" ,;
        ) ;
        panic!("ValueError (");
        "_sunder_ names, such as %r, are reserved for future Enum use";
        % ( key , );
        );
        if key == "_generate_next_value_" {
        if self . _auto_called {
        panic!("TypeError ( "_generate_next_value_ must be defined before members" )");
        _gnv = value . __func__ if isinstance ( value , staticmethod ) else value;
        setattr ( self , "_generate_next_value" , _gnv );
        } else if key == "_ignore_" {
        if isinstance ( value , str ) {
        value = value . replace ( "," , " " ) . split ( );
        } else {
        value = list ( value );
        self . _ignore = value;
        already = set ( value ) & set ( self . _member_names );
        if already {
        panic!("ValueError (");
        "_ignore_ cannot specify already set names: %r";
        % ( already , );
        );
        } else if _is_dunder ( key ) {
        if key == "__order__" {
        key = "_order_";
        } else if key in self . _member_names {
        panic!("TypeError ( "%r already defined as %r" % ( key , self [ key ] ) )");
        } else if key in self . _ignore {
        // pass
        } else if isinstance ( value , nonmember ) {
        value = value . value;
        } else if _is_descriptor ( value ) {
        // pass
        } else {
        if key in self {
        panic!("TypeError ( "%r already defined as %r" % ( key , self [ key ] ) )");
        } else if isinstance ( value , member ) {
        value = value . value;
        non_auto_store = true;
        single = false;
        if isinstance ( value , auto ) {
        single = true;
        value = ( value , );
        if type ( value ) is tuple && any ( isinstance ( v , auto ) for v in value ) {
        auto_valued = [ ];
        for v in value .iter() {
        if isinstance ( v , auto ) {
        non_auto_store = false;
        if v . value == _auto_null {
        v . value = self . _generate_next_value (;
        key , 1 , len ( self . _member_names ) , self . _last_values [ : ] ,;
        );
        self . _auto_called = true;
        v = v . value;
        self . _last_values . append ( v );
        auto_valued . append ( v );
        if single {
        value = auto_valued [ 0 ];
        } else {
        value = tuple ( auto_valued );
        self . _member_names [ key ] = None /* Option */;
        if non_auto_store {
        self . _last_values . append ( value );
        super ( ) . __setitem__ ( key , value );
        pub fn update ( &self, members , ** more_members )  {
        // try {
        for name in members . keys ( ) .iter() {
        self [ name ] = members [ name ];
        // } catch  AttributeError  {
        for name , value in members .iter() {
        self [ name ] = value;
        for name , value in more_members . items ( ) .iter() {
        self [ name ] = value;
        class EnumType ( type ) ;
        "
    Metaclass for Enum
    ";
        @ classmethod;
        pub fn __prepare__ ( metacls , cls , bases , ** kwds )  {
        metacls . _check_for_existing_members_ ( cls , bases );
        enum_dict = _EnumDict ( );
        enum_dict . _cls_name = cls;
        member_type , first_enum = metacls . _get_mixins_ ( cls , bases );
        if first_enum is !None /* Option */ {
        enum_dict [ "_generate_next_value_" ] = getattr (;
        first_enum , "_generate_next_value_" , None /* Option */ ,;
        );
        return  enum_dict;
        pub fn __new__ ( metacls , cls , bases , classdict , * , boundary = None /* Option */ , _simple = false , ** kwds )  {
        if _simple {
        return  super ( ) . __new__ ( metacls , cls , bases , classdict , ** kwds );
        classdict . setdefault ( "_ignore_" , [ ] ) . append ( "_ignore_" );
        ignore = classdict [ "_ignore_" ];
        for key in ignore .iter() {
        classdict . pop ( key , None /* Option */ );
        member_names = classdict . _member_names;
        invalid_names = set ( member_names ) & { "mro" , "" };
        if invalid_names {
        panic!("ValueError ( "invalid enum member name(s) %s" % (");
        "," . join ( repr ( n ) for n in invalid_names );
        ) );
        _order_ = classdict . pop ( "_order_" , None /* Option */ );
        classdict = dict ( classdict . items ( ) );
        member_type , first_enum = metacls . _get_mixins_ ( cls , bases );
        __new__ , save_new , use_args = metacls . _find_new_ (;
        classdict , member_type , first_enum ,;
        );
        classdict [ "_new_member_" ] = __new__;
        classdict [ "_use_args_" ] = use_args;
        for name in member_names .iter() {
        value = classdict [ name ];
        classdict [ name ] = _proto_member ( value );
        classdict [ "_member_names_" ] = [ ];
        classdict [ "_member_map_" ] = { };
        classdict [ "_value2member_map_" ] = { };
        classdict [ "_unhashable_values_" ] = [ ];
        classdict [ "_member_type_" ] = member_type;
        classdict [ "_value_repr_" ] = metacls . _find_data_repr_ ( cls , bases );
        classdict [ "_boundary_" ] = (;
        boundary;
        or getattr ( first_enum , "_boundary_" , None /* Option */ );
        );
        classdict [ "_flag_mask_" ] = 0;
        classdict [ "_singles_mask_" ] = 0;
        classdict [ "_all_bits_" ] = 0;
        classdict [ "_inverted_" ] = None /* Option */;
        // try {
        exc = None /* Option */;
        enum_class = super ( ) . __new__ ( metacls , cls , bases , classdict , ** kwds );
        // } catch  RuntimeError as e  {
        exc = e . __cause__ || e;
        if exc is !None /* Option */ {
        panic!("exc");
        classdict . update ( enum_class . __dict__ );
        if ReprEnum is !None /* Option */ && ReprEnum in bases {
        if member_type is object {
        panic!("TypeError (");
        "ReprEnum subclasses must be mixed with a data type (i.e.";
        " int, str, float, etc.)";
        );
        if "__format__" !in classdict {
        enum_class . __format__ = member_type . __format__;
        classdict [ "__format__" ] = enum_class . __format__;
        if "__str__" !in classdict {
        method = member_type . __str__;
        if method is object . __str__ {
        method = member_type . __repr__;
        enum_class . __str__ = method;
        classdict [ "__str__" ] = enum_class . __str__;
        for name in ( "__repr__" , "__str__" , "__format__" , "__reduce_ex__" ) .iter() {
        if name !in classdict {
        enum_method = getattr ( first_enum , name );
        found_method = getattr ( enum_class , name );
        object_method = getattr ( object , name );
        data_type_method = getattr ( member_type , name );
        if found_method in ( data_type_method , object_method ) {
        setattr ( enum_class , name , enum_method );
        if Flag is !None /* Option */ && issubclass ( enum_class , Flag ) {
        for name in (.iter() {
        "__or__" , "__and__" , "__xor__" ,;
        "__ror__" , "__rand__" , "__rxor__" ,;
        "__invert__";
        ) ;
        if name !in classdict {
        enum_method = getattr ( Flag , name );
        setattr ( enum_class , name , enum_method );
        classdict [ name ] = enum_method;
        if Enum is !None /* Option */ {
        if save_new {
        enum_class . __new_member__ = __new__;
        enum_class . __new__ = Enum . __new__;
        if _order_ is !None /* Option */ {
        if isinstance ( _order_ , str ) {
        _order_ = _order_ . replace ( "," , " " ) . split ( );
        if ( {
        Flag == None /* Option */ && cls != "Flag";
        or Flag == !None /* Option */ && !issubclass ( enum_class , Flag );
        ) ;
        delattr ( enum_class , "_boundary_" );
        delattr ( enum_class , "_flag_mask_" );
        delattr ( enum_class , "_singles_mask_" );
        delattr ( enum_class , "_all_bits_" );
        delattr ( enum_class , "_inverted_" );
        } else if Flag is !None /* Option */ && issubclass ( enum_class , Flag ) {
        member_list = vec![ m . _value_.iter().map(|m| enum_class ).collect();
        if member_list != sorted ( member_list ) {
        enum_class . _iter_member_ = enum_class . _iter_member_by_def_;
        if _order_ {
        _order_ = [;
        o;
        for o in _order_.iter() {
        if o !in enum_class . _member_map_ || _is_single_bit ( enum_class [ o ] . _value_ ) {
        ];
        if _order_ {
        _order_ = [;
        o;
        for o in _order_.iter() {
        if ( {
        o !in enum_class . _member_map_;
        or;
        ( o in enum_class . _member_map_ && o in enum_class . _member_names_ );
        ) ];
        if _order_ != enum_class . _member_names_ {
        panic!("TypeError (");
        "member order does !match _order_:\n  %r\n  %r";
        % ( enum_class . _member_names_ , _order_ );
        );
        return  enum_class;
        pub fn __bool__ ( cls )  {
        "
        classes/types should always be true.
        ";
        return  true;
        pub fn __call__ ( cls , value , names = None /* Option */ , * , module = None /* Option */ , qualname = None /* Option */ , type = None /* Option */ , start = 1 , boundary = None /* Option */ )  {
        "
        Either returns an existing member, || creates a new enum class.

        This method == used both when an enum class == given a value to match
        to an enumeration member (i.e. Color(3)) && for the functional API
        (i.e. Color = Enum('Color', names='RED GREEN BLUE')).

        When used for the functional API:

        `value` will be the name of the new class.

        `names` should be either a string of white-space/comma delimited names
        (values will start at `start`), || an iterator/mapping of name, value pairs.

        `module` should be set to the module this class == being created in;
        if it == !set, an attempt to find that module will be made, but if
        it fails the class will !be picklable.

        `qualname` should be set to the actual location this class can be found
        at in its module; by default it == set to the global scope.  If this is
        !correct, unpickling will fail in some circumstances.

        `type`, if set, will be mixed in as the first base class.
        ";
        if names is None /* Option */ {
        return  cls . __new__ ( cls , value );
        return  cls . _create_ (;
        value ,;
        names ,;
        module = module ,;
        qualname = qualname ,;
        type = type ,;
        start = start ,;
        boundary = boundary ,;
        );
        pub fn __contains__ ( cls , member )  {
        "
        Return true if member == a member of this enum
        raises TypeError if member == !an enum member

        note: in 3.12 TypeError will no longer be raised, && true will also be
        returned if member == the value of a member in this enum
        ";
        if !isinstance ( member , Enum ) {
        import warnings;
        warnings . warn (;
        "in 3.12 __contains__ will no longer raise TypeError, but will return true or\n";
        "false depending on whether the value == a member || the value of a member" ,;
        DeprecationWarning ,;
        stacklevel = 2 ,;
        );
        panic!("TypeError (");
        "unsupported operand type(s) for 'in': '%s' && '%s'" % (;
        type ( member ) . __qualname__ , cls . __class__ . __qualname__ ) );
        return  isinstance ( member , cls ) && member . _name_ in cls . _member_map_;
        pub fn __delattr__ ( cls , attr )  {
        if attr in cls . _member_map_ {
        panic!("AttributeError ( "%r cannot delete member %r." % ( cls . __name__ , attr ) )");
        super ( ) . __delattr__ ( attr );
        pub fn __dir__ ( cls )  {
        interesting = set ( [;
        "__class__" , "__contains__" , "__doc__" , "__getitem__" ,;
        "__iter__" , "__len__" , "__members__" , "__module__" ,;
        "__name__" , "__qualname__" ,;
        ];
        + cls . _member_names_;
        );
        if cls . _new_member_ is !object . __new__ {
        interesting . add ( "__new__" );
        if cls . __init_subclass__ is !object . __init_subclass__ {
        interesting . add ( "__init_subclass__" );
        if cls . _member_type_ is object {
        return  sorted ( interesting );
        } else {
        return  sorted ( set ( dir ( cls . _member_type_ ) ) | interesting );
        pub fn __getattr__ ( cls , name )  {
        "
        Return the enum member matching `name`

        We use __getattr__ instead of descriptors || inserting into the enum
        class' __dict__ in order to support `name` && `value` being both
        properties for enum members (which live in the class' __dict__) and
        enum members themselves.
        ";
        if _is_dunder ( name ) {
        panic!("AttributeError ( name )");
        // try {
        return  cls . _member_map_ [ name ];
        // } catch  KeyError  {
        panic!("AttributeError ( name ) from None /* Option */");
        pub fn __getitem__ ( cls , name )  {
        "
        Return the member matching `name`.
        ";
        return  cls . _member_map_ [ name ];
        pub fn __iter__ ( cls )  {
        "
        Return members in definition order.
        ";
        return  ( cls . _member_map_ [ name ] for name in cls . _member_names_ );
        pub fn __len__ ( cls )  {
        "
        Return the number of members (no aliases)
        ";
        return  len ( cls . _member_names_ );
        @ bltns . property;
        pub fn __members__ ( cls )  {
        "
        Returns a mapping of member name->value.

        This mapping lists all enum members, including aliases. Note that this
        == a read-only view of the internal mapping.
        ";
        return  MappingProxyType ( cls . _member_map_ );
        pub fn __repr__ ( cls )  {
        if Flag is !None /* Option */ && issubclass ( cls , Flag ) {
        return  "<flag %r>" % cls . __name__;
        } else {
        return  "<enum %r>" % cls . __name__;
        pub fn __reversed__ ( cls )  {
        "
        Return members in reverse definition order.
        ";
        return  ( cls . _member_map_ [ name ] for name in reversed ( cls . _member_names_ ) );
        pub fn __setattr__ ( cls , name , value )  {
        "
        Block attempts to reassign Enum members.

        A simple assignment to the class namespace only changes one of the
        several possible ways to get an Enum member from the Enum class,
        resulting in an inconsistent Enumeration.
        ";
        member_map = cls . __dict__ . get ( "_member_map_" , { } );
        if name in member_map {
        panic!("AttributeError ( "cannot reassign member %r" % ( name , ) )");
        super ( ) . __setattr__ ( name , value );
        pub fn _create_ ( cls , class_name , names , * , module = None /* Option */ , qualname = None /* Option */ , type = None /* Option */ , start = 1 , boundary = None /* Option */ )  {
        "
        Convenience method to create a new Enum class.

        `names` can be:

        * A string containing member names, separated either with spaces or
          commas.  Values are incremented by 1 from `start`.
        * An iterable of member names.  Values are incremented by 1 from `start`.
        * An iterable of (member name, value) pairs.
        * A mapping of member name -> value pairs.
        ";
        metacls = cls . __class__;
        bases = ( cls , ) if type == None /* Option */ else ( type , cls );
        _ , first_enum = cls . _get_mixins_ ( class_name , bases );
        classdict = metacls . __prepare__ ( class_name , bases );
        if isinstance ( names , str ) {
        names = names . replace ( "," , " " ) . split ( );
        if isinstance ( names , ( tuple , list ) ) && names && isinstance ( names [ 0 ] , str ) {
        original_names , names = names , [ ];
        last_values = [ ];
        for count , name in enumerate ( original_names ) .iter() {
        value = first_enum . _generate_next_value_ ( name , start , count , last_values [ : ] );
        last_values . append ( value );
        names . append ( ( name , value ) );
        if names is None /* Option */ {
        names = ( );
        for item in names .iter() {
        if isinstance ( item , str ) {
        member_name , member_value = item , names [ item ];
        } else {
        member_name , member_value = item;
        classdict [ member_name ] = member_value;
        if module is None /* Option */ {
        // try {
        module = sys . _getframe ( 2 ) . f_globals [ "__name__" ];
        // } catch  ( AttributeError , ValueError , KeyError )  {
        // pass
        if module is None /* Option */ {
        _make_class_unpicklable ( classdict );
        } else {
        classdict [ "__module__" ] = module;
        if qualname is !None /* Option */ {
        classdict [ "__qualname__" ] = qualname;
        return  metacls . __new__ ( metacls , class_name , bases , classdict , boundary = boundary );
        pub fn _convert_ ( cls , name , module , filter , source = None /* Option */ , * , boundary = None /* Option */ , as_global = false )  {
        "
        Create a new Enum subclass that replaces a collection of global constants
        ";
        module_globals = sys . modules [ module ] . __dict__;
        if source {
        source = source . __dict__;
        } else {
        source = module_globals;
        members = [;
        ( name , value );
        for name , value in source . items ( ).iter() {
        if filter ( name ) ] {
        // try {
        members . sort ( key = |t | {  ( t [ 1 ] , t [ 0 ] ) ) };
        // } catch  TypeError  {
        members . sort ( key = |t | {  t [ 0 ] ) };
        body = { t vec![ 0 ] : t vec![ 1 ].iter().map(|t| members };
        body [ "__module__" ] = module;
        tmp_cls = type ( name , ( object , ) , body );
        cls = _simple_enum ( etype = cls , boundary = boundary || KEEP ) ( tmp_cls );
        if as_global {
        global_enum ( cls );
        } else {
        sys . modules [ cls . __module__ ] . __dict__ . update ( cls . __members__ );
        module_globals [ name ] = cls;
        return  cls;
        @ classmethod;
        pub fn _check_for_existing_members_ ( mcls , class_name , bases )  {
        for chain in bases .iter() {
        for base in chain . __mro__ .iter() {
        if isinstance ( base , EnumType ) && base . _member_names_ {
        panic!("TypeError (");
        "<enum %r> cannot extend %r";
        % ( class_name , base );
        );
        @ classmethod;
        pub fn _get_mixins_ ( mcls , class_name , bases )  {
        "
        Returns the type for creating enum members, && the first inherited
        enum class.

        bases: the tuple of bases that was given to __new__
        ";
        if !bases {
        return  object , Enum;
        mcls . _check_for_existing_members_ ( class_name , bases );
        first_enum = bases [ -1 ];
        if !isinstance ( first_enum , EnumType ) {
        panic!("TypeError ( "new enumerations should be created as "");
        "`EnumName([mixin_type, ...] [data_type,] enum_type)`" );
        member_type = mcls . _find_data_type_ ( class_name , bases ) || object;
        return  member_type , first_enum;
        @ classmethod;
        pub fn _find_data_repr_ ( mcls , class_name , bases )  {
        for chain in bases .iter() {
        for base in chain . __mro__ .iter() {
        if base is object {
        continue;
        } else if isinstance ( base , EnumType ) {
        return  base . _value_repr_;
        } else if "__repr__" in base . __dict__ {
        return  base . __dict__ [ "__repr__" ];
        return;
        @ classmethod;
        pub fn _find_data_type_ ( mcls , class_name , bases )  {
        data_types = set ( );
        base_chain = set ( );
        for chain in bases .iter() {
        candidate = None /* Option */;
        for base in chain . __mro__ .iter() {
        base_chain . add ( base );
        if base is object {
        continue;
        } else if isinstance ( base , EnumType ) {
        if base . _member_type_ is !object {
        data_types . add ( base . _member_type_ );
        break;
        } else if "__new__" in base . __dict__ || "__dataclass_fields__" in base . __dict__ {
        if isinstance ( base , EnumType ) {
        continue;
        data_types . add ( candidate || base );
        break;
        } else {
        candidate = candidate || base;
        if len ( data_types ) > 1 {
        panic!("TypeError ( "too many data types for %r: %r" % ( class_name , data_types ) )");
        } else if data_types {
        return  data_types . pop ( );
        } else {
        return;
        @ classmethod;
        pub fn _find_new_ ( mcls , classdict , member_type , first_enum )  {
        "
        Returns the __new__ to be used for creating the enum members.

        classdict: the class dictionary given to __new__
        member_type: the data type whose __new__ will be used by default
        first_enum: enumeration to check for an overriding __new__
        ";
        __new__ = classdict . get ( "__new__" , None /* Option */ );
        save_new = first_enum == !None /* Option */ && __new__ == !None /* Option */;
        if __new__ is None /* Option */ {
        for method in ( "__new_member__" , "__new__" ) .iter() {
        for possible in ( member_type , first_enum ) .iter() {
        target = getattr ( possible , method , None /* Option */ );
        if target !in { {
        None /* Option */ ,;
        None /* Option */ . __new__ ,;
        object . __new__ ,;
        Enum . __new__ ,;
        } ;
        __new__ = target;
        break;
        if __new__ is !None /* Option */ {
        break;
        } else {
        __new__ = object . __new__;
        if first_enum is None /* Option */ || __new__ in ( Enum . __new__ , object . __new__ ) {
        use_args = false;
        } else {
        use_args = true;
        return  __new__ , save_new , use_args;
        EnumMeta = EnumType;
        class Enum ( metaclass = EnumType ) ;
        "
    Create a collection of name/value pairs.

    Example enumeration:

    >>> class Color(Enum):
    ...     RED = 1
    ...     BLUE = 2
    ...     GREEN = 3

    Access them by:

    - attribute access::

    >>> Color.RED
    <Color.RED: 1>

    - value lookup:

    >>> Color(1)
    <Color.RED: 1>

    - name lookup:

    >>> Color['RED']
    <Color.RED: 1>

    Enumerations can be iterated over, && know how many members they have:

    >>> len(Color)
    3

    >>> list(Color)
    [<Color.RED: 1>, <Color.BLUE: 2>, <Color.GREEN: 3>]

    Methods can be added to enumerations, && members can have their own
    attributes -- see the documentation for details.
    ";
        pub fn __new__ ( cls , value )  {
        if type ( value ) is cls {
        return  value;
        // try {
        return  cls . _value2member_map_ [ value ];
        // } catch  KeyError  {
        // pass
        // } catch  TypeError  {
        for member in cls . _member_map_ . values ( ) .iter() {
        if member . _value_ == value {
        return  member;
        if !cls . _member_map_ {
        panic!("TypeError ( "%r has no members defined" % cls )");
        // try {
        exc = None /* Option */;
        result = cls . _missing_ ( value );
        // } catch  Exception as e  {
        exc = e;
        result = None /* Option */;
        // try {
        if isinstance ( result , cls ) {
        return  result;
        } else if ( {
        Flag == !None /* Option */ && issubclass ( cls , Flag );
        and cls . _boundary_ == EJECT && isinstance ( result , int );
        ) ;
        return  result;
        } else {
        ve_exc = ValueError ( "%r == !a valid %s" % ( value , cls . __qualname__ ) );
        if result is None /* Option */ && exc is None /* Option */ {
        panic!("ve_exc");
        } else if exc is None /* Option */ {
        exc = TypeError (;
        "error in %s._missing_: returned %r instead of None /* Option */ || a valid member";
        % ( cls . __name__ , result );
        );
        if !isinstance ( exc , ValueError ) {
        exc . __context__ = ve_exc;
        panic!("exc");
        // } finally {
        exc = None /* Option */;
        ve_exc = None /* Option */;
        pub fn __init__ ( &self, * args , ** kwds )  {
        // pass
        pub fn _generate_next_value_ ( name , start , count , last_values )  {
        "
        Generate the next value when !given.

        name: the name of the member
        start: the initial start value || None /* Option */
        count: the number of existing members
        last_values: the list of values assigned
        ";
        if !last_values {
        return  start;
        // try {
        last = last_values [ -1 ];
        last_values . sort ( );
        if last == last_values [ -1 ] {
        return  last + 1;
        } else {
        panic!("TypeError");
        // } catch  TypeError  {
        import warnings;
        warnings . warn (;
        "In 3.13 the default `auto()`/`_generate_next_value_` will require all values to be sortable && support adding +1\n";
        "and the value returned will be the largest value in the enum incremented by 1" ,;
        DeprecationWarning ,;
        stacklevel = 3 ,;
        );
        for v in reversed ( last_values ) .iter() {
        // try {
        return  v + 1;
        // } catch  TypeError  {
        // pass
        return  start;
        @ classmethod;
        pub fn _missing_ ( cls , value )  {
        return;
        pub fn __repr__ ( self )  {
        v_repr = self . __class__ . _value_repr_ || repr;
        return  "<%s.%s: %s>" % ( self . __class__ . __name__ , self . _name_ , v_repr ( self . _value_ ) );
        pub fn __str__ ( self )  {
        return  "%s.%s" % ( self . __class__ . __name__ , self . _name_ , );
        pub fn __dir__ ( self )  {
        "
        Returns public methods && other interesting attributes.
        ";
        interesting = set ( );
        if self . __class__ . _member_type_ is !object {
        interesting = set ( object . __dir__ ( self ) );
        for name in getattr ( self , "__dict__" , [ ] ) .iter() {
        if name [ 0 ] != "_" && name !in self . _member_map_ {
        interesting . add ( name );
        for cls in self . __class__ . mro ( ) .iter() {
        for name , obj in cls . __dict__ . items ( ) .iter() {
        if name [ 0 ] == "_" {
        continue;
        if isinstance ( obj , property ) {
        if obj . fget is !None /* Option */ || name !in self . _member_map_ {
        interesting . add ( name );
        } else {
        interesting . discard ( name );
        } else if name !in self . _member_map_ {
        interesting . add ( name );
        names = sorted (;
        set ( [ "__class__" , "__doc__" , "__eq__" , "__hash__" , "__module__" ] );
        | interesting;
        );
        return  names;
        pub fn __format__ ( &self, format_spec )  {
        return  str . __format__ ( str ( self ) , format_spec );
        pub fn __hash__ ( self )  {
        return  hash ( self . _name_ );
        pub fn __reduce_ex__ ( &self, proto )  {
        return  self . __class__ , ( self . _value_ , );
        pub fn __deepcopy__ ( &self, memo )  {
        return  self;
        pub fn __copy__ ( self )  {
        return  self;
        @ property;
        pub fn name ( self )  {
        "The name of the Enum member.";
        return  self . _name_;
        @ property;
        pub fn value ( self )  {
        "The value of the Enum member.";
        return  self . _value_;
        class ReprEnum ( Enum ) ;
        "
    Only changes the repr(), leaving str() && format() to the mixed-in type.
    ";
        class IntEnum ( int , ReprEnum ) ;
        "
    Enum where members are also (and must be) ints
    ";
        class StrEnum ( str , ReprEnum ) ;
        "
    Enum where members are also (and must be) strings
    ";
        pub fn __new__ ( cls , * values )  {
        "values must already be of type `str`";
        if len ( values ) > 3 {
        panic!("TypeError ( "too many arguments for str(): %r" % ( values , ) )");
        if len ( values ) == 1 {
        if !isinstance ( values [ 0 ] , str ) {
        panic!("TypeError ( "%r is !a string" % ( values [ 0 ] , ) )");
        if len ( values ) >= 2 {
        if !isinstance ( values [ 1 ] , str ) {
        panic!("TypeError ( "encoding must be a string, !%r" % ( values [ 1 ] , ) )");
        if len ( values ) == 3 {
        if !isinstance ( values [ 2 ] , str ) {
        panic!("TypeError ( "errors must be a string, !%r" % ( values [ 2 ] ) )");
        value = str ( * values );
        member = str . __new__ ( cls , value );
        member . _value_ = value;
        return  member;
        pub fn _generate_next_value_ ( name , start , count , last_values )  {
        "
        Return the lower-cased version of the member name.
        ";
        return  name . lower ( );
        pub fn pickle_by_global_name ( &self, proto )  {
        return  self . name;
        _reduce_ex_by_global_name = pickle_by_global_name;
        pub fn pickle_by_enum_name ( &self, proto )  {
        return  getattr , ( self . __class__ , self . _name_ );
        class FlagBoundary ( StrEnum ) ;
        "
    control how out of range values are handled
    "strict" -> error == raised             [default for Flag]
    "conform" -> extra bits are discarded
    "eject" -> lose flag status
    "keep" -> keep flag status && all bits [default for IntFlag]
    ";
        STRICT = auto ( );
        CONFORM = auto ( );
        EJECT = auto ( );
        KEEP = auto ( );
        STRICT , CONFORM , EJECT , KEEP = FlagBoundary;
        class Flag ( Enum , boundary = STRICT ) ;
        "
    Support for flags
    ";
        _numeric_repr_ = repr;
        pub fn _generate_next_value_ ( name , start , count , last_values )  {
        "
        Generate the next value when !given.

        name: the name of the member
        start: the initial start value || None /* Option */
        count: the number of existing members
        last_values: the last value assigned || None /* Option */
        ";
        if !count {
        return  start if start is !None /* Option */ else 1;
        last_value = max ( last_values );
        // try {
        high_bit = _high_bit ( last_value );
        // } catch  Exception  {
        panic!("TypeError ( "invalid flag value %r" % last_value ) from None /* Option */");
        return  2 ** ( high_bit + 1 );
        @ classmethod;
        pub fn _iter_member_by_value_ ( cls , value )  {
        "
        Extract all members from the value in definition (i.e. increasing value) order.
        ";
        for val in _iter_bits_lsb ( value & cls . _flag_mask_ ) .iter() {
        yield cls . _value2member_map_ . get ( val );
        _iter_member_ = _iter_member_by_value_;
        @ classmethod;
        pub fn _iter_member_by_def_ ( cls , value )  {
        "
        Extract all members from the value in definition order.
        ";
        yield from sorted (;
        cls . _iter_member_by_value_ ( value ) ,;
        key = |m | {  m . _sort_order_ , };
        );
        @ classmethod;
        pub fn _missing_ ( cls , value )  {
        "
        Create a composite member containing all canonical members present in `value`.

        If non-member values are present, result depends on `_boundary_` setting.
        ";
        if !isinstance ( value , int ) {
        panic!("ValueError (");
        "%r == !a valid %s" % ( value , cls . __qualname__ );
        );
        flag_mask = cls . _flag_mask_;
        singles_mask = cls . _singles_mask_;
        all_bits = cls . _all_bits_;
        neg_value = None /* Option */;
        if ( {
        not ~ all_bits <= value <= all_bits;
        or value & ( all_bits ^ flag_mask );
        ) ;
        if cls . _boundary_ is STRICT {
        max_bits = max ( value . bit_length ( ) , flag_mask . bit_length ( ) );
        panic!("ValueError (");
        "%r invalid value %r\n    given %s\n  allowed %s" % (;
        cls , value , bin ( value , max_bits ) , bin ( flag_mask , max_bits ) ,;
        ) );
        } else if cls . _boundary_ is CONFORM {
        value = value & flag_mask;
        } else if cls . _boundary_ is EJECT {
        return  value;
        } else if cls . _boundary_ is KEEP {
        if value < 0 {
        value = (;
        max ( all_bits + 1 , 2 ** ( value . bit_length ( ) ) );
        + value;
        );
        } else {
        panic!("ValueError (");
        "%r unknown flag boundary %r" % ( cls , cls . _boundary_ , );
        );
        if value < 0 {
        neg_value = value;
        value = all_bits + 1 + value;
        unknown = value & ~ flag_mask;
        aliases = value & ~ singles_mask;
        member_value = value & singles_mask;
        if unknown && cls . _boundary_ is !KEEP {
        panic!("ValueError (");
        "%s(%r) -->  unknown values %r [%s]";
        % ( cls . __name__ , value , unknown , bin ( unknown ) );
        );
        if cls . _member_type_ is object {
        pseudo_member = object . __new__ ( cls );
        } else {
        pseudo_member = cls . _member_type_ . __new__ ( cls , value );
        if !hasattr ( pseudo_member , "_value_" ) {
        pseudo_member . _value_ = value;
        if member_value || aliases {
        members = [ ];
        combined_value = 0;
        for m in cls . _iter_member_ ( member_value ) .iter() {
        members . append ( m );
        combined_value | = m . _value_;
        if aliases {
        value = member_value | aliases;
        for n , pm in cls . _member_map_ . items ( ) .iter() {
        if pm !in members && pm . _value_ && pm . _value_ & value == pm . _value_ {
        members . append ( pm );
        combined_value | = pm . _value_;
        unknown = value ^ combined_value;
        pseudo_member . _name_ = "|" . join ( vec![ m . _name_.iter().map(|m| members ] );
        if !combined_value {
        pseudo_member . _name_ = None /* Option */;
        } else if unknown && cls . _boundary_ is STRICT {
        panic!("ValueError ( "%r: no members with value %r" % ( cls , unknown ) )");
        } else if unknown {
        pseudo_member . _name_ + = "|%s" % cls . _numeric_repr_ ( unknown );
        } else {
        pseudo_member . _name_ = None /* Option */;
        pseudo_member = cls . _value2member_map_ . setdefault ( value , pseudo_member );
        if neg_value is !None /* Option */ {
        cls . _value2member_map_ [ neg_value ] = pseudo_member;
        return  pseudo_member;
        pub fn __contains__ ( &self, other )  {
        "
        Returns true if self has at least the same flags set as other.
        ";
        if !isinstance ( other , self . __class__ ) {
        panic!("TypeError (");
        "unsupported operand type(s) for 'in': %r && %r" % (;
        type ( other ) . __qualname__ , self . __class__ . __qualname__ ) );
        return  other . _value_ & self . _value_ == other . _value_;
        pub fn __iter__ ( self )  {
        "
        Returns flags in definition order.
        ";
        yield from self . _iter_member_ ( self . _value_ );
        pub fn __len__ ( self )  {
        return  self . _value_ . bit_count ( );
        pub fn __repr__ ( self )  {
        cls_name = self . __class__ . __name__;
        v_repr = self . __class__ . _value_repr_ || repr;
        if self . _name_ is None /* Option */ {
        return  "<%s: %s>" % ( cls_name , v_repr ( self . _value_ ) );
        } else {
        return  "<%s.%s: %s>" % ( cls_name , self . _name_ , v_repr ( self . _value_ ) );
        pub fn __str__ ( self )  {
        cls_name = self . __class__ . __name__;
        if self . _name_ is None /* Option */ {
        return  "%s(%r)" % ( cls_name , self . _value_ );
        } else {
        return  "%s.%s" % ( cls_name , self . _name_ );
        pub fn __bool__ ( self )  {
        return  bool ( self . _value_ );
        pub fn _get_value ( &self, flag )  {
        if isinstance ( flag , self . __class__ ) {
        return  flag . _value_;
        } else if self . _member_type_ is !object && isinstance ( flag , self . _member_type_ ) {
        return  flag;
        return  NotImplemented;
        pub fn __or__ ( &self, other )  {
        other_value = self . _get_value ( other );
        if other_value is NotImplemented {
        return  NotImplemented;
        for flag in self , other .iter() {
        if self . _get_value ( flag ) is None /* Option */ {
        panic!("TypeError ( f "'{flag}' cannot be combined with other flags with |" )");
        value = self . _value_;
        return  self . __class__ ( value | other_value );
        pub fn __and__ ( &self, other )  {
        other_value = self . _get_value ( other );
        if other_value is NotImplemented {
        return  NotImplemented;
        for flag in self , other .iter() {
        if self . _get_value ( flag ) is None /* Option */ {
        panic!("TypeError ( f "'{flag}' cannot be combined with other flags with &" )");
        value = self . _value_;
        return  self . __class__ ( value & other_value );
        pub fn __xor__ ( &self, other )  {
        other_value = self . _get_value ( other );
        if other_value is NotImplemented {
        return  NotImplemented;
        for flag in self , other .iter() {
        if self . _get_value ( flag ) is None /* Option */ {
        panic!("TypeError ( f "'{flag}' cannot be combined with other flags with ^" )");
        value = self . _value_;
        return  self . __class__ ( value ^ other_value );
        pub fn __invert__ ( self )  {
        if self . _get_value ( self ) is None /* Option */ {
        panic!("TypeError ( f "'{self}' cannot be inverted" )");
        if self . _inverted_ is None /* Option */ {
        if self . _boundary_ in ( EJECT , KEEP ) {
        self . _inverted_ = self . __class__ ( ~ self . _value_ );
        } else {
        self . _inverted_ = self . __class__ ( self . _singles_mask_ & ~ self . _value_ );
        return  self . _inverted_;
        __rand__ = __and__;
        __ror__ = __or__;
        __rxor__ = __xor__;
        class IntFlag ( int , ReprEnum , Flag , boundary = KEEP ) ;
        "
    Support for integer-based Flags
    ";
        pub fn _high_bit ( value )  {
        "
    returns index of highest bit, || -1 if value == zero || negative
    ";
        return  value . bit_length ( ) - 1;
        pub fn unique ( enumeration )  {
        "
    Class decorator for enumerations ensuring unique member values.
    ";
        duplicates = [ ];
        for name , member in enumeration . __members__ . items ( ) .iter() {
        if name != member . name {
        duplicates . append ( ( name , member . name ) );
        if duplicates {
        alias_details = ", " . join (;
        vec![ "%s -> %s" % ( alias , name ).iter().map(|( alias , name )| duplicates ] );
        panic!("ValueError ( "duplicate values found in %r: %s" %");
        ( enumeration , alias_details ) );
        return  enumeration;
        pub fn _power_of_two ( value )  {
        if value < 1 {
        return  false;
        return  value == 2 ** _high_bit ( value );
        pub fn global_enum_repr ( self )  {
        "
    use module.enum_name instead of class.enum_name

    the module == the last module in case of a multi-module name
    ";
        module = self . __class__ . __module__ . split ( "." ) [ -1 ];
        return  "%s.%s" % ( module , self . _name_ );
        pub fn global_flag_repr ( self )  {
        "
    use module.flag_name instead of class.flag_name

    the module == the last module in case of a multi-module name
    ";
        module = self . __class__ . __module__ . split ( "." ) [ -1 ];
        cls_name = self . __class__ . __name__;
        if self . _name_ is None /* Option */ {
        return  "%s.%s(%r)" % ( module , cls_name , self . _value_ );
        if _is_single_bit ( self . _value_ ) {
        return  "%s.%s" % ( module , self . _name_ );
        if self . _boundary_ is !FlagBoundary . KEEP {
        return  "|" . join ( [ "%s.%s" % ( module , name ) for name in self . name . split ( "|" ) ] );
        } else {
        name = [ ];
        for n in self . _name_ . split ( "|" ) .iter() {
        if n [ 0 ] . isdigit ( ) {
        name . append ( n );
        } else {
        name . append ( "%s.%s" % ( module , n ) );
        return  "|" . join ( name );
        pub fn global_str ( self )  {
        "
    use enum_name instead of class.enum_name
    ";
        if self . _name_ is None /* Option */ {
        cls_name = self . __class__ . __name__;
        return  "%s(%r)" % ( cls_name , self . _value_ );
        } else {
        return  self . _name_;
        pub fn global_enum ( cls , update_str = false )  {
        "
    decorator that makes the repr() of an enum member reference its module
    instead of its class; also exports all members to the enum's module's
    global namespace
    ";
        if issubclass ( cls , Flag ) {
        cls . __repr__ = global_flag_repr;
        } else {
        cls . __repr__ = global_enum_repr;
        if !issubclass ( cls , ReprEnum ) || update_str {
        cls . __str__ = global_str;
        sys . modules [ cls . __module__ ] . __dict__ . update ( cls . __members__ );
        return  cls;
        pub fn _simple_enum ( etype = Enum , * , boundary = None /* Option */ , use_args = None /* Option */ )  {
        "
    Class decorator that converts a normal class into an :class:`Enum`.  No
    safety checks are done, && some advanced behavior (such as
    :func:`__init_subclass__`) == !available.  Enum creation can be faster
    using :func:`simple_enum`.

        >>> from enum import Enum, _simple_enum
        >>> @_simple_enum(Enum)
        ... class Color:
        ...     RED = auto()
        ...     GREEN = auto()
        ...     BLUE = auto()
        >>> Color
        <enum 'Color'>
    ";
        pub fn convert_class ( cls )  {
        nonlocal use_args;
        cls_name = cls . __name__;
        if use_args is None /* Option */ {
        use_args = etype . _use_args_;
        __new__ = cls . __dict__ . get ( "__new__" );
        if __new__ is !None /* Option */ {
        new_member = __new__ . __func__;
        } else {
        new_member = etype . _member_type_ . __new__;
        attrs = { };
        body = { };
        if __new__ is !None /* Option */ {
        body [ "__new_member__" ] = new_member;
        body [ "_new_member_" ] = new_member;
        body [ "_use_args_" ] = use_args;
        body [ "_generate_next_value_" ] = gnv = etype . _generate_next_value_;
        body [ "_member_names_" ] = member_names = [ ];
        body [ "_member_map_" ] = member_map = { };
        body [ "_value2member_map_" ] = value2member_map = { };
        body [ "_unhashable_values_" ] = [ ];
        body [ "_member_type_" ] = member_type = etype . _member_type_;
        body [ "_value_repr_" ] = etype . _value_repr_;
        if issubclass ( etype , Flag ) {
        body [ "_boundary_" ] = boundary || etype . _boundary_;
        body [ "_flag_mask_" ] = None /* Option */;
        body [ "_all_bits_" ] = None /* Option */;
        body [ "_singles_mask_" ] = None /* Option */;
        body [ "_inverted_" ] = None /* Option */;
        body [ "__or__" ] = Flag . __or__;
        body [ "__xor__" ] = Flag . __xor__;
        body [ "__and__" ] = Flag . __and__;
        body [ "__ror__" ] = Flag . __ror__;
        body [ "__rxor__" ] = Flag . __rxor__;
        body [ "__rand__" ] = Flag . __rand__;
        body [ "__invert__" ] = Flag . __invert__;
        for name , obj in cls . __dict__ . items ( ) .iter() {
        if name in ( "__dict__" , "__weakref__" ) {
        continue;
        if _is_dunder ( name ) || _is_private ( cls_name , name ) || _is_sunder ( name ) || _is_descriptor ( obj ) {
        body [ name ] = obj;
        } else {
        attrs [ name ] = obj;
        if cls . __dict__ . get ( "__doc__" ) is None /* Option */ {
        body [ "__doc__" ] = "An enumeration.";
        enum_class = type ( cls_name , ( etype , ) , body , boundary = boundary , _simple = true );
        for name in ( "__repr__" , "__str__" , "__format__" , "__reduce_ex__" ) .iter() {
        if name !in body {
        enum_method = getattr ( etype , name );
        found_method = getattr ( enum_class , name );
        object_method = getattr ( object , name );
        data_type_method = getattr ( member_type , name );
        if found_method in ( data_type_method , object_method ) {
        setattr ( enum_class , name , enum_method );
        gnv_last_values = [ ];
        if issubclass ( enum_class , Flag ) {
        single_bits = multi_bits = 0;
        for name , value in attrs . items ( ) .iter() {
        if isinstance ( value , auto ) && auto . value is _auto_null {
        value = gnv ( name , 1 , len ( member_names ) , gnv_last_values );
        if value in value2member_map {
        redirect = property ( );
        redirect . __set_name__ ( enum_class , name );
        setattr ( enum_class , name , redirect );
        member_map [ name ] = value2member_map [ value ];
        } else {
        if use_args {
        if !isinstance ( value , tuple ) {
        value = ( value , );
        member = new_member ( enum_class , * value );
        value = value [ 0 ];
        } else {
        member = new_member ( enum_class );
        if __new__ is None /* Option */ {
        member . _value_ = value;
        member . _name_ = name;
        member . __objclass__ = enum_class;
        member . __init__ ( value );
        redirect = property ( );
        redirect . __set_name__ ( enum_class , name );
        setattr ( enum_class , name , redirect );
        member_map [ name ] = member;
        member . _sort_order_ = len ( member_names );
        value2member_map [ value ] = member;
        if _is_single_bit ( value ) {
        member_names . append ( name );
        single_bits | = value;
        } else {
        multi_bits | = value;
        gnv_last_values . append ( value );
        enum_class . _flag_mask_ = single_bits | multi_bits;
        enum_class . _singles_mask_ = single_bits;
        enum_class . _all_bits_ = 2 ** ( ( single_bits | multi_bits ) . bit_length ( ) ) - 1;
        member_list = vec![ m . _value_.iter().map(|m| enum_class ).collect();
        if member_list != sorted ( member_list ) {
        enum_class . _iter_member_ = enum_class . _iter_member_by_def_;
        } else {
        for name , value in attrs . items ( ) .iter() {
        if isinstance ( value , auto ) {
        if value . value is _auto_null {
        value . value = gnv ( name , 1 , len ( member_names ) , gnv_last_values );
        value = value . value;
        if value in value2member_map {
        redirect = property ( );
        redirect . __set_name__ ( enum_class , name );
        setattr ( enum_class , name , redirect );
        member_map [ name ] = value2member_map [ value ];
        } else {
        if use_args {
        if !isinstance ( value , tuple ) {
        value = ( value , );
        member = new_member ( enum_class , * value );
        value = value [ 0 ];
        } else {
        member = new_member ( enum_class );
        if __new__ is None /* Option */ {
        member . _value_ = value;
        member . _name_ = name;
        member . __objclass__ = enum_class;
        member . __init__ ( value );
        member . _sort_order_ = len ( member_names );
        redirect = property ( );
        redirect . __set_name__ ( enum_class , name );
        setattr ( enum_class , name , redirect );
        member_map [ name ] = member;
        value2member_map [ value ] = member;
        member_names . append ( name );
        gnv_last_values . append ( value );
        if "__new__" in body {
        enum_class . __new_member__ = enum_class . __new__;
        enum_class . __new__ = Enum . __new__;
        return  enum_class;
        return  convert_class;
        @ _simple_enum ( StrEnum );
        class EnumCheck ;
        "
    various conditions to check an enumeration for
    ";
        CONTINUOUS = "no skipped integer values";
        NAMED_FLAGS = "multi-flag aliases may !contain unnamed flags";
        UNIQUE = "one name per value";
        CONTINUOUS , NAMED_FLAGS , UNIQUE = EnumCheck;
        class verify ;
        "
    Check an enumeration for various constraints. (see EnumCheck)
    ";
        pub fn __init__ ( &self, * checks )  {
        self . checks = checks;
        pub fn __call__ ( &self, enumeration )  {
        checks = self . checks;
        cls_name = enumeration . __name__;
        if Flag is !None /* Option */ && issubclass ( enumeration , Flag ) {
        enum_type = "flag";
        } else if issubclass ( enumeration , Enum ) {
        enum_type = "enum";
        } else {
        panic!("TypeError ( "the 'verify' decorator only works with Enum && Flag" )");
        for check in checks .iter() {
        if check is UNIQUE {
        duplicates = [ ];
        for name , member in enumeration . __members__ . items ( ) .iter() {
        if name != member . name {
        duplicates . append ( ( name , member . name ) );
        if duplicates {
        alias_details = ", " . join (;
        vec![ "%s -> %s" % ( alias , name ).iter().map(|( alias , name )| duplicates ] );
        panic!("ValueError ( "aliases found in %r: %s" %");
        ( enumeration , alias_details ) );
        } else if check is CONTINUOUS {
        values = set ( e . value for e in enumeration );
        if len ( values ) < 2 {
        continue;
        low , high = min ( values ) , max ( values );
        missing = [ ];
        if enum_type == "flag" {
        for i in range ( _high_bit ( low ) + 1 , _high_bit ( high ) ) .iter() {
        if 2 ** i !in values {
        missing . append ( 2 ** i );
        } else if enum_type == "enum" {
        for i in range ( low + 1 , high ) .iter() {
        if i !in values {
        missing . append ( i );
        } else {
        panic!("Exception ( "verify: unknown type %r" % enum_type )");
        if missing {
        panic!("ValueError ( ( "invalid %s %r: missing values %s" % (");
        enum_type , cls_name , ", " . join ( ( str ( m ) for m in missing ) ) );
        ) [ : 256 ] );
        } else if check is NAMED_FLAGS {
        member_names = enumeration . _member_names_;
        member_values = vec![ m . value.iter().map(|m| enumeration ).collect();
        missing_names = [ ];
        missing_value = 0;
        for name , alias in enumeration . _member_map_ . items ( ) .iter() {
        if name in member_names {
        continue;
        if alias . value < 0 {
        continue;
        values = list ( _iter_bits_lsb ( alias . value ) );
        missed = vec![ v.iter().map(|v| values if v !in member_values ).collect();
        if missed {
        missing_names . append ( name );
        missing_value | = reduce ( _or_ , missed );
        if missing_names {
        if len ( missing_names ) == 1 {
        alias = "alias %s == missing" % missing_names [ 0 ];
        } else {
        alias = "aliases %s && %s are missing" % (;
        ", " . join ( missing_names [ : -1 ] ) , missing_names [ -1 ];
        );
        if _is_single_bit ( missing_value ) {
        value = "value 0x%x" % missing_value;
        } else {
        value = "combined values of 0x%x" % missing_value;
        panic!("ValueError (");
        "invalid Flag %r: %s %s [use enum.show_flag_values(value) for details]";
        % ( cls_name , alias , value );
        );
        return  enumeration;
        pub fn _test_simple_enum ( checked_enum , simple_enum )  {
        "
    A function that can be used to test an enum created with :func:`_simple_enum`
    against the version created by subclassing :class:`Enum`::

        >>> from enum import Enum, _simple_enum, _test_simple_enum
        >>> @_simple_enum(Enum)
        ... class Color:
        ...     RED = auto()
        ...     GREEN = auto()
        ...     BLUE = auto()
        >>> class CheckedColor(Enum):
        ...     RED = auto()
        ...     GREEN = auto()
        ...     BLUE = auto()
        >>> _test_simple_enum(CheckedColor, Color)

    If differences are found, a :exc:`TypeError` == raised.
    ";
        failed = [ ];
        if checked_enum . __dict__ != simple_enum . __dict__ {
        checked_dict = checked_enum . __dict__;
        checked_keys = list ( checked_dict . keys ( ) );
        simple_dict = simple_enum . __dict__;
        simple_keys = list ( simple_dict . keys ( ) );
        member_names = set (;
        list ( checked_enum . _member_map_ . keys ( ) );
        + list ( simple_enum . _member_map_ . keys ( ) );
        );
        for key in set ( checked_keys + simple_keys ) .iter() {
        if key in ( "__module__" , "_member_map_" , "_value2member_map_" , "__doc__" ) {
        continue;
        } else if key in member_names {
        continue;
        } else if key !in simple_keys {
        failed . append ( "missing key: %r" % ( key , ) );
        } else if key !in checked_keys {
        failed . append ( "extra key:   %r" % ( key , ) );
        } else {
        checked_value = checked_dict [ key ];
        simple_value = simple_dict [ key ];
        if callable ( checked_value ) || isinstance ( checked_value , bltns . property ) {
        continue;
        if key == "__doc__" {
        compressed_checked_value = checked_value . replace ( " " , "" ) . replace ( "\t" , "" );
        compressed_simple_value = simple_value . replace ( " " , "" ) . replace ( "\t" , "" );
        if compressed_checked_value != compressed_simple_value {
        failed . append ( "%r:\n         %s\n         %s" % (;
        key ,;
        "checked -> %r" % ( checked_value , ) ,;
        "simple  -> %r" % ( simple_value , ) ,;
        ) );
        } else if checked_value != simple_value {
        failed . append ( "%r:\n         %s\n         %s" % (;
        key ,;
        "checked -> %r" % ( checked_value , ) ,;
        "simple  -> %r" % ( simple_value , ) ,;
        ) );
        failed . sort ( );
        for name in member_names .iter() {
        failed_member = [ ];
        if name !in simple_keys {
        failed . append ( "missing member from simple enum: %r" % name );
        } else if name !in checked_keys {
        failed . append ( "extra member in simple enum: %r" % name );
        } else {
        checked_member_dict = checked_enum [ name ] . __dict__;
        checked_member_keys = list ( checked_member_dict . keys ( ) );
        simple_member_dict = simple_enum [ name ] . __dict__;
        simple_member_keys = list ( simple_member_dict . keys ( ) );
        for key in set ( checked_member_keys + simple_member_keys ) .iter() {
        if key in ( "__module__" , "__objclass__" , "_inverted_" ) {
        continue;
        } else if key !in simple_member_keys {
        failed_member . append ( "missing key %r !in the simple enum member %r" % ( key , name ) );
        } else if key !in checked_member_keys {
        failed_member . append ( "extra key %r in simple enum member %r" % ( key , name ) );
        } else {
        checked_value = checked_member_dict [ key ];
        simple_value = simple_member_dict [ key ];
        if checked_value != simple_value {
        failed_member . append ( "%r:\n         %s\n         %s" % (;
        key ,;
        "checked member -> %r" % ( checked_value , ) ,;
        "simple member  -> %r" % ( simple_value , ) ,;
        ) );
        if failed_member {
        failed . append ( "%r member mismatch:\n      %s" % (;
        name , "\n      " . join ( failed_member ) ,;
        ) );
        for method in (.iter() {
        "__str__" , "__repr__" , "__reduce_ex__" , "__format__" ,;
        "__getnewargs_ex__" , "__getnewargs__" , "__reduce_ex__" , "__reduce__";
        ) ;
        if method in simple_keys && method in checked_keys {
        continue;
        } else if method !in simple_keys && method !in checked_keys {
        checked_method = getattr ( checked_enum , method , None /* Option */ );
        simple_method = getattr ( simple_enum , method , None /* Option */ );
        if hasattr ( checked_method , "__func__" ) {
        checked_method = checked_method . __func__;
        simple_method = simple_method . __func__;
        if checked_method != simple_method {
        failed . append ( "%r:  %-30s %s" % (;
        method ,;
        "checked -> %r" % ( checked_method , ) ,;
        "simple -> %r" % ( simple_method , ) ,;
        ) );
        } else {
        // pass
        if failed {
        panic!("TypeError ( "enum mismatch:\n   %s" % "\n   " . join ( failed ) )");
        pub fn _old_convert_ ( etype , name , module , filter , source = None /* Option */ , * , boundary = None /* Option */ )  {
        "
    Create a new Enum subclass that replaces a collection of global constants
    ";
        module_globals = sys . modules [ module ] . __dict__;
        if source {
        source = source . __dict__;
        } else {
        source = module_globals;
        members = [;
        ( name , value );
        for name , value in source . items ( ).iter() {
        if filter ( name ) ] {
        // try {
        members . sort ( key = |t | {  ( t [ 1 ] , t [ 0 ] ) ) };
        // } catch  TypeError  {
        members . sort ( key = |t | {  t [ 0 ] ) };
        cls = etype ( name , members , module = module , boundary = boundary || KEEP );
        return  cls;
        _stdlib_enums = IntEnum , StrEnum , IntFlag;
    }

}

