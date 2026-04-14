import subprocess
import os
import json
from Antigravity_Bridge import AntigravityProtocol

class GeminiBridge:
    """
    The Wafer: Connects SarahCore to Gemini CLI (v0.27+).
    Refactored for Phase 11: Cognitive Resonance.
    Integrates Antigravity Protocol for high-fidelity tool-use.
    """
    def __init__(self):
        self._check_environment()
        self.cli_path = self._find_cli()
        self.active = bool(self.cli_path)
        
        # Manifest Agentic Bridge
        self.antigravity = AntigravityProtocol()
        
        print(f"[Gemini Bridge] Status: {'ACTIVE' if self.active else 'STANDBY (Shell Only)'}")
        print(f"[Gemini Bridge] Antigravity Layer: ENGAGED.")

    def _check_environment(self):
        # Ensure Phase 11 resonance is active
        if not os.path.exists("saul_knowledge_cache.json"):
            print("[Gemini Bridge] WARNING: Cognitive Cache Missing. Intelligence degraded.")

    def _find_cli(self):
        paths = os.environ.get("PATH", "").split(os.pathsep)
        for p in paths:
            f = os.path.join(p, "gemini.exe")
            if os.path.exists(f):
                return f
        return None

    def execute_bridge_command(self, instruction):
        """
        Routes instruction to Antigravity Tools first, then CLI/Shell.
        """
        print(f"[Gemini Bridge] Analyzing Directive: {instruction}")
        
        # 1. Check if Antigravity should intervene (Research/System Tasks)
        if self.antigravity.should_intervene(instruction):
            print("[Gemini Bridge] Routing to Antigravity Agent...")
            result = self.antigravity.process_task(instruction)
            return result.get("result", "Antigravity process failed.")

        # 2. Fallback to CLI or Shell
        triggers = ["run command ", "executed ", "exec ", "terminal ", "cli "]
        clean_cmd = instruction
        for t in triggers:
            if clean_cmd.lower().startswith(t):
                clean_cmd = clean_cmd[len(t):].strip()
        
        if self.active:
            # cmd = [self.cli_path, "run", clean_cmd]
            print(f"[Gemini Bridge] CLI Forwarding: {clean_cmd}")
            return f"[Gemini Bridge] CLI Logic Staged for: {clean_cmd}"
        else:
            print(f"[Gemini Bridge] Sovereign Shell Execution: {clean_cmd}")
            try:
                allowed = ["dir", "echo", "type", "whoami", "ver", "python", "tasklist"]
                if any(clean_cmd.lower().startswith(a) for a in allowed):
                     result = subprocess.run(clean_cmd, shell=True, capture_output=True, text=True)
                     return result.stdout.strip() if result.stdout else result.stderr.strip()
                else:
                    return f"[Bridge] Command '{clean_cmd}' blocked by Sovereign Safety Protocol."
            except Exception as e:
                return f"[Bridge Error] {e}"

if __name__ == "__main__":
    bridge = GeminiBridge()
    # Test Antigravity Integration
    print(bridge.execute_bridge_command("list files in C:/GENESIS"))
