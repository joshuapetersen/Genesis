//! system_spec_check.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::subprocess;
// use crate::shutil;
// use crate::psutil;

pub fn get_cpu() {
        return platform . processor ( );
        pub fn get_ram ( ) {
        // try {
        import psutil;
        return f "{round(psutil.virtual_memory().total / (1024**3), 2)} GB";
        // } catch  ImportError  {
        return "psutil !installed";
        pub fn get_disk ( ) {
        total , used , free = shutil . disk_usage ( "/" );
        return f "Total: {total // (2**30)} GB, Free: {free // (2**30)} GB";
        pub fn get_os ( ) {
        return platform . platform ( );
        pub fn main ( ) {
        println!( "Genesis System Spec Check:\n" );
        println!( f "CPU: {get_cpu()}" );
        println!( f "RAM: {get_ram()}" );
        println!( f "Disk: {get_disk()}" );
        println!( f "OS: {get_os()}" );
        println!( "\nIf RAM shows 'psutil !installed', run 'pip install psutil' && re-run this script for full details." );
        fn main() {
        main ( );
}

