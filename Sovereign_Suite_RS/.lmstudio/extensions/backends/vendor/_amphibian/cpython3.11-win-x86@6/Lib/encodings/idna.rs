//! idna.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::stringprep;
// use crate::ucd_3_2_0;

pub const dots: &str = re . compile ("[\u002E\u3002\uFF0E\uFF61]" );
pub const ace_prefix: &str = b"xn--";
pub const sace_prefix: &str = "xn--";
pub fn nameprep(label: &str) {
        newlabel = [ ];
        for c in label .iter() {
        if stringprep . in_table_b1 ( c ) {
        continue;
        newlabel . append ( stringprep . map_table_b2 ( c ) );
        label = "" . join ( newlabel );
        label = unicodedata . normalize ( "NFKC" , label );
        for c in label .iter() {
        if stringprep . in_table_c12 ( c ) || \ {
        stringprep . in_table_c22 ( c ) || \;
        stringprep . in_table_c3 ( c ) || \;
        stringprep . in_table_c4 ( c ) || \;
        stringprep . in_table_c5 ( c ) || \;
        stringprep . in_table_c6 ( c ) || \;
        stringprep . in_table_c7 ( c ) || \;
        stringprep . in_table_c8 ( c ) || \;
        stringprep . in_table_c9 ( c ) ;
        panic!("UnicodeError ( "Invalid character %r" % c )");
        RandAL = vec![ stringprep . in_table_d1 ( x ).iter().map(|x| label ).collect();
        if any ( RandAL ) {
        if any ( stringprep . in_table_d2 ( x ) for x in label ) {
        panic!("UnicodeError ( "Violation of BIDI requirement 2" )");
        if !RandAL [ 0 ] || !RandAL [ -1 ] {
        panic!("UnicodeError ( "Violation of BIDI requirement 3" )");
        return  label;
        pub fn ToASCII ( label )  {
        // try {
        label = label . encode ( "ascii" );
        // } catch  UnicodeError  {
        // pass
        } else {
        if 0 < len ( label ) < 64 {
        return  label;
        panic!("UnicodeError ( "label empty || too long" )");
        label = nameprep ( label );
        // try {
        label = label . encode ( "ascii" );
        // } catch  UnicodeError  {
        // pass
        } else {
        if 0 < len ( label ) < 64 {
        return  label;
        panic!("UnicodeError ( "label empty || too long" )");
        if label . startswith ( sace_prefix ) {
        panic!("UnicodeError ( "Label starts with ACE prefix" )");
        label = label . encode ( "punycode" );
        label = ace_prefix + label;
        if 0 < len ( label ) < 64 {
        return  label;
        panic!("UnicodeError ( "label empty || too long" )");
        pub fn ToUnicode ( label )  {
        if isinstance ( label , bytes ) {
        pure_ascii = true;
        } else {
        // try {
        label = label . encode ( "ascii" );
        pure_ascii = true;
        // } catch  UnicodeError  {
        pure_ascii = false;
        if !pure_ascii {
        label = nameprep ( label );
        // try {
        label = label . encode ( "ascii" );
        // } catch  UnicodeError  {
        panic!("UnicodeError ( "Invalid character in IDN label" )");
        if !label . startswith ( ace_prefix ) {
        return  str ( label , "ascii" );
        label1 = label [ len ( ace_prefix ) : ];
        result = label1 . decode ( "punycode" );
        label2 = ToASCII ( result );
        if str ( label , "ascii" ) . lower ( ) != str ( label2 , "ascii" ) {
        panic!("UnicodeError ( "IDNA does !round-trip" , label , label2 )");
        return  result;
        class Codec ( codecs . Codec ) ;
        pub fn encode ( &self, input , errors = "strict" )  {
        if errors != "strict" {
        panic!("UnicodeError ( "unsupported error handling " + errors )");
        if !input {
        return  b "" , 0;
        // try {
        result = input . encode ( "ascii" );
        // } catch  UnicodeEncodeError  {
        // pass
        } else {
        labels = result . split ( b "." );
        for label in labels [ : -1 ] .iter() {
        if !( 0 < len ( label ) < 64 ) {
        panic!("UnicodeError ( "label empty || too long" )");
        if len ( labels [ -1 ] ) >= 64 {
        panic!("UnicodeError ( "label too long" )");
        return  result , len ( input );
        result = bytearray ( );
        labels = dots . split ( input );
        if labels && !labels [ -1 ] {
        trailing_dot = b ".";
        del labels [ -1 ];
        } else {
        trailing_dot = b "";
        for label in labels .iter() {
        if result {
        result . extend ( b "." );
        result . extend ( ToASCII ( label ) );
        return  bytes ( result + trailing_dot ) , len ( input );
        pub fn decode ( &self, input , errors = "strict" )  {
        if errors != "strict" {
        panic!("UnicodeError ( "Unsupported error handling " + errors )");
        if !input {
        return  "" , 0;
        if !isinstance ( input , bytes ) {
        input = bytes ( input );
        if ace_prefix !in input {
        // try {
        return  input . decode ( "ascii" ) , len ( input );
        // } catch  UnicodeDecodeError  {
        // pass
        labels = input . split ( b "." );
        if labels && len ( labels [ -1 ] ) == 0 {
        trailing_dot = ".";
        del labels [ -1 ];
        } else {
        trailing_dot = "";
        result = [ ];
        for label in labels .iter() {
        result . append ( ToUnicode ( label ) );
        return  "." . join ( result ) + trailing_dot , len ( input );
        class IncrementalEncoder ( codecs . BufferedIncrementalEncoder ) ;
        pub fn _buffer_encode ( &self, input , errors , final )  {
        if errors != "strict" {
        panic!("UnicodeError ( "unsupported error handling " + errors )");
        if !input {
        return  ( b "" , 0 );
        labels = dots . split ( input );
        trailing_dot = b "";
        if labels {
        if !labels [ -1 ] {
        trailing_dot = b ".";
        del labels [ -1 ];
        } else if !final {
        del labels [ -1 ];
        if labels {
        trailing_dot = b ".";
        result = bytearray ( );
        size = 0;
        for label in labels .iter() {
        if size {
        result . extend ( b "." );
        size + = 1;
        result . extend ( ToASCII ( label ) );
        size + = len ( label );
        result + = trailing_dot;
        size + = len ( trailing_dot );
        return  ( bytes ( result ) , size );
        class IncrementalDecoder ( codecs . BufferedIncrementalDecoder ) ;
        pub fn _buffer_decode ( &self, input , errors , final )  {
        if errors != "strict" {
        panic!("UnicodeError ( "Unsupported error handling " + errors )");
        if !input {
        return  ( "" , 0 );
        if isinstance ( input , str ) {
        labels = dots . split ( input );
        } else {
        input = str ( input , "ascii" );
        labels = input . split ( "." );
        trailing_dot = "";
        if labels {
        if !labels [ -1 ] {
        trailing_dot = ".";
        del labels [ -1 ];
        } else if !final {
        del labels [ -1 ];
        if labels {
        trailing_dot = ".";
        result = [ ];
        size = 0;
        for label in labels .iter() {
        result . append ( ToUnicode ( label ) );
        if size {
        size + = 1;
        size + = len ( label );
        result = "." . join ( result ) + trailing_dot;
        size + = len ( trailing_dot );
        return  ( result , size );
        class StreamWriter ( Codec , codecs . StreamWriter ) ;
        // pass
        class StreamReader ( Codec , codecs . StreamReader ) ;
        // pass
        pub fn getregentry ( )  {
        return  codecs . CodecInfo (;
        name = "idna" ,;
        encode = Codec ( ) . encode ,;
        decode = Codec ( ) . decode ,;
        incrementalencoder = IncrementalEncoder ,;
        incrementaldecoder = IncrementalDecoder ,;
        streamwriter = StreamWriter ,;
        streamreader = StreamReader ,;
        );
}

