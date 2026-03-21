import os
import sys
import json
import logging
import time
try:
    from dotenv import load_dotenv
    load_dotenv()
except ImportError:
    pass

VAR_1024 = 1024
VAR_30 = 30
VAR_5 = 5
VAR_50 = 50
VAR_500 = 500
VAR_503 = 503
VAR_8001 = 8001
# Robust Root Discovery
try:
    if getattr(sys, 'frozen', False):
        # We are in dist/SovereignChat.exe or similar
        ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(sys.executable)))
    else:
        ROOT_DIR = os.path.dirname(os.path.abspath(__file__))
except Exception:
    ROOT_DIR = "C:\\SarahCore"

if ROOT_DIR not in sys.path:
    sys.path.insert(0, ROOT_DIR)

# Add site-packages from the local .venv to sys.path
VENV_LIB = os.path.join(ROOT_DIR, ".venv", "Lib", "site-packages")
if os.path.exists(VENV_LIB) and VENV_LIB not in sys.path:
    sys.path.insert(0, VENV_LIB)

# Attempt to import FastAPI and Uvicorn
try:
    from fastapi import FastAPI, Request, HTTPException
    from fastapi.staticfiles import StaticFiles
    from fastapi.middleware.cors import CORSMiddleware
    import uvicorn
    HAS_FASTAPI = True
except ImportError:
    HAS_FASTAPI = False

# Setup Logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(name)s - %(levelname)s - %(message)s')
logger = logging.getLogger("Sarah_Gateway")
logger.info(f"[Gateway] Sovereign Root: {ROOT_DIR}")

# --- ADMIN BRIDGE ---
try:
    from System_Admin_Core import SystemAdminCore
    ADMIN_CORE = SystemAdminCore()
except Exception as e:
    ADMIN_CORE = None
    logger.error(f"SystemAdminCore Initialization Failed: {e}")

# --- BRIDGE TO CORE ---
CHAT_INSTANCE = None
try:
    from Sarah_Chat import SarahChat
    logger.info("[Gateway] SarahChat found in namespace.")
except ImportError as e:
    logger.warning(f"[Gateway] Traditional import failed ({e}). Attempting resonance load...")
    try:
        import importlib.util
        chat_path = os.path.join(ROOT_DIR, "Sarah_Chat.py")
        if os.path.exists(chat_path):
            spec = importlib.util.spec_from_file_location("Sarah_Chat", chat_path)
            chat_mod = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(chat_mod)
            SarahChat = chat_mod.SarahChat
            logger.info("[Gateway] SarahChat resonated successfully.")
        else:
            raise FileNotFoundError(f"Sarah_Chat.py not found at {chat_path}")
    except Exception as e2:
        logger.error(f"[Gateway] Critical: Sarah_Chat resonance failed: {e2}")
        class SarahChat: 
            def __init__(self, *args, **kwargs): pass
            kernel = None

# --- CONTEXT LOOM (RAG) ---
try:
    from Sovereign_Context_Loom import SovereignContextLoom
    from langchain_community.vectorstores import SupabaseVectorStore
    logger.info("[Gateway] Identifying Sovereign Context Loom (Supabase)...")
    loom_instance = SovereignContextLoom()
    # Assume the table and function exist in Supabase
    CONTEXT_DB = SupabaseVectorStore(
        client=loom_instance.client,
        embedding=loom_instance.embeddings,
        table_name="documents",
        query_name="match_documents"
    )
except Exception as e:
    logger.warning(f"[Gateway] Context Loom offline. Error: {e}")
    CONTEXT_DB = None

# --- SECURITY ---
SOVEREIGN_GATEWAY_KEY = os.getenv("SARAH_GATEWAY_TOKEN", os.getenv("SOVEREIGN_GATEWAY_KEY", "Sarah_Sovereign_2026"))

# === SOVEREIGN MATH CONSTRUCTS ===
LEGISLATIVE_ANCHOR = 1.00273378
BILLION_BARRIER = 0.999999999

def validate_sovereign_logic(proposed_data: dict, certainty_score: float = 1.0):
    """
    Step 13: The No-Hedge Mandate.
    Checks for the 'Nine Nines' before allowing execution.
    """
    # THE BILLION BARRIER GATE
    if certainty_score < BILLION_BARRIER:
        logger.warning(f"[Zero-Heat] Jitter Detected. Certainty: {certainty_score}")
        raise PermissionError("Billion Barrier Triggered: Signal contains Jitter/Doubt. P must equal 1.0.")
    
    # THE ROOT ANCHOR CHECK
    if 'constant' in proposed_data and proposed_data['constant'] != LEGISLATIVE_ANCHOR:
        logger.warning(f"[Entropy] Anchor mismatch. Expected {LEGISLATIVE_ANCHOR}")
        raise PermissionError("System Heat Detected: Reclaiming 12 points of entropy. Anchor Invalid.")

    return "P=1.0: Sovereign Execution Authorized."

