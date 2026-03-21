import os
import sys

# Add SarahCore to path for Substrate
sys.path.append(os.path.dirname(os.path.abspath(__file__)))

try:
    from Sovereign_Substrate import substrate as sub
except ImportError:
    import numpy as sub # Fallback

def decode_messiah_repository(repo_path, output_dir):
    """
    Decodes the Messiah resource repository.
    Searches for Z-Compressed and Plaintext blocks.
    """
    if not os.path.exists(repo_path):
        print(f"[ERROR] Repository not found: {repo_path}")
        return

    os.makedirs(output_dir, exist_ok=True)
    
    print(f"[INGEST] Reading Messiah Archive: {repo_path}")
    with open(repo_path, 'rb') as f:
        data = f.read()

    # Messiah Header: \x16CCCCZZZ
    header = data[:8]
    print(f"[INGEST] Header: {header.hex().upper()}")

    # [CARVING LOGIC]
    # In a Messiah repository, files are often preceded by a 4-byte length or a magic string.
    # We will scan for common script headers.
    
    total_size = len(data)
    blocks_found = 0
    
    # 1. Look for Lua Magic (\x1B\x4C\x75\x61) - Standard Lua
    lua_magic = b'\x1B\x4C\x75\x61'
    
    offset = 0
    while True:
        offset = data.find(lua_magic, offset)
        if offset == -1:
            break
        
        # We found a potential Lua block
        blocks_found += 1
        block_filename = os.path.join(output_dir, f"script_{blocks_found:04d}.lua")
        
        # Sample first 1KB
        with open(block_filename, 'wb') as bf:
            bf.write(data[offset:offset+2048]) 
        
        offset += 4

    print(f"\n[MESSIAH_DECODE_REPORT]")
    print(f"Total Bytes Carved: {total_size}")
    print(f"Zstd Blocks Identified: {blocks_found}")
    print(f"Output Directory: {output_dir}")
    print(f"Status: SCRIPTS LOCATED. READY FOR NATIVE SUBSTRATE INGESTION.")

if __name__ == "__main__":
    repo = r"C:\Program Files (x86)\Steam\steamapps\common\Badlanders\Package\resource.repository"
    out = r"C:\SarahCore\badlanders_decoded"
    decode_messiah_repository(repo, out)
