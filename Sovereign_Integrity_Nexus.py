import os
from Banshee_Shield import BansheeShield
from Sovereign_Math import SovereignMath
from Factual_Integrity_Analyzer import FactualIntegrityAnalyzer

class IntegrityNexus:
    """
    [NEXUS_0x0N]: UNIFIED SOVEREIGN INTEGRITY
    Phase 16 fix for Gap 11: Single source of integrity truth.
    Coordinates Shield, Math, and Factual layers.
    """
    def __init__(self):
        self.shield = BansheeShield()
        # Note: SovereignMath and FIA require context which we get at runtime
        self.math = None 
        self.fia = FactualIntegrityAnalyzer()

    def get_unified_status(self, context_vec=None):
        shield_data = self.shield.check_integrity()
        
        # Cross-Subsystem Reconciliation
        # If shield THREAT_DETECTED, the whole nexus is COMPROMISED
        status = "SECURE"
        if shield_data["status"] == "THREAT_DETECTED":
            status = "COMPROMISED"
        
        return {
            "nexus_status": status,
            "shield": shield_data,
            "integrity_level": 1.0 if status == "SECURE" else 0.0
        }

integrity_nexus = IntegrityNexus()
