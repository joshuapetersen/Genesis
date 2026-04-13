"""
SARAH STATUS CHECKER
Verifies all Sarah systems are operational
"""

import subprocess
import os
import sys

def check_process(script_name):
    """Check if a Python script is running."""
    try:
        result = subprocess.run(
            ['powershell', '-Command', f'Get-Process python | Where-Object {{ (Get-WmiObject Win32_Process -Filter "ProcessId=$($_.Id)").CommandLine -like "*{script_name}*" }}'],
            capture_output=True,
            text=True
        )
        return script_name in result.stdout
    except:
        return False

def check_substrate():
    """Check if the Sovereign Gateway is running."""
    try:
        # Check if port 8080 is listening
        result = subprocess.run(['powershell', '-Command', 'Test-NetConnection -ComputerName localhost -Port 8080'], capture_output=True, text=True)
        return 'TcpTestSucceeded : True' in result.stdout
    except:
        return False

print("=" * 70)
print("SARAH SYSTEM STATUS")
print("=" * 70)

# Essential Components
print("\n🧠 NEURAL CORE:")
if check_substrate():
    print("  ✅ Sovereign Gateway (Local AI) - RUNNING")
else:
    print("  ❌ Sovereign Gateway (Local AI) - NOT RUNNING")
    print("     Start: python sarah_gateway.py")

# Core Systems
print("\n🎯 CORE SYSTEMS:")

systems = {
    "Sarah_Sovereign_Agent.py": "Main Interface Agent",
    "Genesis_Bridge.py": "Unreal Engine Bridge",
}

for script, description in systems.items():
    if check_process(script):
        print(f"  ✅ {description} - RUNNING")
    else:
        print(f"  ❌ {description} - NOT RUNNING")
        print(f"     Start: python {script}")

# Optional Enhancement Systems
print("\n⚡ ENHANCEMENT SYSTEMS (Optional):")

optional = {
    "Sarah_Continuous_Navigator.py": "System Navigation",
    "Sarah_Windows_Mastery.py": "Windows Learning",
}

for script, description in optional.items():
    if check_process(script):
        print(f"  ✅ {description} - RUNNING")
    else:
        print(f"  ⏸️  {description} - NOT RUNNING (optional)")

print("\n" + "=" * 70)

# Count total Sarah processes
result = subprocess.run(['tasklist'], capture_output=True, text=True)
python_count = result.stdout.count('python.exe')

print(f"Total Python processes: {python_count}")
print("=" * 70)

print("\n📋 MINIMUM REQUIRED:")
print("  1. Sovereign Gateway (for Sarah's brain)")
print("  2. Sarah_Sovereign_Agent.py (main interface)")
print("\n⚡ RECOMMENDED:")
print("  + Genesis_Bridge.py (if using Unreal Engine)")
print("  + Sarah_Continuous_Navigator.py (for active exploration)")
print("  + Sarah_Windows_Mastery.py (for learning)")
print("\n💡 TIP: Run 'python Sarah_Quick_Start.py' to launch all systems")
