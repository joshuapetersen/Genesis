"""
GODSEYE 16.3 â€” TARGETED DIMENSION PROBE (256 blocks)
================================================================
New engine. First Principles. Not a rewrite.

Specifically measures the KNOWN strong dimensions from
the 48-block analysis against the full 256-block set.

Original signals (48 blocks):
  DIM  9 (RING 1): r = -0.3586
  DIM  0 (RING 0): r = -0.3408
  DIM 13 (RING 1): r = -0.3095
  DIM 17 (RING 1): r = -0.2832
  DIM 57 (RING 6): r = -0.3953
  DIM 78 (RING 8): r = +0.2987

Questions:
  1. What are these EXACT dimensions at 256 blocks?
  2. Do they still point the same direction?
  3. What is the true effect size with more data?
  4. Also scan 256D bit-direct at lower threshold (r>0.08)
     to find any bit positions with real signal.

"We CREATE, never rewrite."
"""

import sys
import json
import struct
import hashlib
from Sovereign_Statistics import sovereign_significance, sovereign_signal, sovereign_direction, print_sovereign_report

sys.path.insert(0, r"C:\GENESIS\GodsEye")
sys.path.insert(0, r"C:\GENESIS")

GODSEYE_ANCHOR = 1.09277703703
TRINITY_DIM    = 81
TRINITY_RING   = 9
NONCE_MAX      = 0xFFFFFFFF

def expand_81d(data):
    if isinstance(data, str):
        data = data.encode()
    h = hashlib.sha384(data).hexdigest()
    nodes = []
    for i in range(TRINITY_DIM):
        ring   = i // TRINITY_RING
        pos    = i % TRINITY_RING
        offset = ring * TRINITY_RING
        idx1   = (pos + offset) % 96
        idx2   = (pos + offset + TRINITY_RING) % 96
        idx3   = (pos + offset + TRINITY_RING * 2) % 96
        v1 = int(h[idx1], 16) / 15.0
        v2 = int(h[idx2], 16) / 15.0
        v3 = int(h[idx3], 16) / 15.0
        scale = (i + 1) / TRINITY_DIM
        node  = (v1 * v2 * v3) * (GODSEYE_ANCHOR ** scale)
        nodes.append(node % GODSEYE_ANCHOR)
    return nodes

def expand_256d_bits(data):
    if isinstance(data, str):
        data = data.encode()
    digest = hashlib.sha256(data).digest()
    nodes  = []
    for byte in digest:
        for bit in range(7, -1, -1):
            nodes.append(float((byte >> bit) & 1))
    return nodes

def correlation(xs, ys):
    n = len(xs)
    if n < 2: return 0.0
    mx = sum(xs) / n
    my = sum(ys) / n
    num = sum((x - mx) * (y - my) for x, y in zip(xs, ys))
    dx  = (sum((x - mx)**2 for x in xs)) ** 0.5
    dy  = (sum((y - my)**2 for y in ys)) ** 0.5
    if dx == 0 or dy == 0: return 0.0
    return num / (dx * dy)

def build_header_no_nonce(block):
    version   = struct.pack("<I", block.get("ver", 0))
    prevhash  = bytes.fromhex(block["prev_block"])[::-1]
    merkle    = bytes.fromhex(block["mrkl_root"])[::-1]
    timestamp = struct.pack("<I", block.get("time", 0))
    bits      = struct.pack("<I", block.get("bits", 0))
    return version + prevhash + merkle + timestamp + bits

