"""
GODSEYE 25.1 — TRI-ZONE BLOCK TEST (Offline)
================================================================
New engine. First Principles. Not a rewrite.

TEST:
  Using all 256 solved blocks in bitcoin_causation_map.json,
  simulate the Tri-Zone Navigator:

  1. Compute SHA-256(header76) fingerprint
  2. Read BIT 146 and BIT 53
  3. Compute bit score → allocate threads to zones
  4. Check: does the actual winning nonce fall in:
       SOUTH    (0     → 25%  of NONCE_MAX)
       EQUATOR  (37.5% → 62.5% of NONCE_MAX)
       NORTH    (75%   → 100% of NONCE_MAX)

  Compare:
    - Single 25% window hit rate (old approach)
    - Tri-zone 75% coverage hit rate (new approach)
    - Which zone catches the most winners?
    - Does the bit score PREDICT the correct zone?

"We CREATE, never rewrite."
"""

import sys
import json
import struct
import hashlib

sys.path.insert(0, r"C:\GENESIS\GodsEye")
sys.path.insert(0, r"C:\GENESIS")

NONCE_MAX    = 0xFFFFFFFF
SIGNAL_BITS  = [
    {"bit": 146, "signal": 1.140, "name": "BIT146"},
    {"bit":  53, "signal": 1.024, "name": "BIT53"},
]
TOTAL_SIGNAL = sum(b["signal"] for b in SIGNAL_BITS)
EQUATOR_BASE = 0.25

