//! fix_metaclass.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::.::{fixer_base};

pub fn has_metaclass(parent: &str) {
        " we have to check the cls_node without changing it.
        There are two possibilities:
          1)  clsdef => suite => simple_stmt => expr_stmt => Leaf('__meta')
          2)  clsdef => simple_stmt => expr_stmt => Leaf('__meta')
    ";
        for node in parent . children .iter() {
        if node . type == syms . suite {
        return  has_metaclass ( node );
        } else if node . type == syms . simple_stmt && node . children {
        expr_node = node . children [ 0 ];
        if expr_node . type == syms . expr_stmt && expr_node . children {
        left_side = expr_node . children [ 0 ];
        if isinstance ( left_side , Leaf ) && \ {
        left_side . value == "__metaclass__" ;
        return  true;
        return  false;
        pub fn fixup_parse_tree ( cls_node )  {
        " one-line classes don't get a suite in the parse tree so we add
        one to normalize the tree
    ";
        for node in cls_node . children .iter() {
        if node . type == syms . suite {
        return;
        for i , node in enumerate ( cls_node . children ) .iter() {
        if node . type == token . COLON {
        break;
        } else {
        panic!("ValueError ( "No class suite && no ':'!" )");
        suite = Node ( syms . suite , [ ] );
        while cls_node . children [ i + 1 : ]  {
        move_node = cls_node . children [ i + 1 ];
        suite . append_child ( move_node . clone ( ) );
        move_node . remove ( );
        cls_node . append_child ( suite );
        node = suite;
        pub fn fixup_simple_stmt ( parent , i , stmt_node )  {
        " if there == a semi-colon all the parts count as part of the same
        simple_stmt.  We just want the __metaclass__ part so we move
        everything after the semi-colon into its own simple_stmt node
    ";
        for semi_ind , node in enumerate ( stmt_node . children ) .iter() {
        if node . type == token . SEMI {
        break;
        } else {
        return;
        node . remove ( );
        new_expr = Node ( syms . expr_stmt , [ ] );
        new_stmt = Node ( syms . simple_stmt , [ new_expr ] );
        while stmt_node . children [ semi_ind : ]  {
        move_node = stmt_node . children [ semi_ind ];
        new_expr . append_child ( move_node . clone ( ) );
        move_node . remove ( );
        parent . insert_child ( i , new_stmt );
        new_leaf1 = new_stmt . children [ 0 ] . children [ 0 ];
        old_leaf1 = stmt_node . children [ 0 ] . children [ 0 ];
        new_leaf1 . prefix = old_leaf1 . prefix;
        pub fn remove_trailing_newline ( node )  {
        if node . children && node . children [ -1 ] . type == token . NEWLINE {
        node . children [ -1 ] . remove ( );
        pub fn find_metas ( cls_node )  {
        for node in cls_node . children .iter() {
        if node . type == syms . suite {
        break;
        } else {
        panic!("ValueError ( "No class suite!" )");
        for i , simple_node in list ( enumerate ( node . children ) ) .iter() {
        if simple_node . type == syms . simple_stmt && simple_node . children {
        expr_node = simple_node . children [ 0 ];
        if expr_node . type == syms . expr_stmt && expr_node . children {
        left_node = expr_node . children [ 0 ];
        if isinstance ( left_node , Leaf ) && \ {
        left_node . value == "__metaclass__" ;
        fixup_simple_stmt ( node , i , simple_node );
        remove_trailing_newline ( simple_node );
        yield ( node , i , simple_node );
        pub fn fixup_indent ( suite )  {
        " If an INDENT == followed by a thing with a prefix then nuke the prefix
        Otherwise we get in trouble when removing __metaclass__ at suite start
    ";
        kids = suite . children [ : : -1 ];
        while kids  {
        node = kids . pop ( );
        if node . type == token . INDENT {
        break;
        while kids  {
        node = kids . pop ( );
        if isinstance ( node , Leaf ) && node . type != token . DEDENT {
        if node . prefix {
        node . prefix = "";
        return;
        } else {
        kids . extend ( node . children [ : : -1 ] );
        class FixMetaclass ( fixer_base . BaseFix ) ;
        BM_compatible = true;
        PATTERN = "
    classdef<any*>
    ";
        pub fn transform ( &self, node , results )  {
        if !has_metaclass ( node ) {
        return;
        fixup_parse_tree ( node );
        last_metaclass = None /* Option */;
        for suite , i , stmt in find_metas ( node ) .iter() {
        last_metaclass = stmt;
        stmt . remove ( );
        text_type = node . children [ 0 ] . type;
        if len ( node . children ) == 7 {
        if node . children [ 3 ] . type == syms . arglist {
        arglist = node . children [ 3 ];
        } else {
        parent = node . children [ 3 ] . clone ( );
        arglist = Node ( syms . arglist , [ parent ] );
        node . set_child ( 3 , arglist );
        } else if len ( node . children ) == 6 {
        arglist = Node ( syms . arglist , [ ] );
        node . insert_child ( 3 , arglist );
        } else if len ( node . children ) == 4 {
        arglist = Node ( syms . arglist , [ ] );
        node . insert_child ( 2 , Leaf ( token . RPAR , ")" ) );
        node . insert_child ( 2 , arglist );
        node . insert_child ( 2 , Leaf ( token . LPAR , "(" ) );
        } else {
        panic!("ValueError ( "Unexpected class definition" )");
        meta_txt = last_metaclass . children [ 0 ] . children [ 0 ];
        meta_txt . value = "metaclass";
        orig_meta_prefix = meta_txt . prefix;
        if arglist . children {
        arglist . append_child ( Leaf ( token . COMMA , "," ) );
        meta_txt . prefix = " ";
        } else {
        meta_txt . prefix = "";
        expr_stmt = last_metaclass . children [ 0 ];
        assert expr_stmt . type == syms . expr_stmt;
        expr_stmt . children [ 1 ] . prefix = "";
        expr_stmt . children [ 2 ] . prefix = "";
        arglist . append_child ( last_metaclass );
        fixup_indent ( suite );
        if !suite . children {
        suite . remove ( );
        pass_leaf = Leaf ( text_type , "pass" );
        pass_leaf . prefix = orig_meta_prefix;
        node . append_child ( pass_leaf );
        node . append_child ( Leaf ( token . NEWLINE , "\n" ) );
        } else if len ( suite . children ) > 1 && \ {
        ( suite . children [ -2 ] . type == token . INDENT and;
        suite . children [ -1 ] . type == token . DEDENT ) ;
        pass_leaf = Leaf ( text_type , "pass" );
        suite . insert_child ( -1 , pass_leaf );
        suite . insert_child ( -1 , Leaf ( token . NEWLINE , "\n" ) );
}