def run():
    print("[!] GODSEYE 16.3 â€” TARGETED DIMENSION PROBE", flush=True)

    with open("bitcoin_causation_map.json") as f:
        data = json.load(f)
    blocks = [b for b in data["blocks"] if b.get("prev_block")]
    n = len(blocks)
    print(f"    Blocks: {n}\n")

    # Build vectors
    vecs_81  = []
    vecs_256 = []
    nonces   = []
    for block in blocks:
        try:
            h76 = build_header_no_nonce(block)
            vecs_81.append(expand_81d(h76))
            vecs_256.append(expand_256d_bits(h76))
            nonces.append(block["nonce"])
        except Exception:
            pass

    norm_nonces = [x / NONCE_MAX for x in nonces]
    n = len(nonces)
    print(f"    Vectors built: {n}\n")

    # â”€â”€ 1. Known strong 81D dimensions â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    KNOWN_DIMS = [
        (0,  0, -0.3408, "48-block"),
        (9,  1, -0.3586, "48-block STRONGEST"),
        (13, 1, -0.3095, "48-block"),
        (17, 1, -0.2832, "48-block"),
        (57, 6, -0.3953, "81D STRONGEST"),
        (78, 8, +0.2987, "81D POSITIVE"),
    ]

    print("=" * 72)
    print("  KNOWN DIMENSIONS â€” TRUE CORRELATION WITH 256 BLOCKS")
    print("=" * 72)
    print(f"  {'DIM':>4} | {'RING':>5} | {'OLD r':>8} | {'NEW r':>8} | {'SIGNAL':>8} | {'LABEL':<14} | DIRECTION")
    print(f"  {'-'*75}")

    for dim, ring, old_r, note in KNOWN_DIMS:
        vals  = [v[dim] for v in vecs_81]
        new_r = correlation(vals, norm_nonces)
        sig, label = sovereign_significance(new_r, n)
        op, direc  = sovereign_direction(new_r)
        same  = "âœ“" if (new_r < 0) == (old_r < 0) else "âœ— FLIP"
        print(
            f"  {dim:>4} | {ring:>5} | {old_r:>+8.4f} | {new_r:>+8.4f} | "
            f"{sig:>8.3f} | {label:<14} | {same} {direc}"
        )

    # â”€â”€ 2. Full 81D scan at lower threshold â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    print(f"\n{'='*72}")
    print(f"  81D FULL SCAN (threshold r > 0.10, n={n})")
    print(f"{'='*72}")
    print(f"  {'DIM':>4} | {'RING':>5} | {'r':>8} | {'SIGNAL':>8} | {'LABEL':<13}")
    print(f"  {'-'*50}")

    all_81 = []
    for dim in range(81):
        ring = dim // 9
        vals = [v[dim] for v in vecs_81]
        r    = correlation(vals, norm_nonces)
        sig, label = sovereign_significance(r, n)
        all_81.append((dim, ring, r, sig))
        if sig >= 1.0:
            print(f"  {dim:>4} | {ring:>5} | {r:>+8.4f} | {sig:>8.3f} | {label}")

    # â”€â”€ 3. 256D bit scan at lower threshold â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    print(f"\n{'='*72}")
    print(f"  256D BIT-DIRECT SCAN (threshold r > 0.08, bits of SHA-256(header76))")
    print(f"{'='*72}")
    print(f"  {'BIT':>5} | {'BYTE':>5} | {'r':>8} | {'SIGNAL':>8} | {'LABEL':<13}")
    print(f"  {'-'*50}")

    strong_bits = []
    for bit in range(256):
        byte_pos = bit // 8
        vals     = [v[bit] for v in vecs_256]
        r        = correlation(vals, norm_nonces)
        sig, label = sovereign_significance(r, n)
        if sig >= 1.0:
            strong_bits.append((bit, byte_pos, r, sig, label))
            print(f"  {bit:>5} | {byte_pos:>5} | {r:>+8.4f} | {sig:>8.3f} | {label}")

    # â”€â”€ Summary â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    print(f"\n{'='*72}")
    print(f"  SUMMARY")
    print(f"{'='*72}")
    same_dir  = sum(1 for d, r, old, note in KNOWN_DIMS
                    if (correlation([v[d] for v in vecs_81], norm_nonces) < 0) == (old < 0))
    locked    = [(d, r, s, l) for d, r, s, l in all_81 if s >= 1.0]
    print(f"  Known dims keeping same direction   : {same_dir}/{len(KNOWN_DIMS)}")
    print(f"  81D dims at Sovereign signal â‰¥ 1.0  : {len(locked)}")
    print(f"  256D bits at Sovereign signal â‰¥ 1.0 : {len(strong_bits)}")
    if strong_bits:
        print(f"\n  TOP BITS (highest signal):")
        for bit, byte_pos, r, sig, label in sorted(strong_bits, key=lambda x: -x[3])[:5]:
            op, direc = sovereign_direction(r)
            print(f"    BIT {bit:>3} (BYTE {byte_pos}) | r={r:>+.4f} | signal={sig:.3f} | {op} | {direc}")

if __name__ == "__main__":
    run()
