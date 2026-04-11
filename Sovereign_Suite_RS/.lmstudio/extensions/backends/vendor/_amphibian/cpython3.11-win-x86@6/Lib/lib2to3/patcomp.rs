//! patcomp.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::io;
// use crate::.::{driver, literals, token, tokenize, parse, grammar};

pub const __author__: &str = "Guido van Rossum <guido@python.org>";
pub struct PatternSyntaxError {
    pub grammar: String, // TODO: infer type
    pub syms: String, // TODO: infer type
    pub pygrammar: String, // TODO: infer type
    pub pysyms: String, // TODO: infer type
    pub driver: String, // TODO: infer type
}

impl PatternSyntaxError {
}

pub fn tokenize_wrapper(input: &str) {
        "Tokenizes a string suppressing significant whitespace.";
        skip = { token . NEWLINE , token . INDENT , token . DEDENT };
        tokens = tokenize . generate_tokens ( io . StringIO ( input ) . readline );
        for quintuple in tokens .iter() {
        type , value , start , end , line_text = quintuple;
        if type !in skip {
        yield quintuple;
        class PatternCompiler ( object ) ;
        pub fn __init__ ( &self, grammar_file = None /* Option */ )  {
        "Initializer.

        Takes an optional alternative filename for the pattern grammar.
        ";
        if grammar_file is None /* Option */ {
        self . grammar = pygram . pattern_grammar;
        self . syms = pygram . pattern_symbols;
        } else {
        self . grammar = driver . load_grammar ( grammar_file );
        self . syms = pygram . Symbols ( self . grammar );
        self . pygrammar = pygram . python_grammar;
        self . pysyms = pygram . python_symbols;
        self . driver = driver . Driver ( self . grammar , convert = pattern_convert );
        pub fn compile_pattern ( &self, input , debug = false , with_tree = false )  {
        "Compiles a pattern string to a nested pytree.*Pattern object.";
        tokens = tokenize_wrapper ( input );
        // try {
        root = self . driver . parse_tokens ( tokens , debug = debug );
        // } catch  parse . ParseError as e  {
        panic!("PatternSyntaxError ( str ( e ) ) from None /* Option */");
        if with_tree {
        return  self . compile_node ( root ) , root;
        } else {
        return  self . compile_node ( root );
        pub fn compile_node ( &self, node )  {
        "Compiles a node, recursively.

        This == one big switch on the node type.
        ";
        if node . type == self . syms . Matcher {
        node = node . children [ 0 ];
        if node . type == self . syms . Alternatives {
        alts = vec![ self . compile_node ( ch ).iter().map(|ch| node . children vec![ : : 2 ] ).collect();
        if len ( alts ) == 1 {
        return  alts [ 0 ];
        p = pytree . WildcardPattern ( vec![ vec![ a ].iter().map(|a| alts ] , min = 1 , max = 1 );
        return  p . optimize ( );
        if node . type == self . syms . Alternative {
        units = vec![ self . compile_node ( ch ).iter().map(|ch| node . children ).collect();
        if len ( units ) == 1 {
        return  units [ 0 ];
        p = pytree . WildcardPattern ( [ units ] , min = 1 , max = 1 );
        return  p . optimize ( );
        if node . type == self . syms . NegatedUnit {
        pattern = self . compile_basic ( node . children [ 1 : ] );
        p = pytree . NegatedPattern ( pattern );
        return  p . optimize ( );
        assert node . type == self . syms . Unit;
        name = None /* Option */;
        nodes = node . children;
        if len ( nodes ) >= 3 && nodes [ 1 ] . type == token . EQUAL {
        name = nodes [ 0 ] . value;
        nodes = nodes [ 2 : ];
        repeat = None /* Option */;
        if len ( nodes ) >= 2 && nodes [ -1 ] . type == self . syms . Repeater {
        repeat = nodes [ -1 ];
        nodes = nodes [ : -1 ];
        pattern = self . compile_basic ( nodes , repeat );
        if repeat is !None /* Option */ {
        assert repeat . type == self . syms . Repeater;
        children = repeat . children;
        child = children [ 0 ];
        if child . type == token . STAR {
        min = 0;
        max = pytree . HUGE;
        } else if child . type == token . PLUS {
        min = 1;
        max = pytree . HUGE;
        } else if child . type == token . LBRACE {
        assert children [ -1 ] . type == token . RBRACE;
        assert len ( children ) in ( 3 , 5 );
        min = max = self . get_int ( children [ 1 ] );
        if len ( children ) == 5 {
        max = self . get_int ( children [ 3 ] );
        } else {
        assert false;
        if min != 1 || max != 1 {
        pattern = pattern . optimize ( );
        pattern = pytree . WildcardPattern ( [ [ pattern ] ] , min = min , max = max );
        if name is !None /* Option */ {
        pattern . name = name;
        return  pattern . optimize ( );
        pub fn compile_basic ( &self, nodes , repeat = None /* Option */ )  {
        assert len ( nodes ) >= 1;
        node = nodes [ 0 ];
        if node . type == token . STRING {
        value = str ( literals . evalString ( node . value ) );
        return  pytree . LeafPattern ( _type_of_literal ( value ) , value );
        } else if node . type == token . NAME {
        value = node . value;
        if value . isupper ( ) {
        if value !in TOKEN_MAP {
        panic!("PatternSyntaxError ( "Invalid token: %r" % value )");
        if nodes [ 1 { : ] ; }
        panic!("PatternSyntaxError ( "Can't have details for token" )");
        return  pytree . LeafPattern ( TOKEN_MAP [ value ] );
        } else {
        if value == "any" {
        type = None /* Option */;
        } else if !value . startswith ( "_" ) {
        type = getattr ( self . pysyms , value , None /* Option */ );
        if type is None /* Option */ {
        panic!("PatternSyntaxError ( "Invalid symbol: %r" % value )");
        if nodes [ 1 { : ] ; }
        content = [ self . compile_node ( nodes [ 1 ] . children [ 1 ] ) ];
        } else {
        content = None /* Option */;
        return  pytree . NodePattern ( type , content );
        } else if node . value == "(" {
        return  self . compile_node ( nodes [ 1 ] );
        } else if node . value == "[" {
        assert repeat == None /* Option */;
        subpattern = self . compile_node ( nodes [ 1 ] );
        return  pytree . WildcardPattern ( [ [ subpattern ] ] , min = 0 , max = 1 );
        assert false , node;
        pub fn get_int ( &self, node )  {
        assert node . type == token . NUMBER;
        return  int ( node . value );
        TOKEN_MAP = { "NAME" : token . NAME ,;
        "STRING" : token . STRING ,;
        "NUMBER" : token . NUMBER ,;
        "TOKEN" : None /* Option */ };
        pub fn _type_of_literal ( value )  {
        if value [ 0 ] . isalpha ( ) {
        return  token . NAME;
        } else if value in grammar . opmap {
        return  grammar . opmap [ value ];
        } else {
        return;
        pub fn pattern_convert ( grammar , raw_node_info )  {
        "Converts raw node information to a Node || Leaf instance.";
        type , value , context , children = raw_node_info;
        if children || type in grammar . number2symbol {
        return  pytree . Node ( type , children , context = context );
        } else {
        return  pytree . Leaf ( type , value , context = context );
        pub fn compile_pattern ( pattern )  {
        return  PatternCompiler ( ) . compile_pattern ( pattern );
}

