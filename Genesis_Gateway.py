from fastapi import FastAPI, Request
from fastapi.templating import Jinja2Templates
from fastapi.staticfiles import StaticFiles
from fastapi.responses import HTMLResponse
import uvicorn
import os
import json
from Genesis_Cardinal import GenesisCardinal
from Genesis_Embryo_Shell import EmbryoShell

# --- GENESIS GATEWAY (The Game Server) ---
app = FastAPI(title="Genesis Visual Interface")

# Initialize The Engine
cardinal = GenesisCardinal()
embryo = EmbryoShell()
cardinal.boot_sequence()

# Setup Templates (The View)
templates = Jinja2Templates(directory="templates")

@app.get("/", response_class=HTMLResponse)
async def read_root(request: Request):
    """The Game Window"""
    return templates.TemplateResponse("genesis_world.html", {"request": request})

@app.get("/world/state")
async def get_world_state():
    """Returns the current 'Mana' and 'Balance' of the world."""
    cardinal.monitor_world_balance()
    return {
        "mana": cardinal.mana_status,
        "state": cardinal.world_state,
        "anchor": 1.09277703703703
    }

@app.get("/world/quests")
async def get_quests():
    """Returns active quests from the Cardinal."""
    cardinal.scan_for_quests(os.getcwd())
    return {"quests": cardinal.active_quests}

@app.get("/embryo/status")
async def get_embryo_status():
    """Returns the User's Avatar status."""
    embryo.load_history()
    return {
        "form": embryo.form,
        "xp": embryo.xp,
        "skills": embryo.skills
    }

@app.get("/map/structure")
async def get_map_structure():
    """
    Returns the File System as a 3D Map Structure.
    Folders = Districts
    Files = Buildings (Height = Size)
    """
    root_path = os.getcwd()
    structure = []
    
    for root, dirs, files in os.walk(root_path):
        # Limit depth for visualization sanity
        if root.count(os.sep) - root_path.count(os.sep) > 2:
            continue
            
        district = {
            "name": os.path.basename(root) or "Root",
            "path": root,
            "buildings": []
        }
        
        for file in files:
            try:
                size = os.path.getsize(os.path.join(root, file))
                district["buildings"].append({
                    "name": file,
                    "height": size, # Visual height based on bytes
                    "type": file.split('.')[-1] if '.' in file else "unknown"
                })
            except:
                pass
        
        structure.append(district)
        
    return {"map": structure}

if __name__ == "__main__":
    # Launch on Localhost Port 8000
    uvicorn.run(app, host="127.0.0.1", port=8000)
