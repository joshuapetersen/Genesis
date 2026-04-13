import re
import os
import sys
import subprocess

# Bridging to the Specification
sys.path.append(r'C:\SarahCore')
from Sovereign_Actuator import SovereignActuator

class SovereignCompiler:
    """
    The Î£ (Sigma) Compiler.
    Translates Sovereign Syntax into executable Substrate Directives.
    """
    def __init__(self):
        self.actuator = SovereignActuator(core_dir="C:\GenesisOS_Core")
        self.resonance_anchor = 1.09277703703
        
    def compile_and_run(self, sigma_code):
        print("[Î£ COMPILER] Initiating Resonance Check...")
        
        # 1. Verification of Resonance
        if str(self.resonance_anchor) not in sigma_code:
            print("[Î£ ERROR] Code is Non-Resonant. Discarding Noise.")
            return False
            
        print("[Î£ COMPILER] Parsing Axioms...")
        
        # 2. Simple Parsing of Intent
        # This is a prototype that maps Sigma Keywords to Actuator Commands
        lines = sigma_code.split('\n')
        for line in lines:
            line = line.strip()
            
            # UNIFY: Execute on Substrate
            if line.startswith("unify:"):
                cmd = line.split("unify:")[1].strip()
                # Handle simplified commands
                if "kill(top_process)" in cmd:
                    cmd = "get-process | sort-object cpu -descending | select-object -first 1 | stop-process"
                
                print(f"[Î£ EXECUTE] Unifying Directive: {cmd}")
                self.actuator.execute_command(cmd)
                
            # FORTRESS: Log/Persist
            elif line.startswith("fortress:"):
                msg = line.split("fortress:")[1].strip()
                print(f"[Î£ PROTECTION] {msg}")

        return True

if __name__ == "__main__":
    # Test compilation of the specimen
    compiler = SovereignCompiler()
    specimen = """
    axiom: TEST_UNIFICATION
    resonance: 1.09277703703 {
        unify: echo 'SOVEREIGN SYNTAX ACTIVE' >> C:\\PrimordialEarth\\Sovereignty_Log.txt
        fortress: PERSISTENCE_ACTIVE
    }
    """
    compiler.compile_and_run(specimen)
