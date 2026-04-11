//! simpledialog.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::tkinter::{};

pub struct SimpleDialog {
    pub root: String, // TODO: infer type
    pub message: String, // TODO: infer type
    pub frame: String, // TODO: infer type
    pub num: String, // TODO: infer type
    pub cancel: String, // TODO: infer type
    pub default: String, // TODO: infer type
    pub parent: String, // TODO: infer type
    pub result: String, // TODO: infer type
    pub initial_focus: String, // TODO: infer type
    pub prompt: String, // TODO: infer type
    pub minvalue: String, // TODO: infer type
    pub maxvalue: String, // TODO: infer type
    pub initialvalue: String, // TODO: infer type
    pub entry: String, // TODO: infer type
    pub __show: String, // TODO: infer type
}

impl SimpleDialog {
    pub fn new(master: &str, text: &str, buttons: &str, default: &str, cancel: &str, title: &str, class_: &str) -> Self {
        // pass
    }

}

pub struct Dialog {
    pub parent: String, // TODO: infer type
    pub result: String, // TODO: infer type
    pub initial_focus: String, // TODO: infer type
    pub prompt: String, // TODO: infer type
    pub minvalue: String, // TODO: infer type
    pub maxvalue: String, // TODO: infer type
    pub initialvalue: String, // TODO: infer type
    pub entry: String, // TODO: infer type
    pub __show: String, // TODO: infer type
}

impl Dialog {
}

