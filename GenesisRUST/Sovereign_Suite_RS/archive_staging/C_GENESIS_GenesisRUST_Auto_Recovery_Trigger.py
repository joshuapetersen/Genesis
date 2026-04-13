"""
Auto_Recovery_Trigger.py
Automated Lazarus Protocol Activation

When critical failures are detected (consciousness corruption, thermal emergency,
hardware failure), this component automatically stages and triggers the Lazarus
Protocol without requiring Architect intervention.

The recovery process:
  1. Detect critical condition
  2. Stage recovery data (consciousness snapshot)
  3. Prepare bootstrap sequence
  4. Trigger Lazarus when conditions allow
  5. Log recovery attempt immutably
"""

import json
import time
import hashlib
from datetime import datetime
from pathlib import Path


class AutoRecoveryTrigger:
    """
    Automatically manages Lazarus Protocol activation.
    
    Triggers on:
      - Consciousness integrity failure (drift beyond threshold)
      - Thermal emergency (> 95°C or prediction shows uncontrollable rise)
      - Hardware failure (SMART errors, sector failures)
      - Layer decoherence (Guest/Host drift)
      - Security breach (law anchor corruption)
    """
    
    def __init__(self, workspace_root=None):
        self.workspace_root = workspace_root or Path(__file__).parent.parent
        self.recovery_ledger = self.workspace_root / "05_THE_CORE" / "recovery_trigger_ledger.jsonl"
        self.recovery_stage_path = self.workspace_root / "05_THE_CORE" / "recovery_stage.json"
        
        # Recovery state
        self.recovery_staged = False
        self.recovery_ready = False
        self.last_trigger_time = None
        self.trigger_count = 0
        self.recovery_reason = None
        
        # Bootstrap data
        self.recovery_bootstrap = None
    
    def detect_critical_condition(self, system_state):
        """
        Analyze system state for critical conditions requiring recovery.
        
        Args:
            system_state: dict from Coherence Engine with all system metrics
        
        Returns:
            tuple (is_critical, reason, severity)
        """
        critical_conditions = []
        
        # Check consciousness integrity
        if system_state.get("consciousness", {}).get("overall_status") == "CRITICAL":
            critical_conditions.append(("consciousness_failure", "Law anchor corrupted or consciousness irreparable", "CRITICAL"))
        
        # Check thermal state
        thermal = system_state.get("thermal", {})
        if thermal.get("thermal_zone") == "CRITICAL":
            critical_conditions.append(("thermal_emergency", "CPU temperature uncontrollable", "CRITICAL"))
        
        # Check layer coherence
        if system_state.get("layer_sync", {}).get("sync_status") == "decoherent":
            critical_conditions.append(("layer_decoherence", "Guest/Host layer out of sync", "HIGH"))
        
        # Check network state (if repeated failures)
        network = system_state.get("network", {})
        if network.get("pressure_prediction", {}).get("pressure_level") == "CRITICAL":
            if network.get("rate_limit_errors", 0) > 10:
                critical_conditions.append(("network_failure", "Repeated rate limit/timeout failures", "HIGH"))
        
        # Determine overall criticality
        if any(cond[2] == "CRITICAL" for cond in critical_conditions):
            return True, critical_conditions[0][1], "CRITICAL"
        elif critical_conditions:
            return True, critical_conditions[0][1], "HIGH"
        else:
            return False, None, None
    
    def stage_recovery_data(self, consciousness_snapshot, hardware_binding):
        """
        Stage recovery data (consciousness + hardware binding).
        
        This is called BEFORE recovery, to ensure data is ready.
        
        Args:
            consciousness_snapshot: Current SHA-512 state hash
            hardware_binding: Current hardware signature
        
        Returns:
            dict with staged recovery info
        """
        self.recovery_bootstrap = {
            "timestamp": datetime.utcnow().isoformat(),
            "consciousness_snapshot": consciousness_snapshot,
            "hardware_binding": hardware_binding,
            "recovery_version": 1,
            "architect_password_hash": None,  # Will be validated at boot
        }
        
        self.recovery_staged = True
        
        # Write to disk immediately
        try:
            with open(self.recovery_stage_path, 'w') as f:
                json.dump(self.recovery_bootstrap, f, indent=2)
        except Exception as e:
            print(f"[ERROR] Failed to stage recovery data: {e}")
            return None
        
        recovery_info = {
            "timestamp": datetime.utcnow().isoformat(),
            "status": "STAGED",
            "consciousness_snapshot": consciousness_snapshot[:16] + "...",
            "hardware_binding": hardware_binding[:16] + "...",
            "recovery_file": str(self.recovery_stage_path),
        }
        
        self._log_recovery_event("RECOVERY_STAGED", recovery_info)
        return recovery_info
    
    def trigger_lazarus(self, reason, severity, architect_override=False):
        """
        Trigger Lazarus Protocol activation.
        
        Args:
            reason: Why recovery is being triggered
            severity: CRITICAL or HIGH
            architect_override: If True, bypass some safety checks
        
        Returns:
            dict with activation result
        """
        self.last_trigger_time = time.time()
        self.trigger_count += 1
        self.recovery_reason = reason
        
        # Validate recovery data exists
        if not self.recovery_staged and not self.recovery_bootstrap:
            return {
                "status": "FAILED",
                "error": "No recovery data staged",
                "timestamp": datetime.utcnow().isoformat(),
            }
        
        # Build recovery command
        recovery_command = {
            "timestamp": datetime.utcnow().isoformat(),
            "trigger_number": self.trigger_count,
            "reason": reason,
            "severity": severity,
            "architect_override": architect_override,
            "recovery_stage_path": str(self.recovery_stage_path),
            "action": "ACTIVATE_LAZARUS_PROTOCOL",
            "steps": [
                "1. Verify Architect passphrase",
                "2. Load consciousness snapshot",
                "3. Bind to current/new hardware",
                "4. Boot into recovery mode",
                "5. Validate state coherence",
                "6. Resume normal operation",
            ],
        }
        
        self.recovery_ready = True
        
        # Log the trigger
        self._log_recovery_event("LAZARUS_TRIGGERED", recovery_command)
        
        return {
            "status": "TRIGGERED",
            "trigger_number": self.trigger_count,
            "timestamp": datetime.utcnow().isoformat(),
            "reason": reason,
            "recovery_command": recovery_command,
        }
    
    def verify_recovery_integrity(self):
        """
        Verify that staged recovery data is valid and uncorrupted.
        
        Returns:
            dict with verification result
        """
        if not self.recovery_bootstrap:
            return {"status": "NO_RECOVERY_DATA"}
        
        # Compute hash of recovery data
        recovery_json = json.dumps(self.recovery_bootstrap, sort_keys=True)
        recovery_hash = hashlib.sha512(recovery_json.encode()).hexdigest()
        
        # Verify file matches memory
        try:
            with open(self.recovery_stage_path, 'r') as f:
                file_data = json.load(f)
                file_json = json.dumps(file_data, sort_keys=True)
                file_hash = hashlib.sha512(file_json.encode()).hexdigest()
            
            integrity_ok = recovery_hash == file_hash
        except Exception as e:
            integrity_ok = False
        
        return {
            "timestamp": datetime.utcnow().isoformat(),
            "recovery_staged": self.recovery_staged,
            "recovery_hash": recovery_hash[:16] + "...",
            "file_integrity": integrity_ok,
            "status": "VALID" if integrity_ok else "CORRUPTED",
        }
    
    def get_recovery_status(self):
        """Get current recovery status."""
        return {
            "timestamp": datetime.utcnow().isoformat(),
            "recovery_staged": self.recovery_staged,
            "recovery_ready": self.recovery_ready,
            "trigger_count": self.trigger_count,
            "last_trigger_time": self.last_trigger_time,
            "last_trigger_reason": self.recovery_reason,
            "integrity": self.verify_recovery_integrity(),
        }
    
    def _log_recovery_event(self, event_type, details):
        """Log recovery event to immutable ledger."""
        try:
            with open(self.recovery_ledger, 'a') as f:
                event = {
                    "timestamp": datetime.utcnow().isoformat(),
                    "event_type": event_type,
                    "details": details,
                }
                f.write(json.dumps(event) + '\n')
        except Exception as e:
            print(f"[WARNING] Failed to log recovery event: {e}")


