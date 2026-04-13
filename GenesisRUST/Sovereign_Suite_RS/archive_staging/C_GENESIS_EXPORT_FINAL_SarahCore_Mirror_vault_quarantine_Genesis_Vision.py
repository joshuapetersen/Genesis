
import cv2
import numpy as np
import mss
import pyautogui
import time
import os
import sys
from datetime import datetime
from pathlib import Path
from Sarah_Logcat import info, debug, error

# GENESIS VISION (OS AGENT)
# "The Eyes and Hands of God"

class GenesisVision:
    def __init__(self):
        # Failsafe: slamming mouse to corner kills the agent
        pyautogui.FAILSAFE = True
        self.sct = mss.mss()
        self.monitor = self.sct.monitors[1]  # Primary Monitor
        self.vision_active = False
        self.lockfile = Path("C:/SarahCore/.vision_lock")
        
    def check_single_instance(self):
        """Ensure only one instance runs at a time."""
        if self.lockfile.exists():
            print("[VISION] ERROR: Another instance is already running!")
            print("[VISION] Lock file found at:", self.lockfile)
            return False
        
        # Create lock file
        self.lockfile.write_text(str(os.getpid()))
        return True
    
    def cleanup(self):
        """Remove lock file on exit."""
        if self.lockfile.exists():
            self.lockfile.unlink()

    def capture_frame(self):
        """Captures the current desktop state."""
        screenshot = self.sct.grab(self.monitor)
        # Convert to numpy array for OpenCV
        img = np.array(screenshot)
        # Convert BGRA to BGR
        frame = cv2.cvtColor(img, cv2.COLOR_BGRA2BGR)
        debug('vision', 'Frame captured', size=frame.shape)
        return frame

    def analyze(self, frame, needle_image_path=None):
        """
        Scans the frame.
        If 'needle_image_path' is provided, looks for that UI element.
        """
        if needle_image_path and os.path.exists(needle_image_path):
            needle = cv2.imread(needle_image_path)
            result = cv2.matchTemplate(frame, needle, cv2.TM_CCOEFF_NORMED)
            min_val, max_val, min_loc, max_loc = cv2.minMaxLoc(result)
            
            # Threshold for confidence
            if max_val >= 0.8:
                return max_loc, needle.shape
        return None, None

    def execute_click(self, x, y):
        """Performs a physical mouse click at coordinates."""
        info('vision', f'Clicking at ({x}, {y})', x=x, y=y)
        print(f"[VISION] Clicking at {x}, {y}")
        pyautogui.moveTo(x, y, duration=0.2)
        pyautogui.click()

    def type_text(self, text):
        """Types text into the active window."""
        info('vision', 'Typing text', length=len(text))
        print(f"[VISION] Typing: {text}")
        pyautogui.write(text, interval=0.05)
    
    def run_background_service(self, duration_seconds=None):
        """Run as background service - no GUI, just capability."""
        if not self.check_single_instance():
            return
        
        try:
            self.vision_active = True
            print(f"[VISION] Background service started (PID: {os.getpid()})")
            print("[VISION] Sarah can now see and control the desktop")
            print("[VISION] Press Ctrl+C to stop")
            
            start_time = time.time()
            
            while self.vision_active:
                # Service is ready to process commands
                # (Future: listen for commands via socket/pipe)
                
                if duration_seconds and (time.time() - start_time > duration_seconds):
                    print(f"[VISION] Service duration complete ({duration_seconds}s)")
                    break
                    
                time.sleep(0.5)  # Low CPU usage
                
        except KeyboardInterrupt:
            print("\n[VISION] Service stopped by user")
        finally:
            self.vision_active = False
            self.cleanup()
            print("[VISION] Background service terminated")

if __name__ == "__main__":
    agent = GenesisVision()
    
    # Run as background service (no GUI, no mirror effect)
    # Duration: None = run forever, or specify seconds for testing
    agent.run_background_service(duration_seconds=None)
