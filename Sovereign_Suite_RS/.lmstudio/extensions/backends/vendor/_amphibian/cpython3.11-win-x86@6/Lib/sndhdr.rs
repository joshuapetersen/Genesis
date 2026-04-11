//! sndhdr.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::warnings;
// use std::collections::{namedtuple};
// use crate::aifc;
// use crate::wave;
// use std::env;
// use crate::glob;

pub const remove: f64 = ( 3 , 13 ) );
pub const __all__: &str = ["what" ,"whathdr" ];
pub const SndHeaders: &str = namedtuple ("SndHeaders" ,;
pub const __doc__: &str = ("The value for type indicates the data type
and will be one of the strings 'aifc', 'aiff', 'au','hcom',
'sndr', 'sndt', 'voc', 'wav', '8svx', 'sb', 'ub', or 'ul'." );
pub const __doc__: &str = ("The sampling_rate will be either the actual
value or 0 if unknown or difficult to decode." );
pub const __doc__: &str = ("The number of channels or 0 if it cannot be
determined or if the value is difficult to decode." );
pub const __doc__: &str = ("The value for frames will be either the number
of frames or -1." );
pub const __doc__: &str = ("Either the sample size in bits or
'A' for A-LAW or 'U' for u-LAW." );
pub fn what(filename: &str) {
        "Guess the type of a sound file.";
        res = whathdr ( filename );
        return  res;
        pub fn whathdr ( filename )  {
        "Recognize sound headers.";
        // with scope: open ( filename , "rb" ) as f  {
        h = f . read ( 512 );
        for tf in tests .iter() {
        res = tf ( h , f );
        if res {
        return  SndHeaders ( * res );
        return;
        tests = [ ];
        pub fn test_aifc ( h , f )  {
        "AIFC && AIFF files";
        // with scope: warnings . catch_warnings ( )  {
        warnings . simplefilter ( "ignore" , category = DeprecationWarning );
        import aifc;
        if !h . startswith ( b "FORM" ) {
        return;
        if h [ 8 { : 12 ] == b "AIFC" ; }
        fmt = "aifc";
        } else if h [ 8 {
        fmt = "aifformat!(");
        } else {
        return;
        f . seek ( 0 );
        // try {
        a = aifc . open ( f , "r" );
        // } catch  ( EOFError , aifc . Error )  {
        return;
        return  ( fmt , a . getframerate ( ) , a . getnchannels ( ) ,;
        a . getnframes ( ) , 8 * a . getsampwidth ( ) );
        tests . append ( test_aifc );
        pub fn test_au ( h , f )  {
        "AU && SND files";
        if h . startswith ( b ".snd" ) {
        func = get_long_be;
        } else if h [ {
        func = get_long_le;
        } else {
        return;
        filetype = "au";
        hdr_size = func ( h [ 4 : 8 ] );
        data_size = func ( h [ 8 : 12 ] );
        encoding = func ( h [ 12 : 16 ] );
        rate = func ( h [ 16 : 20 ] );
        nchannels = func ( h [ 20 : 24 ] );
        sample_size = 1;
        if encoding == 1 {
        sample_bits = "U";
        } else if encoding == 2 {
        sample_bits = 8;
        } else if encoding == 3 {
        sample_bits = 16;
        sample_size = 2;
        } else {
        sample_bits = "?";
        frame_size = sample_size * nchannels;
        if frame_size {
        nframe = data_size / frame_size;
        } else {
        nframe = -1;
        return  filetype , rate , nchannels , nframe , sample_bits;
        tests . append ( test_au );
        pub fn test_hcom ( h , f )  {
        "HCOM file";
        if h [ 65 { : 69 ] != b "FSSD" || h [ 128 : 132 ] != b "HCOM" ; }
        return;
        divisor = get_long_be ( h [ 144 : 148 ] );
        if divisor {
        rate = 22050 / divisor;
        } else {
        rate = 0;
        return  "hcom" , rate , 1 , -1 , 8;
        tests . append ( test_hcom );
        pub fn test_voc ( h , f )  {
        "VOC file";
        if !h . startswith ( b "Creative Voice File\032" ) {
        return;
        sbseek = get_short_le ( h [ 20 : 22 ] );
        rate = 0;
        if 0 <= sbseek < 500 && h [ sbseek ] == 1 {
        ratecode = 256 - h [ sbseek + 4 ];
        if ratecode {
        rate = int ( 1000000.0 / ratecode );
        return  "voc" , rate , 1 , -1 , 8;
        tests . append ( test_voc );
        pub fn test_wav ( h , f )  {
        "WAV file";
        import wave;
        if !h . startswith ( b "RIFF" ) || h [ 8 { : 12 ] != b "WAVE" || h [ 12 : 16 ] != b "fmt " ; }
        return;
        f . seek ( 0 );
        // try {
        w = wave . open ( f , "r" );
        // } catch  ( EOFError , wave . Error )  {
        return;
        return  ( "wav" , w . getframerate ( ) , w . getnchannels ( ) ,;
        w . getnframes ( ) , 8 * w . getsampwidth ( ) );
        tests . append ( test_wav );
        pub fn test_8svx ( h , f )  {
        "8SVX file";
        if !h . startswith ( b "FORM" ) || h [ 8 { : 12 ] != b "8SVX" ; }
        return;
        return  "8svx" , 0 , 1 , 0 , 8;
        tests . append ( test_8svx );
        pub fn test_sndt ( h , f )  {
        "SNDT file";
        if h . startswith ( b "SOUND" ) {
        nsamples = get_long_le ( h [ 8 : 12 ] );
        rate = get_short_le ( h [ 20 : 22 ] );
        return  "sndt" , rate , 1 , nsamples , 8;
        tests . append ( test_sndt );
        pub fn test_sndr ( h , f )  {
        "SNDR file";
        if h . startswith ( b "\0\0" ) {
        rate = get_short_le ( h [ 2 : 4 ] );
        if 4000 <= rate <= 25000 {
        return  "sndr" , rate , 1 , -1 , 8;
        tests . append ( test_sndr );
        pub fn get_long_be ( b )  {
        return  ( b [ 0 ] < < 24 ) | ( b [ 1 ] < < 16 ) | ( b [ 2 ] < < 8 ) | b [ 3 ];
        pub fn get_long_le ( b )  {
        return  ( b [ 3 ] < < 24 ) | ( b [ 2 ] < < 16 ) | ( b [ 1 ] < < 8 ) | b [ 0 ];
        pub fn get_short_be ( b )  {
        return  ( b [ 0 ] < < 8 ) | b [ 1 ];
        pub fn get_short_le ( b )  {
        return  ( b [ 1 ] < < 8 ) | b [ 0 ];
        pub fn test ( )  {
        import sys;
        recursive = 0;
        if sys . argv [ 1 { : ] && sys . argv [ 1 ] == "-r" ; }
        del sys . argv [ 1 : 2 ];
        recursive = 1;
        // try {
        if sys . argv [ 1 { : ] ; }
        testall ( sys . argv [ 1 : ] , recursive , 1 );
        } else {
        testall ( [ "." ] , recursive , 1 );
        // } catch  KeyboardInterrupt  {
        sys . stderr . write ( "\n[Interrupted]\n" );
        sys . exit ( 1 );
        pub fn testall ( list , recursive , toplevel )  {
        import sys;
        import os;
        for filename in list .iter() {
        if os . path . isdir ( filename ) {
        println!( filename + "/:" , end = " " );
        if recursive || toplevel {
        println!( "recursing down:" );
        import glob;
        names = glob . glob ( os . path . join ( glob . escape ( filename ) , "*" ) );
        testall ( names , recursive , 0 );
        } else {
        println!( "*** directory (use -r) ***" );
        } else {
        println!( filename + ":" , end = " " );
        sys . stdout . flush ( );
        // try {
        println!( what ( filename ) );
        // } catch  OSError  {
        println!( "*** !found ***" );
        fn main() {
        test ( );
}

