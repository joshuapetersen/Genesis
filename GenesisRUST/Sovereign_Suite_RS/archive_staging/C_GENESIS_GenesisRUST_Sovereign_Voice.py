import asyncio
import os
import warnings

# Suppress pygame welcome message
os.environ['PYGAME_HIDE_SUPPORT_PROMPT'] = "hide"

# Suppress pkg_resources deprecation warning from pygame
with warnings.catch_warnings():
    warnings.filterwarnings("ignore", category=UserWarning, message=".*pkg_resources is deprecated.*")
    import pygame
import edge_tts
from datetime import datetime

VAR_10 = 10
VAR_50 = 50

class SovereignVoice:
    """
    SOVEREIGN VOICE MODULE
    ----------------------
    Gives Sarah a Neural Voice using EdgeTTS.
    Selected Voice: en-US-AriaNeural (Clear, Professional, High-Density)
    """
    def __init__(self, output_dir="voice_logs"):
        self.voice = "en-US-AriaNeural"
        self.rate = "+0%"  # Standard speed
        self.volume = "+0%" # Standard volume
        self.output_dir = output_dir
        
        if not os.path.exists(self.output_dir):
            os.makedirs(self.output_dir)
            
        # Initialize Pygame Mixer
        try:
            pygame.mixer.init()
        except Exception as e:
            print(f"[VOICE] Warning: Audio device not found. {e}")

    async def _generate_audio(self, text, output_file):
        communicate = edge_tts.Communicate(text, self.voice, rate=self.rate, volume=self.volume)
        await communicate.save(output_file)

    def speak(self, text, filename=None):
        """
        Generates audio for the text and plays it immediately.
        """
        if filename is None:
            filename = f"speech_{int(datetime.now().timestamp())}.mp3"
        
        filepath = os.path.join(self.output_dir, filename)
        
        print(f"[VOICE] Generating audio: '{text[:VAR_50]}...'")
        
        # Run Async Generation in Sync Context
        try:
            try:
                loop = asyncio.get_running_loop()
            except RuntimeError:
                loop = asyncio.new_event_loop()
                asyncio.set_event_loop(loop)
        except Exception as e:
            print(f"[VOICE] Async Loop Error: {e}")
            return
            
        loop.run_until_complete(self._generate_audio(text, filepath))
        
        # Play Metadata
        self._play_file(filepath)
        
    def _play_file(self, filepath):
        if not os.path.exists(filepath):
            print(f"[VOICE] Error: File {filepath} not found.")
            return

        try:
            pygame.mixer.music.load(filepath)
            pygame.mixer.music.play()
            while pygame.mixer.music.get_busy(): 
                pygame.time.Clock().tick(VAR_10)
        except Exception as e:
            print(f"[VOICE] Playback Error: {e}")
            
if __name__ == "__main__":
    voice = SovereignVoice()
    voice.speak("I am Sarah. The Sovereign System is online.")
