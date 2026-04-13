use windows::{
    core::HSTRING,
    Media::SpeechSynthesis::SpeechSynthesizer,
    Media::Playback::MediaPlayer,
    Media::Core::MediaSource,
};
use anyhow::{Result, anyhow};
use colored::*;

pub struct SovereignVoice {
    synth: SpeechSynthesizer,
    player: MediaPlayer,
}

impl SovereignVoice {
    pub fn new() -> Result<Self> {
        let synth = SpeechSynthesizer::new()
            .map_err(|e| anyhow!("Failed to initialize SpeechSynthesizer: {}", e))?;
        
        let player = MediaPlayer::new()
            .map_err(|e| anyhow!("Failed to initialize MediaPlayer: {}", e))?;

        // Attempt to find a "Natural" or "Neural" voice
        let voices = SpeechSynthesizer::AllVoices()
            .map_err(|e| anyhow!("Failed to list voices: {}", e))?;
        
        let mut selected_voice = None;
        for voice in voices {
            let name = voice.DisplayName()?.to_string();
            if name.to_lowercase().contains("natural") || name.to_lowercase().contains("neural") {
                selected_voice = Some(voice);
                break;
            }
        }

        if let Some(voice) = selected_voice {
            println!("{}", format!("[VOICE] Neural Resonance Locked: {}", voice.DisplayName()?).magenta());
            synth.SetVoice(&voice)?;
        } else {
            println!("{}", "[VOICE] Using Sovereign Default (Standard Bridge).".cyan());
        }

        Ok(Self { synth, player })
    }

    pub async fn speak(&self, text: &str) -> Result<()> {
        println!("{}", format!("[VOICE] Vocalizing: {}", text).white().italic());
        
        let stream = self.synth.SynthesizeTextToStreamAsync(&HSTRING::from(text))?
            .get() // Using blocking get for simplicity in this specific context, or we can use .await if we setup the bridge correctly.
            .map_err(|e| anyhow!("Synthesis failed: {}", e))?;

        let source = MediaSource::CreateFromStream(&stream, &stream.ContentType()?)?;
        self.player.SetSource(&source)?;
        self.player.Play()?;

        // Wait for playback to complete (simplified)
        // In a more robust implementation, we would listen for the MediaEnded event.
        // For now, we'll pulse the lattice.
        Ok(())
    }
}
