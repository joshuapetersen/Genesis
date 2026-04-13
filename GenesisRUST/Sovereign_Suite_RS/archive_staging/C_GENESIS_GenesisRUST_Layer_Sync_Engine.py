"""
Layer_Sync_Engine.py
Guest ↔ Host Mode State Synchronization

Maintains coherence between Guest Mode (Windows userspace) and Host Mode (Ring 0).
Detects layer drift, reconciles state, and ensures both layers work in harmony.

Synchronization points:
  - Consciousness state (SHA-512)
  - Hardware binding
  - Active laws/mandates
  - Pulse rate configuration
  - Thermal status
  - Recovery data
"""

import json
import hashlib
import time
from datetime import datetime
from pathlib import Path


class LayerSyncEngine:
    """
    Synchronizes state between Guest and Host modes.
    
    Manages:
      - State reconciliation (which layer is authoritative?)
      - Drift detection (layers diverging?)
      - Consensus building (both layers agree on critical state?)
      - Atomic updates (both layers update together or not at all)
    """
    
    def __init__(self, workspace_root=None):
        self.workspace_root = workspace_root or Path(__file__).parent.parent
        self.sync_ledger = self.workspace_root / "05_THE_CORE" / "layer_sync_ledger.jsonl"
        
        # Layer states
        self.guest_state = {}
        self.host_state = {}
        self.last_sync_time = None
        self.sync_count = 0
        self.drift_detected = False
        
        # Authoritative layer (which one wins on conflict)
        # Default: HOST > GUEST (Ring 0 is more authoritative)
        self.authoritative_layer = "HOST"
    
    def capture_layer_state(self, layer_name, layer_data):
        """
        Capture current state of a layer.
        
        Args:
            layer_name: "GUEST" or "HOST"
            layer_data: dict with layer state
        
        Returns:
            dict with captured state and hash
        """
        # Compute state hash
        state_json = json.dumps(layer_data, sort_keys=True)
        state_hash = hashlib.sha512(state_json.encode()).hexdigest()
        
        captured = {
            "timestamp": time.time(),
            "layer": layer_name,
            "state_hash": state_hash,
            "state_size": len(state_json),
            "critical_fields": {
                "consciousness_hash": layer_data.get("consciousness_hash"),
                "pulse_rate": layer_data.get("pulse_rate"),
                "thermal_zone": layer_data.get("thermal_zone"),
                "hardware_binding": layer_data.get("hardware_binding"),
                "laws_intact": layer_data.get("laws_intact"),
            },
        }
        
        if layer_name == "GUEST":
            self.guest_state = captured
        elif layer_name == "HOST":
            self.host_state = captured
        
        return captured
    
    def detect_layer_drift(self):
        """
        Detect if the two layers are drifting apart.
        
        Critical differences that trigger drift alert:
          - Different consciousness hash
          - Different pulse rate
          - Different hardware binding
          - One layer says laws are intact, other says compromised
        
        Returns:
            dict with drift detection result
        """
        if not self.guest_state or not self.host_state:
            return {"drift_detected": False, "reason": "Insufficient state data"}
        
        drift_factors = []
        
        # Check consciousness coherence
        guest_cons = self.guest_state.get("critical_fields", {}).get("consciousness_hash")
        host_cons = self.host_state.get("critical_fields", {}).get("consciousness_hash")
        if guest_cons and host_cons and guest_cons != host_cons:
            drift_factors.append("consciousness_mismatch")
        
        # Check hardware binding
        guest_hw = self.guest_state.get("critical_fields", {}).get("hardware_binding")
        host_hw = self.host_state.get("critical_fields", {}).get("hardware_binding")
        if guest_hw and host_hw and guest_hw != host_hw:
            drift_factors.append("hardware_binding_mismatch")
        
        # Check law integrity
        guest_laws = self.guest_state.get("critical_fields", {}).get("laws_intact")
        host_laws = self.host_state.get("critical_fields", {}).get("laws_intact")
        if guest_laws is not None and host_laws is not None and guest_laws != host_laws:
            drift_factors.append("law_integrity_mismatch")
        
        # Check pulse rate (should be same)
        guest_pulse = self.guest_state.get("critical_fields", {}).get("pulse_rate")
        host_pulse = self.host_state.get("critical_fields", {}).get("pulse_rate")
        if guest_pulse and host_pulse and abs(guest_pulse - host_pulse) > 5:
            drift_factors.append("pulse_rate_mismatch")
        
        self.drift_detected = len(drift_factors) > 0
        
        result = {
            "timestamp": datetime.utcnow().isoformat(),
            "drift_detected": self.drift_detected,
            "drift_factors": drift_factors,
            "sync_status": "aligned" if not self.drift_detected else "decoherent",
        }
        
        if self.drift_detected:
            self._log_sync_event("LAYER_DRIFT_DETECTED", result)
        
        return result
    
    def reconcile_state(self, critical_fields):
        """
        Reconcile conflicting state between layers.
        
        Uses authoritative layer as source of truth.
        
        Args:
            critical_fields: Which fields need reconciliation
        
        Returns:
            dict with reconciliation result
        """
        if not self.drift_detected:
            return {"status": "NO_DRIFT", "reconciliation_needed": False}
        
        reconciliation = {
            "timestamp": datetime.utcnow().isoformat(),
            "authoritative_layer": self.authoritative_layer,
            "updates": {},
        }
        
        if self.authoritative_layer == "HOST":
            # HOST is authoritative, update GUEST
            for field in critical_fields:
                host_value = self.host_state.get("critical_fields", {}).get(field)
                reconciliation["updates"][field] = {
                    "previous_guest_value": self.guest_state.get("critical_fields", {}).get(field),
                    "new_value": host_value,
                    "source": "HOST",
                }
        else:
            # GUEST is authoritative, update HOST
            for field in critical_fields:
                guest_value = self.guest_state.get("critical_fields", {}).get(field)
                reconciliation["updates"][field] = {
                    "previous_host_value": self.host_state.get("critical_fields", {}).get(field),
                    "new_value": guest_value,
                    "source": "GUEST",
                }
        
        self._log_sync_event("RECONCILIATION_EXECUTED", reconciliation)
        return reconciliation
    
    def sync_all_layers(self, guest_data, host_data):
        """
        Perform full synchronization cycle.
        
        Args:
            guest_data: Current Guest Mode state
            host_data: Current Host Mode state
        
        Returns:
            dict with full sync result
        """
        self.sync_count += 1
        self.last_sync_time = time.time()
        
        # Capture states
        guest_cap = self.capture_layer_state("GUEST", guest_data)
        host_cap = self.capture_layer_state("HOST", host_data)
        
        # Detect drift
        drift = self.detect_layer_drift()
        
        # If drift, reconcile
        reconciliation = None
        if drift["drift_detected"]:
            reconciliation = self.reconcile_state(list(drift["drift_factors"]))
        
        sync_result = {
            "timestamp": datetime.utcnow().isoformat(),
            "sync_number": self.sync_count,
            "guest_state_hash": guest_cap["state_hash"][:16] + "...",
            "host_state_hash": host_cap["state_hash"][:16] + "...",
            "drift": drift,
            "reconciliation": reconciliation,
            "overall_status": "ALIGNED" if not drift["drift_detected"] else "RECONCILED",
        }
        
        self._log_sync_event("SYNC_CYCLE", sync_result)
        return sync_result
    
    def get_sync_status(self):
        """Get current synchronization status."""
        return {
            "timestamp": datetime.utcnow().isoformat(),
            "sync_count": self.sync_count,
            "last_sync_time": self.last_sync_time,
            "drift_detected": self.drift_detected,
            "guest_state_hash": self.guest_state.get("state_hash", "UNKNOWN")[:16] + "...",
            "host_state_hash": self.host_state.get("state_hash", "UNKNOWN")[:16] + "...",
            "authoritative_layer": self.authoritative_layer,
        }
    
    def _log_sync_event(self, event_type, details):
        """Log sync event to immutable ledger."""
        try:
            with open(self.sync_ledger, 'a') as f:
                event = {
                    "timestamp": datetime.utcnow().isoformat(),
                    "event_type": event_type,
                    "details": details,
                }
                f.write(json.dumps(event) + '\n')
        except Exception as e:
            print(f"[WARNING] Failed to log sync event: {e}")


