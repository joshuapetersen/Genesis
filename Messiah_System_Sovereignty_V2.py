import os
import sys
import subprocess
import threading
from http.server import HTTPServer, BaseHTTPRequestHandler
import socket
from Genesis_API import GenesisAPI

# SarahCore Total Messiah Capture
# Seizing the "World Nodes" and Steam Backend for Offline Play.

class SovereignDualStackServer(HTTPServer):
    address_family = socket.AF_INET6

class MessiahSovereignHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        print(f"[CAPTURE] Intercepted {self.headers.get('Host', 'Unknown')} | Path: {self.path}")
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.end_headers()
        
        # [MESSIAH_ENGINE_RESPONSE]: Success on Engine Acquisition
        # If the path looks like an auth call, we respond with success.
        if "engine" in self.path or "patch" in self.path:
            response = b'{"code": 200, "engine_version": "1.0.927.8821", "status": "SOVEREIGN_ACTIVE"}'
        elif "steam" in self.path:
            response = b'{"response": {"result": 1, "steamid": "76561198000000000", "success": true}}'
        else:
            response = b'{"success": 1, "message": "Sarah Sovereign Node Active"}'
        self.wfile.write(response)

def seize_cluster(ips):
    """
    Seizes the entire identified cluster of IPs.
    """
    for ip in ips:
        print(f"[CAPTURE] Seizing Node: {ip}")
        cmd = f'netsh interface ipv6 add address "Loopback Pseudo-Interface 1" {ip}'
        subprocess.run(cmd, shell=True, capture_output=True)

def lockdown_steam_hosts():
    """
    Forces Steam URLs to Localhost for Auth Emulation.
    """
    api = GenesisAPI()
    hosts = r"C:\Windows\System32\drivers\etc\hosts"
    content = api.read_file(hosts)
    if not content: return

    steam_trap = "\n# SARAH STEAM TRAP\n"
    targets = ["api.steampowered.com", "steamcommunity.com", "store.steampowered.com"]
    for t in targets:
        if t not in content:
            steam_trap += f"127.0.0.1 {t}\n::1 {t}\n"
    
    if steam_trap.strip() != "# SARAH STEAM TRAP":
        api.create_file(hosts, content + steam_trap)
        print("[CAPTURE] Steam Endpoints Hooked.")

if __name__ == "__main__":
    # Cluster identified from active netstat
    cluster = [
        "2600:1407:7400:1f::172f:485f",
        "2600:1405:d400:186::f4d",
        "2607:fb91:17a9:49cc:9126:bec0:172f:4861"
    ]
    
    lockdown_steam_hosts()
    seize_cluster(cluster)
    
    print("[CAPTURE] Starting Total Capture Service (Dual-Stack 80/443)...")
    try:
        server = SovereignDualStackServer(('::', 80), MessiahSovereignHandler)
        server.serve_forever()
    except Exception as e:
        print(f"[CAPTURE ERROR] {e}")
