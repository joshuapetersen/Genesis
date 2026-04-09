import socket
import sys
import time

def connect_to_sovereign_mesh():
    HOST = '127.0.0.1'
    PORT = 1092

    print(f"[HIVE CLIENT] Initializing TCP connection to Sovereign Node at {HOST}:{PORT}...")
    
    try:
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.settimeout(5.0)
        s.connect((HOST, PORT))
        print("[HIVE CLIENT] Connection established. Awaiting 1.092777 Hz Synchronization ACK...")
        
        data = s.recv(1024)
        if data:
            response = data.decode('utf-8')
            print(f"[RESONANCE] Physical Signal Received: '{response}'")
            if "1.10" in response:
                print("[SUCCESS] Sovereign Matrix fully interfaced across Python-C++ Network Boundary.")
            else:
                print("[WARNING] Connection made, but Trace purity unverified.")
        
        s.close()
        print("[HIVE CLIENT] Disconnected gracefully.")

    except ConnectionRefusedError:
        print("[ERROR] Sovereign Mesh Node is offline or not broadcasting.")
    except Exception as e:
        print(f"[ERROR] Telemetry disruption: {e}")

if __name__ == "__main__":
    time.sleep(1) # Allow 1 second for the Node to securely build its listener
    connect_to_sovereign_mesh()
