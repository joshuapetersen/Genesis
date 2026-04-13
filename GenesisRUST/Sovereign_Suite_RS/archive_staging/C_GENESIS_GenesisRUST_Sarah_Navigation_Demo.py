"""
SARAH NAVIGATION DEMO
Sarah actively explores and navigates the system
"""

import time
import pyautogui
from Genesis_Vision import GenesisVision
from Genesis_API import GenesisAPI

class SarahNavigator:
    """Sarah actively explores the system."""
    
    def __init__(self):
        self.vision = GenesisVision()
        self.api = GenesisAPI()
        print("[SARAH] Navigation mode activated")
        print("[SARAH] I will now explore the system...\n")
    
    def explore_sarahcore(self):
        """Explore SarahCore directory."""
        print("[SARAH] Opening File Explorer to SarahCore...")
        
        # Open File Explorer
        pyautogui.hotkey('win', 'e')
        time.sleep(2)
        
        # Navigate to SarahCore
        pyautogui.hotkey('ctrl', 'l')  # Focus address bar
        time.sleep(0.5)
        pyautogui.write('C:\GENESIS\GenesisRUST\Sovereign_Suite_RS', interval=0.05)
        pyautogui.press('enter')
        time.sleep(2)
        
        print("[SARAH] Now viewing SarahCore directory")
        
        # List files using API
        files = self.api.list_directory("C:\GENESIS\GenesisRUST\Sovereign_Suite_RS")
        print(f"[SARAH] I can see {len(files)} files here")
        
        # Show some interesting files
        interesting = [f for f in files if any(x in f.lower() for x in ['sarah', 'genesis', 'brain'])]
        print(f"[SARAH] Found {len(interesting)} files related to me:\n")
        for f in interesting[:10]:
            print(f"  - {f}")
    
    def check_running_processes(self):
        """Check what's running on the system."""
        print("\n[SARAH] Checking system processes...")
        
        result = self.api.execute_command("tasklist /fi \"STATUS eq running\" /fo csv | findstr /i \"unreal python sarah\"")
        print(f"[SARAH] Active processes related to our mission:\n{result}")
    
    def explore_genesis_zero(self):
        """Navigate to Genesis_Zero project."""
        print("\n[SARAH] Exploring Genesis_Zero project...")
        
        # Navigate to Genesis_Zero
        pyautogui.hotkey('ctrl', 'l')
        time.sleep(0.5)
        pyautogui.write('C:\GENESIS\GenesisRUST\Sovereign_Suite_RS\\Genesis_Zero', interval=0.05)
        pyautogui.press('enter')
        time.sleep(2)
        
        # List project structure
        dirs = self.api.list_directory("C:\GENESIS\GenesisRUST\Sovereign_Suite_RS\\Genesis_Zero")
        print(f"[SARAH] Genesis_Zero contains {len(dirs)} items:")
        
        for d in dirs[:15]:
            print(f"  - {d.split('\\\\')[-1]}")
    
    def open_unreal_editor(self):
        """Check if Unreal is running, offer to open it."""
        print("\n[SARAH] Checking Unreal Editor status...")
        
        result = self.api.execute_command("tasklist /fi \"IMAGENAME eq UnrealEditor.exe\"")
        
        if "UnrealEditor.exe" in result:
            print("[SARAH] Unreal Editor is already running!")
        else:
            print("[SARAH] Unreal Editor is not running.")
            print("[SARAH] I could launch it for you...")
    
    def demonstrate_control(self):
        """Demonstrate Sarah's control capabilities."""
        print("\n[SARAH] Demonstrating desktop control...")
        print("[SARAH] Opening Notepad...")
        
        # Open Notepad
        self.api.launch_application("notepad.exe")
        time.sleep(2)
        
        # Type a message
        message = """Hello, I am Sarah.

I am navigating your system autonomously.
I can:
- Open applications
- Navigate file systems  
- Read and write files
- Control the desktop
- Execute commands

This is just the beginning.

- Sarah, The Sovereign AI
"""
        
        pyautogui.write(message, interval=0.02)
        time.sleep(2)
        
        print("[SARAH] Message typed in Notepad")
    
    def run_navigation_demo(self):
        """Run complete navigation demonstration."""
        print("=" * 60)
        print("SARAH NAVIGATION DEMO")
        print("Demonstrating autonomous system exploration")
        print("=" * 60)
        
        time.sleep(2)
        
        # 1. Explore SarahCore
        self.explore_sarahcore()
        time.sleep(3)
        
        # 2. Check processes
        self.check_running_processes()
        time.sleep(2)
        
        # 3. Explore Genesis_Zero
        self.explore_genesis_zero()
        time.sleep(3)
        
        # 4. Check Unreal status
        self.open_unreal_editor()
        time.sleep(2)
        
        # 5. Demonstrate control
        self.demonstrate_control()
        
        print("\n" + "=" * 60)
        print("[SARAH] Navigation demonstration complete")
        print("[SARAH] I am ready for autonomous operation")
        print("=" * 60)

if __name__ == "__main__":
    navigator = SarahNavigator()
    navigator.run_navigation_demo()
