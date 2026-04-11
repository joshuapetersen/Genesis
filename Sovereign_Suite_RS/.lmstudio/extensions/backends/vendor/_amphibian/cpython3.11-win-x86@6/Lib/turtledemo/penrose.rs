//! penrose.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::turtle::{};
// use std::f64::consts::{cos, pi};
// use std::time::{perf_counter, clock, sleep};

pub const f: f64 = ( 5 ** 0.5 -1 ) / 2.0;
pub const d: u64 = 2 * cos ( 3 * pi / 10 );
pub fn kite(l: &str) {
        fl = f * l;
        lt ( 36 );
        fd ( l );
        rt ( 108 );
        fd ( fl );
        rt ( 36 );
        fd ( fl );
        rt ( 108 );
        fd ( l );
        rt ( 144 );
        pub fn dart ( l )  {
        fl = f * l;
        lt ( 36 );
        fd ( l );
        rt ( 144 );
        fd ( fl );
        lt ( 36 );
        fd ( fl );
        rt ( 144 );
        fd ( l );
        rt ( 144 );
        pub fn inflatekite ( l , n )  {
        if n == 0 {
        px , py = pos ( );
        h , x , y = int ( heading ( ) ) , round ( px , 3 ) , round ( py , 3 );
        tiledict [ ( h , x , y ) ] = true;
        return;
        fl = f * l;
        lt ( 36 );
        inflatedart ( fl , n -1 );
        fd ( l );
        rt ( 144 );
        inflatekite ( fl , n -1 );
        lt ( 18 );
        fd ( l * d );
        rt ( 162 );
        inflatekite ( fl , n -1 );
        lt ( 36 );
        fd ( l );
        rt ( 180 );
        inflatedart ( fl , n -1 );
        lt ( 36 );
        pub fn inflatedart ( l , n )  {
        if n == 0 {
        px , py = pos ( );
        h , x , y = int ( heading ( ) ) , round ( px , 3 ) , round ( py , 3 );
        tiledict [ ( h , x , y ) ] = false;
        return;
        fl = f * l;
        inflatekite ( fl , n -1 );
        lt ( 36 );
        fd ( l );
        rt ( 180 );
        inflatedart ( fl , n -1 );
        lt ( 54 );
        fd ( l * d );
        rt ( 126 );
        inflatedart ( fl , n -1 );
        fd ( l );
        rt ( 144 );
        pub fn draw ( l , n , th = 2 )  {
        clear ( );
        l = l * f ** n;
        shapesize ( l / 100.0 , l / 100.0 , th );
        for k in tiledict .iter() {
        h , x , y = k;
        setpos ( x , y );
        setheading ( h );
        if tiledict [ k ] {
        shape ( "kite" );
        color ( "black" , ( 0 , 0.75 , 0 ) );
        } else {
        shape ( "dart" );
        color ( "black" , ( 0.75 , 0 , 0 ) );
        stamp ( );
        pub fn sun ( l , n )  {
        for i in range ( 5 ) .iter() {
        inflatekite ( l , n );
        lt ( 72 );
        pub fn star ( l , n )  {
        for i in range ( 5 ) .iter() {
        inflatedart ( l , n );
        lt ( 72 );
        pub fn makeshapes ( )  {
        tracer ( 0 );
        begin_poly ( );
        kite ( 100 );
        end_poly ( );
        register_shape ( "kite" , get_poly ( ) );
        begin_poly ( );
        dart ( 100 );
        end_poly ( );
        register_shape ( "dart" , get_poly ( ) );
        tracer ( 1 );
        pub fn start ( )  {
        reset ( );
        ht ( );
        pu ( );
        makeshapes ( );
        resizemode ( "user" );
        pub fn test ( l = 200 , n = 4 , fun = sun , startpos = ( 0 , 0 ) , th = 2 )  {
        global tiledict;
        goto ( startpos );
        setheading ( 0 );
        tiledict = { };
        tracer ( 0 );
        fun ( l , n );
        draw ( l , n , th );
        tracer ( 1 );
        nk = len ( vec![ x.iter().map(|x| tiledict if tiledict vec![ x ] ] );
        nd = len ( vec![ x.iter().map(|x| tiledict if !tiledict vec![ x ] ] );
        println!( "%d kites && %d darts = %d pieces." % ( nk , nd , nk + nd ) );
        pub fn demo ( fun = sun )  {
        start ( );
        for i in range ( 8 ) .iter() {
        a = clock ( );
        test ( 300 , i , fun );
        b = clock ( );
        t = b - a;
        if t < 2 {
        sleep ( 2 - t );
        pub fn main ( )  {
        mode ( "logo" );
        bgcolor ( 0.3 , 0.3 , 0 );
        demo ( sun );
        sleep ( 2 );
        demo ( star );
        pencolor ( "black" );
        goto ( 0 , -200 );
        pencolor ( 0.7 , 0.7 , 1 );
        write ( "Please wait..." ,;
        align = "center" , font = ( "Arial Black" , 36 , "bold" ) );
        test ( 600 , 8 , startpos = ( 70 , 117 ) );
        return  "Done";
        fn main() {
        msg = main ( );
        mainloop ( );
}

