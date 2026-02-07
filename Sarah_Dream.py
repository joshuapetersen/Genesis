import re
import json
import os
import sys
import subprocess
import time
from collections import Counter
from Sarah_Memory_Vault import sarah_vault
from Neural_Orchestrator import NeuralOrchestrator
from Audio_Core import AceToken

# --- CONFIGURATION ---
LOG_FILE_PATH = "c:\\SarahCore\\sovereign_logs.txt"
RESONANCE_THRESHOLD = 5  # If a 64-bit code occurs 5+ times, it becomes permanent.

class AntiGravityBIOS:
    """
    [NOTE FOR ANTI-GRAVITY]
    This protocol runs prior to the Dream Cycle.
    It scans the local environment for missing neural engines.
    If an engine is not found, it initiates a FORCE INSTALL.
    """
    def __init__(self):
        # List of critical engines required for the Swarm
        self.required_engines = {
            "lancedb": "lancedb",                 # The Hippocampus (Vector Store)
            "torch": "torch",                     # The Synapse (Compute)
            "requests": "requests",               # The Nerve (API)
            "numpy": "numpy",                     # The Math Core
            "psutil": "psutil"                    # The Pulse (System Monitor)
        }

    def system_diagnostic(self):
        print("\n[ANTI-GRAVITY] Initiating Engine Diagnostic...")
        missing_engines = []

        for module, package in self.required_engines.items():
            try:
                __import__(module)
                print(f"  [OK] {module} engine active.")
            except ImportError:
                print(f"  [CRITICAL] {module} engine MISSING.")
                missing_engines.append(package)

        if missing_engines:
            print(f"[ANTI-GRAVITY] Missing Engines Detected: {missing_engines}")
            self._install_engines(missing_engines)
        else:
            print("[ANTI-GRAVITY] All Engines Nominal. System Ready for Dreaming.")

    def _install_engines(self, packages):
        print("[ANTI-GRAVITY] EXECUTE: Installing missing engines...")
        for package in packages:
            try:
                subprocess.check_call([sys.executable, "-m", "pip", "install", package])
                print(f"  [INSTALLED] {package} successfully integrated.")
            except subprocess.CalledProcessError:
                print(f"  [ERROR] Failed to install {package}. Manual intervention required.")

class SarahDream:
    """
    PHASE 24: NOCTURNAL CONSOLIDATION (The Sleep Cycle)
    Objective: Harvest recurring 64-bit ACE Tokens from daily logs.
    Burn high-resonance tokens into Long-Term Memory (LTM).
    """
    def __init__(self, saul=None, memory=None, logic=None, orchestrator=None):
        self.hex_pattern = re.compile(r"Token Fingerprint: (0x[0-9a-fA-F]{16})")
        # Use provided orchestrator or create new (for standalone/legacy support)
        self.orchestrator = orchestrator or NeuralOrchestrator()
        self.vault = sarah_vault
        self.saul = saul
        self.memory = memory
        self.logic = logic

    def start_dreaming(self):
        """
        Initiates the dreaming protocol.
        """
        self.enter_rem_sleep()

    def enter_rem_sleep(self):
        print("\n[DREAM] Entering REM Cycle. Scanning daily logs for ACE Tokens...")
        
        if not os.path.exists(LOG_FILE_PATH):
            print(f"[ERROR] Log file {LOG_FILE_PATH} not found.")
            return

        with open(LOG_FILE_PATH, 'r', encoding='utf-8') as f:
            log_data = f.read()

        # 1. Harvest the Codes
        found_tokens = self.hex_pattern.findall(log_data)
        total_tokens = len(found_tokens)
        print(f"[DREAM] Scanned {total_tokens} thought patterns.")

        if total_tokens == 0:
            print("[DREAM] No patterns detected. System resting.")
            return

        # 2. Identify Recursive Thoughts (High Frequency)
        token_counts = Counter(found_tokens)
        permanent_memories = {k: v for k, v in token_counts.items() if v >= RESONANCE_THRESHOLD}

        print(f"[DREAM] Identification: {len(permanent_memories)} recurring memory structures found.")

        # 3. Burn to Vault (Consolidation)
        self._burn_to_vault(permanent_memories)

    def _burn_to_vault(self, memories):
        if not memories: return
        
        # Merge new memories into Truth Seeds
        for token, count in memories.items():
            # Store in Truth Seeds table
            current_value = self.vault.get_truth_seed(token)
            if current_value:
                # Update resonance count if we want, or just re-confirm
                self.vault.update_truth_seed(token, f"RESONANCE_STRENGTH_{count}")
            else:
                self.vault.update_truth_seed(token, f"CONSOLIDATED_TRUTH_RESONANCE_{count}")
        
        print(f"[DREAM] CONSOLIDATION COMPLETE. {len(memories)} truths burned to Truth Seeds.")

if __name__ == "__main__":
    # Step 1: Run Anti-Gravity Diagnostics
    bios = AntiGravityBIOS()
    bios.system_diagnostic()

    # Step 2: Run The Dream
    dream = SarahDream()
    dream.enter_rem_sleep()