ZONES = {
    "SOUTH":   (0,                     NONCE_MAX // 4),
    "EQUATOR": (int(NONCE_MAX * 0.375), int(NONCE_MAX * 0.625)),
    "NORTH":   (int(NONCE_MAX * 0.75),  NONCE_MAX),
    "GAP_LO":  (NONCE_MAX // 4,         int(NONCE_MAX * 0.375)),
    "GAP_HI":  (int(NONCE_MAX * 0.625), int(NONCE_MAX * 0.75)),
}
SEARCHED_ZONES = ["SOUTH", "EQUATOR", "NORTH"]

def read_bit(fp, bit_index):
    return (fp[bit_index // 8] >> (7 - (bit_index % 8))) & 1

def compute_score(header76):
    fp = hashlib.sha256(header76).digest()
    s  = sum(read_bit(fp, sb["bit"]) * sb["signal"] for sb in SIGNAL_BITS)
    return s / TOTAL_SIGNAL, fp

def build_header_no_nonce(block):
    version  = struct.pack("<I", block.get("ver", 0))
    prevhash = bytes.fromhex(block["prev_block"])[::-1]
    merkle   = bytes.fromhex(block["mrkl_root"])[::-1]
    ts       = struct.pack("<I", block.get("time", 0))
    bits     = struct.pack("<I", block.get("bits", 0))
    return version + prevhash + merkle + ts + bits

def get_zone(nonce):
    for name, (start, end) in ZONES.items():
        if start <= nonce <= end:
            return name
    return "UNKNOWN"

def allocate_threads(score, total=32):
    eq   = max(2, int(total * EQUATOR_BASE))
    rem  = total - eq
    n    = int(rem * score)
    s    = rem - n
    return {"SOUTH": s, "EQUATOR": eq, "NORTH": n}

def predicted_zone(score):
    if score > 0.55:
        return "NORTH"
    elif score < 0.45:
        return "SOUTH"
    else:
        return "EQUATOR"

def run():
    print("[!] GODSEYE 25.1 — TRI-ZONE BLOCK TEST", flush=True)

    with open("bitcoin_causation_map.json") as f:
        data = json.load(f)
    blocks = [b for b in data["blocks"] if b.get("prev_block")]
    print(f"    Blocks: {len(blocks)}\n")

    zone_counts     = {z: 0 for z in ZONES}
    zone_predicted  = {z: 0 for z in SEARCHED_ZONES}
    correct_predict = 0
    trizone_hits    = 0
    single_hits     = 0  # old: single 25% window matching score

    results = []

    for block in blocks:
        try:
            h76    = build_header_no_nonce(block)
            nonce  = block["nonce"]
            score, fp = compute_score(h76)
            alloc  = allocate_threads(score)
            pred   = predicted_zone(score)
            actual = get_zone(nonce)
            bv     = {sb["bit"]: read_bit(fp, sb["bit"]) for sb in SIGNAL_BITS}

            zone_counts[actual] = zone_counts.get(actual, 0) + 1

            in_trizone = actual in SEARCHED_ZONES
            if in_trizone:
                trizone_hits += 1

            correct = (pred == actual and actual in SEARCHED_ZONES)
            if correct:
                correct_predict += 1

            # Old single-window: 25% window centered on score×NONCE_MAX
            old_center = int(score * NONCE_MAX)
            old_window = NONCE_MAX // 4
            in_single  = abs(nonce - old_center) <= old_window // 2
            if in_single:
                single_hits += 1

            results.append({
                "height": block["height"],
                "nonce":  nonce,
                "score":  score,
                "pred":   pred,
                "actual": actual,
                "correct": correct,
                "trizone": in_trizone,
                "single":  in_single,
                "b146":   bv[146],
                "b53":    bv[53],
                "zeros":  block.get("zeros", 0),
                "alloc":  alloc,
            })
        except Exception as e:
            pass

    n = len(results)

    # ── Zone Distribution ─────────────────────────────────────────────────────
    print("=" * 72)
    print("  NONCE ZONE DISTRIBUTION (where winning nonces actually live)")
    print("=" * 72)
    print(f"  {'ZONE':<12} | {'COUNT':>6} | {'PCT':>6} | {'RANGE'}")
    print(f"  {'-'*55}")
    for zone, (start, end) in ZONES.items():
        count = zone_counts.get(zone, 0)
        pct   = count / n * 100
        marker = " ← TRI-ZONE" if zone in SEARCHED_ZONES else " ← GAP (not searched)"
        print(f"  {zone:<12} | {count:>6} | {pct:>5.1f}% | {start:,}→{end:,}{marker}")

    # ── Bit Score Breakdown ───────────────────────────────────────────────────
    print(f"\n{'='*72}")
    print(f"  BIT STATE BREAKDOWN")
    print(f"{'='*72}")
    cases = [
        (1, 1, "BIT146=SET BIT53=SET  → score=1.0  → NORTH"),
        (1, 0, "BIT146=SET BIT53=CLR  → score=0.527 → NORTH/EQ"),
        (0, 1, "BIT146=CLR BIT53=SET  → score=0.473 → SOUTH/EQ"),
        (0, 0, "BIT146=CLR BIT53=CLR  → score=0.0  → SOUTH"),
    ]
    for b146, b53, label in cases:
        subset = [r for r in results if r["b146"] == b146 and r["b53"] == b53]
        if not subset:
            continue
        zones_in = {}
        for r in subset:
            zones_in[r["actual"]] = zones_in.get(r["actual"], 0) + 1
        zone_str = " | ".join(f"{z}={c}" for z, c in sorted(zones_in.items()))
        print(f"  {label}")
        print(f"    n={len(subset)} | {zone_str}")

    # ── Coverage Comparison ───────────────────────────────────────────────────
    print(f"\n{'='*72}")
    print(f"  COVERAGE COMPARISON")
    print(f"{'='*72}")
    print(f"  Single 25% window (old)   : {single_hits}/{n} ({single_hits/n*100:.1f}%)")
    print(f"  Tri-zone 75% (new)        : {trizone_hits}/{n} ({trizone_hits/n*100:.1f}%)")
    print(f"  Expected random (75%)     : {int(n*0.75)}/{n} (75.0%)")
    print(f"  Prediction accuracy       : {correct_predict}/{n} ({correct_predict/n*100:.1f}%)")

    # ── Per-Zone Prediction Analysis ─────────────────────────────────────────
    print(f"\n{'='*72}")
    print(f"  ZONE PREDICTION ACCURACY (does bit score predict the right zone?)")
    print(f"{'='*72}")
    for zone in SEARCHED_ZONES:
        predicted_here = [r for r in results if r["pred"] == zone]
        actually_here  = [r for r in results if r["actual"] == zone]
        correct_here   = [r for r in results if r["pred"] == zone and r["actual"] == zone]
        precision      = len(correct_here) / max(1, len(predicted_here)) * 100
        recall         = len(correct_here) / max(1, len(actually_here)) * 100
        print(f"  {zone:<8}: predicted={len(predicted_here):>3} "
              f"| actual={len(actually_here):>3} "
              f"| correct={len(correct_here):>3} "
              f"| precision={precision:.1f}% recall={recall:.1f}%")

    # ── Recommendation ────────────────────────────────────────────────────────
    gap_count = sum(zone_counts.get(g, 0) for g in ["GAP_LO", "GAP_HI"])
    print(f"\n{'='*72}")
    print(f"  SUMMARY")
    print(f"{'='*72}")
    print(f"  Nonces in searched tri-zones  : {trizone_hits}/{n} ({trizone_hits/n*100:.1f}%)")
    print(f"  Nonces in GAP (not searched)  : {gap_count}/{n} ({gap_count/n*100:.1f}%)")
    print(f"  Uplift vs single window       : {(trizone_hits-single_hits)/n*100:+.1f}% more coverage")
    if gap_count / n > 0.15:
        print(f"\n  *** GAP IS HOT: {gap_count/n*100:.1f}% of winners in uncovered zone ***")
        print(f"  *** Consider expanding to 4 or 5 zones ***")
    else:
        print(f"\n  Tri-Zone covers {trizone_hits/n*100:.1f}% of historical winners.")

if __name__ == "__main__":
    run()
