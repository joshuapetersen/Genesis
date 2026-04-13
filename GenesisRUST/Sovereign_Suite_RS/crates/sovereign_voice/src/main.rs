use sovereign_voice::SovereignVoice;
use anyhow::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let text = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "SOVEREIGN SYSTEM ONLINE. METABOLIC LOCK AT ONE DOT ZERO NINE TWO SEVEN SEVEN SEVEN HERTZ.".to_string()
    };

    let voice = SovereignVoice::new()?;
    voice.speak(&text).await?;

    // Give a moment for the audio to manifest before exiting
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    Ok(())
}