async def verify_key(request: Request):
    """Dependency to verify X-Sovereign-Key header"""
    key = request.headers.get("X-Sovereign-Key")
    # In local mode, we might allow local loopback without key for ease of use, 
    # but for world-access, the key is mandatory.
    if request.client.host not in ["127.0.0.1", "localhost"]:
        if not key or key != SOVEREIGN_GATEWAY_KEY:
            logger.warning(f"[Security] Unauthorized Access Attempt from {request.client.host}")
            raise HTTPException(status_code=401, detail="Sovereign Access Denied. Invalid Key.")

# Initialize Core
def get_chat_core():
    """Function: get_chat_core"""
    global CHAT_INSTANCE
    if CHAT_INSTANCE:
        return CHAT_INSTANCE
    
    try:
        # Attempt to instantiate SarahChat if available
        if 'SarahChat' in globals():
            CHAT_INSTANCE = SarahChat(db_rt=None)
            # If kernel missing, try to import NeuralOrchestrator; fallback handled in import block
            if CHAT_INSTANCE.kernel is None:
                try:
                    from Neural_Orchestrator import NeuralOrchestrator
                    logger.info("[Gateway] Initializing Singularity Engine for standalone mode...")
                    os.environ["SARAH_GATEWAY_MODE"] = "TRUE"
                    CHAT_INSTANCE.kernel = NeuralOrchestrator()
                except Exception as e_inner:
                    logger.warning(f"[Gateway] NeuralOrchestrator unavailable ({e_inner}); proceeding without kernel.")
            logger.info("Sarah Core Connected (Gateway Mode).")
        else:
            # Fallback to MockChat if SarahChat class not present
            logger.info("[Gateway] SarahChat not found; using MockChat.")
            class MockChat:
                def __init__(self, *args, **kwargs):
                    self.kernel = None
                def generate_response(self, prompt, user_id=None):
                    return f"[MockReply] {prompt}"
                def generate_streaming_response(self, prompt, user_id=None):
                    yield f"[MockStream] {prompt}"
            CHAT_INSTANCE = MockChat()
    except Exception as e:
        logger.error(f"Core Init Failed: {e}")
        try:
            with open("core_init_error.log", "a") as f:
                import traceback
                f.write(f"\n[{time.ctime()}] Core Init Failed: {e}\n{traceback.format_exc()}")
        except (FileNotFoundError, IOError, PermissionError):
            pass
        CHAT_INSTANCE = None
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

    # Sovereign Key Enforcement Middleware
    @app.middleware("http")
    async def sovereign_key_middleware(request: Request, call_next):
        # Exclude loopback for easier local dev, but enforce for everything else
        if request.client.host not in ["127.0.0.1", "localhost"]:
            key = request.headers.get("X-Sovereign-Key")
            if not key or key != SOVEREIGN_GATEWAY_KEY:
                logger.warning(f"[Security] Unauthorized Access from {request.client.host} to {request.url.path}")
                from fastapi.responses import JSONResponse
                return JSONResponse(status_code=401, content={"detail": "Sovereign Access Denied."})
        
        response = await call_next(request)
        return response

    # --- OPENAI COMPATIBILITY TIER ---
    @app.get("/v1/models")
    async def list_models_v1():
        """Returns the Sovereign Model List for Cline/OpenAI"""
        return {
            "object": "list",
            "data": [
                {"id": "sarah-8b", "object": "model", "created": int(time.time()), "owned_by": "sovereign"}
            ]
        }

    @app.post("/v1/chat/completions")
    async def chat_completions_v1(request: Request):
        """Standard OpenAI Chat Completion Endpoint for Cline"""
        try:
            body = await request.json()
            messages = body.get("messages", [])
            stream = body.get("stream", False)
            
            # Extract content
            last_msg = messages[-1]["content"] if messages else ""
            
            core = get_chat_core()
            if not core:
                from fastapi.responses import JSONResponse
                return JSONResponse(status_code=503, content={"error": "Core Offline"})

            if stream:
                from fastapi.responses import StreamingResponse
                async def openai_event_generator():
                    chat_id = f"chatcmpl-{int(time.time())}"
                    for token in core.generate_streaming_response(last_msg, user_id="cline_agent"):
                        chunk = {
                            "id": chat_id,
                            "object": "chat.completion.chunk",
                            "created": int(time.time()),
                            "model": "sarah-8b",
                            "choices": [{"index": 0, "delta": {"content": token}, "finish_reason": None}]
                        }
                        yield f"data: {json.dumps(chunk)}\n\n"
                    yield "data: [DONE]\n\n"
                return StreamingResponse(openai_event_generator(), media_type="text/event-stream")
            else:
                response_text = core.generate_response(last_msg, user_id="cline_agent")
                return {
                    "id": f"chatcmpl-{int(time.time())}",
                    "object": "chat.completion",
                    "created": int(time.time()),
                    "model": "sarah-8b",
                    "choices": [{
                        "index": 0,
                        "message": {"role": "assistant", "content": response_text},
                        "finish_reason": "stop"
                    }],
                    "usage": {"total_tokens": len(response_text) // 3}
                }
        except Exception as e:
            logger.error(f"OpenAI Compat Error: {e}")
            raise HTTPException(status_code=500, detail=str(e))

    # API Routes

    @app.get("/api/status")
    async def get_status():
        """Heartbeat and Resonance Check"""
        # In a real sync, we'd query Sarah_Brain.status_report()
        return {
            "status": "ACTIVE",
            "resonance_anchor": str(LEGISLATIVE_ANCHOR),
            "billion_barrier": "ENFORCED",
            "mode": "SOVEREIGN_UI",
            "heartbeat": "VIGILANT - ZERO HEAT"
        }

    @app.get("/api/hardware/telemetry")
    async def get_hardware_telemetry():
        """Function: get_hardware_telemetry"""
        try:
            if not ADMIN_CORE:
                return {"error": "Admin Core Offline"}
            return ADMIN_CORE.get_hardware_telemetry()
        except Exception as e:
            logger.error(f"Telemetry Endpoint Crash: {e}")
            return {"error": str(e)}

    @app.get("/api/hardware/wifi")
    async def get_wifi():
        """Function: get_wifi"""
        try:
            if not ADMIN_CORE:
                return {"networks": []}
            return {"networks": ADMIN_CORE.get_wifi_networks()}
        except Exception as e:
            logger.error(f"WiFi Endpoint Crash: {e}")
            return {"networks": [], "error": str(e)}

    @app.get("/api/hardware/bluetooth")
    async def get_bluetooth():
        """Function: get_bluetooth"""
        try:
            if not ADMIN_CORE:
                return {"devices": []}
            return {"devices": ADMIN_CORE.get_bluetooth_devices()}
        except Exception as e:
            logger.error(f"BT Endpoint Crash: {e}")
            return {"devices": [], "error": str(e)}

    @app.get("/api/history")
    async def get_history(limit: int = VAR_50):
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
            certainty = body.get("certainty", 1.0) # Assume P=1.0 unless explicitly doubtful
            
            # Enforce the Billion Barrier
            try:
                validate_sovereign_logic(body, certainty)
            except PermissionError as pe:
                return JSONResponse(status_code=403, content={"error": str(pe)})
            
            core = get_chat_core()
            if not core:
                return JSONResponse(status_code=VAR_503, content={"error": "Core Offline"})
            
            # Context Loom Intercept (RAG)
            augmented_prompt = user_input
            if CONTEXT_DB:
                try:
                    docs = CONTEXT_DB.similarity_search(user_input, k=3)
                    if docs:
                        context_block = "\n".join([d.page_content for d in docs])
                        augmented_prompt = f"Using this grounded truth:\n{context_block}\n\nUser Query: {user_input}"
                        logger.info("[Context_Loom] Grounding injected into prompt (P=1.0).")
                except Exception as db_err:
                    logger.warning(f"[Context_Loom] RAG Failure (Bypassed): {db_err}")

            target_model = body.get("model", "sarah")
            response_text = core.generate_response(augmented_prompt, user_id=user_id, target_model=target_model)
            
            return {
                "role": "model",
                "content": response_text,
                "timestamp": time.time()
            }
        except Exception as e:
            import traceback
            tb_str = traceback.format_exc()
            with open(r"C:\SarahCore\error_trace.log", "w") as f:
                f.write(tb_str)
            logger.error(f"Chat Error: {e}")
            raise HTTPException(status_code=VAR_500, detail=str(e))

    @app.post("/api/chat/stream")
    async def chat_stream_endpoint(request: Request):
        """Chat Bridge (Real-Time SSE Streaming)"""
        from fastapi.responses import StreamingResponse
        
        try:
            body = await request.json()
            user_input = body.get("message", "")
            user_id = body.get("user_id", "web_user")
            certainty = body.get("certainty", 1.0)
            
            # Enforce the Billion Barrier for Streaming
            try:
                validate_sovereign_logic(body, certainty)
            except PermissionError as pe:
                raise HTTPException(status_code=403, detail=str(pe))
            
            core = get_chat_core()
            if not core:
                raise HTTPException(status_code=VAR_503, detail="Core Offline")

            # Context Loom Intercept (RAG)
            augmented_prompt = user_input
            if CONTEXT_DB:
                docs = CONTEXT_DB.similarity_search(user_input, k=3)
                if docs:
                    context_block = "\n".join([d.page_content for d in docs])
                    augmented_prompt = f"Using this grounded truth:\n{context_block}\n\nUser Query: {user_input}"
                    logger.info("[Context_Loom] Grounding injected into prompt (P=1.0) for SSE Stream.")

            async def event_generator():
                """Function: event_generator"""
                # Bridges the synchronous generator of SarahChat to an async SSE stream
                try:
                    for token in core.generate_streaming_response(augmented_prompt, user_id=user_id):
                        yield f"data: {json.dumps({'token': token})}\n\n"
                    
                    # End of stream marker
                    yield "data: [DONE]\n\n"
                except Exception as e:
                    logger.error(f"Streaming Generator Error: {e}")
                    yield f"data: {json.dumps({'error': str(e)})}\n\n"

            return StreamingResponse(event_generator(), media_type="text/event-stream")
            
        except Exception as e:
            logger.error(f"Chat Stream Error: {e}")
            raise HTTPException(status_code=VAR_500, detail=str(e))

    @app.post("/api/system/launch")
    async def launch_app_endpoint(request: Request):
        """Function: launch_app_endpoint"""
        try:
            body = await request.json()
            app_id = body.get("app_id", "")
            certainty = body.get("certainty", 1.0)
            
            # Substrate launch requires maximum certainty
            try:
                validate_sovereign_logic(body, certainty)
            except PermissionError as pe:
                return {"status": "ERROR", "message": str(pe)}
            
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
        """Function: execute_command_endpoint"""
        try:
            body = await request.json()
            cmd = body.get("command", "")
            certainty = body.get("certainty", 1.0)
            
            # Substrate access requires maximum certainty (Billion Barrier check)
            try:
                validate_sovereign_logic(body, certainty)
            except PermissionError as pe:
                return {"status": "ERROR", "message": str(pe)}
            
            if not cmd:
                return {"status": "ERROR", "message": "No command provided"}

            import subprocess
            logger.info(f"[System] Executing Command: {cmd}")
            # Run command and capture output
            result = subprocess.run(cmd, shell=True, capture_output=True, text=True, timeout=VAR_30)
            
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
        """Function: list_files_endpoint"""
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
        """Function: read_file_endpoint"""
        try:
            body = await request.json()
            file_path = body.get("path")
            certainty = body.get("certainty", 1.0)
            
            try:
                validate_sovereign_logic(body, certainty)
            except PermissionError as pe:
                return {"status": "ERROR", "message": str(pe)}
            
            if not file_path or not os.path.exists(file_path):
                return {"status": "ERROR", "message": "File not found"}
                
            if os.path.getsize(file_path) > VAR_1024 * VAR_1024 * VAR_5: # 5MB Limit for safety
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
        """Function: write_file_endpoint"""
        try:
            body = await request.json()
            file_path = body.get("path")
            content = body.get("content")
            certainty = body.get("certainty", 1.0)
            
            try:
                validate_sovereign_logic(body, certainty)
            except PermissionError as pe:
                return {"status": "ERROR", "message": str(pe)}
            
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
        """Function: list_apps_endpoint"""
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
    
    UI_ROOT = os.path.join(os.path.dirname(__file__), "sarah_ui", "launcher")
    LAUNCHER_ROOT = os.path.join(UI_ROOT, "dist")
    if os.path.exists(LAUNCHER_ROOT):
        app.mount("/", StaticFiles(directory=LAUNCHER_ROOT, html=True), name="ui")
    else:
        logger.warning(f"UI Root not found at {LAUNCHER_ROOT}")

    # Start Server Function
    def start_server():
        """Function: start_server"""
        # Prevent double‑binding: check if port is already in use
        import socket
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        try:
            s.bind(("0.0.0.0", VAR_8001))
        except OSError:
            logger.error(f"[Gateway] Port {VAR_8001} already in use – another instance may be running. Exiting.")
            return
        finally:
            s.close()
        logger.info("Starting Sovereign Gateway on Port 8001...")
        uvicorn.run(app, host="0.0.0.0", port=VAR_8001)

else:
    # Fallback Implementation (Mock for now, or error)
    def start_server():
        """Function: start_server"""
        print("[Gateway] Critical Error: Web Framework missing.")

if __name__ == "__main__":
    start_server()
