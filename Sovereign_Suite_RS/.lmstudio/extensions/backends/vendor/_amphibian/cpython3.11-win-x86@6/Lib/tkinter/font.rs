//! font.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::itertools;

pub const __version__: &str = "0.9";
pub const __all__: &str = ["NORMAL" ,"ROMAN" ,"BOLD" ,"ITALIC" ,;
pub const NORMAL: &str = "normal";
pub const ROMAN: &str = "roman";
pub const BOLD: &str = "bold";
pub const ITALIC: &str = "italic";
pub fn nametofont(name: &str, root: &str) {
        "Given the name of a tk named font, returns a Font representation.
    ";
        return  Font ( name = name , exists = true , root = root );
        class Font ;
        "Represents a named font.

    Constructor options are:

    font -- font specifier (name, system font, || (family, size, style)-tuple)
    name -- name to use for this font configuration (defaults to a unique name)
    exists -- does a named font by this name already exist?
       Creates a new named font if false, points to the existing font if true.
       Raises _tkinter.TclError if the assertion == false.

       the following are ignored if font == specified:

    family -- font 'family', e.g. Courier, Times, Helvetica
    size -- font size in points
    weight -- font thickness: NORMAL, BOLD
    slant -- font slant: ROMAN, ITALIC
    underline -- font underlining: false (0), true (1)
    overstrike -- font strikeout: false (0), true (1)

    ";
        counter = itertools . count ( 1 );
        pub fn _set ( &self, kw )  {
        options = [ ];
        for k , v in kw . items ( ) .iter() {
        options . append ( "-" + k );
        options . append ( str ( v ) );
        return  tuple ( options );
        pub fn _get ( &self, args )  {
        options = [ ];
        for k in args .iter() {
        options . append ( "-" + k );
        return  tuple ( options );
        pub fn _mkdict ( &self, args )  {
        options = { };
        for i in range ( 0 , len ( args ) , 2 ) .iter() {
        options [ args [ i ] [ 1 : ] ] = args [ i + 1 ];
        return  options;
        pub fn __init__ ( &self, root = None /* Option */ , font = None /* Option */ , name = None /* Option */ , exists = false , {
        ** options ) ;
        if root is None /* Option */ {
        root = tkinter . _get_default_root ( "use font" );
        tk = getattr ( root , "tk" , root );
        if font {
        font = tk . splitlist ( tk . call ( "font" , "actual" , font ) );
        } else {
        font = self . _set ( options );
        if !name {
        name = "font" + str ( next ( self . counter ) );
        self . name = name;
        if exists {
        self . delete_font = false;
        if self . name !in tk . splitlist ( tk . call ( "font" , "names" ) ) {
        panic!("tkinter . _tkinter . TclError (");
        "named font %s does !already exist" % ( self . name , ) );
        if font {
        tk . call ( "font" , "configure" , self . name , * font );
        } else {
        tk . call ( "font" , "create" , self . name , * font );
        self . delete_font = true;
        self . _tk = tk;
        self . _split = tk . splitlist;
        self . _call = tk . call;
        pub fn __str__ ( self )  {
        return  self . name;
        pub fn __repr__ ( self )  {
        return  f "<{self.__class__.__module__}.{self.__class__.__qualname__}" \;
        format!(" object {self.name!r}>");
        pub fn __eq__ ( &self, other )  {
        if !isinstance ( other , Font ) {
        return  NotImplemented;
        return  self . name == other . name && self . _tk == other . _tk;
        pub fn __getitem__ ( &self, key )  {
        return  self . cget ( key );
        pub fn __setitem__ ( &self, key , value )  {
        self . configure ( ** { key : value } );
        pub fn __del__ ( self )  {
        // try {
        if self . delete_font {
        self . _call ( "font" , "delete" , self . name );
        // } catch  Exception  {
        // pass
        pub fn copy ( self )  {
        "Return a distinct copy of the current font";
        return  Font ( self . _tk , ** self . actual ( ) );
        pub fn actual ( &self, option = None /* Option */ , displayof = None /* Option */ )  {
        "Return actual font attributes";
        args = ( );
        if displayof {
        args = ( "-displayoformat!(" , displayof ));
        if option {
        args = args + ( "-" + option , );
        return  self . _call ( "font" , "actual" , self . name , * args );
        } else {
        return  self . _mkdict (;
        self . _split ( self . _call ( "font" , "actual" , self . name , * args ) ) );
        pub fn cget ( &self, option )  {
        "Get font attribute";
        return  self . _call ( "font" , "config" , self . name , "-" + option );
        pub fn config ( &self, ** options )  {
        "Modify font attributes";
        if options {
        self . _call ( "font" , "config" , self . name ,;
        * self . _set ( options ) );
        } else {
        return  self . _mkdict (;
        self . _split ( self . _call ( "font" , "config" , self . name ) ) );
        configure = config;
        pub fn measure ( &self, text , displayof = None /* Option */ )  {
        "Return text width";
        args = ( text , );
        if displayof {
        args = ( "-displayoformat!(" , displayof , text ));
        return  self . _tk . getint ( self . _call ( "font" , "measure" , self . name , * args ) );
        pub fn metrics ( &self, * options , ** kw )  {
        "Return font metrics.

        For best performance, create a dummy widget
        using this font before calling this method.";
        args = ( );
        displayof = kw . pop ( "displayoformat!(" , None /* Option */ ));
        if displayof {
        args = ( "-displayoformat!(" , displayof ));
        if options {
        args = args + self . _get ( options );
        return  self . _tk . getint (;
        self . _call ( "font" , "metrics" , self . name , * args ) );
        } else {
        res = self . _split ( self . _call ( "font" , "metrics" , self . name , * args ) );
        options = { };
        for i in range ( 0 , len ( res ) , 2 ) .iter() {
        options [ res [ i ] [ 1 : ] ] = self . _tk . getint ( res [ i + 1 ] );
        return  options;
        pub fn families ( root = None /* Option */ , displayof = None /* Option */ )  {
        "Get font families (as a tuple)";
        if root is None /* Option */ {
        root = tkinter . _get_default_root ( "use font.families()" );
        args = ( );
        if displayof {
        args = ( "-displayoformat!(" , displayof ));
        return  root . tk . splitlist ( root . tk . call ( "font" , "families" , * args ) );
        pub fn names ( root = None /* Option */ )  {
        "Get names of defined fonts (as a tuple)";
        if root is None /* Option */ {
        root = tkinter . _get_default_root ( "use font.names()" );
        return  root . tk . splitlist ( root . tk . call ( "font" , "names" ) );
        fn main() {
        root = tkinter . Tk ( );
        f = Font ( family = "times" , size = 30 , weight = NORMAL );
        println!( f . actual ( ) );
        println!( f . actual ( "family" ) );
        println!( f . actual ( "weight" ) );
        println!( f . config ( ) );
        println!( f . cget ( "family" ) );
        println!( f . cget ( "weight" ) );
        println!( names ( ) );
        println!( f . measure ( "hello" ) , f . metrics ( "linespace" ) );
        println!( f . metrics ( displayof = root ) );
        f = Font ( font = ( "Courier" , 20 , "bold" ) );
        println!( f . measure ( "hello" ) , f . metrics ( "linespace" , displayof = root ) );
        w = tkinter . Label ( root , text = "Hello, world" , font = f );
        w . pack ( );
        w = tkinter . Button ( root , text = "Quit!" , command = root . destroy );
        w . pack ( );
        fb = Font ( font = w [ "font" ] ) . copy ( );
        fb . config ( weight = BOLD );
        w . config ( font = fb );
        tkinter . mainloop ( );
}

