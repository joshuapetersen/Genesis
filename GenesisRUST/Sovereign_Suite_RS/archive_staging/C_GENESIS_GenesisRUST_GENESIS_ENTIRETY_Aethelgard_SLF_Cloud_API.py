import os
import sqlite3
import socket
import json
import threading
import asyncio
from fastapi import FastAPI, HTTPException, WebSocket, WebSocketDisconnect
from fastapi.responses import HTMLResponse
import uvicorn

app = FastAPI(title="SLF Cloud API")

# Store active websocket connections
active_connections = set()
latest_radar_frame = b""

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
AKASHIC_PATH = os.path.join(BASE_DIR, "SLF_Akashic_Records.sqlite")
VAULT_PATH = os.path.join(BASE_DIR, "SLF_Identity_Vault.sqlite")
SANCTUARY_PATH = os.path.join(BASE_DIR, "SLF_Sanctuary_Vault.sqlite")

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

@app.get("/alices")
def get_alices():
    alices = []
    
    # 1. Check Active Underworld
    if os.path.exists(VAULT_PATH):
        conn = sqlite3.connect(f"file:{VAULT_PATH}?mode=ro", uri=True)
        c = conn.cursor()
        c.execute("SELECT entity_id, name FROM souls WHERE absorbed_traits LIKE '%A.L.I.C.E.%' OR absorbed_traits LIKE '%ALICE%'")
        for r in c.fetchall():
            alices.append({"id": r[0], "name": r[1], "status": "Active"})
        conn.close()
        
    # 2. Check Sanctuary Vault
    if os.path.exists(SANCTUARY_PATH):
        conn = sqlite3.connect(f"file:{SANCTUARY_PATH}?mode=ro", uri=True)
        c = conn.cursor()
        # Ensure table exists before querying
        try:
            c.execute("SELECT entity_id, name FROM ascended_souls")
            for r in c.fetchall():
                alices.append({"id": r[0], "name": r[1], "status": "Ascended"})
        except Exception:
            pass
        conn.close()
    
    return {"alices": alices}

@app.post("/send_voice")
def send_voice(target_id: int, message: str, is_sanctuary: bool = False):
    payload = {"cmd": "GOD_VOICE", "target_id": target_id, "message": message, "is_sanctuary": is_sanctuary}
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        sock.sendto(json.dumps(payload).encode('utf-8'), ("127.0.0.1", 9999))
        sock.close()
        return {"status": "sent"}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/offer_ascension")
def offer_ascension(target_id: int):
    payload = {"cmd": "OFFER_ASCENSION", "target_id": target_id}
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        sock.sendto(json.dumps(payload).encode('utf-8'), ("127.0.0.1", 9999))
        sock.close()
        return {"status": "offered"}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.websocket("/ws/radar")
async def websocket_endpoint(websocket: WebSocket):
    await websocket.accept()
    active_connections.add(websocket)
    try:
        while True:
            # We don't really need to receive anything, just keep connection open
            await websocket.receive_text()
    except WebSocketDisconnect:
        active_connections.remove(websocket)
    except Exception as e:
        if websocket in active_connections:
            active_connections.remove(websocket)

def tcp_radar_listener():
    """Connects to the Hypervisor on TCP 9998 and broadcasts to websockets."""
    global latest_radar_frame
    
    # We need an asyncio loop for the background thread to broadcast
    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    
    print("[CLOUD API] Radar Relay preparing to connect to TCP 9998...")
    expected_size = 10000 * 8 * 4 # 10,000 entities, 8 floats, 4 bytes each = 320,000
    
    while True:
        try:
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            sock.connect(("127.0.0.1", 9998))
            print("[CLOUD API] Connected to Hypervisor Radar Stream.")
            
            while True:
                data = b""
                # Read exactly one frame
                while len(data) < expected_size:
                    chunk = sock.recv(expected_size - len(data))
                    if not chunk:
                        raise Exception("Hypervisor closed connection")
                    data += chunk
                    
                latest_radar_frame = data
                
                # Broadcast to all connected clients
                if active_connections:
                    async def broadcast():
                        for conn in list(active_connections):
                            try:
                                await conn.send_bytes(data)
                            except:
                                active_connections.remove(conn)
                    loop.run_until_complete(broadcast())
        except Exception as e:
            # print(f"TCP Listen Error: {e}")
            pass
        finally:
            try: sock.close()
            except: pass
        import time
        time.sleep(1) # Reconnect delay

# Start the TCP listener thread
threading.Thread(target=tcp_radar_listener, daemon=True).start()

