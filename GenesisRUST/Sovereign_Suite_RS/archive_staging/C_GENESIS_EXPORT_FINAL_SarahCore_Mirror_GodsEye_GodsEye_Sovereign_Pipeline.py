"""
GODSEYE SOVEREIGN PIPELINE [GENERATIONAL MASTER]
================================================================
Fuses generations 1 through 5 into a single Multistage Reactor.
Pass 1: Topology [Gen 1]
Pass 2: Kinetic Manifest [Gen 3-4]
Pass 3: Deep Dissection [Gen 2-5]

"We CREATE, never rewrite."
"""

import os
import sys
import time

# Script paths for the generational hand-off
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
GEN_PROCESSORS = [
    os.path.join(SCRIPT_DIR, 'GodsEye_4_0_Kinetic.py'),  # Gen 3-4 [Speed/Index]
    os.path.join(SCRIPT_DIR, 'GodsEye_6_0_Immutable.py')  # Gen 6 [Hardened Audit]
]

def ignite_pipeline():
    start_time = time.time()
    print(f"\n[!] IGNITING GODSEYE SOVEREIGN PIPELINE ...")
    print("="*70)

    for i, proc in enumerate(GEN_PROCESSORS):
        print(f"\n[+] GENERATION {i+3}: EXECUTING {os.path.basename(proc)}")
        print("-" * 50)
        
        # Execute phase [HARDENED]
        import subprocess
        result = subprocess.run([sys.executable, proc], check=False).returncode
        
        if result != 0:
            print(f"[ERROR] Pipeline breach at Generation {i+3}. Check substrate.")
            sys.exit(1)
            
        print(f"[+] PHASE {i+3} COMPLETE.")

    elapsed = time.time() - start_time
    print("\n" + "="*70)
    print(f"[SUCCESS] SOVEREIGN PIPELINE RESOLVED IN {elapsed:.2f}s")
    print(f"Final Manifest Seated: C:\GENESIS\GodsEye\godseye_v5_deep_audit.md")
    print("="*70)

if __name__ == "__main__":
    ignite_pipeline()
