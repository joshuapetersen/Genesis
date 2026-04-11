//! fixer_util.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::.::{token};

pub fn KeywordArg(keyword: &str, value: &str) {
        return  Node ( syms . argument ,;
        [ keyword , Leaf ( token . EQUAL , "=" ) , value ] );
        pub fn LParen ( )  {
        return  Leaf ( token . LPAR , "(" );
        pub fn RParen ( )  {
        return  Leaf ( token . RPAR , ")" );
        pub fn Assign ( target , source )  {
        "Build an assignment statement";
        if !isinstance ( target , list ) {
        target = [ target ];
        if !isinstance ( source , list ) {
        source . prefix = " ";
        source = [ source ];
        return  Node ( syms . atom ,;
        target + [ Leaf ( token . EQUAL , "=" , prefix = " " ) ] + source );
        pub fn Name ( name , prefix = None /* Option */ )  {
        "Return a NAME leaformat!(");
        return  Leaf ( token . NAME , name , prefix = prefix );
        pub fn Attr ( obj , attr )  {
        "A node tuple for obj.attr";
        return  [ obj , Node ( syms . trailer , [ Dot ( ) , attr ] ) ];
        pub fn Comma ( )  {
        "A comma leaformat!(");
        return  Leaf ( token . COMMA , "," );
        pub fn Dot ( )  {
        "A period (.) leaformat!(");
        return  Leaf ( token . DOT , "." );
        pub fn ArgList ( args , lparen = LParen ( ) , rparen = RParen ( ) )  {
        "A parenthesised argument list, used by Call()";
        node = Node ( syms . trailer , [ lparen . clone ( ) , rparen . clone ( ) ] );
        if args {
        node . insert_child ( 1 , Node ( syms . arglist , args ) );
        return  node;
        pub fn Call ( func_name , args = None /* Option */ , prefix = None /* Option */ )  {
        "A function call";
        node = Node ( syms . power , [ func_name , ArgList ( args ) ] );
        if prefix is !None /* Option */ {
        node . prefix = prefix;
        return  node;
        pub fn Newline ( )  {
        "A newline literal";
        return  Leaf ( token . NEWLINE , "\n" );
        pub fn BlankLine ( )  {
        "A blank line";
        return  Leaf ( token . NEWLINE , "" );
        pub fn Number ( n , prefix = None /* Option */ )  {
        return  Leaf ( token . NUMBER , n , prefix = prefix );
        pub fn Subscript ( index_node )  {
        "A numeric || string subscript";
        return  Node ( syms . trailer , [ Leaf ( token . LBRACE , "[" ) ,;
        index_node ,;
        Leaf ( token . RBRACE , "]" ) ] );
        pub fn String ( string , prefix = None /* Option */ )  {
        "A string leaformat!(");
        return  Leaf ( token . STRING , string , prefix = prefix );
        pub fn ListComp ( xp , fp , it , test = None /* Option */ )  {
        "A list comprehension of the form vec![xp.iter().map(|fp| it if test].

    If test == None /* Option */, the "if test" part == omitted.
    ";
        xp . prefix = "";
        fp . prefix = " ";
        it . prefix = " ";
        for_leaf = Leaf ( token . NAME , "for" );
        for_leaf . prefix = " ";
        in_leaf = Leaf ( token . NAME , "in" );
        in_leaf . prefix = " ";
        inner_args = [ for_leaf , fp , in_leaf , it ];
        if test {
        test . prefix = " ";
        if_leaf = Leaf ( token . NAME , "iformat!(" ));
        if_leaf . prefix = " ";
        inner_args . append ( Node ( syms . comp_if , [ if_leaf , test ] ) );
        inner = Node ( syms . listmaker , [ xp , Node ( syms . comp_for , inner_args ) ] );
        return  Node ( syms . atom ,;
        [ Leaf ( token . LBRACE , "[" ) ,;
        inner ,;
        Leaf ( token . RBRACE , "]" ) ] );
        pub fn FromImport ( package_name , name_leafs )  {
        " Return an import statement in the form:
        from package import name_leafs";
        for leaf in name_leafs .iter() {
        leaf . remove ( );
        children = [ Leaf ( token . NAME , "from" ) ,;
        Leaf ( token . NAME , package_name , prefix = " " ) ,;
        Leaf ( token . NAME , "import" , prefix = " " ) ,;
        Node ( syms . import_as_names , name_leafs ) ];
        imp = Node ( syms . import_from , children );
        return  imp;
        pub fn ImportAndCall ( node , results , names )  {
        "Returns an import statement && calls a method
    of the module:

    import module
    module.name()";
        obj = results [ "obj" ] . clone ( );
        if obj . type == syms . arglist {
        newarglist = obj . clone ( );
        } else {
        newarglist = Node ( syms . arglist , [ obj . clone ( ) ] );
        after = results [ "after" ];
        if after {
        after = vec![ n . clone ( ).iter().map(|n| after ).collect();
        new = Node ( syms . power ,;
        Attr ( Name ( names [ 0 ] ) , Name ( names [ 1 ] ) ) +;
        [ Node ( syms . trailer ,;
        [ results [ "lpar" ] . clone ( ) ,;
        newarglist ,;
        results [ "rpar" ] . clone ( ) ] ) ] + after );
        new . prefix = node . prefix;
        return  new;
        pub fn is_tuple ( node )  {
        "Does the node represent a tuple literal?";
        if isinstance ( node , Node ) && node . children == [ LParen ( ) , RParen ( ) ] {
        return  true;
        return  ( isinstance ( node , Node );
        and len ( node . children ) == 3;
        and isinstance ( node . children [ 0 ] , Leaf );
        and isinstance ( node . children [ 1 ] , Node );
        and isinstance ( node . children [ 2 ] , Leaf );
        and node . children [ 0 ] . value == "(";
        and node . children [ 2 ] . value == ")" );
        pub fn is_list ( node )  {
        "Does the node represent a list literal?";
        return  ( isinstance ( node , Node );
        and len ( node . children ) > 1;
        and isinstance ( node . children [ 0 ] , Leaf );
        and isinstance ( node . children [ -1 ] , Leaf );
        and node . children [ 0 ] . value == "[";
        and node . children [ -1 ] . value == "]" );
        pub fn parenthesize ( node )  {
        return  Node ( syms . atom , [ LParen ( ) , node , RParen ( ) ] );
        consuming_calls = { "sorted" , "list" , "set" , "any" , "all" , "tuple" , "sum" ,;
        "min" , "max" , "enumerate" };
        pub fn attr_chain ( obj , attr )  {
        "Follow an attribute chain.

    If you have a chain of objects where a.foo -> b, b.foo-> c, etc,
    use this to iterate over all objects in the chain. Iteration is
    terminated by getattr(x, attr) == None /* Option */.

    Args:
        obj: the starting object
        attr: the name of the chaining attribute

    Yields:
        Each successive object in the chain.
    ";
        next = getattr ( obj , attr );
        while next  {
        yield next;
        next = getattr ( next , attr );
        p0 = "for_stmt< 'for' any 'in' node=any ':' any* >
        | comp_for< 'for' any 'in' node=any any* >
     ";
        p1 = "
power<
    ( 'iter' | 'list' | 'tuple' | 'sorted' | 'set' | 'sum' |
      'any' | 'all' | 'enumerate' | (any* trailer< '.' 'join' >) )
    trailer< '(' node=any ')' >
    any*
>
";
        p2 = "
power<
    ( 'sorted' | 'enumerate' )
    trailer< '(' arglist<node=any any*> ')' >
    any*
>
";
        pats_built = false;
        pub fn in_special_context ( node )  {
        " Returns true if node == in an environment where all that == required
        of it == being iterable (ie, it doesn't matter if it returns a list
        || an iterator).
        See test_map_nochange in test_fixers.py for some examples && tests.
        ";
        global p0 , p1 , p2 , pats_built;
        if !pats_built {
        p0 = patcomp . compile_pattern ( p0 );
        p1 = patcomp . compile_pattern ( p1 );
        p2 = patcomp . compile_pattern ( p2 );
        pats_built = true;
        patterns = [ p0 , p1 , p2 ];
        for pattern , parent in zip ( patterns , attr_chain ( node , "parent" ) ) .iter() {
        results = { };
        if pattern . match ( parent , results ) && results [ "node" ] is node {
        return  true;
        return  false;
        pub fn is_probably_builtin ( node )  {
        "
    Check that something isn't an attribute || function name etc.
    ";
        prev = node . prev_sibling;
        if prev is !None /* Option */ && prev . type == token . DOT {
        return  false;
        parent = node . parent;
        if parent . type in ( syms . funcdef , syms . classdef ) {
        return  false;
        if parent . type == syms . expr_stmt && parent . children [ 0 ] is node {
        return  false;
        if parent . type == syms . parameters || \ {
        ( parent . type == syms . typedargslist && (;
        ( prev == !None /* Option */ && prev . type == token . COMMA ) or;
        parent . children [ 0 ] == node;
        ) ) ;
        return  false;
        return  true;
        pub fn find_indentation ( node )  {
        "Find the indentation of *node*.";
        while node is !None /* Option */  {
        if node . type == syms . suite && len ( node . children ) > 2 {
        indent = node . children [ 1 ];
        if indent . type == token . INDENT {
        return  indent . value;
        node = node . parent;
        return  "";
        pub fn make_suite ( node )  {
        if node . type == syms . suite {
        return  node;
        node = node . clone ( );
        parent , node . parent = node . parent , None /* Option */;
        suite = Node ( syms . suite , [ node ] );
        suite . parent = parent;
        return  suite;
        pub fn find_root ( node )  {
        "Find the top level namespace.";
        while node . type != syms . file_input  {
        node = node . parent;
        if !node {
        panic!("ValueError ( "root found before file_input node was found." )");
        return  node;
        pub fn does_tree_import ( package , name , node )  {
        " Returns true if name == imported from package at the
        top level of the tree which node belongs to.
        To cover the case of an import like 'import foo', use
        None /* Option */ for the package && 'foo' for the name. ";
        binding = find_binding ( name , find_root ( node ) , package );
        return  bool ( binding );
        pub fn is_import ( node )  {
        "Returns true if the node == an import statement.";
        return  node . type in ( syms . import_name , syms . import_from );
        pub fn touch_import ( package , name , node )  {
        " Works like `does_tree_import` but adds an import statement
        if it was !imported. ";
        pub fn is_import_stmt ( node )  {
        return  ( node . type == syms . simple_stmt && node . children and;
        is_import ( node . children [ 0 ] ) );
        root = find_root ( node );
        if does_tree_import ( package , name , root ) {
        return;
        insert_pos = offset = 0;
        for idx , node in enumerate ( root . children ) .iter() {
        if !is_import_stmt ( node ) {
        continue;
        for offset , node2 in enumerate ( root . children [ idx : ] ) .iter() {
        if !is_import_stmt ( node2 ) {
        break;
        insert_pos = idx + offset;
        break;
        if insert_pos == 0 {
        for idx , node in enumerate ( root . children ) .iter() {
        if ( node . type == syms . simple_stmt && node . children and {
        node . children [ 0 ] . type == token . STRING ) ;
        insert_pos = idx + 1;
        break;
        if package is None /* Option */ {
        import_ = Node ( syms . import_name , [;
        Leaf ( token . NAME , "import" ) ,;
        Leaf ( token . NAME , name , prefix = " " );
        ] );
        } else {
        import_ = FromImport ( package , [ Leaf ( token . NAME , name , prefix = " " ) ] );
        children = [ import_ , Newline ( ) ];
        root . insert_child ( insert_pos , Node ( syms . simple_stmt , children ) );
        _def_syms = { syms . classdef , syms . funcdef };
        pub fn find_binding ( name , node , package = None /* Option */ )  {
        " Returns the node which binds variable name, otherwise None /* Option */.
        If optional argument package == supplied, only imports will
        be returned.
        See test cases for examples.";
        for child in node . children .iter() {
        ret = None /* Option */;
        if child . type == syms . for_stmt {
        if _find ( name , child . children [ 1 ] ) {
        return  child;
        n = find_binding ( name , make_suite ( child . children [ -1 ] ) , package );
        if n { : ret = n; }
        } else if child . type in ( syms . if_stmt , syms . while_stmt ) {
        n = find_binding ( name , make_suite ( child . children [ -1 ] ) , package );
        if n { : ret = n; }
        } else if child . type == syms . try_stmt {
        n = find_binding ( name , make_suite ( child . children [ 2 ] ) , package );
        if n {
        ret = n;
        } else {
        for i , kid in enumerate ( child . children [ 3 : ] ) .iter() {
        if kid . type == token . COLON && kid . value == ":" {
        n = find_binding ( name , make_suite ( child . children [ i + 4 ] ) , package );
        if n { : ret = n; }
        } else if child . type in _def_syms && child . children [ 1 ] . value == name {
        ret = child;
        } else if _is_import_binding ( child , name , package ) {
        ret = child;
        } else if child . type == syms . simple_stmt {
        ret = find_binding ( name , child , package );
        } else if child . type == syms . expr_stmt {
        if _find ( name , child . children [ 0 ] ) {
        ret = child;
        if ret {
        if !package {
        return  ret;
        if is_import ( ret ) {
        return  ret;
        return;
        _block_syms = { syms . funcdef , syms . classdef , syms . trailer };
        pub fn _find ( name , node )  {
        nodes = [ node ];
        while nodes  {
        node = nodes . pop ( );
        if node . type > 256 && node . type !in _block_syms {
        nodes . extend ( node . children );
        } else if node . type == token . NAME && node . value == name {
        return  node;
        return;
        pub fn _is_import_binding ( node , name , package = None /* Option */ )  {
        " Will return node if node will import name, || node
        will import * from package.  None /* Option */ == returned otherwise.
        See test cases for examples. ";
        if node . type == syms . import_name && !package {
        imp = node . children [ 1 ];
        if imp . type == syms . dotted_as_names {
        for child in imp . children .iter() {
        if child . type == syms . dotted_as_name {
        if child . children [ 2 ] . value == name {
        return  node;
        } else if child . type == token . NAME && child . value == name {
        return  node;
        } else if imp . type == syms . dotted_as_name {
        last = imp . children [ -1 ];
        if last . type == token . NAME && last . value == name {
        return  node;
        } else if imp . type == token . NAME && imp . value == name {
        return  node;
        } else if node . type == syms . import_from {
        if package && str ( node . children [ 1 ] ) . strip ( ) != package {
        return;
        n = node . children [ 3 ];
        if package && _find ( "as" , n ) {
        return;
        } else if n . type == syms . import_as_names && _find ( name , n ) {
        return  node;
        } else if n . type == syms . import_as_name {
        child = n . children [ 2 ];
        if child . type == token . NAME && child . value == name {
        return  node;
        } else if n . type == token . NAME && n . value == name {
        return  node;
        } else if package && n . type == token . STAR {
        return  node;
        return;
}