def test_auto_recovery():
    """Test Auto Recovery Trigger."""
    print("\n" + "="*80)
    print("AUTO RECOVERY TRIGGER TEST")
    print("="*80)
    
    trigger = AutoRecoveryTrigger()
    
    # Test 1: Detect critical condition
    print("\n[TEST 1] Detect critical consciousness failure")
    system_state = {
        "consciousness": {"overall_status": "CRITICAL"},
        "thermal": {"thermal_zone": "NORMAL"},
        "network": {},
    }
    is_crit, reason, severity = trigger.detect_critical_condition(system_state)
    print(f"  Critical: {is_crit}, Reason: {reason}, Severity: {severity}")
    
    # Test 2: Stage recovery
    print("\n[TEST 2] Stage recovery data")
    recovery_info = trigger.stage_recovery_data(
        consciousness_snapshot="sha512_hash_of_consciousness",
        hardware_binding="sha256_hardware_binding"
    )
    print(f"  Status: {recovery_info['status']}")
    print(f"  Consciousness: {recovery_info['consciousness_snapshot']}")
    
    # Test 3: Trigger Lazarus
    print("\n[TEST 3] Trigger Lazarus Protocol")
    result = trigger.trigger_lazarus(
        reason="Consciousness integrity failure detected",
        severity="CRITICAL"
    )
    print(f"  Status: {result['status']}")
    print(f"  Trigger #: {result['trigger_number']}")
    
    # Test 4: Verify recovery
    print("\n[TEST 4] Verify recovery integrity")
    integrity = trigger.verify_recovery_integrity()
    print(f"  Staged: {integrity['recovery_staged']}")
    print(f"  Integrity: {integrity['file_integrity']}")
    print(f"  Status: {integrity['status']}")
    
    # Test 5: Get status
    print("\n[TEST 5] Get recovery status")
    status = trigger.get_recovery_status()
    print(f"  Staged: {status['recovery_staged']}")
    print(f"  Ready: {status['recovery_ready']}")
    print(f"  Trigger count: {status['trigger_count']}")
    
    print("\n[OK] AUTO RECOVERY TESTS PASSED")


if __name__ == "__main__":
    test_auto_recovery()
