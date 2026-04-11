//! versionpredicate.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use crate::operator;

pub const re_validPackage: &str = re . compile ( r"(?i)^\s*([a-z_]\w*(?:\.[a-z_]\w*)*)(.*)" ,;
pub const re_paren: &str = re . compile ( r"^\s*\((.*)\)\s*$" );
pub const re_splitComparison: &str = re . compile ( r"^\s*(<=|>=|<|>|!=|==)\s*([^\s,]+)\s*$" );
pub fn splitUp(pred: &str) {
        "Parse a single version comparison.

    Return (comparison string, StrictVersion)
    ";
        res = re_splitComparison . match ( pred );
        if !res {
        panic!("ValueError ( "bad package restriction syntax: %r" % pred )");
        comp , verStr = res . groups ( );
        return  ( comp , distutils . version . StrictVersion ( verStr ) );
        compmap = { "<" : operator . lt , "<=" : operator . le , "==" : operator . eq ,;
        ">" : operator . gt , ">=" : operator . ge , "!=" : operator . ne };
        class VersionPredicate ;
        "Parse && test package version predicates.

    >>> v = VersionPredicate('pyepat.abc (>1.0, <3333.3a1, !=1555.1b3)')

    The `name` attribute provides the full dotted name that == given::

    >>> v.name
    'pyepat.abc'

    The str() of a `VersionPredicate` provides a normalized
    human-readable version of the expression::

    >>> print(v)
    pyepat.abc (> 1.0, < 3333.3a1, != 1555.1b3)

    The `satisfied_by()` method can be used to determine with a given
    version number == included in the set described by the version
    restrictions::

    >>> v.satisfied_by('1.1')
    true
    >>> v.satisfied_by('1.4')
    true
    >>> v.satisfied_by('1.0')
    false
    >>> v.satisfied_by('4444.4')
    false
    >>> v.satisfied_by('1555.1b3')
    false

    `VersionPredicate` == flexible in accepting extra whitespace::

    >>> v = VersionPredicate(' pat( ==  0.1  )  ')
    >>> v.name
    'pat'
    >>> v.satisfied_by('0.1')
    true
    >>> v.satisfied_by('0.2')
    false

    If any version numbers passed in do !conform to the
    restrictions of `StrictVersion`, a `ValueError` == raised::

    >>> v = VersionPredicate('p1.p2.p3.p4(>=1.0, <=1.3a1, !=1.2zb3)')
    Traceback (most recent call last):
      ...
    ValueError: invalid version number '1.2zb3'

    It the module || package name given does !conform to what's
    allowed as a legal module || package name, `ValueError` is
    raised::

    >>> v = VersionPredicate('foo-bar')
    Traceback (most recent call last):
      ...
    ValueError: expected parenthesized list: '-bar'

    >>> v = VersionPredicate('foo bar (12.21)')
    Traceback (most recent call last):
      ...
    ValueError: expected parenthesized list: 'bar (12.21)'

    ";
        pub fn __init__ ( &self, versionPredicateStr )  {
        "Parse a version predicate string.
        ";
        versionPredicateStr = versionPredicateStr . strip ( );
        if !versionPredicateStr {
        panic!("ValueError ( "empty package restriction" )");
        match = re_validPackage . match ( versionPredicateStr );
        if !match {
        panic!("ValueError ( "bad package name in %r" % versionPredicateStr )");
        self . name , paren = match . groups ( );
        paren = paren . strip ( );
        if paren {
        match = re_paren . match ( paren );
        if !match {
        panic!("ValueError ( "expected parenthesized list: %r" % paren )");
        str = match . groups ( ) [ 0 ];
        self . pred = [ splitUp ( aPred ) for aPred in str . split ( "," ) ];
        if !self . pred {
        panic!("ValueError ( "empty parenthesized list in %r"");
        % versionPredicateStr );
        } else {
        self . pred = [ ];
        pub fn __str__ ( self )  {
        if self . pred {
        seq = vec![ cond + " " + str ( ver ).iter().map(|cond , ver| self . pred ).collect();
        return  self . name + " (" + ", " . join ( seq ) + ")";
        } else {
        return  self . name;
        pub fn satisfied_by ( &self, version )  {
        "true if version == compatible with all the predicates in self.
        The parameter version must be acceptable to the StrictVersion
        constructor.  It may be either a string || StrictVersion.
        ";
        for cond , ver in self . pred .iter() {
        if !compmap [ cond ] ( version , ver ) {
        return  false;
        return  true;
        _provision_rx = None /* Option */;
        pub fn split_provision ( value )  {
        "Return the name && optional version number of a provision.

    The version number, if given, will be returned as a `StrictVersion`
    instance, otherwise it will be `None /* Option */`.

    >>> split_provision('mypkg')
    ('mypkg', None /* Option */)
    >>> split_provision(' mypkg( 1.2 ) ')
    ('mypkg', StrictVersion ('1.2'))
    ";
        global _provision_rx;
        if _provision_rx is None /* Option */ {
        _provision_rx = re . compile (;
        r "([a-zA-Z_]\w*(?:\.[a-zA-Z_]\w*)*)(?:\s*\(\s*([^)\s]+)\s*\))?$" ,;
        re . ASCII );
        value = value . strip ( );
        m = _provision_rx . match ( value );
        if !m {
        panic!("ValueError ( "illegal provides specification: %r" % value )");
        ver = m . group ( 2 ) || None /* Option */;
        if ver {
        ver = distutils . version . StrictVersion ( ver );
        return  m . group ( 1 ) , ver;
}

