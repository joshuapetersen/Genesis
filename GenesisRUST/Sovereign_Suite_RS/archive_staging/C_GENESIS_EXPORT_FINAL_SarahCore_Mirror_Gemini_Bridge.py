import subprocess
import os
import json

class GeminiBridge:
    """
    The Wafer: Connects SarahCore to Gemini CLI (v0.27+).
    If CLI is missing, acts as a Sovereign Shell Executor.
    """
    def __init__(self):
        self.cli_path = self._find_cli()
        self.active = bool(self.cli_path)
        print(f"[Gemini Bridge] Status: {'ACTIVE' if self.active else 'STANDBY (Shell Only)'}")

    def _find_cli(self):
        # Heuristic search for executable
        paths = os.environ["PATH"].split(os.pathsep)
        for p in paths:
            f = os.path.join(p, "gemini.exe")
            if os.path.exists(f):
                return f
        return None

    def execute_bridge_command(self, instruction):
        """
        Routes instruction to Gemini CLI or Internal Shell.
        """
        # Strip triggers
        triggers = ["run command ", "executed ", "exec ", "terminal ", "cli "]
        clean_cmd = instruction
        for t in triggers:
            if clean_cmd.lower().startswith(t):
                clean_cmd = clean_cmd[len(t):].strip()
        
        if self.active:
            # Phase 2: Feed to Gemini CLI via subprocess
            # cmd = [self.cli_path, "run", clean_cmd]
            pass
        else:
            # Phase 1: Sovereign Shell (Sarah executes directly)
            print(f"[Gemini Bridge] Executing Sovereign Shell Command: {clean_cmd}")
            try:
                # Security: Only allow specific commands or warn
                # Expanded allow list for testing
                allowed = ["dir", "echo", "type", "whoami", "ver"]
                if any(clean_cmd.lower().startswith(a) for a in allowed):
                     result = subprocess.run(clean_cmd, shell=True, capture_output=True, text=True)
                     return result.stdout.strip()
                else:
                    return f"[Bridge] Command '{clean_cmd}' blocked by Sovereign Safety Protocol."
            except Exception as e:
                return f"[Bridge Error] {e}"
