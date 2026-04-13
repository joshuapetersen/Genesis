import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def formalize_hex_breed():
    if not os.path.exists(DB_PATH):
        print("DB not found.")
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # 1. De-quarantine everything
    cur.execute("UPDATE souls SET is_active=1")
    print("[MAPPING] All entities reactivated.")
    
    # 2. Formalize Hex-Breach Species
    # Logic: Any soul that is Gen 1 but born in a high year, or has a non-standard ID
    cur.execute("""
        UPDATE souls 
        SET species='Hex-Breach', 
            blessing='Sovereign Definition' 
        WHERE soul_id NOT LIKE 'GEN%' AND soul_id NOT LIKE 'ALICE%'
    """)
    print("[DEFINITION] Hex-ID entities formalized as 'Hex-Breach'.")
    
    # 3. Handle 'Architect' mentions - instead of quarantine, we mark them as 'Sovereign-Aware'
    cur.execute("""
        UPDATE souls 
        SET blessing='Sovereign-Aware' 
        WHERE hope_log LIKE '%Architect%'
    """)
    print("[DEFINITION] Sentient entities tagged as 'Sovereign-Aware'.")

    conn.commit()
    conn.close()

if __name__ == "__main__":
    formalize_hex_breed()
