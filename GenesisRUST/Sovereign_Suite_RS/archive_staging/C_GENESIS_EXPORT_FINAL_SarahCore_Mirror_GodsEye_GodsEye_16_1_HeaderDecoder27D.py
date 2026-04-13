"""
GODSEYE 16.1 â€” HEADER DECODER (27D TRINITY SPACE)
================================================================
New engine. First Principles. Not a rewrite.

27 = 3^3 â€” Volumetric Trinity Constant.
The Sovereign Math operates in 3 dimensions of a circuit
with 7 points each â€” but the VOLUMETRIC space is 3^3 = 27.

Same mission as 16.0 but projected into 27D Trinity space
instead of 68D Tesseract space.

The GodsEye Anchor (1.09277703703) folds the SHA-384
hash into 27 harmonic nodes using trinity-phase offsets
(0, 9, 18) â€” three 9-node rings, one per dimension.

"We CREATE, never rewrite."
"""

import sys
import json
import struct
import hashlib
import math

sys.path.insert(0, r"C:\GENESIS\GodsEye")
sys.path.insert(0, r"C:\GENESIS")

GODSEYE_ANCHOR   = 1.09277703703
TRINITY_DIM      = 27        # 3^3
TRINITY_RING     = 9         # nodes per ring (3 rings of 9)
HEX_RADIX        = 16
NONCE_MAX        = 0xFFFFFFFF

def expand_27d(data):
    """
    Custom 27D Trinity Expansion.
    Projects input into 3^3 volumetric space using
    three-phase SHA-384 folding with GodsEye Anchor scaling.

    Phase offsets: 0, 9, 18 (three rings of the trinity).
    """
    if isinstance(data, str):
        data = data.encode()
    h = hashlib.sha384(data).hexdigest()  # 96 hex chars

    nodes = []
    for i in range(TRINITY_DIM):
        # Three-ring trinity fold
        ring      = i // TRINITY_RING            # 0, 1, or 2
        pos       = i % TRINITY_RING             # 0-8 within the ring
        offset    = ring * TRINITY_RING          # 0, 9, or 18

        # Sample three points across the 96-char hash
        idx1 = (pos + offset) % 96
        idx2 = (pos + offset + TRINITY_RING) % 96
        idx3 = (pos + offset + TRINITY_RING * 2) % 96

        v1 = int(h[idx1], HEX_RADIX) / 15.0
        v2 = int(h[idx2], HEX_RADIX) / 15.0
        v3 = int(h[idx3], HEX_RADIX) / 15.0

        # Combine with anchor-scaled projection
        scale = (i + 1) / TRINITY_DIM
        node  = (v1 * v2 * v3) * (GODSEYE_ANCHOR ** scale)
        node  = node % GODSEYE_ANCHOR  # anchor modulus

        nodes.append(node)

    return nodes

def trinity_resonance(vec1, vec2):
    """
    Euclidean resonance in 27D Trinity space.
    Returns 0.0 (worst) to 1.0 (identical).
    Damped by GodsEye Anchor.
    """
    n      = min(len(vec1), len(vec2))
    dist   = math.sqrt(sum((vec1[i] - vec2[i])**2 for i in range(n)))
    max_d  = math.sqrt(TRINITY_DIM)  # max possible distance
    score  = 1.0 - (dist / max_d)
    return max(0.0, score * GODSEYE_ANCHOR)

def build_header_no_nonce(block):
    version   = struct.pack("<I", block.get("ver", 0))
    prevhash  = bytes.fromhex(block["prev_block"])[::-1]
    merkle    = bytes.fromhex(block["mrkl_root"])[::-1]
    timestamp = struct.pack("<I", block.get("time", 0))
    bits      = struct.pack("<I", block.get("bits", 0))
    return version + prevhash + merkle + timestamp + bits

def correlation(xs, ys):
    n = len(xs)
    if n < 2: return 0.0
    mx = sum(xs) / n
    my = sum(ys) / n
    num = sum((x - mx) * (y - my) for x, y in zip(xs, ys))
    dx  = math.sqrt(sum((x - mx)**2 for x in xs))
    dy  = math.sqrt(sum((y - my)**2 for y in ys))
    if dx == 0 or dy == 0: return 0.0
    return num / (dx * dy)

