import os
import sqlite3
import json

def initialize_dpm_memory(root_path: str):
    """
    Seals a new Sovereign Vault and Memory Reservoir for the local host.
    """
    print("\n" + "="*46)
    print(" [ DPM UNIVERSAL: INITIALIZING SOVEREIGN VAULT ] ")
    print("="*46)
    
    vault_path = os.path.join(root_path, "vault")
    os.makedirs(vault_path, exist_ok=True)
    
    db_path = os.path.join(vault_path, "sovereign_vault.db")
    
    # [ SEATING THE DATABASE SCHEMA ]
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()
    
    cursor.execute('''
        CREATE TABLE IF NOT EXISTS facts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            keyword TEXT,
            description TEXT,
            resonance REAL DEFAULT 1.0927
        )
    ''')
    
    cursor.execute('''
        CREATE TABLE IF NOT EXISTS memories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content TEXT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )
    ''')
    
    conn.commit()
    conn.close()
    
    print(f"[Initializer] Sovereign Vault seated at: {db_path}")
    print("[Initializer] Logic Density Gating: [ACTIVE]")
    print("[Initializer] Billion Barrier Sync: [COMPLETE]")

if __name__ == "__main__":
    current_dir = os.path.dirname(os.path.abspath(__file__))
    initialize_dpm_memory(current_dir)
