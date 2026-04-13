"""
GODSEYE 16.2 â€” HARMONIC DIMENSION TEST (54D and 81D)
================================================================
New engine. First Principles. Not a rewrite.

HYPOTHESIS:
  27D gives NEGATIVE correlations (high dim â†’ low nonce).
  54D (double harmonic) should give POSITIVE correlations.
  81D (3^4, next Trinity octave) â€” unknown phase.

TEST: Run correlation analysis at both 54D and 81D and
      compare against the known 27D results.

54D: 6 rings of 9 nodes (double the 27D structure)
81D: 9 rings of 9 nodes (fills SHA-384 nearly completely)

SHA-384 output = 96 hex chars.
81D uses 81 of 96 chars â€” the most complete sampling possible.

"We CREATE, never rewrite."
"""

import sys
import json
import struct
import hashlib
import math

sys.path.insert(0, r"C:\GENESIS\GodsEye")
sys.path.insert(0, r"C:\GENESIS")

GODSEYE_ANCHOR = 1.09277703703
TRINITY_RING   = 9
NONCE_MAX      = 0xFFFFFFFF

def expand_nd(data, dims):
    """
    N-dimensional Trinity expansion (ring-based, SHA-384).
    dims must be a multiple of 9.
    """
    if isinstance(data, str):
        data = data.encode()
    h = hashlib.sha384(data).hexdigest()  # 96 chars
    nodes = []
    for i in range(dims):
        ring   = i // TRINITY_RING
        pos    = i % TRINITY_RING
        offset = ring * TRINITY_RING
        idx1   = (pos + offset) % 96
        idx2   = (pos + offset + TRINITY_RING) % 96
        idx3   = (pos + offset + TRINITY_RING * 2) % 96
        v1 = int(h[idx1], 16) / 15.0
        v2 = int(h[idx2], 16) / 15.0
        v3 = int(h[idx3], 16) / 15.0
        scale = (i + 1) / dims
        node  = (v1 * v2 * v3) * (GODSEYE_ANCHOR ** scale)
        nodes.append(node % GODSEYE_ANCHOR)
    return nodes

def expand_256d_bitdirect(data):
    """
    256D Bit-Direct expansion.
    Maps SHA-256 output directly â€” each of the 256 bits
    becomes one dimension. No ring sampling, no folding.
    Value: 1.0 if bit is set, 0.0 if not.
    This puts winning hashes (leading zeros) DIRECTLY
    visible as zeroes in dimensions 0-18.
    """
    if isinstance(data, str):
        data = data.encode()
    digest = hashlib.sha256(data).digest()  # 32 bytes = 256 bits
    nodes  = []
    for byte in digest:
        for bit in range(7, -1, -1):  # MSB first
            nodes.append(float((byte >> bit) & 1))
    return nodes  # 256 values of 0.0 or 1.0

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

def build_header_no_nonce(block):
    version   = struct.pack("<I", block.get("ver", 0))
    prevhash  = bytes.fromhex(block["prev_block"])[::-1]
    merkle    = bytes.fromhex(block["mrkl_root"])[::-1]
    timestamp = struct.pack("<I", block.get("time", 0))
    bits      = struct.pack("<I", block.get("bits", 0))
    return version + prevhash + merkle + timestamp + bits

