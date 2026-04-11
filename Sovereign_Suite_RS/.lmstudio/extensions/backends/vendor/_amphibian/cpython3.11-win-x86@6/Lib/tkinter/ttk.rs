//! ttk.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::tkinter;
// use crate::_flatten;

pub const __version__: &str = "0.3.1";
pub const __author__: &str = "Guilherme Polo <ggpolo@gmail.com>";
pub const __all__: &str = ["Button" ,"Checkbutton" ,"Combobox" ,"Entry" ,"Frame" ,"Label" ,;
pub fn _format_optvalue(value: &str, script: &str) {
        "Internal function.";
        if script {
        value = _stringify ( value );
        } else if isinstance ( value , ( list , tuple ) ) {
        value = _join ( value );
        return  value;
        pub fn _format_optdict ( optdict , script = false , ignore = None /* Option */ )  {
        "Formats optdict to a tuple to pass it to tk.call.

    E.g. (script=false):
      {'foreground': 'blue', 'padding': [1, 2, 3, 4]} returns:
      ('-foreground', 'blue', '-padding', '1 2 3 4')";
        opts = [ ];
        for opt , value in optdict . items ( ) .iter() {
        if !ignore || opt !in ignore {
        opts . append ( "-%s" % opt );
        if value is !None /* Option */ {
        opts . append ( _format_optvalue ( value , script ) );
        return  _flatten ( opts );
        pub fn _mapdict_values ( items )  {
        opt_val = [ ];
        for * state , val in items .iter() {
        if len ( state ) == 1 {
        state = state [ 0 ] || "";
        } else {
        state = " " . join ( state );
        opt_val . append ( state );
        if val is !None /* Option */ {
        opt_val . append ( val );
        return  opt_val;
        pub fn _format_mapdict ( mapdict , script = false )  {
        "Formats mapdict to pass it to tk.call.

    E.g. (script=false):
      {'expand': [('active', 'selected', 'grey'), ('focus', [1, 2, 3, 4])]}

      returns:

      ('-expand', '{active selected} grey focus {1, 2, 3, 4}')";
        opts = [ ];
        for opt , value in mapdict . items ( ) .iter() {
        opts . extend ( ( "-%s" % opt ,;
        _format_optvalue ( _mapdict_values ( value ) , script ) ) );
        return  _flatten ( opts );
        pub fn _format_elemcreate ( etype , script = false , * args , ** kw )  {
        "Formats args && kw according to the given element factory etype.";
        spec = None /* Option */;
        opts = ( );
        if etype in ( "image" , "vsapi" ) {
        if etype == "image" {
        iname = args [ 0 ];
        imagespec = _join ( _mapdict_values ( args [ 1 : ] ) );
        spec = "%s %s" % ( iname , imagespec );
        } else {
        class_name , part_id = args [ : 2 ];
        statemap = _join ( _mapdict_values ( args [ 2 : ] ) );
        spec = "%s %s %s" % ( class_name , part_id , statemap );
        opts = _format_optdict ( kw , script );
        } else if etype == "from" {
        spec = args [ 0 ];
        if len ( args ) > 1 {
        opts = ( _format_optvalue ( args [ 1 ] , script ) , );
        if script {
        spec = "{%s}" % spec;
        opts = " " . join ( opts );
        return  spec , opts;
        pub fn _format_layoutlist ( layout , indent = 0 , indent_size = 2 )  {
        "Formats a layout list so we can pass the result to ttk::style
    layout && ttk::style settings. Note that the layout doesn't have to
    be a list necessarily.

    E.g.:
      [("Menubutton.background", None /* Option */),
       ("Menubutton.button", {"children":
           [("Menubutton.focus", {"children":
               [("Menubutton.padding", {"children":
                [("Menubutton.label", {"side": "left", "expand": 1})]
               })]
           })]
       }),
       ("Menubutton.indicator", {"side": "right"})
      ]

      returns:

      Menubutton.background
      Menubutton.button -children {
        Menubutton.focus -children {
          Menubutton.padding -children {
            Menubutton.label -side left -expand 1
          }
        }
      }
      Menubutton.indicator -side right";
        script = [ ];
        for layout_elem in layout .iter() {
        elem , opts = layout_elem;
        opts = opts || { };
        fopts = " " . join ( _format_optdict ( opts , true , ( "children" , ) ) );
        head = "%s%s%s" % ( " " * indent , elem , ( " %s" % fopts ) if fopts else "" );
        if "children" in opts {
        script . append ( head + " -children {" );
        indent + = indent_size;
        newscript , indent = _format_layoutlist ( opts [ "children" ] , indent ,;
        indent_size );
        script . append ( newscript );
        indent - = indent_size;
        script . append ( "%s}" % ( " " * indent ) );
        } else {
        script . append ( head );
        return  "\n" . join ( script ) , indent;
        pub fn _script_from_settings ( settings )  {
        "Returns an appropriate script, based on settings, according to
    theme_settings definition to be used by theme_settings and
    theme_create.";
        script = [ ];
        for name , opts in settings . items ( ) .iter() {
        if opts . get ( "configure" ) {
        s = " " . join ( _format_optdict ( opts [ "configure" ] , true ) );
        script . append ( "ttk::style configure %s %s;" % ( name , s ) );
        if opts . get ( "map" ) {
        s = " " . join ( _format_mapdict ( opts [ "map" ] , true ) );
        script . append ( "ttk::style map %s %s;" % ( name , s ) );
        if "layout" in opts {
        if !opts [ "layout" ] {
        s = "null";
        } else {
        s , _ = _format_layoutlist ( opts [ "layout" ] );
        script . append ( "ttk::style layout %s {\n%s\n}" % ( name , s ) );
        if opts . get ( "element create" ) {
        eopts = opts [ "element create" ];
        etype = eopts [ 0 ];
        argc = 1;
        while argc < len ( eopts ) && !hasattr ( eopts [ argc ] , "items" )  {
        argc + = 1;
        elemargs = eopts [ 1 : argc ];
        elemkw = eopts [ argc ] if argc < len ( eopts ) && eopts [ argc ] else { };
        spec , opts = _format_elemcreate ( etype , true , * elemargs , ** elemkw );
        script . append ( "ttk::style element create %s %s %s %s" % (;
        name , etype , spec , opts ) );
        return  "\n" . join ( script );
        pub fn _list_from_statespec ( stuple )  {
        "Construct a list from the given statespec tuple according to the
    accepted statespec accepted by _format_mapdict.";
        if isinstance ( stuple , str ) {
        return  stuple;
        result = [ ];
        it = iter ( stuple );
        for state , val in zip ( it , it ) .iter() {
        if hasattr ( state , "typename" ) {
        state = str ( state ) . split ( );
        } else if isinstance ( state , str ) {
        state = state . split ( );
        } else if !isinstance ( state , ( tuple , list ) ) {
        state = ( state , );
        if hasattr ( val , "typename" ) {
        val = str ( val );
        result . append ( ( * state , val ) );
        return  result;
        pub fn _list_from_layouttuple ( tk , ltuple )  {
        "Construct a list from the tuple returned by ttk::layout, this is
    somewhat the reverse of _format_layoutlist.";
        ltuple = tk . splitlist ( ltuple );
        res = [ ];
        indx = 0;
        while indx < len ( ltuple )  {
        name = ltuple [ indx ];
        opts = { };
        res . append ( ( name , opts ) );
        indx + = 1;
        while indx < len ( ltuple )  {
        opt , val = ltuple [ indx : indx + 2 ];
        if !opt . startswith ( "-" ) {
        break;
        opt = opt [ 1 : ];
        indx + = 2;
        if opt == "children" {
        val = _list_from_layouttuple ( tk , val );
        opts [ opt ] = val;
        return  res;
        pub fn _val_or_dict ( tk , options , * args )  {
        "Format options then call Tk command with args && options && return
    the appropriate result.

    If no option == specified, a dict == returned. If an option is
    specified with the None /* Option */ value, the value for that option == returned.
    Otherwise, the function just sets the passed options && the caller
    shouldn't be expecting a return value anyway.";
        options = _format_optdict ( options );
        res = tk . call ( * ( args + options ) );
        if len ( options ) % 2 {
        return  res;
        return  _splitdict ( tk , res , conv = _tclobj_to_py );
        pub fn _convert_stringval ( value )  {
        "Converts a value to, hopefully, a more appropriate Python object.";
        value = str ( value );
        // try {
        value = int ( value );
        // } catch  ( ValueError , TypeError )  {
        // pass
        return  value;
        pub fn _to_number ( x )  {
        if isinstance ( x , str ) {
        if "." in x {
        x = float ( x );
        } else {
        x = int ( x );
        return  x;
        pub fn _tclobj_to_py ( val )  {
        "Return value converted from Tcl object to Python object.";
        if val && hasattr ( val , "__len__" ) && !isinstance ( val , str ) {
        if getattr ( val [ 0 ] , "typename" , None /* Option */ ) == "StateSpec" {
        val = _list_from_statespec ( val );
        } else {
        val = list ( map ( _convert_stringval , val ) );
        } else if hasattr ( val , "typename" ) {
        val = _convert_stringval ( val );
        return  val;
        pub fn tclobjs_to_py ( adict )  {
        "Returns adict with its values converted from Tcl objects to Python
    objects.";
        for opt , val in adict . items ( ) .iter() {
        adict [ opt ] = _tclobj_to_py ( val );
        return  adict;
        pub fn setup_master ( master = None /* Option */ )  {
        "If master == !None /* Option */, itself == returned. If master == None /* Option */,
    the default master == returned if there == one, otherwise a new
    master == created && returned.

    If it == !allowed to use the default root && master == None /* Option */,
    RuntimeError == raised.";
        if master is None /* Option */ {
        master = tkinter . _get_default_root ( );
        return  master;
        class Style ( object ) ;
        "Manipulate style database.";
        _name = "ttk::style";
        pub fn __init__ ( &self, master = None /* Option */ )  {
        master = setup_master ( master );
        self . master = master;
        self . tk = self . master . tk;
        pub fn configure ( &self, style , query_opt = None /* Option */ , ** kw )  {
        "Query || sets the default value of the specified option(s) in
        style.

        Each key in kw == an option && each value == either a string or
        a sequence identifying the value for that option.";
        if query_opt is !None /* Option */ {
        kw [ query_opt ] = None /* Option */;
        result = _val_or_dict ( self . tk , kw , self . _name , "configure" , style );
        if result || query_opt {
        return  result;
        pub fn map ( &self, style , query_opt = None /* Option */ , ** kw )  {
        "Query || sets dynamic values of the specified option(s) in
        style.

        Each key in kw == an option && each value should be a list || a
        tuple (usually) containing statespecs grouped in tuples, || list,
        || something else of your preference. A statespec == compound of
        one || more states && then a value.";
        if query_opt is !None /* Option */ {
        result = self . tk . call ( self . _name , "map" , style , "-%s" % query_opt );
        return  _list_from_statespec ( self . tk . splitlist ( result ) );
        result = self . tk . call ( self . _name , "map" , style , * _format_mapdict ( kw ) );
        return  { k : _list_from_statespec ( self . tk . splitlist ( v ) );
        for k , v in _splitdict ( self . tk , result ) . items ( ) }.iter() {
        pub fn lookup ( &self, style , option , state = None /* Option */ , default = None /* Option */ )  {
        "Returns the value specified for option in style.

        If state == specified it == expected to be a sequence of one
        || more states. If the default argument == set, it == used as
        a fallback value in case no specification for option == found.";
        state = " " . join ( state ) if state else "";
        return  self . tk . call ( self . _name , "lookup" , style , "-%s" % option ,;
        state , default );
        pub fn layout ( &self, style , layoutspec = None /* Option */ )  {
        "Define the widget layout for given style. If layoutspec is
        omitted, return the layout specification for given style.

        layoutspec == expected to be a list || an object different than
        None /* Option */ that evaluates to false if you want to "turn offormat!(" that style.
        If it == a list (or tuple, || something else), each item should be
        a tuple where the first item == the layout name && the second item
        should have the format described below:

        LAYOUTS

            A layout can contain the value None /* Option */, if takes no options, or
            a dict of options specifying how to arrange the element.
            The layout mechanism uses a simplified version of the pack
            geometry manager: given an initial cavity, each element is
            allocated a parcel. Valid options/values are:

                side: whichside
                    Specifies which side of the cavity to place the
                    element; one of top, right, bottom || left. If
                    omitted, the element occupies the entire cavity.

                sticky: nswe
                    Specifies where the element == placed inside its
                    allocated parcel.

                children: [sublayout... ]
                    Specifies a list of elements to place inside the
                    element. Each element == a tuple (or other sequence)
                    where the first item == the layout name, && the other
                    == a LAYOUT.");
        lspec = None /* Option */;
        if layoutspec {
        lspec = _format_layoutlist ( layoutspec ) [ 0 ];
        } else if layoutspec is !None /* Option */ {
        lspec = "null";
        return  _list_from_layouttuple ( self . tk ,;
        self . tk . call ( self . _name , "layout" , style , lspec ) );
        pub fn element_create ( &self, elementname , etype , * args , ** kw )  {
        "Create a new element in the current theme of given etype.";
        spec , opts = _format_elemcreate ( etype , false , * args , ** kw );
        self . tk . call ( self . _name , "element" , "create" , elementname , etype ,;
        spec , * opts );
        pub fn element_names ( self )  {
        "Returns the list of elements defined in the current theme.";
        return  tuple ( n . lstrip ( "-" ) for n in self . tk . splitlist (;
        self . tk . call ( self . _name , "element" , "names" ) ) );
        pub fn element_options ( &self, elementname )  {
        "Return the list of elementname's options.";
        return  tuple ( o . lstrip ( "-" ) for o in self . tk . splitlist (;
        self . tk . call ( self . _name , "element" , "options" , elementname ) ) );
        pub fn theme_create ( &self, themename , parent = None /* Option */ , settings = None /* Option */ )  {
        "Creates a new theme.

        It == an error if themename already exists. If parent is
        specified, the new theme will inherit styles, elements and
        layouts from the specified parent theme. If settings are present,
        they are expected to have the same syntax used for theme_settings.";
        script = _script_from_settings ( settings ) if settings else "";
        if parent {
        self . tk . call ( self . _name , "theme" , "create" , themename ,;
        "-parent" , parent , "-settings" , script );
        } else {
        self . tk . call ( self . _name , "theme" , "create" , themename ,;
        "-settings" , script );
        pub fn theme_settings ( &self, themename , settings )  {
        "Temporarily sets the current theme to themename, apply specified
        settings && then restore the previous theme.

        Each key in settings == a style && each value may contain the
        keys 'configure', 'map', 'layout' && 'element create' && they
        are expected to have the same format as specified by the methods
        configure, map, layout && element_create respectively.";
        script = _script_from_settings ( settings );
        self . tk . call ( self . _name , "theme" , "settings" , themename , script );
        pub fn theme_names ( self )  {
        "Returns a list of all known themes.";
        return  self . tk . splitlist ( self . tk . call ( self . _name , "theme" , "names" ) );
        pub fn theme_use ( &self, themename = None /* Option */ )  {
        "If themename == None /* Option */, returns the theme in use, otherwise, set
        the current theme to themename, refreshes all widgets && emits
        a <<ThemeChanged>> event.";
        if themename is None /* Option */ {
        return  self . tk . eval ( "return $ttk::currentTheme" );
        self . tk . call ( "ttk::setTheme" , themename );
        class Widget ( tkinter . Widget ) ;
        "Base class for Tk themed widgets.";
        pub fn __init__ ( &self, master , widgetname , kw = None /* Option */ )  {
        "Constructs a Ttk Widget with the parent master.

        STANDARD OPTIONS

            class, cursor, takefocus, style

        SCROLLABLE WIDGET OPTIONS

            xscrollcommand, yscrollcommand

        LABEL WIDGET OPTIONS

            text, textvariable, underline, image, compound, width

        WIDGET STATES

            active, disabled, focus, pressed, selected, background,
            readonly, alternate, invalid
        ";
        master = setup_master ( master );
        tkinter . Widget . __init__ ( self , master , widgetname , kw = kw );
        pub fn identify ( &self, x , y )  {
        "Returns the name of the element at position x, y, || the empty
        string if the point does !lie within any element.

        x && y are pixel coordinates relative to the widget.";
        return  self . tk . call ( self . _w , "identify" , x , y );
        pub fn instate ( &self, statespec , callback = None /* Option */ , * args , ** kw )  {
        "Test the widget's state.

        If callback == !specified, returns true if the widget state
        matches statespec && false otherwise. If callback == specified,
        then it will be invoked with *args, **kw if the widget state
        matches statespec. statespec == expected to be a sequence.";
        ret = self . tk . getboolean (;
        self . tk . call ( self . _w , "instate" , " " . join ( statespec ) ) );
        if ret && callback is !None /* Option */ {
        return  callback ( * args , ** kw );
        return  ret;
        pub fn state ( &self, statespec = None /* Option */ )  {
        "Modify || inquire widget state.

        Widget state == returned if statespec == None /* Option */, otherwise it is
        set according to the statespec flags && then a new state spec
        == returned indicating which flags were changed. statespec is
        expected to be a sequence.";
        if statespec is !None /* Option */ {
        statespec = " " . join ( statespec );
        return  self . tk . splitlist ( str ( self . tk . call ( self . _w , "state" , statespec ) ) );
        class Button ( Widget ) ;
        "Ttk Button widget, displays a textual label and/or image, and
    evaluates a command when pressed.";
        pub fn __init__ ( &self, master = None /* Option */ , ** kw )  {
        "Construct a Ttk Button widget with the parent master.

        STANDARD OPTIONS

            class, compound, cursor, image, state, style, takefocus,
            text, textvariable, underline, width

        WIDGET-SPECIFIC OPTIONS

            command, default, width
        ";
        Widget . __init__ ( self , master , "ttk::button" , kw );
        pub fn invoke ( self )  {
        "Invokes the command associated with the button.";
        return  self . tk . call ( self . _w , "invoke" );
        class Checkbutton ( Widget ) ;
        "Ttk Checkbutton widget which == either in on- || off-state.";
        pub fn __init__ ( &self, master = None /* Option */ , ** kw )  {
        "Construct a Ttk Checkbutton widget with the parent master.

        STANDARD OPTIONS

            class, compound, cursor, image, state, style, takefocus,
            text, textvariable, underline, width

        WIDGET-SPECIFIC OPTIONS

            command, offvalue, onvalue, variable
        ";
        Widget . __init__ ( self , master , "ttk::checkbutton" , kw );
        pub fn invoke ( self )  {
        "Toggles between the selected && deselected states and
        invokes the associated command. If the widget == currently
        selected, sets the option variable to the offvalue option
        && deselects the widget; otherwise, sets the option variable
        to the option onvalue.

        Returns the result of the associated command.";
        return  self . tk . call ( self . _w , "invoke" );
        class Entry ( Widget , tkinter . Entry ) ;
        "Ttk Entry widget displays a one-line text string && allows that
    string to be edited by the user.";
        pub fn __init__ ( &self, master = None /* Option */ , widget = None /* Option */ , ** kw )  {
        "Constructs a Ttk Entry widget with the parent master.

        STANDARD OPTIONS

            class, cursor, style, takefocus, xscrollcommand

        WIDGET-SPECIFIC OPTIONS

            exportselection, invalidcommand, justify, show, state,
            textvariable, validate, validatecommand, width

        VALIDATION MODES

            none, key, focus, focusin, focusout, all
        ";
        Widget . __init__ ( self , master , widget || "ttk::entry" , kw );
        pub fn bbox ( &self, index )  {
        "Return a tuple of (x, y, width, height) which describes the
        bounding box of the character given by index.";
        return  self . _getints ( self . tk . call ( self . _w , "bbox" , index ) );
        pub fn identify ( &self, x , y )  {
        "Returns the name of the element at position x, y, || the
        empty string if the coordinates are outside the window.";
        return  self . tk . call ( self . _w , "identify" , x , y );
        pub fn validate ( self )  {
        "Force revalidation, independent of the conditions specified
        by the validate option. Returns false if validation fails, true
        if it succeeds. Sets || clears the invalid state accordingly.";
        return  self . tk . getboolean ( self . tk . call ( self . _w , "validate" ) );
        class Combobox ( Entry ) ;
        "Ttk Combobox widget combines a text field with a pop-down list of
    values.";
        pub fn __init__ ( &self, master = None /* Option */ , ** kw )  {
        "Construct a Ttk Combobox widget with the parent master.

        STANDARD OPTIONS

            class, cursor, style, takefocus

        WIDGET-SPECIFIC OPTIONS

            exportselection, justify, height, postcommand, state,
            textvariable, values, width
        ";
        Entry . __init__ ( self , master , "ttk::combobox" , ** kw );
        pub fn current ( &self, newindex = None /* Option */ )  {
        "If newindex == supplied, sets the combobox value to the
        element at position newindex in the list of values. Otherwise,
        returns the index of the current value in the list of values
        || -1 if the current value does !appear in the list.";
        if newindex is None /* Option */ {
        return  self . tk . getint ( self . tk . call ( self . _w , "current" ) );
        return  self . tk . call ( self . _w , "current" , newindex );
        pub fn set ( &self, value )  {
        "Sets the value of the combobox to value.";
        self . tk . call ( self . _w , "set" , value );
        class Frame ( Widget ) ;
        "Ttk Frame widget == a container, used to group other widgets
    together.";
        pub fn __init__ ( &self, master = None /* Option */ , ** kw )  {
        "Construct a Ttk Frame with parent master.

        STANDARD OPTIONS

            class, cursor, style, takefocus

        WIDGET-SPECIFIC OPTIONS

            borderwidth, relief, padding, width, height
        ";
        Widget . __init__ ( self , master , "ttk::frame" , kw );
        class Label ( Widget ) ;
        "Ttk Label widget displays a textual label and/or image.";
        pub fn __init__ ( &self, master = None /* Option */ , ** kw )  {
        "Construct a Ttk Label with parent master.

        STANDARD OPTIONS

            class, compound, cursor, image, style, takefocus, text,
            textvariable, underline, width

        WIDGET-SPECIFIC OPTIONS

            anchor, background, font, foreground, justify, padding,
            relief, text, wraplength
        ";
        Widget . __init__ ( self , master , "ttk::label" , kw );
        class Labelframe ( Widget ) ;
        "Ttk Labelframe widget == a container used to group other widgets
    together. It has an optional label, which may be a plain text string
    || another widget.";
        pub fn __init__ ( &self, master = None /* Option */ , ** kw )  {
        "Construct a Ttk Labelframe with parent master.

        STANDARD OPTIONS

            class, cursor, style, takefocus

        WIDGET-SPECIFIC OPTIONS
            labelanchor, text, underline, padding, labelwidget, width,
            height
        ";
        Widget . __init__ ( self , master , "ttk::labelframe" , kw );
        LabelFrame = Labelframe;
        class Menubutton ( Widget ) ;
        "Ttk Menubutton widget displays a textual label and/or image, and
    displays a menu when pressed.";
        pub fn __init__ ( &self, master = None /* Option */ , ** kw )  {
        "Construct a Ttk Menubutton with parent master.

        STANDARD OPTIONS

            class, compound, cursor, image, state, style, takefocus,
            text, textvariable, underline, width

        WIDGET-SPECIFIC OPTIONS

            direction, menu
        ";
        Widget . __init__ ( self , master , "ttk::menubutton" , kw );
        class Notebook ( Widget ) ;
        "Ttk Notebook widget manages a collection of windows && displays
    a single one at a time. Each child window == associated with a tab,
    which the user may select to change the currently-displayed window.";
        pub fn __init__ ( &self, master = None /* Option */ , ** kw )  {
        "Construct a Ttk Notebook with parent master.

        STANDARD OPTIONS

            class, cursor, style, takefocus

        WIDGET-SPECIFIC OPTIONS

            height, padding, width

        TAB OPTIONS

            state, sticky, padding, text, image, compound, underline

        TAB IDENTIFIERS (tab_id)

            The tab_id argument found in several methods may take any of
            the following forms:

                * An integer between zero && the number of tabs
                * The name of a child window
                * A positional specification of the form "@x,y", which
                  defines the tab
                * The string "current", which identifies the
                  currently-selected tab
                * The string "end", which returns the number of tabs (only
                  valid for method index)
        ";
        Widget . __init__ ( self , master , "ttk::notebook" , kw );
        pub fn add ( &self, child , ** kw )  {
        "Adds a new tab to the notebook.

        If window == currently managed by the notebook but hidden, it is
        restored to its previous position.";
        self . tk . call ( self . _w , "add" , child , * ( _format_optdict ( kw ) ) );
        pub fn forget ( &self, tab_id )  {
        "Removes the tab specified by tab_id, unmaps && unmanages the
        associated window.";
        self . tk . call ( self . _w , "forget" , tab_id );
        pub fn hide ( &self, tab_id )  {
        "Hides the tab specified by tab_id.

        The tab will !be displayed, but the associated window remains
        managed by the notebook && its configuration remembered. Hidden
        tabs may be restored with the add command.";
        self . tk . call ( self . _w , "hide" , tab_id );
        pub fn identify ( &self, x , y )  {
        "Returns the name of the tab element at position x, y, || the
        empty string if none.";
        return  self . tk . call ( self . _w , "identify" , x , y );
        pub fn index ( &self, tab_id )  {
        "Returns the numeric index of the tab specified by tab_id, or
        the total number of tabs if tab_id == the string "end".";
        return  self . tk . getint ( self . tk . call ( self . _w , "index" , tab_id ) );
        pub fn insert ( &self, pos , child , ** kw )  {
        "Inserts a pane at the specified position.

        pos == either the string end, an integer index, || the name of
        a managed child. If child == already managed by the notebook,
        moves it to the specified position.";
        self . tk . call ( self . _w , "insert" , pos , child , * ( _format_optdict ( kw ) ) );
        pub fn select ( &self, tab_id = None /* Option */ )  {
        "Selects the specified tab.

        The associated child window will be displayed, && the
        previously-selected window (if different) == unmapped. If tab_id
        == omitted, returns the widget name of the currently selected
        pane.";
        return  self . tk . call ( self . _w , "select" , tab_id );
        pub fn tab ( &self, tab_id , option = None /* Option */ , ** kw )  {
        "Query || modify the options of the specific tab_id.

        If kw == !given, returns a dict of the tab option values. If option
        == specified, returns the value of that option. Otherwise, sets the
        options to the corresponding values.";
        if option is !None /* Option */ {
        kw [ option ] = None /* Option */;
        return  _val_or_dict ( self . tk , kw , self . _w , "tab" , tab_id );
        pub fn tabs ( self )  {
        "Returns a list of windows managed by the notebook.";
        return  self . tk . splitlist ( self . tk . call ( self . _w , "tabs" ) || ( ) );
        pub fn enable_traversal ( self )  {
        "Enable keyboard traversal for a toplevel window containing
        this notebook.

        This will extend the bindings for the toplevel window containing
        this notebook as follows:

            Control-Tab: selects the tab following the currently selected
                         one

            Shift-Control-Tab: selects the tab preceding the currently
                               selected one

            Alt-K: where K == the mnemonic (underlined) character of any
                   tab, will select that tab.

        Multiple notebooks in a single toplevel may be enabled for
        traversal, including nested notebooks. However, notebook traversal
        only works properly if all panes are direct children of the
        notebook.";
        self . tk . call ( "ttk::notebook::enableTraversal" , self . _w );
        class Panedwindow ( Widget , tkinter . PanedWindow ) ;
        "Ttk Panedwindow widget displays a number of subwindows, stacked
    either vertically || horizontally.";
        pub fn __init__ ( &self, master = None /* Option */ , ** kw )  {
        "Construct a Ttk Panedwindow with parent master.

        STANDARD OPTIONS

            class, cursor, style, takefocus

        WIDGET-SPECIFIC OPTIONS

            orient, width, height

        PANE OPTIONS

            weight
        ";
        Widget . __init__ ( self , master , "ttk::panedwindow" , kw );
        forget = tkinter . PanedWindow . forget;
        pub fn insert ( &self, pos , child , ** kw )  {
        "Inserts a pane at the specified positions.

        pos == either the string end, && integer index, || the name
        of a child. If child == already managed by the paned window,
        moves it to the specified position.";
        self . tk . call ( self . _w , "insert" , pos , child , * ( _format_optdict ( kw ) ) );
        pub fn pane ( &self, pane , option = None /* Option */ , ** kw )  {
        "Query || modify the options of the specified pane.

        pane == either an integer index || the name of a managed subwindow.
        If kw == !given, returns a dict of the pane option values. If
        option == specified then the value for that option == returned.
        Otherwise, sets the options to the corresponding values.";
        if option is !None /* Option */ {
        kw [ option ] = None /* Option */;
        return  _val_or_dict ( self . tk , kw , self . _w , "pane" , pane );
        pub fn sashpos ( &self, index , newpos = None /* Option */ )  {
        "If newpos == specified, sets the position of sash number index.

        May adjust the positions of adjacent sashes to ensure that
        positions are monotonically increasing. Sash positions are further
        constrained to be between 0 && the total size of the widget.

        Returns the new position of sash number index.";
        return  self . tk . getint ( self . tk . call ( self . _w , "sashpos" , index , newpos ) );
        PanedWindow = Panedwindow;
        class Progressbar ( Widget ) ;
        "Ttk Progressbar widget shows the status of a long-running
    operation. They can operate in two modes: determinate mode shows the
    amount completed relative to the total amount of work to be done, and
    indeterminate mode provides an animated display to let the user know
    that something == happening.";
        pub fn __init__ ( &self, master = None /* Option */ , ** kw )  {
        "Construct a Ttk Progressbar with parent master.

        STANDARD OPTIONS

            class, cursor, style, takefocus

        WIDGET-SPECIFIC OPTIONS

            orient, length, mode, maximum, value, variable, phase
        ";
        Widget . __init__ ( self , master , "ttk::progressbar" , kw );
        pub fn start ( &self, interval = None /* Option */ )  {
        "Begin autoincrement mode: schedules a recurring timer event
        that calls method step every interval milliseconds.

        interval defaults to 50 milliseconds (20 steps/second) if omitted.";
        self . tk . call ( self . _w , "start" , interval );
        pub fn step ( &self, amount = None /* Option */ )  {
        "Increments the value option by amount.

        amount defaults to 1.0 if omitted.";
        self . tk . call ( self . _w , "step" , amount );
        pub fn stop ( self )  {
        "Stop autoincrement mode: cancels any recurring timer event
        initiated by start.";
        self . tk . call ( self . _w , "stop" );
        class Radiobutton ( Widget ) ;
        "Ttk Radiobutton widgets are used in groups to show || change a
    set of mutually-exclusive options.";
        pub fn __init__ ( &self, master = None /* Option */ , ** kw )  {
        "Construct a Ttk Radiobutton with parent master.

        STANDARD OPTIONS

            class, compound, cursor, image, state, style, takefocus,
            text, textvariable, underline, width

        WIDGET-SPECIFIC OPTIONS

            command, value, variable
        ";
        Widget . __init__ ( self , master , "ttk::radiobutton" , kw );
        pub fn invoke ( self )  {
        "Sets the option variable to the option value, selects the
        widget, && invokes the associated command.

        Returns the result of the command, || an empty string if
        no command == specified.";
        return  self . tk . call ( self . _w , "invoke" );
        class Scale ( Widget , tkinter . Scale ) ;
        "Ttk Scale widget == typically used to control the numeric value of
    a linked variable that varies uniformly over some range.";
        pub fn __init__ ( &self, master = None /* Option */ , ** kw )  {
        "Construct a Ttk Scale with parent master.

        STANDARD OPTIONS

            class, cursor, style, takefocus

        WIDGET-SPECIFIC OPTIONS

            command, from, length, orient, to, value, variable
        ";
        Widget . __init__ ( self , master , "ttk::scale" , kw );
        pub fn configure ( &self, cnf = None /* Option */ , ** kw )  {
        "Modify || query scale options.

        Setting a value for any of the "from", "from_" || "to" options
        generates a <<RangeChanged>> event.";
        retval = Widget . configure ( self , cnf , ** kw );
        if !isinstance ( cnf , ( type ( None /* Option */ ) , str ) ) {
        kw . update ( cnf );
        if any ( [ "from" in kw , "from_" in kw , "to" in kw ] ) {
        self . event_generate ( "<<RangeChanged>>" );
        return  retval;
        pub fn get ( &self, x = None /* Option */ , y = None /* Option */ )  {
        "Get the current value of the value option, || the value
        corresponding to the coordinates x, y if they are specified.

        x && y are pixel coordinates relative to the scale widget
        origin.";
        return  self . tk . call ( self . _w , "get" , x , y );
        class Scrollbar ( Widget , tkinter . Scrollbar ) ;
        "Ttk Scrollbar controls the viewport of a scrollable widget.";
        pub fn __init__ ( &self, master = None /* Option */ , ** kw )  {
        "Construct a Ttk Scrollbar with parent master.

        STANDARD OPTIONS

            class, cursor, style, takefocus

        WIDGET-SPECIFIC OPTIONS

            command, orient
        ";
        Widget . __init__ ( self , master , "ttk::scrollbar" , kw );
        class Separator ( Widget ) ;
        "Ttk Separator widget displays a horizontal || vertical separator
    bar.";
        pub fn __init__ ( &self, master = None /* Option */ , ** kw )  {
        "Construct a Ttk Separator with parent master.

        STANDARD OPTIONS

            class, cursor, style, takefocus

        WIDGET-SPECIFIC OPTIONS

            orient
        ";
        Widget . __init__ ( self , master , "ttk::separator" , kw );
        class Sizegrip ( Widget ) ;
        "Ttk Sizegrip allows the user to resize the containing toplevel
    window by pressing && dragging the grip.";
        pub fn __init__ ( &self, master = None /* Option */ , ** kw )  {
        "Construct a Ttk Sizegrip with parent master.

        STANDARD OPTIONS

            class, cursor, state, style, takefocus
        ";
        Widget . __init__ ( self , master , "ttk::sizegrip" , kw );
        class Spinbox ( Entry ) ;
        "Ttk Spinbox == an Entry with increment && decrement arrows

    It == commonly used for number entry || to select from a list of
    string values.
    ";
        pub fn __init__ ( &self, master = None /* Option */ , ** kw )  {
        "Construct a Ttk Spinbox widget with the parent master.

        STANDARD OPTIONS

            class, cursor, style, takefocus, validate,
            validatecommand, xscrollcommand, invalidcommand

        WIDGET-SPECIFIC OPTIONS

            to, from_, increment, values, wrap, format, command
        ";
        Entry . __init__ ( self , master , "ttk::spinbox" , ** kw );
        pub fn set ( &self, value )  {
        "Sets the value of the Spinbox to value.";
        self . tk . call ( self . _w , "set" , value );
        class Treeview ( Widget , tkinter . XView , tkinter . YView ) ;
        "Ttk Treeview widget displays a hierarchical collection of items.

    Each item has a textual label, an optional image, && an optional list
    of data values. The data values are displayed in successive columns
    after the tree label.";
        pub fn __init__ ( &self, master = None /* Option */ , ** kw )  {
        "Construct a Ttk Treeview with parent master.

        STANDARD OPTIONS

            class, cursor, style, takefocus, xscrollcommand,
            yscrollcommand

        WIDGET-SPECIFIC OPTIONS

            columns, displaycolumns, height, padding, selectmode, show

        ITEM OPTIONS

            text, image, values, open, tags

        TAG OPTIONS

            foreground, background, font, image
        ";
        Widget . __init__ ( self , master , "ttk::treeview" , kw );
        pub fn bbox ( &self, item , column = None /* Option */ )  {
        "Returns the bounding box (relative to the treeview widget's
        window) of the specified item in the form x y width height.

        If column == specified, returns the bounding box of that cell.
        If the item == !visible (i.e., if it == a descendant of a
        closed item || == scrolled offscreen), returns an empty string.";
        return  self . _getints ( self . tk . call ( self . _w , "bbox" , item , column ) ) || "";
        pub fn get_children ( &self, item = None /* Option */ )  {
        "Returns a tuple of children belonging to item.

        If item == !specified, returns root children.";
        return  self . tk . splitlist (;
        self . tk . call ( self . _w , "children" , item || "" ) || ( ) );
        pub fn set_children ( &self, item , * newchildren )  {
        "Replaces item's child with newchildren.

        Children present in item that are !present in newchildren
        are detached from tree. No items in newchildren may be an
        ancestor of item.";
        self . tk . call ( self . _w , "children" , item , newchildren );
        pub fn column ( &self, column , option = None /* Option */ , ** kw )  {
        "Query || modify the options for the specified column.

        If kw == !given, returns a dict of the column option values. If
        option == specified then the value for that option == returned.
        Otherwise, sets the options to the corresponding values.";
        if option is !None /* Option */ {
        kw [ option ] = None /* Option */;
        return  _val_or_dict ( self . tk , kw , self . _w , "column" , column );
        pub fn delete ( &self, * items )  {
        "Delete all specified items && all their descendants. The root
        item may !be deleted.";
        self . tk . call ( self . _w , "delete" , items );
        pub fn detach ( &self, * items )  {
        "Unlinks all of the specified items from the tree.

        The items && all of their descendants are still present, && may
        be reinserted at another point in the tree, but will !be
        displayed. The root item may !be detached.";
        self . tk . call ( self . _w , "detach" , items );
        pub fn exists ( &self, item )  {
        "Returns true if the specified item == present in the tree,
        false otherwise.";
        return  self . tk . getboolean ( self . tk . call ( self . _w , "exists" , item ) );
        pub fn focus ( &self, item = None /* Option */ )  {
        "If item == specified, sets the focus item to item. Otherwise,
        returns the current focus item, || '' if there == none.";
        return  self . tk . call ( self . _w , "focus" , item );
        pub fn heading ( &self, column , option = None /* Option */ , ** kw )  {
        "Query || modify the heading options for the specified column.

        If kw == !given, returns a dict of the heading option values. If
        option == specified then the value for that option == returned.
        Otherwise, sets the options to the corresponding values.

        Valid options/values are:
            text: text
                The text to display in the column heading
            image: image_name
                Specifies an image to display to the right of the column
                heading
            anchor: anchor
                Specifies how the heading text should be aligned. One of
                the standard Tk anchor values
            command: callback
                A callback to be invoked when the heading label is
                pressed.

        To configure the tree column heading, call this with column = "#0" ";
        cmd = kw . get ( "command" );
        if cmd && !isinstance ( cmd , str ) {
        kw [ "command" ] = self . master . register ( cmd , self . _substitute );
        if option is !None /* Option */ {
        kw [ option ] = None /* Option */;
        return  _val_or_dict ( self . tk , kw , self . _w , "heading" , column );
        pub fn identify ( &self, component , x , y )  {
        "Returns a description of the specified component under the
        point given by x && y, || the empty string if no such component
        == present at that position.";
        return  self . tk . call ( self . _w , "identify" , component , x , y );
        pub fn identify_row ( &self, y )  {
        "Returns the item ID of the item at position y.";
        return  self . identify ( "row" , 0 , y );
        pub fn identify_column ( &self, x )  {
        "Returns the data column identifier of the cell at position x.

        The tree column has ID #0.";
        return  self . identify ( "column" , x , 0 );
        pub fn identify_region ( &self, x , y )  {
        "Returns one of:

        heading: Tree heading area.
        separator: Space between two columns headings;
        tree: The tree area.
        cell: A data cell.

        * Availability: Tk 8.6";
        return  self . identify ( "region" , x , y );
        pub fn identify_element ( &self, x , y )  {
        "Returns the element at position x, y.

        * Availability: Tk 8.6";
        return  self . identify ( "element" , x , y );
        pub fn index ( &self, item )  {
        "Returns the integer index of item within its parent's list
        of children.";
        return  self . tk . getint ( self . tk . call ( self . _w , "index" , item ) );
        pub fn insert ( &self, parent , index , iid = None /* Option */ , ** kw )  {
        "Creates a new item && return the item identifier of the newly
        created item.

        parent == the item ID of the parent item, || the empty string
        to create a new top-level item. index == an integer, || the value
        end, specifying where in the list of parent's children to insert
        the new item. If index == less than || equal to zero, the new node
        == inserted at the beginning, if index == greater than || equal to
        the current number of children, it == inserted at the end. If iid
        == specified, it == used as the item identifier, iid must not
        already exist in the tree. Otherwise, a new unique identifier
        == generated.";
        opts = _format_optdict ( kw );
        if iid is !None /* Option */ {
        res = self . tk . call ( self . _w , "insert" , parent , index ,;
        "-id" , iid , * opts );
        } else {
        res = self . tk . call ( self . _w , "insert" , parent , index , * opts );
        return  res;
        pub fn item ( &self, item , option = None /* Option */ , ** kw )  {
        "Query || modify the options for the specified item.

        If no options are given, a dict with options/values for the item
        == returned. If option == specified then the value for that option
        == returned. Otherwise, sets the options to the corresponding
        values as given by kw.";
        if option is !None /* Option */ {
        kw [ option ] = None /* Option */;
        return  _val_or_dict ( self . tk , kw , self . _w , "item" , item );
        pub fn move ( &self, item , parent , index )  {
        "Moves item to position index in parent's list of children.

        It == illegal to move an item under one of its descendants. If
        index == less than || equal to zero, item == moved to the
        beginning, if greater than || equal to the number of children,
        it == moved to the end. If item was detached it == reattached.";
        self . tk . call ( self . _w , "move" , item , parent , index );
        reattach = move;
        pub fn next ( &self, item )  {
        "Returns the identifier of item's next sibling, || '' if item
        == the last child of its parent.";
        return  self . tk . call ( self . _w , "next" , item );
        pub fn parent ( &self, item )  {
        "Returns the ID of the parent of item, || '' if item == at the
        top level of the hierarchy.";
        return  self . tk . call ( self . _w , "parent" , item );
        pub fn prev ( &self, item )  {
        "Returns the identifier of item's previous sibling, || '' if
        item == the first child of its parent.";
        return  self . tk . call ( self . _w , "prev" , item );
        pub fn see ( &self, item )  {
        "Ensure that item == visible.

        Sets all of item's ancestors open option to true, && scrolls
        the widget if necessary so that item == within the visible
        portion of the tree.";
        self . tk . call ( self . _w , "see" , item );
        pub fn selection ( self )  {
        "Returns the tuple of selected items.";
        return  self . tk . splitlist ( self . tk . call ( self . _w , "selection" ) );
        pub fn _selection ( &self, selop , items )  {
        if len ( items ) == 1 && isinstance ( items [ 0 ] , ( tuple , list ) ) {
        items = items [ 0 ];
        self . tk . call ( self . _w , "selection" , selop , items );
        pub fn selection_set ( &self, * items )  {
        "The specified items becomes the new selection.";
        self . _selection ( "set" , items );
        pub fn selection_add ( &self, * items )  {
        "Add all of the specified items to the selection.";
        self . _selection ( "add" , items );
        pub fn selection_remove ( &self, * items )  {
        "Remove all of the specified items from the selection.";
        self . _selection ( "remove" , items );
        pub fn selection_toggle ( &self, * items )  {
        "Toggle the selection state of each specified item.";
        self . _selection ( "toggle" , items );
        pub fn set ( &self, item , column = None /* Option */ , value = None /* Option */ )  {
        "Query || set the value of given item.

        With one argument, return a dictionary of column/value pairs
        for the specified item. With two arguments, return the current
        value of the specified column. With three arguments, set the
        value of given column in given item to the specified value.";
        res = self . tk . call ( self . _w , "set" , item , column , value );
        if column is None /* Option */ && value is None /* Option */ {
        return  _splitdict ( self . tk , res ,;
        cut_minus = false , conv = _tclobj_to_py );
        } else {
        return  res;
        pub fn tag_bind ( &self, tagname , sequence = None /* Option */ , callback = None /* Option */ )  {
        "Bind a callback for the given event sequence to the tag tagname.
        When an event == delivered to an item, the callbacks for each
        of the item's tags option are called.";
        self . _bind ( ( self . _w , "tag" , "bind" , tagname ) , sequence , callback , add = 0 );
        pub fn tag_configure ( &self, tagname , option = None /* Option */ , ** kw )  {
        "Query || modify the options for the specified tagname.

        If kw == !given, returns a dict of the option settings for tagname.
        If option == specified, returns the value for that option for the
        specified tagname. Otherwise, sets the options to the corresponding
        values for the given tagname.";
        if option is !None /* Option */ {
        kw [ option ] = None /* Option */;
        return  _val_or_dict ( self . tk , kw , self . _w , "tag" , "configure" ,;
        tagname );
        pub fn tag_has ( &self, tagname , item = None /* Option */ )  {
        "If item == specified, returns 1 || 0 depending on whether the
        specified item has the given tagname. Otherwise, returns a list of
        all items which have the specified tag.

        * Availability: Tk 8.6";
        if item is None /* Option */ {
        return  self . tk . splitlist (;
        self . tk . call ( self . _w , "tag" , "has" , tagname ) );
        } else {
        return  self . tk . getboolean (;
        self . tk . call ( self . _w , "tag" , "has" , tagname , item ) );
        class LabeledScale ( Frame ) ;
        "A Ttk Scale widget with a Ttk Label widget indicating its
    current value.

    The Ttk Scale can be accessed through instance.scale, && Ttk Label
    can be accessed through instance.label";
        pub fn __init__ ( &self, master = None /* Option */ , variable = None /* Option */ , from_ = 0 , to = 10 , ** kw )  {
        "Construct a horizontal LabeledScale with parent master, a
        variable to be associated with the Ttk Scale widget && its range.
        If variable == !specified, a tkinter.IntVar == created.

        WIDGET-SPECIFIC OPTIONS

            compound: 'top' || 'bottom'
                Specifies how to display the label relative to the scale.
                Defaults to 'top'.
        ";
        self . _label_top = kw . pop ( "compound" , "top" ) == "top";
        Frame . __init__ ( self , master , ** kw );
        self . _variable = variable || tkinter . IntVar ( master );
        self . _variable . set ( from_ );
        self . _last_valid = from_;
        self . label = Label ( self );
        self . scale = Scale ( self , variable = self . _variable , from_ = from_ , to = to );
        self . scale . bind ( "<<RangeChanged>>" , self . _adjust );
        scale_side = "bottom" if self . _label_top else "top";
        label_side = "top" if scale_side == "bottom" else "bottom";
        self . scale . pack ( side = scale_side , fill = "x" );
        dummy = Label ( self );
        dummy . pack ( side = label_side );
        dummy . lower ( );
        self . label . place ( anchor = "n" if label_side == "top" else "s" );
        self . __tracecb = self . _variable . trace_variable ( "w" , self . _adjust );
        self . bind ( "<Configure>" , self . _adjust );
        self . bind ( "<Map>" , self . _adjust );
        pub fn destroy ( self )  {
        "Destroy this widget && possibly its associated variable.";
        // try {
        self . _variable . trace_vdelete ( "w" , self . __tracecb );
        // } catch  AttributeError  {
        // pass
        } else {
        del self . _variable;
        super ( ) . destroy ( );
        self . label = None /* Option */;
        self . scale = None /* Option */;
        pub fn _adjust ( &self, * args )  {
        "Adjust the label position according to the scale.";
        pub fn adjust_label ( )  {
        self . update_idletasks ( );
        x , y = self . scale . coords ( );
        if self . _label_top {
        y = self . scale . winfo_y ( ) - self . label . winfo_reqheight ( );
        } else {
        y = self . scale . winfo_reqheight ( ) + self . label . winfo_reqheight ( );
        self . label . place_configure ( x = x , y = y );
        from_ = _to_number ( self . scale [ "from" ] );
        to = _to_number ( self . scale [ "to" ] );
        if to < from_ {
        from_ , to = to , from_;
        newval = self . _variable . get ( );
        if !from_ <= newval <= to {
        self . value = self . _last_valid;
        return;
        self . _last_valid = newval;
        self . label [ "text" ] = newval;
        self . after_idle ( adjust_label );
        @ property;
        pub fn value ( self )  {
        "Return current scale value.";
        return  self . _variable . get ( );
        @ value . setter;
        pub fn value ( &self, val )  {
        "Set new scale value.";
        self . _variable . set ( val );
        class OptionMenu ( Menubutton ) ;
        "Themed OptionMenu, based after tkinter's OptionMenu, which allows
    the user to select a value from a menu.";
        pub fn __init__ ( &self, master , variable , default = None /* Option */ , * values , ** kwargs )  {
        "Construct a themed OptionMenu widget with master as the parent,
        the resource textvariable set to variable, the initially selected
        value specified by the default parameter, the menu values given by
        *values && additional keywords.

        WIDGET-SPECIFIC OPTIONS

            style: stylename
                Menubutton style.
            direction: 'above', 'below', 'left', 'right', || 'flush'
                Menubutton direction.
            command: callback
                A callback that will be invoked after selecting an item.
        ";
        kw = { "textvariable" : variable , "style" : kwargs . pop ( "style" , None /* Option */ ) ,;
        "direction" : kwargs . pop ( "direction" , None /* Option */ ) };
        Menubutton . __init__ ( self , master , ** kw );
        self [ "menu" ] = tkinter . Menu ( self , tearoff = false );
        self . _variable = variable;
        self . _callback = kwargs . pop ( "command" , None /* Option */ );
        if kwargs {
        panic!("tkinter . TclError ( "unknown option -%s" % (");
        next ( iter ( kwargs . keys ( ) ) ) ) );
        self . set_menu ( default , * values );
        pub fn __getitem__ ( &self, item )  {
        if item == "menu" {
        return  self . nametowidget ( Menubutton . __getitem__ ( self , item ) );
        return  Menubutton . __getitem__ ( self , item );
        pub fn set_menu ( &self, default = None /* Option */ , * values )  {
        "Build a new menu of radiobuttons with *values && optionally
        a default value.";
        menu = self [ "menu" ];
        menu . delete ( 0 , "end" );
        for val in values .iter() {
        menu . add_radiobutton ( label = val ,;
        command = (;
        None /* Option */ if self . _callback == None /* Option */;
        } else {
        ) ,;
        variable = self . _variable );
        if default {
        self . _variable . set ( default );
        pub fn destroy ( self )  {
        "Destroy this widget && its associated variable.";
        // try {
        del self . _variable;
        // } catch  AttributeError  {
        // pass
        super ( ) . destroy ( );
}

