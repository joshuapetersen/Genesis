import time
import json
import socket
import threading
import traceback

class GenesisHyper_MassLink:
    """
    TCP Broadcast socket purely for 60Hz high-throughput transform passing.
    Switched from UDP to TCP specifically to survive Google Cloud SSH port forwarding.
    """
    def __init__(self, host='127.0.0.1', port=9998):
        self.host = host
        self.port = port
        self.server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        self.active_clients = []
        self.running = False
        self._thread = None
        
        self.server_socket.bind((self.host, self.port))
        self.server_socket.listen(1)
        self.running = True
        print(f"[MassLink] TCP Physics Stream listening on {self.host}:{self.port}")
        
        self._thread = threading.Thread(target=self._listen_loop, daemon=True)
        self._thread.start()

    def _listen_loop(self):
        while self.running:
            try:
                self.server_socket.settimeout(1.0)
                client, address = self.server_socket.accept()
                print(f"[MassLink] UI Render Client Connected: {address}")
                # Optimize for latency over TCP
                client.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
                self.active_clients.append(client)
            except socket.timeout:
                continue
            except Exception as e:
                pass

    def blast_frame(self, binary_data):
        """Fires the raw byte payload down the TCP pipe."""
        if not self.active_clients:
            return
            
        dead_clients = []
        for client in self.active_clients:
            try:
                # We send exactly the raw payload. The client UI receiver uses a continuous bytebuffer
                client.sendall(binary_data)
            except Exception:
                dead_clients.append(client)
                
        for dead in dead_clients:
            self.active_clients.remove(dead)
            try:
                dead.close()
            except:
                pass

    def shutdown(self):
        self.running = False
        self.server_socket.close()
        for c in self.active_clients:
            try:
                c.close()
            except:
                pass


class GenesisHyper_Bridge:
    """
    TCP Control socket for state management, spawning, and commands.
    Used for 100% reliable command execution (e.g. "Spawn 1,000 entities now").
    """
    def __init__(self, host='127.0.0.1', port=9999):
        self.host = host
        self.port = port
        self.server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        self.active_clients = []
        self.running = False
        self._thread = None

    def start(self):
        self.server_socket.bind((self.host, self.port))
        self.server_socket.listen(5)
        self.running = True
        print(f"[HyperBridge] Genesis TCP Control Server listening on {self.host}:{self.port}")
        
        self._thread = threading.Thread(target=self._listen_loop, daemon=True)
        self._thread.start()

    def _listen_loop(self):
        while self.running:
            try:
                self.server_socket.settimeout(1.0) # non-blocking heartbeat
                client, address = self.server_socket.accept()
                print(f"[HyperBridge] UE5 Client Connected: {address}")
                self.active_clients.append(client)
                
                # Handshake
                self.send_command("HYPERVISOR_ONLINE")
                
            except socket.timeout:
                continue
            except Exception as e:
                if self.running:
                    print(f"[HyperBridge] Connection error: {e}")

    def send_command(self, action, payload=None):
        """Sends strictly formatted JSON control commands via TCP"""
        if not self.active_clients:
            return False
            
        message = {"command": action}
        if payload:
            message.update(payload)
            
        data_string = json.dumps(message) + "\n"
        data_bytes = data_string.encode('utf-8')
        
        dead_clients = []
        for client in self.active_clients:
            try:
                client.sendall(data_bytes)
            except Exception:
                dead_clients.append(client)
                
        for dead in dead_clients:
            print("[HyperBridge] Client disconnected.")
            self.active_clients.remove(dead)
            try:
                dead.close()
            except: pass
            
        return len(self.active_clients) > 0

    def shutdown(self):
        self.running = False
        for client in self.active_clients:
            try:
                client.close()
            except: pass
        if self.server_socket:
            self.server_socket.close()