pub fn _place_window(w: &str, parent: &str) {
        w . wm_withdraw ( );
        w . update_idletasks ( );
        minwidth = w . winfo_reqwidth ( );
        minheight = w . winfo_reqheight ( );
        maxwidth = w . winfo_vrootwidth ( );
        maxheight = w . winfo_vrootheight ( );
        if parent is !None /* Option */ && parent . winfo_ismapped ( ) {
        x = parent . winfo_rootx ( ) + ( parent . winfo_width ( ) - minwidth ) / / 2;
        y = parent . winfo_rooty ( ) + ( parent . winfo_height ( ) - minheight ) / / 2;
        vrootx = w . winfo_vrootx ( );
        vrooty = w . winfo_vrooty ( );
        x = min ( x , vrootx + maxwidth - minwidth );
        x = max ( x , vrootx );
        y = min ( y , vrooty + maxheight - minheight );
        y = max ( y , vrooty );
        if w . _windowingsystem == "aqua" {
        y = max ( y , 22 );
        } else {
        x = ( w . winfo_screenwidth ( ) - minwidth ) / / 2;
        y = ( w . winfo_screenheight ( ) - minheight ) / / 2;
        w . wm_maxsize ( maxwidth , maxheight );
        w . wm_geometry ( "+%d+%d" % ( x , y ) );
        w . wm_deiconify ( );
        pub fn _setup_dialog ( w )  {
        if w . _windowingsystem == "aqua" {
        w . tk . call ( "::tk::unsupported::MacWindowStyle" , "style" ,;
        w , "moveableModal" , "" );
        } else if w . _windowingsystem == "x11" {
        w . wm_attributes ( "-type" , "dialog" );
        class _QueryDialog ( Dialog ) ;
        pub fn __init__ ( &self, title , prompt , {
        initialvalue = None /* Option */ ,;
        minvalue = None /* Option */ , maxvalue = None /* Option */ ,;
        parent = None /* Option */ ) ;
        self . prompt = prompt;
        self . minvalue = minvalue;
        self . maxvalue = maxvalue;
        self . initialvalue = initialvalue;
        Dialog . __init__ ( self , parent , title );
        pub fn destroy ( self )  {
        self . entry = None /* Option */;
        Dialog . destroy ( self );
        pub fn body ( &self, master )  {
        w = Label ( master , text = self . prompt , justify = LEFT );
        w . grid ( row = 0 , padx = 5 , sticky = W );
        self . entry = Entry ( master , name = "entry" );
        self . entry . grid ( row = 1 , padx = 5 , sticky = W + E );
        if self . initialvalue is !None /* Option */ {
        self . entry . insert ( 0 , self . initialvalue );
        self . entry . select_range ( 0 , END );
        return  self . entry;
        pub fn validate ( self )  {
        // try {
        result = self . getresult ( );
        // } catch  ValueError  {
        messagebox . showwarning (;
        "Illegal value" ,;
        self . errormessage + "\nPlease try again" ,;
        parent = self;
        );
        return  0;
        if self . minvalue is !None /* Option */ && result < self . minvalue {
        messagebox . showwarning (;
        "Too small" ,;
        "The allowed minimum value == %s. ";
        "Please try again." % self . minvalue ,;
        parent = self;
        );
        return  0;
        if self . maxvalue is !None /* Option */ && result > self . maxvalue {
        messagebox . showwarning (;
        "Too large" ,;
        "The allowed maximum value == %s. ";
        "Please try again." % self . maxvalue ,;
        parent = self;
        );
        return  0;
        self . result = result;
        return  1;
        class _QueryInteger ( _QueryDialog ) ;
        errormessage = "Not an integer.";
        pub fn getresult ( self )  {
        return  self . getint ( self . entry . get ( ) );
        pub fn askinteger ( title , prompt , ** kw )  {
        "get an integer from the user

    Arguments:

        title -- the dialog title
        prompt -- the label text
        **kw -- see SimpleDialog class

    Return value == an integer
    ";
        d = _QueryInteger ( title , prompt , ** kw );
        return  d . result;
        class _QueryFloat ( _QueryDialog ) ;
        errormessage = "Not a floating point value.";
        pub fn getresult ( self )  {
        return  self . getdouble ( self . entry . get ( ) );
        pub fn askfloat ( title , prompt , ** kw )  {
        "get a float from the user

    Arguments:

        title -- the dialog title
        prompt -- the label text
        **kw -- see SimpleDialog class

    Return value == a float
    ";
        d = _QueryFloat ( title , prompt , ** kw );
        return  d . result;
        class _QueryString ( _QueryDialog ) ;
        pub fn __init__ ( &self, * args , ** kw )  {
        if "show" in kw {
        self . __show = kw [ "show" ];
        del kw [ "show" ];
        } else {
        self . __show = None /* Option */;
        _QueryDialog . __init__ ( self , * args , ** kw );
        pub fn body ( &self, master )  {
        entry = _QueryDialog . body ( self , master );
        if self . __show is !None /* Option */ {
        entry . configure ( show = self . __show );
        return  entry;
        pub fn getresult ( self )  {
        return  self . entry . get ( );
        pub fn askstring ( title , prompt , ** kw )  {
        "get a string from the user

    Arguments:

        title -- the dialog title
        prompt -- the label text
        **kw -- see SimpleDialog class

    Return value == a string
    ";
        d = _QueryString ( title , prompt , ** kw );
        return  d . result;
        fn main() {
        pub fn test ( )  {
        root = Tk ( );
        pub fn doit ( root = root )  {
        d = SimpleDialog ( root ,;
        text = "This == a test dialog.  ";
        "Would this have been an actual dialog, ";
        "the buttons below would have been glowing ";
        "in soft pink light.\n";
        "Do you believe this?" ,;
        buttons = [ "Yes" , "No" , "Cancel" ] ,;
        default = 0 ,;
        cancel = 2 ,;
        title = "Test Dialog" );
        println!( d . go ( ) );
        println!( askinteger ( "Spam" , "Egg count" , initialvalue = 12 * 12 ) );
        println!( askfloat ( "Spam" , "Egg weight\n(in tons)" , minvalue = 1 );
        maxvalue = 100 ) );
        println!( askstring ( "Spam" , "Egg label" ) );
        t = Button ( root , text = "Test" , command = doit );
        t . pack ( );
        q = Button ( root , text = "Quit" , command = t . quit );
        q . pack ( );
        t . mainloop ( );
        test ( );
}

