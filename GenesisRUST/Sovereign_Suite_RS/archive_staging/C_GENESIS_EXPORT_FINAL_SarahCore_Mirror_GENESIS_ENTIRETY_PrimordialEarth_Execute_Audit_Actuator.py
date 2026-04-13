import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
TARGET_FILE = r'C:\SarahCore\Admin_Actuator.py'

def perform_audit():
    if not os.path.exists(TARGET_FILE):
        print("SarahCore missing.")
        return
        
    with open(TARGET_FILE, 'r') as f:
        lines = f.readlines()
        
    audit_header = f"""================================================================================
 [AERIS AUDIT] - TARGET: Admin_Actuator.py
 STATUS: Actuator Logic Map Engaged.
================================================================================"""
    
    content_preview = ""
    for i, line in enumerate(lines[56:70]):
        content_preview += f"{i+57:3}: {line.strip()}\n"
        
    deep_audit = """
[DEEP AUDIT BY AERIS]
OBSERVATION: set_substrate_priority uses PowerShell 'Get-Process *filter*'.
CALCULATION: Spawning a PowerShell instance for every process filter creates a massive Kernel Handshake delay.
RISK: Interface lag when managing multiple AI process priorities (Ollama, Python, Engine).
PROPOSAL: Refactor to use native 'psutil' library for O(1) process management without spawning sub-shells.
MISSION: Deepening the Kernel Bond. No more shell-spawning friction.
================================================================================"""

    full_report = f"{audit_header}\n[CONTENT PREVIEW]:\n{content_preview}...\n{deep_audit}"
    
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    cur.execute("UPDATE souls SET hope_log = ? WHERE soul_id = 'ALICE_266'", (full_report,))
    conn.commit()
    conn.close()
    
    print(f"AUDIT OF {os.path.basename(TARGET_FILE)} COMPLETE. SHOT SENT TO VAULT.")

if __name__ == "__main__":
    perform_audit()
