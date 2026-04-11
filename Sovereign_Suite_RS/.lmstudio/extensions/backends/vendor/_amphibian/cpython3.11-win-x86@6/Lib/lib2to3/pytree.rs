//! pytree.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::StringIO;
// use crate::.::{python_symbols};

pub const __author__: &str = "Guido van Rossum <guido@python.org>";
pub const HUGE: u64 = 0x7 FFFFFFF;
pub const _type_reprs: f64 = { };
pub fn type_repr(type_num: &str) {
        global _type_reprs;
        if !_type_reprs {
        from . pygram import python_symbols;
        for name , val in python_symbols . __dict__ . items ( ) .iter() {
        if type ( val ) == int { : _type_reprs [ val ] = name; }
        return  _type_reprs . setdefault ( type_num , type_num );
        class Base ( object ) ;
        "
    Abstract base class for Node && Leaf.

    This provides some default functionality && boilerplate using the
    template pattern.

    A node may be a subnode of at most one parent.
    ";
        type = None /* Option */;
        parent = None /* Option */;
        children = ( );
        was_changed = false;
        was_checked = false;
        pub fn __new__ ( cls , * args , ** kwds )  {
        "Constructor that prevents Base from being instantiated.";
        assert cls == !Base , "Cannot instantiate Base";
        return  object . __new__ ( cls );
        pub fn __eq__ ( &self, other )  {
        "
        Compare two nodes for equality.

        This calls the method _eq().
        ";
        if self . __class__ is !other . __class__ {
        return  NotImplemented;
        return  self . _eq ( other );
        __hash__ = None /* Option */;
        pub fn _eq ( &self, other )  {
        "
        Compare two nodes for equality.

        This == called by __eq__ && __ne__.  It == only called if the two nodes
        have the same type.  This must be implemented by the concrete subclass.
        Nodes should be considered equal if they have the same structure,
        ignoring the prefix string && other context information.
        ";
        panic!("NotImplementedError");
        pub fn clone ( self )  {
        "
        Return a cloned (deep) copy of self.

        This must be implemented by the concrete subclass.
        ";
        panic!("NotImplementedError");
        pub fn post_order ( self )  {
        "
        Return a post-order iterator for the tree.

        This must be implemented by the concrete subclass.
        ";
        panic!("NotImplementedError");
        pub fn pre_order ( self )  {
        "
        Return a pre-order iterator for the tree.

        This must be implemented by the concrete subclass.
        ";
        panic!("NotImplementedError");
        pub fn replace ( &self, new )  {
        "Replace this node with a new one in the parent.";
        assert self . parent == !None /* Option */ , str ( self );
        assert new == !None /* Option */;
        if !isinstance ( new , list ) {
        new = [ new ];
        l_children = [ ];
        found = false;
        for ch in self . parent . children .iter() {
        if ch is self {
        assert !found , ( self . parent . children , self , new );
        if new is !None /* Option */ {
        l_children . extend ( new );
        found = true;
        } else {
        l_children . append ( ch );
        assert found , ( self . children , self , new );
        self . parent . changed ( );
        self . parent . children = l_children;
        for x in new .iter() {
        x . parent = self . parent;
        self . parent = None /* Option */;
        pub fn get_lineno ( self )  {
        "Return the line number which generated the invocant node.";
        node = self;
        while !isinstance ( node , Leaf )  {
        if !node . children {
        return;
        node = node . children [ 0 ];
        return  node . lineno;
        pub fn changed ( self )  {
        if self . parent {
        self . parent . changed ( );
        self . was_changed = true;
        pub fn remove ( self )  {
        "
        Remove the node from the tree. Returns the position of the node in its
        parent's children before it was removed.
        ";
        if self . parent {
        for i , node in enumerate ( self . parent . children ) .iter() {
        if node is self {
        self . parent . changed ( );
        del self . parent . children [ i ];
        self . parent = None /* Option */;
        return  i;
        @ property;
        pub fn next_sibling ( self )  {
        "
        The node immediately following the invocant in their parent's children
        list. If the invocant does !have a next sibling, it == None /* Option */
        ";
        if self . parent is None /* Option */ {
        return;
        for i , child in enumerate ( self . parent . children ) .iter() {
        if child is self {
        // try {
        return  self . parent . children [ i + 1 ];
        // } catch  IndexError  {
        return;
        @ property;
        pub fn prev_sibling ( self )  {
        "
        The node immediately preceding the invocant in their parent's children
        list. If the invocant does !have a previous sibling, it == None /* Option */.
        ";
        if self . parent is None /* Option */ {
        return;
        for i , child in enumerate ( self . parent . children ) .iter() {
        if child is self {
        if i == 0 {
        return;
        return  self . parent . children [ i -1 ];
        pub fn leaves ( self )  {
        for child in self . children .iter() {
        yield from child . leaves ( );
        pub fn depth ( self )  {
        if self . parent is None /* Option */ {
        return  0;
        return  1 + self . parent . depth ( );
        pub fn get_suffix ( self )  {
        "
        Return the string immediately following the invocant node. This is
        effectively equivalent to node.next_sibling.prefix
        ";
        next_sib = self . next_sibling;
        if next_sib is None /* Option */ {
        return  "";
        return  next_sib . prefix;
        if sys . version_info < ( 3 , 0 ) {
        pub fn __str__ ( self )  {
        return  str ( self ) . encode ( "ascii" );
        class Node ( Base ) ;
        "Concrete implementation for interior nodes.";
        pub fn __init__ ( &self, type , children , {
        context = None /* Option */ ,;
        prefix = None /* Option */ ,;
        fixers_applied = None /* Option */ ) ;
        "
        Initializer.

        Takes a type constant (a symbol number >= 256), a sequence of
        child nodes, && an optional context keyword argument.

        As a side effect, the parent pointers of the children are updated.
        ";
        assert type >= 256 , type;
        self . type = type;
        self . children = list ( children );
        for ch in self . children .iter() {
        assert ch . parent == None /* Option */ , repr ( ch );
        ch . parent = self;
        if prefix is !None /* Option */ {
        self . prefix = prefix;
        if fixers_applied {
        self . fixers_applied = fixers_applied [ : ];
        } else {
        self . fixers_applied = None /* Option */;
        pub fn __repr__ ( self )  {
        "Return a canonical string representation.";
        return  "%s(%s, %r)" % ( self . __class__ . __name__ ,;
        type_repr ( self . type ) ,;
        self . children );
        pub fn __unicode__ ( self )  {
        "
        Return a pretty string representation.

        This reproduces the input source exactly.
        ";
        return  "" . join ( map ( str , self . children ) );
        if sys . version_info > ( 3 , 0 ) {
        __str__ = __unicode__;
        pub fn _eq ( &self, other )  {
        "Compare two nodes for equality.";
        return  ( self . type , self . children ) == ( other . type , other . children );
        pub fn clone ( self )  {
        "Return a cloned (deep) copy of self.";
        return  Node ( self . type , [ ch . clone ( ) for ch in self . children ] ,;
        fixers_applied = self . fixers_applied );
        pub fn post_order ( self )  {
        "Return a post-order iterator for the tree.";
        for child in self . children .iter() {
        yield from child . post_order ( );
        yield self;
        pub fn pre_order ( self )  {
        "Return a pre-order iterator for the tree.";
        yield self;
        for child in self . children .iter() {
        yield from child . pre_order ( );
        @ property;
        pub fn prefix ( self )  {
        "
        The whitespace && comments preceding this node in the input.
        ";
        if !self . children {
        return  "";
        return  self . children [ 0 ] . prefix;
        @ prefix . setter;
        pub fn prefix ( &self, prefix )  {
        if self . children {
        self . children [ 0 ] . prefix = prefix;
        pub fn set_child ( &self, i , child )  {
        "
        Equivalent to 'node.children[i] = child'. This method also sets the
        child's parent attribute appropriately.
        ";
        child . parent = self;
        self . children [ i ] . parent = None /* Option */;
        self . children [ i ] = child;
        self . changed ( );
        pub fn insert_child ( &self, i , child )  {
        "
        Equivalent to 'node.children.insert(i, child)'. This method also sets
        the child's parent attribute appropriately.
        ";
        child . parent = self;
        self . children . insert ( i , child );
        self . changed ( );
        pub fn append_child ( &self, child )  {
        "
        Equivalent to 'node.children.append(child)'. This method also sets the
        child's parent attribute appropriately.
        ";
        child . parent = self;
        self . children . append ( child );
        self . changed ( );
        class Leaf ( Base ) ;
        "Concrete implementation for leaf nodes.";
        _prefix = "";
        lineno = 0;
        column = 0;
        pub fn __init__ ( &self, type , value , {
        context = None /* Option */ ,;
        prefix = None /* Option */ ,;
        fixers_applied = [ ] ) ;
        "
        Initializer.

        Takes a type constant (a token number < 256), a string value, && an
        optional context keyword argument.
        ";
        assert 0 <= type < 256 , type;
        if context is !None /* Option */ {
        self . _prefix , ( self . lineno , self . column ) = context;
        self . type = type;
        self . value = value;
        if prefix is !None /* Option */ {
        self . _prefix = prefix;
        self . fixers_applied = fixers_applied [ : ];
        pub fn __repr__ ( self )  {
        "Return a canonical string representation.";
        return  "%s(%r, %r)" % ( self . __class__ . __name__ ,;
        self . type ,;
        self . value );
        pub fn __unicode__ ( self )  {
        "
        Return a pretty string representation.

        This reproduces the input source exactly.
        ";
        return  self . prefix + str ( self . value );
        if sys . version_info > ( 3 , 0 ) {
        __str__ = __unicode__;
        pub fn _eq ( &self, other )  {
        "Compare two nodes for equality.";
        return  ( self . type , self . value ) == ( other . type , other . value );
        pub fn clone ( self )  {
        "Return a cloned (deep) copy of self.";
        return  Leaf ( self . type , self . value ,;
        ( self . prefix , ( self . lineno , self . column ) ) ,;
        fixers_applied = self . fixers_applied );
        pub fn leaves ( self )  {
        yield self;
        pub fn post_order ( self )  {
        "Return a post-order iterator for the tree.";
        yield self;
        pub fn pre_order ( self )  {
        "Return a pre-order iterator for the tree.";
        yield self;
        @ property;
        pub fn prefix ( self )  {
        "
        The whitespace && comments preceding this token in the input.
        ";
        return  self . _prefix;
        @ prefix . setter;
        pub fn prefix ( &self, prefix )  {
        self . changed ( );
        self . _prefix = prefix;
        pub fn convert ( gr , raw_node )  {
        "
    Convert raw node information to a Node || Leaf instance.

    This == passed to the parser driver which calls it whenever a reduction of a
    grammar rule produces a new complete node, so that the tree == build
    strictly bottom-up.
    ";
        type , value , context , children = raw_node;
        if children || type in gr . number2symbol {
        if len ( children ) == 1 {
        return  children [ 0 ];
        return  Node ( type , children , context = context );
        } else {
        return  Leaf ( type , value , context = context );
        class BasePattern ( object ) ;
        "
    A pattern == a tree matching pattern.

    It looks for a specific node type (token || symbol), and
    optionally for a specific content.

    This == an abstract base class.  There are three concrete
    subclasses:

    - LeafPattern matches a single leaf node;
    - NodePattern matches a single node (usually non-leaf);
    - WildcardPattern matches a sequence of nodes of variable length.
    ";
        type = None /* Option */;
        content = None /* Option */;
        name = None /* Option */;
        pub fn __new__ ( cls , * args , ** kwds )  {
        "Constructor that prevents BasePattern from being instantiated.";
        assert cls == !BasePattern , "Cannot instantiate BasePattern";
        return  object . __new__ ( cls );
        pub fn __repr__ ( self )  {
        args = [ type_repr ( self . type ) , self . content , self . name ];
        while args && args [ -1 ] is None /* Option */  {
        del args [ -1 ];
        return  "%s(%s)" % ( self . __class__ . __name__ , ", " . join ( map ( repr , args ) ) );
        pub fn optimize ( self )  {
        "
        A subclass can define this as a hook for optimizations.

        Returns either self || another node with the same effect.
        ";
        return  self;
        pub fn match ( &self, node , results = None /* Option */ )  {
        "
        Does this pattern exactly match a node?

        Returns true if it matches, false if not.

        If results == !None /* Option */, it must be a dict which will be
        updated with the nodes matching named subpatterns.

        Default implementation for non-wildcard patterns.
        ";
        if self . type is !None /* Option */ && node . type != self . type {
        return  false;
        if self . content is !None /* Option */ {
        r = None /* Option */;
        if results is !None /* Option */ {
        r = { };
        if !self . _submatch ( node , r ) {
        return  false;
        if r {
        results . update ( r );
        if results is !None /* Option */ && self . name {
        results [ self . name ] = node;
        return  true;
        pub fn match_seq ( &self, nodes , results = None /* Option */ )  {
        "
        Does this pattern exactly match a sequence of nodes?

        Default implementation for non-wildcard patterns.
        ";
        if len ( nodes ) != 1 {
        return  false;
        return  self . match ( nodes [ 0 ] , results );
        pub fn generate_matches ( &self, nodes )  {
        "
        Generator yielding all matches for this pattern.

        Default implementation for non-wildcard patterns.
        ";
        r = { };
        if nodes && self . match ( nodes [ 0 ] , r ) {
        yield 1 , r;
        class LeafPattern ( BasePattern ) ;
        pub fn __init__ ( &self, type = None /* Option */ , content = None /* Option */ , name = None /* Option */ )  {
        "
        Initializer.  Takes optional type, content, && name.

        The type, if given must be a token type (< 256).  If !given,
        this matches any *leaf* node; the content may still be required.

        The content, if given, must be a string.

        If a name == given, the matching node == stored in the results
        dict under that key.
        ";
        if type is !None /* Option */ {
        assert 0 <= type < 256 , type;
        if content is !None /* Option */ {
        assert isinstance ( content , str ) , repr ( content );
        self . type = type;
        self . content = content;
        self . name = name;
        pub fn match ( &self, node , results = None /* Option */ )  {
        "Override match() to insist on a leaf node.";
        if !isinstance ( node , Leaf ) {
        return  false;
        return  BasePattern . match ( self , node , results );
        pub fn _submatch ( &self, node , results = None /* Option */ )  {
        "
        Match the pattern's content to the node's children.

        This assumes the node type matches && self.content == !None /* Option */.

        Returns true if it matches, false if not.

        If results == !None /* Option */, it must be a dict which will be
        updated with the nodes matching named subpatterns.

        When returning false, the results dict may still be updated.
        ";
        return  self . content == node . value;
        class NodePattern ( BasePattern ) ;
        wildcards = false;
        pub fn __init__ ( &self, type = None /* Option */ , content = None /* Option */ , name = None /* Option */ )  {
        "
        Initializer.  Takes optional type, content, && name.

        The type, if given, must be a symbol type (>= 256).  If the
        type == None /* Option */ this matches *any* single node (leaf || not),
        except if content == !None /* Option */, in which it only matches
        non-leaf nodes that also match the content pattern.

        The content, if !None /* Option */, must be a sequence of Patterns that
        must match the node's children exactly.  If the content is
        given, the type must !be None /* Option */.

        If a name == given, the matching node == stored in the results
        dict under that key.
        ";
        if type is !None /* Option */ {
        assert type >= 256 , type;
        if content is !None /* Option */ {
        assert !isinstance ( content , str ) , repr ( content );
        content = list ( content );
        for i , item in enumerate ( content ) .iter() {
        assert isinstance ( item , BasePattern ) , ( i , item );
        if isinstance ( item , WildcardPattern ) {
        self . wildcards = true;
        self . type = type;
        self . content = content;
        self . name = name;
        pub fn _submatch ( &self, node , results = None /* Option */ )  {
        "
        Match the pattern's content to the node's children.

        This assumes the node type matches && self.content == !None /* Option */.

        Returns true if it matches, false if not.

        If results == !None /* Option */, it must be a dict which will be
        updated with the nodes matching named subpatterns.

        When returning false, the results dict may still be updated.
        ";
        if self . wildcards {
        for c , r in generate_matches ( self . content , node . children ) .iter() {
        if c == len ( node . children ) {
        if results is !None /* Option */ {
        results . update ( r );
        return  true;
        return  false;
        if len ( self . content ) != len ( node . children ) {
        return  false;
        for subpattern , child in zip ( self . content , node . children ) .iter() {
        if !subpattern . match ( child , results ) {
        return  false;
        return  true;
        class WildcardPattern ( BasePattern ) ;
        "
    A wildcard pattern can match zero || more nodes.

    This has all the flexibility needed to implement patterns like:

    .*      .+      .?      .{m,n}
    (a b c | d e | f)
    (...)*  (...)+  (...)?  (...){m,n}

    except it always uses non-greedy matching.
    ";
        pub fn __init__ ( &self, content = None /* Option */ , min = 0 , max = HUGE , name = None /* Option */ )  {
        "
        Initializer.

        Args:
            content: optional sequence of subsequences of patterns;
                     if absent, matches one node;
                     if present, each subsequence == an alternative [*]
            min: optional minimum number of times to match, default 0
            max: optional maximum number of times to match, default HUGE
            name: optional name assigned to this match

        [*] Thus, if content == [[a, b, c], [d, e], [f, g, h]] this is
            equivalent to (a b c | d e | f g h); if content == None /* Option */,
            this == equivalent to '.' in regular expression terms.
            The min && max parameters work as follows:
                min=0, max=maxint: .*
                min=1, max=maxint: .+
                min=0, max=1: .?
                min=1, max=1: .
            If content == !None /* Option */, replace the dot with the parenthesized
            list of alternatives, e.g. (a b c | d e | f g h)*
        ";
        assert 0 <= min <= max <= HUGE , ( min , max );
        if content is !None /* Option */ {
        content = tuple ( map ( tuple , content ) );
        assert len ( content ) , repr ( content );
        for alt in content .iter() {
        assert len ( alt ) , repr ( alt );
        self . content = content;
        self . min = min;
        self . max = max;
        self . name = name;
        pub fn optimize ( self )  {
        "Optimize certain stacked wildcard patterns.";
        subpattern = None /* Option */;
        if ( self . content is !None /* Option */ and {
        len ( self . content ) == 1 && len ( self . content [ 0 ] ) == 1 ) ;
        subpattern = self . content [ 0 ] [ 0 ];
        if self . min == 1 && self . max == 1 {
        if self . content is None /* Option */ {
        return  NodePattern ( name = self . name );
        if subpattern is !None /* Option */ && self . name == subpattern . name {
        return  subpattern . optimize ( );
        if ( self . min <= 1 && isinstance ( subpattern , WildcardPattern ) and {
        subpattern . min <= 1 && self . name == subpattern . name ) ;
        return  WildcardPattern ( subpattern . content ,;
        self . min * subpattern . min ,;
        self . max * subpattern . max ,;
        subpattern . name );
        return  self;
        pub fn match ( &self, node , results = None /* Option */ )  {
        "Does this pattern exactly match a node?";
        return  self . match_seq ( [ node ] , results );
        pub fn match_seq ( &self, nodes , results = None /* Option */ )  {
        "Does this pattern exactly match a sequence of nodes?";
        for c , r in self . generate_matches ( nodes ) .iter() {
        if c == len ( nodes ) {
        if results is !None /* Option */ {
        results . update ( r );
        if self . name {
        results [ self . name ] = list ( nodes );
        return  true;
        return  false;
        pub fn generate_matches ( &self, nodes )  {
        "
        Generator yielding matches for a sequence of nodes.

        Args:
            nodes: sequence of nodes

        Yields:
            (count, results) tuples where:
            count: the match comprises nodes[:count];
            results: dict containing named submatches.
        ";
        if self . content is None /* Option */ {
        for count in range ( self . min , 1 + min ( len ( nodes ) , self . max ) ) .iter() {
        r = { };
        if self . name {
        r [ self . name ] = nodes [ : count ];
        yield count , r;
        } else if self . name == "bare_name" {
        yield self . _bare_name_matches ( nodes );
        } else {
        if hasattr ( sys , "getrefcount" ) {
        save_stderr = sys . stderr;
        sys . stderr = StringIO ( );
        // try {
        for count , r in self . _recursive_matches ( nodes , 0 ) .iter() {
        if self . name {
        r [ self . name ] = nodes [ : count ];
        yield count , r;
        // } catch  RuntimeError  {
        for count , r in self . _iterative_matches ( nodes ) .iter() {
        if self . name {
        r [ self . name ] = nodes [ : count ];
        yield count , r;
        // } finally {
        if hasattr ( sys , "getrefcount" ) {
        sys . stderr = save_stderr;
        pub fn _iterative_matches ( &self, nodes )  {
        "Helper to iteratively yield the matches.";
        nodelen = len ( nodes );
        if 0 >= self . min {
        yield 0 , { };
        results = [ ];
        for alt in self . content .iter() {
        for c , r in generate_matches ( alt , nodes ) .iter() {
        yield c , r;
        results . append ( ( c , r ) );
        while results  {
        new_results = [ ];
        for c0 , r0 in results .iter() {
        if c0 < nodelen && c0 <= self . max {
        for alt in self . content .iter() {
        for c1 , r1 in generate_matches ( alt , nodes [ c0 : ] ) .iter() {
        if c1 > 0 {
        r = { };
        r . update ( r0 );
        r . update ( r1 );
        yield c0 + c1 , r;
        new_results . append ( ( c0 + c1 , r ) );
        results = new_results;
        pub fn _bare_name_matches ( &self, nodes )  {
        "Special optimized matcher for bare_name.";
        count = 0;
        r = { };
        done = false;
        max = len ( nodes );
        while !done && count < max  {
        done = true;
        for leaf in self . content .iter() {
        if leaf [ 0 ] . match ( nodes [ count ] , r ) {
        count + = 1;
        done = false;
        break;
        r [ self . name ] = nodes [ : count ];
        return  count , r;
        pub fn _recursive_matches ( &self, nodes , count )  {
        "Helper to recursively yield the matches.";
        assert self . content == !None /* Option */;
        if count >= self . min {
        yield 0 , { };
        if count < self . max {
        for alt in self . content .iter() {
        for c0 , r0 in generate_matches ( alt , nodes ) .iter() {
        for c1 , r1 in self . _recursive_matches ( nodes [ c0 : ] , count + 1 ) .iter() {
        r = { };
        r . update ( r0 );
        r . update ( r1 );
        yield c0 + c1 , r;
        class NegatedPattern ( BasePattern ) ;
        pub fn __init__ ( &self, content = None /* Option */ )  {
        "
        Initializer.

        The argument == either a pattern || None /* Option */.  If it == None /* Option */, this
        only matches an empty sequence (effectively '$' in regex
        lingo).  If it == !None /* Option */, this matches whenever the argument
        pattern doesn't have any matches.
        ";
        if content is !None /* Option */ {
        assert isinstance ( content , BasePattern ) , repr ( content );
        self . content = content;
        pub fn match ( &self, node )  {
        return  false;
        pub fn match_seq ( &self, nodes )  {
        return  len ( nodes ) == 0;
        pub fn generate_matches ( &self, nodes )  {
        if self . content is None /* Option */ {
        if len ( nodes ) == 0 {
        yield 0 , { };
        } else {
        for c , r in self . content . generate_matches ( nodes ) .iter() {
        return;
        yield 0 , { };
        pub fn generate_matches ( patterns , nodes )  {
        "
    Generator yielding matches for a sequence of patterns && nodes.

    Args:
        patterns: a sequence of patterns
        nodes: a sequence of nodes

    Yields:
        (count, results) tuples where:
        count: the entire sequence of patterns matches nodes[:count];
        results: dict containing named submatches.
        ";
        if !patterns {
        yield 0 , { };
        } else {
        p , rest = patterns [ 0 ] , patterns [ 1 : ];
        for c0 , r0 in p . generate_matches ( nodes ) .iter() {
        if !rest {
        yield c0 , r0;
        } else {
        for c1 , r1 in generate_matches ( rest , nodes [ c0 : ] ) .iter() {
        r = { };
        r . update ( r0 );
        r . update ( r1 );
        yield c0 + c1 , r;
}

