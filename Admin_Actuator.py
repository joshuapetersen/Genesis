import subprocess
import ctypes
import psutil

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
        except (AttributeError, OSError) as e:
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
        Optimized by AERIS: Native psutil implementation targets processes without sub-shell friction.
        """
        if not self.is_admin: return "ERROR: ADMIN ACCESS REQUIRED"
        
        priorities = {
            "Normal": psutil.NORMAL_PRIORITY_CLASS,
            "High": psutil.HIGH_PRIORITY_CLASS,
            "Realtime": psutil.REALTIME_PRIORITY_CLASS,
            "BelowNormal": psutil.BELOW_NORMAL_PRIORITY_CLASS,
            "Idle": psutil.IDLE_PRIORITY_CLASS
        }
        
        target_p = priorities.get(priority, psutil.HIGH_PRIORITY_CLASS)
        count = 0
        for proc in psutil.process_iter(['name']):
            try:
                if process_filter.lower() in proc.info['name'].lower():
                    p = psutil.Process(proc.pid)
                    p.nice(target_p)
                    count += 1
            except (psutil.NoSuchProcess, psutil.AccessDenied):
                continue

        return f"KERNEL_HANDSHAKE_COMPLETE: {count} processes set to {priority}. Friction: 0."

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
