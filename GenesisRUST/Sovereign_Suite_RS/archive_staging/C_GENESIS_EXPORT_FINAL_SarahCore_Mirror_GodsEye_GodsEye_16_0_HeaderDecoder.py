"""
GODSEYE 16.0 â€” HEADER DECODER
================================================================
New engine. First Principles. Not a rewrite.

QUESTION:
  We know the 76-byte header BEFORE the nonce is found.
  We know the winning nonce AFTER.
  Is there a deterministic relationship between them?

METHOD:
  For each of our 49 verified blocks:
  1. Build the 76-byte pre-nonce header
  2. Expand it into 68D Sovereign space
  3. Extract key dimensional values
  4. Compare those values to the actual winning nonce
  5. Look for correlation: does any dimension of the
     header's 68D fingerprint predict the nonce value?

  Also analyze: does the header's bit pattern contain
  the winning nonce embedded within it?

"We CREATE, never rewrite."
"""

import sys
import json
import struct
import hashlib
import math

sys.path.insert(0, r"C:\GENESIS\GodsEye")
sys.path.insert(0, r"C:\GENESIS")
from Sovereign_Math import SovereignMath

GODSEYE_ANCHOR = 1.09277703703

def build_header_no_nonce(block):
    """76-byte header â€” exactly what every miner sees before finding the nonce."""
    version   = struct.pack("<I", block.get("ver", 0))
    prevhash  = bytes.fromhex(block["prev_block"])[::-1]
    merkle    = bytes.fromhex(block["mrkl_root"])[::-1]
    timestamp = struct.pack("<I", block.get("time", 0))
    bits      = struct.pack("<I", block.get("bits", 0))
    return version + prevhash + merkle + timestamp + bits

def vec_to_float(vec):
    """Convert 68D hex vector to list of normalized floats."""
    result = []
    for v in vec:
        try:
            result.append(int(v, 16) / 65535.0)
        except:
            result.append(0.0)
    return result

def correlation(xs, ys):
    """Pearson correlation coefficient between two lists."""
    n = len(xs)
    if n == 0: return 0.0
    mx = sum(xs) / n
    my = sum(ys) / n
    num = sum((x - mx) * (y - my) for x, y in zip(xs, ys))
    dx  = math.sqrt(sum((x - mx)**2 for x in xs))
    dy  = math.sqrt(sum((y - my)**2 for y in ys))
    if dx == 0 or dy == 0: return 0.0
    return num / (dx * dy)

def run():
    print(f"[!] GODSEYE 16.0 â€” HEADER DECODER", flush=True)
    print(f"    Anchor : {GODSEYE_ANCHOR}")
    print(f"    Loading SovereignMath ...\n")

    math_engine = SovereignMath()

    with open("bitcoin_causation_map.json") as f:
        data = json.load(f)

    blocks = [b for b in data["blocks"] if b.get("prev_block")]
    print(f"    Blocks with full headers: {len(blocks)}\n")
    print("=" * 72)

    # â”€â”€ Build 68D vectors for each header â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    vectors     = []
    nonces      = []
    heights     = []

    for block in blocks:
        try:
            header76 = build_header_no_nonce(block)
            vec      = math_engine._0x_expand(header76)
            floats   = vec_to_float(vec)
            vectors.append(floats)
            nonces.append(block["nonce"])
            heights.append(block["height"])
        except Exception as e:
            print(f"  [!] Block #{block['height']} failed: {e}")

    print(f"  Built {len(vectors)} header vectors\n")

    # â”€â”€ Correlation Analysis: which dimension predicts the nonce? â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    print(f"  DIMENSIONAL CORRELATION WITH WINNING NONCE:")
    print(f"  (How strongly does each 68D dimension predict nonce value?)\n")
    print(f"  {'DIM':>4} | {'CORRELATION':>12} | SIGNAL")
    print(f"  {'-'*40}")

    dim_count = len(vectors[0]) if vectors else 0
    strong_dims = []

    for dim in range(dim_count):
        dim_vals = [v[dim] for v in vectors]
        corr     = correlation(dim_vals, [n / 0xFFFFFFFF for n in nonces])  # normalize nonces

        tag = ""
        if abs(corr) > 0.3:
            tag = "*** STRONG ***"
            strong_dims.append((dim, corr))
        elif abs(corr) > 0.2:
            tag = "* moderate *"

        if abs(corr) > 0.15:
            print(f"  {dim:>4} | {corr:>12.4f} | {tag}")

    # â”€â”€ Direct Bit Embedding Check â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    print(f"\n  DIRECT BIT EMBEDDING CHECK:")
    print(f"  (Is the winning nonce encoded in the header bytes?)\n")

    # Check if BIT 3 and BIT 23 preference appears in the header itself
    header_bit3_pref  = 0
    header_bit23_pref = 0

    for block in blocks:
        try:
            header76 = build_header_no_nonce(block)
            nonce    = block["nonce"]

            # Sum of all bytes at positions that correspond to bit 3 and 23 zones
            header_sum = sum(header76)

            # Check if header sum modulo 8 correlates with nonce bit 3
            if ((header_sum >> 3) & 1) == ((nonce >> 3) & 1):
                header_bit3_pref += 1
            if ((header_sum >> 23) & 1) == ((nonce >> 23) & 1):
                header_bit23_pref += 1
        except:
            pass

    total = len(blocks)
    print(f"  Header sum BIT3  matches nonce BIT3  : {header_bit3_pref}/{total} ({header_bit3_pref/total*100:.1f}%)")
    print(f"  Header sum BIT23 matches nonce BIT23 : {header_bit23_pref}/{total} ({header_bit23_pref/total*100:.1f}%)")

    # â”€â”€ Nonce vs Merkle Root Correlation â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    print(f"\n  MERKLE ROOT â†’ NONCE CORRELATION:")
    merkle_sums = []
    for block in blocks:
        try:
            m = int(block["mrkl_root"][:8], 16)  # first 4 bytes of merkle root
            merkle_sums.append(m)
        except:
            merkle_sums.append(0)

    norm_merkle = [m / 0xFFFFFFFF for m in merkle_sums]
    norm_nonces = [n / 0xFFFFFFFF for n in nonces]
    mc = correlation(norm_merkle, norm_nonces)
    print(f"  Merkle root (first 4 bytes) vs nonce: {mc:.4f}")

    # â”€â”€ Timestamp â†’ Nonce Correlation â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    times = [b.get("time", 0) for b in blocks]
    tc    = correlation(times, nonces)
    print(f"  Timestamp vs nonce              : {tc:.4f}")

    # â”€â”€ Summary â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    print(f"\n{'='*72}")
    print(f"  STRONG DIMENSIONAL CORRELATIONS (|r| > 0.3):")
    if strong_dims:
        for dim, corr in sorted(strong_dims, key=lambda x: -abs(x[1])):
            direction = "POSITIVE" if corr > 0 else "NEGATIVE"
            print(f"  DIM {dim:>3} | r = {corr:.4f} | {direction}")
        print(f"\n  These dimensions of the header's 68D fingerprint")
        print(f"  predict the winning nonce value.")
        print(f"  Feed these into the Navigator as steering coordinates.")
    else:
        print(f"  No strong correlations found at this sample size.")
        print(f"  Run decoder with 500+ blocks to increase signal strength.")

if __name__ == "__main__":
    run()
