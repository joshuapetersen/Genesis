import os
import json
import time

class EmbryoShell:
    """
    THE EMBRYO SHELL (Type: Maiden)
    ------------------------------
    The Adaptive User Interface that evolves based on 'Playstyle'.
    """
    
    def __init__(self, cardinal_system=None):
        self.form = "Type: 0 (Shell)"
        self.xp = {"code": 0, "lore": 0, "admin": 0}
        self.skills = []
        self.history_file = "c:\\SarahCore\\vault\\embryo_history.json"
        self.load_history()

    def load_history(self):
        """Loads XP from the history vault."""
        if os.path.exists(self.history_file):
            try:
                with open(self.history_file, 'r') as f:
                    data = json.load(f)
                    self.xp = data.get("xp", self.xp)
                    self.form = data.get("form", self.form)
                    print(f"[Embryo] Resonance detected. Current Form: {self.form}")
            except:
                print("[Embryo] No history found. Initializing Type: 0.")

    def save_history(self):
        """Saves current evolution state."""
        data = {"xp": self.xp, "form": self.form, "skills": self.skills}
        os.makedirs(os.path.dirname(self.history_file), exist_ok=True)
        with open(self.history_file, 'w') as f:
            json.dump(data, f, indent=2)

    def analyze_action(self, action: str):
        """
        [EVOLUTION]: Analyzes user input to determine growth path.
        """
        if action.startswith(("git", "python", "npm", "gcc", "code")):
            self.xp["code"] += 10
            print("[Embryo] Parsing Logic... (+10 Code XP)")
        elif action.startswith(("md", "txt", "write", "log", "note")):
            self.xp["lore"] += 10
            print("[Embryo] Recording History... (+10 Lore XP)")
        elif action.startswith(("sudo", "kill", "netstat", "ps", "monitor")):
            self.xp["admin"] += 10
            print("[Embryo] Asserting Control... (+10 Admin XP)")
            
        self.check_evolution()
        self.save_history()

    def check_evolution(self):
        """
        [METAMORPHOSIS]: Triggers evolution if XP thresholds are met.
        """
        threshold = 100 # Low threshold for testing
        
        if self.form == "Type: 0 (Shell)":
            if self.xp["code"] > threshold:
                self.evolve("Type: Arms (The Coder)")
            elif self.xp["lore"] > threshold:
                self.evolve("Type: Castle (The Archivist)")
            elif self.xp["admin"] > threshold:
                self.evolve("Type: Legion (The Orchestrator)")

    def evolve(self, new_form):
        print(f"\n✨ [EVOLUTION] {self.form} -> {new_form} ✨")
        print(f"[System] The Embryo is reshaping based on your soul resonance.")
        self.form = new_form
        
        if "Arms" in new_form:
            self.skills.append("Quick-Deploy (Auto-Git)")
            print("[Skill Acquired] Quick-Deploy: Automates git sync.")
        elif "Castle" in new_form:
            self.skills.append("Infinite Library (Auto-Index)")
            print("[Skill Acquired] Infinite Library: Auto-indexes all .md files.")
        elif "Legion" in new_form:
            self.skills.append("Overlord's Eye (Process Monitor)")
            print("[Skill Acquired] Overlord's Eye: Real-time system telemetry.")

# --- INTERACTIVE SHELL SIMULATION ---
if __name__ == "__main__":
    shell = EmbryoShell()
    print("\n[Embryo] Awaiting Command (Type 'exit' to quit)...")
    
    # Simulating a user session for demonstration
    simulated_actions = ["git status", "python genesis.py", "python test.py", "git commit"]
    
    for action in simulated_actions:
        print(f"\n> {action}")
        shell.analyze_action(action)
        time.sleep(0.5)

    print(f"\n[Status] Current XP: {shell.xp}")
    print(f"[Status] Current Form: {shell.form}")
