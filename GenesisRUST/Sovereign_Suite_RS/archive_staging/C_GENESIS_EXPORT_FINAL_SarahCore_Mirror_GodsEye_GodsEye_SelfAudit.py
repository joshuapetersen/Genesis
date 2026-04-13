"""
GODSEYE SELF-AUDIT [RECURSIVE DIAGNOSTIC]
================================================================
"Turn it on the GodsEye itself."
Run the Deep Dissector against the GodsEye directory.
"""

import os
import sys

# Configuration
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
GODSEYE_DIR = SCRIPT_DIR
DISSECTOR_PATH = os.path.join(SCRIPT_DIR, 'GodsEye_5_0_DeepDissector.py')
OUTPUT_MD = os.path.join(SCRIPT_DIR, 'godseye_self_audit.md')

def ignite_self_audit():
    print(f"\n[!] INITIATING RECURSIVE SOVEREIGN AUDIT ...")
    print(f"[!] TARGET: {GODSEYE_DIR}")
    print("="*70)

    # Inject temporary target for self-audit
    # We modify the dissector's SCAN_ROOT for this run only
    with open(DISSECTOR_PATH, 'r', encoding='utf-8') as f:
        code = f.read()
    
    audit_code = code.replace('SCAN_ROOT = r"C:\GenesisOS_Core\rust"', f'SCAN_ROOT = r"{GODSEYE_DIR}"')
    audit_code = audit_code.replace('OUTPUT_MD = os.path.join(SCRIPT_DIR, \'godseye_v5_deep_audit.md\')', f'OUTPUT_MD = r"{OUTPUT_MD}"')
    
    TEMP_DISSECTOR = os.path.join(SCRIPT_DIR, 'tmp_self_dissector.py')
    with open(TEMP_DISSECTOR, 'w', encoding='utf-8') as f:
        f.write(audit_code)
    
    print("[+] PHASE 1: EXECUTING RECURSIVE DISSECTION ...")
    import subprocess
    subprocess.run([sys.executable, TEMP_DISSECTOR], check=False)
    
    print("\n" + "="*70)
    print(f"[SUCCESS] RECURSIVE AUDIT COMPLETE.")
    print(f"Self-Audit Report seated: {OUTPUT_MD}")
    print("="*70)

    # Cleanup
    if os.path.exists(TEMP_DISSECTOR):
        os.remove(TEMP_DISSECTOR)

if __name__ == "__main__":
    ignite_self_audit()
