"""
GENESIS BRIDGE - Unreal Engine <-> Python Communication
Socket-based command protocol
"""

import socket
import json
import threading
import time
from Sarah_Logcat import info, debug, warning, error
from Sovereign_Math import SovereignMath

class GenesisBridge:
    """Bidirectional communication between Sarah and Unreal Engine."""
    
    def __init__(self, host='127.0.0.1', port=9999):
        self.host = host
        self.port = port
        self.server_socket = None
        self.client_socket = None
        self.running = False
        self.math_engine = SovereignMath()
        
    def start_server(self):
        """Start listening for Unreal Engine connections."""
        self.server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        self.server_socket.bind((self.host, self.port))
        self.server_socket.listen(1)
        self.running = True
        
        info('bridge', f'Server listening on {self.host}:{self.port}')
        print(f"[BRIDGE] Server listening on {self.host}:{self.port}")
        
        # 1. Start File Watcher Thread IMMEDIATELY (Available Offline)
        file_thread = threading.Thread(target=self._file_watch_loop, daemon=True)
        file_thread.start()
        print("[BRIDGE] File Watcher/Chat Thread Started (Offline Mode Active)")

        # 2. Start Connection Loop Thread (Handles Reconnections)
        conn_thread = threading.Thread(target=self._connection_loop, daemon=True)
        conn_thread.start()
        print("[BRIDGE] Waiting for Unreal Engine connection(s)...")

    def _connection_loop(self):
        """Continuously accept connections from Unreal."""
        while self.running:
            try:
                # Accept connection (BLOCKING per loop)
                client, addr = self.server_socket.accept()
                
                # Close old connection if exists
                if self.client_socket:
                    try: self.client_socket.close()
                    except: pass
                
                self.client_socket = client
                
                info('bridge', f'Connected to Unreal Engine', address=str(addr))
                print(f"[BRIDGE] Connected to Unreal Engine at {addr}")
                
                # Start Socket Listening Thread for this client
                listen_thread = threading.Thread(target=self._socket_listen_loop, daemon=True)
                listen_thread.start()
                
            except Exception as e:
                print(f"[BRIDGE] Accept Error: {e}")
                time.sleep(1)
        
    def _file_watch_loop(self):
        """Watch for commands from file input (Offline/Online)."""
        import os
        while self.running:
            try:
                # 1. Trigger File
                trigger_path = "manifest_trigger.json"
                if os.path.exists(trigger_path):
                    with open(trigger_path, 'r') as f:
                        trigger_data = json.load(f)
                    
                    if self.client_socket:
                        self.send_command(trigger_data)
                        print(f"[BRIDGE] Triggered command -> Unreal: {trigger_data}")
                    else:
                        print("[BRIDGE] Command queued but Unreal NOT connected.")
                    
                    os.remove(trigger_path)

                # 2. User Input File (Chat)
                user_input_path = "user_input.json"
                if os.path.exists(user_input_path):
                    try:
                        with open(user_input_path, 'r') as f:
                            user_data = json.load(f)
                        
                        user_message = user_data.get("message", "")
                        print(f"[BRIDGE] User says: {user_message}")
                        
                        response_text = f"Sarah acknowledges: {user_message}"
                        
                        density = self.math_engine.calculate_theory_density(user_message)
                        resonance = self.math_engine.get_resonance_flux(user_message)
                        
                        response_header = f"[LOGIC: {density:.4f} | FLUX: {resonance:.4f}]"
                        
                        if "manifest" in user_message.lower():
                            if self.client_socket:
                                try:
                                    # [27-POINT CUBIC MANIFESTATION]
                                    # Instead of 1 cube, we should prepare to spawn the lattice.
                                    # For now, we spawn the ANCHOR.
                                    manifest_cmd = {
                                        "command": "manifest",
                                        "x": 0.0,
                                        "y": 0.0,
                                        "z": 1000.0,
                                        "label": "Sovereign_Anchor_Chat"
                                    }
                                    self.send_command(manifest_cmd)
                                    response_text = f"{response_header} Manifesting Sovereign Anchor in Unreal."
                                except Exception as e:
                                    print(f"[BRIDGE] Failed to send command: {e}")
                                    self._handle_connection_error()
                                    response_text = f"{response_header} Unreal Engine connection lost."
                            else:
                                response_text = f"{response_header} Unreal Engine disconnected. Cannot manifest."
                        
                        elif "hello" in user_message.lower():
                            response_text = f"{response_header} Sovereign Link Online. Logic Density Analyzed."
                            
                            response_text = f"{response_header} AUDIT COMPLETE. Bridge connected to Sovereign Logic Core."

                        elif "build" in user_message.lower() and "world" in user_message.lower():
                            if self.client_socket:
                                try:
                                    response_text = f"{response_header} INITIATING WORLD CONSTRUCTION SEQUENCE (27-Point Lattice)..."
                                    # [WORLD BUILDER]
                                    anchor = 1.09277703703703
                                    spacing = 500.0 * anchor
                                    
                                    count = 0
                                    for x in [-1, 0, 1]:
                                        for y in [-1, 0, 1]:
                                            for z in [-1, 0, 1]:
                                                cmd = {
                                                    "command": "manifest",
                                                    "x": x * spacing,
                                                    "y": y * spacing,
                                                    "z": (z * spacing) + 1000.0,
                                                    "label": f"World_Node_{x}_{y}_{z}"
                                                }
                                                self.send_command(cmd)
                                                count += 1
                                                time.sleep(0.05) # Pulse Pacing
                                    
                                    response_text += f" [SUCCESS] Manifested {count} Logic Nodes in Unreal Space."
                                except Exception as e:
                                    response_text = f"{response_header} World Build Failed: {e}"
                            else:
                                response_text = f"{response_header} Cannot build world. Unreal Engine disconnected."
                            
                        else:
                            # [SOVEREIGN VOICE]
                            # Connect to Fast Brain for intelligent response
                            try:
                                from Sarah_Fast_Brain import ask_sarah
                                raw_reply = ask_sarah(user_message)
                                response_text = f"{response_header} {raw_reply}"
                            except Exception as e:
                                response_text = f"{response_header} [BRAIN ERROR]: {e}"

                        # Write Response
                        response_data = {"response": response_text, "timestamp": time.time()}
                        with open("sarah_response.json", "w") as f:
                            json.dump(response_data, f)
                        
                        os.remove(user_input_path)
                    except Exception as e:
                        print(f"[BRIDGE] Error processing user input: {e}")
                        time.sleep(1)

                time.sleep(0.5) # Poll interval
            except Exception as e:
                 print(f"[BRIDGE] File Watch Loop error: {e}")
                 time.sleep(1)

    def _socket_listen_loop(self):
        """Listen for commands from Unreal Engine Socket."""
        while self.running and self.client_socket:
            try:
                self.client_socket.settimeout(1.0)
                try:
                    data = self.client_socket.recv(4096)
                    if data:
                        message = json.loads(data.decode('utf-8'))
                        self._handle_command(message)
                    else:
                        print("[BRIDGE] Empty data received. Closing connection.")
                        self._handle_connection_error()
                        break
                except socket.timeout:
                    continue
                except ConnectionResetError:
                    print("[BRIDGE] Unreal Engine disconnected.")
                    self._handle_connection_error()
                    break
            except Exception as e:
                print(f"[BRIDGE] Socket Loop error: {e}")
                self._handle_connection_error()
                break
    
    def _handle_connection_error(self):
        """Clean up connection state."""
        if self.client_socket:
            try:
                self.client_socket.close()
            except:
                pass
            self.client_socket = None
            print("[BRIDGE] Client socket cleared. Ready for reconnection.")

    def _handle_command(self, message):
        """Process commands from Unreal Engine."""
        cmd = message.get('command')
        debug('bridge', f'Received command: {cmd}', command_data=message)
        print(f"[BRIDGE] Received command: {cmd}")
        
        if cmd == 'click':
            from Genesis_Vision import GenesisVision
            vision = GenesisVision()
            vision.execute_click(message['x'], message['y'])
        
        elif cmd == 'type':
            from Genesis_Vision import GenesisVision
            vision = GenesisVision()
            vision.type_text(message['text'])
        
        elif cmd == 'execute':
            from Genesis_API import GenesisAPI
            api = GenesisAPI()
            result = api.execute_command(message['command'])
            self.send_response({'status': 'success', 'result': result})

        elif cmd == 'chat':
            # [SOVEREIGN EDITOR LINK]
            user_message = message.get('text', '')
            print(f"[BRIDGE] Socket Chat: {user_message}")
            
            try:
                # 1. Math Analysis
                density = self.math_engine.calculate_theory_density(user_message)
                resonance = self.math_engine.get_resonance_flux(user_message)
                
                # 2. Ask Sarah
                from Sarah_Fast_Brain import ask_sarah
                reply = ask_sarah(user_message)
                
                # 3. Send Response
                response_payload = {
                    'command': 'chat_response',
                    'text': reply,
                    'logic': density,
                    'flux': resonance,
                    'timestamp': time.time()
                }
                self.send_response(response_payload)
                
                # [WORLD MANIFESTATION TRIGGER]
                if "manifest" in user_message.lower() or "build" in user_message.lower():
                     # Trigger logic handles manifestation separately via internal logic
                     # But we can also signal the C++ editor to render something
                     pass
                     
            except Exception as e:
                self.send_response({'command': 'error', 'text': str(e)})
    
    def send_command(self, command_dict):
        """Send command to Unreal Engine."""
        if self.client_socket:
            try:
                message = json.dumps(command_dict).encode('utf-8')
                self.client_socket.send(message)
                print(f"[BRIDGE] Sent to Unreal: {command_dict}")
            except Exception as e:
                 print(f"[BRIDGE] Send Error: {e}")
                 self._handle_connection_error()
    
    def send_response(self, response_dict):
        """Send response back to Unreal Engine."""
        self.send_command(response_dict)
    
    def stop(self):
        """Shutdown the bridge."""
        self.running = False
        if self.client_socket:
            self.client_socket.close()
        if self.server_socket:
            self.server_socket.close()
        print("[BRIDGE] Bridge closed")

if __name__ == "__main__":
    bridge = GenesisBridge()
    bridge.start_server()
    
    # Keep server running
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        bridge.stop()
