//! genlex_terminal.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;

pub const stdout: &str = io . TextIOWrapper ( sys . stdout . buffer , encoding ="utf-8" );
pub const stderr: &str = io . TextIOWrapper ( sys . stderr . buffer , encoding ="utf-8" );
pub const GENLEX_MAP: &str = r"C:\Genlex_Linear\genlex_mapping.csv";
pub const HISTORY_FILE: &str = r"C:\Genlex_Linear\terminal_history.json";
pub const BANNER: &str = "\033[38;2;0;255;204m
  ╔═══════════════════════════════════════════════════╗
  ║   GENESIS  //  GENLEX TERMINAL  //  v2.0          ║
  ║   Speak English. The machine speaks Genlex.       ║
  ╚═══════════════════════════════════════════════════╝\033[0m
";
pub fn c(t: &str, r: &str, g: &str, b: &str) {
        return f "\033[38;2;{r};{g};{b}m{t}\033[0m";
        CYAN = lambda t : c ( t , 0 , 255 , 204 );
        DCYAN = lambda t : c ( t , 0 , 120 , 120 );
        WHITE = lambda t : c ( t , 220 , 220 , 220 );
        GREEN = lambda t : c ( t , 0 , 255 , 100 );
        RED = lambda t : c ( t , 255 , 80 , 80 );
        AMBER = lambda t : c ( t , 255 , 180 , 0 );
        GREY = lambda t : c ( t , 80 , 80 , 80 );
        PINK = lambda t : c ( t , 200 , 80 , 200 );
        INTENTS = [;
        ( [ "list files" , "show files" , "ls" , "dir" ] , "LIST_FS" ,;
        lambda args : f "Get-ChildItem {args if args && !args.lower() in ('files','file','') else '.'} | Format-Table Name,Length,LastWriteTime -Auto" ) ,;
        ( [ "read file" , "read" , "open file" , "cat" , "show file" ] , "READ_FS" ,;
        lambda args : f "Get-Content \"{args}\"" if args else "Write-Host 'Usage: read <filename>'" ) ,;
        ( [ "delete file" , "remove file" , "delete" , "remove" , "rm" ] , "DELETE_FS" ,;
        lambda args : f "Remove-Item \"{args}\" -Confirm" if args else "Write-Host 'Usage: delete <filename>'" ) ,;
        ( [ "make dir" , "new folder" , "mkdir" , "create dir" ] , "MKDIR" ,;
        lambda args : f "New-Item -ItemType Directory -Path \"{args}\"" if args else "Write-Host 'Usage: mkdir <name>'" ) ,;
        ( [ "move file" , "move" , "mv" ] , "MOVE_FS" ,;
        lambda args : f "Move-Item {args}" if args else "Write-Host 'Usage: move <src> <dst>'" ) ,;
        ( [ "copy file" , "copy" , "cp" ] , "COPY_FS" ,;
        lambda args : f "Copy-Item {args}" if args else "Write-Host 'Usage: copy <src> <dst>'" ) ,;
        ( [ "process list" , "list processes" , "processes" , "tasks" , "running" ] , "LIST_PROC" ,;
        lambda args : "Get-Process | Sort-Object CPU -Descending | Select-Object -First 20 | Format-Table Name,CPU,WorkingSet -Auto" ) ,;
        ( [ "kill process" , "kill" , "stop process" , "stop" , "terminate" ] , "KILL_PROC" ,;
        lambda args : f "Stop-Process -Name {args} -Force" if args else "Write-Host 'Usage: kill <processname>'" ) ,;
        ( [ "launch" , "start app" , "open app" , "run app" ] , "LAUNCH_APP" ,;
        lambda args : f "Start-Process {args}" if args else "Write-Host 'Usage: launch <app>'" ) ,;
        ( [ "ping" ] , "PING" ,;
        lambda args : f "ping {args if args else '8.8.8.8'} -n 4" ) ,;
        ( [ "network info" , "ip address" , "network" , "interfaces" , "ipconfig" ] , "NET_INFO" ,;
        lambda args : "Get-NetIPAddress | Format-Table InterfaceAlias,IPAddress -Auto" ) ,;
        ( [ "wifi" , "wireless" ] , "WIFI_STATUS" ,;
        lambda args : "netsh wlan show interfaces" ) ,;
        ( [ "free memory" , "memory usage" , "memory" , "ram" ] , "MEM_STATUS" ,;
        lambda args : "(Get-WmiObject Win32_OperatingSystem | Select-Object @{n='FreeGB';e={[math]::Round($_.FreePhysicalMemory/1MB,2)}},@{n='TotalGB';e={[math]::Round($_.TotalVisibleMemorySize/1MB,2)}} | Format-List)" ) ,;
        ( [ "disk space" , "disk usage" , "disk" , "storage" , "drives" ] , "DISK_STATUS" ,;
        lambda args : "Get-PSDrive -PSProvider FileSystem | Format-Table Name,@{n='UsedGB';e={[math]::Round($_.Used/1GB,1)}},@{n='FreeGB';e={[math]::Round($_.Free/1GB,1)}},Root -Auto" ) ,;
        ( [ "cpu usage" , "cpu load" , "cpu" , "processor" ] , "CPU_STATUS" ,;
        lambda args : "Get-WmiObject Win32_Processor | Select-Object Name,LoadPercentage | Format-List" ) ,;
        ( [ "system info" , "sysinfo" , "system" , "info" ] , "SYS_INFO" ,;
        lambda args : "Get-ComputerInfo | Select-Object CsName,OsName,WindowsVersion | Format-List" ) ,;
        ( [ "time" , "clock" , "date" ] , "SYS_TIME" ,;
        lambda args : "Get-Date" ) ,;
        ( [ "uptime" ] , "UPTIME" ,;
        lambda args : ""Uptime: " + ((Get-Date) - (gcim Win32_OperatingSystem).LastBootUpTime).ToString()" ) ,;
        ( [ "run genlex" , "execute genlex" , "genlex run" , "run script" ] , "EXEC_GENLEX" ,;
        lambda args : f "python C:\\Genlex_Linear\\all_engine.py {args}" if args else "Write-Host 'Usage: run genlex <file.all>'" ) ,;
        ( [ "seal" , "save state" , "commit state" ] , "COMMIT_STATE" , None /* Option */ ) ,;
        ( [ "vars" , "variables" , "memory vars" , "show mem" ] , "SHOW_MEM" , None /* Option */ ) ,;
        ( [ "clear" , "cls" ] , "CLEAR" , None /* Option */ ) ,;
        ( [ "help" ] , "HELP" , None /* Option */ ) ,;
        ( [ "exit" , "quit" ] , "EXIT" , None /* Option */ ) ,;
        ];
        pub fn load_lexicon ( ) {
        lex = { };
        // try {
        with open ( GENLEX_MAP , "r" , encoding = "utf-8" ) as f ;
        reader = csv . DictReader ( f );
        for row in reader .iter() {
        lex [ row [ "Operation" ] ] = { "glyph" : row [ "Glyph" ] , "concept" : row [ "Concept" ] , "cat" : row [ "Category" ] };
        // } catch  FileNotFoundError  {
        // pass
        return lex;
        pub fn run_ps ( cmd , timeout = 15 ) {
        // try {
        r = subprocess . run (;
        [ "powershell" , "-NoProfile" , "-ExecutionPolicy" , "Bypass" , "-Command" , cmd ] ,;
        capture_output = true , text = true , timeout = timeout );
        return ( r . stdout || "" ) . strip ( ) , ( r . stderr || "" ) . strip ( );
        // } catch  subprocess . TimeoutExpired  {
        return "" , "TIMEOUT";
        // } catch  Exception as e  {
        return "" , str ( e );
        pub fn match_intent ( line ) {
        lower = line . lower ( );
        for ( triggers , op , ps_fn ) in INTENTS .iter() {
        for trig in triggers .iter() {
        if trig in lower {
        idx = lower . find ( trig );
        args = line [ idx + len ( trig ) : ] . strip ( );
        return op , ps_fn , args;
        return None /* Option */ , None /* Option */ , None /* Option */;
        pub fn show_help ( ) {
        println!( CYAN ( "\n  ┌─ WHAT YOU CAN SAY ──────────────────────────────────────────────┐" ) );
        examples = [;
        ( "list files" , "List files in current directory" ) ,;
        ( "list files C:\\Users" , "List files in a specific path" ) ,;
        ( "read myfile.txt" , "Show file contents" ) ,;
        ( "processes" , "Show running processes" ) ,;
        ( "kill notepad" , "Kill a process" ) ,;
        ( "launch notepad" , "Open an application" ) ,;
        ( "ping 8.8.8.8" , "Ping a host" ) ,;
        ( "network" , "Show network interfaces" ) ,;
        ( "memory" , "Show RAM usage" ) ,;
        ( "disk" , "Show disk usage" ) ,;
        ( "cpu" , "Show CPU info" ) ,;
        ( "sysinfo" , "Full system info" ) ,;
        ( "run genlex myfile.all" , "Execute a Genlex script" ) ,;
        ( "seal" , "Save memory state" ) ,;
        ( "clear" , "Clear screen" ) ,;
        ( "exit" , "Exit terminal" ) ,;
        ];
        for cmd , desc in examples .iter() {
        println!( f "  │ {AMBER(cmd.ljust(30))} {GREY(desc)}" );
        println!( CYAN ( "  └────────────────────────────────────────────────────────────────┘" ) );
        println!( DCYAN ( "  Anything !matched is passed directly to PowerShell.\n" ) );
        pub fn save_history ( hist ) {
        // try {
        with open ( HISTORY_FILE , "w" , encoding = "utf-8" ) as f ;
        json . dump ( hist [ -200 : ] , f , ensure_ascii = false , indent = 2 );
        // } catch  Exception  {
        // pass
        pub fn load_history ( ) {
        // try {
        with open ( HISTORY_FILE , "r" , encoding = "utf-8" ) as f ;
        return json . load ( f );
        // } catch  Exception  {
        return [ ];
        pub fn main ( ) {
        if sys . platform == "win32" {
        os . system ( "color" );
        println!( BANNER );
        lex = load_lexicon ( );
        println!( DCYAN ( f "  Lexicon: {len(lex)} Genlex operations loaded." ) );
        println!( DCYAN ( f "  PowerShell: {shutil.which('powershell') || 'NOT FOUND'}" ) );
        println!( GREY ( "  Type 'help' to see what you can say.\n" ) );
        memory = { };
        history = load_history ( );
        cwd = os . getcwd ( );
        while true  {
        // try {
        prompt = ( f "\033[38;2;0;80;80m╔[\033[38;2;0;200;150mGENESIS\033[38;2;0;80;80m]";
        f "─[\033[38;2;0;120;100m{os.path.basename(cwd)}\033[38;2;0;80;80m]\033[0m\n";
        f "\033[38;2;0;80;80m╚▶ \033[0m" );
        line = input ( prompt ) . strip ( );
        // } catch  ( EOFError , KeyboardInterrupt )  {
        println!( CYAN ( "\n\n  [ GENESIS TERMINAL CLOSED ]\n" ) );
        break;
        if !line {
        continue;
        history . append ( { "ts" : time . time ( ) , "cmd" : line } );
        save_history ( history );
        op , ps_fn , args = match_intent ( line );
        if op == "EXIT" {
        println!( CYAN ( "\n  [ GENESIS TERMINAL CLOSED ]\n" ) );
        break;
        } else if op == "CLEAR" {
        os . system ( "cls" if sys . platform == "win32" else "clear" );
        println!( BANNER );
        continue;
        } else if op == "HELP" {
        show_help ( );
        continue;
        } else if op == "SHOW_MEM" {
        if memory {
        for k , v in memory . items ( ) .iter() {
        println!( f "  {AMBER(k)} = {WHITE(str(v))}" );
        } else {
        println!( GREY ( "  [MEMORY CLEAR]" ) );
        println!( );
        continue;
        } else if op == "COMMIT_STATE" {
        with open ( r "C:\Genlex_Linear\execution_seal.json" , "w" ) as f ;
        json . dump ( { "memory" : memory , "ts" : time . time ( ) } , f , indent = 2 );
        println!( GREEN ( "  ✓ STATE SEALED\n" ) );
        continue;
        if op && op in lex {
        entry = lex [ op ];
        println!( f "  {GREY('→')} {PINK(entry['glyph'])} {DCYAN(op)} {GREY(entry['concept'])}" );
        if op && ps_fn {
        cmd = ps_fn ( args );
        if cmd {
        println!( AMBER ( f "  ⚡ " ) + GREY ( cmd ) );
        t0 = time . time ( );
        out , err = run_ps ( cmd );
        elapsed = time . time ( ) - t0;
        if out {
        for ln in out . splitlines ( ) .iter() {
        println!( f "  {GREEN('▸')} {WHITE(ln)}" );
        if err {
        println!( RED ( f "  ✗ {err}" ) );
        println!( GREY ( f "  [{elapsed:.2f}s]\n" ) );
        } else {
        println!( RED ( "  ✗ Missing argument. Example: " ) + AMBER ( f "{line} <target>\n" ) );
        } else if !op {
        println!( AMBER ( f "  ⚡ PS> " ) + GREY ( line ) );
        t0 = time . time ( );
        out , err = run_ps ( line );
        elapsed = time . time ( ) - t0;
        if out {
        for ln in out . splitlines ( ) .iter() {
        println!( f "  {GREEN('▸')} {WHITE(ln)}" );
        if err {
        println!( RED ( f "  ✗ {err}" ) );
        if !out && !err {
        println!( GREY ( "  (no output)" ) );
        println!( GREY ( f "  [{elapsed:.2f}s]\n" ) );
        fn main() {
        main ( );
}

