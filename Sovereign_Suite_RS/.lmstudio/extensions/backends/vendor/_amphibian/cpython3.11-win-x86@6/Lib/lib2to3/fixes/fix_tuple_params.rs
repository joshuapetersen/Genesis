//! fix_tuple_params.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::.::{pytree};

pub fn is_docstring(stmt: &str) {
        return  isinstance ( stmt , pytree . Node ) && \;
        stmt . children [ 0 ] . type == token . STRING;
        class FixTupleParams ( fixer_base . BaseFix ) ;
        run_order = 4;
        BM_compatible = true;
        PATTERN = "
              funcdef< 'def' any parameters< '(' args=any ')' >
                       ['->' any] ':' suite=any+ >
              |
              lambda=
              lambdef< 'lambda' args=vfpdef< '(' inner=any ')' >
                       ':' body=any
              >
              ";
        pub fn transform ( &self, node , results )  {
        if "lambda" in results {
        return  self . transform_lambda ( node , results );
        new_lines = [ ];
        suite = results [ "suite" ];
        args = results [ "args" ];
        if suite [ 0 ] . children [ 1 ] . type == token . INDENT {
        start = 2;
        indent = suite [ 0 ] . children [ 1 ] . value;
        end = Newline ( );
        } else {
        start = 0;
        indent = "; ";
        end = pytree . Leaf ( token . INDENT , "" );
        pub fn handle_tuple ( tuple_arg , add_prefix = false )  {
        n = Name ( self . new_name ( ) );
        arg = tuple_arg . clone ( );
        arg . prefix = "";
        stmt = Assign ( arg , n . clone ( ) );
        if add_prefix {
        n . prefix = " ";
        tuple_arg . replace ( n );
        new_lines . append ( pytree . Node ( syms . simple_stmt ,;
        [ stmt , end . clone ( ) ] ) );
        if args . type == syms . tfpdef {
        handle_tuple ( args );
        } else if args . type == syms . typedargslist {
        for i , arg in enumerate ( args . children ) .iter() {
        if arg . type == syms . tfpdef {
        handle_tuple ( arg , add_prefix = ( i > 0 ) );
        if !new_lines {
        return;
        for line in new_lines .iter() {
        line . parent = suite [ 0 ];
        after = start;
        if start == 0 {
        new_lines [ 0 ] . prefix = " ";
        } else if is_docstring ( suite [ 0 ] . children [ start ] ) {
        new_lines [ 0 ] . prefix = indent;
        after = start + 1;
        for line in new_lines .iter() {
        line . parent = suite [ 0 ];
        suite [ 0 ] . children [ after : after ] = new_lines;
        for i in range ( after + 1 , after + len ( new_lines ) + 1 ) .iter() {
        suite [ 0 ] . children [ i ] . prefix = indent;
        suite [ 0 ] . changed ( );
        pub fn transform_lambda ( &self, node , results )  {
        args = results [ "args" ];
        body = results [ "body" ];
        inner = simplify_args ( results [ "inner" ] );
        if inner . type == token . NAME {
        inner = inner . clone ( );
        inner . prefix = " ";
        args . replace ( inner );
        return;
        params = find_params ( args );
        to_index = map_to_index ( params );
        tup_name = self . new_name ( tuple_name ( params ) );
        new_param = Name ( tup_name , prefix = " " );
        args . replace ( new_param . clone ( ) );
        for n in body . post_order ( ) .iter() {
        if n . type == token . NAME && n . value in to_index {
        subscripts = vec![ c . clone ( ).iter().map(|c| to_index vec![ n . value ] ).collect();
        new = pytree . Node ( syms . power ,;
        [ new_param . clone ( ) ] + subscripts );
        new . prefix = n . prefix;
        n . replace ( new );
        pub fn simplify_args ( node )  {
        if node . type in ( syms . vfplist , token . NAME ) {
        return  node;
        } else if node . type == syms . vfpdef {
        while node . type == syms . vfpdef  {
        node = node . children [ 1 ];
        return  node;
        panic!("RuntimeError ( "Received unexpected node %s" % node )");
        pub fn find_params ( node )  {
        if node . type == syms . vfpdef {
        return  find_params ( node . children [ 1 ] );
        } else if node . type == token . NAME {
        return  node . value;
        return  [ find_params ( c ) for c in node . children if c . type != token . COMMA ];
        pub fn map_to_index ( param_list , prefix = [ ] , d = None /* Option */ )  {
        if d is None /* Option */ {
        d = { };
        for i , obj in enumerate ( param_list ) .iter() {
        trailer = [ Subscript ( Number ( str ( i ) ) ) ];
        if isinstance ( obj , list ) {
        map_to_index ( obj , trailer , d = d );
        } else {
        d [ obj ] = prefix + trailer;
        return  d;
        pub fn tuple_name ( param_list )  {
        l = [ ];
        for obj in param_list .iter() {
        if isinstance ( obj , list ) {
        l . append ( tuple_name ( obj ) );
        } else {
        l . append ( obj );
        return  "_" . join ( l );
}