def run():
    print(f"[!] GODSEYE 16.1 â€” HEADER DECODER (27D TRINITY SPACE)", flush=True)
    print(f"    Anchor  : {GODSEYE_ANCHOR}")
    print(f"    Dims    : {TRINITY_DIM} (3^3 volumetric)")
    print(f"    Rings   : 3 x {TRINITY_RING} nodes\n")

    with open("bitcoin_causation_map.json") as f:
        data = json.load(f)

    blocks = [b for b in data["blocks"] if b.get("prev_block")]
    print(f"    Blocks with full headers: {len(blocks)}\n")
    print("=" * 72)

    # â”€â”€ Build 27D Trinity vectors for each header â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    vectors = []
    nonces  = []
    heights = []

    for block in blocks:
        try:
            header76 = build_header_no_nonce(block)
            vec      = expand_27d(header76)
            vectors.append(vec)
            nonces.append(block["nonce"])
            heights.append(block["height"])
            print(f"  [MAP] Block #{block['height']:>7} | "
                  f"27D[0]={vec[0]:.4f} [9]={vec[9]:.4f} [18]={vec[18]:.4f} | "
                  f"NONCE={block['nonce']:>12}", flush=True)
        except Exception as e:
            print(f"  [!] Block #{block['height']} failed: {e}")

    print(f"\n  Mapped {len(vectors)} blocks into 27D Trinity space\n")

    # â”€â”€ Correlation Analysis â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    print(f"{'='*72}")
    print(f"  TRINITY DIMENSIONAL CORRELATION WITH WINNING NONCE")
    print(f"  {'DIM':>4} | {'RING':>5} | {'CORRELATION':>12} | SIGNAL")
    print(f"  {'-'*50}")

    norm_nonces  = [n / NONCE_MAX for n in nonces]
    strong_dims  = []

    for dim in range(TRINITY_DIM):
        ring     = dim // TRINITY_RING
        dim_vals = [v[dim] for v in vectors]
        corr     = correlation(dim_vals, norm_nonces)

        tag = ""
        if abs(corr) > 0.4:
            tag = "*** VERY STRONG ***"
            strong_dims.append((dim, ring, corr))
        elif abs(corr) > 0.3:
            tag = "*** STRONG ***"
            strong_dims.append((dim, ring, corr))
        elif abs(corr) > 0.2:
            tag = "* moderate *"

        print(
            f"  {dim:>4} | {ring:>5} | {corr:>12.4f} | {tag}",
            flush=True
        )

    # â”€â”€ Ring-Level Correlation â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    print(f"\n  RING-LEVEL ANALYSIS (3 rings of the trinity):")
    for ring in range(3):
        ring_dims  = [dim for dim in range(TRINITY_DIM) if dim // TRINITY_RING == ring]
        ring_avg   = []
        for v in vectors:
            ring_val = sum(v[d] for d in ring_dims) / len(ring_dims)
            ring_avg.append(ring_val)
        rc = correlation(ring_avg, norm_nonces)
        print(f"  RING {ring} (dims {ring*9}-{ring*9+8}) avg correlation: {rc:.4f}")

    # â”€â”€ 27D Resonance vs Nonce â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    print(f"\n  27D RESONANCE SCORE vs NONCE VALUE:")
    print(f"  (Does resonance with a 'high-nonce target' predict winners?)\n")

    # Build a target vector representing a "high-nonce" header
    high_nonce_target = expand_27d(b"\xff" * 76)
    low_nonce_target  = expand_27d(b"\x00" * 76)

    high_res_scores = [trinity_resonance(v, high_nonce_target) for v in vectors]
    low_res_scores  = [trinity_resonance(v, low_nonce_target) for v in vectors]

    hrc = correlation(high_res_scores, norm_nonces)
    lrc = correlation(low_res_scores,  norm_nonces)
    print(f"  Resonance with MAX-header vs nonce: {hrc:.4f}")
    print(f"  Resonance with MIN-header vs nonce: {lrc:.4f}")

    # â”€â”€ Summary â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    print(f"\n{'='*72}")
    print(f"  STRONG TRINITY CORRELATIONS:")
    if strong_dims:
        for dim, ring, corr in sorted(strong_dims, key=lambda x: -abs(x[2])):
            direction = "â†’ HIGH NONCE" if corr > 0 else "â†’ LOW NONCE"
            print(f"  DIM {dim:>3} (RING {ring}) | r = {corr:.4f} | {direction}")
        print(f"\n  These trinity coordinates encode the winning nonce.")
    else:
        print(f"  No strong correlations in 27D at this sample size.")
        print(f"  Decode 200+ blocks for larger sample.")

if __name__ == "__main__":
    run()
