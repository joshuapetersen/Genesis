"""
SOVEREIGN HUNTER CLI
The fully integrated program combining ATS Topology Search and Intelligence Amplification.
"""
import os
import argparse
import sys
import time
import re
import json

# Ensure python can find our local modules
sys.path.append(os.path.dirname(os.path.abspath(__file__)))

from ats_v4 import collect_files, analyze_structure
import ats_v4
from IntelligenceAmplifier import IntelligenceAmplifier

class HunterCLI:
    def __init__(self, target_dir):
        self.target_dir = os.path.abspath(target_dir)
        self.bounties = []
        self.secret_patterns = {
            "Bitcoin_WIF_Key": r'^[5KL][1-9A-HJ-NP-Za-km-z]{50,51}$',
            "BIP39_Seed_Phrase": r'\b(?:[a-z]{3,8}\s){11}[a-z]{3,8}\b',
            "Hardcoded_API_Key": r'(?i)(?:api_key|secret|token|password)[\s\=\:]+[\'"]([a-zA-Z0-9_\-]{16,})[\'"]'
        }
        
    def run(self):
        print("\n" + "="*70)
        print(" [ SOVEREIGN HUNTER v1.0 - LIVE ]")
        print(" [ TARGETING: " + self.target_dir + " ]")
        print("="*70)
        
        # 1. Initialize Amplifier (DEACTIVATED FOR STANDALONE STABILITY)
        print("\n[*] [STANDALONE] Intelligence Amplifier bypassed. Seating local kernel...")
        self.amplifier = None # IntelligenceAmplifier()
        
        # 2. Map Topology (Pulse 1 - Drive Level)
        print(f"\n[*] [PULSE 1] Indexing Total Substrate Topography ({self.target_dir})...")
        ats_v4.SCAN_ROOT = self.target_dir
        files = collect_files()
        
        if not files:
            print("[!] FATAL: No logic neurons found on substrate. Halting.")
            sys.exit(1)
            
        print(f"[+] Pulse 1 Complete. Locked onto {len(files)} Total Neurons natively.")
        
        # 3. Execute Unbridled Parallel Audit (Pulse 2)
        print(f"\n[*] [PULSE 2] Igniting Unbridled Parallel Audit across {len(files)} neurons...")
        depends_on, depended_by, edges, all_bounties = ats_v4.build_dependency_graph(files)
        self.bounties = all_bounties

        # 4. Generate Output Report (MARKDOWN RESTORATION)
        report_path = os.path.join(self.target_dir, "Sovereign_Bounty_Report.md")
        with open(report_path, "w", encoding="utf-8") as f:
            f.write("# Sovereign GodsEye Bounty Report\n\n")
            f.write("> [!IMPORTANT]\n")
            f.write(f"> Synchronous audit of {self.target_dir} complete.\n\n")
            f.write(f"## Total Critical Hints: {len(self.bounties)}\n\n")
            
            if self.bounties:
                f.write("| Type | File | Hint |\n")
                f.write("| :--- | :--- | :--- |\n")
                for b in self.bounties:
                    f.write(f"| {b['type']} | {b['file']} | {b['target']} |\n")
            else:
                f.write("*No critical hints identified in this substrate.*")
            
        print("\n" + "="*70)
        print(f" [ AUDIT COMPLETE ]")
        print(f" [ TOTAL CRITICAL HINTS: {len(self.bounties)} ]")
        print(f" [ REPORT SEATED AT: {report_path} ]")
        print("="*70 + "\n")


def main():
    parser = argparse.ArgumentParser(description="Sovereign Hunter: Automated Topographic Penetration Tester")
    parser.add_argument("--target", "-t", required=True, help="The target directory to scan (e.g. C:/TargetRepo)")
    
    args = parser.parse_args()
    
    if not os.path.exists(args.target):
        print(f"[!] Error: Target directory {args.target} does not exist.")
        sys.exit(1)
        
    hunter = HunterCLI(args.target)
    hunter.run()

if __name__ == "__main__":
    main()
