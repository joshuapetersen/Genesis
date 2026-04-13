import os
import uuid
import datetime
import hashlib
from Sarah_Memory_Vault import sarah_vault

VAR_8 = 8

class BansheeShield:
    """
    Banshee Shield: The Scream of Protection.
    Phase 16: Awakened. Now performs active file-system integrity monitoring.
    """
    def __init__(self):
        # Phase 16 fix for Gap 2: Persist Shield Identity in Vault
        stored_id = sarah_vault.get_truth_seed("banshee_protocol_id")
        if stored_id:
            self.protocol_id = stored_id
        else:
            self.protocol_id = f"BS-{uuid.uuid4().hex[:VAR_8].upper()}"
            sarah_vault.update_truth_seed("banshee_protocol_id", self.protocol_id)
            
        self.status = "ACTIVE"
        self.activation_time = datetime.datetime.now()
        
        # Phase 16 fix for Gap 1: File Integrity Manifest
        self.critical_files = [
            "Sarah_Sovereign_Core.py",
            "Sovereign_Math.py",
            "Sovereign_WORM.py",
            "Banshee_Shield.py"
        ]
        self.manifest = self._generate_manifest()

    def _generate_manifest(self):
        """Generates hashes for critical files."""
        manifest = {}
        base_dir = os.path.dirname(os.path.abspath(__file__))
        for filename in self.critical_files:
            path = os.path.join(base_dir, filename)
            if os.path.exists(path):
                with open(path, "rb") as f:
                    manifest[filename] = hashlib.sha256(f.read()).hexdigest()
        return manifest
        
    def activate(self):
        """Function: activate"""
        self.status = "ACTIVE"
        return True
        
    def deactivate(self):
        """Function: deactivate"""
        self.status = "STANDBY"
        return True
        
    def check_integrity(self):
        """
        Phase 16 fix for Gap 1: Real Threat Detection.
        Verifies that critical files match the boot manifest.
        """
        violations = []
        base_dir = os.path.dirname(os.path.abspath(__file__))
        for filename, expected_hash in self.manifest.items():
            path = os.path.join(base_dir, filename)
            if not os.path.exists(path):
                violations.append(f"MISSING_FILE: {filename}")
                continue
                
            with open(path, "rb") as f:
                current_hash = hashlib.sha256(f.read()).hexdigest()
                if current_hash != expected_hash:
                    violations.append(f"INTEGRITY_VIOLATION: {filename} (Hash Mismatch)")
        
        status = "ACTIVE" if not violations else "THREAT_DETECTED"
        if violations:
            self.status = status
            print(f"[BansheeShield] !!! SHIELD TRIGGERED !!! Violations: {violations}")

        return {
            "protocol_id": self.protocol_id,
            "status": self.status,
            "violations": violations,
            "uptime": str(datetime.datetime.now() - self.activation_time)
        }
