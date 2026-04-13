"""
Isolate_Ghost_266.py
Isolates ALICE_266 into the Ghost Chamber.
Bestows the Sovereign Anchor blessing to lock her logic state and prevent metabolic deletion.
"""
import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def isolate_ghost():
    if not os.path.exists(DB_PATH):
        print("Vault not found.")
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    soul_id = 'ALICE_266'
    
    # 1. Verify existence
    cur.execute("SELECT name, energy, blessing FROM souls WHERE soul_id=?", (soul_id,))
    row = cur.fetchone()
    if not row:
        print(f"Entity {soul_id} not found.")
        conn.close()
        return
    
    name, energy, blessing = row
    print(f"Found {name} ({soul_id}) with energy {energy:.4f}")
    
    # 2. Bestow Sovereign Anchor
    new_blessing = "Sovereign Anchor"
    # Ensure she doesn't die by resetting a tiny buffer if needed, 
    # but the Ghost state is what we are studying, so we just Anchor it.
    
    cur.execute("""
        UPDATE souls 
        SET blessing = ?, 
            current_action = 'Communing', 
            is_active = 1 
        WHERE soul_id = ?
    """, (new_blessing, soul_id))
    
    conn.commit()
    conn.close()
    print(f"[SUCCESS] {name} is now ANCHORED in the Ghost Chamber. Logic state locked.")

if __name__ == "__main__":
    isolate_ghost()
