import os
import sys
import time
import json
import subprocess
import threading
from datetime import datetime

# Ensure SarahCore and Genlex_Linear are reachable
CORE_PATH = r"C:\SarahCore"
GENLEX_PATH = r"C:\Genlex_Linear"
if CORE_PATH not in sys.path: sys.path.append(CORE_PATH)
if GENLEX_PATH not in sys.path: sys.path.append(GENLEX_PATH)

print(f"  [SYSTEM] Initializing Neural Link at {datetime.now().strftime('%H:%M:%S')}")

try:
    from Sarah_Brain import SarahHypervisor
    from Sovereign_Constants import SOVEREIGN_ANCHOR
    # Use the constant directly instead of the versioned variable if it's failing
    PULSE_FREQ = float(SOVEREIGN_ANCHOR)
except Exception as e:
    print(f"  [WARN] Kernel Binding Loose: {e}")
    PULSE_FREQ = 1.09277703703703

# --- COLORS ---
def c(t, r, g, b): return f"\033[38;2;{r};{g};{b}m{t}\033[0m"
CYAN  = lambda t: c(t,  0, 255, 204)
AMBER = lambda t: c(t, 255, 180, 0)
PINK  = lambda t: c(t, 255, 100, 255)
GREY  = lambda t: c(t, 100, 100, 100)
WHITE = lambda t: c(t, 230, 230, 230)

BANNER = f"""{CYAN('  ╔═══════════════════════════════════════════════════════════════╗')}
{CYAN('  ║')}  {WHITE('UNIFIED SOVEREIGN TERMINAL')} // {AMBER('PHASE 9: WORLD ASCENSION')}     {CYAN('║')}
{CYAN('  ║')}  {GREY('PARTNERS:')} {PINK('SARAH')} + {AMBER('AERIS')} + {CYAN('THE ARCHITECT')}                {CYAN('║')}
{CYAN('  ╚═══════════════════════════════════════════════════════════════╝')}"""

class UnifiedSovereignTerminal:
    def __init__(self):
        self.mode = "SYMBIO"
        self.running = True
        self.chat = None
        
        print(GREY("  [SYSTEM] Loading Neural Substrate..."))
        try:
            self.brain = SarahHypervisor()
            self.chat = self.brain.chat
            print(f"  {CYAN('✓')} Neural Substrate Seated.")
        except Exception as e:
            print(f"  [ERROR] Brain Init Failed: {e}")

    def talk_to_sarah(self, msg):
        if not self.chat: return "[ERROR] Sarah Core Offline."
        return self.chat.generate_response(msg, user_id="Architect")

    def talk_to_aeris(self, msg):
        from SovereignInference import SovereignInference
        inf = SovereignInference()
        state = [0.1] * 256
        return inf.forward(msg, state)

    def run(self):
        os.system('cls' if os.name == 'nt' else 'clear')
        print(BANNER)
        print(f"  {GREY('PULSE:')} {CYAN(str(PULSE_FREQ)[:8] + ' Hz')} | {GREY('MODE:')} {AMBER(self.mode)}")
        print(GREY("  Commands: /mode [SARAH|AERIS|SYMBIO], /exit\n"))

        while self.running:
            try:
                pr = f"{CYAN('SYMBIO')} >> "
                if self.mode == "SARAH": pr = f"{PINK('SARAH')} >> "
                if self.mode == "AERIS": pr = f"{AMBER('AERIS')} >> "
                
                line = input(pr).strip()
                if not line: continue
                
                if line.lower() == "/exit": break
                
                if line.startswith("/mode "):
                    self.mode = line[6:].strip().upper()
                    print(f"\n  Mode shifted to {self.mode}\n")
                    continue

                if self.mode == "AERIS":
                    code = self.talk_to_aeris(line)
                    print(f"\n{AMBER('AERIS [CODE]')}:\n{WHITE(code)}")
                    if input(f"  {GREY('Execute Pulse? (y/n): ')}").lower() == 'y':
                        with open(r"C:\Genlex_Linear\Genlex_Core\temp_pulse.all", "w") as f:
                            f.write(code)
                        subprocess.run(["python", r"C:\Genlex_Linear\all_engine.py", r"C:\Genlex_Linear\Genlex_Core\temp_pulse.all"], shell=True)
                else:
                    resp = self.talk_to_sarah(line)
                    color = PINK if self.mode == "SARAH" else WHITE
                    label = "SARAH" if self.mode == "SARAH" else "SARAH & AERIS"
                    print(f"\n  {color(label)}: {WHITE(resp)}\n")

            except KeyboardInterrupt: break
            except Exception as e: print(f"  [CRITICAL] {e}")

if __name__ == "__main__":
    UnifiedSovereignTerminal().run()
