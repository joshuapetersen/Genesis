use anyhow::Result;
use sovereign_path_re_seater::PathReSeater;

#[tokio::main(flavor = "multi_thread", worker_threads = 32)]
async fn main() -> Result<()> {
    println!("\x1b[93m============================================================\x1b[0m");
    println!("\x1b[93m  UNIVERSAL SOVEREIGN ENGINE - RE-SEATER [IGNITING]  \x1b[0m");
    println!("\x1b[93m  [Cross-Platform Re-Anchoring / 32-Thread Async] \x1b[0m");
    println!("\x1b[93m============================================================\x1b[0m");

    let re_seater = PathReSeater::new()?;
    re_seater.audit_and_fix().await?;

    Ok(())
}
