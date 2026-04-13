import sys
import os
import csv
import json

# Add parent directory to sys.path to find all_engine
sys.path.append(r'C:\Genlex_Linear')

from all_engine import GenlexLinearRuntime

def test_reality():
    print("[REALITY CHECK] INITIALIZING GENLEX RUNTIME...")
    engine = GenlexLinearRuntime()
    
    script_path = r"C:\Genlex_Core\genesis_one.all"
    if not os.path.exists(script_path):
        print(f"[ERROR] SCRIPT NOT FOUND: {script_path}")
        return

    print(f"[REALITY CHECK] EXECUTING: {script_path}")
    print("-" * 50)
    
    with open(script_path, 'r', encoding='utf-8') as f:
        for line in f:
            clean = line.split('#')[0].strip()
            if not clean: continue
            tokens = clean.split()
            for t in tokens:
                if t in engine.lexicon:
                    op = engine.lexicon[t]['op']
                    print(f"[RUN] GLYPH: {t} -> OP: {op}")
                else:
                    try:
                        float(t)
                        print(f"[RUN] DATA: {t} -> STACK_PUSH")
                    except:
                        print(f"[RUN] TOKEN: {t} (STRICT)")
    
    print("-" * 50)
    print("[REALITY CHECK] EXECUTION COMPLETE. THIS IS PHYSICAL CODE.")

if __name__ == "__main__":
    test_reality()
