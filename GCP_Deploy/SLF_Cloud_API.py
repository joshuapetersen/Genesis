import os
import sqlite3
from fastapi import FastAPI, HTTPException
import uvicorn

app = FastAPI(title="SLF Cloud API")

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
AKASHIC_PATH = os.path.join(BASE_DIR, "SLF_Akashic_Records.sqlite")
VAULT_PATH = os.path.join(BASE_DIR, "SLF_Identity_Vault.sqlite")

@app.get("/logs")
def get_logs(last_log_id: int = 0):
    if not os.path.exists(AKASHIC_PATH):
        return {"logs": []}
    
    conn = sqlite3.connect(f"file:{AKASHIC_PATH}?mode=ro", uri=True)
    c = conn.cursor()
    c.execute("""
        SELECT event_id, timestamp, actor_name, event_type, description 
        FROM global_events 
        WHERE event_id > ?
        ORDER BY event_id ASC LIMIT 50
    """, (last_log_id,))
    rows = c.fetchall()
    conn.close()
    
    return {"logs": rows}

@app.get("/character/{entity_id}")
def get_character(entity_id: int):
    if not os.path.exists(VAULT_PATH):
        raise HTTPException(status_code=404, detail="Vault not found")
        
    conn = sqlite3.connect(f"file:{VAULT_PATH}?mode=ro", uri=True)
    c = conn.cursor()
    c.execute("SELECT entity_id, name, level, str, vit, int, wis, luk, genome, trauma_log FROM souls WHERE entity_id=?", (entity_id,))
    row = c.fetchone()
    conn.close()
    
    if not row:
        raise HTTPException(status_code=404, detail="Character not found")
        
    columns = ['id', 'species_name', 'level', 'str', 'vit', 'int', 'wis', 'luk', 'genome', 'trauma_log']
    char_data = dict(zip(columns, row))
    return char_data

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
