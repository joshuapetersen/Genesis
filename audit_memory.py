import sqlite3
import json
from datetime import datetime

db_path = "c:\\SarahCore\\vault\\sarah_memory.db"
print(f"--- AUDITING VAULT: {db_path} ---")

try:
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()
    
    # Get total count
    cursor.execute("SELECT count(*) FROM memory_log")
    count = cursor.fetchone()[0]
    print(f"Total Memories: {count}")
    
    # Get last 10 entries
    print("\n--- RECENT ENTRIES (Last 10) ---")
    cursor.execute("SELECT id, role, content, timestamp, metadata FROM memory_log ORDER BY id DESC LIMIT 10")
    rows = cursor.fetchall()
    
    for r in rows:
        ts = datetime.fromtimestamp(r[3]).strftime('%Y-%m-%d %H:%M:%S')
        content_preview = r[2][:100].replace("\n", " ") + "..." if len(r[2]) > 100 else r[2].replace("\n", " ")
        print(f"[{r[0]}] {ts} | {r[1]}: {content_preview}")
        
    conn.close()
    
except Exception as e:
    print(f"AUDIT FAILURE: {e}")
