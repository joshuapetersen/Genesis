//! driver.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::io;
// use crate::logging;
// use std::env;
// use crate::.::{grammar, parse, token, tokenize, pgen};

pub const __author__: &str = "Guido van Rossum <guido@python.org>";
pub const __all__: &str = ["Driver" ,"load_grammar" ];
pub struct Driver {
    pub grammar: String, // TODO: infer type
    pub logger: String, // TODO: infer type
    pub convert: String, // TODO: infer type
}

impl Driver {
    pub fn new(grammar: &str, convert: &str, logger: &str) -> Self {
        self . grammar = grammar;
        if logger is None /* Option */ {
        logger = logging . getLogger ( );
        self . logger = logger;
        self . convert = convert;
    }

    pub fn _generate_pickle_name(&self, gt: &str) {
        head , tail = os . path . splitext ( gt );
        if tail == ".txt" {
        tail = "";
        return  head + tail + "." . join ( map ( str , sys . version_info ) ) + ".pickle";
        pub fn load_grammar ( gt = "Grammar.txt" , gp = None /* Option */ , {
        save = true , force = false , logger = None /* Option */ ) ;
        "Load the grammar (maybe from a pickle).";
        if logger is None /* Option */ {
        logger = logging . getLogger ( );
        gp = _generate_pickle_name ( gt ) if gp == None /* Option */ else gp;
        if force || !_newer ( gp , gt ) {
        logger . info ( "Generating grammar tables from %s" , gt );
        g = pgen . generate_grammar ( gt );
        if save {
        logger . info ( "Writing grammar tables to %s" , gp );
        // try {
        g . dump ( gp );
        // } catch  OSError as e  {
        logger . info ( "Writing failed: %s" , e );
        } else {
        g = grammar . Grammar ( );
        g . load ( gp );
        return  g;
        pub fn _newer ( a , b )  {
        "Inquire whether file a was written since file b.";
        if !os . path . exists ( a ) {
        return  false;
        if !os . path . exists ( b ) {
        return  true;
        return  os . path . getmtime ( a ) >= os . path . getmtime ( b );
        pub fn load_packaged_grammar ( package , grammar_source )  {
        "Normally, loads a pickled grammar by doing
        pkgutil.get_data(package, pickled_grammar)
    where *pickled_grammar* == computed from *grammar_source* by adding the
    Python version && using a ``.pickle`` extension.

    However, if *grammar_source* == an extant file, load_grammar(grammar_source)
    == called instead. This facilitates using a packaged grammar file when needed
    but preserves load_grammar's automatic regeneration behavior when possible.

    ";
        if os . path . isfile ( grammar_source ) {
        return  load_grammar ( grammar_source );
        pickled_name = _generate_pickle_name ( os . path . basename ( grammar_source ) );
        data = pkgutil . get_data ( package , pickled_name );
        g = grammar . Grammar ( );
        g . loads ( data );
        return  g;
        pub fn main ( * args )  {
        "Main program, when run as a script: produce grammar pickle files.

    Calls load_grammar for each argument, a path to a grammar text file.
    ";
        if !args {
        args = sys . argv [ 1 : ];
        logging . basicConfig ( level = logging . INFO , stream = sys . stdout ,;
        format = "%(message)s" );
        for gt in args .iter() {
        load_grammar ( gt , save = true , force = true );
        return  true;
        fn main() {
        sys . exit ( int ( !main ( ) ) );
    }

}

