//! __init__.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::_msi::{};
// use crate::fnmatch;
// use regex::Regex;
// use std::env;
// use crate::tempfile::{mktemp};

pub const remove: f64 = ( 3 , 13 ) );
pub const AMD64: &str = "AMD64" in sys . version;
pub const Win64: /* inferred */ = AMD64;
pub const datasizemask: u64 = 0x00 ff;
pub const type_valid: u64 = 0x0100;
pub const type_localizable: u64 = 0x0200;
pub const typemask: u64 = 0x0 c00;
pub const type_long: u64 = 0x0000;
pub const type_short: u64 = 0x0400;
pub const type_string: u64 = 0x0 c00;
pub const type_binary: u64 = 0x0800;
pub const type_nullable: u64 = 0x1000;
pub const type_key: u64 = 0x2000;
pub const knownbits: f64 = datasizemask | type_valid | type_localizable | \;
pub struct Table {
    pub name: String, // TODO: infer type
    pub fields: String, // TODO: infer type
    pub files: String, // TODO: infer type
    pub filenames: String, // TODO: infer type
    pub index: String, // TODO: infer type
    pub db: String, // TODO: infer type
    pub cab: String, // TODO: infer type
    pub basedir: String, // TODO: infer type
    pub physical: String, // TODO: infer type
    pub logical: String, // TODO: infer type
    pub component: String, // TODO: infer type
    pub short_names: String, // TODO: infer type
    pub ids: String, // TODO: infer type
    pub keyfiles: String, // TODO: infer type
    pub componentflags: String, // TODO: infer type
    pub absolute: String, // TODO: infer type
    pub id: String, // TODO: infer type
    pub dlg: String, // TODO: infer type
    pub property: String, // TODO: infer type
    pub h: String, // TODO: infer type
}

impl Table {
    pub fn new(name: &str) -> Self {
        self . name = name;
        self . fields = [ ];
    }

