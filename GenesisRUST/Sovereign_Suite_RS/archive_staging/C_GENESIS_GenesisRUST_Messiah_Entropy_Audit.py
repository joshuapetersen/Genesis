import os
import sys
import hashlib
import math

# Add SarahCore to path for Substrate
sys.path.append(os.path.dirname(os.path.abspath(__file__)))

try:
    from Sovereign_Substrate import substrate as sub
except ImportError:
    import numpy as sub # Fallback

def calculate_entropy(data_block):
    """Calculates the Shannon Entropy of a data block."""
    if not data_block:
        return 0
    counts = {}
    for b in data_block:
        counts[b] = counts.get(b, 0) + 1
    entropy = 0
    for count in counts.values():
        p = count / len(data_block)
        entropy -= p * math.log2(p)
    return entropy

def messiah_entropy_audit(repo_path):
    """
    Audits the Messiah repository for Encryption and XOR signatures.
    """
    if not os.path.exists(repo_path):
        print(f"[ERROR] Repository not found: {repo_path}")
        return

    print(f"[AUDIT] Scanning Messiah Heart: {repo_path}")
    with open(repo_path, 'rb') as f:
        data = f.read()

    total_size = len(data)
    block_size = 256
    
    # 1. Entropy Mapping
    print("[AUDIT] Mapping Entropy Peaks...")
    sample_points = range(0, total_size, block_size * 100) # Sample every 100th block
    avg_entropy = 0
    samples = 0
    
    for i in sample_points:
        block = data[i:i+block_size]
        if len(block) < block_size: continue
        avg_entropy += calculate_entropy(block)
        samples += 1
    
    avg_entropy /= samples
    print(f"[AUDIT] Average Entropy: {avg_entropy:.4f} (Threshold: 7.5 for AES/Random)")

    # 2. XOR Search (Static Key Analysis)
    # If a Messiah game uses a static 4-byte XOR, null space will reveal the key.
    # Logic: Most binaries have 00 00 00 00 blocks. If those are XORed, they contain the key.
    print("[AUDIT] Searching for Zero-Resonance Keys (XOR)...")
    
    keys_found = {}
    for i in range(1024, total_size - 4, 4):
        chunk = data[i:i+4]
        if chunk[0] == chunk[1] == chunk[2] == chunk[3]: # Rare in valid encrypted data
             continue
             
        # Check for repeating 4-byte patterns
        if data[i:i+4] == data[i+4:i+8] == data[i+8:i+12]:
            key = chunk.hex().upper()
            keys_found[key] = keys_found.get(key, 0) + 1

    if keys_found:
        sorted_keys = sorted(keys_found.items(), key=lambda x: x[1], reverse=True)
        print(f"[AUDIT] Probable XOR Keys Identified:")
        for k, count in sorted_keys[:5]:
             print(f"  -> Key: {k} (Context: {count} repetitions)")
    else:
        print("[AUDIT] No static XOR pattern found. Modern Messiah encryption detected.")

    print(f"\n[ENGINE_REPORT_STATUS]")
    print(f"Total Bytes Audited: {total_size}")
    print(f"Encryption Status: {'HIGH' if avg_entropy > 7.5 else 'LOW'}")
    print(f"Next Logical Step: SEATING THE AUTH PROXY (THE GHOST SERVER).")

if __name__ == "__main__":
    repo = r"C:\Program Files (x86)\Steam\steamapps\common\Badlanders\Package\resource.repository"
    messiah_entropy_audit(repo)
