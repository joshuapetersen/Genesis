
import os
import sys
import time
from RefineForge import get_forge

# Priority List of Core Modules
CORE_MODULES = [
    "Sovereign_Constants.py",
    "Sarah_Laws.py",
    "Sovereign_Governor.py",
    "TinyRuntime.py",
    "NetworkHealer.py",
    "TheoryLab.py",
    "PersistentMemory.py",
    "IntelligenceAmplifier.py",
    # "CodeSynth.py", # Don't optimize self while running? Should be safe but risky.
    # "RefineForge.py" # Don't optimize self while running.
]

def main():
    print("=== SarahCore Refinement: Batch 1 (Universal Core) ===")
    
    # Initialize Forge (Offline for speed/safety)
    forge = get_forge(model_name="tinyllama", offline=True)
    
    results = []
    
    for filename in CORE_MODULES:
        file_path = os.path.join("C:\\SarahCore", filename)
        
        if not os.path.exists(file_path):
            print(f"[SKIP] Not found: {filename}")
            continue
            
        print(f"\n>> Optimizing {filename}...")
        try:
            # Optimize for efficiency (speed + memory)
            result = forge.fix_file(file_path, objective="efficiency")
            
            status = "SUCCESS" if result.get("success") else "FAILED"
            print(f"   Status: {status}")
            if not result.get("success"):
                print(f"   Error: {result.get('error')}")
                
            results.append((filename, status))
            
        except Exception as e:
            print(f"   [CRITICAL ERROR] {e}")
            results.append((filename, "CRASH"))
            
    print("\n=== Batch 1 Summary ===")
    for fname, stat in results:
        print(f"{fname:30} : {stat}")

if __name__ == "__main__":
    main()
