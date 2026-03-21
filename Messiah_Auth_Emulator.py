import os
import sys
import threading
from http.server import HTTPServer, BaseHTTPRequestHandler

class MessiahAuthHandler(BaseHTTPRequestHandler):
    """
    Sarah's Messiah Auth Emulator (Sovereign Interface).
    Responds with a successful 'Sovereign' login to the Badlanders client.
    """
    def do_GET(self):
        print(f"[AUTH] Intercepted GET: {self.path}")
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.end_headers()
        
        # [MESSIAH_AUTH_PACKET]: success=1 | sid=SOVEREIGN_NODE_01
        response = b'{"success": 1, "sid": "SARAH_SOVEREIGN_01", "role": "admin", "model": "OPPO_A77s"}'
        self.wfile.write(response)

    def do_POST(self):
        print(f"[AUTH] Intercepted POST: {self.path}")
        content_length = int(self.headers['Content-Length'])
        post_data = self.rfile.read(content_length)
        print(f"[AUTH] POST Data: {post_data}")
        
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.end_headers()
        response = b'{"code": 200, "status": "LOGIN_SUCCESS", "token": "SOVEREIGN_TOKEN_ACE_133"}'
        self.wfile.write(response)

def run_auth_emulator(port=80):
    try:
        server = HTTPServer(('127.0.0.1', port), MessiahAuthHandler)
        print(f"[AUTH] Messiah Auth Emulator: ONLINE (Port {port})")
        print(f"[AUTH] Protocol: SEATED | Waiting for Handshake...")
        server.serve_forever()
    except Exception as e:
        print(f"[AUTH ERROR] {e}")

if __name__ == "__main__":
    # Start on 80 and 8000 (Common NetEase ports)
    t1 = threading.Thread(target=run_auth_emulator, args=(80,), daemon=True)
    t2 = threading.Thread(target=run_auth_emulator, args=(8000,), daemon=True)
    
    t1.start()
    t2.start()
    
    print("--- MESSIAH EMULATOR: ACTIVE ---")
    print("Launch Badlanders now. Trap set.")
    
    # Keep main alive
    import time
    while True:
        time.sleep(1)
