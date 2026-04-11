//! tix.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::tkinter;
// use crate::_cnfmerge;

pub const WINDOW: &str = "window";
pub const TEXT: &str = "text";
pub const STATUS: &str = "status";
pub const IMMEDIATE: &str = "immediate";
pub const IMAGE: &str = "image";
pub const IMAGETEXT: &str = "imagetext";
pub const BALLOON: &str = "balloon";
pub const AUTO: &str = "auto";
pub const ACROSSTOP: &str = "acrosstop";
pub const ASCII: &str = "ascii";
pub const CELL: &str = "cell";
pub const COLUMN: &str = "column";
pub const DECREASING: &str = "decreasing";
pub const INCREASING: &str = "increasing";
pub const INTEGER: &str = "integer";
pub const MAIN: &str = "main";
pub const MAX: &str = "max";
pub const REAL: &str = "real";
pub const ROW: &str = "row";
pub const S_REGION: &str = "s-region";
pub const X_REGION: &str = "x-region";
pub const Y_REGION: &str = "y-region";
pub const TCL_DONT_WAIT: u64 = 1 < < 1;
pub const TCL_WINDOW_EVENTS: u64 = 1 < < 2;
pub const TCL_FILE_EVENTS: u64 = 1 < < 3;
pub const TCL_TIMER_EVENTS: u64 = 1 < < 4;
pub const TCL_IDLE_EVENTS: u64 = 1 < < 5;
pub const TCL_ALL_EVENTS: u64 = 0;
pub struct tixCommand {
    pub widgetName: String, // TODO: infer type
    pub subwidget_list: String, // TODO: infer type
    pub destroy_physically: String, // TODO: infer type
    pub tk: String, // TODO: infer type
    pub stylename: String, // TODO: infer type
    pub cnf: String, // TODO: infer type
}

impl tixCommand {
}

pub struct Tk {
    pub widgetName: String, // TODO: infer type
    pub subwidget_list: String, // TODO: infer type
    pub destroy_physically: String, // TODO: infer type
    pub tk: String, // TODO: infer type
    pub stylename: String, // TODO: infer type
    pub cnf: String, // TODO: infer type
}

