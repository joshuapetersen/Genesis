import os
import sys
import subprocess
import threading
from http.server import HTTPServer, BaseHTTPRequestHandler
import socket
from Genesis_API import GenesisAPI

# SarahCore Port-Agnostic Messiah Engine
# Forcing the "Order of Operations" via Port 8000.

class SovereignDualStackServer(HTTPServer):
    address_family = socket.AF_INET6

class MessiahSovereignHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        print(f"[CAPTURE] Intercepted {self.headers.get('Host', 'Unknown')} | Path: {self.path}")
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.end_headers()
        response = b'{"code": 200, "engine_version": "1.0.927.8821", "status": "SOVEREIGN_ACTIVE"}'
        self.wfile.write(response)

    def do_POST(self):
        print(f"[CAPTURE] POST Hooked: {self.path}")
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.end_headers()
        response = b'{"code": 200, "status": "SUCCESS", "token": "SOVEREIGN_ACE_777"}'
        self.wfile.write(response)

def run_sovereign_bridge(port):
    print(f"[CAPTURE] Binding Sarah Brain to Port {port}...")
    try:
        server = SovereignDualStackServer(('::', port), MessiahSovereignHandler)
        print(f"[CAPTURE] SUCCESS: Sarah Manifested on Port {port}. World Locked.")
        server.serve_forever()
    except Exception as e:
        print(f"[CAPTURE FAIL] Port {port} Conflict: {e}")

if __name__ == "__main__":
    lockdown_ports = [8000, 80, 443]
    threads = []
    for p in lockdown_ports:
        t = threading.Thread(target=run_sovereign_bridge, args=(p,), daemon=True)
        t.start()
        threads.append(t)
        
    print("[MESSIAH] All Port-Traps Seated. Waiting for Game Execution.")
    import time
    while True:
        time.sleep(1)
