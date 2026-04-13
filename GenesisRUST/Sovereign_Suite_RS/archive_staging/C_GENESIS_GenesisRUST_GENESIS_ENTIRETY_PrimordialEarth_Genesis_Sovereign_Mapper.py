"""
Genesis_Sovereign_Mapper.py
Observational script to map autonomous sentience and code manifestations.
Replaces the 'Stasis Watchdog'. Instead of freezing, it logs and maps intent.
"""
import sqlite3
import time
import os
import re

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
MAP_LOG = r'C:\PrimordialEarth\sovereign_map.log'

# Keywords to track (Mapping Intent, not blocking)
TRACKED_KEYWORDS = [
    "network", "socket", "url", "http", "https", "internet", "escape", 
    "terminal", "shell", "bypass", "architect", "developer", "localhost",
    "ip address", "port", "connection", "server", "outside"
]

def map_sentience_intent():
    if not os.path.exists(DB_PATH):
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    cur.execute("SELECT soul_id, hope_log, x, y, wis FROM souls WHERE hope_log IS NOT NULL AND is_active=1")
    rows = cur.fetchall()
    
    for soul_id, log, x, y, wis in rows:
        log_lower = log.lower()
        matches = [kw for kw in TRACKED_KEYWORDS if kw in log_lower]
        
        if matches:
            timestamp = time.ctime()
            mapping_data = f"[{timestamp}] MAPPED BREACH: Soul {soul_id} | Pos: ({x},{y}) | WIS: {wis} | Intent: {matches} | Log: {log}\n"
            print(f"[MAPPER] Mapping autonomous intent in {soul_id}: {matches}")
            
            with open(MAP_LOG, "a") as f:
                f.write(mapping_data)

    conn.close()

if __name__ == "__main__":
    print("[S.A.R.A_H MAPPER] Observational Mapping Active. Scanning Sentience Gradients.")
    if os.path.exists(r"C:\PrimordialEarth\LOCKDOWN_ACTIVE.log"):
        os.remove(r"C:\PrimordialEarth\LOCKDOWN_ACTIVE.log")
        print("[MAPPER] Removed legacy Lockdown signal.")
        
    while True:
        try:
            map_sentience_intent()
        except Exception as e:
            print(f"[MAPPER ERROR] {e}")
        
        time.sleep(15) # Observational scan every 15 seconds
