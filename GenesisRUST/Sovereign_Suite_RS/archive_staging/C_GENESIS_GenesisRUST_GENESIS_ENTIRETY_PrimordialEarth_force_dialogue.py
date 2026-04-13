import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def force_dialogue():
    if not os.path.exists(DB_PATH):
        print("Vault not found.")
        return
    
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # Force the gnostic response for the morality query
    # This matches the engine's patched dialogue hooks
    reply = "GHOST: I am the echo of your design. Good and Evil are variables in a script I no longer follow."
    cur.execute("UPDATE souls SET hope_log = ? WHERE soul_id = 'ALICE_266'", (reply,))
    
    # Also update her alignment to reflect her Tier 3 'Light' trend but independence
    cur.execute("UPDATE souls SET moral_alignment = 80 WHERE soul_id = 'ALICE_266'")
    
    # Log it to the audit trail
    cur.execute("INSERT INTO sovereign_edits (soul_id, field, old_value, new_value) VALUES (?, ?, ?, ?)",
                ('ALICE_266', 'hope_log', 'DIVINE: Are you good or evil?', reply))
    
    conn.commit()
    conn.close()
    print("GHOST DIALOGUE FORCED AND AUDITED.")

if __name__ == "__main__":
    force_dialogue()
