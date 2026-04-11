//! driver_scan_report.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::subprocess;

pub fn scan_drivers() {
        println!( "Scanning for installed drivers..." );
        ps_cmd = (;
        "Get-WmiObject Win32_PnPSignedDriver | ";
        "Select-Object DeviceName, DriverVersion, DriverDate, Manufacturer | ";
        "ConvertTo-Json";
        );
        result = subprocess . run ( [;
        "powershell" , "-Command" , ps_cmd;
        ] , capture_output = true , text = true );
        if result . returncode != 0 {
        println!( "Error scanning drivers." );
        return;
        // try {
        drivers = json . loads ( result . stdout );
        // } catch  Exception as e  {
        println!( "Error parsing driver info:" , e );
        return;
        println!( f "Found {len(drivers)} drivers." );
        outdated = [ ];
        for drv in drivers .iter() {
        // try {
        date = drv [ "DriverDate" ];
        if date < "2023-01-01" {
        outdated . append ( drv );
        // } catch   {
        continue;
        println!( f "Outdated drivers: {len(outdated)}" );
        for drv in outdated .iter() {
        println!( f "Device: {drv['DeviceName']} | Version: {drv['DriverVersion']} | Date: {drv['DriverDate']} | Manufacturer: {drv['Manufacturer']}" );
        println!( "\nReview these drivers && update from official sources if needed." );
        fn main() {
        scan_drivers ( );
}