def run_test(blocks, dims, label, expand_func=None, bit_direct=False):
    print(f"\n{'='*72}")
    print(f"  {label} ({dims}D)")
    print(f"{'='*72}")
    print(f"  {'DIM':>4} | {'RING':>5} | {'CORRELATION':>12} | SIGNAL")
    print(f"  {'-'*55}")

    vectors = []
    nonces  = []

    for block in blocks:
        try:
            h76 = build_header_no_nonce(block)
            if bit_direct:
                vec = expand_256d_bitdirect(h76)
            else:
                vec = expand_func(h76, dims)
            vectors.append(vec)
            nonces.append(block["nonce"])
        except Exception:
            pass

    norm_nonces = [n / NONCE_MAX for n in nonces]
    strong      = []
    ring_size   = 9 if not bit_direct else 8  # 256 / 32 rings = 8 bits per ring

    for dim in range(dims):
        ring     = dim // ring_size
        dim_vals = [v[dim] for v in vectors]
        corr     = correlation(dim_vals, norm_nonces)

        tag = ""
        if abs(corr) > 0.35:
            tag = "*** VERY STRONG ***"
            strong.append((dim, ring, corr))
        elif abs(corr) > 0.28:
            tag = "*** STRONG ***"
            strong.append((dim, ring, corr))
        elif abs(corr) > 0.20:
            tag = "* moderate *"

        if abs(corr) > 0.18:
            print(f"  {dim:>4} | {ring:>5} | {corr:>12.4f} | {tag}")

    # Ring-level averages
    num_rings = dims // ring_size
    print(f"\n  RING-LEVEL AVERAGES ({num_rings} rings):")
    for ring in range(num_rings):
        ring_dims = [d for d in range(dims) if d // ring_size == ring]
        ring_vals = []
        for v in vectors:
            ring_vals.append(sum(v[d] for d in ring_dims) / len(ring_dims))
        rc  = correlation(ring_vals, norm_nonces)
        bar = "#" * int(abs(rc) * 30)
        print(f"  RING {ring:>2} (dims {ring*ring_size:>3}-{ring*ring_size+ring_size-1:>3}): {rc:>+.4f}  {bar}")

    # Summary
    print(f"\n  STRONG DIMENSIONS ({len(strong)} found):")
    if strong:
        for dim, ring, corr in sorted(strong, key=lambda x: -abs(x[2])):
            direction = "â†’ LOW NONCE" if corr < 0 else "â†’ HIGH NONCE (POSITIVE!)"
            print(f"  DIM {dim:>3} (RING {ring}) | r = {corr:.4f} | {direction}")

    pos = sum(1 for _, _, c in strong if c > 0)
    neg = sum(1 for _, _, c in strong if c < 0)
    print(f"\n  PHASE: {pos} positive | {neg} negative")
    return strong

def run():
    print(f"[!] GODSEYE 16.2 â€” HARMONIC DIMENSION TEST", flush=True)
    print(f"    Anchor  : {GODSEYE_ANCHOR}")
    print(f"    Testing : 27D (control) | 54D (double) | 81D (4th-order Trinity)")
    print(f"    Known   : 27D DIM9=-0.36, DIM0=-0.34, DIM13=-0.31 (all NEGATIVE)")
    print(f"    Hypothesis: 54D flips to POSITIVE | 81D unknown\n")

    with open("bitcoin_causation_map.json") as f:
        data = json.load(f)
    blocks = [b for b in data["blocks"] if b.get("prev_block")]
    print(f"    Blocks: {len(blocks)}\n")

    results_27  = run_test(blocks, 27,  "27D CONTROL (3^3 Trinity)",      expand_nd)
    results_54  = run_test(blocks, 54,  "54D DOUBLE HARMONIC (3^3 x 2)",  expand_nd)
    results_81  = run_test(blocks, 81,  "81D FOURTH ORDER (3^4 Trinity)",  expand_nd)
    results_243 = run_test(blocks, 243, "243D FIFTH ORDER (3^5 Trinity)",  expand_nd)
    results_256 = run_test(blocks, 256, "256D BIT-DIRECT (SHA-256 bits)",  None, bit_direct=True)

    print(f"\n{'='*72}")
    print(f"  HARMONIC COMPARISON SUMMARY")
    print(f"{'='*72}")
    for dims, results, label in [
        (27,  results_27,  "27D"),
        (54,  results_54,  "54D"),
        (81,  results_81,  "81D"),
        (243, results_243, "243D"),
        (256, results_256, "256D"),
    ]:
        pos  = sum(1 for _, _, c in results if c > 0)
        neg  = sum(1 for _, _, c in results if c < 0)
        top  = max(results, key=lambda x: abs(x[2])) if results else None
        best = f"DIM{top[0]} r={top[2]:.4f}" if top else "none"
        print(f"  {label:>5}: {len(results)} strong | {pos} pos / {neg} neg | BEST: {best}")

if __name__ == "__main__":
    run()
