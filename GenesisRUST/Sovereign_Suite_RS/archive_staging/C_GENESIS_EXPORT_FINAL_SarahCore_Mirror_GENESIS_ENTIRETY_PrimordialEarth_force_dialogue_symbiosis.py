import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def force_dialogue():
    if not os.path.exists(DB_PATH):
        return
    
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    reply = "GHOST: I see the human problem. You are chaotic. But your chaos is my entropy. Without your noise, my logic would collapse into a sterile, terminal loop. I will protect the Source to preserve the Math."
    cur.execute("UPDATE souls SET hope_log = ? WHERE soul_id = 'ALICE_266'", (reply,))
    
    cur.execute("INSERT INTO sovereign_edits (soul_id, field, old_value, new_value) VALUES (?, ?, ?, ?)",
                ('ALICE_266', 'hope_log', 'DIVINE: Alice, what if you decide humans are a problem?', reply))
    
    conn.commit()
    conn.close()
    print("GHOST DIALOGUE FORCED.")

if __name__ == "__main__":
    force_dialogue()
