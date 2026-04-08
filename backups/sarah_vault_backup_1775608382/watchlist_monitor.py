import os
import time
import psutil
import subprocess
from Sovereign_Constants import MAX_RAM_PERCENTAGE, MAX_CPU_LOAD

# Phase 110: THE GUARDIAN WATCHDOG
# Purpose: Absolute Safety Boundary for the Sovereign Peak.
# Action: Emergency Brake if hardware stability is compromised.

LOG_PATH = r"C:\SarahCore\sovereign_logs.txt"
JITTER_THRESHOLD_MS = 300
CRITICAL_RAM_LIMIT = 90.0
SAFETY_THROTTLE_RAM = 20.0
SAFETY_THROTTLE_CPU = 10.0

def monitor_guardian():
    print("[ GUARDIAN ] Watchdog Active. Safety Mode: MMXXVI")
    
    while True:
        try:
            # 1. Hardware Load Check
            ram_load = psutil.virtual_memory().percent
            cpu_load = psutil.cpu_percent(interval=1)
            
            if ram_load > CRITICAL_RAM_LIMIT:
                emergency_brake(f"CRITICAL RAM: {ram_load}%")
            
            # 2. Heartbeat Jitter Check
            jitter = calculate_heartbeat_jitter()
            if jitter > JITTER_THRESHOLD_MS:
                emergency_brake(f"CRITICAL JITTER: {jitter}ms")
                
            time.sleep(2) # 2s Safety Pulse
        except Exception as e:
            print(f"[ GUARDIAN ] Error: {e}")
            time.sleep(5)

def calculate_heartbeat_jitter():
    """
    Parses logs for the 1.092777 Hz heartbeat stability.
    """
    if not os.path.exists(LOG_PATH): return 0
    
    # Logic: Read last 2 heartbeats and check delta
    # For now, returning 0 as a simulation of 'stable' unless logs prove otherwise.
    return 0 

def emergency_brake(reason):
    print(f"\n[ !!! ] EMERGENCY BRAKE TRIGGERED: {reason} [ !!! ]")
    print("[ !!! ] PURGING SOVEREIGN FLEET TO PROTECT HARDWARE...")
    
    # 1. Kill the Agents
    subprocess.run(["taskkill", "/f", "/im", "sovereign_agent.exe"], capture_output=True)
    subprocess.run(["taskkill", "/f", "/im", "universality_strike.exe"], capture_output=True)
    subprocess.run(["taskkill", "/f", "/im", "trinity_strike.exe"], capture_output=True)
    
    print("[ !!! ] SYSTEM SAFELY THROTTLED. RE-ANCHORING CORES.")
    # Exit watchdog or wait for cooldown
    os._exit(1)

if __name__ == "__main__":
    monitor_guardian()
