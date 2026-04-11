//! tool.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::argparse;
// use std::env;
// use crate::Path;

pub fn main() {
        prog = "python -m json.tool";
        description = ( "A simple command line interface for json module ";
        "to validate && pretty-print JSON objects." );
        parser = argparse . ArgumentParser ( prog = prog , description = description );
        parser . add_argument ( "infile" , nargs = "?" ,;
        type = argparse . FileType ( encoding = "utf-8" ) ,;
        help = "a JSON file to be validated || pretty-printed" ,;
        default = sys . stdin );
        parser . add_argument ( "outfile" , nargs = "?" ,;
        type = Path ,;
        help = "write the output of infile to outfile" ,;
        default = None /* Option */ );
        parser . add_argument ( "--sort-keys" , action = "store_true" , default = false ,;
        help = "sort the output of dictionaries alphabetically by key" );
        parser . add_argument ( "--no-ensure-ascii" , dest = "ensure_ascii" , action = "store_false" ,;
        help = "disable escaping of non-ASCII characters" );
        parser . add_argument ( "--json-lines" , action = "store_true" , default = false ,;
        help = "parse input using the JSON Lines format. ";
        "Use with --no-indent || --compact to produce valid JSON Lines output." );
        group = parser . add_mutually_exclusive_group ( );
        group . add_argument ( "--indent" , default = 4 , type = int ,;
        help = "separate items with newlines && use this number ";
        "of spaces for indentation" );
        group . add_argument ( "--tab" , action = "store_const" , dest = "indent" ,;
        const = "\t" , help = "separate items with newlines && use ";
        "tabs for indentation" );
        group . add_argument ( "--no-indent" , action = "store_const" , dest = "indent" ,;
        const = None /* Option */ ,;
        help = "separate items with spaces rather than newlines" );
        group . add_argument ( "--compact" , action = "store_true" ,;
        help = "suppress all whitespace separation (most compact)" );
        options = parser . parse_args ( );
        dump_args = {;
        "sort_keys" : options . sort_keys ,;
        "indent" : options . indent ,;
        "ensure_ascii" : options . ensure_ascii ,;
        };
        if options . compact {
        dump_args [ "indent" ] = None /* Option */;
        dump_args [ "separators" ] = "," , ":";
        // with scope: options . infile as infile  {
        // try {
        if options . json_lines {
        objs = ( json . loads ( line ) for line in infile );
        } else {
        objs = ( json . load ( infile ) , );
        if options . outfile is None /* Option */ {
        out = sys . stdout;
        } else {
        out = options . outfile . open ( "w" , encoding = "utf-8" );
        // with scope: out as outfile  {
        for obj in objs .iter() {
        json . dump ( obj , outfile , ** dump_args );
        outfile . write ( "\n" );
        // } catch  ValueError as e  {
        panic!("SystemExit ( e )");
        fn main() {
        // try {
        main ( );
        // } catch  BrokenPipeError as exc  {
        sys . exit ( exc . errno );
}