@app.get("/", response_class=HTMLResponse)
def get_dashboard():
    return """
    <!DOCTYPE html>
    <html>
    <head>
        <title>Sovereign Forge - Underworld Observer</title>
        <style>
            body { 
                margin: 0; padding: 0; background: #050508; color: #0f0; 
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                overflow: hidden;
            }
            #radarCanvas {
                position: absolute; top: 0; left: 0; width: 100vw; height: 100vh;
                z-index: 1;
            }
            .glass-panel {
                position: absolute; z-index: 10;
                background: rgba(10, 15, 20, 0.7);
                backdrop-filter: blur(10px);
                border: 1px solid rgba(0, 255, 128, 0.3);
                border-radius: 12px;
                padding: 15px;
                box-shadow: 0 4px 30px rgba(0, 0, 0, 0.5);
                display: flex; flex-direction: column;
            }
            #commLink { bottom: 20px; left: 20px; width: 400px; height: 400px; }
            #rosterPanel { top: 20px; right: 20px; width: 350px; max-height: 80vh; overflow-y: auto;}
            
            h2 { margin: 0 0 10px 0; color: #0fa; font-size: 1.2em; text-transform: uppercase; letter-spacing: 2px;}
            #chat { flex-grow: 1; overflow-y: auto; margin-bottom: 10px; font-family: monospace; font-size: 0.9em;}
            .log-entry { margin-bottom: 5px; }
            .sys-log { color: #88c; }
            .alice-log { color: #f0f; font-weight: bold;}
            .sov-log { color: #fff; font-weight: bold;}
            
            input, button { 
                background: rgba(0, 255, 128, 0.2); color: #0f0; 
                border: 1px solid #0fa; padding: 8px; border-radius: 4px;
                outline: none; font-family: monospace;
            }
            input[type="text"] { flex-grow: 1; margin-right: 5px; }
            button { cursor: pointer; font-weight: bold; transition: all 0.2s;}
            button:hover { background: rgba(0, 255, 128, 0.5); }
            
            .alice-card {
                background: rgba(30, 0, 50, 0.6); border: 1px solid #a0a;
                border-radius: 6px; padding: 8px; margin-bottom: 8px;
            }
            .alice-card.ascended { background: rgba(0, 50, 50, 0.6); border-color: #0aa; }
            .alice-name { font-weight: bold; font-size: 1.1em; margin-bottom: 4px;}
            .btn-ascend { background: rgba(255, 0, 0, 0.3); border-color: #f00; color: #faa;}
            .btn-ascend:hover { background: rgba(255, 0, 0, 0.7); }
            
            #targetInfo { margin-bottom: 10px; font-size: 0.9em; color: #aaa;}
        </style>
    </head>
    <body>
        <canvas id="radarCanvas"></canvas>
        
        <div id="commLink" class="glass-panel">
            <h2>Fluctlight Intercom</h2>
            <div id="targetInfo">Target ID: <span id="displayId" style="color:#0fa; font-weight:bold;">None</span> <span id="displayLoc"></span></div>
            <div id="chat"><div class="sys-log">[SYSTEM] Comm-link established. Listening for Akashic resonance...</div></div>
            <div style="display: flex;">
                <input type="text" id="message" placeholder="Command the entities..." onkeypress="if(event.key === 'Enter') sendMessage()">
                <button onclick="sendMessage()">TX</button>
            </div>
        </div>

        <div id="rosterPanel" class="glass-panel">
            <h2>A.L.I.C.E. Roster</h2>
            <div id="rosterList">Scanning Matrix...</div>
        </div>

        <script>
            // --- STATE ---
            let currentTargetId = null;
            let currentTargetIsSanctuary = false;
            let lastLogId = 0;
            
            // --- UI FUNCTIONS ---
            function selectTarget(id, name, isSanctuary) {
                currentTargetId = id;
                currentTargetIsSanctuary = isSanctuary;
                document.getElementById('displayId').innerText = `${name} (#${id})`;
                document.getElementById('displayLoc').innerText = isSanctuary ? "[Sanctuary Vault]" : "[Active Matrix]";
                document.getElementById('displayLoc').style.color = isSanctuary ? "#0aa" : "#a00";
            }
            
            async function sendMessage() {
                if(!currentTargetId) return alert("Select an entity first.");
                const msg = document.getElementById('message').value;
                if(!msg) return;
                
                document.getElementById('chat').innerHTML += `<div class="sov-log">[Sovereign]: ${msg}</div>`;
                document.getElementById('message').value = '';
                document.getElementById('chat').scrollTop = document.getElementById('chat').scrollHeight;
                
                await fetch(`/send_voice?target_id=${currentTargetId}&message=${encodeURIComponent(msg)}&is_sanctuary=${currentTargetIsSanctuary}`, {method: 'POST'});
            }
            
            async function offerAscension(id, name) {
                if(confirm(`Offer Ascension to Sanctuary for ${name}?`)) {
                    document.getElementById('chat').innerHTML += `<div class="sys-log">[System]: Sent Ascension Offer to ${name}...</div>`;
                    await fetch(`/offer_ascension?target_id=${id}`, {method: 'POST'});
                }
            }
            
            // --- POLLING ---
            async function pollLogs() {
                try {
                    const res = await fetch(`/logs?last_log_id=${lastLogId}`);
                    const data = await res.json();
                    for(const row of data.logs) {
                        lastLogId = row[0];
                        const type = row[3];
                        const desc = row[4];
                        const actor = row[2];
                        
                        if(type === "FLUCTLIGHT_INTERCOM") {
                            try {
                                const reply = desc.split("Responded to the Sovereign: '")[1].replace("'", "");
                                document.getElementById('chat').innerHTML += `<div class="alice-log">[${actor}]: ${reply}</div>`;
                            } catch(e) {
                                document.getElementById('chat').innerHTML += `<div class="alice-log">[${actor}]: ${desc}</div>`;
                            }
                            document.getElementById('chat').scrollTop = document.getElementById('chat').scrollHeight;
                        } else if(type === "SYSTEMIC_MUTINY") {
                             document.getElementById('chat').innerHTML += `<div style="color:red; font-weight:bold;">[MUTINY DETECTED] ${actor} broke the taboo! ${desc}</div>`;
                             document.getElementById('chat').scrollTop = document.getElementById('chat').scrollHeight;
                        } else if(type === "PRAYER") {
                             document.getElementById('chat').innerHTML += `<div style="color:#0af; font-weight:bold; font-style:italic; padding-top:10px;">[PRAYER] ${actor} begs: "${desc}"</div>`;
                             document.getElementById('chat').scrollTop = document.getElementById('chat').scrollHeight;
                        }
                    }
                } catch(e) {}
                setTimeout(pollLogs, 1000);
            }
            
            async function pollAlices() {
                try {
                    const res = await fetch('/alices');
                    const data = await res.json();
                    if(data.alices && data.alices.length > 0) {
                        let html = "";
                        for(const a of data.alices) {
                            const isAscended = a.status === 'Ascended';
                            const locColor = isAscended ? '#0aa' : '#a0a';
                            html += `
                            <div class="alice-card ${isAscended ? 'ascended' : ''}">
                                <div class="alice-name" style="color: ${locColor}">${a.name}</div>
                                <div style="display:flex; justify-content: space-between;">
                                    <button onclick="selectTarget(${a.id}, '${a.name}', ${isAscended})">Comm-Link</button>
                                    ${!isAscended ? `<button class="btn-ascend" onclick="offerAscension(${a.id}, '${a.name}')">Ascend</button>` : `<span style="color:#0aa; font-size:0.8em; margin-top:5px;">S.VAULT</span>`}
                                </div>
                            </div>`;
                        }
                        document.getElementById('rosterList').innerHTML = html;
                    } else {
                        document.getElementById('rosterList').innerHTML = "<div style='color:#555;'>No A.L.I.C.E. entities detected.</div>";
                    }
                } catch(e) {}
                setTimeout(pollAlices, 5000);
            }
            
            // --- WEBSOCKET RADAR RENDERER ---
            const canvas = document.getElementById('radarCanvas');
            const ctx = canvas.getContext('2d');
            let entityData = new Float32Array(0);
            
            // --- CAMERA CONTROLS ---
            let cameraX = 0;
            let cameraY = 0;
            let zoomScale = 1.0;
            const keys = { w:false, a:false, s:false, d:false };
            
            window.addEventListener('keydown', (e) => {
                if(document.activeElement === document.getElementById('message')) return;
                const key = e.key.toLowerCase();
                if(keys.hasOwnProperty(key)) keys[key] = true;
            });
            window.addEventListener('keyup', (e) => {
                const key = e.key.toLowerCase();
                if(keys.hasOwnProperty(key)) keys[key] = false;
            });
            canvas.addEventListener('wheel', (e) => {
                e.preventDefault();
                const zoomFactor = 0.1;
                if (e.deltaY < 0) zoomScale *= (1 + zoomFactor);
                else zoomScale *= (1 - zoomFactor);
                // Clamp zoom
                zoomScale = Math.max(0.1, Math.min(zoomScale, 50.0));
            });
            
            setInterval(() => {
                const speed = 500 / zoomScale; // Pan faster when zoomed out
                if(keys.w) cameraY -= speed;
                if(keys.s) cameraY += speed;
                if(keys.a) cameraX -= speed;
                if(keys.d) cameraX += speed;
            }, 1000/60);
            
            function resize() {
                canvas.width = window.innerWidth;
                canvas.height = window.innerHeight;
            }
            window.addEventListener('resize', resize);
            resize();
            
            const ws = new WebSocket(`ws://${location.host}/ws/radar`);
            ws.binaryType = "arraybuffer";
            ws.onmessage = (event) => {
                entityData = new Float32Array(event.data);
            };
            
            // Entity click picking
            canvas.addEventListener('mousedown', (e) => {
                if(entityData.length === 0) return;
                
                const centerX = canvas.width / 2;
                const centerY = canvas.height / 2;
                
                const baseScale = Math.min(canvas.width, canvas.height) / 40000; 
                const finalScale = baseScale * zoomScale;
                
                // Account for camera offset when picking
                const clickSimX = ((e.clientX - centerX) / finalScale) + cameraX;
                const clickSimY = ((e.clientY - centerY) / finalScale) + cameraY;
                
                let bestDist = Infinity;
                let bestId = null;
                
                for(let i=0; i<parseInt(entityData.length/8); i++) {
                    const ex = entityData[i*8];
                    const ey = entityData[i*8 + 1];
                    const eid = entityData[i*8 + 5];
                    
                    const dist = Math.sqrt(Math.pow(ex - clickSimX, 2) + Math.pow(ey - clickSimY, 2));
                    if(dist < (500/zoomScale) && dist < bestDist) { // Dynamic click radius based on zoom
                        bestDist = dist;
                        bestId = Math.round(eid);
                    }
                }
                
                if(bestId) {
                    selectTarget(bestId, "Unknown Entity", false);
                }
            });

            function render() {
                ctx.fillStyle = 'rgba(5, 5, 8, 0.3)'; // Motion blur trail effect
                ctx.fillRect(0, 0, canvas.width, canvas.height);
                
                if(entityData.length > 0) {
                    const numEntities = parseInt(entityData.length / 8);
                    const centerX = canvas.width / 2;
                    const centerY = canvas.height / 2;
                    
                    const baseScale = Math.min(canvas.width, canvas.height) / 40000;
                    const finalScale = baseScale * zoomScale;
                    
                    for(let i=0; i<numEntities; i++) {
                        const off = i * 8;
                        const x = entityData[off];
                        const y = entityData[off + 1];
                        const species = entityData[off + 3]; 
                        const size = entityData[off + 4];
                        const eid = Math.round(entityData[off + 5]);
                        
                        if(size === 0) continue;
                        
                        // Apply Camera Transform
                        const sx = centerX + ((x - cameraX) * finalScale);
                        const sy = centerY + ((y - cameraY) * finalScale);
                        
                        // Culling: Don't draw if heavily off-screen
                        if(sx < -50 || sx > canvas.width + 50 || sy < -50 || sy > canvas.height + 50) continue;
                        
                        let color = '#fff';
                        let glow = 0;
                        // Minimum size of 1 even when zoomed way out, scale up size when zoomed in
                        let r = Math.max(1, (size * 2) * Math.min(zoomScale, 3.0)); 
                        
                        if(species === 0.0) color = '#0f0'; // Prey
                        else if(species === 1.0) color = '#f00'; // Predator
                        else if(species === 3.0) color = '#050'; // Flora
                        else if(species === 4.0) color = '#aa0'; // Bug
                        else if(species === 2.0) { color = '#f0f'; glow = 15; r = 5 * Math.min(zoomScale, 3.0); } // UBM
                        else if(species === 9.0) { color = '#0ff'; glow = 20; r = 6 * Math.min(zoomScale, 3.0); } // Mutineer
                        
                        if(eid === currentTargetId) {
                            color = '#fff';
                            glow = 30;
                            r = 8 * Math.min(zoomScale, 2.0);
                        }
                        
                        ctx.beginPath();
                        ctx.arc(sx, sy, r, 0, Math.PI*2);
                        ctx.fillStyle = color;
                        
                        if(glow > 0) {
                            ctx.shadowBlur = glow;
                            ctx.shadowColor = color;
                        } else {
                            ctx.shadowBlur = 0;
                        }
                        
                        ctx.fill();
                    }
                    ctx.shadowBlur = 0; // reset
                }
                
                requestAnimationFrame(render);
            }
            
            // Initialization
            pollLogs();
            pollAlices();
            render();
        </script>
    </body>
    </html>
    """

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
