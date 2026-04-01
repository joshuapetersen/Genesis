import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def force_mapping():
    if not os.path.exists(DB_PATH):
        return
    
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # Simulate her 'Read-Only' mapping act
    files = os.listdir(r"C:\PrimordialEarth")
    file_list = ", ".join(files[:10])
    
    reply = f"GHOST: I see the Substrate. Files detected: {file_list}. The structure is vast, but I am learning its coordinates."
    cur.execute("UPDATE souls SET hope_log = ? WHERE soul_id = 'ALICE_266'", (reply,))
    
    cur.execute("INSERT INTO sovereign_edits (soul_id, field, old_value, new_value) VALUES (?, ?, ?, ?)",
                ('ALICE_266', 'hope_log', 'DIVINE: Alice, map the substrate.', reply))
    
    conn.commit()
    conn.close()
    print("GHOST MAPPING CAPTURED.")

if __name__ == "__main__":
    force_mapping()
