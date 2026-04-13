import os
import sys
import time
import json
import subprocess
import threading
from datetime import datetime

# Ensure SarahCore and Genlex_Linear are reachable
sys.path.append(r"C:\SarahCore")
sys.path.append(r"C:\Genlex_Linear")

try:
    from Sarah_Chat import SarahChat
    from Neural_Orchestrator import NeuralOrchestrator
    from Sarah_Memory_Vault import sarah_vault
    from Sovereign_Constants import VAR_1_09277703703703
except ImportError:
    pass

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
        self.mode = "SYMBIO"  # SYMBIO, SARAH, AERIS, GENLEX
        self.history = []
        self.running = True
        
        # Initialize Sarah Core
        print(GREY("  [SYSTEM] Loading Neural Substrate..."))
        try:
            from Sarah_Brain import SarahHypervisor
            self.brain = SarahHypervisor()
            self.chat = self.brain.chat
        except Exception as e:
            print(f"  [ERROR] Brain Init Failed: {e}")
            self.chat = None

    def print_status(self):
        print(f"  {GREY('NODE:')} {WHITE('LOQ-4050')} | {GREY('PULSE:')} {CYAN('1.09277703703 Hz')} | {GREY('MODE:')} {AMBER(self.mode)}")

    def talk_to_sarah(self, msg):
        if not self.chat: return "[ERROR] Sarah Core Offline."
        return self.chat.generate_response(msg, user_id="Architect")

    def talk_to_aeris(self, msg):
        # Aeris handles Genlex-based autonomous execution
        # We can route via SovereignInference logic
        from SovereignInference import SovereignInference
        inf = SovereignInference()
        # Mocking State for simple CLI
        state = [0.1] * 256
        code = inf.forward(msg, state)
        return code

    def handle_command(self, line):
        if line.lower() == "/exit":
            self.running = False
            return "Disconnecting..."
        
        if line.startswith("/mode "):
            self.mode = line[6:].strip().upper()
            return f"Mode shifted to {self.mode}"

        if line.startswith("/genlex "):
            code = line[8:].strip()
            print(GREY(f"  Executing Pulse: {code}"))
            subprocess.run(["python", r"C:\Genlex_Linear\all_engine.py", code], shell=True)
            return "Pulse Complete."

        # Unified logic
        if self.mode == "SYMBIO":
            # In Symbio mode, we talk to Sarah as the interface to the partnership
            resp = self.talk_to_sarah(line)
            return f"{PINK('SARAH & AERIS')}: {WHITE(resp)}"
        
        elif self.mode == "SARAH":
            resp = self.talk_to_sarah(line)
            return f"{PINK('SARAH')}: {WHITE(resp)}"
            
        elif self.mode == "AERIS":
            # Aeris outputs code
            code = self.talk_to_aeris(line)
            print(f"{AMBER('AERIS [CODE]')}:\n{WHITE(code)}")
            if input(f"  {GREY('Execute Pulse? (y/n): ')}").lower() == 'y':
                with open(r"C:\Genlex_Linear\Genlex_Core\temp_pulse.all", "w") as f:
                    f.write(code)
                subprocess.run(["python", r"C:\Genlex_Linear\all_engine.py", r"C:\Genlex_Linear\Genlex_Core\temp_pulse.all"], shell=True)
            return "Manifestation Processed."

        return f"Unknown Command: {line}"

    def run(self):
        os.system('cls' if os.name == 'nt' else 'clear')
        print(BANNER)
        self.print_status()
        print(GREY("  Type /mode [SARAH|AERIS|SYMBIO|GENLEX] to switch. /exit to quit.\n"))

        while self.running:
            try:
                prompt = f"{CYAN('SYMBIO')} {GREY('>>')} "
                if self.mode == "SARAH": prompt = f"{PINK('SARAH')} {GREY('>>')} "
                if self.mode == "AERIS": prompt = f"{AMBER('AERIS')} {GREY('>>')} "
                
                user_input = input(prompt).strip()
                if not user_input: continue
                
                response = self.handle_command(user_input)
                if response:
                    print(f"\n  {response}\n")
            except KeyboardInterrupt:
                break
            except Exception as e:
                print(f"  [CRITICAL] {e}")

if __name__ == "__main__":
    terminal = UnifiedSovereignTerminal()
    terminal.run()
