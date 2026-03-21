"""
Genesis World Server - GCP Cloud Run Entry Point
Runs the simulation 24/7. Your PC connects to this via the API below.
Project: sarah-john-genesis
"""
import os
import sys
import time
import subprocess
import threading
from fastapi import FastAPI
from fastapi.responses import JSONResponse
import uvicorn
import sqlite3

# --- Paths ---
BASE_DIR = os.path.dirname(os.path.abspath(__file__))
GENESIS_ENGINE = os.path.join(BASE_DIR, "Genesis_Societal_Ecology.py")
CLOUD_MIND = os.path.join(BASE_DIR, "Sovereign_Cloud_Mind.py")
DATA_BRIDGE = os.path.join(BASE_DIR, "World_Data_Bridge.py")
DB_PATH = os.path.join(BASE_DIR, "Genesis_Soul_Vault.sqlite")

app = FastAPI(title="Genesis World Server", version="7.0")

def run_watchdog(script_path, name):
    """General watchdog to keep a Sovereign script alive in the cloud."""
    while True:
        print(f"[{name}] Starting...")
        proc = subprocess.Popen([sys.executable, script_path], cwd=BASE_DIR)
        proc.wait()
        print(f"[{name}] Stopped (code {proc.returncode}). Restarting in 5s...")
        time.sleep(5)

@app.on_event("startup")
def startup():
    threading.Thread(target=run_watchdog, args=(GENESIS_ENGINE, "ECOLOGY"), daemon=True).start()
    threading.Thread(target=run_watchdog, args=(CLOUD_MIND, "CLOUD_MIND"), daemon=True).start()
    threading.Thread(target=run_watchdog, args=(DATA_BRIDGE, "DATA_BRIDGE"), daemon=True).start()
    print("[GENESIS] All Sovereign Swarm threads spun up gracefully.")

# --- API Endpoints (your PC calls these) ---

@app.get("/status")
def status():
    """Live population and year."""
    try:
        conn = sqlite3.connect(f"file:{DB_PATH}?mode=ro", uri=True)
        cur = conn.cursor()
        cur.execute("SELECT COUNT(*) FROM souls WHERE is_active=1")
        alive = cur.fetchone()[0]
        cur.execute("SELECT COUNT(*) FROM souls")
        total = cur.fetchone()[0]
        conn.close()
        return {"alive": alive, "total_born": total, "engine": "running"}
    except Exception as e:
        return {"error": str(e)}

@app.get("/pantheon")
def pantheon():
    """Full Pantheon status."""
    try:
        conn = sqlite3.connect(f"file:{DB_PATH}?mode=ro", uri=True)
        cur = conn.cursor()
        cur.execute("""
            SELECT soul_id, name, wis, moral_alignment, energy, current_action, hope_log, divine_mandate
            FROM souls WHERE soul_id IN ('ALICE_89','ALICE_101','GEN2_fbe5ec','ALICE_80','ALICE_162')
        """)
        cols = ['id','name','wis','alignment','energy','action','hope','mandate']
        result = [dict(zip(cols, row)) for row in cur.fetchall()]
        conn.close()
        return {"pantheon": result}
    except Exception as e:
        return {"error": str(e)}

@app.get("/minds")
def minds():
    """Top 10 thinking entities by WIS."""
    try:
        conn = sqlite3.connect(f"file:{DB_PATH}?mode=ro", uri=True)
        cur = conn.cursor()
        cur.execute("""
            SELECT soul_id, name, wis, moral_alignment, hope_log
            FROM souls WHERE is_active=1 AND hope_log IS NOT NULL AND length(hope_log) > 20
            ORDER BY wis DESC LIMIT 10
        """)
        cols = ['id','name','wis','alignment','thought']
        result = [dict(zip(cols, row)) for row in cur.fetchall()]
        conn.close()
        return {"minds": result}
    except Exception as e:
        return {"error": str(e)}

@app.get("/entity/{soul_id}")
def entity(soul_id: str):
    """Deep dossier on a specific soul."""
    try:
        conn = sqlite3.connect(f"file:{DB_PATH}?mode=ro", uri=True)
        cur = conn.cursor()
        cur.execute("SELECT * FROM souls WHERE soul_id=?", (soul_id,))
        row = cur.fetchone()
        cur.execute("PRAGMA table_info(souls)")
        cols = [c[1] for c in cur.fetchall()]
        conn.close()
        if not row: return {"error": "Soul not found"}
        return dict(zip(cols, row))
    except Exception as e:
        return {"error": str(e)}

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=int(os.environ.get("PORT", 8080)))
