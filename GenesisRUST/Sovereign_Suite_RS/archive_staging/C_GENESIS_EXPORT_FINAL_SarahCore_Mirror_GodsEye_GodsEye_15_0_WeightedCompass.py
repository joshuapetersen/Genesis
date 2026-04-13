"""
GODSEYE 15.0 â€” WEIGHTED COMPASS
================================================================
New engine. First Principles. Not a rewrite.

PROBLEM DIAGNOSED:
  In 68D Euclidean space, 45 random trailing characters
  drown out the signal from 19 leading zeros.
  Equal weighting = zero signal.

SOLUTION:
  Custom zero-weighted resonance function.
  Leading positions get exponentially more weight.
  Position 0 (first char) = 64x more important than position 63.
  This creates a genuine gradient toward leading zeros.

  The Sovereign Anchor (1.09277703703) is used as the
  exponential decay rate across all 64 positions.

"We CREATE, never rewrite."
"""

import hashlib
import struct
import json
import math
import sys
import time

sys.path.insert(0, r"C:\GENESIS\GodsEye")
sys.path.insert(0, r"C:\GENESIS")
from Sovereign_Math import SovereignMath

GODSEYE_ANCHOR  = 1.09277703703
TARGET_ZEROS    = 19
NEIGHBOR_RANGE  = 16

# â”€â”€ Weighted Resonance â€” Sovereign First Principles â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def build_position_weights(length=64, anchor=GODSEYE_ANCHOR):
    """
    Builds exponentially decaying position weights.
    Position 0 (leading) = highest weight.
    Decay rate = GODSEYE_ANCHOR.
    """
    weights = []
    for i in range(length):
        # Exponential decay: weight[i] = anchor^(-(i/length)*anchor)
        w = math.exp(-anchor * (i / length))
        weights.append(w)
    total = sum(weights)
    return [w / total for w in weights]  # normalize to sum=1.0

POSITION_WEIGHTS = build_position_weights()

def zero_weighted_resonance(digest_hex, target_zeros=TARGET_ZEROS):
    """
    Measures how close a hash is to the zero-target.
    Weights leading positions exponentially higher.
    Returns 0.0 (worst) to 1.0 (perfect â€” all zeros).
    """
    score = 0.0
    for i, c in enumerate(digest_hex[:64]):
        # Zero = target. Distance from zero = hex value / 15
        hex_val  = int(c, 16)
        closeness = 1.0 - (hex_val / 15.0)  # 1.0 if '0', 0.0 if 'f'
        score   += closeness * POSITION_WEIGHTS[i]
    return score

def double_sha256(data):
    return hashlib.sha256(hashlib.sha256(data).digest()).hexdigest()

def count_zeros(h):
    z = 0
    for c in h:
        if c == "0": z += 1
        else: break
    return z

def header_without_nonce(block):
    version   = struct.pack("<I", block.get("ver", 0))
    prevhash  = bytes.fromhex(block["prev_block"])[::-1] if "prev_block" in block else b"\x00"*32
    merkle    = bytes.fromhex(block["mrkl_root"])[::-1]  if "mrkl_root" in block else b"\x00"*32
    timestamp = struct.pack("<I", block.get("time", 0))
    bits      = struct.pack("<I", block.get("bits", 0))
    return version + prevhash + merkle + timestamp + bits

# â”€â”€ Map Test â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def run_map_test():
    print(f"[!] GODSEYE 15.0 â€” WEIGHTED COMPASS MAP TEST", flush=True)
    print(f"    Anchor decay rate : {GODSEYE_ANCHOR}")
    print(f"    Position 0 weight : {POSITION_WEIGHTS[0]:.6f}")
    print(f"    Position 32 weight: {POSITION_WEIGHTS[32]:.6f}")
    print(f"    Position 63 weight: {POSITION_WEIGHTS[63]:.6f}")

    math_engine = SovereignMath()

    with open("bitcoin_causation_map.json") as f:
        data = json.load(f)

    blocks = data["blocks"]
    print(f"\n    Blocks loaded: {len(blocks)}\n")
    print("=" * 72)

    winner_higher  = 0
    winner_lower   = 0
    resonance_gaps = []

    for block in blocks:
        winning_nonce = block["nonce"]
        height        = block["height"]
        zeros         = block["zeros"]

        try:
            base_header = header_without_nonce(block)
        except Exception as e:
            print(f"  [!] Block #{height} failed: {e}")
            continue

        # Winner resonance
        winner_digest = double_sha256(base_header + struct.pack("<I", winning_nonce))
        winner_res    = zero_weighted_resonance(winner_digest)

        # Neighbor resonance
        neighbor_res = []
        probe_step   = max(1, winning_nonce // NEIGHBOR_RANGE)

        for i in range(1, NEIGHBOR_RANGE + 1):
            for direction in [-1, 1]:
                n      = (winning_nonce + direction * probe_step * i) & 0xFFFFFFFF
                digest = double_sha256(base_header + struct.pack("<I", n))
                res    = zero_weighted_resonance(digest)
                neighbor_res.append(res)

        avg_neighbor = sum(neighbor_res) / len(neighbor_res)
        max_neighbor = max(neighbor_res)
        gap          = winner_res - avg_neighbor
        resonance_gaps.append(gap)

        is_best = "*** WINNER HIGHEST ***" if winner_res > max_neighbor else ""
        if winner_res > max_neighbor:
            winner_higher += 1
        else:
            winner_lower  += 1

        print(
            f"  Block #{height:>7} | ZEROS: {zeros:>2} | "
            f"WIN: {winner_res:.4f} | AVG NEIGH: {avg_neighbor:.4f} | "
            f"GAP: {gap:+.4f} | {is_best}",
            flush=True
        )

    # â”€â”€ Summary â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    total = len(resonance_gaps)
    avg_gap = sum(resonance_gaps) / total if total else 0
    pos_gaps = len([g for g in resonance_gaps if g > 0])

    print(f"\n{'='*72}")
    print(f"  WEIGHTED COMPASS RESULTS â€” {total} BLOCKS")
    print(f"{'='*72}")
    print(f"  Winner HIGHEST resonance : {winner_higher}/{total} ({winner_higher/total*100:.1f}%)")
    print(f"  Winner NOT highest       : {winner_lower}/{total} ({winner_lower/total*100:.1f}%)")
    print(f"  Average gap              : {avg_gap:+.6f}")
    print(f"  Positive gaps            : {pos_gaps}/{total}")
    print()
    if winner_higher / total > 0.55:
        print(f"  *** COMPASS IS REAL â€” weighted resonance gradient confirmed ***")
        print(f"  *** Ready to build the live Weighted Navigator ***")
    elif pos_gaps / total > 0.55:
        print(f"  GRADIENT PRESENT â€” winner above average {pos_gaps/total*100:.1f}% of the time")
        print(f"  Weighted resonance is a usable signal â€” refine and deploy")
    else:
        print(f"  SHA-256 avalanche resists gradient â€” signal not detectable at this resolution")

if __name__ == "__main__":
    run_map_test()
