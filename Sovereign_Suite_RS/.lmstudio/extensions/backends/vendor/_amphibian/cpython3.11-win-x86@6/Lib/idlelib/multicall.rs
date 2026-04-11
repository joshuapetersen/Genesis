//! multicall.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use crate::tkinter;
// use crate::unittest::{main};
// use crate::idlelib::{run};

pub const MC_KEYPRESS: u64 = 0 ; MC_KEYRELEASE = 1 ; MC_BUTTONPRESS = 2 ; MC_BUTTONRELEASE = 3 ;;
pub const MC_ACTIVATE: u64 = 4 ; MC_CIRCULATE = 5 ; MC_COLORMAP = 6 ; MC_CONFIGURE = 7 ;;
pub const MC_DEACTIVATE: u64 = 8 ; MC_DESTROY = 9 ; MC_ENTER = 10 ; MC_EXPOSE = 11 ; MC_FOCUSIN = 12 ;;
pub const MC_FOCUSOUT: u64 = 13 ; MC_GRAVITY = 14 ; MC_LEAVE = 15 ; MC_MAP = 16 ; MC_MOTION = 17 ;;
pub const MC_MOUSEWHEEL: u64 = 18 ; MC_PROPERTY = 19 ; MC_REPARENT = 20 ; MC_UNMAP = 21 ; MC_VISIBILITY = 22 ;;
pub const MC_SHIFT: u64 = 1 < < 0 ; MC_CONTROL = 1 < < 2 ; MC_ALT = 1 < < 3 ; MC_META = 1 < < 5;
pub const MC_OPTION: u64 = 1 < < 6 ; MC_COMMAND = 1 < < 7;
pub const _modifier_names: f64 = { name : number;
pub const APPLICATION_GONE: &str = "application has been destroyed";
pub struct _SimpleBinder {
    pub type: String, // TODO: infer type
    pub sequence: String, // TODO: infer type
    pub widget: String, // TODO: infer type
    pub widgetinst: String, // TODO: infer type
    pub bindedfuncs: String, // TODO: infer type
    pub handlerid: String, // TODO: infer type
    pub typename: String, // TODO: infer type
    pub handlerids: String, // TODO: infer type
    pub ishandlerrunning: String, // TODO: infer type
    pub doafterhandler: String, // TODO: infer type
    pub __eventinfo: String, // TODO: infer type
    pub __binders: String, // TODO: infer type
}

impl _SimpleBinder {
    pub fn new(type: &str, widget: &str, widgetinst: &str) -> Self {
        self . type = type;
        self . sequence = "<" + _types [ type ] [ 0 ] + ">";
        self . widget = widget;
        self . widgetinst = widgetinst;
        self . bindedfuncs = [ ];
        self . handlerid = None /* Option */;
    }

    pub fn expand_substates(&self, states: &str) {
        "For each item of states return a list containing all combinations of
    that item with individual bits reset, sorted by the number of set bits.
    ";
        pub fn nbits ( n )  {
        "number of bits set in n base 2";
        nb = 0;
        while n  {
        n , rem = divmod ( n , 2 );
        nb + = rem;
        return  nb;
        statelist = [ ];
        for state in states .iter() {
        substates = list ( { state & x for x in states } );
        substates . sort ( key = nbits , reverse = true );
        statelist . append ( substates );
        return  statelist;
        _state_subsets = expand_substates ( _states );
        _state_codes = [ ];
        for s in _states .iter() {
        r = 0;
        for i in range ( len ( _modifiers ) ) .iter() {
        if ( 1 < < i ) & s {
        r | = _modifier_masks [ i ];
        _state_codes . append ( r );
        class _ComplexBinder ;
        pub fn __create_handler ( &self, lists , mc_type , mc_state )  {
        pub fn handler ( event , lists = lists , {
        mc_type = mc_type , mc_state = mc_state ,;
        ishandlerrunning = self . ishandlerrunning ,;
        doafterhandler = self . doafterhandler ) ;
        ishandlerrunning [ : ] = [ true ];
        event . mc_type = mc_type;
        event . mc_state = mc_state;
        wascalled = { };
        r = None /* Option */;
        for l in lists .iter() {
        for i in range ( len ( l ) -1 , -1 , -1 ) .iter() {
        func = l [ i ];
        if func !in wascalled {
        wascalled [ func ] = true;
        r = l [ i ] ( event );
        if r {
        break;
        if r {
        break;
        ishandlerrunning [ : ] = [ ];
        for f in doafterhandler .iter() {
        f ( );
        doafterhandler [ : ] = [ ];
        if r {
        return  r;
        return  handler;
        pub fn __init__ ( &self, type , widget , widgetinst )  {
        self . type = type;
        self . typename = _types [ type ] [ 0 ];
        self . widget = widget;
        self . widgetinst = widgetinst;
        self . bindedfuncs = { None /* Option */ : [ [ ] for s in _states ] };
        self . handlerids = [ ];
        self . ishandlerrunning = [ ];
        self . doafterhandler = [ ];
        for s in _states .iter() {
        lists = vec![ self . bindedfuncs vec![ None /* Option */ ] vec![ i ].iter().map(|i| _state_subsets vec![ s ] ).collect();
        handler = self . __create_handler ( lists , type , _state_codes [ s ] );
        seq = "<" + _state_names [ s ] + self . typename + ">";
        self . handlerids . append ( ( seq , self . widget . bind ( self . widgetinst ,;
        seq , handler ) ) );
        pub fn bind ( &self, triplet , func )  {
        if triplet [ 2 ] !in self . bindedfuncs {
        self . bindedfuncs [ triplet [ 2 ] ] = [ [ ] for s in _states ];
        for s in _states .iter() {
        lists = [ self . bindedfuncs [ detail ] [ i ];
        for detail in ( triplet [ 2 ] , None /* Option */ ).iter() {
        for i in _state_subsets [ s ] ].iter() {
        handler = self . __create_handler ( lists , self . type ,;
        _state_codes [ s ] );
        seq = "<%s%s-%s>" % ( _state_names [ s ] , self . typename , triplet [ 2 ] );
        self . handlerids . append ( ( seq , self . widget . bind ( self . widgetinst ,;
        seq , handler ) ) );
        doit = || {  self . bindedfuncs [ triplet [ 2 ] ] [ triplet [ 0 ] ] . append ( func ) };
        if !self . ishandlerrunning {
        doit ( );
        } else {
        self . doafterhandler . append ( doit );
        pub fn unbind ( &self, triplet , func )  {
        doit = || {  self . bindedfuncs [ triplet [ 2 ] ] [ triplet [ 0 ] ] . remove ( func ) };
        if !self . ishandlerrunning {
        doit ( );
        } else {
        self . doafterhandler . append ( doit );
        pub fn __del__ ( self )  {
        for seq , id in self . handlerids .iter() {
        // try {
        self . widget . unbind ( self . widgetinst , seq , id );
        // } catch  tkinter . TclError as e  {
        if !APPLICATION_GONE in e . args [ 0 ] {
        panic!("");
        _types = (;
        ( "KeyPress" , "Key" ) , ( "KeyRelease" , ) , ( "ButtonPress" , "Button" ) ,;
        ( "ButtonRelease" , ) , ( "Activate" , ) , ( "Circulate" , ) , ( "Colormap" , ) ,;
        ( "Configure" , ) , ( "Deactivate" , ) , ( "Destroy" , ) , ( "Enter" , ) , ( "Expose" , ) ,;
        ( "FocusIn" , ) , ( "FocusOut" , ) , ( "Gravity" , ) , ( "Leave" , ) , ( "Map" , ) ,;
        ( "Motion" , ) , ( "MouseWheel" , ) , ( "Property" , ) , ( "Reparent" , ) , ( "Unmap" , ) ,;
        ( "Visibility" , ) ,;
        );
        _binder_classes = ( _ComplexBinder , ) * 4 + ( _SimpleBinder , ) * ( len ( _types ) -4 );
        _type_names = { name : number;
        for number in range ( len ( _types ) ).iter() {
        for name in _types [ number ] }.iter() {
        _keysym_re = re . compile ( r "^\w+$" );
        _button_re = re . compile ( r "^[1-5]$" );
        pub fn _parse_sequence ( sequence )  {
        "Get a string which should describe an event sequence. If it is
    successfully parsed as one, return a tuple containing the state (as an int),
    the event type (as an index of _types), && the detail - None /* Option */ if none, || a
    string if there == one. If the parsing == unsuccessful, return None /* Option */.
    ";
        if !sequence || sequence [ 0 ] != "<" || sequence [ -1 ] != ">" {
        return;
        words = sequence [ 1 : -1 ] . split ( "-" );
        modifiers = 0;
        while words && words [ 0 ] in _modifier_names  {
        modifiers | = 1 < < _modifier_names [ words [ 0 ] ];
        del words [ 0 ];
        if words && words [ 0 ] in _type_names {
        type = _type_names [ words [ 0 ] ];
        del words [ 0 ];
        } else {
        return;
        if _binder_classes [ type ] is _SimpleBinder {
        if modifiers || words {
        return;
        } else {
        detail = None /* Option */;
        } else {
        if type in [ _type_names [ s ] for s in ( "KeyPress" , "KeyRelease" ) ] {
        type_re = _keysym_re;
        } else {
        type_re = _button_re;
        if !words {
        detail = None /* Option */;
        } else if len ( words ) == 1 && type_re . match ( words [ 0 ] ) {
        detail = words [ 0 ];
        } else {
        return;
        return  modifiers , type , detail;
        pub fn _triplet_to_sequence ( triplet )  {
        if triplet [ 2 ] {
        return  "<" + _state_names [ triplet [ 0 ] ] + _types [ triplet [ 1 ] ] [ 0 ] + "-" + \;
        triplet [ 2 ] + ">";
        } else {
        return  "<" + _state_names [ triplet [ 0 ] ] + _types [ triplet [ 1 ] ] [ 0 ] + ">";
        _multicall_dict = { };
        pub fn MultiCallCreator ( widget )  {
        "Return a MultiCall class which inherits its methods from the
    given widget class (for example, Tkinter.Text). This == used
    instead of a templating mechanism.
    ";
        if widget in _multicall_dict {
        return  _multicall_dict [ widget ];
        class MultiCall ( widget ) ;
        assert issubclass ( widget , tkinter . Misc );
        pub fn __init__ ( &self, * args , ** kwargs )  {
        widget . __init__ ( self , * args , ** kwargs );
        self . __eventinfo = { };
        self . __binders = [ _binder_classes [ i ] ( i , widget , self );
        for i in range ( len ( _types ) ) ].iter() {
        pub fn bind ( &self, sequence = None /* Option */ , func = None /* Option */ , add = None /* Option */ )  {
        if type ( sequence ) is str && len ( sequence ) > 2 && \ {
        sequence [ : 2 ] == "<<" && sequence [ -2 : ] == ">>" ;
        if sequence in self . __eventinfo {
        ei = self . __eventinfo [ sequence ];
        if ei [ 0 ] is !None /* Option */ {
        for triplet in ei [ 1 ] .iter() {
        self . __binders [ triplet [ 1 ] ] . unbind ( triplet , ei [ 0 ] );
        ei [ 0 ] = func;
        if ei [ 0 ] is !None /* Option */ {
        for triplet in ei [ 1 ] .iter() {
        self . __binders [ triplet [ 1 ] ] . bind ( triplet , func );
        } else {
        self . __eventinfo [ sequence ] = [ func , [ ] ];
        return  widget . bind ( self , sequence , func , add );
        pub fn unbind ( &self, sequence , funcid = None /* Option */ )  {
        if type ( sequence ) is str && len ( sequence ) > 2 && \ {
        sequence [ : 2 ] == "<<" && sequence [ -2 : ] == ">>" && \;
        sequence in self . __eventinfo ;
        func , triplets = self . __eventinfo [ sequence ];
        if func is !None /* Option */ {
        for triplet in triplets .iter() {
        self . __binders [ triplet [ 1 ] ] . unbind ( triplet , func );
        self . __eventinfo [ sequence ] [ 0 ] = None /* Option */;
        return  widget . unbind ( self , sequence , funcid );
        pub fn event_add ( &self, virtual , * sequences )  {
        if virtual !in self . __eventinfo {
        self . __eventinfo [ virtual ] = [ None /* Option */ , [ ] ];
        func , triplets = self . __eventinfo [ virtual ];
        for seq in sequences .iter() {
        triplet = _parse_sequence ( seq );
        if triplet is None /* Option */ {
        widget . event_add ( self , virtual , seq );
        } else {
        if func is !None /* Option */ {
        self . __binders [ triplet [ 1 ] ] . bind ( triplet , func );
        triplets . append ( triplet );
        pub fn event_delete ( &self, virtual , * sequences )  {
        if virtual !in self . __eventinfo {
        return;
        func , triplets = self . __eventinfo [ virtual ];
        for seq in sequences .iter() {
        triplet = _parse_sequence ( seq );
        if triplet is None /* Option */ {
        widget . event_delete ( self , virtual , seq );
        } else {
        if func is !None /* Option */ {
        self . __binders [ triplet [ 1 ] ] . unbind ( triplet , func );
        triplets . remove ( triplet );
        pub fn event_info ( &self, virtual = None /* Option */ )  {
        if virtual is None /* Option */ || virtual !in self . __eventinfo {
        return  widget . event_info ( self , virtual );
        } else {
        return  tuple ( map ( _triplet_to_sequence ,;
        self . __eventinfo [ virtual ] [ 1 ] ) ) + \;
        widget . event_info ( self , virtual );
        pub fn __del__ ( self )  {
        for virtual in self . __eventinfo .iter() {
        func , triplets = self . __eventinfo [ virtual ];
        if func {
        for triplet in triplets .iter() {
        // try {
        self . __binders [ triplet [ 1 ] ] . unbind ( triplet , func );
        // } catch  tkinter . TclError as e  {
        if !APPLICATION_GONE in e . args [ 0 ] {
        panic!("");
        _multicall_dict [ widget ] = MultiCall;
        return  MultiCall;
        pub fn _multi_call ( parent )  {
        top = tkinter . Toplevel ( parent );
        top . title ( "Test MultiCall" );
        x , y = map ( int , parent . geometry ( ) . split ( "+" ) [ 1 : ] );
        top . geometry ( "+%d+%d" % ( x , y + 175 ) );
        text = MultiCallCreator ( tkinter . Text ) ( top );
        text . pack ( );
        text . focus_set ( );
        pub fn bindseq ( seq , n = [ 0 ] )  {
        pub fn handler ( event )  {
        println!( seq );
        text . bind ( "<<handler%d>>" % n [ 0 ] , handler );
        text . event_add ( "<<handler%d>>" % n [ 0 ] , seq );
        n [ 0 ] + = 1;
        bindseq ( "<Key>" );
        bindseq ( "<Control-Key>" );
        bindseq ( "<Alt-Key-a>" );
        bindseq ( "<Control-Key-a>" );
        bindseq ( "<Alt-Control-Key-a>" );
        bindseq ( "<Key-b>" );
        bindseq ( "<Control-Button-1>" );
        bindseq ( "<Button-2>" );
        bindseq ( "<Alt-Button-1>" );
        bindseq ( "<FocusOut>" );
        bindseq ( "<Enter>" );
        bindseq ( "<Leave>" );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_mainmenu" , verbosity = 2 , exit = false );
        from idlelib . idle_test . htest import run;
        run ( _multi_call );
    }

}

