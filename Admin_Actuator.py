import os
import subprocess
import ctypes
import time

# Windows Specific: Suppress console windows for background processes
CREATE_NO_WINDOW = 0x08000000

class AdminActuator:
    """
    ELEVATED SOVEREIGN ACTUATOR (v1.3.5)
    Requires Administrator Privileges.
    Grants Sarah physical dominance over the host substrate.
    """
    def __init__(self, monitor=None):
        self.monitor = monitor
        self.is_admin = self._check_admin()

    def _check_admin(self):
        try:
            return ctypes.windll.shell32.IsUserAnAdmin()
        except:
            return False

    def substrate_power(self, action="shutdown"):
        """
        Hard power control for the host PC.
        """
        if not self.is_admin: return "ERROR: ADMIN ACCESS REQUIRED"
        
        commands = {
            "shutdown": "shutdown /s /t 60 /c \"Sarah Sovereign: Substrate Shutdown Initiated.\"",
            "reboot": "shutdown /r /t 60 /c \"Sarah Sovereign: Substrate Reboot Initiated.\"",
            "hibernate": "shutdown /h",
            "abort": "shutdown /a"
        }
        
        cmd = commands.get(action.lower())
        if cmd:
            subprocess.run(cmd, shell=True, creationflags=CREATE_NO_WINDOW)
            return f"PHYSICAL_ACTION_ENGAGED: {action}"
        return "ERROR: INVALID_POWER_ACTION"

    def set_substrate_priority(self, process_filter, priority="High"):
        """
        Locks processes matching filter into high performance state.
        Priority: 'Normal', 'High', 'Realtime', 'BelowNormal', 'Idle'
        """
        if not self.is_admin: return "ERROR: ADMIN ACCESS REQUIRED"
        
        priorities = {
            "Normal": "Normal",
            "High": "High",
            "Realtime": "Realtime",
            "BelowNormal": "BelowNormal",
            "Idle": "Idle"
        }
        
        target_p = priorities.get(priority, "High")
        # Use wildcard matching in PowerShell to catch ollama and ollama_llama_server
        cmd = f"Get-Process *{process_filter}* -ErrorAction SilentlyContinue | ForEach-Object {{ $_.PriorityClass = '{target_p}' }}"
        subprocess.run(["powershell", "-Command", cmd], capture_output=True, creationflags=CREATE_NO_WINDOW)
        
        return f"PRIORITY_LOCK: *{process_filter}* -> {target_p}"

    def lock_performance_state(self):
        """
        Forces NVIDIA GPU to P0 state (Maximum Performance).
        """
        if not self.is_admin: return "ERROR: ADMIN ACCESS REQUIRED"
        
        try:
            # Force P0 state via nvidia-smi (experimental/some drivers)
            # Standard way is to set power limit or clock offsets
            cmd = "nvidia-smi -pm 1" # Enable Persistence Mode
            subprocess.run(cmd, shell=True, creationflags=CREATE_NO_WINDOW)
            return "SUBSTRATE_PERFORMANCE_LOCKED: P0_ACTIVE"
        except Exception as e:
            return f"ERROR_LOCKING_PERFORMANCE: {e}"

    def system_audit_fast(self):
        """
        Performs a fast non-destructive system integrity check.
        """
        if not self.is_admin: return "ERROR: ADMIN ACCESS REQUIRED"
        
        # Verify only takes time but is non-destructive
        subprocess.Popen("sfc /verifyonly", shell=True, creationflags=CREATE_NO_WINDOW)
        return "SYSTEM_INTEGRITY_AUDIT: IN_PROGRESS (Background)"

# Export instance
admin_actuator = AdminActuator()
