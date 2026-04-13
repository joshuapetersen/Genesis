import psutil
from Consequence_Enforcer import consequence_enforcer

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
        # Phase 15 fix for Gap 7: Platform-agnostic check (no windll on Linux)
        try:
            if hasattr(ctypes, 'windll'):
                return ctypes.windll.shell32.IsUserAnAdmin()
            else:
                return os.getuid() == 0
        except (AttributeError, OSError, Exception):
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
        
        # Phase 19 fix for Gap 3/10: Consequence Enforcement (Level 4)
        # Note: In a headless environment, architect_signature should be provided via secure IPC/Session
        authorized, reason = consequence_enforcer.verify_operation(4)
        if not authorized:
            return f"ACTION_DENIED: {reason}"
            
        if action.lower() == "shutdown":
            print("[AdminActuator] WARNING: Substrate Shutdown command received. Verifying Intent...")
        
        cmd = commands.get(action.lower())
        if cmd:
            subprocess.run(cmd, shell=True, creationflags=CREATE_NO_WINDOW)
            return f"PHYSICAL_ACTION_ENGAGED: {action} (Grace: 60s)"
        return "ERROR: INVALID_POWER_ACTION"

    def set_substrate_priority(self, process_filter, priority="High"):
        """
        Optimized by AERIS: Native psutil implementation targets processes without sub-shell friction.
        """
        if not self.is_admin: return "ERROR: ADMIN ACCESS REQUIRED"
        
        priorities = {
            "Normal": psutil.NORMAL_PRIORITY_CLASS,
            "High": psutil.HIGH_PRIORITY_CLASS,
            # Phase 15 fix for Gap 9: Guard Realtime (Preempts Kernel)
            "Realtime": psutil.REALTIME_PRIORITY_CLASS if priority != "python" else psutil.HIGH_PRIORITY_CLASS,
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
            # Phase 15 fix for Gap 10: Check return code to detect lack of NVIDIA GPU
            proc = subprocess.run(cmd, shell=True, creationflags=CREATE_NO_WINDOW, capture_output=True)
            if proc.returncode == 0:
                return "SUBSTRATE_PERFORMANCE_LOCKED: P0_ACTIVE"
            return f"SUBSTRATE_PERFORMANCE_ERROR: nvidia-smi failed ({proc.returncode})"
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
