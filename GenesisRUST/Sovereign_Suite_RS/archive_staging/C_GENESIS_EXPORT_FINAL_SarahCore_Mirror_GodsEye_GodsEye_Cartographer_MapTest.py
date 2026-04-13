"""
GODSEYE 14.1 â€” CARTOGRAPHER OFFLINE MAP TEST
================================================================
Runs the Sovereign Cartographer against the 50 downloaded blocks.
For each block we know:
  - The exact 80-byte header
  - The WINNING nonce (the one that solved the block)

Test: Does the Cartographer assign HIGHER resonance to the
      winning nonce than to its neighbors?

If YES: The resonance compass is real.
If NO:  The compass needs recalibration.

"We CREATE, never rewrite."
"""

import sys
import json
import struct
import hashlib
import time

sys.path.insert(0, r"C:\GENESIS\GodsEye")
sys.path.insert(0, r"C:\GENESIS")
from Sovereign_Math import SovereignMath

GODSEYE_ANCHOR = 1.09277703703
TARGET_ZEROS   = 19
ZERO_TARGET_STR = "0" * TARGET_ZEROS + "a" * (64 - TARGET_ZEROS)

# â”€â”€ Rebuild 80-byte header from block record â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def build_header(block):
    version   = struct.pack("<I", block["ver"] if "ver" in block else 0)
    prevhash  = bytes.fromhex(block["prev_block"])[::-1] if "prev_block" in block else b"\x00"*32
    merkle    = bytes.fromhex(block["mrkl_root"])[::-1]  if "mrkl_root" in block else b"\x00"*32
    timestamp = struct.pack("<I", block["time"])          if "time" in block else b"\x00"*4
    bits      = struct.pack("<I", block["bits"])          if "bits" in block else b"\x00"*4
    nonce     = struct.pack("<I", block["nonce"])         if "nonce" in block else b"\x00"*4
    return version + prevhash + merkle + timestamp + bits + nonce

def double_sha256(data):
    return hashlib.sha256(hashlib.sha256(data).digest()).hexdigest()

def header_without_nonce(block):
    version   = struct.pack("<I", block.get("ver", 0))
    prevhash  = bytes.fromhex(block["prev_block"])[::-1] if "prev_block" in block else b"\x00"*32
    merkle    = bytes.fromhex(block["mrkl_root"])[::-1]  if "mrkl_root" in block else b"\x00"*32
    timestamp = struct.pack("<I", block.get("time", 0))
    bits      = struct.pack("<I", block.get("bits", 0))
    return version + prevhash + merkle + timestamp + bits

def count_zeros(h):
    z = 0
    for c in h:
        if c == "0": z += 1
        else: break
    return z

# â”€â”€ Main Map Test â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def run():
    print(f"[!] GODSEYE 14.1 â€” CARTOGRAPHER OFFLINE MAP TEST", flush=True)
    print(f"    Loading SovereignMath ...", flush=True)

    math_engine = SovereignMath()
    target_vec  = math_engine._0x_expand(ZERO_TARGET_STR)
    print(f"    Target vector: {len(target_vec)}D\n")

    # Load the downloaded blocks
    with open("bitcoin_causation_map.json") as f:
        data = json.load(f)

    blocks = data["blocks"]
    print(f"    Blocks loaded: {len(blocks)}\n")
    print("=" * 72)

    winner_higher  = 0   # cases where winner has highest resonance
    winner_lower   = 0   # cases where winner is NOT highest
    total_blocks   = 0
    resonance_gaps = []  # difference between winner res and neighbor avg

    NEIGHBOR_RANGE = 16  # probe this many nonces each side of the winner

    for block in blocks:
        winning_nonce = block["nonce"]
        zeros         = block["zeros"]
        height        = block["height"]

        # Rebuild header WITHOUT nonce (76 bytes)
        try:
            base_header = header_without_nonce(block)
        except Exception as e:
            print(f"  [!] Block #{height} header build failed: {e}")
            continue

        # Measure resonance of the WINNING nonce (hash output, not raw input)
        winner_nonce_bytes = struct.pack("<I", winning_nonce)
        winner_digest = double_sha256(base_header + winner_nonce_bytes)
        winner_vec = math_engine._0x_expand(winner_digest)
        winner_res = math_engine._0x_resonance(winner_vec, target_vec)

        # Measure resonance of NEIGHBORS (also via hash output)
        neighbor_res = []
        probe_step   = max(1, winning_nonce // NEIGHBOR_RANGE)

        for i in range(1, NEIGHBOR_RANGE + 1):
            for direction in [-1, 1]:
                n = (winning_nonce + direction * probe_step * i) & 0xFFFFFFFF
                digest = double_sha256(base_header + struct.pack("<I", n))
                vec = math_engine._0x_expand(digest)
                res = math_engine._0x_resonance(vec, target_vec)
                neighbor_res.append(res)

        avg_neighbor = sum(neighbor_res) / len(neighbor_res) if neighbor_res else 0
        max_neighbor = max(neighbor_res) if neighbor_res else 0
        gap          = winner_res - avg_neighbor
        resonance_gaps.append(gap)

        is_winner = "*** WINNER ***" if winner_res > max_neighbor else "  lower     "
        if winner_res > max_neighbor:
            winner_higher += 1
        else:
            winner_lower  += 1

        total_blocks += 1

        print(
            f"  Block #{height:>7} | ZEROS: {zeros:>2} | "
            f"WIN RES: {winner_res:.4f} | AVG NEIGH: {avg_neighbor:.4f} | "
            f"GAP: {gap:+.4f} | {is_winner}",
            flush=True
        )

    # â”€â”€ Summary â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    print(f"\n{'='*72}")
    print(f"  CARTOGRAPHER MAP RESULTS â€” {total_blocks} BLOCKS")
    print(f"{'='*72}")
    print(f"  Winner had HIGHER resonance than neighbors : {winner_higher}/{total_blocks} ({winner_higher/total_blocks*100:.1f}%)")
    print(f"  Winner had LOWER  resonance than neighbors : {winner_lower}/{total_blocks} ({winner_lower/total_blocks*100:.1f}%)")

    avg_gap = sum(resonance_gaps) / len(resonance_gaps) if resonance_gaps else 0
    pos_gaps = [g for g in resonance_gaps if g > 0]
    print(f"\n  Average resonance gap (winner - neighbors): {avg_gap:+.4f}")
    print(f"  Positive gaps (winner above avg)          : {len(pos_gaps)}/{len(resonance_gaps)}")

    print(f"\n  VERDICT:")
    if winner_higher / total_blocks > 0.55:
        print(f"  *** COMPASS IS REAL â€” winning nonces cluster at higher resonance ***")
        print(f"  *** The Cartographer can navigate toward the target. ***")
    elif winner_higher / total_blocks > 0.45:
        print(f"  SIGNAL PRESENT but weak â€” increase sample size to confirm")
    else:
        print(f"  No consistent resonance advantage detected in this sample")
        print(f"  The raw input may be too distant from hash output in 68D space")
        print(f"  Consider: expand hash OUTPUT instead of raw input for mapping")

if __name__ == "__main__":
    run()