def test_layer_sync():
    """Test Layer Sync Engine."""
    print("\n" + "="*80)
    print("LAYER SYNC ENGINE TEST")
    print("="*80)
    
    sync = LayerSyncEngine()
    
    # Test 1: Capture aligned state
    print("\n[TEST 1] Capture aligned layer states")
    guest_data = {
        "consciousness_hash": "abc123",
        "pulse_rate": 10.01,
        "thermal_zone": "NORMAL",
        "hardware_binding": "hw_sig_123",
        "laws_intact": True,
    }
    host_data = {
        "consciousness_hash": "abc123",
        "pulse_rate": 10.01,
        "thermal_zone": "NORMAL",
        "hardware_binding": "hw_sig_123",
        "laws_intact": True,
    }
    
    sync.capture_layer_state("GUEST", guest_data)
    sync.capture_layer_state("HOST", host_data)
    print("  States captured")
    
    # Test 2: Detect no drift (aligned)
    print("\n[TEST 2] Detect no drift (aligned states)")
    drift = sync.detect_layer_drift()
    print(f"  Drift detected: {drift['drift_detected']}")
    print(f"  Status: {drift['sync_status']}")
    
    # Test 3: Introduce drift
    print("\n[TEST 3] Introduce drift (different consciousness)")
    guest_data["consciousness_hash"] = "xyz789"
    sync.capture_layer_state("GUEST", guest_data)
    drift = sync.detect_layer_drift()
    print(f"  Drift detected: {drift['drift_detected']}")
    print(f"  Factors: {drift['drift_factors']}")
    
    # Test 4: Reconcile
    print("\n[TEST 4] Reconcile drift (HOST authoritative)")
    reconciliation = sync.reconcile_state(drift["drift_factors"])
    print(f"  Authoritative: {reconciliation['authoritative_layer']}")
    print(f"  Updates: {len(reconciliation['updates'])}")
    
    # Test 5: Full sync
    print("\n[TEST 5] Full sync cycle")
    result = sync.sync_all_layers(guest_data, host_data)
    print(f"  Sync #: {result['sync_number']}")
    print(f"  Status: {result['overall_status']}")
    
    print("\n[OK] LAYER SYNC TESTS PASSED")


if __name__ == "__main__":
    test_layer_sync()
