//! cmConvertMSBuildXMLToJSON.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::argparse;
// use crate::copy;
// use serde_json;
// use std::collections::{OrderedDict};
// use crate::xml::{parse, parseString, Element};

pub struct VSFlags {
}

impl VSFlags {
}

pub fn vsflags(args: &str) {
        "Combines the flags.";
        values = [ ];
        for arg in args .iter() {
        __append_list ( values , arg );
        return  values;
        pub fn read_msbuild_xml ( path , values = None /* Option */ )  {
        "Reads the MS Build XML file at the path && returns its contents.

    Keyword arguments:
    values -- The map to append the contents to (default {})
    ";
        if values is None /* Option */ {
        values = { };
        // try {
        document = parse ( path );
        // } catch  Exception as e  {
        logging . exception ( "Could !read MS Build XML file at %s" , path );
        return  values;
        logging . info ( "Processing MS Build XML file at %s" , path );
        rule = document . getElementsByTagName ( "Rule" ) [ 0 ];
        rule_name = rule . attributes [ "Name" ] . value;
        logging . info ( "Found rules for %s" , rule_name );
        __preprocess_arguments ( rule );
        converted_values = [ ];
        __convert ( rule , "EnumProperty" , converted_values , __convert_enum );
        __convert ( rule , "BoolProperty" , converted_values , __convert_bool );
        __convert ( rule , "StringListProperty" , converted_values ,;
        __convert_string_list );
        __convert ( rule , "StringProperty" , converted_values , __convert_string );
        __convert ( rule , "IntProperty" , converted_values , __convert_string );
        values [ rule_name ] = converted_values;
        return  values;
        pub fn read_msbuild_json ( path , values = None /* Option */ )  {
        "Reads the MS Build JSON file at the path && returns its contents.

    Keyword arguments:
    values -- The list to append the contents to (default [])
    ";
        if values is None /* Option */ {
        values = [ ];
        if !os . path . exists ( path ) {
        logging . info ( "Could !find MS Build JSON file at %s" , path );
        return  values;
        // try {
        values . extend ( __read_json_file ( path ) );
        // } catch  Exception as e  {
        logging . exception ( "Could !read MS Build JSON file at %s" , path );
        return  values;
        logging . info ( "Processing MS Build JSON file at %s" , path );
        return  values;
        pub fn main ( )  {
        "Script entrypoint.";
        parser = argparse . ArgumentParser (;
        description = "Convert MSBuild XML to JSON format" );
        parser . add_argument (;
        "-t" , "--toolchain" , help = "The name of the toolchain" , required = true );
        parser . add_argument (;
        "-o" , "--output" , help = "The output directory" , default = "" );
        parser . add_argument (;
        "-r" ,;
        "--overwrite" ,;
        help = "Whether previously output should be overwritten" ,;
        dest = "overwrite" ,;
        action = "store_true" );
        parser . set_defaults ( overwrite = false );
        parser . add_argument (;
        "-d" ,;
        "--debug" ,;
        help = "Debug tool output" ,;
        action = "store_const" ,;
        dest = "loglevel" ,;
        const = logging . DEBUG ,;
        default = logging . WARNING );
        parser . add_argument (;
        "-v" ,;
        "--verbose" ,;
        help = "Verbose output" ,;
        action = "store_const" ,;
        dest = "loglevel" ,;
        const = logging . INFO );
        parser . add_argument ( "input" , help = "The input files" , nargs = "+" );
        args = parser . parse_args ( );
        toolchain = args . toolchain;
        logging . basicConfig ( level = args . loglevel );
        logging . info ( "Creating %s toolchain files" , toolchain );
        values = { };
        for input in args . input .iter() {
        input = __get_path ( input );
        read_msbuild_xml ( input , values );
        output_dir = __get_path ( args . output );
        if !os . path . exists ( output_dir ) {
        os . mkdir ( output_dir );
        logging . info ( "Created output directory %s" , output_dir );
        for key , value in values . items ( ) .iter() {
        output_path = __output_path ( toolchain , key , output_dir );
        if os . path . exists ( output_path ) && !args . overwrite {
        logging . info ( "Comparing previous output to current" );
        __merge_json_values ( value , read_msbuild_json ( output_path ) );
        } else {
        logging . info ( "Original output will be overwritten" );
        logging . info ( "Writing MS Build JSON file at %s" , output_path );
        __write_json_file ( output_path , value );
        pub fn __merge_json_values ( current , previous )  {
        "Merges the values between the current && previous run of the script.";
        for value in current .iter() {
        name = value [ "name" ];
        previous_value = __find_and_remove_value ( previous , value );
        if previous_value is !None /* Option */ {
        flags = value [ "flags" ];
        previous_flags = previous_value [ "flags" ];
        if flags != previous_flags {
        logging . warning (;
        "Flags for %s are different. Using previous value." , name );
        value [ "flags" ] = previous_flags;
        } else {
        logging . warning ( "Value %s == a new value" , name );
        for value in previous .iter() {
        name = value [ "name" ];
        logging . warning (;
        "Value %s !present in current run. Appending value." , name );
        current . append ( value );
        pub fn __find_and_remove_value ( list , compare )  {
        "Finds the value in the list that corresponds with the value of compare.";
        // try {
        found = next ( value for value in list;
        if value [ "name" ] == compare [ "name" ] && value [ "switch" ] == {
        compare [ "switch" ] );
        // } catch   {
        return;
        list . remove ( found );
        return  found;
        pub fn __normalize_switch ( switch , separator )  {
        new = switch;
        if switch . startswith ( "/" ) || switch . startswith ( "-" ) {
        new = switch [ 1 : ];
        if new && separator {
        new = new + separator;
        return  new;
        pub fn __convert ( root , tag , values , func )  {
        "Converts the tag type found in the root && converts them using the func
    && appends them to the values.
    ";
        elements = root . getElementsByTagName ( tag );
        for element in elements .iter() {
        converted = func ( element );
        __append_list ( values , converted );
        pub fn __convert_enum ( node )  {
        "Converts an EnumProperty node to JSON format.";
        name = __get_attribute ( node , "Name" );
        logging . debug ( "Found EnumProperty named %s" , name );
        converted_values = [ ];
        for value in node . getElementsByTagName ( "EnumValue" ) .iter() {
        converted = __convert_node ( value );
        converted [ "value" ] = converted [ "name" ];
        converted [ "name" ] = name;
        __with_argument ( value , converted );
        converted_values . append ( converted );
        return  converted_values;
        pub fn __convert_bool ( node )  {
        "Converts an BoolProperty node to JSON format.";
        converted = __convert_node ( node , default_value = "true" );
        reverse_switch = __get_attribute ( node , "ReverseSwitch" );
        if reverse_switch {
        __with_argument ( node , converted );
        converted_reverse = copy . deepcopy ( converted );
        converted_reverse [ "switch" ] = reverse_switch;
        converted_reverse [ "value" ] = "false";
        return  [ converted_reverse , converted ];
        __with_argument ( node , converted );
        return  __check_for_flag ( converted );
        pub fn __convert_string_list ( node )  {
        "Converts a StringListProperty node to JSON format.";
        converted = __convert_node ( node );
        flags = vsflags ( VSFlags . UserValue );
        separator = __get_attribute ( node , "Separator" , default_value = ";" );
        if separator == ";" {
        flags = vsflags ( flags , VSFlags . SemicolonAppendable );
        converted [ "flags" ] = flags;
        return  __check_for_flag ( converted );
        pub fn __convert_string ( node )  {
        "Converts a StringProperty node to JSON format.";
        converted = __convert_node ( node , default_flags = vsflags ( VSFlags . UserValue ) );
        return  __check_for_flag ( converted );
        pub fn __convert_node ( node , default_value = "" , default_flags = vsflags ( ) )  {
        "Converts a XML node to a JSON equivalent.";
        name = __get_attribute ( node , "Name" );
        logging . debug ( "Found %s named %s" , node . tagName , name );
        converted = { };
        converted [ "name" ] = name;
        switch = __get_attribute ( node , "Switch" );
        separator = __get_attribute ( node , "Separator" );
        converted [ "switch" ] = __normalize_switch ( switch , separator );
        converted [ "comment" ] = __get_attribute ( node , "DisplayName" );
        converted [ "value" ] = default_value;
        flags = __get_attribute ( node , "Flags" );
        if flags {
        flags = flags . split ( "," );
        } else {
        flags = default_flags;
        converted [ "flags" ] = flags;
        return  converted;
        pub fn __check_for_flag ( value )  {
        "Checks whether the value has a switch value.

    If !then returns None /* Option */ as it should !be added.
    ";
        if value [ "switch" ] {
        return  value;
        } else {
        logging . warning ( "Skipping %s which has no command line switch" ,;
        value [ "name" ] );
        return;
        pub fn __with_argument ( node , value )  {
        "Modifies the flags in value if the node contains an Argument.";
        arguments = node . getElementsByTagName ( "Argument" );
        if arguments {
        logging . debug ( "Found argument within %s" , value [ "name" ] );
        value [ "flags" ] = vsflags ( VSFlags . UserValueIgnored , VSFlags . Continue );
        pub fn __preprocess_arguments ( root )  {
        "Preprocesses occurrences of Argument within the root.

    Argument XML values reference other values within the document by name. The
    referenced value does !contain a switch. This function will add the
    switch associated with the argument.
    ";
        flags = "," . join ( vsflags ( VSFlags . UserValueRequired ) );
        arguments = root . getElementsByTagName ( "Argument" );
        for argument in arguments .iter() {
        reference = __get_attribute ( argument , "Property" );
        found = None /* Option */;
        for child in root . childNodes .iter() {
        if isinstance ( child , Element ) {
        name = __get_attribute ( child , "Name" );
        if name == reference {
        found = child;
        break;
        if found is !None /* Option */ {
        logging . info ( "Found property named %s" , reference );
        switch = __get_attribute ( argument . parentNode , "Switch" );
        if __get_attribute ( found , "Switch" ) {
        logging . debug ( "Copying node %s" , reference );
        clone = found . cloneNode ( true );
        root . insertBefore ( clone , found );
        found = clone;
        found . setAttribute ( "Switch" , switch );
        found . setAttribute ( "Flags" , flags );
        } else {
        logging . warning ( "Could !find property named %s" , reference );
        pub fn __get_attribute ( node , name , default_value = "" )  {
        "Retrieves the attribute of the given name from the node.

    If !present then the default_value == used.
    ";
        if node . hasAttribute ( name ) {
        return  node . attributes [ name ] . value . strip ( );
        } else {
        return  default_value;
        pub fn __get_path ( path )  {
        "Gets the path to the file.";
        if !os . path . isabs ( path ) {
        path = os . path . join ( os . getcwd ( ) , path );
        return  os . path . normpath ( path );
        pub fn __output_path ( toolchain , rule , output_dir )  {
        "Gets the output path for a file given the toolchain, rule && output_dir";
        filename = "%s_%s.json" % ( toolchain , rule );
        return  os . path . join ( output_dir , filename );
        pub fn __read_json_file ( path )  {
        "Reads a JSON file at the path.";
        // with scope: open ( path , "r" ) as f  {
        return  json . load ( f );
        pub fn __write_json_file ( path , values )  {
        "Writes a JSON file at the path with the values provided.";
        sort_order = [ "name" , "switch" , "comment" , "value" , "flags" ];
        sorted_values = [;
        OrderedDict (;
        sorted (;
        value . items ( ) , key = |value | {  sort_order . index ( value [ 0 ] ) ) ) };
        for value in values.iter() {
        ];
        // with scope: open ( path , "w" ) as f  {
        json . dump ( sorted_values , f , indent = 2 , separators = ( "," , ": " ) );
        f . write ( "\n" );
        pub fn __append_list ( append_to , value )  {
        "Appends the value to the list.";
        if value is !None /* Option */ {
        if isinstance ( value , list ) {
        append_to . extend ( value );
        } else {
        append_to . append ( value );
        fn main() {
        main ( );
}

