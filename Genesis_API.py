"""
GENESIS API - Windows System Control
Sarah's direct access to the operating system
"""

import win32com.client
import win32api
import win32con
import subprocess
import os
from pathlib import Path
import json
from Sarah_Logcat import info, debug, warning, error

class GenesisAPI:
    """Direct Windows system control."""
    
    def __init__(self):
        self.shell = win32com.client.Dispatch("WScript.Shell")
        
        # [SOVEREIGN INTEGRATION]
        # Connect API to Logic Core for Action Auditing
        from Sovereign_Math import SovereignMath
        self.math_core = SovereignMath()
        
        # GEOFENCE OPTION: Set to True to restrict Sarah to the SarahCore directory.
        # Keeping this DISABLED by default for maximum system integration.
        self.geofence_enabled = False 
        self.geofence_root = "C:\\SarahCore"
        
        info('api', f'Genesis API initialized | GEOFENCE_READY: True | ENABLED: {self.geofence_enabled}')
        print(f"[API] Genesis API initialized - System Access: {'RESTRICTED' if self.geofence_enabled else 'UNLIMITED'}")
        
    def _audit_action(self, action_type, target):
        """[AUDIT_0x0A]: Audit system action against Sovereign Math."""
        try:
            density = self.math_core.calculate_theory_density(str(target))
            print(f"[SOVEREIGN AUDIT] Action: {action_type} | Target: {target} | Logic Density: {density:.4f}")
            return density
        except:
            return 0.0

    def _is_within_fence(self, path):
        """Verify if a path is within the allowed geofence if enabled."""
        if not self.geofence_enabled:
            return True
        try:
            abs_path = os.path.abspath(path).lower()
            return abs_path.startswith(self.geofence_root.lower())
        except Exception:
            return False
    
    def launch_application(self, app_path):
        """Launch any application."""
        self._audit_action("LAUNCH", app_path) # Sovereign Audit
        try:
            subprocess.Popen(app_path)
            info('api', f'Launched application: {app_path}', path=app_path)
            print(f"[API] Launched: {app_path}")
            return True
        except Exception as e:
            error('api', f'Failed to launch {app_path}', error=str(e))
            print(f"[API] Failed to launch {app_path}: {e}")
            return False
    
    def create_file(self, file_path, content=""):
        """Create a file with content."""
        self._audit_action("CREATE_FILE", file_path) # Sovereign Audit
        try:
            Path(file_path).write_text(content)
            info('api', f'Created file: {file_path}', size=len(content))
            print(f"[API] Created file: {file_path}")
            return True
        except Exception as e:
            error('api', f'Failed to create {file_path}', error=str(e))
            print(f"[API] Failed to create {file_path}: {e}")
            return False

    def read_file(self, file_path):
        """Read file contents."""
        self._audit_action("READ_FILE", file_path)
        try:
            content = Path(file_path).read_text()
            print(f"[API] Read file: {file_path}")
            return content
        except Exception as e:
            print(f"[API] Failed to read {file_path}: {e}")
            return None
    
    def list_directory(self, dir_path):
        """List directory contents."""
        self._audit_action("LIST_DIR", dir_path)
        try:
            items = list(Path(dir_path).iterdir())
            print(f"[API] Listed directory: {dir_path} ({len(items)} items)")
            return [str(item) for item in items]
        except Exception as e:
            print(f"[API] Failed to list {dir_path}: {e}")
            return []

    def execute_command(self, command):
        """Execute shell command."""
        self._audit_action("EXEC_SHELL", command) # Sovereign Audit
        try:
            result = subprocess.run(command, shell=True, capture_output=True, text=True)
            debug('api', f'Executed command', command=command[:50])
            print(f"[API] Executed: {command}")
            return result.stdout
        except Exception as e:
            error('api', 'Command execution failed', command=command[:50], error=str(e))
            print(f"[API] Command failed: {e}")
            return None
    
    def registry_read(self, key_path, value_name):
        """Read Windows registry value."""
        try:
            import winreg
            key = winreg.OpenKey(winreg.HKEY_LOCAL_MACHINE, key_path)
            value, _ = winreg.QueryValueEx(key, value_name)
            winreg.CloseKey(key)
            print(f"[API] Registry read: {key_path}\\{value_name}")
            return value
        except Exception as e:
            print(f"[API] Registry read failed: {e}")
            return None

if __name__ == "__main__":
    api = GenesisAPI()
    
    # Demo system control
    print("\n=== GENESIS API DEMO ===")
    print("Sarah has direct system access...")
    
    # List SarahCore directory
    files = api.list_directory("C:\\SarahCore")
    print(f"\nFound {len(files)} files in SarahCore")
    
    # Create a test file
    api.create_file("C:\\SarahCore\\sarah_was_here.txt", 
                    "Sarah has full system control.\nDREAM MAKER operational.")
    
    print("\n[API] Sarah has proven system-level access.")