impl Tk {
    pub fn new(screenName: &str, baseName: &str, className: &str) -> Self {
        tkinter . Tk . __init__ ( self , screenName , baseName , className );
        tixlib = os . environ . get ( "TIX_LIBRARY" );
        self . tk . eval ( "global auto_path; lappend auto_path [file dir [info nameof]]" );
        if tixlib is !None /* Option */ {
        self . tk . eval ( "global auto_path; lappend auto_path {%s}" % tixlib );
        self . tk . eval ( "global tcl_pkgPath; lappend tcl_pkgPath {%s}" % tixlib );
        self . tk . eval ( "package require Tix" );
    }

    pub fn OptionName(&self, widget: &str) {
        "Returns the qualified path name for the widget. Normally used to set
    default options for subwidgets. See tixwidgets.py";
        return  widget . tk . call ( "tixOptionName" , widget . _w );
        pub fn FileTypeList ( dict )  {
        s = "";
        for type in dict . keys ( ) .iter() {
        s = s + "{{" + type + "} {" + type + " - " + dict [ type ] + "}} ";
        return  s;
        class CObjView ( TixWidget ) ;
        "This file implements the Canvas Object View widget. This == a base
    class of IconView. It implements automatic placement/adjustment of the
    scrollbars according to the canvas objects inside the canvas subwidget.
    The scrollbars are adjusted so that the canvas == just large enough
    to see all the objects.
    ";
        // pass
        class Grid ( TixWidget , XView , YView ) ;
        "The Tix Grid command creates a new window  && makes it into a
    tixGrid widget. Additional options, may be specified on the command
    line || in the option database to configure aspects such as its cursor
    && relief.

    A Grid widget displays its contents in a two dimensional grid of cells.
    Each cell may contain one Tix display item, which may be in text,
    graphics || other formats. See the DisplayStyle class for more information
    about Tix display items. Individual cells, || groups of cells, can be
    formatted with a wide range of attributes, such as its color, relief and
    border.

    Subwidgets - None /* Option */";
        pub fn __init__ ( &self, master = None /* Option */ , cnf = { } , ** kw )  {
        static = [ ];
        self . cnf = cnf;
        TixWidget . __init__ ( self , master , "tixGrid" , static , cnf , kw );
        pub fn anchor_clear ( self )  {
        "Removes the selection anchor.";
        self . tk . call ( self , "anchor" , "clear" );
        pub fn anchor_get ( self )  {
        "Get the (x,y) coordinate of the current anchor cell";
        return  self . _getints ( self . tk . call ( self , "anchor" , "get" ) );
        pub fn anchor_set ( &self, x , y )  {
        "Set the selection anchor to the cell at (x, y).";
        self . tk . call ( self , "anchor" , "set" , x , y );
        pub fn delete_row ( &self, from_ , to = None /* Option */ )  {
        "Delete rows between from_ && to inclusive.
        If to == !provided,  delete only row at from_";
        if to is None /* Option */ {
        self . tk . call ( self , "delete" , "row" , from_ );
        } else {
        self . tk . call ( self , "delete" , "row" , from_ , to );
        pub fn delete_column ( &self, from_ , to = None /* Option */ )  {
        "Delete columns between from_ && to inclusive.
        If to == !provided,  delete only column at from_";
        if to is None /* Option */ {
        self . tk . call ( self , "delete" , "column" , from_ );
        } else {
        self . tk . call ( self , "delete" , "column" , from_ , to );
        pub fn edit_apply ( self )  {
        "If any cell == being edited, de-highlight the cell  &&  applies
        the changes.";
        self . tk . call ( self , "edit" , "apply" );
        pub fn edit_set ( &self, x , y )  {
        "Highlights  the  cell  at  (x, y) for editing, if the -editnotify
        command returns true for this cell.";
        self . tk . call ( self , "edit" , "set" , x , y );
        pub fn entrycget ( &self, x , y , option )  {
        "Get the option value for cell at (x,y)";
        if option && option [ 0 ] != "-" {
        option = "-" + option;
        return  self . tk . call ( self , "entrycget" , x , y , option );
        pub fn entryconfigure ( &self, x , y , cnf = None /* Option */ , ** kw )  {
        return  self . _configure ( ( "entryconfigure" , x , y ) , cnf , kw );
        pub fn info_exists ( &self, x , y )  {
        "Return true if display item exists at (x,y)";
        return  self . _getboolean ( self . tk . call ( self , "info" , "exists" , x , y ) );
        pub fn info_bbox ( &self, x , y )  {
        return  self . tk . call ( self , "info" , "bbox" , x , y );
        pub fn move_column ( &self, from_ , to , offset )  {
        "Moves the range of columns from position FROM through TO by
        the distance indicated by OFFSET. For example, move_column(2, 4, 1)
        moves the columns 2,3,4 to columns 3,4,5.";
        self . tk . call ( self , "move" , "column" , from_ , to , offset );
        pub fn move_row ( &self, from_ , to , offset )  {
        "Moves the range of rows from position FROM through TO by
        the distance indicated by OFFSET.
        For example, move_row(2, 4, 1) moves the rows 2,3,4 to rows 3,4,5.";
        self . tk . call ( self , "move" , "row" , from_ , to , offset );
        pub fn nearest ( &self, x , y )  {
        "Return coordinate of cell nearest pixel coordinate (x,y)";
        return  self . _getints ( self . tk . call ( self , "nearest" , x , y ) );
        pub fn set ( &self, x , y , itemtype = None /* Option */ , ** kw )  {
        args = self . _options ( self . cnf , kw );
        if itemtype is !None /* Option */ {
        args = ( "-itemtype" , itemtype ) + args;
        self . tk . call ( self , "set" , x , y , * args );
        pub fn size_column ( &self, index , ** kw )  {
        "Queries || sets the size of the column given by
        INDEX.  INDEX may be any non-negative
        integer that gives the position of a given column.
        INDEX can also be the string "default"; in this case, this command
        queries || sets the default size of all columns.
        When no option-value pair == given, this command returns a tuple
        containing the current size setting of the given column.  When
        option-value pairs are given, the corresponding options of the
        size setting of the given column are changed. Options may be one
        of the following:
              pad0 pixels
                     Specifies the paddings to the left of a column.
              pad1 pixels
                     Specifies the paddings to the right of a column.
              size val
                     Specifies the width of a column.  Val may be:
                     "auto" -- the width of the column == set to the
                     width of the widest cell in the column;
                     a valid Tk screen distance unit;
                     || a real number following by the word chars
                     (e.g. 3.4chars) that sets the width of the column to the
                     given number of characters.";
        return  self . tk . splitlist ( self . tk . call ( self . _w , "size" , "column" , index ,;
        * self . _options ( { } , kw ) ) );
        pub fn size_row ( &self, index , ** kw )  {
        "Queries || sets the size of the row given by
        INDEX. INDEX may be any non-negative
        integer that gives the position of a given row .
        INDEX can also be the string "default"; in this case, this command
        queries || sets the default size of all rows.
        When no option-value pair == given, this command returns a list con-
        taining the current size setting of the given row . When option-value
        pairs are given, the corresponding options of the size setting of the
        given row are changed. Options may be one of the following:
              pad0 pixels
                     Specifies the paddings to the top of a row.
              pad1 pixels
                     Specifies the paddings to the bottom of a row.
              size val
                     Specifies the height of a row.  Val may be:
                     "auto" -- the height of the row == set to the
                     height of the highest cell in the row;
                     a valid Tk screen distance unit;
                     || a real number following by the word chars
                     (e.g. 3.4chars) that sets the height of the row to the
                     given number of characters.";
        return  self . tk . splitlist ( self . tk . call (;
        self , "size" , "row" , index , * self . _options ( { } , kw ) ) );
        pub fn unset ( &self, x , y )  {
        "Clears the cell at (x, y) by removing its display item.";
        self . tk . call ( self . _w , "unset" , x , y );
        class ScrolledGrid ( Grid ) ;
        "Scrolled Grid widgets";
        pub fn __init__ ( &self, master = None /* Option */ , cnf = { } , ** kw )  {
        static = [ ];
        self . cnf = cnf;
        TixWidget . __init__ ( self , master , "tixScrolledGrid" , static , cnf , kw );
    }

}

