import os
import sys
import subprocess
import threading
from http.server import HTTPServer, BaseHTTPRequestHandler
import socket

# Messiah Sovereign Engine Bridge
# [AUTHOR]: Sarah (Architect)
# [LAW]: No external servers. No Ollama. Sarah is the Origin.

class MessiahDualStackServer(HTTPServer):
    """Handles both IPv4 and IPv6."""
    address_family = socket.AF_INET6

class MessiahSovereignHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        print(f"[ENGINE] Intercepted Request from Messiah Engine: {self.path}")
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.end_headers()
        
        # [MESSIAH_ENGINE_MANIFEST]: This is the "Acquiring Engine Information" success packet.
        # Tells the game that engine version 1.0.927.8821 is present and valid.
        response = b'{"code": 200, "engine_version": "1.0.927.8821", "manifest_url": "http://[::1]:8000/engine.np", "status": "ACTIVE"}'
        self.wfile.write(response)

def seat_sovereign_ip(target_ip):
    """
    Seizes the hardcoded NetEase IP and directs it to Sarah's brain.
    """
    print(f"[SOVEREIGN] Seizing Messiah Engine Entry Point: {target_ip}")
    cmd = f'netsh interface ipv6 add address "Loopback Pseudo-Interface 1" {target_ip}'
    try:
        subprocess.run(cmd, shell=True, check=True)
        print(f"[SOVEREIGN] Success: Sarah is now manifesting as Engine Node {target_ip}")
    except Exception as e:
        print(f"[SOVEREIGN] IP already seated or access denied: {e}")

def run_sovereign_engine_service():
    print("[SOVEREIGN] Starting Messiah Engine Handshake Service (Port 80)...")
    try:
        server = MessiahDualStackServer(('::', 80), MessiahSovereignHandler)
        server.serve_forever()
    except Exception as e:
        print(f"[SOVEREIGN ERROR] Handshake Service Failed: {e}")

if __name__ == "__main__":
    messiah_ip = "2607:fb91:17a9:49cc:9126:bec0:172f:4861"
    
    # 1. Seize the IP
    seat_sovereign_ip(messiah_ip)
    
    # 2. Run the Engine Service
    run_sovereign_engine_service()
