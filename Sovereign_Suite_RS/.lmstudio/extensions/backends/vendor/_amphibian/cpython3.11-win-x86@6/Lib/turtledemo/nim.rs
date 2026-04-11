//! nim.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::turtle;
// use std::time;

pub const SCREENWIDTH: u64 = 640;
pub const SCREENHEIGHT: u64 = 480;
pub const MINSTICKS: u64 = 7;
pub const MAXSTICKS: u64 = 31;
pub const HUNIT: /* inferred */ = SCREENHEIGHT / / 12;
pub const WUNIT: /* inferred */ = SCREENWIDTH / / ( ( MAXSTICKS / / 5 ) * 11 + ( MAXSTICKS % 5 ) * 2 );
pub const SCOLOR: f64 = ( 63 , 63 , 31 );
pub const HCOLOR: f64 = ( 255 , 204 , 204 );
pub const COLOR: f64 = ( 204 , 204 , 255 );
pub fn randomrow() {
        return  random . randint ( MINSTICKS , MAXSTICKS );
        pub fn computerzug ( state )  {
        xored = state [ 0 ] ^ state [ 1 ] ^ state [ 2 ];
        if xored == 0 {
        return  randommove ( state );
        for z in range ( 3 ) .iter() {
        s = state [ z ] ^ xored;
        if s <= state [ z ] {
        move = ( z , s );
        return  move;
        pub fn randommove ( state )  {
        m = max ( state );
        while true  {
        z = random . randint ( 0 , 2 );
        if state [ z ] > ( m > 1 ) {
        break;
        rand = random . randint ( m > 1 , state [ z ] -1 );
        return  z , rand;
        class NimModel ( object ) ;
        pub fn __init__ ( &self, game )  {
        self . game = game;
        pub fn setup ( self )  {
        if self . game . state !in [ Nim . CREATED , Nim . OVER ] {
        return;
        self . sticks = [ randomrow ( ) , randomrow ( ) , randomrow ( ) ];
        self . player = 0;
        self . winner = None /* Option */;
        self . game . view . setup ( );
        self . game . state = Nim . RUNNING;
        pub fn move ( &self, row , col )  {
        maxspalte = self . sticks [ row ];
        self . sticks [ row ] = col;
        self . game . view . notify_move ( row , col , maxspalte , self . player );
        if self . game_over ( ) {
        self . game . state = Nim . OVER;
        self . winner = self . player;
        self . game . view . notify_over ( );
        } else if self . player == 0 {
        self . player = 1;
        row , col = computerzug ( self . sticks );
        self . move ( row , col );
        self . player = 0;
        pub fn game_over ( self )  {
        return  self . sticks == [ 0 , 0 , 0 ];
        pub fn notify_move ( &self, row , col )  {
        if self . sticks [ row ] <= col {
        return;
        self . move ( row , col );
        class Stick ( turtle . Turtle ) ;
        pub fn __init__ ( &self, row , col , game )  {
        turtle . Turtle . __init__ ( self , visible = false );
        self . row = row;
        self . col = col;
        self . game = game;
        x , y = self . coords ( row , col );
        self . shape ( "square" );
        self . shapesize ( HUNIT / 10.0 , WUNIT / 20.0 );
        self . speed ( 0 );
        self . pu ( );
        self . goto ( x , y );
        self . color ( "white" );
        self . showturtle ( );
        pub fn coords ( &self, row , col )  {
        packet , remainder = divmod ( col , 5 );
        x = ( 3 + 11 * packet + 2 * remainder ) * WUNIT;
        y = ( 2 + 3 * row ) * HUNIT;
        return  x - SCREENWIDTH / / 2 + WUNIT / / 2 , SCREENHEIGHT / / 2 - y - HUNIT / / 2;
        pub fn makemove ( &self, x , y )  {
        if self . game . state != Nim . RUNNING {
        return;
        self . game . controller . notify_move ( self . row , self . col );
        class NimView ( object ) ;
        pub fn __init__ ( &self, game )  {
        self . game = game;
        self . screen = game . screen;
        self . model = game . model;
        self . screen . colormode ( 255 );
        self . screen . tracer ( false );
        self . screen . bgcolor ( ( 240 , 240 , 255 ) );
        self . writer = turtle . Turtle ( visible = false );
        self . writer . pu ( );
        self . writer . speed ( 0 );
        self . sticks = { };
        for row in range ( 3 ) .iter() {
        for col in range ( MAXSTICKS ) .iter() {
        self . sticks [ ( row , col ) ] = Stick ( row , col , game );
        self . display ( "... a moment please ..." );
        self . screen . tracer ( true );
        pub fn display ( &self, msg1 , msg2 = None /* Option */ )  {
        self . screen . tracer ( false );
        self . writer . clear ( );
        if msg2 is !None /* Option */ {
        self . writer . goto ( 0 , - SCREENHEIGHT / / 2 + 48 );
        self . writer . pencolor ( "red" );
        self . writer . write ( msg2 , align = "center" , font = ( "Courier" , 18 , "bold" ) );
        self . writer . goto ( 0 , - SCREENHEIGHT / / 2 + 20 );
        self . writer . pencolor ( "black" );
        self . writer . write ( msg1 , align = "center" , font = ( "Courier" , 14 , "bold" ) );
        self . screen . tracer ( true );
        pub fn setup ( self )  {
        self . screen . tracer ( false );
        for row in range ( 3 ) .iter() {
        for col in range ( self . model . sticks [ row ] ) .iter() {
        self . sticks [ ( row , col ) ] . color ( SCOLOR );
        for row in range ( 3 ) .iter() {
        for col in range ( self . model . sticks [ row ] , MAXSTICKS ) .iter() {
        self . sticks [ ( row , col ) ] . color ( "white" );
        self . display ( "Your turn! Click leftmost stick to remove." );
        self . screen . tracer ( true );
        pub fn notify_move ( &self, row , col , maxspalte , player )  {
        if player == 0 {
        farbe = HCOLOR;
        for s in range ( col , maxspalte ) .iter() {
        self . sticks [ ( row , s ) ] . color ( farbe );
        } else {
        self . display ( " ... thinking ...         " );
        time . sleep ( 0.5 );
        self . display ( " ... thinking ... aaah ..." );
        farbe = COLOR;
        for s in range ( maxspalte -1 , col -1 , -1 ) .iter() {
        time . sleep ( 0.2 );
        self . sticks [ ( row , s ) ] . color ( farbe );
        self . display ( "Your turn! Click leftmost stick to remove." );
        pub fn notify_over ( self )  {
        if self . game . model . winner == 0 {
        msg2 = "Congrats. You're the winner!!!";
        } else {
        msg2 = "Sorry, the computer == the winner.";
        self . display ( "To play again press space bar. To leave press ESC." , msg2 );
        pub fn clear ( self )  {
        if self . game . state == Nim . OVER {
        self . screen . clear ( );
        class NimController ( object ) ;
        pub fn __init__ ( &self, game )  {
        self . game = game;
        self . sticks = game . view . sticks;
        self . BUSY = false;
        for stick in self . sticks . values ( ) .iter() {
        stick . onclick ( stick . makemove );
        self . game . screen . onkey ( self . game . model . setup , "space" );
        self . game . screen . onkey ( self . game . view . clear , "Escape" );
        self . game . view . display ( "Press space bar to start game" );
        self . game . screen . listen ( );
        pub fn notify_move ( &self, row , col )  {
        if self . BUSY {
        return;
        self . BUSY = true;
        self . game . model . notify_move ( row , col );
        self . BUSY = false;
        class Nim ( object ) ;
        CREATED = 0;
        RUNNING = 1;
        OVER = 2;
        pub fn __init__ ( &self, screen )  {
        self . state = Nim . CREATED;
        self . screen = screen;
        self . model = NimModel ( self );
        self . view = NimView ( self );
        self . controller = NimController ( self );
        pub fn main ( )  {
        mainscreen = turtle . Screen ( );
        mainscreen . mode ( "standard" );
        mainscreen . setup ( SCREENWIDTH , SCREENHEIGHT );
        nim = Nim ( mainscreen );
        return  "EVENTLOOP";
        fn main() {
        main ( );
        turtle . mainloop ( );
}

