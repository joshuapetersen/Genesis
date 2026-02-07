
import webview
import threading
import sys
import os
import uvicorn
from sarah_gateway import app, start_server, CHAT_INSTANCE

# Configure Gateway to Serve DIST (Built UI)
# Log to AppData to avoid permission issues in Program Files
appdata_dir = os.path.join(os.environ.get('APPDATA', os.path.expanduser('~')), 'SarahSovereign')
os.makedirs(appdata_dir, exist_ok=True)
log_path = os.path.join(appdata_dir, "native_debug.log")
with open(log_path, "w") as f:
    f.write("Initializing Native Client...\n")

if getattr(sys, 'frozen', False):
    # Running in a PyInstaller bundle
    base_path = sys._MEIPASS
    UI_DIST = os.path.join(base_path, "SovereignChat", "dist")
    with open(log_path, "a") as f:
        f.write(f"[Native] Frozen Mode. MEIPASS: {base_path}\n")
        f.write(f"[Native] UI Path: {UI_DIST}\n")
else:
    # Running in a normal Python environment
    base_path = os.path.dirname(os.path.abspath(__file__))
    UI_DIST = os.path.join(base_path, "SovereignChat", "dist")
    with open(log_path, "a") as f:
        f.write(f"[Native] Script Mode. UI Path: {UI_DIST}\n")

# Mount UI if exists
if os.path.exists(UI_DIST):
    from fastapi.staticfiles import StaticFiles
    # Mount root to DIST
    app.mount("/", StaticFiles(directory=UI_DIST, html=True), name="ui_dist")
else:
    print(f"[Native] WARNING: UI Dist not found at {UI_DIST}")

def run_server():
    """Run FastAPI in a separate thread"""
    try:
        with open(log_path, "a") as f:
            f.write("[Native] Starting Uvicorn on 127.0.0.1:8001...\n")
        # Force host to localhost to prevent external access
        uvicorn.run(app, host="127.0.0.1", port=8001, log_level="error")
    except Exception as e:
        with open(log_path, "a") as f:
            f.write(f"[Native] Uvicorn Error: {e}\n")

def start_native():
    """Launch the Native Window"""
    # Start Backend
    t = threading.Thread(target=run_server, daemon=True)
    t.start()
    
    # Start Frontend Window pointing to INTERNAL Localhost
    # This keeps the Chrome/Browser window hidden from the user
    # Port 8001: The Gateway Frequency
    print("[Native] Launching PyWebview Wrapper for http://127.0.0.1:8001")
    webview.create_window(
        title='Sarah Sovereign', 
        url='http://127.0.0.1:8001',
        width=1200,
        height=800,
        resizable=True,
        text_select=True,
        background_color='#000000'
    )
    
    webview.start(debug=False)

if __name__ == '__main__':
    print("--- SARAH SOVEREIGN NATIVE CLIENT ---")
    start_native()
