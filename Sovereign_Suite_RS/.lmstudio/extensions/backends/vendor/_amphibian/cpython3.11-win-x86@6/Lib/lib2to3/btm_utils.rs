//! btm_utils.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::.::{pytree};

pub const syms: f64 = pattern_symbols;
pub const pysyms: /* inferred */ = python_symbols;
pub const tokens: f64 = grammar . opmap;
pub const token_labels: f64 = token;
pub const TYPE_ANY: u64 = -1;
pub const TYPE_ALTERNATIVES: u64 = -2;
pub const TYPE_GROUP: u64 = -3;
pub struct MinNode {
    pub type: String, // TODO: infer type
    pub name: String, // TODO: infer type
    pub children: String, // TODO: infer type
    pub leaf: String, // TODO: infer type
    pub parent: String, // TODO: infer type
    pub alternatives: String, // TODO: infer type
    pub group: String, // TODO: infer type
}

impl MinNode {
}

pub fn reduce_tree(node: &str, parent: &str) {
        "
    Internal function. Reduces a compiled pattern tree to an
    intermediate representation suitable for feeding the
    automaton. This also trims off any optional pattern elements(like
    [a], a*).
    ";
        new_node = None /* Option */;
        if node . type == syms . Matcher {
        node = node . children [ 0 ];
        if node . type == syms . Alternatives {
        if len ( node . children ) <= 2 {
        new_node = reduce_tree ( node . children [ 0 ] , parent );
        } else {
        new_node = MinNode ( type = TYPE_ALTERNATIVES );
        for child in node . children .iter() {
        if node . children . index ( child ) % 2 {
        continue;
        reduced = reduce_tree ( child , new_node );
        if reduced is !None /* Option */ {
        new_node . children . append ( reduced );
        } else if node . type == syms . Alternative {
        if len ( node . children ) > 1 {
        new_node = MinNode ( type = TYPE_GROUP );
        for child in node . children .iter() {
        reduced = reduce_tree ( child , new_node );
        if reduced {
        new_node . children . append ( reduced );
        if !new_node . children {
        new_node = None /* Option */;
        } else {
        new_node = reduce_tree ( node . children [ 0 ] , parent );
        } else if node . type == syms . Unit {
        if ( isinstance ( node . children [ 0 ] , pytree . Leaf ) and {
        node . children [ 0 ] . value == "(" ) ;
        return  reduce_tree ( node . children [ 1 ] , parent );
        if ( ( isinstance ( node . children [ 0 ] , pytree . Leaf ) and {
        node . children [ 0 ] . value == "[" );
        or;
        ( len ( node . children ) > 1 and;
        hasattr ( node . children [ 1 ] , "value" ) and;
        node . children [ 1 ] . value == "[" ) ) ;
        return;
        leaf = true;
        details_node = None /* Option */;
        alternatives_node = None /* Option */;
        has_repeater = false;
        repeater_node = None /* Option */;
        has_variable_name = false;
        for child in node . children .iter() {
        if child . type == syms . Details {
        leaf = false;
        details_node = child;
        } else if child . type == syms . Repeater {
        has_repeater = true;
        repeater_node = child;
        } else if child . type == syms . Alternatives {
        alternatives_node = child;
        if hasattr ( child , "value" ) && child . value == "=" {
        has_variable_name = true;
        if has_variable_name {
        name_leaf = node . children [ 2 ];
        if hasattr ( name_leaf , "value" ) && name_leaf . value == "(" {
        name_leaf = node . children [ 3 ];
        } else {
        name_leaf = node . children [ 0 ];
        if name_leaf . type == token_labels . NAME {
        if name_leaf . value == "any" {
        new_node = MinNode ( type = TYPE_ANY );
        } else {
        if hasattr ( token_labels , name_leaf . value ) {
        new_node = MinNode ( type = getattr ( token_labels , name_leaf . value ) );
        } else {
        new_node = MinNode ( type = getattr ( pysyms , name_leaf . value ) );
        } else if name_leaf . type == token_labels . STRING {
        name = name_leaf . value . strip ( "'" );
        if name in tokens {
        new_node = MinNode ( type = tokens [ name ] );
        } else {
        new_node = MinNode ( type = token_labels . NAME , name = name );
        } else if name_leaf . type == syms . Alternatives {
        new_node = reduce_tree ( alternatives_node , parent );
        if has_repeater {
        if repeater_node . children [ 0 ] . value == "*" {
        new_node = None /* Option */;
        } else if repeater_node . children [ 0 ] . value == "+" {
        // pass
        } else {
        panic!("NotImplementedError");
        if details_node && new_node is !None /* Option */ {
        for child in details_node . children [ 1 : -1 ] .iter() {
        reduced = reduce_tree ( child , new_node );
        if reduced is !None /* Option */ {
        new_node . children . append ( reduced );
        if new_node {
        new_node . parent = parent;
        return  new_node;
        pub fn get_characteristic_subpattern ( subpatterns )  {
        "Picks the most characteristic from a list of linear patterns
    Current order used is:
    names > common_names > common_chars
    ";
        if !isinstance ( subpatterns , list ) {
        return  subpatterns;
        if len ( subpatterns ) == 1 {
        return  subpatterns [ 0 ];
        subpatterns_with_names = [ ];
        subpatterns_with_common_names = [ ];
        common_names = [ "in" , "for" , "iformat!(" , "not" , "None /* Option */" ]);
        subpatterns_with_common_chars = [ ];
        common_chars = "[]().,:";
        for subpattern in subpatterns .iter() {
        if any ( rec_test ( subpattern , lambda x { : type ( x ) == str ) ) ; }
        if any ( rec_test ( subpattern , {
        |x | {  isinstance ( x , str ) && x in common_chars ) ) : };
        subpatterns_with_common_chars . append ( subpattern );
        } else if any ( rec_test ( subpattern , {
        |x | {  isinstance ( x , str ) && x in common_names ) ) : };
        subpatterns_with_common_names . append ( subpattern );
        } else {
        subpatterns_with_names . append ( subpattern );
        if subpatterns_with_names {
        subpatterns = subpatterns_with_names;
        } else if subpatterns_with_common_names {
        subpatterns = subpatterns_with_common_names;
        } else if subpatterns_with_common_chars {
        subpatterns = subpatterns_with_common_chars;
        return  max ( subpatterns , key = len );
        pub fn rec_test ( sequence , test_func )  {
        "Tests test_func on all items of sequence && items of included
    sub-iterables";
        for x in sequence .iter() {
        if isinstance ( x , ( list , tuple ) ) {
        yield from rec_test ( x , test_func );
        } else {
        yield test_func ( x );
}

