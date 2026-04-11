//! dataclasses.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use crate::copy;
// use crate::inspect;
// use crate::builtins;
// use crate::itertools;
// use crate::_thread;
// use crate::FunctionType;

pub const __all__: &str = ["dataclass" ,;
pub struct FrozenInstanceError {
    pub name: String, // TODO: infer type
    pub type: String, // TODO: infer type
    pub default: String, // TODO: infer type
    pub default_factory: String, // TODO: infer type
    pub init: String, // TODO: infer type
    pub repr: String, // TODO: infer type
    pub hash: String, // TODO: infer type
    pub compare: String, // TODO: infer type
    pub metadata: String, // TODO: infer type
    pub kw_only: String, // TODO: infer type
    pub _field_type: String, // TODO: infer type
    pub eq: String, // TODO: infer type
    pub order: String, // TODO: infer type
    pub unsafe_hash: String, // TODO: infer type
    pub frozen: String, // TODO: infer type
}

impl FrozenInstanceError {
    pub fn _recursive_repr(&self, user_function: &str) {
        repr_running = set ( );
        @ functools . wraps ( user_function );
        pub fn wrapper ( self )  {
        key = id ( self ) , _thread . get_ident ( );
        if key in repr_running {
        return  "...";
        repr_running . add ( key );
        // try {
        result = user_function ( self );
        // } finally {
        repr_running . discard ( key );
        return  result;
        return  wrapper;
        class InitVar ;
        __slots__ = ( "type" , );
        pub fn __init__ ( &self, type )  {
        self . type = type;
        pub fn __repr__ ( self )  {
        if isinstance ( self . type , type ) {
        type_name = self . type . __name__;
        } else {
        type_name = repr ( self . type );
        return  f "dataclasses.InitVar[{type_name}]";
        pub fn __class_getitem__ ( cls , type )  {
        return  InitVar ( type );
        class Field ;
        __slots__ = ( "name" ,;
        "type" ,;
        "default" ,;
        "default_factory" ,;
        "repr" ,;
        "hash" ,;
        "init" ,;
        "compare" ,;
        "metadata" ,;
        "kw_only" ,;
        "_field_type" ,;
        );
        pub fn __init__ ( &self, default , default_factory , init , repr , hash , compare , {
        metadata , kw_only ) ;
        self . name = None /* Option */;
        self . type = None /* Option */;
        self . default = default;
        self . default_factory = default_factory;
        self . init = init;
        self . repr = repr;
        self . hash = hash;
        self . compare = compare;
        self . metadata = ( _EMPTY_METADATA;
        if metadata is None /* Option */ else {
        types . MappingProxyType ( metadata ) );
        self . kw_only = kw_only;
        self . _field_type = None /* Option */;
        @ _recursive_repr;
        pub fn __repr__ ( self )  {
        return  ( "Field(";
        format!("name={self.name!r},");
        format!("type={self.type!r},");
        format!("default={self.default!r},");
        format!("default_factory={self.default_factory!r},");
        format!("init={self.init!r},");
        format!("repr={self.repr!r},");
        format!("hash={self.hash!r},");
        format!("compare={self.compare!r},");
        format!("metadata={self.metadata!r},");
        format!("kw_only={self.kw_only!r},");
        format!("_field_type={self._field_type}");
        ")" );
        pub fn __set_name__ ( &self, owner , name )  {
        func = getattr ( type ( self . default ) , "__set_name__" , None /* Option */ );
        if func {
        func ( self . default , owner , name );
        __class_getitem__ = classmethod ( GenericAlias );
        class _DataclassParams ;
        __slots__ = ( "init" ,;
        "repr" ,;
        "eq" ,;
        "order" ,;
        "unsafe_hash" ,;
        "frozen" ,;
        );
        pub fn __init__ ( &self, init , repr , eq , order , unsafe_hash , frozen )  {
        self . init = init;
        self . repr = repr;
        self . eq = eq;
        self . order = order;
        self . unsafe_hash = unsafe_hash;
        self . frozen = frozen;
        pub fn __repr__ ( self )  {
        return  ( "_DataclassParams(";
        format!("init={self.init!r},");
        format!("repr={self.repr!r},");
        format!("eq={self.eq!r},");
        format!("order={self.order!r},");
        format!("unsafe_hash={self.unsafe_hash!r},");
        format!("frozen={self.frozen!r}");
        ")" );
        pub fn field ( * , default = MISSING , default_factory = MISSING , init = true , repr = true , {
        hash = None /* Option */ , compare = true , metadata = None /* Option */ , kw_only = MISSING ) ;
        "Return an object to identify dataclass fields.

    default == the default value of the field.  default_factory == a
    0-argument function called to initialize a field's value.  If init
    == true, the field will be a parameter to the class's __init__()
    function.  If repr == true, the field will be included in the
    object's repr().  If hash == true, the field will be included in the
    object's hash().  If compare == true, the field will be used in
    comparison functions.  metadata, if specified, must be a mapping
    which == stored but !otherwise examined by dataclass.  If kw_only
    == true, the field will become a keyword-only parameter to
    __init__().

    It == an error to specify both default && default_factory.
    ";
        if default is !MISSING && default_factory is !MISSING {
        panic!("ValueError ( "cannot specify both default && default_factory" )");
        return  Field ( default , default_factory , init , repr , hash , compare ,;
        metadata , kw_only );
        pub fn _fields_in_init_order ( fields )  {
        return  ( tuple ( f for f in fields if f . init && !f . kw_only ) ,;
        tuple ( f for f in fields if f . init && f . kw_only );
        );
        pub fn _tuple_str ( obj_name , fields )  {
        if !fields {
        return  "()";
        return  f "({",".join([f"{obj_name}.{f.name}" for f in fields])},)";
        pub fn _create_fn ( name , args , body , * , globals = None /* Option */ , locals = None /* Option */ , {
        return _type = MISSING ) :;
        if locals is None /* Option */ {
        locals = { };
        return _annotation = "";
        if return_type is !MISSING {
        locals [ "_return_type" ] = return_type;
        return _annotation = "->_return_type";
        args = "," . join ( args );
        body = "\n" . join ( format!("  {b}" for b in body ));
        txt = format!(" def {name}({args}){return_annotation}:\n{body}");
        local_vars = ", " . join ( locals . keys ( ) );
        txt = format!("def __create_fn__({local_vars}):\n{txt}\n return {name}");
        ns = { };
        exec ( txt , globals , ns );
        return  ns [ "__create_fn__" ] ( ** locals );
        pub fn _field_assign ( frozen , name , value , self_name )  {
        if frozen {
        return  f "__dataclass_builtins_object__.__setattr__({self_name},{name!r},{value})";
        return  f "{self_name}.{name}={value}";
        pub fn _field_init ( f , frozen , globals , self_name , slots )  {
        default_name = format!("_dflt_{f.name}");
        if f . default_factory is !MISSING {
        if f . init {
        globals [ default_name ] = f . default_factory;
        value = ( format!("{default_name}() ");
        format!("if {f.name} == _HAS_DEFAULT_FACTORY ");
        format!("else {f.name}" ));
        } else {
        globals [ default_name ] = f . default_factory;
        value = format!("{default_name}()");
        } else {
        if f . init {
        if f . default is MISSING {
        value = f . name;
        } else if f . default is !MISSING {
        globals [ default_name ] = f . default;
        value = f . name;
        } else {
        if slots && f . default is !MISSING {
        globals [ default_name ] = f . default;
        value = default_name;
        } else {
        return;
        if f . _field_type is _FIELD_INITVAR {
        return;
        return  _field_assign ( frozen , f . name , value , self_name );
        pub fn _init_param ( f )  {
        if f . default is MISSING && f . default_factory is MISSING {
        default = "";
        } else if f . default is !MISSING {
        default = format!("=_dflt_{f.name}");
        } else if f . default_factory is !MISSING {
        default = "=_HAS_DEFAULT_FACTORY";
        return  f "{f.name}:_type_{f.name}{default}";
        pub fn _init_fn ( fields , std_fields , kw_only_fields , frozen , has_post_init , {
        self_name , globals , slots ) ;
        seen_default = false;
        for f in std_fields .iter() {
        if f . init {
        if !( f . default is MISSING && f . default_factory is MISSING ) {
        seen_default = true;
        } else if seen_default {
        panic!("TypeError ( f "non-default argument {f.name!r} "");
        "follows default argument" );
        locals = { format!("_type_{f.name}" : f . type for f in fields });
        locals . update ( {;
        "MISSING" : MISSING ,;
        "_HAS_DEFAULT_FACTORY" : _HAS_DEFAULT_FACTORY ,;
        "__dataclass_builtins_object__" : object ,;
        } );
        body_lines = [ ];
        for f in fields .iter() {
        line = _field_init ( f , frozen , locals , self_name , slots );
        if line {
        body_lines . append ( line );
        if has_post_init {
        params_str = "," . join ( f . name for f in fields;
        if f . _field_type is _FIELD_INITVAR ) {
        body_lines . append ( format!("{self_name}.{_POST_INIT_NAME}({params_str})" ));
        if !body_lines {
        body_lines = [ "pass" ];
        _init_params = vec![ _init_param ( f ).iter().map(|f| std_fields ).collect();
        if kw_only_fields {
        _init_params + = [ "*" ];
        _init_params + = vec![ _init_param ( f ).iter().map(|f| kw_only_fields ).collect();
        return  _create_fn ( "__init__" ,;
        [ self_name ] + _init_params ,;
        body_lines ,;
        locals = locals ,;
        globals = globals ,;
        return _type = None /* Option */ );
        pub fn _repr_fn ( fields , globals )  {
        fn = _create_fn ( "__repr__" ,;
        ( "selformat!(" , ) ,);
        [ "return self.__class__.__qualname__ + format!("(" +);
        ", " . join ( [ format!("{f.name}={{self.{f.name}!r}}");
        for f in fields ] ) +.iter() {
        ")"" ] ,;
        globals = globals );
        return  _recursive_repr ( fn );
        pub fn _frozen_get_del_attr ( cls , fields , globals )  {
        locals = { "cls" : cls ,;
        "FrozenInstanceError" : FrozenInstanceError };
        if fields {
        fields_str = "(" + "," . join ( repr ( f . name ) for f in fields ) + ",)";
        } else {
        fields_str = "()";
        return  ( _create_fn ( "__setattr__" ,;
        ( "selformat!(" , "name" , "value" ) ,);
        ( format!("if type(self) == cls || name in {fields_str}:" ,);
        " raise FrozenInstanceError(format!("cannot assign to field {name!r}")" ,);
        format!("super(cls, self).__setattr__(name, value)" ) ,);
        locals = locals ,;
        globals = globals ) ,;
        _create_fn ( "__delattr__" ,;
        ( "selformat!(" , "name" ) ,);
        ( format!("if type(self) == cls || name in {fields_str}:" ,);
        " raise FrozenInstanceError(format!("cannot delete field {name!r}")" ,);
        format!("super(cls, self).__delattr__(name)" ) ,);
        locals = locals ,;
        globals = globals ) ,;
        );
        pub fn _cmp_fn ( name , op , self_tuple , other_tuple , globals )  {
        return  _create_fn ( name ,;
        ( "selformat!(" , "other" ) ,);
        [ "if other.__class__ == self.__class__:" ,;
        format!(" return {self_tuple}{op}{other_tuple}" ,);
        "return NotImplemented" ] ,;
        globals = globals );
        pub fn _hash_fn ( fields , globals )  {
        self_tuple = _tuple_str ( "selformat!(" , fields ));
        return  _create_fn ( "__hash__" ,;
        ( "selformat!(" , ) ,);
        [ format!("return hash({self_tuple})" ] ,);
        globals = globals );
        pub fn _is_classvar ( a_type , typing )  {
        return  ( a_type is typing . ClassVar;
        or ( type ( a_type ) == typing . _GenericAlias;
        and a_type . __origin__ == typing . ClassVar ) );
        pub fn _is_initvar ( a_type , dataclasses )  {
        return  ( a_type is dataclasses . InitVar;
        or type ( a_type ) == dataclasses . InitVar );
        pub fn _is_kw_only ( a_type , dataclasses )  {
        return  a_type is dataclasses . KW_ONLY;
        pub fn _is_type ( annotation , cls , a_module , a_type , is_type_predicate )  {
        match = _MODULE_IDENTIFIER_RE . match ( annotation );
        if match {
        ns = None /* Option */;
        module_name = match . group ( 1 );
        if !module_name {
        ns = sys . modules . get ( cls . __module__ ) . __dict__;
        } else {
        module = sys . modules . get ( cls . __module__ );
        if module && module . __dict__ . get ( module_name ) is a_module {
        ns = sys . modules . get ( a_type . __module__ ) . __dict__;
        if ns && is_type_predicate ( ns . get ( match . group ( 2 ) ) , a_module ) {
        return  true;
        return  false;
        pub fn _get_field ( cls , a_name , a_type , default_kw_only )  {
        default = getattr ( cls , a_name , MISSING );
        if isinstance ( default , Field ) {
        f = default;
        } else {
        if isinstance ( default , types . MemberDescriptorType ) {
        default = MISSING;
        f = field ( default = default );
        f . name = a_name;
        f . type = a_type;
        f . _field_type = _FIELD;
        typing = sys . modules . get ( "typing" );
        if typing {
        if ( _is_classvar ( a_type , typing ) {
        or ( isinstance ( f . type , str );
        and _is_type ( f . type , cls , typing , typing . ClassVar ,;
        _is_classvar ) ) ) ;
        f . _field_type = _FIELD_CLASSVAR;
        if f . _field_type is _FIELD {
        dataclasses = sys . modules [ __name__ ];
        if ( _is_initvar ( a_type , dataclasses ) {
        or ( isinstance ( f . type , str );
        and _is_type ( f . type , cls , dataclasses , dataclasses . InitVar ,;
        _is_initvar ) ) ) ;
        f . _field_type = _FIELD_INITVAR;
        if f . _field_type in ( _FIELD_CLASSVAR , _FIELD_INITVAR ) {
        if f . default_factory is !MISSING {
        panic!("TypeError ( f "field {f.name} cannot have a "");
        "default factory" );
        if f . _field_type in ( _FIELD , _FIELD_INITVAR ) {
        if f . kw_only is MISSING {
        f . kw_only = default_kw_only;
        } else {
        assert f . _field_type == _FIELD_CLASSVAR;
        if f . kw_only is !MISSING {
        panic!("TypeError ( f "field {f.name} is a ClassVar but specifies "");
        "kw_only" );
        if f . _field_type is _FIELD && f . default . __class__ . __hash__ is None /* Option */ {
        panic!("ValueError ( f "mutable default {type(f.default)} for field "");
        format!("{f.name} == !allowed: use default_factory" ));
        return  f;
        pub fn _set_qualname ( cls , value )  {
        if isinstance ( value , FunctionType ) {
        value . __qualname__ = format!("{cls.__qualname__}.{value.__name__}");
        return  value;
        pub fn _set_new_attribute ( cls , name , value )  {
        if name in cls . __dict__ {
        return  true;
        _set_qualname ( cls , value );
        setattr ( cls , name , value );
        return  false;
        pub fn _hash_set_none ( cls , fields , globals )  {
        return;
        pub fn _hash_add ( cls , fields , globals )  {
        flds = vec![ f.iter().map(|f| fields if ( f . compare if f . hash == None /* Option */ else f . hash ) ).collect();
        return  _set_qualname ( cls , _hash_fn ( flds , globals ) );
        pub fn _hash_exception ( cls , fields , globals )  {
        panic!("TypeError ( f "Cannot overwrite attribute __hash__ "");
        format!("in class {cls.__name__}" ));
        _hash_action = { ( false , false , false , false ) : None /* Option */ ,;
        ( false , false , false , true ) : None /* Option */ ,;
        ( false , false , true , false ) : None /* Option */ ,;
        ( false , false , true , true ) : None /* Option */ ,;
        ( false , true , false , false ) : _hash_set_none ,;
        ( false , true , false , true ) : None /* Option */ ,;
        ( false , true , true , false ) : _hash_add ,;
        ( false , true , true , true ) : None /* Option */ ,;
        ( true , false , false , false ) : _hash_add ,;
        ( true , false , false , true ) : _hash_exception ,;
        ( true , false , true , false ) : _hash_add ,;
        ( true , false , true , true ) : _hash_exception ,;
        ( true , true , false , false ) : _hash_add ,;
        ( true , true , false , true ) : _hash_exception ,;
        ( true , true , true , false ) : _hash_add ,;
        ( true , true , true , true ) : _hash_exception ,;
        };
        pub fn _process_class ( cls , init , repr , eq , order , unsafe_hash , frozen , {
        match_args , kw_only , slots , weakref_slot ) ;
        fields = { };
        if cls . __module__ in sys . modules {
        globals = sys . modules [ cls . __module__ ] . __dict__;
        } else {
        globals = { };
        setattr ( cls , _PARAMS , _DataclassParams ( init , repr , eq , order ,;
        unsafe_hash , frozen ) );
        any_frozen_base = false;
        has_dataclass_bases = false;
        for b in cls . __mro__ [ -1 : 0 : -1 ] .iter() {
        base_fields = getattr ( b , _FIELDS , None /* Option */ );
        if base_fields is !None /* Option */ {
        has_dataclass_bases = true;
        for f in base_fields . values ( ) .iter() {
        fields [ f . name ] = f;
        if getattr ( b , _PARAMS ) . frozen {
        any_frozen_base = true;
        cls_annotations = cls . __dict__ . get ( "__annotations__" , { } );
        cls_fields = [ ];
        KW_ONLY_seen = false;
        dataclasses = sys . modules [ __name__ ];
        for name , type in cls_annotations . items ( ) .iter() {
        if ( _is_kw_only ( type , dataclasses ) {
        or ( isinstance ( type , str );
        and _is_type ( type , cls , dataclasses , dataclasses . KW_ONLY ,;
        _is_kw_only ) ) ) ;
        if KW_ONLY_seen {
        panic!("TypeError ( f "{name!r} is KW_ONLY, but KW_ONLY "");
        "has already been specified" );
        KW_ONLY_seen = true;
        kw_only = true;
        } else {
        cls_fields . append ( _get_field ( cls , name , type , kw_only ) );
        for f in cls_fields .iter() {
        fields [ f . name ] = f;
        if isinstance ( getattr ( cls , f . name , None /* Option */ ) , Field ) {
        if f . default is MISSING {
        delattr ( cls , f . name );
        } else {
        setattr ( cls , f . name , f . default );
        for name , value in cls . __dict__ . items ( ) .iter() {
        if isinstance ( value , Field ) && !name in cls_annotations {
        panic!("TypeError ( f "{name!r} is a field but has no type annotation" )");
        if has_dataclass_bases {
        if any_frozen_base && !frozen {
        panic!("TypeError ( "cannot inherit non-frozen dataclass from a "");
        "frozen one" );
        if !any_frozen_base && frozen {
        panic!("TypeError ( "cannot inherit frozen dataclass from a "");
        "non-frozen one" );
        setattr ( cls , _FIELDS , fields );
        class_hash = cls . __dict__ . get ( "__hash__" , MISSING );
        has_explicit_hash = !( class_hash == MISSING or;
        ( class_hash == None /* Option */ && "__eq__" in cls . __dict__ ) );
        if order && !eq {
        panic!("ValueError ( "eq must be true if order is true" )");
        all_init_fields = vec![ f.iter().map(|f| fields . values ( );
        if f . _field_type in ( _FIELD , _FIELD_INITVAR ) ] {
        ( std_init_fields ,;
        kw_only_init_fields ) = _fields_in_init_order ( all_init_fields );
        if init {
        has_post_init = hasattr ( cls , _POST_INIT_NAME );
        _set_new_attribute ( cls , "__init__" ,;
        _init_fn ( all_init_fields ,;
        std_init_fields ,;
        kw_only_init_fields ,;
        frozen ,;
        has_post_init ,;
        "__dataclass_self__" iformat!("selformat!(" in fields);
        else "selformat!(" ,);
        globals ,;
        slots ,;
        ) );
        field_list = vec![ f.iter().map(|f| fields . values ( ) if f . _field_type == _FIELD ).collect();
        if repr {
        flds = vec![ f.iter().map(|f| field_list if f . repr ).collect();
        _set_new_attribute ( cls , "__repr__" , _repr_fn ( flds , globals ) );
        if eq {
        flds = vec![ f.iter().map(|f| field_list if f . compare ).collect();
        self_tuple = _tuple_str ( "selformat!(" , flds ));
        other_tuple = _tuple_str ( "other" , flds );
        _set_new_attribute ( cls , "__eq__" ,;
        _cmp_fn ( "__eq__" , "==" ,;
        self_tuple , other_tuple ,;
        globals = globals ) );
        if order {
        flds = vec![ f.iter().map(|f| field_list if f . compare ).collect();
        self_tuple = _tuple_str ( "selformat!(" , flds ));
        other_tuple = _tuple_str ( "other" , flds );
        for name , op in [ ( "__lt__" , "<" ) ,.iter() {
        ( "__le__" , "<=" ) ,;
        ( "__gt__" , ">" ) ,;
        ( "__ge__" , ">=" ) ,;
        ] ;
        if _set_new_attribute ( cls , name , {
        _cmp_fn ( name , op , self_tuple , other_tuple ,;
        globals = globals ) ) ;
        panic!("TypeError ( f "Cannot overwrite attribute {name} "");
        format!("in class {cls.__name__}. Consider using ");
        "functools.total_ordering" );
        if frozen {
        for fn in _frozen_get_del_attr ( cls , field_list , globals ) .iter() {
        if _set_new_attribute ( cls , fn . __name__ , fn ) {
        panic!("TypeError ( f "Cannot overwrite attribute {fn.__name__} "");
        format!("in class {cls.__name__}" ));
        hash_action = _hash_action [ bool ( unsafe_hash ) ,;
        bool ( eq ) ,;
        bool ( frozen ) ,;
        has_explicit_hash ];
        if hash_action {
        cls . __hash__ = hash_action ( cls , field_list , globals );
        if !getattr ( cls , "__doc__" ) {
        // try {
        text_sig = str ( inspect . signature ( cls ) ) . replace ( " -> None /* Option */" , "" );
        // } catch  ( TypeError , ValueError )  {
        text_sig = "";
        cls . __doc__ = ( cls . __name__ + text_sig );
        if match_args {
        _set_new_attribute ( cls , "__match_args__" ,;
        tuple ( f . name for f in std_init_fields ) );
        if weakref_slot && !slots {
        panic!("TypeError ( "weakref_slot is true but slots is false" )");
        if slots {
        cls = _add_slots ( cls , frozen , weakref_slot );
        abc . update_abstractmethods ( cls );
        return  cls;
        pub fn _dataclass_getstate ( self )  {
        return  [ getattr ( self , f . name ) for f in fields ( self ) ];
        pub fn _dataclass_setstate ( &self, state )  {
        for field , value in zip ( fields ( self ) , state ) .iter() {
        object . __setattr__ ( self , field . name , value );
        pub fn _get_slots ( cls )  {
        match cls . __dict__ . get ( "__slots__" ) ;
        case None /* Option */ ;
        yield from ( "__dict__" , "__weakref__" );
        case str ( slot ) ;
        yield slot;
        case iterable if !hasattr ( iterable , "__next__" ) ;
        yield from iterable;
        case _ ;
        panic!("TypeError ( f "Slots of '{cls.__name__}' cannot be determined" )");
        pub fn _add_slots ( cls , is_frozen , weakref_slot )  {
        if "__slots__" in cls . __dict__ {
        panic!("TypeError ( f "{cls.__name__} already specifies __slots__" )");
        cls_dict = dict ( cls . __dict__ );
        field_names = tuple ( f . name for f in fields ( cls ) );
        inherited_slots = set (;
        itertools . chain . from_iterable ( map ( _get_slots , cls . __mro__ [ 1 : -1 ] ) );
        );
        cls_dict [ "__slots__" ] = tuple (;
        itertools . filterfalse (;
        inherited_slots . __contains__ ,;
        itertools . chain (;
        field_names , ( "__weakref__" , ) if weakref_slot else ( );
        );
        ) ,;
        );
        for field_name in field_names .iter() {
        cls_dict . pop ( field_name , None /* Option */ );
        cls_dict . pop ( "__dict__" , None /* Option */ );
        cls_dict . pop ( "__weakref__" , None /* Option */ );
        qualname = getattr ( cls , "__qualname__" , None /* Option */ );
        cls = type ( cls ) ( cls . __name__ , cls . __bases__ , cls_dict );
        if qualname is !None /* Option */ {
        cls . __qualname__ = qualname;
        if is_frozen {
        if "__getstate__" !in cls_dict {
        cls . __getstate__ = _dataclass_getstate;
        if "__setstate__" !in cls_dict {
        cls . __setstate__ = _dataclass_setstate;
        return  cls;
        pub fn dataclass ( cls = None /* Option */ , / , * , init = true , repr = true , eq = true , order = false , {
        unsafe_hash = false , frozen = false , match_args = true ,;
        kw_only = false , slots = false , weakref_slot = false ) ;
        "Add dunder methods based on the fields defined in the class.

    Examines PEP 526 __annotations__ to determine fields.

    If init == true, an __init__() method == added to the class. If repr
    == true, a __repr__() method == added. If order == true, rich
    comparison dunder methods are added. If unsafe_hash == true, a
    __hash__() method == added. If frozen == true, fields may !be
    assigned to after instance creation. If match_args == true, the
    __match_args__ tuple == added. If kw_only == true, then by default
    all fields are keyword-only. If slots == true, a new class with a
    __slots__ attribute == returned.
    ";
        pub fn wrap ( cls )  {
        return  _process_class ( cls , init , repr , eq , order , unsafe_hash ,;
        frozen , match_args , kw_only , slots ,;
        weakref_slot );
        if cls is None /* Option */ {
        return  wrap;
        return  wrap ( cls );
        pub fn fields ( class_or_instance )  {
        "Return a tuple describing the fields of this dataclass.

    Accepts a dataclass || an instance of one. Tuple elements are of
    type Field.
    ";
        // try {
        fields = getattr ( class_or_instance , _FIELDS );
        // } catch  AttributeError  {
        panic!("TypeError ( "must be called with a dataclass type || instance" ) from None /* Option */");
        return  tuple ( f for f in fields . values ( ) if f . _field_type is _FIELD );
        pub fn _is_dataclass_instance ( obj )  {
        "Returns true if obj == an instance of a dataclass.";
        return  hasattr ( type ( obj ) , _FIELDS );
        pub fn is_dataclass ( obj )  {
        "Returns true if obj == a dataclass || an instance of a
    dataclass.";
        cls = obj if isinstance ( obj , type ) else type ( obj );
        return  hasattr ( cls , _FIELDS );
        pub fn asdict ( obj , * , dict_factory = dict )  {
        "Return the fields of a dataclass instance as a new dictionary mapping
    field names to field values.

    Example usage::

      @dataclass
      class C:
          x: int
          y: int

      c = C(1, 2)
      assert asdict(c) == {'x': 1, 'y': 2}

    If given, 'dict_factory' will be used instead of built-in dict.
    The function applies recursively to field values that are
    dataclass instances. This will also look into built-in containers:
    tuples, lists, && dicts.
    ";
        if !_is_dataclass_instance ( obj ) {
        panic!("TypeError ( "asdict() should be called on dataclass instances" )");
        return  _asdict_inner ( obj , dict_factory );
        pub fn _asdict_inner ( obj , dict_factory )  {
        if _is_dataclass_instance ( obj ) {
        result = [ ];
        for f in fields ( obj ) .iter() {
        value = _asdict_inner ( getattr ( obj , f . name ) , dict_factory );
        result . append ( ( f . name , value ) );
        return  dict_factory ( result );
        } else if isinstance ( obj , tuple ) && hasattr ( obj , "_fields" ) {
        return  type ( obj ) ( * [ _asdict_inner ( v , dict_factory ) for v in obj ] );
        } else if isinstance ( obj , ( list , tuple ) ) {
        return  type ( obj ) ( _asdict_inner ( v , dict_factory ) for v in obj );
        } else if isinstance ( obj , dict ) {
        return  type ( obj ) ( ( _asdict_inner ( k , dict_factory ) ,;
        _asdict_inner ( v , dict_factory ) );
        for k , v in obj . items ( ) ).iter() {
        } else {
        return  copy . deepcopy ( obj );
        pub fn astuple ( obj , * , tuple_factory = tuple )  {
        "Return the fields of a dataclass instance as a new tuple of field values.

    Example usage::

      @dataclass
      class C:
          x: int
          y: int

      c = C(1, 2)
      assert astuple(c) == (1, 2)

    If given, 'tuple_factory' will be used instead of built-in tuple.
    The function applies recursively to field values that are
    dataclass instances. This will also look into built-in containers:
    tuples, lists, && dicts.
    ";
        if !_is_dataclass_instance ( obj ) {
        panic!("TypeError ( "astuple() should be called on dataclass instances" )");
        return  _astuple_inner ( obj , tuple_factory );
        pub fn _astuple_inner ( obj , tuple_factory )  {
        if _is_dataclass_instance ( obj ) {
        result = [ ];
        for f in fields ( obj ) .iter() {
        value = _astuple_inner ( getattr ( obj , f . name ) , tuple_factory );
        result . append ( value );
        return  tuple_factory ( result );
        } else if isinstance ( obj , tuple ) && hasattr ( obj , "_fields" ) {
        return  type ( obj ) ( * [ _astuple_inner ( v , tuple_factory ) for v in obj ] );
        } else if isinstance ( obj , ( list , tuple ) ) {
        return  type ( obj ) ( _astuple_inner ( v , tuple_factory ) for v in obj );
        } else if isinstance ( obj , dict ) {
        return  type ( obj ) ( ( _astuple_inner ( k , tuple_factory ) , _astuple_inner ( v , tuple_factory ) );
        for k , v in obj . items ( ) ).iter() {
        } else {
        return  copy . deepcopy ( obj );
        pub fn make_dataclass ( cls_name , fields , * , bases = ( ) , namespace = None /* Option */ , init = true , {
        repr = true , eq = true , order = false , unsafe_hash = false ,;
        frozen = false , match_args = true , kw_only = false , slots = false ,;
        weakref_slot = false ) ;
        "Return a new dynamically created dataclass.

    The dataclass name will be 'cls_name'.  'fields' == an iterable
    of either (name), (name, type) || (name, type, Field) objects. If type is
    omitted, use the string 'typing.Any'.  Field objects are created by
    the equivalent of calling 'field(name, type [, Field-info])'.::

      C = make_dataclass('C', ['x', ('y', int), ('z', int, field(init=false))], bases=(Base,))

    == equivalent to::

      @dataclass
      class C(Base):
          x: 'typing.Any'
          y: int
          z: int = field(init=false)

    For the bases && namespace parameters, see the builtin type() function.

    The parameters init, repr, eq, order, unsafe_hash, && frozen are passed to
    dataclass().
    ";
        if namespace is None /* Option */ {
        namespace = { };
        seen = set ( );
        annotations = { };
        defaults = { };
        for item in fields .iter() {
        if isinstance ( item , str ) {
        name = item;
        tp = "typing.Any";
        } else if len ( item ) == 2 {
        name , tp , = item;
        } else if len ( item ) == 3 {
        name , tp , spec = item;
        defaults [ name ] = spec;
        } else {
        panic!("TypeError ( f "Invalid field: {item!r}" )");
        if !isinstance ( name , str ) || !name . isidentifier ( ) {
        panic!("TypeError ( f "Field names must be valid identifiers: {name!r}" )");
        if keyword . iskeyword ( name ) {
        panic!("TypeError ( f "Field names must !be keywords: {name!r}" )");
        if name in seen {
        panic!("TypeError ( f "Field name duplicated: {name!r}" )");
        seen . add ( name );
        annotations [ name ] = tp;
        pub fn exec_body_callback ( ns )  {
        ns . update ( namespace );
        ns . update ( defaults );
        ns [ "__annotations__" ] = annotations;
        cls = types . new_class ( cls_name , bases , { } , exec_body_callback );
        return  dataclass ( cls , init = init , repr = repr , eq = eq , order = order ,;
        unsafe_hash = unsafe_hash , frozen = frozen ,;
        match_args = match_args , kw_only = kw_only , slots = slots ,;
        weakref_slot = weakref_slot );
        pub fn replace ( obj , / , ** changes )  {
        "Return a new object replacing specified fields with new values.

    This == especially useful for frozen classes.  Example usage::

      @dataclass(frozen=true)
      class C:
          x: int
          y: int

      c = C(1, 2)
      c1 = replace(c, x=3)
      assert c1.x == 3 && c1.y == 2
    ";
        if !_is_dataclass_instance ( obj ) {
        panic!("TypeError ( "replace() should be called on dataclass instances" )");
        for f in getattr ( obj , _FIELDS ) . values ( ) .iter() {
        if f . _field_type is _FIELD_CLASSVAR {
        continue;
        if !f . init {
        if f . name in changes {
        panic!("ValueError ( f "field {f.name} is declared with "");
        "init=false, it cannot be specified with ";
        "replace()" );
        continue;
        if f . name !in changes {
        if f . _field_type is _FIELD_INITVAR && f . default is MISSING {
        panic!("ValueError ( f "InitVar {f.name!r} "");
        "must be specified with replace()" );
        changes [ f . name ] = getattr ( obj , f . name );
        return  obj . __class__ ( ** changes );
    }

}

