
import os
import sys
import threading
import time
import socket
import requests

# Virtualized Verification Logic
# Ensure the backend starts and the frontend can reach it.

def check_port(port):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        return s.connect_ex(('127.0.0.1', port)) == 0

def test_gateway():
    print("--- STARTING VIRTUAL VERIFICATION ---")
    
    # 1. Start Native Wrapper (Dry Run)
    import subprocess
    cmd = [r".\.venv\Scripts\python.exe", "sarah_native.py"]
    proc = subprocess.Popen(cmd, cwd=r"c:\SarahCore")
    
    print("Waiting for Gateway (Port 8001)...")
    success = False
    for i in range(30):
        if check_port(8001):
            print("[SUCCESS] Gateway responding on Port 8001.")
            success = True
            break
        time.sleep(1)
        
    if not success:
        print("[FAIL] Gateway failed to start on Port 8001.")
        proc.terminate()
        sys.exit(1)
        
    # 2. Check API Endpoint
    try:
        resp = requests.get("http://127.0.0.1:8001/api/status", timeout=5)
        print(f"[API] Status Response: {resp.json()}")
    except Exception as e:
        print(f"[FAIL] API unreachable: {e}")
        proc.terminate()
        sys.exit(1)
        
    print("--- VIRTUAL VERIFICATION COMPLETE: SYSTEM STABLE ---")
    proc.terminate()

if __name__ == "__main__":
    test_gateway()
