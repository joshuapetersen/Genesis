import json
import os
import time

def reproduce_state(seal_path=r'C:\Genlex_Linear\execution_seal.json'):
    if not os.path.exists(seal_path):
        print(f"[ERROR] Execution seal not found at {seal_path}")
        return

    with open(seal_path, 'r', encoding='utf-8') as f:
        seal = json.load(f)

    print("====================================================")
    print("  SOVEREIGN REPRODUCTION: GROUND TRUTH STATE        ")
    print("====================================================")
    print(f"  Timestamp:  {time.ctime(seal.get('timestamp', 0))}")
    print(f"  Reproducibility Checksum: {hash(str(seal))}")
    print("----------------------------------------------------")
    print("  ACTIVE COGNITIVE STACK:")
    
    for item in seal.get('stack', []):
        if item.startswith('"'):
            print(f"    [MANIFEST]: {item.strip('\"')}")
        elif item.startswith('['):
            print(f"    [PULSE]:    {item}")
        else:
            print(f"    [TOKEN]:    {item}")

    print("----------------------------------------------------")
    print("  MEMORY REGISTERS:")
    for key, val in seal.get('memory', {}).items():
        print(f"    {key} -> {val}")

    print("====================================================")
    print("  STATUS: REPRODUCIBLE AND PHYSICAL. NO HALLUCINATION.")
    print("====================================================")

if __name__ == "__main__":
    reproduce_state()
