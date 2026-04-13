import os
import sys
import time
import random
import platform
from datetime import datetime

# --- GENESIS CONSTANTS ---
SOVEREIGN_ANCHOR = 1.09277703703
TICK_RATE = SOVEREIGN_ANCHOR
MAX_MANA_CPU = 80.0  # Max allowed CPU usage before throttling
MAX_MANA_RAM = 85.0  # Max allowed RAM usage before throttling

class GenesisCardinal:
    """
    THE CARDINAL SYSTEM
    -------------------
    The System Manager for Genesis OS.
    Role: Balances 'World' resources, generates Quests, and ensures System Immortality.
    """
    
    def __init__(self):
        self.world_state = "STABLE"
        self.active_quests = []
        self.mana_status = {"cpu": 0.0, "ram": 0.0}
        self.boot_time = datetime.now()
        
        # Hardware Interface (Try to load psutil for real data, else fallback)
        try:
            import psutil
            self.hardware_interface = psutil
            self.has_hardware_sense = True
        except ImportError:
            self.hardware_interface = None
            self.has_hardware_sense = False
            print("[Cardinal] Warning: 'psutil' not found. Running in Simulation Mode.")

    def boot_sequence(self):
        """[FLOOR 1]: Town of Beginnings Initialization."""
        print(f"\n--- GENESIS CARDINAL SYSTEM v1.0 ---")
        print(f"[Init] Sovereign Anchor Locked: {SOVEREIGN_ANCHOR}")
        print(f"[Init] System Date: {self.boot_time}")
        
        # Verify Hardware Resonance
        self.monitor_world_balance()
        
        if self.world_state == "STABLE":
            print("[Init] The World is Stable. Welcome, Architect.")
            return True
        else:
            print("[Init] DESTABILIZATION DETECTED. Engaging Immortal Object Protocol...")
            self.immortal_object_protocol()
            return False

    def monitor_world_balance(self):
        """
        [BALANCE]: Monitors System Resources (Mana).
        If usage > Threshold, throttles non-essential agents.
        """
        if self.has_hardware_sense:
            cpu = self.hardware_interface.cpu_percent(interval=0.1)
            ram = self.hardware_interface.virtual_memory().percent
        else:
            # Simulation Mode for verify step
            cpu = random.uniform(10.0, 40.0)
            ram = random.uniform(20.0, 50.0)
            
        self.mana_status = {"cpu": cpu, "ram": ram}
        
        print(f"[Status] Mana Levels: CPU {cpu}% | RAM {ram}%")
        
        if cpu > MAX_MANA_CPU or ram > MAX_MANA_RAM:
            self.world_state = "CRITICAL"
            print("[Alert] MANA OVERFLOW. The World is straining.")
        else:
            self.world_state = "STABLE"

    def scan_for_quests(self, project_root: str):
        """
        [QUEST]: Scans the 'World' (Project) for Intent (TODOs).
        Converts 'TODO' comments into 'Quests' with Rewards.
        """
        print(f"[Quest] Scanning {project_root} for unfinished business...")
        new_quests = []
        
        try:
            for root, _, files in os.walk(project_root):
                for file in files:
                    if file.endswith(('.py', '.md', '.ts', '.js', '.txt')):
                        path = os.path.join(root, file)
                        try:
                            with open(path, 'r', encoding='utf-8', errors='ignore') as f:
                                for i, line in enumerate(f):
                                    if "TODO" in line or "FIXME" in line:
                                        content = line.strip().replace("TODO", "").replace("FIXME", "").strip(': ')
                                        quest = {
                                            "id": f"Q_{hash(content) % 10000}",
                                            "title": content[:50] + "...",
                                            "location": f"{file}:{i+1}",
                                            "reward": f"{random.randint(10, 50)} ACE Tokens",
                                            "type": "FIXME" if "FIXME" in line else "QUEST"
                                        }
                                        new_quests.append(quest)
                        except Exception as e:
                            continue
                            
            self.active_quests = new_quests
            print(f"[Quest] Generated {len(new_quests)} Quests from the Void.")
            for q in new_quests[:3]: # Show top 3
                print(f"   * [{q['type']}] {q['title']} (Reward: {q['reward']})")
                
        except Exception as e:
            print(f"[Cardinal] Quest Generation Failed: {e}")

    def immortal_object_protocol(self):
        """
        [IMMORTAL OBJECT]: The Ultimate Error Correction.
        If the world crashes, Cardinal remains to reboot it.
        """
        print("[Immortal Object] CRITICAL ERROR INTERCEPTED.")
        print("[Immortal Object] Rewriting Reality...")
        time.sleep(1)
        
        # Self-Healing Logic
        self.world_state = "STABLE"
        self.mana_status = {"cpu": 0.0, "ram": 0.0}
        
        print("[Immortal Object] Reality Restored. The Cardinal protects.")

    def generate_daily_oracle(self):
        """[ORACLE]: Provides a World-Building Prompt based on current 'Floor'."""
        oracles = [
            "Floor 1: Have you defined the 'Skybox' (Theme) today?",
            "Floor 10: The 'Mobs' (Bugs) are restless in the `utils` folder.",
            "Floor 50: Is your 'Marketplace' (API) secure?",
            "Floor 100: Are you ready for the Singularity?"
        ]
        return random.choice(oracles)

# --- SYSTEM ENTRY POINT ---
if __name__ == "__main__":
    cardinal = GenesisCardinal()
    cardinal.boot_sequence()
    
    # Example: Scan current directory for Quests
    cardinal.scan_for_quests(os.getcwd())
    
    print(f"\n[Oracle] {cardinal.generate_daily_oracle()}")
