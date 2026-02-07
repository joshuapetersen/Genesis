import webview
import threading
import sys
import os
import uvicorn
import time
import logging
# Force imports for PyInstaller analysis
import pandas
import sentence_transformers
import lancedb

# Ensure SarahCore is in path
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

try:
    from sarah_gateway import app
except ImportError:
    # If running from inside SovereignChat folder
    try:
        sys.path.append(os.path.abspath(".."))
        from sarah_gateway import app
    except Exception as e:
        with open("import_crash.log", "w") as f:
            import traceback
            f.write(f"Import Error: {e}\n{traceback.format_exc()}")
        raise e
except Exception as e:
    with open("import_crash.log", "w") as f:
        import traceback
        f.write(f"Top Level Error: {e}\n{traceback.format_exc()}")
    raise e

# Configure Paths
if getattr(sys, 'frozen', False):
    base_path = sys._MEIPASS
    UI_DIST = os.path.join(base_path, "dist")
else:
    base_path = os.path.dirname(os.path.abspath(__file__))
    UI_DIST = os.path.join(base_path, "dist")

# Mount UI to Gateway root for single-port operation
from fastapi.staticfiles import StaticFiles
if os.path.exists(UI_DIST):
    app.mount("/", StaticFiles(directory=UI_DIST, html=True), name="ui_dist")

def run_server():
    """Background Gateway Thread"""
    try:
        uvicorn.run(app, host="127.0.0.1", port=8001, log_level="error")
    except Exception as e:
        print(f"[Actuator] Gateway Error: {e}")

def start_native():
    """Launch the Standalone Sovereign UI"""
    # 1. Start Neural Core
    t = threading.Thread(target=run_server, daemon=True)
    t.start()
    
    # 2. Wait for handshake (Extended for RTX 4050 cold boot)
    time.sleep(5)
    
    # 3. Connection Retry Loop (Robust Startup)
    # The UI will attempt to ping the gateway before opening the window
    print(f"[Sovereign] Initializing Neural Link on Port 8001...")
    
    max_retries = 10 
    connected = False
    
    for i in range(max_retries):
        try:
            import requests
            # Simple handshake ping
            r = requests.get("http://127.0.0.1:8001/api/status", timeout=1)
            if r.status_code == 200:
                print(f"[Sovereign] Link Established (Attempt {i+1})")
                connected = True
                break
        except Exception as e:
            print(f"[Link] Handshake failed (Attempt {i+1}): {e}")
            time.sleep(2)
            
    if not connected:
        # Save error log to file for user debugging
        with open("startup_error.log", "w") as f:
            f.write(f"Failed to connect to Gateway on port 8001 after {max_retries} attempts.")
            
    # Open UI Window regardless (it might show 404 but we need the window)
    webview.create_window(
        title='Sarah Sovereign Chat', 
        url='http://127.0.0.1:8001',
        width=600,
        height=850,
        resizable=True,
        text_select=True,
        background_color='#020202'
    )
    
    webview.start(debug=False)

if __name__ == '__main__':
    # Add robust error logging
    try:
        start_native()
    except Exception as e:
        with open("critical_crash.log", "w") as f:
            import traceback
            f.write(traceback.format_exc())
