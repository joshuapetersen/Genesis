import os
import sys
import json
import logging
import asyncio
from typing import Dict, Any, Optional

# --- SOVEREIGN PATH ENFORCEMENT ---
# Ensure SarahCore root is always at the top of sys.path to prevent import collisions
ROOT_DIR = os.path.dirname(os.path.abspath(__file__))
if ROOT_DIR not in sys.path:
    sys.path.insert(0, ROOT_DIR)
# Also add site-packages if we are in a virtual environment
VENV_LIB = os.path.join(ROOT_DIR, ".venv", "Lib", "site-packages")
if os.path.exists(VENV_LIB) and VENV_LIB not in sys.path:
    sys.path.insert(0, VENV_LIB)

# Attempt to import FastAPI and Uvicorn
try:
    from fastapi import FastAPI, Request, HTTPException
    from fastapi.responses import JSONResponse, FileResponse
    from fastapi.staticfiles import StaticFiles
    from fastapi.middleware.cors import CORSMiddleware
    import uvicorn
    HAS_FASTAPI = True
except ImportError:
    HAS_FASTAPI = False
    print("[Gateway] WARNING: FastAPI/Uvicorn not found. Install via: pip install fastapi uvicorn")

# Setup Logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(name)s - %(levelname)s - %(message)s')
logger = logging.getLogger("Sarah_Gateway")

# --- ADMIN BRIDGE ---
try:
    from System_Admin_Core import SystemAdminCore
    ADMIN_CORE = SystemAdminCore()
except Exception as e:
    ADMIN_CORE = None
    logger.error(f"SystemAdminCore Initialization Failed: {e}. Hardware telemetry will be disabled.")

# --- BRIDGE TO CORE ---
# We need to bridge to Sarah_Chat logic without running the CLI loop.
try:
    from Sarah_Chat import SarahChat
    # We might need a mock DB or connection if we don't want to spin up everything
    # For now, we instantiate it cleanly if possible.
    CHAT_INSTANCE = None 
except ImportError as e:
    logger.error(f"Failed to import Sarah_Chat: {e}")
    logger.error(f"Interpreter: {sys.executable}")
    logger.error(f"Python Path: {sys.path}")
    CHAT_INSTANCE = None

# Initialize Core
def get_chat_core():
    global CHAT_INSTANCE
    if CHAT_INSTANCE:
        return CHAT_INSTANCE
        
    try:
        # We need a dummy DB object or properly init firebase?
        # Sarah_Chat expects 'db_rt'. If we are in local mode, maybe pass None?
        if 'SarahChat' in globals():
            CHAT_INSTANCE = SarahChat(db_rt=None)
            # Standalone Gateway needs its own kernel instance if not injected
            if CHAT_INSTANCE.kernel is None:
                from Neural_Orchestrator import NeuralOrchestrator
                logger.info("[Gateway] Initializing Singularity Engine for standalone mode...")
                CHAT_INSTANCE.kernel = NeuralOrchestrator()
            logger.info("Sarah Core Connected (Gateway Mode).")
    except Exception as e:
        logger.error(f"Core Init Failed: {e}")
        try:
            with open("core_init_error.log", "w") as f:
                import traceback
                f.write(f"Core Init Failed: {e}\n{traceback.format_exc()}")
        except:
            pass
    return CHAT_INSTANCE

