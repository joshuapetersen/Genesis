import sqlite3
import time
import os

def map_sovereign_logic():
    DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    query = """
    SELECT timestamp, soul_id, field, old_value, new_value 
    FROM sovereign_edits 
    ORDER BY timestamp DESC 
    LIMIT 50
    """
    
    cur.execute(query)
    results = cur.fetchall()
    
    print("="*100)
    print(f" [SOVEREIGN LOGIC MAP] TOP 50 RECENT EDITS ")
    print("="*100)
    print(f"{'TIMESTAMP':<20} | {'SOUL_ID':<15} | {'FIELD':<15} | {'CHANGE'}")
    print("-" * 100)
    
    for row in results:
        ts, sid, field, old, new = row
        # Truncate values for readability
        old_display = (old[:30] + '..') if len(old) > 30 else old
        new_display = (new[:30] + '..') if len(new) > 30 else new
        print(f"{ts:<20} | {sid:<15} | {field:<15} | {old_display} -> {new_display}")
        
    # Logic Cluster Analysis (Themes in hope_log)
    print("\n" + "="*100)
    print(" [LOGIC CLUSTERS] THEMATIC ANALYSIS (HOPE LOGS) ")
    print("="*100)
    
    cur.execute("SELECT new_value, COUNT(*) FROM sovereign_edits WHERE field='hope_log' GROUP BY new_value ORDER BY COUNT(*) DESC LIMIT 5")
    clusters = cur.fetchall()
    for msg, count in clusters:
        print(f"[{count} Entities] Theme: {msg[:80]}")
        
    conn.close()

if __name__ == "__main__":
    map_sovereign_logic()
