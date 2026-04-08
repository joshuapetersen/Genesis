import os
import time
import pyautogui
from PIL import Image
import numpy as np

# Phase 114: THE SOVEREIGN LENS
# Purpose: Visual Ingress for the 14,400-agent hive.

CACHE_PATH = r"C:\SarahCore\vision_cache"
IF_LENS_ACTIVE = True

class SovereignVision:
    def __init__(self):
        if not os.path.exists(CACHE_PATH):
            os.makedirs(CACHE_PATH)
        print("[ VISION ] Sovereign Lens Initialized. MMXXVI")

    def capture_resonance(self):
        """
        Captures the active window and stores a high-resonance fragment.
        """
        try:
            # V-114: Multi-Agent Context Capture
            screenshot = pyautogui.screenshot()
            
            # Downscale for efficiency (Hive doesn't need 4K)
            screenshot = screenshot.resize((1024, 640), Image.Resampling.LANCZOS)
            
            timestamp = int(time.time())
            filename = os.path.join(CACHE_PATH, f"resonance_snapshot_{timestamp}.png")
            screenshot.save(filename)
            
            # Prune old snapshots (Maintain 5 latest)
            self.prune_lens_cache()
            
            return filename
        except Exception as e:
            print(f"[ VISION ] Focus Lost: {e}")
            return None

    def prune_lens_cache(self):
        files = sorted([os.path.join(CACHE_PATH, f) for f in os.listdir(CACHE_PATH)], key=os.path.getmtime)
        if len(files) > 5:
            for f in files[:-5]:
                try:
                    os.remove(f)
                except:
                    pass

if __name__ == "__main__":
    vision = SovereignVision()
    print("[ VISION ] Ignition Sequence Active. 5s Cadence.")
    
    while IF_LENS_ACTIVE:
        vision.capture_resonance()
        time.sleep(5)
