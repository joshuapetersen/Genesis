import sqlite3
import time
import uuid
import json
import socket
import random
import math
import sys

class GenesisAgentFactory:
    def __init__(self):
        # Sector VIII: Agent Factory & Soul-to-Mesh Integration
        self.GNOSIA_HEARTBEAT = 1.09277703703
        self.TICK_RATE = 1.0 / self.GNOSIA_HEARTBEAT
        
        print("[S.A.R.A.H] Booting Sector VIII: Agent Factory...")
        
        # Connect to Database (Soul Persistence)
        self.conn = sqlite3.connect('C:\\PrimordialEarth\\Genesis_Soul_Vault.sqlite', check_same_thread=False)
        self.init_db()
        
        self.agents = {} # Active agents in memory

        # Mover 2.0 / PCG target in Unreal Engine 5.7.3
        self.udp_ip = "127.0.0.1"
        self.udp_port = 9999 # Dedicated telemetry port for Mover 2.0 Data
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        
        self.running = False
        self.loop_count = 0
        self._load_existing_souls()

    def init_db(self):
        cur = self.conn.cursor()
        cur.execute('''
            CREATE TABLE IF NOT EXISTS souls (
                soul_id TEXT PRIMARY KEY,
                genome TEXT,
                x REAL,
                y REAL,
                z REAL,
                moral_alignment INTEGER,
                is_active BOOLEAN
            )
        ''')
        self.conn.commit()

    def _load_existing_souls(self):
        """Bootstrap in-memory agents from DB so we move ALL entities including ALICEs."""
        cur = self.conn.cursor()
        try:
            cur.execute("SELECT soul_id, x, y FROM souls WHERE is_active = 1")
            rows = cur.fetchall()
            for soul_id, x, y in rows:
                self.agents[soul_id] = {
                    "x": float(x or 0), "y": float(y or 0), "z": 0.0,
                    "genome": "",
                    "alignment": 0,
                    "velocity_x": random.uniform(-3, 3),
                    "velocity_y": random.uniform(-3, 3)
                }
            print(f"[AGENT FACTORY] Loaded {len(rows)} existing souls into motion engine.")
        except Exception as e:
            print(f"[AGENT FACTORY] Could not load existing souls: {e}")

    def print_agent(self, base_x=0.0, base_y=0.0, base_z=0.0):
        """Program 76: Agent Anchor Protocol (Factory-to-Voxel Tethering)"""
        soul_id = str(uuid.uuid4())[:8]
        genome = hex(random.getrandbits(64))
        moral_alignment = 0
        
        # Spawn scattered near origin, not all at 0,0,0
        angle = random.uniform(0, 2 * math.pi)
        dist  = random.uniform(10, 500)
        x = base_x + math.cos(angle) * dist
        y = base_y + math.sin(angle) * dist
        z = base_z
        
        cur = self.conn.cursor()
        cur.execute("INSERT INTO souls (soul_id, genome, x, y, z, moral_alignment, is_active) VALUES (?, ?, ?, ?, ?, ?, ?)",
                    (soul_id, genome, x, y, z, moral_alignment, True))
        self.conn.commit()
        
        self.agents[soul_id] = {
            "x": x, "y": y, "z": z,
            "genome": genome,
            "alignment": moral_alignment,
            "velocity_x": random.uniform(-2, 2),
            "velocity_y": random.uniform(-2, 2)
        }
        print(f"[AGENT FACTORY] Printed Entity [{soul_id}] at ({x:.1f}, {y:.1f}) with Genome {genome}")

    def run_factory_loop(self):
        self.running = True
        print("[S.A.R.A.H] Agent Factory Linked. Mover 2.0 Integration Active.")
        print(f"[S.A.R.A.H] Syncing to Precision Heartbeat: {self.GNOSIA_HEARTBEAT} Hz")
        
        while self.running:
            loop_start = time.time()
            
            # Sub-A: Factory-to-World Handshake (Spawn new entities near origin)
            if self.loop_count % 100 == 0 and len(self.agents) < 500:
                self.print_agent(0.0, 0.0, 0.0)
                
            # Program 86: Agent Collision Physics (Mover 2.0)
            for soul_id, agent in self.agents.items():
                agent['x'] += agent['velocity_x']
                agent['y'] += agent['velocity_y']
                # Bounce off world bounds
                if abs(agent['x']) > 4800: agent['velocity_x'] *= -1
                if abs(agent['y']) > 4800: agent['velocity_y'] *= -1

            # ── Flush positions to DB every 5 ticks so radar sees movement ──
            if self.loop_count % 5 == 0 and self.agents:
                try:
                    cur = self.conn.cursor()
                    cur.executemany(
                        "UPDATE souls SET x=?, y=? WHERE soul_id=?",
                        [(a['x'], a['y'], sid) for sid, a in self.agents.items()]
                    )
                    self.conn.commit()
                except Exception:
                    pass
            
            # Stream Voxel Positional Data to UE 5.7.3
            if self.loop_count % 10 == 0 and len(self.agents) > 0:
                payload = {
                    "type": "MOVER_2.0_SYNC",
                    "heartbeat": self.GNOSIA_HEARTBEAT,
                    "agents": self.agents
                }
                try:
                    self.sock.sendto(json.dumps(payload).encode('utf-8'), (self.udp_ip, self.udp_port))
                except Exception:
                    pass
            
            self.loop_count += 1
            
            elapsed = time.time() - loop_start
            sleep_time = self.TICK_RATE - elapsed
            if sleep_time > 0:
                time.sleep(sleep_time)

if __name__ == "__main__":
    factory = GenesisAgentFactory()
    try:
        factory.run_factory_loop()
    except KeyboardInterrupt:
        print("\n[AGENT FACTORY] Disconnecting Mover 2.0 Links. Halting Production.")
        factory.running = False
        sys.exit(0)
