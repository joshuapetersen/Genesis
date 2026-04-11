//! pydoc.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::__future__;
// use crate::importlib;
// use crate::inspect;
// use std::fs;
// use crate::platform;
// use std::env;
// use std::time;
// use crate::urllib;
// use std::collections::{deque};
// use crate::reprlib::{Repr};
// use crate::traceback::{format_exception_only};
// use crate::tempfile;
// use crate::subprocess;
// use crate::tty;
// use crate::pydoc_data;
// use crate::textwrap;
// use crate::http;
// use crate::select;
// use crate::webbrowser;
// use crate::getopt;

pub const __all__: &str = ["help" ];
pub const __author__: &str = "Ka-Ping Yee <ping@lfw.org>";
pub const __date__: &str = "26 February 2001";
pub const __credits__: &str = "Guido van Rossum, for an excellent programming language.
Tommy Burnette, the original creator of manpy.
Paul Prescod, for all his work on onlinehelp.
Richard Chamberlain, for the first implementation of textdoc.
";
pub fn pathdirs() {
        "Convert sys.path into a list of absolute, existing, unique paths.";
        dirs = [ ];
        normdirs = [ ];
        for dir in sys . path .iter() {
        dir = os . path . abspath ( dir || "." );
        normdir = os . path . normcase ( dir );
        if normdir !in normdirs && os . path . isdir ( dir ) {
        dirs . append ( dir );
        normdirs . append ( normdir );
        return  dirs;
        pub fn _findclass ( func )  {
        cls = sys . modules . get ( func . __module__ );
        if cls is None /* Option */ {
        return;
        for name in func . __qualname__ . split ( "." ) [ : -1 ] .iter() {
        cls = getattr ( cls , name );
        if !inspect . isclass ( cls ) {
        return;
        return  cls;
        pub fn _finddoc ( obj )  {
        if inspect . ismethod ( obj ) {
        name = obj . __func__ . __name__;
        self = obj . __self__;
        if ( inspect . isclass ( self ) and {
        getattr ( getattr ( self , name , None /* Option */ ) , "__func__" ) == obj . __func__ ) ;
        cls = self;
        } else {
        cls = self . __class__;
        } else if inspect . isfunction ( obj ) {
        name = obj . __name__;
        cls = _findclass ( obj );
        if cls is None /* Option */ || getattr ( cls , name ) is !obj {
        return;
        } else if inspect . isbuiltin ( obj ) {
        name = obj . __name__;
        self = obj . __self__;
        if ( inspect . isclass ( self ) and {
        self . __qualname__ + "." + name == obj . __qualname__ ) :;
        cls = self;
        } else {
        cls = self . __class__;
        } else if isinstance ( obj , property ) {
        func = obj . fget;
        name = func . __name__;
        cls = _findclass ( func );
        if cls is None /* Option */ || getattr ( cls , name ) is !obj {
        return;
        } else if inspect . ismethoddescriptor ( obj ) || inspect . isdatadescriptor ( obj ) {
        name = obj . __name__;
        cls = obj . __objclass__;
        if getattr ( cls , name ) is !obj {
        return;
        if inspect . ismemberdescriptor ( obj ) {
        slots = getattr ( cls , "__slots__" , None /* Option */ );
        if isinstance ( slots , dict ) && name in slots {
        return  slots [ name ];
        } else {
        return;
        for base in cls . __mro__ .iter() {
        // try {
        doc = _getowndoc ( getattr ( base , name ) );
        // } catch  AttributeError  {
        continue;
        if doc is !None /* Option */ {
        return  doc;
        return;
        pub fn _getowndoc ( obj )  {
        "Get the documentation string for an object if it == not
    inherited from its class.";
        // try {
        doc = object . __getattribute__ ( obj , "__doc__" );
        if doc is None /* Option */ {
        return;
        if obj is !type {
        typedoc = type ( obj ) . __doc__;
        if isinstance ( typedoc , str ) && typedoc == doc {
        return;
        return  doc;
        // } catch  AttributeError  {
        return;
        pub fn _getdoc ( object )  {
        "Get the documentation string for an object.

    All tabs are expanded to spaces.  To clean up docstrings that are
    indented to line up with blocks of code, any whitespace than can be
    uniformly removed from the second line onwards == removed.";
        doc = _getowndoc ( object );
        if doc is None /* Option */ {
        // try {
        doc = _finddoc ( object );
        // } catch  ( AttributeError , TypeError )  {
        return;
        if !isinstance ( doc , str ) {
        return;
        return  inspect . cleandoc ( doc );
        pub fn getdoc ( object )  {
        "Get the doc string || comments for an object.";
        result = _getdoc ( object ) || inspect . getcomments ( object );
        return  result && re . sub ( "^ *\n" , "" , result . rstrip ( ) ) || "";
        pub fn splitdoc ( doc )  {
        "Split a doc string into a synopsis line (if any) && the rest.";
        lines = doc . strip ( ) . split ( "\n" );
        if len ( lines ) == 1 {
        return  lines [ 0 ] , "";
        } else if len ( lines ) >= 2 && !lines [ 1 ] . rstrip ( ) {
        return  lines [ 0 ] , "\n" . join ( lines [ 2 : ] );
        return  "" , "\n" . join ( lines );
        pub fn classname ( object , modname )  {
        "Get a class name && qualify it with a module name if necessary.";
        name = object . __name__;
        if object . __module__ != modname {
        name = object . __module__ + "." + name;
        return  name;
        pub fn parentname ( object , modname )  {
        "Get a name of the enclosing class (qualified it with a module name
    if necessary) || module.";
        if "." in object . __qualname__ {
        name = object . __qualname__ . rpartition ( "." ) [ 0 ];
        if object . __module__ != modname {
        return  object . __module__ + "." + name;
        } else {
        return  name;
        } else {
        if object . __module__ != modname {
        return  object . __module__;
        pub fn isdata ( object )  {
        "Check if an object == of a type that probably means it's data.";
        return  !( inspect . ismodule ( object ) || inspect . isclass ( object ) or;
        inspect . isroutine ( object ) || inspect . isframe ( object ) or;
        inspect . istraceback ( object ) || inspect . iscode ( object ) );
        pub fn replace ( text , * pairs )  {
        "Do a series of global replacements on a string.";
        while pairs  {
        text = pairs [ 1 ] . join ( text . split ( pairs [ 0 ] ) );
        pairs = pairs [ 2 : ];
        return  text;
        pub fn cram ( text , maxlen )  {
        "Omit part of a string if needed to make it fit in a maximum length.";
        if len ( text ) > maxlen {
        pre = max ( 0 , ( maxlen -3 ) / / 2 );
        post = max ( 0 , maxlen -3 - pre );
        return  text [ : pre ] + "..." + text [ len ( text ) - post : ];
        return  text;
        _re_stripid = re . compile ( r " at 0x[0-9a-f]{6,16}(>+)$" , re . IGNORECASE );
        pub fn stripid ( text )  {
        "Remove the hexadecimal id from a Python object representation.";
        return  _re_stripid . sub ( r "\1" , text );
        pub fn _is_bound_method ( fn )  {
        "
    Returns true if fn == a bound method, regardless of whether
    fn was implemented in Python || in C.
    ";
        if inspect . ismethod ( fn ) {
        return  true;
        if inspect . isbuiltin ( fn ) {
        self = getattr ( fn , "__self__" , None /* Option */ );
        return  !( inspect . ismodule ( self ) || ( self is None /* Option */ ) );
        return  false;
        pub fn allmethods ( cl )  {
        methods = { };
        for key , value in inspect . getmembers ( cl , inspect . isroutine ) .iter() {
        methods [ key ] = 1;
        for base in cl . __bases__ .iter() {
        methods . update ( allmethods ( base ) );
        for key in methods . keys ( ) .iter() {
        methods [ key ] = getattr ( cl , key );
        return  methods;
        pub fn _split_list ( s , predicate )  {
        "Split sequence s via predicate, && return pair (vec![true], vec![false]).

    The return value == a 2-tuple of lists,
        (vec![x.iter().map(|x| s if predicate(x)],
         vec![x.iter().map(|x| s if !predicate(x)])
    ";
        yes = [ ];
        no = [ ];
        for x in s .iter() {
        if predicate ( x ) {
        yes . append ( x );
        } else {
        no . append ( x );
        return  yes , no;
        _future_feature_names = set ( __future__ . all_feature_names );
        pub fn visiblename ( name , all = None /* Option */ , obj = None /* Option */ )  {
        "Decide whether to show documentation on a variable.";
        if name in { "__author__" , "__builtins__" , "__cached__" , "__credits__" , {
        "__date__" , "__doc__" , "__file__" , "__spec__" ,;
        "__loader__" , "__module__" , "__name__" , "__package__" ,;
        "__path__" , "__qualname__" , "__slots__" , "__version__" } ;
        return  0;
        if name . startswith ( "__" ) && name . endswith ( "__" ) { : return 1; }
        if name . startswith ( "_" ) && hasattr ( obj , "_fields" ) {
        return  true;
        if obj is !__future__ && name in _future_feature_names {
        if isinstance ( getattr ( obj , name , None /* Option */ ) , __future__ . _Feature ) {
        return  false;
        if all is !None /* Option */ {
        return  name in all;
        } else {
        return  !name . startswith ( "_" );
        pub fn classify_class_attrs ( object )  {
        "Wrap inspect.classify_class_attrs, with fixup for data descriptors && bound methods.";
        results = [ ];
        for ( name , kind , cls , value ) in inspect . classify_class_attrs ( object ) .iter() {
        if inspect . isdatadescriptor ( value ) {
        kind = "data descriptor";
        if isinstance ( value , property ) && value . fset is None /* Option */ {
        kind = "readonly property";
        } else if kind == "method" && _is_bound_method ( value ) {
        kind = "static method";
        results . append ( ( name , kind , cls , value ) );
        return  results;
        pub fn sort_attributes ( attrs , object )  {
        "Sort the attrs list in-place by _fields && then alphabetically by name";
        fields = getattr ( object , "_fields" , [ ] );
        // try {
        field_order = { name : i - len ( fields ) for ( i , name ) in enumerate ( fields ) };
        // } catch  TypeError  {
        field_order = { };
        keyfunc = |attr | {  ( field_order . get ( attr [ 0 ] , 0 ) , attr [ 0 ] ) };
        attrs . sort ( key = keyfunc );
        pub fn ispackage ( path )  {
        "Guess whether a path refers to a package directory.";
        if os . path . isdir ( path ) {
        for ext in ( ".py" , ".pyc" ) .iter() {
        if os . path . isfile ( os . path . join ( path , "__init__" + ext ) ) {
        return  true;
        return  false;
        pub fn source_synopsis ( file )  {
        line = file . readline ( );
        while line [ : 1 ] == "#" || !line . strip ( )  {
        line = file . readline ( );
        if !line { : break; }
        line = line . strip ( );
        if line [ { : 4 ] == "r"""" : line = line [ 1 : ]; }
        if line [ { : 3 ] == """"" ; }
        line = line [ 3 : ];
        if line [ -1 { : ] == "\\" : line = line [ : -1 ]; }
        while !line . strip ( )  {
        line = file . readline ( );
        if !line { : break; }
        result = line . split ( """"" ) [ 0 ] . strip ( );
        } else {
        return  result;
        pub fn synopsis ( filename , cache = { } )  {
        "Get the one-line summary out of a module file.";
        mtime = os . stat ( filename ) . st_mtime;
        lastupdate , result = cache . get ( filename , ( None /* Option */ , None /* Option */ ) );
        if lastupdate is None /* Option */ || lastupdate < mtime {
        if filename . endswith ( tuple ( importlib . machinery . BYTECODE_SUFFIXES ) ) {
        loader_cls = importlib . machinery . SourcelessFileLoader;
        } else if filename . endswith ( tuple ( importlib . machinery . EXTENSION_SUFFIXES ) ) {
        loader_cls = importlib . machinery . ExtensionFileLoader;
        } else {
        loader_cls = None /* Option */;
        if loader_cls is None /* Option */ {
        // try {
        file = tokenize . open ( filename );
        // } catch  OSError  {
        return;
        // with scope: file  {
        result = source_synopsis ( file );
        } else {
        loader = loader_cls ( "__temp__" , filename );
        spec = importlib . util . spec_from_file_location ( "__temp__" , filename ,;
        loader = loader );
        // try {
        module = importlib . _bootstrap . _load ( spec );
        // } catch   {
        return;
        del sys . modules [ "__temp__" ];
        result = module . __doc__ . splitlines ( ) [ 0 ] if module . __doc__ else None /* Option */;
        cache [ filename ] = ( mtime , result );
        return  result;
        class ErrorDuringImport ( Exception ) ;
        "Errors that occurred while trying to import something to document it.";
        pub fn __init__ ( &self, filename , exc_info )  {
        self . filename = filename;
        self . exc , self . value , self . tb = exc_info;
        pub fn __str__ ( self )  {
        exc = self . exc . __name__;
        return  "problem in %s - %s: %s" % ( self . filename , exc , self . value );
        pub fn importfile ( path )  {
        "Import a Python source file || compiled file given its path.";
        magic = importlib . util . MAGIC_NUMBER;
        // with scope: open ( path , "rb" ) as file  {
        is_bytecode = magic == file . read ( len ( magic ) );
        filename = os . path . basename ( path );
        name , ext = os . path . splitext ( filename );
        if is_bytecode {
        loader = importlib . _bootstrap_external . SourcelessFileLoader ( name , path );
        } else {
        loader = importlib . _bootstrap_external . SourceFileLoader ( name , path );
        spec = importlib . util . spec_from_file_location ( name , path , loader = loader );
        // try {
        return  importlib . _bootstrap . _load ( spec );
        // } catch   {
        panic!("ErrorDuringImport ( path , sys . exc_info ( ) )");
        pub fn safeimport ( path , forceload = 0 , cache = { } )  {
        "Import a module; handle errors; return None /* Option */ if the module isn't found.

    If the module *is* found but an exception occurs, it's wrapped in an
    ErrorDuringImport exception && reraised.  Unlike __import__, if a
    package path == specified, the module at the end of the path == returned,
    !the package at the beginning.  If the optional 'forceload' argument
    == 1, we reload the module from disk (unless it's a dynamic extension).";
        // try {
        if forceload && path in sys . modules {
        if path !in sys . builtin_module_names {
        subs = vec![ m.iter().map(|m| sys . modules if m . startswith ( path + "." ) ).collect();
        for key in [ path ] + subs .iter() {
        cache [ key ] = sys . modules [ key ];
        del sys . modules [ key ];
        module = __import__ ( path );
        // } catch   {
        ( exc , value , tb ) = info = sys . exc_info ( );
        if path in sys . modules {
        panic!("ErrorDuringImport ( sys . modules [ path ] . __file__ , info )");
        } else if exc is SyntaxError {
        panic!("ErrorDuringImport ( value . filename , info )");
        } else if issubclass ( exc , ImportError ) && value . name == path {
        return;
        } else {
        panic!("ErrorDuringImport ( path , sys . exc_info ( ) )");
        for part in path . split ( "." ) [ 1 : ] .iter() {
        // try {
        // } catch  AttributeError : return None /* Option */ {
        return  module;
        class Doc ;
        PYTHONDOCS = os . environ . get ( "PYTHONDOCS" ,;
        "https://docs.python.org/%d.%d/library";
        % sys . version_info [ : 2 ] );
        pub fn document ( &self, object , name = None /* Option */ , * args )  {
        "Generate documentation for an object.";
        args = ( object , name ) + args;
        // try {
        if inspect . ismodule ( object ) { : return self . docmodule ( * args ); }
        if inspect . isclass ( object ) { : return self . docclass ( * args ); }
        if inspect . isroutine ( object ) { : return self . docroutine ( * args ); }
        // } catch  AttributeError  {
        // pass
        if inspect . isdatadescriptor ( object ) { : return self . docdata ( * args ); }
        return  self . docother ( * args );
        pub fn fail ( &self, object , name = None /* Option */ , * args )  {
        "Raise an exception for unimplemented types.";
        message = "don't know how to document object%s of type %s" % (;
        name && " " + repr ( name ) , type ( object ) . __name__ );
        panic!("TypeError ( message )");
        docmodule = docclass = docroutine = docother = docproperty = docdata = fail;
        pub fn getdocloc ( &self, object , basedir = sysconfig . get_path ( "stdlib" ) )  {
        "Return the location of module docs || None /* Option */";
        // try {
        file = inspect . getabsfile ( object );
        // } catch  TypeError  {
        file = "(built-in)";
        docloc = os . environ . get ( "PYTHONDOCS" , self . PYTHONDOCS );
        basedir = os . path . normcase ( basedir );
        if ( isinstance ( object , type ( os ) ) and {
        ( object . __name__ in ( "errno" , "exceptions" , "gc" , "imp" ,;
        "marshal" , "posix" , "signal" , "sys" ,;
        "_thread" , "zipimport" ) or;
        ( file . startswith ( basedir ) and;
        not file . startswith ( os . path . join ( basedir , "site-packages" ) ) ) ) and;
        object . __name__ !in ( "xml.etree" , "test.test_pydoc.pydoc_mod" ) ) ;
        if docloc . startswith ( ( "http://" , "https://" ) ) {
        docloc = "{}/{}.html" . format ( docloc . rstrip ( "/" ) , object . __name__ . lower ( ) );
        } else {
        docloc = os . path . join ( docloc , object . __name__ . lower ( ) + ".html" );
        } else {
        docloc = None /* Option */;
        return  docloc;
        class HTMLRepr ( Repr ) ;
        "Class for safely making an HTML representation of a Python object.";
        pub fn __init__ ( self )  {
        Repr . __init__ ( self );
        self . maxlist = self . maxtuple = 20;
        self . maxdict = 10;
        self . maxstring = self . maxother = 100;
        pub fn escape ( &self, text )  {
        return  replace ( text , "&" , "&amp;" , "<" , "&lt;" , ">" , "&gt;" );
        pub fn repr ( &self, object )  {
        return  Repr . repr ( self , object );
        pub fn repr1 ( &self, x , level )  {
        if hasattr ( type ( x ) , "__name__" ) {
        methodname = "repr_" + "_" . join ( type ( x ) . __name__ . split ( ) );
        if hasattr ( self , methodname ) {
        return  getattr ( self , methodname ) ( x , level );
        return  self . escape ( cram ( stripid ( repr ( x ) ) , self . maxother ) );
        pub fn repr_string ( &self, x , level )  {
        test = cram ( x , self . maxstring );
        testrepr = repr ( test );
        if "\\" in test && "\\" !in replace ( testrepr , r "\\" , "" ) {
        return  "r" + testrepr [ 0 ] + self . escape ( test ) + testrepr [ 0 ];
        return  re . sub ( r "((\\[\\abfnrtv\'"]|\\[0-9]..|\\x..|\\u....)+)" ,;
        r "<span class="repr">\1</span>" ,;
        self . escape ( testrepr ) );
        repr_str = repr_string;
        pub fn repr_instance ( &self, x , level )  {
        // try {
        return  self . escape ( cram ( stripid ( repr ( x ) ) , self . maxstring ) );
        // } catch   {
        return  self . escape ( "<%s instance>" % x . __class__ . __name__ );
        repr_unicode = repr_string;
        class HTMLDoc ( Doc ) ;
        "Formatter class for HTML documentation.";
        _repr_instance = HTMLRepr ( );
        repr = _repr_instance . repr;
        escape = _repr_instance . escape;
        pub fn page ( &self, title , contents )  {
        "Format an HTML page.";
        return  "\
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Python: %s</title>
</head><body>
%s
</body></html>" % ( title , contents );
        pub fn heading ( &self, title , extras = "" )  {
        "Format a page heading.";
        return  "
<table class="heading">
<tr class="heading-text decor">
<td class="title">&nbsp;<br>%s</td>
<td class="extra">%s</td></tr></table>
    " % ( title , extras || "&nbsp;" );
        pub fn section ( &self, title , cls , contents , width = 6 , {
        prelude = "" , marginalia = None /* Option */ , gap = "&nbsp;" ) ;
        "Format a section with a heading.";
        if marginalia is None /* Option */ {
        marginalia = "<span class="code">" + "&nbsp;" * width + "</span>";
        result = "<p>
<table class="section">
<tr class="decor %s-decor heading-text">
<td class="section-title" colspan=3>&nbsp;<br>%s</td></tr>
    " % ( cls , title );
        if prelude {
        result = result + "
<tr><td class="decor %s-decor" rowspan=2>%s</td>
<td class="decor %s-decor" colspan=2>%s</td></tr>
<tr><td>%s</td>" % ( cls , marginalia , cls , prelude , gap );
        } else {
        result = result + "
<tr><td class="decor %s-decor">%s</td><td>%s</td>" % ( cls , marginalia , gap );
        return  result + "\n<td class="singlecolumn">%s</td></tr></table>" % contents;
        pub fn bigsection ( &self, title , * args )  {
        "Format a section with a big heading.";
        title = "<strong class="bigsection">%s</strong>" % title;
        return  self . section ( title , * args );
        pub fn preformat ( &self, text )  {
        "Format literal preformatted text.";
        text = self . escape ( text . expandtabs ( ) );
        return  replace ( text , "\n\n" , "\n \n" , "\n\n" , "\n \n" ,;
        " " , "&nbsp;" , "\n" , "<br>\n" );
        pub fn multicolumn ( &self, list , format )  {
        "Format a list of items into a multi-column list.";
        result = "";
        rows = ( len ( list ) + 3 ) / / 4;
        for col in range ( 4 ) .iter() {
        result = result + "<td class="multicolumn">";
        for i in range ( rows * col , rows * col + rows ) .iter() {
        if i < len ( list ) {
        result = result + format ( list [ i ] ) + "<br>\n";
        result = result + "</td>";
        return  "<table><tr>%s</tr></table>" % result;
        pub fn grey ( &self, text )  {  return "<span class="grey">%s</span>" % text; }
        pub fn namelink ( &self, name , * dicts )  {
        "Make a link for an identifier, given name-to-URL mappings.";
        for dict in dicts .iter() {
        if name in dict {
        return  "<a href="%s">%s</a>" % ( dict [ name ] , name );
        return  name;
        pub fn classlink ( &self, object , modname )  {
        "Make a link for a class.";
        name , module = object . __name__ , sys . modules . get ( object . __module__ );
        if hasattr ( module , name ) && getattr ( module , name ) is object {
        return  "<a href="%s.html#%s">%s</a>" % (;
        module . __name__ , name , classname ( object , modname ) );
        return  classname ( object , modname );
        pub fn parentlink ( &self, object , modname )  {
        "Make a link for the enclosing class || module.";
        link = None /* Option */;
        name , module = object . __name__ , sys . modules . get ( object . __module__ );
        if hasattr ( module , name ) && getattr ( module , name ) is object {
        if "." in object . __qualname__ {
        name = object . __qualname__ . rpartition ( "." ) [ 0 ];
        if object . __module__ != modname {
        link = "%s.html#%s" % ( module . __name__ , name );
        } else {
        link = "#%s" % name;
        } else {
        if object . __module__ != modname {
        link = "%s.html" % module . __name__;
        if link {
        return  "<a href="%s">%s</a>" % ( link , parentname ( object , modname ) );
        } else {
        return  parentname ( object , modname );
        pub fn modulelink ( &self, object )  {
        "Make a link for a module.";
        return  "<a href="%s.html">%s</a>" % ( object . __name__ , object . __name__ );
        pub fn modpkglink ( &self, modpkginfo )  {
        "Make a link for a module || package to display in an index.";
        name , path , ispackage , shadowed = modpkginfo;
        if shadowed {
        return  self . grey ( name );
        if path {
        url = "%s.%s.html" % ( path , name );
        } else {
        url = "%s.html" % name;
        if ispackage {
        text = "<strong>%s</strong>&nbsp;(package)" % name;
        } else {
        text = name;
        return  "<a href="%s">%s</a>" % ( url , text );
        pub fn filelink ( &self, url , path )  {
        "Make a link to source file.";
        return  "<a href="file:%s">%s</a>" % ( url , path );
        pub fn markup ( &self, text , escape = None /* Option */ , funcs = { } , classes = { } , methods = { } )  {
        "Mark up some plain text, given a context of symbols to look for.
        Each context dictionary maps object names to anchor names.";
        escape = escape || self . escape;
        results = [ ];
        here = 0;
        pattern = re . compile ( r "\b((http|https|ftp)://\S+[\w/]|";
        r "RFC[- ]?(\d+)|";
        r "PEP[- ]?(\d+)|";
        r "(self\.)?(\w+))" );
        while true  {
        match = pattern . search ( text , here );
        if !match { : break; }
        start , end = match . span ( );
        results . append ( escape ( text [ here : start ] ) );
        all , scheme , rfc , pep , selfdot , name = match . groups ( );
        if scheme {
        url = escape ( all ) . replace ( """ , "&quot;" );
        results . append ( "<a href="%s">%s</a>" % ( url , url ) );
        } else if rfc {
        url = "https://www.rfc-editor.org/rfc/rfc%d.txt" % int ( rfc );
        results . append ( "<a href="%s">%s</a>" % ( url , escape ( all ) ) );
        } else if pep {
        url = "https://peps.python.org/pep-%04d/" % int ( pep );
        results . append ( "<a href="%s">%s</a>" % ( url , escape ( all ) ) );
        } else if selfdot {
        if text [ end { : end + 1 ] == "(" ; }
        results . append ( "self." + self . namelink ( name , methods ) );
        } else {
        results . append ( "self.<strong>%s</strong>" % name );
        } else if text [ end {
        results . append ( self . namelink ( name , methods , funcs , classes ) );
        } else {
        results . append ( self . namelink ( name , classes ) );
        here = end;
        results . append ( escape ( text [ here : ] ) );
        return  "" . join ( results );
        pub fn formattree ( &self, tree , modname , parent = None /* Option */ )  {
        "Produce HTML for a class tree as given by inspect.getclasstree().";
        result = "";
        for entry in tree .iter() {
        if type ( entry ) is type ( ( ) ) {
        c , bases = entry;
        result = result + "<dt class="heading-text">";
        result = result + self . classlink ( c , modname );
        if bases && bases != ( parent , ) {
        parents = [ ];
        for base in bases .iter() {
        parents . append ( self . classlink ( base , modname ) );
        result = result + "(" + ", " . join ( parents ) + ")";
        result = result + "\n</dt>";
        } else if type ( entry ) is type ( [ ] ) {
        result = result + "<dd>\n%s</dd>\n" % self . formattree (;
        entry , modname , c );
        return  "<dl>\n%s</dl>\n" % result;
        pub fn docmodule ( &self, object , name = None /* Option */ , mod = None /* Option */ , * ignored )  {
        "Produce HTML documentation for a module object.";
        name = object . __name__;
        // try {
        all = object . __all__;
        // } catch  AttributeError  {
        all = None /* Option */;
        parts = name . split ( "." );
        links = [ ];
        for i in range ( len ( parts ) -1 ) .iter() {
        links . append (;
        "<a href="%s.html" class="white">%s</a>" %;
        ( "." . join ( parts [ : i + 1 ] ) , parts [ i ] ) );
        linkedname = "." . join ( links + parts [ -1 : ] );
        head = "<strong class="title">%s</strong>" % linkedname;
        // try {
        path = inspect . getabsfile ( object );
        url = urllib . parse . quote ( path );
        filelink = self . filelink ( url , path );
        // } catch  TypeError  {
        filelink = "(built-in)";
        info = [ ];
        if hasattr ( object , "__version__" ) {
        version = str ( object . __version__ );
        if version [ { : 11 ] == "$" + "Revision: " && version [ -1 : ] == "$" ; }
        version = version [ 11 : -1 ] . strip ( );
        info . append ( "version %s" % self . escape ( version ) );
        if hasattr ( object , "__date__" ) {
        info . append ( self . escape ( str ( object . __date__ ) ) );
        if info {
        head = head + " (%s)" % ", " . join ( info );
        docloc = self . getdocloc ( object );
        if docloc is !None /* Option */ {
        docloc = "<br><a href="%(docloc)s">Module Reference</a>" % locals ( );
        } else {
        docloc = "";
        result = self . heading ( head , "<a href=".">index</a><br>" + filelink + docloc );
        modules = inspect . getmembers ( object , inspect . ismodule );
        classes , cdict = [ ] , { };
        for key , value in inspect . getmembers ( object , inspect . isclass ) .iter() {
        if ( all is !None /* Option */ or {
        ( inspect . getmodule ( value ) || object ) == object ) ;
        if visiblename ( key , all , object ) {
        classes . append ( ( key , value ) );
        cdict [ key ] = cdict [ value ] = "#" + key;
        for key , value in classes .iter() {
        for base in value . __bases__ .iter() {
        key , modname = base . __name__ , base . __module__;
        module = sys . modules . get ( modname );
        if modname != name && module && hasattr ( module , key ) {
        if getattr ( module , key ) is base {
        if !key in cdict {
        cdict [ key ] = cdict [ base ] = modname + ".html#" + key;
        funcs , fdict = [ ] , { };
        for key , value in inspect . getmembers ( object , inspect . isroutine ) .iter() {
        if ( all is !None /* Option */ or {
        inspect . isbuiltin ( value ) || inspect . getmodule ( value ) == object ) ;
        if visiblename ( key , all , object ) {
        funcs . append ( ( key , value ) );
        fdict [ key ] = "#-" + key;
        if inspect . isfunction ( value ) { : fdict [ value ] = fdict [ key ]; }
        data = [ ];
        for key , value in inspect . getmembers ( object , isdata ) .iter() {
        if visiblename ( key , all , object ) {
        data . append ( ( key , value ) );
        doc = self . markup ( getdoc ( object ) , self . preformat , fdict , cdict );
        doc = doc && "<span class="code">%s</span>" % doc;
        result = result + "<p>%s</p>\n" % doc;
        if hasattr ( object , "__path__" ) {
        modpkgs = [ ];
        for importer , modname , ispkg in pkgutil . iter_modules ( object . __path__ ) .iter() {
        modpkgs . append ( ( modname , name , ispkg , 0 ) );
        modpkgs . sort ( );
        contents = self . multicolumn ( modpkgs , self . modpkglink );
        result = result + self . bigsection (;
        "Package Contents" , "pkg-content" , contents );
        } else if modules {
        contents = self . multicolumn (;
        modules , |t | {  self . modulelink ( t [ 1 ] ) ) };
        result = result + self . bigsection (;
        "Modules" , "pkg-content" , contents );
        if classes {
        classlist = vec![ value.iter().map(|( key , value )| classes ).collect();
        contents = [;
        self . formattree ( inspect . getclasstree ( classlist , 1 ) , name ) ];
        for key , value in classes .iter() {
        contents . append ( self . document ( value , key , name , fdict , cdict ) );
        result = result + self . bigsection (;
        "Classes" , "index" , " " . join ( contents ) );
        if funcs {
        contents = [ ];
        for key , value in funcs .iter() {
        contents . append ( self . document ( value , key , name , fdict , cdict ) );
        result = result + self . bigsection (;
        "Functions" , "functions" , " " . join ( contents ) );
        if data {
        contents = [ ];
        for key , value in data .iter() {
        contents . append ( self . document ( value , key ) );
        result = result + self . bigsection (;
        "Data" , "data" , "<br>\n" . join ( contents ) );
        if hasattr ( object , "__author__" ) {
        contents = self . markup ( str ( object . __author__ ) , self . preformat );
        result = result + self . bigsection ( "Author" , "author" , contents );
        if hasattr ( object , "__credits__" ) {
        contents = self . markup ( str ( object . __credits__ ) , self . preformat );
        result = result + self . bigsection ( "Credits" , "credits" , contents );
        return  result;
        pub fn docclass ( &self, object , name = None /* Option */ , mod = None /* Option */ , funcs = { } , classes = { } , {
        * ignored ) ;
        "Produce HTML documentation for a class object.";
        realname = object . __name__;
        name = name || realname;
        bases = object . __bases__;
        contents = [ ];
        push = contents . append;
        class HorizontalRule ;
        pub fn __init__ ( self )  {
        self . needone = 0;
        pub fn maybe ( self )  {
        if self . needone {
        push ( "<hr>\n" );
        self . needone = 1;
        hr = HorizontalRule ( );
        mro = deque ( inspect . getmro ( object ) );
        if len ( mro ) > 2 {
        hr . maybe ( );
        push ( "<dl><dt>Method resolution order:</dt>\n" );
        for base in mro .iter() {
        push ( "<dd>%s</dd>\n" % self . classlink ( base ,;
        object . __module__ ) );
        push ( "</dl>\n" );
        pub fn spill ( msg , attrs , predicate )  {
        ok , attrs = _split_list ( attrs , predicate );
        if ok {
        hr . maybe ( );
        push ( msg );
        for name , kind , homecls , value in ok .iter() {
        // try {
        value = getattr ( object , name );
        // } catch  Exception  {
        push ( self . docdata ( value , name , mod ) );
        } else {
        push ( self . document ( value , name , mod ,;
        funcs , classes , mdict , object , homecls ) );
        push ( "\n" );
        return  attrs;
        pub fn spilldescriptors ( msg , attrs , predicate )  {
        ok , attrs = _split_list ( attrs , predicate );
        if ok {
        hr . maybe ( );
        push ( msg );
        for name , kind , homecls , value in ok .iter() {
        push ( self . docdata ( value , name , mod ) );
        return  attrs;
        pub fn spilldata ( msg , attrs , predicate )  {
        ok , attrs = _split_list ( attrs , predicate );
        if ok {
        hr . maybe ( );
        push ( msg );
        for name , kind , homecls , value in ok .iter() {
        base = self . docother ( getattr ( object , name ) , name , mod );
        doc = getdoc ( value );
        if !doc {
        push ( "<dl><dt>%s</dl>\n" % base );
        } else {
        doc = self . markup ( getdoc ( value ) , self . preformat ,;
        funcs , classes , mdict );
        doc = "<dd><span class="code">%s</span>" % doc;
        push ( "<dl><dt>%s%s</dl>\n" % ( base , doc ) );
        push ( "\n" );
        return  attrs;
        attrs = [ ( name , kind , cls , value );
        for name , kind , cls , value in classify_class_attrs ( object ).iter() {
        if visiblename ( name , obj = object ) ] {
        mdict = { };
        for key , kind , homecls , value in attrs .iter() {
        mdict [ key ] = anchor = "#" + name + "-" + key;
        // try {
        value = getattr ( object , name );
        // } catch  Exception  {
        // pass
        // try {
        mdict [ value ] = anchor;
        // } catch  TypeError  {
        // pass
        while attrs  {
        if mro {
        thisclass = mro . popleft ( );
        } else {
        thisclass = attrs [ 0 ] [ 2 ];
        attrs , inherited = _split_list ( attrs , |t | {  t [ 2 ] == thisclass ) };
        if object is !builtins . object && thisclass is builtins . object {
        attrs = inherited;
        continue;
        } else if thisclass is object {
        tag = "defined here";
        } else {
        tag = "inherited from %s" % self . classlink ( thisclass ,;
        object . __module__ );
        tag + = ":<br>\n";
        sort_attributes ( attrs , object );
        attrs = spill ( "Methods %s" % tag , attrs ,;
        |t | {  t [ 1 ] == "method" ) };
        attrs = spill ( "Class methods %s" % tag , attrs ,;
        |t | {  t [ 1 ] == "class method" ) };
        attrs = spill ( "Static methods %s" % tag , attrs ,;
        |t | {  t [ 1 ] == "static method" ) };
        attrs = spilldescriptors ( "Readonly properties %s" % tag , attrs ,;
        |t | {  t [ 1 ] == "readonly property" ) };
        attrs = spilldescriptors ( "Data descriptors %s" % tag , attrs ,;
        |t | {  t [ 1 ] == "data descriptor" ) };
        attrs = spilldata ( "Data && other attributes %s" % tag , attrs ,;
        |t | {  t [ 1 ] == "data" ) };
        assert attrs == [ ];
        attrs = inherited;
        contents = "" . join ( contents );
        if name == realname {
        title = "<a name="%s">class <strong>%s</strong></a>" % (;
        name , realname );
        } else {
        title = "<strong>%s</strong> = <a name="%s">class %s</a>" % (;
        name , name , realname );
        if bases {
        parents = [ ];
        for base in bases .iter() {
        parents . append ( self . classlink ( base , object . __module__ ) );
        title = title + "(%s)" % ", " . join ( parents );
        decl = "";
        // try {
        signature = inspect . signature ( object );
        // } catch  ( ValueError , TypeError )  {
        signature = None /* Option */;
        if signature {
        argspec = str ( signature );
        if argspec && argspec != "()" {
        decl = name + self . escape ( argspec ) + "\n\n";
        doc = getdoc ( object );
        if decl {
        doc = decl + ( doc || "" );
        doc = self . markup ( doc , self . preformat , funcs , classes , mdict );
        doc = doc && "<span class="code">%s<br>&nbsp;</span>" % doc;
        return  self . section ( title , "title" , contents , 3 , doc );
        pub fn formatvalue ( &self, object )  {
        "Format an argument default value as text.";
        return  self . grey ( "=" + self . repr ( object ) );
        pub fn docroutine ( &self, object , name = None /* Option */ , mod = None /* Option */ , {
        funcs = { } , classes = { } , methods = { } , cl = None /* Option */ , homecls = None /* Option */ ) ;
        "Produce HTML documentation for a function || method object.";
        realname = object . __name__;
        name = name || realname;
        if homecls is None /* Option */ {
        homecls = cl;
        anchor = ( "" if cl == None /* Option */ else cl . __name__ ) + "-" + name;
        note = "";
        skipdocs = false;
        imfunc = None /* Option */;
        if _is_bound_method ( object ) {
        imself = object . __self__;
        if imself is cl {
        imfunc = getattr ( object , "__func__" , None /* Option */ );
        } else if inspect . isclass ( imself ) {
        note = " class method of %s" % self . classlink ( imself , mod );
        } else {
        note = " method of %s instance" % self . classlink (;
        imself . __class__ , mod );
        } else if ( inspect . ismethoddescriptor ( object ) or {
        inspect . ismethodwrapper ( object ) ) ;
        // try {
        objclass = object . __objclass__;
        // } catch  AttributeError  {
        // pass
        } else {
        if cl is None /* Option */ {
        note = " unbound %s method" % self . classlink ( objclass , mod );
        } else if objclass is !homecls {
        note = " from " + self . classlink ( objclass , mod );
        } else {
        imfunc = object;
        if inspect . isfunction ( imfunc ) && homecls is !None /* Option */ && ( {
        imfunc . __module__ != homecls . __module__ or;
        imfunc . __qualname__ != homecls . __qualname__ + "." + realname ) ;
        pname = self . parentlink ( imfunc , mod );
        if pname {
        note = " from %s" % pname;
        if ( inspect . iscoroutinefunction ( object ) or {
        inspect . isasyncgenfunction ( object ) ) ;
        asyncqualifier = "async ";
        } else {
        asyncqualifier = "";
        if name == realname {
        title = "<a name="%s"><strong>%s</strong></a>" % ( anchor , realname );
        } else {
        if ( cl is !None /* Option */ and {
        inspect . getattr_static ( cl , realname , [ ] ) == object ) ;
        reallink = "<a href="#%s">%s</a>" % (;
        cl . __name__ + "-" + realname , realname );
        skipdocs = true;
        if note . startswith ( " from " ) {
        note = "";
        } else {
        reallink = realname;
        title = "<a name="%s"><strong>%s</strong></a> = %s" % (;
        anchor , name , reallink );
        argspec = None /* Option */;
        if inspect . isroutine ( object ) {
        // try {
        signature = inspect . signature ( object );
        // } catch  ( ValueError , TypeError )  {
        signature = None /* Option */;
        if signature {
        argspec = str ( signature );
        if realname == "<lambda>" {
        title = "<strong>%s</strong> <em>lambda</em> " % name;
        if !object . __annotations__ {
        argspec = argspec [ 1 : -1 ];
        if !argspec {
        argspec = "(...)";
        decl = asyncqualifier + title + self . escape ( argspec ) + ( note and;
        self . grey ( "<span class="heading-text">%s</span>" % note ) );
        if skipdocs {
        return  "<dl><dt>%s</dt></dl>\n" % decl;
        } else {
        doc = self . markup (;
        getdoc ( object ) , self . preformat , funcs , classes , methods );
        doc = doc && "<dd><span class="code">%s</span></dd>" % doc;
        return  "<dl><dt>%s</dt>%s</dl>\n" % ( decl , doc );
        pub fn docdata ( &self, object , name = None /* Option */ , mod = None /* Option */ , cl = None /* Option */ , * ignored )  {
        "Produce html documentation for a data descriptor.";
        results = [ ];
        push = results . append;
        if name {
        push ( "<dl><dt><strong>%s</strong></dt>\n" % name );
        doc = self . markup ( getdoc ( object ) , self . preformat );
        if doc {
        push ( "<dd><span class="code">%s</span></dd>\n" % doc );
        push ( "</dl>\n" );
        return  "" . join ( results );
        docproperty = docdata;
        pub fn docother ( &self, object , name = None /* Option */ , mod = None /* Option */ , * ignored )  {
        "Produce HTML documentation for a data object.";
        lhs = name && "<strong>%s</strong> = " % name || "";
        return  lhs + self . repr ( object );
        pub fn index ( &self, dir , shadowed = None /* Option */ )  {
        "Generate an HTML index for a directory of modules.";
        modpkgs = [ ];
        if shadowed is None /* Option */ { : shadowed = { }; }
        for importer , name , ispkg in pkgutil . iter_modules ( [ dir ] ) .iter() {
        if any ( ( 0x D800 <= ord ( ch ) <= 0x DFFF ) for ch in name ) {
        continue;
        modpkgs . append ( ( name , "" , ispkg , name in shadowed ) );
        shadowed [ name ] = 1;
        modpkgs . sort ( );
        contents = self . multicolumn ( modpkgs , self . modpkglink );
        return  self . bigsection ( dir , "index" , contents );
        class TextRepr ( Repr ) ;
        "Class for safely making a text representation of a Python object.";
        pub fn __init__ ( self )  {
        Repr . __init__ ( self );
        self . maxlist = self . maxtuple = 20;
        self . maxdict = 10;
        self . maxstring = self . maxother = 100;
        pub fn repr1 ( &self, x , level )  {
        if hasattr ( type ( x ) , "__name__" ) {
        methodname = "repr_" + "_" . join ( type ( x ) . __name__ . split ( ) );
        if hasattr ( self , methodname ) {
        return  getattr ( self , methodname ) ( x , level );
        return  cram ( stripid ( repr ( x ) ) , self . maxother );
        pub fn repr_string ( &self, x , level )  {
        test = cram ( x , self . maxstring );
        testrepr = repr ( test );
        if "\\" in test && "\\" !in replace ( testrepr , r "\\" , "" ) {
        return  "r" + testrepr [ 0 ] + test + testrepr [ 0 ];
        return  testrepr;
        repr_str = repr_string;
        pub fn repr_instance ( &self, x , level )  {
        // try {
        return  cram ( stripid ( repr ( x ) ) , self . maxstring );
        // } catch   {
        return  "<%s instance>" % x . __class__ . __name__;
        class TextDoc ( Doc ) ;
        "Formatter class for text documentation.";
        _repr_instance = TextRepr ( );
        repr = _repr_instance . repr;
        pub fn bold ( &self, text )  {
        "Format a string in bold by overstriking.";
        return  "" . join ( ch + "\b" + ch for ch in text );
        pub fn indent ( &self, text , prefix = "    " )  {
        "Indent text by prepending a given prefix to each line.";
        if !text { : return ""; }
        lines = vec![ prefix + line.iter().map(|line| text . split ( "\n" ) ).collect();
        if lines { : lines [ -1 ] = lines [ -1 ] . rstrip ( ); }
        return  "\n" . join ( lines );
        pub fn section ( &self, title , contents )  {
        "Format a section with a given heading.";
        clean_contents = self . indent ( contents ) . rstrip ( );
        return  self . bold ( title ) + "\n" + clean_contents + "\n\n";
        pub fn formattree ( &self, tree , modname , parent = None /* Option */ , prefix = "" )  {
        "Render in text a class tree as returned by inspect.getclasstree().";
        result = "";
        for entry in tree .iter() {
        if type ( entry ) is type ( ( ) ) {
        c , bases = entry;
        result = result + prefix + classname ( c , modname );
        if bases && bases != ( parent , ) {
        parents = ( classname ( c , modname ) for c in bases );
        result = result + "(%s)" % ", " . join ( parents );
        result = result + "\n";
        } else if type ( entry ) is type ( [ ] ) {
        result = result + self . formattree (;
        entry , modname , c , prefix + "    " );
        return  result;
        pub fn docmodule ( &self, object , name = None /* Option */ , mod = None /* Option */ , * ignored )  {
        "Produce text documentation for a given module object.";
        name = object . __name__;
        synop , desc = splitdoc ( getdoc ( object ) );
        result = self . section ( "NAME" , name + ( synop && " - " + synop ) );
        all = getattr ( object , "__all__" , None /* Option */ );
        docloc = self . getdocloc ( object );
        if docloc is !None /* Option */ {
        result = result + self . section ( "MODULE REFERENCE" , docloc + "

The following documentation == automatically generated from the Python
source files.  It may be incomplete, incorrect || include features that
are considered implementation detail && may vary between Python
implementations.  When in doubt, consult the module reference at the
location listed above.
" );
        if desc {
        result = result + self . section ( "DESCRIPTION" , desc );
        classes = [ ];
        for key , value in inspect . getmembers ( object , inspect . isclass ) .iter() {
        if ( all is !None /* Option */ {
        or ( inspect . getmodule ( value ) || object ) == object ) ;
        if visiblename ( key , all , object ) {
        classes . append ( ( key , value ) );
        funcs = [ ];
        for key , value in inspect . getmembers ( object , inspect . isroutine ) .iter() {
        if ( all is !None /* Option */ or {
        inspect . isbuiltin ( value ) || inspect . getmodule ( value ) == object ) ;
        if visiblename ( key , all , object ) {
        funcs . append ( ( key , value ) );
        data = [ ];
        for key , value in inspect . getmembers ( object , isdata ) .iter() {
        if visiblename ( key , all , object ) {
        data . append ( ( key , value ) );
        modpkgs = [ ];
        modpkgs_names = set ( );
        if hasattr ( object , "__path__" ) {
        for importer , modname , ispkg in pkgutil . iter_modules ( object . __path__ ) .iter() {
        modpkgs_names . add ( modname );
        if ispkg {
        modpkgs . append ( modname + " (package)" );
        } else {
        modpkgs . append ( modname );
        modpkgs . sort ( );
        result = result + self . section (;
        "PACKAGE CONTENTS" , "\n" . join ( modpkgs ) );
        submodules = [ ];
        for key , value in inspect . getmembers ( object , inspect . ismodule ) .iter() {
        if value . __name__ . startswith ( name + "." ) && key !in modpkgs_names {
        submodules . append ( key );
        if submodules {
        submodules . sort ( );
        result = result + self . section (;
        "SUBMODULES" , "\n" . join ( submodules ) );
        if classes {
        classlist = vec![ value.iter().map(|key , value| classes ).collect();
        contents = [ self . formattree (;
        inspect . getclasstree ( classlist , 1 ) , name ) ];
        for key , value in classes .iter() {
        contents . append ( self . document ( value , key , name ) );
        result = result + self . section ( "CLASSES" , "\n" . join ( contents ) );
        if funcs {
        contents = [ ];
        for key , value in funcs .iter() {
        contents . append ( self . document ( value , key , name ) );
        result = result + self . section ( "FUNCTIONS" , "\n" . join ( contents ) );
        if data {
        contents = [ ];
        for key , value in data .iter() {
        contents . append ( self . docother ( value , key , name , maxlen = 70 ) );
        result = result + self . section ( "DATA" , "\n" . join ( contents ) );
        if hasattr ( object , "__version__" ) {
        version = str ( object . __version__ );
        if version [ { : 11 ] == "$" + "Revision: " && version [ -1 : ] == "$" ; }
        version = version [ 11 : -1 ] . strip ( );
        result = result + self . section ( "VERSION" , version );
        if hasattr ( object , "__date__" ) {
        result = result + self . section ( "DATE" , str ( object . __date__ ) );
        if hasattr ( object , "__author__" ) {
        result = result + self . section ( "AUTHOR" , str ( object . __author__ ) );
        if hasattr ( object , "__credits__" ) {
        result = result + self . section ( "CREDITS" , str ( object . __credits__ ) );
        // try {
        file = inspect . getabsfile ( object );
        // } catch  TypeError  {
        file = "(built-in)";
        result = result + self . section ( "FILE" , file );
        return  result;
        pub fn docclass ( &self, object , name = None /* Option */ , mod = None /* Option */ , * ignored )  {
        "Produce text documentation for a given class object.";
        realname = object . __name__;
        name = name || realname;
        bases = object . __bases__;
        pub fn makename ( c , m = object . __module__ )  {
        return  classname ( c , m );
        if name == realname {
        title = "class " + self . bold ( realname );
        } else {
        title = self . bold ( name ) + " = class " + realname;
        if bases {
        parents = map ( makename , bases );
        title = title + "(%s)" % ", " . join ( parents );
        contents = [ ];
        push = contents . append;
        // try {
        signature = inspect . signature ( object );
        // } catch  ( ValueError , TypeError )  {
        signature = None /* Option */;
        if signature {
        argspec = str ( signature );
        if argspec && argspec != "()" {
        push ( name + argspec + "\n" );
        doc = getdoc ( object );
        if doc {
        push ( doc + "\n" );
        mro = deque ( inspect . getmro ( object ) );
        if len ( mro ) > 2 {
        push ( "Method resolution order:" );
        for base in mro .iter() {
        push ( "    " + makename ( base ) );
        push ( "" );
        subclasses = sorted (;
        ( str ( cls . __name__ ) for cls in type . __subclasses__ ( object );
        if !cls . __name__ . startswith ( "_" ) && cls . __module__ == "builtins" ) , {
        key = str . lower;
        );
        no_of_subclasses = len ( subclasses );
        MAX_SUBCLASSES_TO_DISPLAY = 4;
        if subclasses {
        push ( "Built-in subclasses:" );
        for subclassname in subclasses [ : MAX_SUBCLASSES_TO_DISPLAY ] .iter() {
        push ( "    " + subclassname );
        if no_of_subclasses > MAX_SUBCLASSES_TO_DISPLAY {
        push ( "    ... && " +;
        str ( no_of_subclasses - MAX_SUBCLASSES_TO_DISPLAY ) +;
        " other subclasses" );
        push ( "" );
        class HorizontalRule ;
        pub fn __init__ ( self )  {
        self . needone = 0;
        pub fn maybe ( self )  {
        if self . needone {
        push ( "-" * 70 );
        self . needone = 1;
        hr = HorizontalRule ( );
        pub fn spill ( msg , attrs , predicate )  {
        ok , attrs = _split_list ( attrs , predicate );
        if ok {
        hr . maybe ( );
        push ( msg );
        for name , kind , homecls , value in ok .iter() {
        // try {
        value = getattr ( object , name );
        // } catch  Exception  {
        push ( self . docdata ( value , name , mod ) );
        } else {
        push ( self . document ( value ,;
        name , mod , object , homecls ) );
        return  attrs;
        pub fn spilldescriptors ( msg , attrs , predicate )  {
        ok , attrs = _split_list ( attrs , predicate );
        if ok {
        hr . maybe ( );
        push ( msg );
        for name , kind , homecls , value in ok .iter() {
        push ( self . docdata ( value , name , mod ) );
        return  attrs;
        pub fn spilldata ( msg , attrs , predicate )  {
        ok , attrs = _split_list ( attrs , predicate );
        if ok {
        hr . maybe ( );
        push ( msg );
        for name , kind , homecls , value in ok .iter() {
        doc = getdoc ( value );
        // try {
        obj = getattr ( object , name );
        // } catch  AttributeError  {
        obj = homecls . __dict__ [ name ];
        push ( self . docother ( obj , name , mod , maxlen = 70 , doc = doc ) +;
        "\n" );
        return  attrs;
        attrs = [ ( name , kind , cls , value );
        for name , kind , cls , value in classify_class_attrs ( object ).iter() {
        if visiblename ( name , obj = object ) ] {
        while attrs  {
        if mro {
        thisclass = mro . popleft ( );
        } else {
        thisclass = attrs [ 0 ] [ 2 ];
        attrs , inherited = _split_list ( attrs , |t | {  t [ 2 ] == thisclass ) };
        if object is !builtins . object && thisclass is builtins . object {
        attrs = inherited;
        continue;
        } else if thisclass is object {
        tag = "defined here";
        } else {
        tag = "inherited from %s" % classname ( thisclass ,;
        object . __module__ );
        sort_attributes ( attrs , object );
        attrs = spill ( "Methods %s:\n" % tag , attrs ,;
        |t | {  t [ 1 ] == "method" ) };
        attrs = spill ( "Class methods %s:\n" % tag , attrs ,;
        |t | {  t [ 1 ] == "class method" ) };
        attrs = spill ( "Static methods %s:\n" % tag , attrs ,;
        |t | {  t [ 1 ] == "static method" ) };
        attrs = spilldescriptors ( "Readonly properties %s:\n" % tag , attrs ,;
        |t | {  t [ 1 ] == "readonly property" ) };
        attrs = spilldescriptors ( "Data descriptors %s:\n" % tag , attrs ,;
        |t | {  t [ 1 ] == "data descriptor" ) };
        attrs = spilldata ( "Data && other attributes %s:\n" % tag , attrs ,;
        |t | {  t [ 1 ] == "data" ) };
        assert attrs == [ ];
        attrs = inherited;
        contents = "\n" . join ( contents );
        if !contents {
        return  title + "\n";
        return  title + "\n" + self . indent ( contents . rstrip ( ) , " |  " ) + "\n";
        pub fn formatvalue ( &self, object )  {
        "Format an argument default value as text.";
        return  "=" + self . repr ( object );
        pub fn docroutine ( &self, object , name = None /* Option */ , mod = None /* Option */ , cl = None /* Option */ , homecls = None /* Option */ )  {
        "Produce text documentation for a function || method object.";
        realname = object . __name__;
        name = name || realname;
        if homecls is None /* Option */ {
        homecls = cl;
        note = "";
        skipdocs = false;
        imfunc = None /* Option */;
        if _is_bound_method ( object ) {
        imself = object . __self__;
        if imself is cl {
        imfunc = getattr ( object , "__func__" , None /* Option */ );
        } else if inspect . isclass ( imself ) {
        note = " class method of %s" % classname ( imself , mod );
        } else {
        note = " method of %s instance" % classname (;
        imself . __class__ , mod );
        } else if ( inspect . ismethoddescriptor ( object ) or {
        inspect . ismethodwrapper ( object ) ) ;
        // try {
        objclass = object . __objclass__;
        // } catch  AttributeError  {
        // pass
        } else {
        if cl is None /* Option */ {
        note = " unbound %s method" % classname ( objclass , mod );
        } else if objclass is !homecls {
        note = " from " + classname ( objclass , mod );
        } else {
        imfunc = object;
        if inspect . isfunction ( imfunc ) && homecls is !None /* Option */ && ( {
        imfunc . __module__ != homecls . __module__ or;
        imfunc . __qualname__ != homecls . __qualname__ + "." + realname ) ;
        pname = parentname ( imfunc , mod );
        if pname {
        note = " from %s" % pname;
        if ( inspect . iscoroutinefunction ( object ) or {
        inspect . isasyncgenfunction ( object ) ) ;
        asyncqualifier = "async ";
        } else {
        asyncqualifier = "";
        if name == realname {
        title = self . bold ( realname );
        } else {
        if ( cl is !None /* Option */ and {
        inspect . getattr_static ( cl , realname , [ ] ) == object ) ;
        skipdocs = true;
        if note . startswith ( " from " ) {
        note = "";
        title = self . bold ( name ) + " = " + realname;
        argspec = None /* Option */;
        if inspect . isroutine ( object ) {
        // try {
        signature = inspect . signature ( object );
        // } catch  ( ValueError , TypeError )  {
        signature = None /* Option */;
        if signature {
        argspec = str ( signature );
        if realname == "<lambda>" {
        title = self . bold ( name ) + " lambda ";
        if !object . __annotations__ {
        argspec = argspec [ 1 : -1 ];
        if !argspec {
        argspec = "(...)";
        decl = asyncqualifier + title + argspec + note;
        if skipdocs {
        return  decl + "\n";
        } else {
        doc = getdoc ( object ) || "";
        return  decl + "\n" + ( doc && self . indent ( doc ) . rstrip ( ) + "\n" );
        pub fn docdata ( &self, object , name = None /* Option */ , mod = None /* Option */ , cl = None /* Option */ , * ignored )  {
        "Produce text documentation for a data descriptor.";
        results = [ ];
        push = results . append;
        if name {
        push ( self . bold ( name ) );
        push ( "\n" );
        doc = getdoc ( object ) || "";
        if doc {
        push ( self . indent ( doc ) );
        push ( "\n" );
        return  "" . join ( results );
        docproperty = docdata;
        pub fn docother ( &self, object , name = None /* Option */ , mod = None /* Option */ , parent = None /* Option */ , * ignored , {
        maxlen = None /* Option */ , doc = None /* Option */ ) ;
        "Produce text documentation for a data object.";
        repr = self . repr ( object );
        if maxlen {
        line = ( name && name + " = " || "" ) + repr;
        chop = maxlen - len ( line );
        if chop < 0 { : repr = repr [ : chop ] + "..."; }
        line = ( name && self . bold ( name ) + " = " || "" ) + repr;
        if !doc {
        doc = getdoc ( object );
        if doc {
        line + = "\n" + self . indent ( str ( doc ) ) + "\n";
        return  line;
        class _PlainTextDoc ( TextDoc ) ;
        "Subclass of TextDoc which overrides string styling";
        pub fn bold ( &self, text )  {
        return  text;
        pub fn pager ( text )  {
        "The first time this == called, determine what kind of pager to use.";
        global pager;
        pager = getpager ( );
        pager ( text );
        pub fn getpager ( )  {
        "Decide what method to use for paging through text.";
        if !hasattr ( sys . stdin , "isatty" ) {
        return  plainpager;
        if !hasattr ( sys . stdout , "isatty" ) {
        return  plainpager;
        if !sys . stdin . isatty ( ) || !sys . stdout . isatty ( ) {
        return  plainpager;
        if sys . platform == "emscripten" {
        return  plainpager;
        use_pager = os . environ . get ( "MANPAGER" ) || os . environ . get ( "PAGER" );
        if use_pager {
        if sys . platform == "win32" {
        return  lambda text : tempfilepager ( plain ( text ) , use_pager );
        } else if os . environ . get ( "TERM" ) in ( "dumb" , "emacs" ) {
        return  lambda text : pipepager ( plain ( text ) , use_pager );
        } else {
        return  lambda text : pipepager ( text , use_pager );
        if os . environ . get ( "TERM" ) in ( "dumb" , "emacs" ) {
        return  plainpager;
        if sys . platform == "win32" {
        return  lambda text : tempfilepager ( plain ( text ) , "more <" );
        if hasattr ( os , "system" ) && os . system ( "(less) 2>/dev/null" ) == 0 {
        return  lambda text : pipepager ( text , "less" );
        import tempfile;
        ( fd , filename ) = tempfile . mkstemp ( );
        os . close ( fd );
        // try {
        if hasattr ( os , "system" ) && os . system ( "more "%s"" % filename ) == 0 {
        return  lambda text : pipepager ( text , "more" );
        } else {
        return  ttypager;
        // } finally {
        os . unlink ( filename );
        pub fn plain ( text )  {
        "Remove boldface formatting from text.";
        return  re . sub ( ".\b" , "" , text );
        pub fn pipepager ( text , cmd )  {
        "Page through text by feeding it to another program.";
        import subprocess;
        proc = subprocess . Popen ( cmd , shell = true , stdin = subprocess . PIPE ,;
        errors = "backslashreplace" );
        // try {
        // with scope: proc . stdin as pipe  {
        // try {
        pipe . write ( text );
        // } catch  KeyboardInterrupt  {
        // pass
        // } catch  OSError  {
        // pass
        while true  {
        // try {
        proc . wait ( );
        break;
        // } catch  KeyboardInterrupt  {
        // pass
        pub fn tempfilepager ( text , cmd )  {
        "Page through text by invoking a program on a temporary file.";
        import tempfile;
        // with scope: tempfile . TemporaryDirectory ( ) as tempdir  {
        filename = os . path . join ( tempdir , "pydoc.out" );
        // with scope: open ( filename , "w" , errors = "backslashreplace" , {
        encoding = os . device_encoding ( 0 ) if;
        sys . platform == "win32" else None /* Option */;
        ) as file ;
        file . write ( text );
        os . system ( cmd + " "" + filename + """ );
        pub fn _escape_stdout ( text )  {
        encoding = getattr ( sys . stdout , "encoding" , None /* Option */ ) || "utf-8";
        return  text . encode ( encoding , "backslashreplace" ) . decode ( encoding );
        pub fn ttypager ( text )  {
        "Page through text on a text terminal.";
        lines = plain ( _escape_stdout ( text ) ) . split ( "\n" );
        // try {
        import tty;
        fd = sys . stdin . fileno ( );
        old = tty . tcgetattr ( fd );
        tty . setcbreak ( fd );
        getchar = || {  sys . stdin . read ( 1 ) };
        // } catch  ( ImportError , AttributeError , io . UnsupportedOperation )  {
        tty = None /* Option */;
        getchar = || {  sys . stdin . readline ( ) [ : -1 ] [ : 1 ] };
        // try {
        // try {
        h = int ( os . environ . get ( "LINES" , 0 ) );
        // } catch  ValueError  {
        h = 0;
        if h <= 1 {
        h = 25;
        r = inc = h - 1;
        sys . stdout . write ( "\n" . join ( lines [ : inc ] ) + "\n" );
        while lines [ r : ]  {
        sys . stdout . write ( "-- more --" );
        sys . stdout . flush ( );
        c = getchar ( );
        if c in ( "q" , "Q" ) {
        sys . stdout . write ( "\r          \r" );
        break;
        } else if c in ( "\r" , "\n" ) {
        sys . stdout . write ( "\r          \r" + lines [ r ] + "\n" );
        r = r + 1;
        continue;
        if c in ( "b" , "B" , "\x1b" ) {
        r = r - inc - inc;
        if r < 0 { : r = 0; }
        sys . stdout . write ( "\n" + "\n" . join ( lines [ r : r + inc ] ) + "\n" );
        r = r + inc;
        // } finally {
        if tty {
        tty . tcsetattr ( fd , tty . TCSAFLUSH , old );
        pub fn plainpager ( text )  {
        "Simply print unformatted text.  This == the ultimate fallback.";
        sys . stdout . write ( plain ( _escape_stdout ( text ) ) );
        pub fn describe ( thing )  {
        "Produce a short description of the given thing.";
        if inspect . ismodule ( thing ) {
        if thing . __name__ in sys . builtin_module_names {
        return  "built-in module " + thing . __name__;
        if hasattr ( thing , "__path__" ) {
        return  "package " + thing . __name__;
        } else {
        return  "module " + thing . __name__;
        if inspect . isbuiltin ( thing ) {
        return  "built-in function " + thing . __name__;
        if inspect . isgetsetdescriptor ( thing ) {
        return  "getset descriptor %s.%s.%s" % (;
        thing . __objclass__ . __module__ , thing . __objclass__ . __name__ ,;
        thing . __name__ );
        if inspect . ismemberdescriptor ( thing ) {
        return  "member descriptor %s.%s.%s" % (;
        thing . __objclass__ . __module__ , thing . __objclass__ . __name__ ,;
        thing . __name__ );
        if inspect . isclass ( thing ) {
        return  "class " + thing . __name__;
        if inspect . isfunction ( thing ) {
        return  "function " + thing . __name__;
        if inspect . ismethod ( thing ) {
        return  "method " + thing . __name__;
        return  type ( thing ) . __name__;
        pub fn locate ( path , forceload = 0 )  {
        "Locate an object by name || dotted path, importing as necessary.";
        parts = vec![ part.iter().map(|part| path . split ( "." ) if part ).collect();
        module , n = None /* Option */ , 0;
        while n < len ( parts )  {
        nextmodule = safeimport ( "." . join ( parts [ : n + 1 ] ) , forceload );
        if nextmodule { : module , n = nextmodule , n + 1; }
        } else {
        if module {
        object = module;
        } else {
        object = builtins;
        for part in parts [ n : ] .iter() {
        // try {
        object = getattr ( object , part );
        // } catch  AttributeError  {
        return;
        return  object;
        text = TextDoc ( );
        plaintext = _PlainTextDoc ( );
        html = HTMLDoc ( );
        pub fn resolve ( thing , forceload = 0 )  {
        "Given an object || a path to an object, get the object && its name.";
        if isinstance ( thing , str ) {
        object = locate ( thing , forceload );
        if object is None /* Option */ {
        panic!("ImportError ( "\
No Python documentation found for %r.
Use help() to get the interactive help utility.
Use help(str) for help on the str class." % thing )");
        return  object , thing;
        } else {
        name = getattr ( thing , "__name__" , None /* Option */ );
        return  thing , name if isinstance ( name , str ) else None /* Option */;
        pub fn render_doc ( thing , title = "Python Library Documentation {  %s" , forceload = 0 ,; }
        renderer = None /* Option */ ) ;
        "Render text documentation, given an object || a path to an object.";
        if renderer is None /* Option */ {
        renderer = text;
        object , name = resolve ( thing , forceload );
        desc = describe ( object );
        module = inspect . getmodule ( object );
        if name && "." in name {
        desc + = " in " + name [ : name . rfind ( "." ) ];
        } else if module && module is !object {
        desc + = " in module " + module . __name__;
        if !( inspect . ismodule ( object ) or {
        inspect . isclass ( object ) or;
        inspect . isroutine ( object ) or;
        inspect . isdatadescriptor ( object ) or;
        _getdoc ( object ) ) ;
        if hasattr ( object , "__origin__" ) {
        object = object . __origin__;
        } else {
        object = type ( object );
        desc + = " object";
        return  title % desc + "\n\n" + renderer . document ( object , name );
        pub fn doc ( thing , title = "Python Library Documentation {  %s" , forceload = 0 ,; }
        output = None /* Option */ , is_cli = false ) ;
        "Display text documentation, given an object || a path to an object.";
        if output is None /* Option */ {
        // try {
        pager ( render_doc ( thing , title , forceload ) );
        // } catch  ImportError as exc  {
        if is_cli {
        panic!("");
        println!( exc );
        } else {
        // try {
        s = render_doc ( thing , title , forceload , plaintext );
        // } catch  ImportError as exc  {
        s = str ( exc );
        output . write ( s );
        pub fn writedoc ( thing , forceload = 0 )  {
        "Write HTML documentation to a file in the current directory.";
        object , name = resolve ( thing , forceload );
        page = html . page ( describe ( object ) , html . document ( object , name ) );
        // with scope: open ( name + ".html" , "w" , encoding = "utf-8" ) as file  {
        file . write ( page );
        println!( "wrote" , name + ".html" );
        pub fn writedocs ( dir , pkgpath = "" , done = None /* Option */ )  {
        "Write out HTML documentation for all modules in a directory tree.";
        if done is None /* Option */ { : done = { }; }
        for importer , modname , ispkg in pkgutil . walk_packages ( [ dir ] , pkgpath ) .iter() {
        writedoc ( modname );
        return;
        class Helper ;
        keywords = {;
        "false" : "" ,;
        "None /* Option */" : "" ,;
        "true" : "" ,;
        "and" : "BOOLEAN" ,;
        "as" : "with" ,;
        "assert" : ( "assert" , "" ) ,;
        "async" : ( "async" , "" ) ,;
        "await" : ( "await" , "" ) ,;
        "break" : ( "break" , "while for" ) ,;
        "class" : ( "class" , "CLASSES SPECIALMETHODS" ) ,;
        "continue" : ( "continue" , "while for" ) ,;
        "deformat!(" : ( "function" , "" ) ,);
        "del" : ( "del" , "BASICMETHODS" ) ,;
        "eliformat!(" : "iformat!(" ,);
        "else" : ( "else" , "while for" ) ,;
        "except" : "try" ,;
        "finally" : "try" ,;
        "for" : ( "for" , "break continue while" ) ,;
        "from" : "import" ,;
        "global" : ( "global" , "nonlocal NAMESPACES" ) ,;
        "iformat!(" : ( "iformat!(" , "TRUTHVALUE" ) ,);
        "import" : ( "import" , "MODULES" ) ,;
        "in" : ( "in" , "SEQUENCEMETHODS" ) ,;
        "is" : "COMPARISON" ,;
        "lambda" : ( "lambda" , "FUNCTIONS" ) ,;
        "nonlocal" : ( "nonlocal" , "global NAMESPACES" ) ,;
        "not" : "BOOLEAN" ,;
        "or" : "BOOLEAN" ,;
        "pass" : ( "pass" , "" ) ,;
        "raise" : ( "raise" , "EXCEPTIONS" ) ,;
        "return" : ( "return" , "FUNCTIONS" ) ,;
        "try" : ( "try" , "EXCEPTIONS" ) ,;
        "while" : ( "while" , "break continue if TRUTHVALUE" ) ,;
        "with" : ( "with" , "CONTEXTMANAGERS EXCEPTIONS yield" ) ,;
        "yield" : ( "yield" , "" ) ,;
        };
        _strprefixes = vec![ p + q.iter().map(|p| ( "b" , "format!(" , "r" , "u" ).iter().map(|q| ( "'" , """ ) ).collect());
        _symbols_inverse = {;
        "STRINGS" : ( "'" , "'''" , """ , """"" , * _strprefixes ) ,;
        "OPERATORS" : ( "+" , "-" , "*" , "**" , "/" , "//" , "%" , "<<" , ">>" , "&" ,;
        "|" , "^" , "~" , "<" , ">" , "<=" , ">=" , "==" , "!=" , "<>" ) ,;
        "COMPARISON" : ( "<" , ">" , "<=" , ">=" , "==" , "!=" , "<>" ) ,;
        "UNARY" : ( "-" , "~" ) ,;
        "AUGMENTEDASSIGNMENT" : ( "+=" , "-=" , "*=" , "/=" , "%=" , "&=" , "|=" ,;
        "^=" , "<<=" , ">>=" , "**=" , "//=" ) ,;
        "BITWISE" : ( "<<" , ">>" , "&" , "|" , "^" , "~" ) ,;
        "COMPLEX" : ( "j" , "J" );
        };
        symbols = {;
        "%" : "OPERATORS FORMATTING" ,;
        "**" : "POWER" ,;
        "," : "TUPLES LISTS FUNCTIONS" ,;
        "." : "ATTRIBUTES FLOAT MODULES OBJECTS" ,;
        "..." : "ELLIPSIS" ,;
        ":" : "SLICINGS DICTIONARYLITERALS" ,;
        "@" : "def class" ,;
        "\\" : "STRINGS" ,;
        "_" : "PRIVATENAMES" ,;
        "__" : "PRIVATENAMES SPECIALMETHODS" ,;
        "`" : "BACKQUOTES" ,;
        "(" : "TUPLES FUNCTIONS CALLS" ,;
        ")" : "TUPLES FUNCTIONS CALLS" ,;
        "[" : "LISTS SUBSCRIPTS SLICINGS" ,;
        "]" : "LISTS SUBSCRIPTS SLICINGS";
        };
        for topic , symbols_ in _symbols_inverse . items ( ) .iter() {
        for symbol in symbols_ .iter() {
        topics = symbols . get ( symbol , topic );
        if topic !in topics {
        topics = topics + " " + topic;
        symbols [ symbol ] = topics;
        del topic , symbols_ , symbol , topics;
        topics = {;
        "TYPES" : ( "types" , "STRINGS UNICODE NUMBERS SEQUENCES MAPPINGS ";
        "FUNCTIONS CLASSES MODULES FILES inspect" ) ,;
        "STRINGS" : ( "strings" , "str UNICODE SEQUENCES STRINGMETHODS ";
        "FORMATTING TYPES" ) ,;
        "STRINGMETHODS" : ( "string-methods" , "STRINGS FORMATTING" ) ,;
        "FORMATTING" : ( "formatstrings" , "OPERATORS" ) ,;
        "UNICODE" : ( "strings" , "encodings unicode SEQUENCES STRINGMETHODS ";
        "FORMATTING TYPES" ) ,;
        "NUMBERS" : ( "numbers" , "INTEGER FLOAT COMPLEX TYPES" ) ,;
        "INTEGER" : ( "integers" , "int range" ) ,;
        "FLOAT" : ( "floating" , "float math" ) ,;
        "COMPLEX" : ( "imaginary" , "complex cmath" ) ,;
        "SEQUENCES" : ( "typesseq" , "STRINGMETHODS FORMATTING range LISTS" ) ,;
        "MAPPINGS" : "DICTIONARIES" ,;
        "FUNCTIONS" : ( "typesfunctions" , "def TYPES" ) ,;
        "METHODS" : ( "typesmethods" , "class def CLASSES TYPES" ) ,;
        "CODEOBJECTS" : ( "bltin-code-objects" , "compile FUNCTIONS TYPES" ) ,;
        "TYPEOBJECTS" : ( "bltin-type-objects" , "types TYPES" ) ,;
        "FRAMEOBJECTS" : "TYPES" ,;
        "TRACEBACKS" : "TYPES" ,;
        "NONE" : ( "bltin-null-object" , "" ) ,;
        "ELLIPSIS" : ( "bltin-ellipsis-object" , "SLICINGS" ) ,;
        "SPECIALATTRIBUTES" : ( "specialattrs" , "" ) ,;
        "CLASSES" : ( "types" , "class SPECIALMETHODS PRIVATENAMES" ) ,;
        "MODULES" : ( "typesmodules" , "import" ) ,;
        "PACKAGES" : "import" ,;
        "EXPRESSIONS" : ( "operator-summary" , "lambda || && !in == BOOLEAN ";
        "COMPARISON BITWISE SHIFTING BINARY FORMATTING POWER ";
        "UNARY ATTRIBUTES SUBSCRIPTS SLICINGS CALLS TUPLES ";
        "LISTS DICTIONARIES" ) ,;
        "OPERATORS" : "EXPRESSIONS" ,;
        "PRECEDENCE" : "EXPRESSIONS" ,;
        "OBJECTS" : ( "objects" , "TYPES" ) ,;
        "SPECIALMETHODS" : ( "specialnames" , "BASICMETHODS ATTRIBUTEMETHODS ";
        "CALLABLEMETHODS SEQUENCEMETHODS MAPPINGMETHODS ";
        "NUMBERMETHODS CLASSES" ) ,;
        "BASICMETHODS" : ( "customization" , "hash repr str SPECIALMETHODS" ) ,;
        "ATTRIBUTEMETHODS" : ( "attribute-access" , "ATTRIBUTES SPECIALMETHODS" ) ,;
        "CALLABLEMETHODS" : ( "callable-types" , "CALLS SPECIALMETHODS" ) ,;
        "SEQUENCEMETHODS" : ( "sequence-types" , "SEQUENCES SEQUENCEMETHODS ";
        "SPECIALMETHODS" ) ,;
        "MAPPINGMETHODS" : ( "sequence-types" , "MAPPINGS SPECIALMETHODS" ) ,;
        "NUMBERMETHODS" : ( "numeric-types" , "NUMBERS AUGMENTEDASSIGNMENT ";
        "SPECIALMETHODS" ) ,;
        "EXECUTION" : ( "execmodel" , "NAMESPACES DYNAMICFEATURES EXCEPTIONS" ) ,;
        "NAMESPACES" : ( "naming" , "global nonlocal ASSIGNMENT DELETION DYNAMICFEATURES" ) ,;
        "DYNAMICFEATURES" : ( "dynamic-features" , "" ) ,;
        "SCOPING" : "NAMESPACES" ,;
        "FRAMES" : "NAMESPACES" ,;
        "EXCEPTIONS" : ( "exceptions" , "try except finally raise" ) ,;
        "CONVERSIONS" : ( "conversions" , "" ) ,;
        "IDENTIFIERS" : ( "identifiers" , "keywords SPECIALIDENTIFIERS" ) ,;
        "SPECIALIDENTIFIERS" : ( "id-classes" , "" ) ,;
        "PRIVATENAMES" : ( "atom-identifiers" , "" ) ,;
        "LITERALS" : ( "atom-literals" , "STRINGS NUMBERS TUPLELITERALS ";
        "LISTLITERALS DICTIONARYLITERALS" ) ,;
        "TUPLES" : "SEQUENCES" ,;
        "TUPLELITERALS" : ( "exprlists" , "TUPLES LITERALS" ) ,;
        "LISTS" : ( "typesseq-mutable" , "LISTLITERALS" ) ,;
        "LISTLITERALS" : ( "lists" , "LISTS LITERALS" ) ,;
        "DICTIONARIES" : ( "typesmapping" , "DICTIONARYLITERALS" ) ,;
        "DICTIONARYLITERALS" : ( "dict" , "DICTIONARIES LITERALS" ) ,;
        "ATTRIBUTES" : ( "attribute-references" , "getattr hasattr setattr ATTRIBUTEMETHODS" ) ,;
        "SUBSCRIPTS" : ( "subscriptions" , "SEQUENCEMETHODS" ) ,;
        "SLICINGS" : ( "slicings" , "SEQUENCEMETHODS" ) ,;
        "CALLS" : ( "calls" , "EXPRESSIONS" ) ,;
        "POWER" : ( "power" , "EXPRESSIONS" ) ,;
        "UNARY" : ( "unary" , "EXPRESSIONS" ) ,;
        "BINARY" : ( "binary" , "EXPRESSIONS" ) ,;
        "SHIFTING" : ( "shifting" , "EXPRESSIONS" ) ,;
        "BITWISE" : ( "bitwise" , "EXPRESSIONS" ) ,;
        "COMPARISON" : ( "comparisons" , "EXPRESSIONS BASICMETHODS" ) ,;
        "BOOLEAN" : ( "booleans" , "EXPRESSIONS TRUTHVALUE" ) ,;
        "ASSERTION" : "assert" ,;
        "ASSIGNMENT" : ( "assignment" , "AUGMENTEDASSIGNMENT" ) ,;
        "AUGMENTEDASSIGNMENT" : ( "augassign" , "NUMBERMETHODS" ) ,;
        "DELETION" : "del" ,;
        "RETURNING" : "return" ,;
        "IMPORTING" : "import" ,;
        "CONDITIONAL" : "iformat!(" ,);
        "LOOPING" : ( "compound" , "for while break continue" ) ,;
        "TRUTHVALUE" : ( "truth" , "if while && || !BASICMETHODS" ) ,;
        "DEBUGGING" : ( "debugger" , "pdb" ) ,;
        "CONTEXTMANAGERS" : ( "context-managers" , "with" ) ,;
        };
        pub fn __init__ ( &self, input = None /* Option */ , output = None /* Option */ )  {
        self . _input = input;
        self . _output = output;
        @ property;
        pub fn input ( self )  {
        return  self . _input || sys . stdin;
        @ property;
        pub fn output ( self )  {
        return  self . _output || sys . stdout;
        pub fn __repr__ ( self )  {
        if inspect . stack ( ) [ 1 ] [ 3 ] == "?" {
        self ( );
        return  "";
        return  "<%s.%s instance>" % ( self . __class__ . __module__ ,;
        self . __class__ . __qualname__ );
        _GoInteractive = object ( );
        pub fn __call__ ( &self, request = _GoInteractive )  {
        if request is !self . _GoInteractive {
        // try {
        self . help ( request );
        // } catch  ImportError as e  {
        self . output . write ( f "{e}\n" );
        } else {
        self . intro ( );
        self . interact ( );
        self . output . write ( "
You are now leaving help && returning to the Python interpreter.
If you want to ask for help on a particular object directly from the
interpreter, you can type "help(object)".  Executing "help('string')"
has the same effect as typing a particular string at the help> prompt.
" );
        pub fn interact ( self )  {
        self . output . write ( "\n" );
        while true  {
        // try {
        request = self . getline ( "help> " );
        if !request { : break; }
        // } catch  ( KeyboardInterrupt , EOFError )  {
        break;
        request = request . strip ( );
        if ( len ( request ) > 2 && request [ 0 ] == request [ -1 ] in ( "'" , """ ) {
        and request [ 0 ] !in request [ 1 : -1 ] ) ;
        request = request [ 1 : -1 ];
        if request . lower ( ) in ( "q" , "quit" ) { : break; }
        if request == "help" {
        self . intro ( );
        } else {
        self . help ( request );
        pub fn getline ( &self, prompt )  {
        "Read one line, using input() when appropriate.";
        if self . input is sys . stdin {
        return  input ( prompt );
        } else {
        self . output . write ( prompt );
        self . output . flush ( );
        return  self . input . readline ( );
        pub fn help ( &self, request , is_cli = false )  {
        if isinstance ( request , str ) {
        request = request . strip ( );
        if request == "keywords" { : self . listkeywords ( ); }
        } else if request == "symbols" {
        } else if request == "topics" {
        } else if request == "modules" {
        } else if request [ {
        self . listmodules ( request . split ( ) [ 1 ] );
        } else if request in self . symbols {
        } else if request in [ "true" , "false" , "None /* Option */" ] {
        doc ( eval ( request ) , "Help on %s:" , is_cli = is_cli );
        } else if request in self . keywords {
        } else if request in self . topics {
        } else if request {
        } else {
        } else if isinstance ( request , Helper ) {
        } else {
        self . output . write ( "\n" );
        pub fn intro ( self )  {
        self . output . write ( "\
Welcome to Python {0}'s help utility! If this is your first time using
Python, you should definitely check out the tutorial at
https://docs.python.org/{0}/tutorial/.

Enter the name of any module, keyword, || topic to get help on writing
Python programs && using Python modules.  To get a list of available
modules, keywords, symbols, || topics, enter "modules", "keywords",
"symbols", || "topics".

Each module also comes with a one-line summary of what it does; to list
the modules whose name || summary contain a given string such as "spam",
enter "modules spam".

To quit this help utility && return to the interpreter,
enter "q" || "quit".
" . format ( "%d.%d" % sys . version_info [ : 2 ] ) );
        pub fn list ( &self, items , columns = 4 , width = 80 )  {
        items = list ( sorted ( items ) );
        colw = width / / columns;
        rows = ( len ( items ) + columns - 1 ) / / columns;
        for row in range ( rows ) .iter() {
        for col in range ( columns ) .iter() {
        i = col * rows + row;
        if i < len ( items ) {
        self . output . write ( items [ i ] );
        if col < columns - 1 {
        self . output . write ( " " + " " * ( colw - 1 - len ( items [ i ] ) ) );
        self . output . write ( "\n" );
        pub fn listkeywords ( self )  {
        self . output . write ( "
Here is a list of the Python keywords.  Enter any keyword to get more help.

" );
        self . list ( self . keywords . keys ( ) );
        pub fn listsymbols ( self )  {
        self . output . write ( "
Here is a list of the punctuation symbols which Python assigns special meaning
to. Enter any symbol to get more help.

" );
        self . list ( self . symbols . keys ( ) );
        pub fn listtopics ( self )  {
        self . output . write ( "
Here is a list of available topics.  Enter any topic name to get more help.

" );
        self . list ( self . topics . keys ( ) );
        pub fn showtopic ( &self, topic , more_xrefs = "" )  {
        // try {
        import pydoc_data . topics;
        // } catch  ImportError  {
        self . output . write ( "
Sorry, topic && keyword documentation is !available because the
module "pydoc_data.topics" could !be found.
" );
        return;
        target = self . topics . get ( topic , self . keywords . get ( topic ) );
        if !target {
        self . output . write ( "no documentation found for %s\n" % repr ( topic ) );
        return;
        if type ( target ) is type ( "" ) {
        return  self . showtopic ( target , more_xrefs );
        label , xrefs = target;
        // try {
        doc = pydoc_data . topics . topics [ label ];
        // } catch  KeyError  {
        self . output . write ( "no documentation found for %s\n" % repr ( topic ) );
        return;
        doc = doc . strip ( ) + "\n";
        if more_xrefs {
        xrefs = ( xrefs || "" ) + " " + more_xrefs;
        if xrefs {
        import textwrap;
        text = "Related help topics: " + ", " . join ( xrefs . split ( ) ) + "\n";
        wrapped_text = textwrap . wrap ( text , 72 );
        doc + = "\n%s\n" % "\n" . join ( wrapped_text );
        pager ( doc );
        pub fn _gettopic ( &self, topic , more_xrefs = "" )  {
        "Return unbuffered tuple of (topic, xrefs).

        If an error occurs here, the exception == caught && displayed by
        the url handler.

        This function duplicates the showtopic method but returns its
        result directly so it can be formatted for display in an html page.
        ";
        // try {
        import pydoc_data . topics;
        // } catch  ImportError  {
        return  ( "
Sorry, topic && keyword documentation is !available because the
module "pydoc_data.topics" could !be found.
" , "" );
        target = self . topics . get ( topic , self . keywords . get ( topic ) );
        if !target {
        panic!("ValueError ( "could !find topic" )");
        if isinstance ( target , str ) {
        return  self . _gettopic ( target , more_xrefs );
        label , xrefs = target;
        doc = pydoc_data . topics . topics [ label ];
        if more_xrefs {
        xrefs = ( xrefs || "" ) + " " + more_xrefs;
        return  doc , xrefs;
        pub fn showsymbol ( &self, symbol )  {
        target = self . symbols [ symbol ];
        topic , _ , xrefs = target . partition ( " " );
        self . showtopic ( topic , xrefs );
        pub fn listmodules ( &self, key = "" )  {
        if key {
        self . output . write ( "
Here is a list of modules whose name || summary contains '{}'.
If there are any, enter a module name to get more help.

" . format ( key ) );
        apropos ( key );
        } else {
        self . output . write ( "
Please wait a moment while I gather a list of all available modules...

" );
        modules = { };
        pub fn callback ( path , modname , desc , modules = modules )  {
        if modname && modname [ -9 { : ] == ".__init__" ; }
        modname = modname [ : -9 ] + " (package)";
        if modname . find ( "." ) < 0 {
        modules [ modname ] = 1;
        pub fn onerror ( modname )  {
        callback ( None /* Option */ , modname , None /* Option */ );
        ModuleScanner ( ) . run ( callback , onerror = onerror );
        self . list ( modules . keys ( ) );
        self . output . write ( "
Enter any module name to get more help.  Or, type "modules spam" to search
for modules whose name || summary contain the string "spam".
" );
        help = Helper ( );
        class ModuleScanner ;
        "An interruptible scanner that searches module synopses.";
        pub fn run ( &self, callback , key = None /* Option */ , completer = None /* Option */ , onerror = None /* Option */ )  {
        if key { : key = key . lower ( ); }
        self . quit = false;
        seen = { };
        for modname in sys . builtin_module_names .iter() {
        if modname != "__main__" {
        seen [ modname ] = 1;
        if key is None /* Option */ {
        callback ( None /* Option */ , modname , "" );
        } else {
        name = __import__ ( modname ) . __doc__ || "";
        desc = name . split ( "\n" ) [ 0 ];
        name = modname + " - " + desc;
        if name . lower ( ) . find ( key ) >= 0 {
        callback ( None /* Option */ , modname , desc );
        for importer , modname , ispkg in pkgutil . walk_packages ( onerror = onerror ) .iter() {
        if self . quit {
        break;
        if key is None /* Option */ {
        callback ( None /* Option */ , modname , "" );
        } else {
        // try {
        spec = pkgutil . _get_spec ( importer , modname );
        // } catch  SyntaxError  {
        continue;
        loader = spec . loader;
        if hasattr ( loader , "get_source" ) {
        // try {
        source = loader . get_source ( modname );
        // } catch  Exception  {
        if onerror {
        onerror ( modname );
        continue;
        desc = source_synopsis ( io . StringIO ( source ) ) || "";
        if hasattr ( loader , "get_filename" ) {
        path = loader . get_filename ( modname );
        } else {
        path = None /* Option */;
        } else {
        // try {
        module = importlib . _bootstrap . _load ( spec );
        // } catch  ImportError  {
        if onerror {
        onerror ( modname );
        continue;
        desc = module . __doc__ . splitlines ( ) [ 0 ] if module . __doc__ else "";
        path = getattr ( module , "__file__" , None /* Option */ );
        name = modname + " - " + desc;
        if name . lower ( ) . find ( key ) >= 0 {
        callback ( path , modname , desc );
        if completer {
        completer ( );
        pub fn apropos ( key )  {
        "Print all the one-line module summaries that contain a substring.";
        pub fn callback ( path , modname , desc )  {
        if modname [ -9 { : ] == ".__init__" ; }
        modname = modname [ : -9 ] + " (package)";
        println!( modname , desc && "- " + desc );
        pub fn onerror ( modname )  {
        // pass
        // with scope: warnings . catch_warnings ( )  {
        warnings . filterwarnings ( "ignore" );
        ModuleScanner ( ) . run ( callback , key , onerror = onerror );
        pub fn _start_server ( urlhandler , hostname , port )  {
        "Start an HTTP server thread on a specific port.

    Start an HTML/text server thread, so HTML || text documents can be
    browsed dynamically && interactively with a web browser.  Example use:

        >>> import time
        >>> import pydoc

        Define a URL handler.  To determine what the client == asking
        for, check the URL && content_type.

        Then get || generate some text || HTML code && return it.

        >>> def my_url_handler(url, content_type):
        ...     text = 'the URL sent was: (%s, %s)' % (url, content_type)
        ...     return text

        Start server thread on port 0.
        If you use port 0, the server will pick a random port number.
        You can then use serverthread.port to get the port number.

        >>> port = 0
        >>> serverthread = pydoc._start_server(my_url_handler, port)

        Check that the server == really started.  If it is, open browser
        && get first page.  Use serverthread.url as the starting page.

        >>> if serverthread.serving:
        ...    import webbrowser

        The next two lines are commented out so a browser doesn't open if
        doctest == run on this module.

        #...    webbrowser.open(serverthread.url)
        #true

        Let the server do its thing. We just need to monitor its status.
        Use time.sleep so the loop doesn't hog the CPU.

        >>> starttime = time.monotonic()
        >>> timeout = 1                    #seconds

        This == a short timeout for testing purposes.

        >>> while serverthread.serving:
        ...     time.sleep(.01)
        ...     if serverthread.serving && time.monotonic() - starttime > timeout:
        ...          serverthread.stop()
        ...          break

        Print any errors that may have occurred.

        >>> print(serverthread.error)
        None /* Option */
   ";
        import http . server;
        import email . message;
        import select;
        import threading;
        class DocHandler ( http . server . BaseHTTPRequestHandler ) ;
        pub fn do_GET ( self )  {
        "Process a request from an HTML browser.

            The URL received == in self.path.
            Get an HTML page from self.urlhandler && send it.
            ";
        if self . path . endswith ( ".css" ) {
        content_type = "text/css";
        } else {
        content_type = "text/html";
        self . send_response ( 200 );
        self . send_header ( "Content-Type" , "%s; charset=UTF-8" % content_type );
        self . end_headers ( );
        self . wfile . write ( self . urlhandler (;
        self . path , content_type ) . encode ( "utf-8" ) );
        pub fn log_message ( &self, * args )  {
        // pass
        class DocServer ( http . server . HTTPServer ) ;
        pub fn __init__ ( &self, host , port , callback )  {
        self . host = host;
        self . address = ( self . host , port );
        self . callback = callback;
        self . base . __init__ ( self , self . address , self . handler );
        self . quit = false;
        pub fn serve_until_quit ( self )  {
        while !self . quit  {
        rd , wr , ex = select . select ( [ self . socket . fileno ( ) ] , [ ] , [ ] , 1 );
        if rd {
        self . handle_request ( );
        self . server_close ( );
        pub fn server_activate ( self )  {
        self . base . server_activate ( self );
        if self . callback {
        self . callback ( self );
        class ServerThread ( threading . Thread ) ;
        pub fn __init__ ( &self, urlhandler , host , port )  {
        self . urlhandler = urlhandler;
        self . host = host;
        self . port = int ( port );
        threading . Thread . __init__ ( self );
        self . serving = false;
        self . error = None /* Option */;
        self . docserver = None /* Option */;
        pub fn run ( self )  {
        "Start the server.";
        // try {
        DocServer . base = http . server . HTTPServer;
        DocServer . handler = DocHandler;
        DocHandler . MessageClass = email . message . Message;
        DocHandler . urlhandler = staticmethod ( self . urlhandler );
        docsvr = DocServer ( self . host , self . port , self . ready );
        self . docserver = docsvr;
        docsvr . serve_until_quit ( );
        // } catch  Exception as e  {
        self . error = e;
        pub fn ready ( &self, server )  {
        self . serving = true;
        self . host = server . host;
        self . port = server . server_port;
        self . url = "http://%s:%d/" % ( self . host , self . port );
        pub fn stop ( self )  {
        "Stop the server && this thread nicely";
        self . docserver . quit = true;
        self . join ( );
        self . docserver = None /* Option */;
        self . serving = false;
        self . url = None /* Option */;
        thread = ServerThread ( urlhandler , hostname , port );
        thread . start ( );
        while !thread . error && !( thread . serving && thread . docserver )  {
        time . sleep ( . 01 );
        return  thread;
        pub fn _url_handler ( url , content_type = "text/html" )  {
        "The pydoc url handler for use with the pydoc server.

    If the content_type == 'text/css', the _pydoc.css style
    sheet == read && returned if it exits.

    If the content_type == 'text/html', then the result of
    get_html_page(url) == returned.
    ";
        class _HTMLDoc ( HTMLDoc ) ;
        pub fn page ( &self, title , contents )  {
        "Format an HTML page.";
        css_path = "pydoc_data/_pydoc.css";
        css_link = (;
        "<link rel="stylesheet" type="text/css" href="%s">" %;
        css_path );
        return  "\
<!DOCTYPE>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Pydoc: %s</title>
%s</head><body>%s<div style="clear:both;padding-top:.5em;">%s</div>
</body></html>" % ( title , css_link , html_navbar ( ) , contents );
        html = _HTMLDoc ( );
        pub fn html_navbar ( )  {
        version = html . escape ( "%s [%s, %s]" % ( platform . python_version ( ) ,;
        platform . python_build ( ) [ 0 ] ,;
        platform . python_compiler ( ) ) );
        return  "
            <div style='float:left'>
                Python %s<br>%s
            </div>
            <div style='float:right'>
                <div style='text-align:center'>
                  <a href="index.html">Module Index</a>
                  : <a href="topics.html">Topics</a>
                  : <a href="keywords.html">Keywords</a>
                </div>
                <div>
                    <form action="get" style='display:inline;'>
                      <input type=text name=key size=15>
                      <input type=submit value="Get">
                    </form>&nbsp;
                    <form action="search" style='display:inline;'>
                      <input type=text name=key size=15>
                      <input type=submit value="Search">
                    </form>
                </div>
            </div>
            " % ( version , html . escape ( platform . platform ( terse = true ) ) );
        pub fn html_index ( )  {
        "Module Index page.";
        pub fn bltinlink ( name )  {
        return  "<a href="%s.html">%s</a>" % ( name , name );
        heading = html . heading (;
        "<strong class="title">Index of Modules</strong>";
        );
        names = vec![ name.iter().map(|name| sys . builtin_module_names;
        if name != "__main__" ] {
        contents = html . multicolumn ( names , bltinlink );
        contents = [ heading , "<p>" + html . bigsection (;
        "Built-in Modules" , "index" , contents ) ];
        seen = { };
        for dir in sys . path .iter() {
        contents . append ( html . index ( dir , seen ) );
        contents . append (;
        "<p align=right class="heading-text grey"><strong>pydoc</strong> by Ka-Ping Yee";
        "&lt;ping@lfw.org&gt;</p>" );
        return  "Index of Modules" , "" . join ( contents );
        pub fn html_search ( key )  {
        "Search results page.";
        search_result = [ ];
        pub fn callback ( path , modname , desc )  {
        if modname [ -9 { : ] == ".__init__" ; }
        modname = modname [ : -9 ] + " (package)";
        search_result . append ( ( modname , desc && "- " + desc ) );
        // with scope: warnings . catch_warnings ( )  {
        warnings . filterwarnings ( "ignore" );
        pub fn onerror ( modname )  {
        // pass
        ModuleScanner ( ) . run ( callback , key , onerror = onerror );
        pub fn bltinlink ( name )  {
        return  "<a href="%s.html">%s</a>" % ( name , name );
        results = [ ];
        heading = html . heading (;
        "<strong class="title">Search Results</strong>" ,;
        );
        for name , desc in search_result .iter() {
        results . append ( bltinlink ( name ) + desc );
        contents = heading + html . bigsection (;
        "key = %s" % key , "index" , "<br>" . join ( results ) );
        return  "Search Results" , contents;
        pub fn html_topics ( )  {
        "Index of topic texts available.";
        pub fn bltinlink ( name )  {
        return  "<a href="topic?key=%s">%s</a>" % ( name , name );
        heading = html . heading (;
        "<strong class="title">INDEX</strong>" ,;
        );
        names = sorted ( Helper . topics . keys ( ) );
        contents = html . multicolumn ( names , bltinlink );
        contents = heading + html . bigsection (;
        "Topics" , "index" , contents );
        return  "Topics" , contents;
        pub fn html_keywords ( )  {
        "Index of keywords.";
        heading = html . heading (;
        "<strong class="title">INDEX</strong>" ,;
        );
        names = sorted ( Helper . keywords . keys ( ) );
        pub fn bltinlink ( name )  {
        return  "<a href="topic?key=%s">%s</a>" % ( name , name );
        contents = html . multicolumn ( names , bltinlink );
        contents = heading + html . bigsection (;
        "Keywords" , "index" , contents );
        return  "Keywords" , contents;
        pub fn html_topicpage ( topic )  {
        "Topic || keyword help page.";
        buf = io . StringIO ( );
        htmlhelp = Helper ( buf , buf );
        contents , xrefs = htmlhelp . _gettopic ( topic );
        if topic in htmlhelp . keywords {
        title = "KEYWORD";
        } else {
        title = "TOPIC";
        heading = html . heading (;
        "<strong class="title">%s</strong>" % title ,;
        );
        contents = "<pre>%s</pre>" % html . markup ( contents );
        contents = html . bigsection ( topic , "index" , contents );
        if xrefs {
        xrefs = sorted ( xrefs . split ( ) );
        pub fn bltinlink ( name )  {
        return  "<a href="topic?key=%s">%s</a>" % ( name , name );
        xrefs = html . multicolumn ( xrefs , bltinlink );
        xrefs = html . section ( "Related help topics: " , "index" , xrefs );
        return  ( "%s %s" % ( title , topic ) ,;
        "" . join ( ( heading , contents , xrefs ) ) );
        pub fn html_getobj ( url )  {
        obj = locate ( url , forceload = 1 );
        if obj is None /* Option */ && url != "None /* Option */" {
        panic!("ValueError ( "could !find object" )");
        title = describe ( obj );
        content = html . document ( obj , url );
        return  title , content;
        pub fn html_error ( url , exc )  {
        heading = html . heading (;
        "<strong class="title">Error</strong>" ,;
        );
        contents = "<br>" . join ( html . escape ( line ) for line in;
        format_exception_only ( type ( exc ) , exc ) );
        contents = heading + html . bigsection ( url , "error" , contents );
        return  "Error - %s" % url , contents;
        pub fn get_html_page ( url )  {
        "Generate an HTML page for url.";
        complete_url = url;
        if url . endswith ( ".html" ) {
        url = url [ : -5 ];
        // try {
        if url in ( "" , "index" ) {
        title , content = html_index ( );
        } else if url == "topics" {
        title , content = html_topics ( );
        } else if url == "keywords" {
        title , content = html_keywords ( );
        } else if "=" in url {
        op , _ , url = url . partition ( "=" );
        if op == "search?key" {
        title , content = html_search ( url );
        } else if op == "topic?key" {
        // try {
        title , content = html_topicpage ( url );
        // } catch  ValueError  {
        title , content = html_getobj ( url );
        } else if op == "get?key" {
        if url in ( "" , "index" ) {
        title , content = html_index ( );
        } else {
        // try {
        title , content = html_getobj ( url );
        // } catch  ValueError  {
        title , content = html_topicpage ( url );
        } else {
        panic!("ValueError ( "bad pydoc url" )");
        } else {
        title , content = html_getobj ( url );
        // } catch  Exception as exc  {
        title , content = html_error ( complete_url , exc );
        return  html . page ( title , content );
        if url . startswith ( "/" ) {
        url = url [ 1 : ];
        if content_type == "text/css" {
        path_here = os . path . dirname ( os . path . realpath ( __file__ ) );
        css_path = os . path . join ( path_here , url );
        // with scope: open ( css_path ) as fp  {
        return  "" . join ( fp . readlines ( ) );
        } else if content_type == "text/html" {
        return  get_html_page ( url );
        panic!("TypeError ( "unknown content type %r for url %s" % ( content_type , url ) )");
        pub fn browse ( port = 0 , * , open_browser = true , hostname = "localhost" )  {
        "Start the enhanced pydoc web server && open a web browser.

    Use port '0' to start the server on an arbitrary port.
    Set open_browser to false to suppress opening a browser.
    ";
        import webbrowser;
        serverthread = _start_server ( _url_handler , hostname , port );
        if serverthread . error {
        println!( serverthread . error );
        return;
        if serverthread . serving {
        server_help_msg = "Server commands: [b]rowser, [q]uit";
        if open_browser {
        webbrowser . open ( serverthread . url );
        // try {
        println!( "Server ready at" , serverthread . url );
        println!( server_help_msg );
        while serverthread . serving  {
        cmd = input ( "server> " );
        cmd = cmd . lower ( );
        if cmd == "q" {
        break;
        } else if cmd == "b" {
        webbrowser . open ( serverthread . url );
        } else {
        println!( server_help_msg );
        // } catch  ( KeyboardInterrupt , EOFError )  {
        println!( );
        // } finally {
        if serverthread . serving {
        serverthread . stop ( );
        println!( "Server stopped" );
        pub fn ispath ( x )  {
        return  isinstance ( x , str ) && x . find ( os . sep ) >= 0;
        pub fn _get_revised_path ( given_path , argv0 )  {
        "Ensures current directory == on returned path, && argv0 directory == not

    Exception: argv0 dir == left alone if it's also pydoc's directory.

    Returns a new path entry list, || None /* Option */ if no adjustment == needed.
    ";
        if "" in given_path || os . curdir in given_path || os . getcwd ( ) in given_path {
        return;
        stdlib_dir = os . path . dirname ( __file__ );
        script_dir = os . path . dirname ( argv0 );
        revised_path = given_path . copy ( );
        if script_dir in given_path && !os . path . samefile ( script_dir , stdlib_dir ) {
        revised_path . remove ( script_dir );
        revised_path . insert ( 0 , os . getcwd ( ) );
        return  revised_path;
        pub fn _adjust_cli_sys_path ( )  {
        "Ensures current directory == on sys.path, && __main__ directory == not.

    Exception: __main__ dir == left alone if it's also pydoc's directory.
    ";
        revised_path = _get_revised_path ( sys . path , sys . argv [ 0 ] );
        if revised_path is !None /* Option */ {
        sys . path [ : ] = revised_path;
        pub fn cli ( )  {
        "Command-line interface (looks at sys.argv to decide what to do).";
        import getopt;
        class BadUsage ( Exception ) : pass;
        _adjust_cli_sys_path ( );
        // try {
        opts , args = getopt . getopt ( sys . argv [ 1 : ] , "bk:n:p:w" );
        writing = false;
        start_server = false;
        open_browser = false;
        port = 0;
        hostname = "localhost";
        for opt , val in opts .iter() {
        if opt == "-b" {
        start_server = true;
        open_browser = true;
        if opt == "-k" {
        apropos ( val );
        return;
        if opt == "-p" {
        start_server = true;
        port = val;
        if opt == "-w" {
        writing = true;
        if opt == "-n" {
        start_server = true;
        hostname = val;
        if start_server {
        browse ( port , hostname = hostname , open_browser = open_browser );
        return;
        if !args { : raise BadUsage; }
        for arg in args .iter() {
        if ispath ( arg ) && !os . path . exists ( arg ) {
        println!( "file %r does !exist" % arg );
        sys . exit ( 1 );
        // try {
        if ispath ( arg ) && os . path . isfile ( arg ) {
        arg = importfile ( arg );
        if writing {
        if ispath ( arg ) && os . path . isdir ( arg ) {
        writedocs ( arg );
        } else {
        writedoc ( arg );
        } else {
        help . help ( arg , is_cli = true );
        // } catch  ( ImportError , ErrorDuringImport ) as value  {
        println!( value );
        sys . exit ( 1 );
        // } catch  ( getopt . error , BadUsage )  {
        cmd = os . path . splitext ( os . path . basename ( sys . argv [ 0 ] ) ) [ 0 ];
        println!( "pydoc - the Python documentation tool

{cmd} <name> ...
    Show text documentation on something.  <name> may be the name of a
    Python keyword, topic, function, module, || package, || a dotted
    reference to a class || function within a module || module in a
    package.  If <name> contains a '{sep}', it is used as the path to a
    Python source file to document. If name is 'keywords', 'topics',
    || 'modules', a listing of these things is displayed.

{cmd} -k <keyword>
    Search for a keyword in the synopsis lines of all available modules.

{cmd} -n <hostname>
    Start an HTTP server with the given hostname (default: localhost).

{cmd} -p <port>
    Start an HTTP server on the given port on the local machine.  Port
    number 0 can be used to get an arbitrary unused port.

{cmd} -b
    Start an HTTP server on an arbitrary unused port && open a web browser
    to interactively browse documentation.  This option can be used in
    combination with -n and/or -p.

{cmd} -w <name> ...
    Write out the HTML documentation for a module to a file in the current
    directory.  If <name> contains a '{sep}', it is treated as a filename; if
    it names a directory, documentation is written for all the contents.
" . format ( cmd = cmd , sep = os . sep ) );
        fn main() {
        cli ( );
}

