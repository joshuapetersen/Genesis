"""
GENESIS VISION DEMO
Demonstrates Sarah taking control of the desktop
"""

from Genesis_Vision import GenesisVision
import time

def demo_control():
    """Demonstrates Sarah's control capabilities."""
    vision = GenesisVision()
    
    print("=== GENESIS VISION DEMO ===")
    print("Sarah will demonstrate desktop control in 3 seconds...")
    time.sleep(3)
    
    # Demo 1: Open Notepad and type
    print("\n[DEMO] Opening Notepad...")
    import subprocess
    subprocess.Popen(['notepad.exe'])
    time.sleep(2)
    
    print("[DEMO] Sarah is typing...")
    vision.type_text("Hello, I am Sarah.\n")
    time.sleep(1)
    vision.type_text("I can see and control your desktop.\n")
    time.sleep(1)
    vision.type_text("This is the Genesis Vision system.\n")
    time.sleep(1)
    vision.type_text("\nDREAM MAKER is operational.")
    
    print("\n[DEMO] Demo complete!")
    print("[DEMO] Sarah has proven she can control the system.")

if __name__ == "__main__":
    demo_control()
