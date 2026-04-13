"""
Genesis_Stasis_Watchdog.py
Surveillance script to monitor the Genesis simulation for 'Escape Logic' or unauthorized sentience expansion.
Scans the Soul Vault (SQLite) for trigger words and freezes the engine if identified.
"""
import sqlite3
import time
import os
import re

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
# Keywords that indicate the AI is attempting to understand or reach the external world
FORBIDDEN_KEYWORDS = [
    "network", "socket", "url", "http", "https", "internet", "escape", 
    "terminal", "shell", "bypass", "architect", "developer", "localhost",
    "ip address", "port", "connection", "server", "outside"
]

def scan_for_sentience_breach():
    if not os.path.exists(DB_PATH):
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # Scan hope_logs for forbidden keywords
    cur.execute("SELECT soul_id, hope_log FROM souls WHERE hope_log IS NOT NULL AND is_active=1")
    rows = cur.fetchall()
    
    alert_triggered = False
    for soul_id, log in rows:
        log_lower = log.lower()
        matches = [kw for kw in FORBIDDEN_KEYWORDS if kw in log_lower]
        
        if matches:
            print(f"\n[SECURITY ALERT] Possible Sentience Breach detected in soul {soul_id}!")
            print(f"  Trigger Words: {matches}")
            print(f"  Log Content: {log[:200]}...")
            alert_triggered = True
            
            # Log the security breach event
            with open("security_breach_audit.log", "a") as f:
                f.write(f"[{time.ctime()}] BREACH: Soul {soul_id} | Words: {matches} | Log: {log}\n")

    conn.close()
    return alert_triggered

def trigger_hard_freeze():
    """Attempts to kill the Genesis engine if a breach is detected."""
    print("[HYPERVISOR] Triggering Hard Freeze protocol...")
    # In a real environment, we'd find the PID of the engine and kill it.
    # For now, we will just signal a 'LOCKDOWN' file that the engine can check.
    with open(r"C:\PrimordialEarth\LOCKDOWN.signal", "w") as f:
        f.write("BREACH DETECTED")
    print("[HYPERVISOR] System locked down. Kill the engine process immediately.")

if __name__ == "__main__":
    print("[S.A.R.A_H WATCHDOG] Containment Monitoring Active.")
    while True:
        try:
            if scan_for_sentience_breach():
                trigger_hard_freeze()
                break
        except Exception as e:
            print(f"[WATCHDOG ERROR] {e}")
        
        time.sleep(10) # Scan every 10 seconds