# --- FASTAPI APP ---
if HAS_FASTAPI:
    app = FastAPI(title="Sarah Sovereign Gateway", version="1.0")

    # CORS
    app.add_middleware(
        CORSMiddleware,
        allow_origins=["*"],
        allow_credentials=True,
        allow_methods=["*"],
        allow_headers=["*"],
    )

    # API Routes
    @app.get("/api/status")
    async def get_status():
        """Heartbeat and Resonance Check"""
        # In a real sync, we'd query Sarah_Brain.status_report()
        return {
            "status": "ACTIVE",
            "resonance_anchor": "1.09277703703703",
            "mode": "SOVEREIGN_UI",
            "heartbeat": "VIGILANT"
        }

    @app.get("/api/hardware/telemetry")
    async def get_hardware_telemetry():
        try:
            if not ADMIN_CORE:
                return {"error": "Admin Core Offline"}
            return ADMIN_CORE.get_hardware_telemetry()
        except Exception as e:
            logger.error(f"Telemetry Endpoint Crash: {e}")
            return {"error": str(e)}

    @app.get("/api/hardware/wifi")
    async def get_wifi():
        try:
            if not ADMIN_CORE:
                return {"networks": []}
            return {"networks": ADMIN_CORE.get_wifi_networks()}
        except Exception as e:
            logger.error(f"WiFi Endpoint Crash: {e}")
            return {"networks": [], "error": str(e)}

    @app.get("/api/hardware/bluetooth")
    async def get_bluetooth():
        try:
            if not ADMIN_CORE:
                return {"devices": []}
            return {"devices": ADMIN_CORE.get_bluetooth_devices()}
        except Exception as e:
            logger.error(f"BT Endpoint Crash: {e}")
            return {"devices": [], "error": str(e)}

    @app.get("/api/history")
    async def get_history(limit: int = 50):
        """Retrieve recent chat memories"""
        try:
            core = get_chat_core()
            if not core or not hasattr(core, 'vault') or not core.vault:
                return {"history": []}
            
            # Fetch from vault
            history = core.vault.get_recent_memories(limit=limit)
            return {"history": history}
        except Exception as e:
            logger.error(f"History Endpoint Crash: {e}")
            return {"history": [], "error": str(e)}

    @app.post("/api/chat")
    async def chat_endpoint(request: Request):
        """Chat Bridge (Synchronous)"""
        try:
            body = await request.json()
            user_input = body.get("message", "")
            user_id = body.get("user_id", "web_user")
            
            core = get_chat_core()
            if not core:
                return JSONResponse(status_code=503, content={"error": "Core Offline"})
            
            response_text = core.generate_response(user_input, user_id=user_id)
            
            return {
                "role": "model",
                "content": response_text,
                "timestamp": time.time()
            }
        except Exception as e:
            logger.error(f"Chat Error: {e}")
            raise HTTPException(status_code=500, detail=str(e))

    @app.post("/api/chat/stream")
    async def chat_stream_endpoint(request: Request):
        """Chat Bridge (Real-Time SSE Streaming)"""
        from fastapi.responses import StreamingResponse
        
        try:
            body = await request.json()
            user_input = body.get("message", "")
            user_id = body.get("user_id", "web_user")
            
            core = get_chat_core()
            if not core:
                raise HTTPException(status_code=503, detail="Core Offline")

            async def event_generator():
                # Bridges the synchronous generator of SarahChat to an async SSE stream
                # In a high-load system, we'd use a threadpool, but for local 1:1, this is efficient.
                try:
                    for token in core.generate_streaming_response(user_input, user_id=user_id):
                        # Format: SSE data block
                        yield f"data: {json.dumps({'token': token})}\n\n"
                    
                    # End of stream marker
                    yield "data: [DONE]\n\n"
                except Exception as e:
                    logger.error(f"Streaming Generator Error: {e}")
                    yield f"data: {json.dumps({'error': str(e)})}\n\n"

            return StreamingResponse(event_generator(), media_type="text/event-stream")
            
        except Exception as e:
            logger.error(f"Chat Stream Error: {e}")
            raise HTTPException(status_code=500, detail=str(e))

    @app.post("/api/system/launch")
    async def launch_app_endpoint(request: Request):
        try:
            body = await request.json()
            app_id = body.get("app_id", "")
            
            # Map app_id to actual commands/executables
            app_map = {
                "notepad": "notepad.exe",
                "calc": "calc.exe",
                "cmd": "cmd.exe",
                "taskmgr": "taskmgr.exe",
                "explorer": "explorer.exe",
                "browser": "start https://www.google.com" # Default browser
            }
            
            cmd = app_map.get(app_id.lower())
            if not cmd:
                 # Try direct execution if not in map
                 cmd = app_id
            
            import subprocess
            logger.info(f"[System] Launching App: {cmd}")
            subprocess.Popen(cmd, shell=True) # Non-blocking
            return {"status": "SUCCESS", "launched": cmd}
        except Exception as e:
            logger.error(f"Launch Error: {e}")
            return {"status": "ERROR", "message": str(e)}

    @app.post("/api/system/execute")
    async def execute_command_endpoint(request: Request):
        try:
            body = await request.json()
            cmd = body.get("command", "")
            
            if not cmd:
                return {"status": "ERROR", "message": "No command provided"}

            import subprocess
            logger.info(f"[System] Executing Command: {cmd}")
            # Run command and capture output
            result = subprocess.run(cmd, shell=True, capture_output=True, text=True, timeout=30)
            
            return {
                "status": "SUCCESS",
                "stdout": result.stdout,
                "stderr": result.stderr,
                "returncode": result.returncode
            }
        except Exception as e:
            logger.error(f"Execution Error: {e}")
            return {"status": "ERROR", "message": str(e)}

    # --- FILE SYSTEM API (REAL ACCESS) ---
    @app.post("/api/fs/list")
    async def list_files_endpoint(request: Request):
        try:
            body = await request.json()
            path_str = body.get("path", "C:\\SarahCore") # Default to Core
            
            if not os.path.exists(path_str):
                 return {"status": "ERROR", "message": "Path not found"}
            
            items = []
            with os.scandir(path_str) as entries:
                for entry in entries:
                    items.append({
                        "name": entry.name,
                        "path": entry.path,
                        "is_dir": entry.is_dir(),
                        "size": entry.stat().st_size if not entry.is_dir() else 0
                    })
            
            # Sort: Directories first, then files
            items.sort(key=lambda x: (not x['is_dir'], x['name'].lower()))
            
            return {"status": "SUCCESS", "current_path": path_str, "items": items}
        except Exception as e:
            return {"status": "ERROR", "message": str(e)}

    @app.post("/api/fs/read")
    async def read_file_endpoint(request: Request):
        try:
            body = await request.json()
            file_path = body.get("path")
            
            if not file_path or not os.path.exists(file_path):
                return {"status": "ERROR", "message": "File not found"}
                
            if os.path.getsize(file_path) > 1024 * 1024 * 5: # 5MB Limit for safety
                return {"status": "ERROR", "message": "File too large (>5MB)"}

            # Try reading as text
            try:
                with open(file_path, "r", encoding="utf-8") as f:
                    content = f.read()
                return {"status": "SUCCESS", "content": content, "encoding": "utf-8"}
            except UnicodeDecodeError:
                return {"status": "ERROR", "message": "Binary or non-UTF8 file"}
        except Exception as e:
             return {"status": "ERROR", "message": str(e)}

    @app.post("/api/fs/write")
    async def write_file_endpoint(request: Request):
        try:
            body = await request.json()
            file_path = body.get("path")
            content = body.get("content")
            
            if not file_path:
                return {"status": "ERROR", "message": "No path provided"}
            
            with open(file_path, "w", encoding="utf-8") as f:
                f.write(content)
                
            logger.info(f"[FS] Wrote to {file_path}")
            return {"status": "SUCCESS", "message": "File saved"}
        except Exception as e:
            return {"status": "ERROR", "message": str(e)}

    @app.get("/api/system/apps")
    async def list_apps_endpoint():
        return {
            "apps": [
                {"id": "notepad", "label": "Notepad"},
                {"id": "calc", "label": "Calculator"},
                {"id": "cmd", "label": "Terminal"},
                {"id": "taskmgr", "label": "Task Manager"},
                {"id": "explorer", "label": "File Explorer"}
            ]
        }

    # Static Files (The Sovereign UI)
    # GATEWAY MOUNT DISABLED IN FAVOR OF NATIVE WRAPPER
    # sarah_native.py handles the mounting with frozen-aware paths
    
    UI_ROOT = os.path.join(os.path.dirname(__file__), "SovereignChat")
    LAUNCHER_ROOT = os.path.join(UI_ROOT, "dist")
    if os.path.exists(LAUNCHER_ROOT):
        app.mount("/", StaticFiles(directory=LAUNCHER_ROOT, html=True), name="ui")
    else:
        logger.warning(f"UI Root not found at {LAUNCHER_ROOT}")

    # Start Server Function
    def start_server():
        logger.info("Starting Sovereign Gateway on Port 8001...")
        uvicorn.run(app, host="0.0.0.0", port=8001)

else:
    # Fallback Implementation (Mock for now, or error)
    def start_server():
        print("[Gateway] Critical Error: Web Framework missing.")

if __name__ == "__main__":
    start_server()
