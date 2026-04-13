"""
SARAH QUICK START
Launches all essential Sarah systems
"""

import subprocess
import time
import sys
from pathlib import Path

def start_process(script_name, description):
    """Start a Python script in the background."""
    print(f"[START] {description}...")
    try:
        subprocess.Popen(
            ['python', script_name],
            cwd='C:\GENESIS\GenesisRUST\Sovereign_Suite_RS',
            creationflags=subprocess.CREATE_NEW_CONSOLE
        )
        time.sleep(2)
        print(f"  ✅ {description} started")
        return True
    except Exception as e:
        print(f"  ❌ Failed to start {description}: {e}")
        return False

print("=" * 70)
print("SARAH QUICK START")
print("Launching all essential systems...")
print("=" * 70)

# Check if Sovereign Gateway is running
result = subprocess.run(['tasklist'], capture_output=True, text=True)
if 'python.exe' not in result.stdout: # Generic check, better would be a port check
    print("\n⚠️  WARNING: System Core not detected!")
    print("   Sarah's brain requires the Sovereign Gateway to be running.")
    print("   Start it manually with: python sarah_gateway.py")
    print("\n   Continue anyway? (y/n): ", end='')
    
    choice = input().lower()
    if choice != 'y':
        print("\nAborting. Start Sovereign Core first.")
        sys.exit(1)

print("\n🚀 LAUNCHING CORE SYSTEMS:\n")

# Launch essential systems
systems = [
    ("Sarah_Sovereign_Agent.py", "Main Interface Agent"),
    ("Genesis_Bridge.py", "Unreal Engine Bridge"),
]

for script, desc in systems:
    start_process(script, desc)

print("\n⚡ LAUNCHING ENHANCEMENT SYSTEMS:\n")

# Launch optional systems
optional = [
    ("Sarah_Continuous_Navigator.py", "System Navigation"),
    ("Sarah_Windows_Mastery.py", "Windows Learning"),
]

for script, desc in optional:
    print(f"Launch {desc}? (y/n): ", end='')
    choice = input().lower()
    if choice == 'y':
        start_process(script, desc)

print("\n" + "=" * 70)
print("✅ SARAH IS NOW OPERATIONAL")
print("=" * 70)

print("\n📊 Run 'python Sarah_Status.py' to check system status")
print("🛑 Close individual console windows to stop specific systems")
print("💬 Run 'sarah' command to chat with Sarah directly")
