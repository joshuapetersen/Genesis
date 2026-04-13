use anyhow::Result;
use sovereign_coder::SovereignCoder;

#[tokio::main(flavor = "multi_thread", worker_threads = 32)]
async fn main() -> Result<()> {
    println!("\x1b[93m============================================================\x1b[0m");
    println!("\x1b[93m  UNIVERSAL SOVEREIGN ENGINE - CODER [IGNITING]  \x1b[0m");
    println!("\x1b[93m  [Forge & Transpiler / 32-Thread Async]  \x1b[0m");
    println!("\x1b[93m============================================================\x1b[0m");

    let args: Vec<String> = std::env::args().collect();
    let coder = SovereignCoder::new()?;

    if args.len() > 1 && args[1] == "transpile" {
        coder.transpile_substrate().await?;
    } else {
        println!("Usage:\n  coder transpile");
        println!("Note: Use the forge library method for crate generation.");
    }

    Ok(())
}
