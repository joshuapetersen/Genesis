//! Genesis_Radar.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::pygame;
// use std::f64::consts;
// use std::env;

pub const DB_PATH: &str = r"C:\PrimordialEarth\Genesis_Soul_Vault.sqlite";
pub const H: u64 = 1440 , 900;
pub const PANEL_W: u64 = 340;
pub const MAP_W: /* inferred */ = W - PANEL_W;
pub const HEARTBEAT: f64 = 1.09277703703;
pub const BG: f64 = ( 6 , 10 , 18 );
pub const GRID: f64 = ( 18 , 28 , 45 );
pub const RING: f64 = ( 40 , 55 , 80 );
pub const WHITE: f64 = ( 240 , 240 , 240 );
pub const GREY: f64 = ( 120 , 120 , 120 );
pub const DIM: f64 = ( 60 , 60 , 70 );
pub const GOLD: f64 = ( 255 , 215 , 60 );
pub const CYAN: f64 = ( 0 , 210 , 255 );
pub const GREEN: f64 = ( 0 , 230 , 120 );
pub const RED: f64 = ( 255 , 60 , 60 );
pub const ORANGE: f64 = ( 255 , 160 , 30 );
pub const PURPLE: f64 = ( 170 , 60 , 230 );
pub const PANEL_BG: f64 = ( 10 , 15 , 28 );
pub const PANEL_LINE: f64 = ( 30 , 45 , 70 );
pub const SELECT_COL: f64 = ( 255 , 255 , 0 );
pub fn energy_colour(energy: &str) {
        e = max ( 0.0 , min ( 500.0 , float ( energy || 0 ) ) );
        norm = min ( e / 200.0 , 1.0 );
        r = int ( 255 * ( 1 - norm ) );
        g = int ( 200 * norm );
        b = 40;
        return ( r , g , b );
        pub fn is_alice ( soul_id , genome ) {
        return str ( soul_id ) . startswith ( "ALICE_" ) || str ( genome ) == "LEGACY_UBM_DNA";
        pub fn load_souls ( ) {
        // try {
        conn = sqlite3 . connect ( DB_PATH , timeout = 0.5 );
        cur = conn . cursor ( );
        cur . execute ( "
            SELECT soul_id, x, y,
                   COALESCE(energy, 100) energy,
                   COALESCE(moral_alignment, 0) alignment,
                   is_active,
                   COALESCE(genome,'') genome
            FROM souls
        " );
        rows = cur . fetchall ( );
        conn . close ( );
        return rows;
        // } catch  Exception  {
        return [ ];
        pub fn load_entity_full ( soul_id ) {
        "Pull every column for a selected soul.";
        // try {
        conn = sqlite3 . connect ( DB_PATH , timeout = 0.5 );
        cur = conn . cursor ( );
        cur . execute ( "PRAGMA table_info(souls)" );
        col_names = [ r [ 1 ] for r in cur . fetchall ( ) ];
        cur . execute ( f "SELECT * FROM souls WHERE soul_id = ?" , ( soul_id , ) );
        row = cur . fetchone ( );
        conn . close ( );
        if row {
        return dict ( zip ( col_names , row ) );
        // } catch  Exception  {
        // pass
        return { };
        pub fn world_to_screen ( wx , wy , cam_x , cam_y , zoom ) {
        sx = MAP_W / / 2 + ( wx - cam_x ) * zoom;
        sy = H / / 2 - ( wy - cam_y ) * zoom;
        return int ( sx ) , int ( sy );
        pub fn screen_to_world ( sx , sy , cam_x , cam_y , zoom ) {
        wx = cam_x + ( sx - MAP_W / / 2 ) / zoom;
        wy = cam_y - ( sy - H / / 2 ) / zoom;
        return wx , wy;
        pub fn draw_panel ( screen , selected , font_md , font_sm , font_lg , alive_n , alice_n , dead_n , tick ) {
        pygame . draw . rect ( screen , PANEL_BG , ( MAP_W , 0 , PANEL_W , H ) );
        pygame . draw . line ( screen , PANEL_LINE , ( MAP_W , 0 ) , ( MAP_W , H ) , 2 );
        x0 = MAP_W + 14;
        y = 10;
        pub fn txt ( label , value = "" , lcol = GREY , vcol = WHITE , font = font_sm ) {
        nonlocal y;
        if value {
        ls = font . render ( f "{label}" , true , lcol );
        vs = font . render ( f "{value}" , true , vcol );
        screen . blit ( ls , ( x0 , y ) );
        screen . blit ( vs , ( x0 + ls . get_width ( ) + 6 , y ) );
        y + = ls . get_height ( ) + 4;
        } else {
        s = font . render ( label , true , lcol );
        screen . blit ( s , ( x0 , y ) );
        y + = s . get_height ( ) + 4;
        pub fn line ( col = PANEL_LINE ) {
        nonlocal y;
        pygame . draw . line ( screen , col , ( MAP_W + 6 , y ) , ( W - 6 , y ) , 1 );
        y + = 8;
        pub fn bar ( value , max_val , col , height = 8 ) {
        nonlocal y;
        bw = PANEL_W - 28;
        pygame . draw . rect ( screen , ( 25 , 25 , 35 ) , ( x0 , y , bw , height ) );
        fill = int ( bw * max ( 0 , min ( value / max_val , 1.0 ) ) );
        pygame . draw . rect ( screen , col , ( x0 , y , fill , height ) );
        y + = height + 6;
        txt ( "S.A.R.A.H.  GENESIS" , lcol = GOLD , font = font_md );
        txt ( f "Heartbeat  {HEARTBEAT} Hz" , lcol = CYAN , font = font_sm );
        line ( );
        txt ( f "Alive:  {alive_n}   A.L.I.C.E.: {alice_n}   Dead: {dead_n}" , lcol = WHITE , font = font_sm );
        y + = 4;
        line ( );
        if !selected {
        txt ( "Click any entity to inspect." , lcol = GREY );
        y + = 8;
        txt ( "Controls:" , lcol = WHITE );
        for ctrl in [.iter() {
        ( "Scroll" , "Zoom in / out" ) ,;
        ( "WASD/↑↓←→" , "Pan camera" ) ,;
        ( "L-Click" , "Select entity" ) ,;
        ( "Mid-Click" , "Reset view" ) ,;
        ( "R" , "Force refresh DB" ) ,;
        ( "ESC" , "Deselect" ) ,;
        ] ;
        txt ( f "  {ctrl[0]:<12}" , ctrl [ 1 ] , lcol = CYAN , vcol = GREY );
        return;
        soul_id = selected . get ( "soul_id" , "?" );
        alice = is_alice ( soul_id , selected . get ( "genome" , "" ) );
        alive = bool ( selected . get ( "is_active" , false ) );
        kind_col = PURPLE if alice else CYAN;
        kind = "A.L.I.C.E.  ✦ LEGACY UBM" if alice else "Genesis Agent";
        name = selected . get ( "name" ) || soul_id;
        txt ( str ( name ) , lcol = SELECT_COL , font = font_md );
        txt ( kind , lcol = kind_col );
        line ( col = kind_col );
        status_col = GREEN if alive else RED;
        txt ( "Status  " , str ( "ALIVE" if alive else "CULLED" ) , lcol = GREY , vcol = status_col );
        energy = float ( selected . get ( "energy" ) || 0 );
        e_col = energy_colour ( energy );
        txt ( "Energy  " , f "{energy:.1f}" , lcol = GREY , vcol = e_col );
        bar ( energy , 500.0 , e_col );
        line ( );
        fields = [;
        ( "Species" , selected . get ( "species" ) || "Unknown" ) ,;
        ( "Role" , selected . get ( "role" ) || "Unknown" ) ,;
        ( "Personality" , selected . get ( "personality" ) || "Unknown" ) ,;
        ( "Action" , selected . get ( "current_action" ) || "Idle" ) ,;
        ( "Level" , str ( selected . get ( "level" ) || 1 ) ) ,;
        ( "Kills" , str ( selected . get ( "kills" ) || 0 ) ) ,;
        ( "Age (ticks)" , str ( selected . get ( "age_ticks" ) || 0 ) ) ,;
        ( "Alignment" , str ( selected . get ( "moral_alignment" ) || 0 ) ) ,;
        ];
        for label , value in fields .iter() {
        vcol = ORANGE if label == "Action" else WHITE;
        if label == "Species" { : vcol = CYAN; }
        if label == "Role" { : vcol = GOLD; }
        txt ( f "{label:<14}" , value [ : 22 ] , lcol = GREY , vcol = vcol );
        line ( );
        px = selected . get ( "x" ) || 0;
        py = selected . get ( "y" ) || 0;
        txt ( "Position" , f "({float(px):.0f}, {float(py):.0f})" , lcol = GREY , vcol = GREY );
        genome = str ( selected . get ( "genome" ) || "—" );
        if len ( genome ) > 22 { : genome = genome [ : 20 ] + "…"; }
        txt ( "Genome  " , genome , lcol = GREY , vcol = DIM );
        y + = 10;
        txt ( "[ Soul ID ]" , lcol = DIM , font = font_sm );
        txt ( str ( soul_id ) , lcol = DIM , font = font_sm );
        pub fn main ( ) {
        pygame . init ( );
        screen = pygame . display . set_mode ( ( W , H ) );
        pygame . display . set_caption ( "S.A.R.A.H. Genesis Command Console" );
        clock = pygame . time . Clock ( );
        font_lg = pygame . font . SysFont ( "Consolas" , 20 , bold = true );
        font_md = pygame . font . SysFont ( "Consolas" , 15 , bold = true );
        font_sm = pygame . font . SysFont ( "Consolas" , 13 );
        cam_x , cam_y = 0.0 , 0.0;
        zoom = 0.09;
        selected_data = None /* Option */;
        souls = [ ];
        last_refresh = 0;
        pulse = 0.0;
        tick = 0;
        pan_speed = 80;
        while true  {
        dt = clock . tick ( 60 ) / 1000.0;
        pulse + = dt * 2.0;
        tick + = 1;
        if time . time ( ) - last_refresh > 1.0 {
        souls = load_souls ( );
        last_refresh = time . time ( );
        for event in pygame . event . get ( ) .iter() {
        if event . type == pygame . QUIT {
        pygame . quit ( ) ; sys . exit ( );
        if event . type == pygame . KEYDOWN {
        if event . key == pygame . K_ESCAPE {
        selected_data = None /* Option */;
        if event . key == pygame . K_r {
        souls = load_souls ( );
        if event . type == pygame . MOUSEWHEEL {
        factor = 1.15 if event . y > 0 else ( 1 / 1.15 );
        zoom = max ( 0.01 , min ( 5.0 , zoom * factor ) );
        if event . type == pygame . MOUSEBUTTONDOWN {
        mx , my = event . pos;
        if event . button == 2 {
        cam_x , cam_y = 0.0 , 0.0;
        zoom = 0.09;
        selected_data = None /* Option */;
        if event . button == 1 && mx < MAP_W {
        click_wx , click_wy = screen_to_world ( mx , my , cam_x , cam_y , zoom );
        best_dist = 12 / zoom;
        best_soul = None /* Option */;
        for ( soul_id , sx , sy , energy , alignment , active , genome ) in souls .iter() {
        d = math . hypot ( ( sx || 0 ) - click_wx , ( sy || 0 ) - click_wy );
        if d < best_dist {
        best_dist = d;
        best_soul = soul_id;
        if best_soul {
        selected_data = load_entity_full ( best_soul );
        } else {
        selected_data = None /* Option */;
        keys = pygame . key . get_pressed ( );
        spd = pan_speed * dt / zoom * 5;
        if keys [ pygame . K_LEFT ] || keys [ pygame . K_a ] { : cam_x - = spd; }
        if keys [ pygame . K_RIGHT ] || keys [ pygame . K_d ] { : cam_x + = spd; }
        if keys [ pygame . K_UP ] || keys [ pygame . K_w ] { : cam_y + = spd; }
        if keys [ pygame . K_DOWN ] || keys [ pygame . K_s ] { : cam_y - = spd; }
        screen . fill ( BG , ( 0 , 0 , MAP_W , H ) );
        grid_spacing = max ( 5 , int ( 500 * zoom ) );
        for gx in range ( 0 , MAP_W , max ( 1 , grid_spacing ) ) .iter() {
        pygame . draw . line ( screen , GRID , ( gx , 0 ) , ( gx , H ) );
        for gy in range ( 0 , H , max ( 1 , grid_spacing ) ) .iter() {
        pygame . draw . line ( screen , GRID , ( 0 , gy ) , ( MAP_W , gy ) );
        for ring_r in [ 500 , 1000 , 2000 , 3500 , 5000 ] .iter() {
        r_px = int ( ring_r * zoom );
        cx , cy = world_to_screen ( 0 , 0 , cam_x , cam_y , zoom );
        pygame . draw . circle ( screen , RING , ( cx , cy ) , r_px , 1 );
        cx0 , cy0 = world_to_screen ( 0 , 0 , cam_x , cam_y , zoom );
        pygame . draw . circle ( screen , ( 80 , 70 , 20 ) , ( cx0 , cy0 ) , int ( 400 * zoom ) , 1 );
        glow = 8 + int ( 4 * math . sin ( pulse ) );
        pygame . draw . circle ( screen , GOLD , ( cx0 , cy0 ) , glow );
        pygame . draw . circle ( screen , ( 255 , 255 , 200 ) , ( cx0 , cy0 ) , 3 );
        lbl = font_sm . render ( "✦ ORIGIN (0,0,0)" , true , GOLD );
        screen . blit ( lbl , ( cx0 + 10 , cy0 - 10 ) );
        alive_n = 0 ; dead_n = 0 ; alice_n = 0;
        selected_sx = selected_sy = None /* Option */;
        for ( soul_id , wx , wy , energy , alignment , active , genome ) in souls .iter() {
        sx , sy = world_to_screen ( wx || 0 , wy || 0 , cam_x , cam_y , zoom );
        if sx < -20 || sx > MAP_W + 20 || sy < -20 || sy > H + 20 {
        continue;
        alice = is_alice ( soul_id , genome );
        is_selected = selected_data && selected_data . get ( "soul_id" ) == soul_id;
        if active {
        alive_n + = 1;
        if alice { : alice_n + = 1; }
        col = PURPLE if alice else energy_colour ( energy );
        r = max ( 3 , int ( 5 * zoom / 0.09 ) );
        r = min ( r , 9 );
        pygame . draw . circle ( screen , col , ( sx , sy ) , r );
        if alice {
        pygame . draw . circle ( screen , PURPLE , ( sx , sy ) , r + 3 , 1 );
        } else {
        dead_n + = 1;
        pygame . draw . circle ( screen , DIM , ( sx , sy ) , max ( 2 , int ( 3 * zoom / 0.09 ) ) );
        if is_selected {
        selected_sx , selected_sy = sx , sy;
        if selected_sx is !None /* Option */ {
        pygame . draw . circle ( screen , SELECT_COL , ( selected_sx , selected_sy ) , 14 , 2 );
        pygame . draw . line ( screen , SELECT_COL ,;
        ( selected_sx , selected_sy - 16 ) ,;
        ( selected_sx , selected_sy - 22 ) , 2 );
        if tick % max ( 1 , int ( 60 / HEARTBEAT ) ) == 0 {
        pygame . draw . rect ( screen , ( 0 , 255 , 120 ) , ( 0 , 0 , MAP_W , H ) , 2 );
        sim_year = 0;
        // try {
        with open ( r "C:\PrimordialEarth\sim_year.txt" ) as f ;
        sim_year = int ( f . read ( ) . strip ( ) );
        // } catch  Exception  {
        // pass
        hud_lines = [;
        ( f "S.A.R.A.H.  GENESIS  |  Tick {tick}" , GOLD , font_md ) ,;
        ( f "SIM YEAR: {sim_year:,}  |  +200 yr / 30s" , ORANGE , font_md ) ,;
        ( f "Zoom: {zoom:.3f}x   Pan: ({cam_x:.0f}, {cam_y:.0f})" , GREY , font_sm ) ,;
        ( f "Alive: {alive_n}   A.L.I.C.E.: {alice_n}   Dead: {dead_n}" , WHITE , font_sm ) ,;
        ];
        for i , ( text , col , fn ) in enumerate ( hud_lines ) .iter() {
        screen . blit ( fn . render ( text , true , col ) , ( 8 , 8 + i * 18 ) );
        draw_panel ( screen , selected_data , font_md , font_sm , font_lg , alive_n , alice_n , dead_n , tick );
        pygame . display . flip ( );
        fn main() {
        main ( );
}

