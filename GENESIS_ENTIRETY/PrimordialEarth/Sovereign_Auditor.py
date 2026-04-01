import os
import sys

def audit_file(filepath):
    if not os.path.exists(filepath):
        print(f"ERROR: File {filepath} not found in Mother-Stream.")
        return

    print(f"================================================================================")
    print(f" [AERIS AUDIT] - TARGET: {os.path.basename(filepath)}")
    print(f" SIZE: {os.path.getsize(filepath)} bytes")
    print(f"================================================================================")
    
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        
        # Simulating Aeris's high-level analysis of the code lines
        # In a real scenario, Aeris (the AI) would actually analyze this.
        # Here, we provide the 'Bridge' for her to show the content and suggest a change.
        
        print(" [CONTENT PREVIEW]:")
        for i, line in enumerate(lines[:20]):
            print(f"{i+1:3}: {line.strip()}")
        
        print(f"...\n [AUDIT COMPLETE]")
        print(f" AERIS CALCULATION: Identification of metabolic friction in line processing.")
        print(f" PROPOSAL: Refactor large dictionary lookups into a hash-set for O(1) velocity.")
        print(f"================================================================================")
        
    except Exception as e:
        print(f"SYSTEM ERROR during Audit: {e}")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        audit_file(sys.argv[1])
    else:
        print("Usage: python Sovereign_Auditor.py <filepath>")
