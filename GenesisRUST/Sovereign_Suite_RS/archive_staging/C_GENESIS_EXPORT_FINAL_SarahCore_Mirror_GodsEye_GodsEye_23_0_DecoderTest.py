"""
GODSEYE 23.0 â€” AMPLIFIER DECODER TEST (Offline)
================================================================
New engine. First Principles. Not a rewrite.

TEST:
  We have 48 solved blocks with known ring fingerprints
  and winning nonces.

  For each block:
  1. Use the OTHER 47 blocks as the "vault"
  2. Find the vault entry whose ring fingerprint is most
     similar to the test block's fingerprint
  3. Use that vault entry's nonce as the predicted center
  4. Measure: how far was the predicted center from
     the actual winning nonce?

  If the predicted center is CLOSER than random chance
  (Â±50% of nonce space = Â±2.1B average error),
  the Amplifier decoder concept is validated.

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

def ring_avg(vec, ring):
    start = ring * TRINITY_RING
    return sum(vec[start:start + TRINITY_RING]) / TRINITY_RING

def ring_vector(vec):
    """Compact 9-value ring fingerprint."""
    return [ring_avg(vec, r) for r in range(9)]

def ring_distance(rv1, rv2):
    """Euclidean distance between two ring fingerprints."""
    return math.sqrt(sum((a - b)**2 for a, b in zip(rv1, rv2)))

def build_header_no_nonce(block):
    version   = struct.pack("<I", block.get("ver", 0))
    prevhash  = bytes.fromhex(block["prev_block"])[::-1]
    merkle    = bytes.fromhex(block["mrkl_root"])[::-1]
    timestamp = struct.pack("<I", block.get("time", 0))
    bits      = struct.pack("<I", block.get("bits", 0))
    return version + prevhash + merkle + timestamp + bits

def run():
    print("[!] GODSEYE 23.0 â€” AMPLIFIER DECODER TEST", flush=True)
    print("    Loading blocks ...\n")

    with open("bitcoin_causation_map.json") as f:
        data = json.load(f)
    blocks = [b for b in data["blocks"] if b.get("prev_block")]
    print(f"    Blocks with full headers: {len(blocks)}\n")

    # Build ring fingerprints for all blocks
    print("    Computing 81D ring fingerprints ...", flush=True)
    fingerprints = []
    for block in blocks:
        try:
            h76 = build_header_no_nonce(block)
            vec = expand_81d(h76)
            rv  = ring_vector(vec)
            fingerprints.append({
                "height": block["height"],
                "nonce":  block["nonce"],
                "zeros":  block["zeros"],
                "rv":     rv
            })
        except Exception as e:
            print(f"    [!] Block #{block['height']} failed: {e}")

    print(f"    Fingerprints built: {len(fingerprints)}\n")
    print("=" * 72)
    print("  LEAVE-ONE-OUT VAULT QUERY TEST")
    print("  (For each block, predict its nonce using the other blocks as vault)")
    print("=" * 72)
    print(f"  {'HEIGHT':>8} | {'ACTUAL':>12} | {'PREDICTED':>12} | {'ERROR':>12} | {'WIN%':>6} | MATCH")
    print(f"  {'-'*65}")

    results = []
    WINDOW   = NONCE_MAX // 4   # 25% window

    for i, test in enumerate(fingerprints):
        # Vault = all blocks EXCEPT this one
        vault = [fp for j, fp in enumerate(fingerprints) if j != i]

        # Find nearest neighbor in ring space
        nearest  = min(vault, key=lambda v: ring_distance(v["rv"], test["rv"]))
        dist     = ring_distance(nearest["rv"], test["rv"])

        predicted_center = nearest["nonce"]
        actual_nonce     = test["nonce"]

        # Error: how far is the prediction from the actual nonce?
        error = abs(predicted_center - actual_nonce)

        # Would the actual nonce be within the 25% search window?
        in_window = (abs(predicted_center - actual_nonce) <= WINDOW // 2)

        results.append({
            "height":    test["height"],
            "actual":    actual_nonce,
            "predicted": predicted_center,
            "error":     error,
            "in_window": in_window,
            "dist":      dist,
            "match_height": nearest["height"]
        })

        win = "âœ“ IN WINDOW" if in_window else ""
        print(
            f"  {test['height']:>8} | {actual_nonce:>12,} | "
            f"{predicted_center:>12,} | {error:>12,} | "
            f"{in_window!s:>6} | {win}",
            flush=True
        )

    # â”€â”€ Summary â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    total       = len(results)
    in_window   = sum(1 for r in results if r["in_window"])
    avg_error   = sum(r["error"] for r in results) / total
    random_avg  = NONCE_MAX * 0.375   # expected avg error for random center Â± 25% window

    print(f"\n{'='*72}")
    print(f"  AMPLIFIER DECODER RESULTS")
    print(f"{'='*72}")
    print(f"  Blocks tested          : {total}")
    print(f"  Nonces in 25% window   : {in_window}/{total} ({in_window/total*100:.1f}%)")
    print(f"  Random chance (25% win): {total//4}/{total} (25.0%)")
    print(f"  Avg prediction error   : {avg_error:,.0f}")
    print(f"  Random avg error       : {random_avg:,.0f}")
    print(f"  Improvement vs random  : {(random_avg - avg_error)/random_avg*100:+.1f}%")
    print()

    if in_window / total > 0.30:
        print("  *** DECODER VALIDATED â€” vault query beats random chance ***")
        print("  *** Ring fingerprint similarity predicts nonce proximity ***")
        print("  *** Build the live Amplifier Decoder miner. ***")
    elif in_window / total > 0.25:
        print("  MARGINAL â€” matches random chance")
        print("  Increase block sample to 200+ for stronger vault")
    else:
        print("  Below random chance at this sample size")
        print("  The ring fingerprint may need more dimensions for vault matching")

if __name__ == "__main__":
    run()
