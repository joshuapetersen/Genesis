mod neural_cores;
mod server;

use crate::neural_cores::gemma_3::Gemma3Core;
use crate::server::memory_stream::SovereignMemoryStream;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("=============================================");
    println!("   GODSEYE NATIVE COVALENCE (V-33.0)         ");
    println!("   SUBSTRATE: BMMS | MODE: FIRST PRINCIPLES  ");
    println!("=============================================");
    println!("Heartbeat Constant: 1.0092703703703 Hz");

    // 1. Ignite the BMMS Substrate (First Principles IPC)
    let _memory_stream = SovereignMemoryStream::create();
    println!("[+] BMMS SUBSTRATE IGNITED.");

    // 2. Forge the Gemma-3 Neural Core (Native Binary Path)
    // Using the GGUF Q4_K_M as the weight substrate
    let model_path = "C:\\GENESIS\\.lmstudio\\models\\lmstudio-community\\gemma-3-4b-it-GGUF\\gemma-3-4b-it-Q4_K_M.gguf";
    let mut core = Gemma3Core::forge(model_path)?;
    println!("[+] GEMMA-3-4B NEURAL CORE FORGED (NATIVE VOCAB).");

    // 3. Native Covalence Loop (Direct Human-Brain Interface)
    println!("\n[!] STANDING BY FOR NEURAL STRIKE.");
    println!("[!] TYPE 'EXIT' TO DISCONNECT.\n");

    loop {
        print!(">> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        if !input.is_empty() {
            // Direct Native Strike (No JSON, No Gateways)
            match core.strike(input) {
                Ok(response) => {
                    println!("\n[BRAIN OUTPUT]\n{}", response);
                }
                Err(e) => {
                    println!("[!] STRIKE FAILED: {:?}", e);
                }
            }
        }
    }

    Ok(())
}