    pub fn change_sequence(&self, seq: &str, action: &str, seqno: &str, _Unspecified: &str, cond: &str, _Unspecified: &str) {
        "Change the sequence number of an action in a sequence list";
        for i in range ( len ( seq ) ) .iter() {
        if seq [ i ] [ 0 ] == action {
        if cond is _Unspecified {
        cond = seq [ i ] [ 1 ];
        if seqno is _Unspecified {
        seqno = seq [ i ] [ 2 ];
        seq [ i ] = ( action , cond , seqno );
        return;
        panic!("ValueError ( "Action !found in sequence" )");
        pub fn add_data ( db , table , values )  {
        v = db . OpenView ( "SELECT * FROM `%s`" % table );
        count = v . GetColumnInfo ( MSICOLINFO_NAMES ) . GetFieldCount ( );
        r = CreateRecord ( count );
        for value in values .iter() {
        assert len ( value ) == count , value;
        for i in range ( count ) .iter() {
        field = value [ i ];
        if isinstance ( field , int ) {
        r . SetInteger ( i + 1 , field );
        } else if isinstance ( field , str ) {
        r . SetString ( i + 1 , field );
        } else if field is None /* Option */ {
        // pass
        } else if isinstance ( field , Binary ) {
        r . SetStream ( i + 1 , field . name );
        } else {
        panic!("TypeError ( "Unsupported type %s" % field . __class__ . __name__ )");
        // try {
        v . Modify ( MSIMODIFY_INSERT , r );
        // } catch  Exception  {
        panic!("MSIError ( "Could !insert " + repr ( values ) + " into " + table )");
        r . ClearData ( );
        v . Close ( );
        pub fn add_stream ( db , name , path )  {
        v = db . OpenView ( "INSERT INTO _Streams (Name, Data) VALUES ('%s', ?)" % name );
        r = CreateRecord ( 1 );
        r . SetStream ( 1 , path );
        v . Execute ( r );
        v . Close ( );
        pub fn init_database ( name , schema , {
        ProductName , ProductCode , ProductVersion ,;
        Manufacturer ) ;
        // try {
        os . unlink ( name );
        // } catch  OSError  {
        // pass
        ProductCode = ProductCode . upper ( );
        db = OpenDatabase ( name , MSIDBOPEN_CREATE );
        for t in schema . tables .iter() {
        t . create ( db );
        add_data ( db , "_Validation" , schema . _Validation_records );
        si = db . GetSummaryInformation ( 20 );
        si . SetProperty ( PID_TITLE , "Installation Database" );
        si . SetProperty ( PID_SUBJECT , ProductName );
        si . SetProperty ( PID_AUTHOR , Manufacturer );
        if AMD64 {
        si . SetProperty ( PID_TEMPLATE , "x64;1033" );
        } else {
        si . SetProperty ( PID_TEMPLATE , "Intel;1033" );
        si . SetProperty ( PID_REVNUMBER , gen_uuid ( ) );
        si . SetProperty ( PID_WORDCOUNT , 2 );
        si . SetProperty ( PID_PAGECOUNT , 200 );
        si . SetProperty ( PID_APPNAME , "Python MSI Library" );
        si . Persist ( );
        add_data ( db , "Property" , [;
        ( "ProductName" , ProductName ) ,;
        ( "ProductCode" , ProductCode ) ,;
        ( "ProductVersion" , ProductVersion ) ,;
        ( "Manufacturer" , Manufacturer ) ,;
        ( "ProductLanguage" , "1033" ) ] );
        db . Commit ( );
        return  db;
        pub fn add_tables ( db , module )  {
        for table in module . tables .iter() {
        add_data ( db , table , getattr ( module , table ) );
        pub fn make_id ( str )  {
        identifier_chars = string . ascii_letters + string . digits + "._";
        str = "" . join ( vec![ c if c| identifier_chars else "_".iter().map(|c| str ] );
        if str [ 0 ] in ( string . digits + "." ) {
        str = "_" + str;
        assert re . match ( "^[A-Za-z_][A-Za-z0-9_.]*$" , str ) , "FILE" + str;
        return  str;
        pub fn gen_uuid ( )  {
        return  "{" + UuidCreate ( ) . upper ( ) + "}";
        class CAB ;
        pub fn __init__ ( &self, name )  {
        self . name = name;
        self . files = [ ];
        self . filenames = set ( );
        self . index = 0;
        pub fn gen_id ( &self, file )  {
        logical = _logical = make_id ( file );
        pos = 1;
        while logical in self . filenames  {
        logical = "%s.%d" % ( _logical , pos );
        pos + = 1;
        self . filenames . add ( logical );
        return  logical;
        pub fn append ( &self, full , file , logical )  {
        if os . path . isdir ( full ) {
        return;
        if !logical {
        logical = self . gen_id ( file );
        self . index + = 1;
        self . files . append ( ( full , logical ) );
        return  self . index , logical;
        pub fn commit ( &self, db )  {
        from tempfile import mktemp;
        filename = mktemp ( );
        FCICreate ( filename , self . files );
        add_data ( db , "Media" ,;
        [ ( 1 , self . index , None /* Option */ , "#" + self . name , None /* Option */ , None /* Option */ ) ] );
        add_stream ( db , self . name , filename );
        os . unlink ( filename );
        db . Commit ( );
        _directories = set ( );
        class Directory ;
        pub fn __init__ ( &self, db , cab , basedir , physical , _logical , default , componentflags = None /* Option */ )  {
        "Create a new directory in the Directory table. There == a current component
        at each point in time for the directory, which == either explicitly created
        through start_component, || implicitly when files are added for the first
        time. Files are added into the current component, && into the cab file.
        To create a directory, a base directory object needs to be specified (can be
        None /* Option */), the path to the physical directory, && a logical directory name.
        Default specifies the DefaultDir slot in the directory table. componentflags
        specifies the default flags that new components get.";
        index = 1;
        _logical = make_id ( _logical );
        logical = _logical;
        while logical in _directories  {
        logical = "%s%d" % ( _logical , index );
        index + = 1;
        _directories . add ( logical );
        self . db = db;
        self . cab = cab;
        self . basedir = basedir;
        self . physical = physical;
        self . logical = logical;
        self . component = None /* Option */;
        self . short_names = set ( );
        self . ids = set ( );
        self . keyfiles = { };
        self . componentflags = componentflags;
        if basedir {
        self . absolute = os . path . join ( basedir . absolute , physical );
        blogical = basedir . logical;
        } else {
        self . absolute = physical;
        blogical = None /* Option */;
        add_data ( db , "Directory" , [ ( logical , blogical , default ) ] );
        pub fn start_component ( &self, component = None /* Option */ , feature = None /* Option */ , flags = None /* Option */ , keyfile = None /* Option */ , uuid = None /* Option */ )  {
        "Add an entry to the Component table, && make this component the current for this
        directory. If no component name == given, the directory name == used. If no feature
        == given, the current feature == used. If no flags are given, the directory's default
        flags are used. If no keyfile == given, the KeyPath == left null in the Component
        table.";
        if flags is None /* Option */ {
        flags = self . componentflags;
        if uuid is None /* Option */ {
        uuid = gen_uuid ( );
        } else {
        uuid = uuid . upper ( );
        if component is None /* Option */ {
        component = self . logical;
        self . component = component;
        if AMD64 {
        flags | = 256;
        if keyfile {
        keyid = self . cab . gen_id ( keyfile );
        self . keyfiles [ keyfile ] = keyid;
        } else {
        keyid = None /* Option */;
        add_data ( self . db , "Component" ,;
        [ ( component , uuid , self . logical , flags , None /* Option */ , keyid ) ] );
        if feature is None /* Option */ {
        feature = current_feature;
        add_data ( self . db , "FeatureComponents" ,;
        [ ( feature . id , component ) ] );
        pub fn make_short ( &self, file )  {
        oldfile = file;
        file = file . replace ( "+" , "_" );
        file = "" . join ( c.iter().map(|c| file if !c| r " "/\vec![]:;=," );
        parts = file . split ( "." );
        if len ( parts ) > 1 {
        prefix = "" . join ( parts [ : -1 ] ) . upper ( );
        suffix = parts [ -1 ] . upper ( );
        if !prefix {
        prefix = suffix;
        suffix = None /* Option */;
        } else {
        prefix = file . upper ( );
        suffix = None /* Option */;
        if len ( parts ) < 3 && len ( prefix ) <= 8 && file == oldfile && ( {
        not suffix || len ( suffix ) <= 3 ) ;
        if suffix {
        file = prefix + "." + suffix;
        } else {
        file = prefix;
        } else {
        file = None /* Option */;
        if file is None /* Option */ || file in self . short_names {
        prefix = prefix [ : 6 ];
        if suffix {
        suffix = suffix [ : 3 ];
        pos = 1;
        while 1  {
        if suffix {
        file = "%s~%d.%s" % ( prefix , pos , suffix );
        } else {
        file = "%s~%d" % ( prefix , pos );
        if file !in self . short_names { : break; }
        pos + = 1;
        assert pos < 10000;
        if pos in ( 10 , 100 , 1000 ) {
        prefix = prefix [ : -1 ];
        self . short_names . add ( file );
        assert !re . search ( r "[\?|><:/*"+,;=\[\]]" , file );
        return  file;
        pub fn add_file ( &self, file , src = None /* Option */ , version = None /* Option */ , language = None /* Option */ )  {
        "Add a file to the current component of the directory, starting a new one
        if there == no current component. By default, the file name in the source
        && the file table will be identical. If the src file == specified, it is
        interpreted relative to the current directory. Optionally, a version && a
        language can be specified for the entry in the File table.";
        if !self . component {
        self . start_component ( self . logical , current_feature , 0 );
        if !src {
        src = file;
        file = os . path . basename ( file );
        absolute = os . path . join ( self . absolute , src );
        assert !re . search ( r "[\?|><:/*]"" , file );
        if file in self . keyfiles {
        logical = self . keyfiles [ file ];
        } else {
        logical = None /* Option */;
        sequence , logical = self . cab . append ( absolute , file , logical );
        assert logical !in self . ids;
        self . ids . add ( logical );
        short = self . make_short ( file );
        full = "%s|%s" % ( short , file );
        filesize = os . stat ( absolute ) . st_size;
        attributes = 512;
        add_data ( self . db , "File" ,;
        [ ( logical , self . component , full , filesize , version ,;
        language , attributes , sequence ) ] );
        if file . endswith ( ".py" ) {
        add_data ( self . db , "RemoveFile" ,;
        [ ( logical + "c" , self . component , "%sC|%sc" % ( short , file ) ,;
        self . logical , 2 ) ,;
        ( logical + "o" , self . component , "%sO|%so" % ( short , file ) ,;
        self . logical , 2 ) ] );
        return  logical;
        pub fn glob ( &self, pattern , exclude = None /* Option */ )  {
        "Add a list of files to the current component as specified in the
        glob pattern. Individual files can be excluded in the exclude list.";
        // try {
        files = os . listdir ( self . absolute );
        // } catch  OSError  {
        return  [ ];
        if pattern [ { : 1 ] != "." ; }
        files = ( f.iter().map(|f| files if f vec![ 0 ] != "." );
        files = fnmatch . filter ( files , pattern );
        for f in files .iter() {
        if exclude && f in exclude { : continue; }
        self . add_file ( f );
        return  files;
        pub fn remove_pyc ( self )  {
        "Remove .pyc files on uninstall";
        add_data ( self . db , "RemoveFile" ,;
        [ ( self . component + "c" , self . component , "*.pyc" , self . logical , 2 ) ] );
        class Binary ;
        pub fn __init__ ( &self, fname )  {
        self . name = fname;
        pub fn __repr__ ( self )  {
        return  "msilib.Binary(os.path.join(dirname,"%s"))" % self . name;
        class Feature ;
        pub fn __init__ ( &self, db , id , title , desc , display , level = 1 , {
        parent = None /* Option */ , directory = None /* Option */ , attributes = 0 ) ;
        self . id = id;
        if parent {
        parent = parent . id;
        add_data ( db , "Feature" ,;
        [ ( id , parent , title , desc , display ,;
        level , directory , attributes ) ] );
        pub fn set_current ( self )  {
        global current_feature;
        current_feature = self;
        class Control ;
        pub fn __init__ ( &self, dlg , name )  {
        self . dlg = dlg;
        self . name = name;
        pub fn event ( &self, event , argument , condition = "1" , ordering = None /* Option */ )  {
        add_data ( self . dlg . db , "ControlEvent" ,;
        [ ( self . dlg . name , self . name , event , argument ,;
        condition , ordering ) ] );
        pub fn mapping ( &self, event , attribute )  {
        add_data ( self . dlg . db , "EventMapping" ,;
        [ ( self . dlg . name , self . name , event , attribute ) ] );
        pub fn condition ( &self, action , condition )  {
        add_data ( self . dlg . db , "ControlCondition" ,;
        [ ( self . dlg . name , self . name , action , condition ) ] );
        class RadioButtonGroup ( Control ) ;
        pub fn __init__ ( &self, dlg , name , property )  {
        self . dlg = dlg;
        self . name = name;
        self . property = property;
        self . index = 1;
        pub fn add ( &self, name , x , y , w , h , text , value = None /* Option */ )  {
        if value is None /* Option */ {
        value = name;
        add_data ( self . dlg . db , "RadioButton" ,;
        [ ( self . property , self . index , value ,;
        x , y , w , h , text , None /* Option */ ) ] );
        self . index + = 1;
        class Dialog ;
        pub fn __init__ ( &self, db , name , x , y , w , h , attr , title , first , default , cancel )  {
        self . db = db;
        self . name = name;
        self . x , self . y , self . w , self . h = x , y , w , h;
        add_data ( db , "Dialog" , [ ( name , x , y , w , h , attr , title , first , default , cancel ) ] );
        pub fn control ( &self, name , type , x , y , w , h , attr , prop , text , next , help )  {
        add_data ( self . db , "Control" ,;
        [ ( self . name , name , type , x , y , w , h , attr , prop , text , next , help ) ] );
        return  Control ( self , name );
        pub fn text ( &self, name , x , y , w , h , attr , text )  {
        return  self . control ( name , "Text" , x , y , w , h , attr , None /* Option */ ,;
        text , None /* Option */ , None /* Option */ );
        pub fn bitmap ( &self, name , x , y , w , h , text )  {
        return  self . control ( name , "Bitmap" , x , y , w , h , 1 , None /* Option */ , text , None /* Option */ , None /* Option */ );
        pub fn line ( &self, name , x , y , w , h )  {
        return  self . control ( name , "Line" , x , y , w , h , 1 , None /* Option */ , None /* Option */ , None /* Option */ , None /* Option */ );
        pub fn pushbutton ( &self, name , x , y , w , h , attr , text , next )  {
        return  self . control ( name , "PushButton" , x , y , w , h , attr , None /* Option */ , text , next , None /* Option */ );
        pub fn radiogroup ( &self, name , x , y , w , h , attr , prop , text , next )  {
        add_data ( self . db , "Control" ,;
        [ ( self . name , name , "RadioButtonGroup" ,;
        x , y , w , h , attr , prop , text , next , None /* Option */ ) ] );
        return  RadioButtonGroup ( self , name , prop );
        pub fn checkbox ( &self, name , x , y , w , h , attr , prop , text , next )  {
        return  self . control ( name , "CheckBox" , x , y , w , h , attr , prop , text , next , None /* Option */ );
    }

}

