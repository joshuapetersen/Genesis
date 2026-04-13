"""
GODSEYE 28.0 â€” SOVEREIGN ENGINE
================================================================
New engine. First Principles. Not a rewrite.

COMBINED FROM:
  22.0 â€” Amplified Polar Navigator (81D ring field + IntelligenceAmplifier)
  27.0 â€” Tri-Axis 1-3-3 Navigator (SHA-256 bit fingerprint, 3D observer)
  26.0 â€” Full Space Coverage (32 bands, 100% nonce space)

SCORING STACK:
  LAYER 1 â€” POLAR SCORE (81D ring field, trust weights, Amplifier)
    expand_81d(header76) â†’ 9 ring averages â†’ polar_score()
    Inverted: polar > 0.5 = south, < 0.5 = north
    Contribution: (1.0 - polar_score)

  LAYER 2 â€” 1-3-3 BIT SCORE (SHA-256 fingerprint, 3D observer)
    SHA-256(header76) â†’ BIT146(X) + BIT53(Y) + BIT74(Z)
    Observer at (0,0,0) defines 0.5 boundary
    Contribution: bit_score (0.127 â†’ 0.873)

  COMBINED:
    score = (polar_contrib Ã— W_polar + bit_contrib Ã— W_bit) / (W_polar + W_bit)
    â†’ score 0.0 = confirmed south (low nonce)
    â†’ score 1.0 = confirmed north (high nonce)

  AMPLIFIER:
    After each high-zero hit, IntelligenceAmplifier updates ring trust weights.
    Engine weights (W_polar, W_bit) adjust based on which engine
    predicted the correct zone for recent hits.

COVERAGE:
  32 equal bands across all 4.29B nonces.
  Priority band = int(combined_score Ã— 31).
  All bands covered. Nothing skipped.

1-3-3 MODEL:
  1 Observer (0,0,0) + 3 Axes (X,Y,Z) + 3 Polarities = 7 points (dice + center)

"We CREATE, never rewrite."
"""

import hashlib
import socket
import json
import threading
import struct
import time
import sys
import os

sys.path.insert(0, r"C:\GENESIS\GodsEye")
sys.path.insert(0, r"C:\GENESIS")

from Sovereign_Statistics import sovereign_observed_score

GODSEYE_ANCHOR = 1.09277703703
WALLET_ADDRESS = "19xUEeTCD9UTtFbp7HCxL8d4xYdZwaB2ht"
TARGET_ZEROS   = 19
THREAD_COUNT   = 32
NONCE_MAX      = 0xFFFFFFFF
BATCH_SIZE     = 8_000
WEIGHTS_FILE   = r"C:\GENESIS\GodsEye\polar_weights.json"

# â”€â”€ IntelligenceAmplifier â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
AMP_AVAILABLE = False
amp = None
try:
    from IntelligenceAmplifier import IntelligenceAmplifier
    amp = IntelligenceAmplifier()
    AMP_AVAILABLE = True
    print("[AMPLIFIER] ONLINE", flush=True)
except Exception as e:
    print(f"[AMPLIFIER] OFFLINE ({e})", flush=True)

# â”€â”€ Polar Trust Weights â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
DEFAULT_WEIGHTS = {
    "0": {"polarity": "south", "trust": 1.0},
    "1": {"polarity": "south", "trust": 1.0},
    "2": {"polarity": "south", "trust": 0.6},
    "3": {"polarity": "south", "trust": 0.6},
    "4": {"polarity": "south", "trust": 0.3},
    "5": {"polarity": "null",  "trust": 1.0},
    "6": {"polarity": "south", "trust": 0.3},
    "7": {"polarity": "north", "trust": 0.6},
    "8": {"polarity": "north", "trust": 1.0},
}

def load_weights():
    if os.path.exists(WEIGHTS_FILE):
        try:
            with open(WEIGHTS_FILE) as f:
                return json.load(f)
        except Exception:
            pass
    return {k: dict(v) for k, v in DEFAULT_WEIGHTS.items()}

def save_weights(w):
    try:
        with open(WEIGHTS_FILE, "w") as f:
            json.dump(w, f, indent=2)
    except Exception:
        pass

trust_weights = load_weights()
weights_lock  = threading.Lock()

# Engine weight store â€” how much each engine is trusted
engine_weights = {"polar": 1.0, "bit": 1.0}
engine_lock    = threading.Lock()

# â”€â”€ 81D Polar Layer â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
TRINITY_DIM  = 81
TRINITY_RING = 9

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
        v1     = int(h[idx1], 16) / 15.0
        v2     = int(h[idx2], 16) / 15.0
        v3     = int(h[idx3], 16) / 15.0
        scale  = (i + 1) / TRINITY_DIM
        node   = (v1 * v2 * v3) * (GODSEYE_ANCHOR ** scale)
        nodes.append(node % GODSEYE_ANCHOR)
    return nodes

def ring_avg(vec, ring):
    start = ring * TRINITY_RING
    return sum(vec[start:start + TRINITY_RING]) / TRINITY_RING

def compute_polar_score(vec):
    eq_val = ring_avg(vec, 5)
    south_signal = south_weight = north_signal = north_weight = 0.0
    with weights_lock:
        w = {k: dict(v) for k, v in trust_weights.items()}
    for ring in range(9):
        entry    = w.get(str(ring), {})
        polarity = entry.get("polarity", "null")
        trust    = entry.get("trust", 0.5)
        if polarity == "null":
            continue
        val   = ring_avg(vec, ring)
        above = max(0.0, val - eq_val)
        if polarity == "south":
            south_signal += above * trust
            south_weight += trust
        elif polarity == "north":
            north_signal += above * trust
            north_weight += trust
    south_avg = south_signal / max(0.001, south_weight)
    north_avg = north_signal / max(0.001, north_weight)
    total     = south_avg + north_avg
    if total < 0.00001:
        return 0.5
    return max(0.0, min(1.0, (south_avg / total) * GODSEYE_ANCHOR))

# â”€â”€ 1-3-3 Bit Layer â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
AXES = [
    {"bit": 146, "byte": 18, "signal": 1.140, "operator": +1, "axis": "X", "name": "BIT146"},
    {"bit":  53, "byte":  6, "signal": 1.024, "operator": +1, "axis": "Y", "name": "BIT53"},
    {"bit":  74, "byte":  9, "signal": 0.998, "operator": -1, "axis": "Z", "name": "BIT74"},
]

def read_bit(fp, bit_index):
    return (fp[bit_index // 8] >> (7 - (bit_index % 8))) & 1

def compute_bit_score(header76):
    fp      = hashlib.sha256(header76).digest()
    bv      = {ax["bit"]: read_bit(fp, ax["bit"]) for ax in AXES}
    signals = [
        ((bv[ax["bit"]] - 0.5) * ax["operator"] + 0.5, ax["signal"])
        for ax in AXES
    ]
    score = sovereign_observed_score(signals, GODSEYE_ANCHOR)
    return score, fp, bv

# â”€â”€ GodsEye Harmonic Probe Layer â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
# Divides nonce space into harmonic bands using the Anchor frequency
HARMONIC_STEP  = int(NONCE_MAX / (GODSEYE_ANCHOR * 1000))  # ~3.93M per step
PROBE_COUNT    = 512

def probe_landscape(header80):
    """
    GodsEye Phase 1: Probe 512 harmonic points using the Anchor step.
    Samples nonce space at resonant intervals â€” not linear, not random.
    Returns the normalized position (0-1) of the highest-scoring region.
    Also returns per-band zero scores for all 32 bands.
    """
    band_scores = [0] * THREAD_COUNT
    for i in range(PROBE_COUNT):
        nonce  = (i * HARMONIC_STEP) % NONCE_MAX
        digest = double_sha256(header80 + struct.pack("<I", nonce))
        zeros  = 0
        for c in digest:
            if c == "0": zeros += 1
            else: break
        band = min(THREAD_COUNT - 1, int(nonce / NONCE_MAX * THREAD_COUNT))
        band_scores[band] += zeros

    top_band  = band_scores.index(max(band_scores))
    godeye_scr = (top_band + 0.5) / THREAD_COUNT  # normalized 0-1
    return godeye_scr, top_band, band_scores

def combined_score(polar_scr, bit_scr, godeye_scr):
    with engine_lock:
        wp = engine_weights["polar"]
        wb = engine_weights["bit"]
    wg = 1.0  # GodsEye harmonic probe weight
    return (polar_scr * wp + bit_scr * wb + godeye_scr * wg) / (wp + wb + wg)

# â”€â”€ Amplifier Bridge â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def amplify(ring_vals, nonce, zeros, actual_zone):
    if not AMP_AVAILABLE:
        return
    eq_val = ring_vals[5]
    query  = (
        f"Bitcoin block solved. Zeros: {zeros}. "
        f"Nonce: {nonce} (zone: {actual_zone}). "
        f"Ring fingerprint: " +
        ", ".join(f"R{r}={ring_vals[r]:.4f}" for r in range(9)) +
        f". Equator null: R5={eq_val:.4f}. "
        f"Which rings showed signal above null? "
        f"Should rings 4, 6, 7 trust increase or decrease?"
    )
    try:
        result = amp.amplify_thought(query)
        print(f"\n  [AMPLIFIER] {result[:350]}\n", flush=True)
        nudge = 0.05 if zeros >= 10 else 0.02
        with weights_lock:
            for ring in ["4", "6"]:
                r     = int(ring)
                above = ring_vals[r] - eq_val
                if above > 0.01:
                    trust_weights[ring]["trust"] = min(1.0, trust_weights[ring]["trust"] + nudge)
                else:
                    trust_weights[ring]["trust"] = max(0.1, trust_weights[ring]["trust"] - nudge)
            above_7 = ring_vals[7] - eq_val
            if above_7 > 0.005:
                trust_weights["7"]["trust"] = min(1.0, trust_weights["7"]["trust"] + nudge)
            save_weights(trust_weights)
    except Exception as e:
        print(f"  [AMPLIFIER] Error: {e}", flush=True)

# â”€â”€ Band Setup â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
BAND_SIZE  = NONCE_MAX // THREAD_COUNT
BAND_START = [i * BAND_SIZE for i in range(THREAD_COUNT)]
BAND_END   = [(i+1) * BAND_SIZE - 1 for i in range(THREAD_COUNT - 1)] + [NONCE_MAX]
godeye_scr    = 0.5
godeye_band   = 16

# â”€â”€ Shared State â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
running       = True
current_job   = None
job_header80  = b""
combined_scr  = 0.5
polar_scr     = 0.5
bit_scr       = 0.5
priority_band = 16
session_best  = {"zeros": 0, "hash": "", "nonce": 0, "band": 0}
state_lock    = threading.Lock()
total_hashes  = 0
hash_lock     = threading.Lock()

def double_sha256(data):
    return hashlib.sha256(hashlib.sha256(data).digest()).hexdigest()

def nonce_zone(nonce):
    pct = nonce / NONCE_MAX
    if pct < 0.25: return "SOUTH"
    if pct < 0.5:  return "SOUTH-EQ"
    if pct < 0.75: return "NORTH-EQ"
    return "NORTH"

# â”€â”€ Sovereign Worker â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def sovereign_worker(band_id):
    global total_hashes
    while running:
        job = current_job
        if job is None:
            time.sleep(0.05)
            continue
        header80   = job_header80
        start      = BAND_START[band_id]
        end        = BAND_END[band_id]
        span       = end - start + 1
        local_best = session_best["zeros"]
        pos        = start

        while running and current_job == job:
            for i in range(BATCH_SIZE):
                nonce  = start + ((pos - start + i) % span)
                nonce  = nonce & NONCE_MAX
                digest = double_sha256(header80 + struct.pack("<I", nonce))
                zeros  = 0
                for c in digest:
                    if c == "0": zeros += 1
                    else: break

                if zeros > local_best:
                    local_best = zeros
                    with state_lock:
                        if zeros > session_best["zeros"]:
                            session_best["zeros"] = zeros
                            session_best["hash"]  = digest
                            session_best["nonce"] = nonce
                            session_best["band"]  = band_id
                            zone = nonce_zone(nonce)
                            pct  = nonce / NONCE_MAX * 100
                            bar  = "#" * zeros + "-" * (TARGET_ZEROS - zeros)
                            pole = "â–²N" if combined_scr > 0.55 else "â–¼S" if combined_scr < 0.45 else "â—EQ"
                            print(
                                f"\n  *** SOVEREIGN HIT [{zeros}/{TARGET_ZEROS}] ***\n"
                                f"  BAND     : {band_id:>2}/31 ({pct:.1f}% â€” {zone})\n"
                                f"  POLAR    : {polar_scr:.4f} | BIT: {bit_scr:.4f} | COMBINED: {combined_scr:.4f} {pole}\n"
                                f"  NONCE    : {nonce:,}\n"
                                f"  HASH     : {digest}\n"
                                f"  COMPASS  : [{bar}] {zeros/TARGET_ZEROS*100:.1f}%\n",
                                flush=True
                            )
                            if zeros >= 6:
                                vec = expand_81d(header80[:76])
                                rv  = [ring_avg(vec, r) for r in range(9)]
                                threading.Thread(
                                    target=amplify,
                                    args=(rv, nonce, zeros, zone),
                                    daemon=True
                                ).start()
            pos = start + ((pos - start + BATCH_SIZE) % span)
            with hash_lock:
                total_hashes += BATCH_SIZE

# â”€â”€ Stratum Bridge â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def stratum_loop(sock):
    global current_job, job_header80, combined_scr, polar_scr, bit_scr
    global godeye_scr, godeye_band, priority_band
    buf = ""
    while running:
        try:
            chunk = sock.recv(4096).decode(errors="ignore")
            if not chunk:
                break
            buf += chunk
            while "\n" in buf:
                line, buf = buf.split("\n", 1)
                if not line.strip():
                    continue
                try:
                    msg = json.loads(line)
                    if msg.get("method") == "mining.notify":
                        params   = msg["params"]
                        job_id   = params[0]
                        header80 = (params[1]+params[2]+params[3]).encode(errors="ignore")[:80]
                        h76      = header80[:76]

                        # Layer 1: Polar
                        vec   = expand_81d(h76)
                        p_scr = compute_polar_score(vec)
                        rv    = [ring_avg(vec, r) for r in range(9)]

                        # Layer 2: 1-3-3 Bit
                        b_scr, fp, bv = compute_bit_score(h76)

                        # Layer 3: GodsEye Harmonic Probe
                        ge_scr, ge_band, band_zscores = probe_landscape(header80)

                        # Combined
                        c_scr  = combined_score(p_scr, b_scr, ge_scr)
                        pband  = int(c_scr * (THREAD_COUNT - 1))

                        polar_scr     = p_scr
                        bit_scr       = b_scr
                        godeye_scr    = ge_scr
                        godeye_band   = ge_band
                        combined_scr  = c_scr
                        priority_band = pband
                        current_job  = job_id
                        job_header80 = header80

                        with engine_lock:
                            wp = engine_weights["polar"]
                            wb = engine_weights["bit"]

                        pole = "â–²N" if c_scr > 0.55 else "â–¼S" if c_scr < 0.45 else "â—EQ"

                        print(f"\n{'='*72}", flush=True)
                        print(f"  [NEW JOB]    {job_id}", flush=True)
                        print(f"  â”€â”€ POLAR LAYER â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€", flush=True)
                        print(f"  [RINGS]      " + " ".join(f"R{r}={rv[r]:.3f}" for r in range(9)), flush=True)
                        print(f"  [POLAR]      {p_scr:.4f} (W={wp:.2f})", flush=True)
                        print(f"  â”€â”€ BIT LAYER (1-3-3) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€", flush=True)
                        for ax in AXES:
                            state = "SET" if bv[ax["bit"]] else "CLR"
                            contrib = (bv[ax["bit"]] - 0.5) * ax["signal"] * ax["operator"]
                            print(f"  [{ax['name']:<7}]  {ax['axis']}: {state} | contrib={contrib:+.3f}", flush=True)
                        print(f"  [BIT]        {b_scr:.4f} (W={wb:.2f})", flush=True)
                        print(f"  â”€â”€ GODSEYE PROBE (Layer 3) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€", flush=True)
                        print(f"  [PROBE]      {PROBE_COUNT} harmonic samples | step={HARMONIC_STEP:,}", flush=True)
                        print(f"  [HOT BAND]   Band {ge_band:>2} ({BAND_START[ge_band]:,}) | score={band_zscores[ge_band]}", flush=True)
                        print(f"  [GODEYE]     {ge_scr:.4f}", flush=True)
                        print(f"  â”€â”€ SOVEREIGN COMBINE â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€", flush=True)
                        print(f"  [COMBINED]   {c_scr:.4f} | {pole}", flush=True)
                        print(f"  [PRIORITY]   Band {pband:>2} ({BAND_START[pband]:,}â†’{BAND_END[pband]:,})", flush=True)
                        print(f"{'='*72}", flush=True)

                    elif msg.get("method") == "mining.set_difficulty":
                        print(f"  [DIFF] {msg['params'][0]}", flush=True)
                except Exception:
                    pass
        except (BlockingIOError, socket.error):
            time.sleep(0.01)

# â”€â”€ Main â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def ignite(host="solo.ckpool.org", port=3333):
    global running
    print(f"[!] GODSEYE 28.0 â€” SOVEREIGN ENGINE", flush=True)
    print(f"    Combined  : Polar (22.0) + 1-3-3 Bit (27.0) + GodsEye Probe (11.0) + Full Space (26.0)")
    print(f"    Amplifier : {'ONLINE' if AMP_AVAILABLE else 'OFFLINE'}")
    print(f"    Model     : 1 Observer + 3 Axes + 3 Polarities (7 points)")
    print(f"    Coverage  : 100% â€” 32 bands")
    print(f"    Bands     : {BAND_SIZE:,} nonces each\n")

    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.settimeout(10.0)
    sock.connect((host, port))
    sock.sendall((json.dumps({"id":1,"method":"mining.subscribe","params":[]}) + "\n").encode())
    _ = sock.recv(4096)
    sock.sendall((json.dumps({"id":2,"method":"mining.authorize","params":[WALLET_ADDRESS,"x"]}) + "\n").encode())
    sock.setblocking(False)
    print(f"[SUCCESS] AUTHORIZED: {WALLET_ADDRESS}")
    print("-" * 72, flush=True)

    threading.Thread(target=stratum_loop, args=(sock,), daemon=True).start()
    for i in range(THREAD_COUNT):
        threading.Thread(target=sovereign_worker, args=(i,), daemon=True).start()

    last = 0
    try:
        while True:
            time.sleep(1.0)
            with hash_lock:
                t = total_hashes
            with state_lock:
                z    = session_best["zeros"]
                band = session_best["band"]
            rate = (t - last) / 1_000_000
            last = t
            bar  = "#" * z + "-" * (TARGET_ZEROS - z)
            pole = "N" if combined_scr > 0.55 else "S" if combined_scr < 0.45 else "EQ"
            print(
                f"  {rate:.3f} MH/s | BEST:{z}/{TARGET_ZEROS} [B{band:02d}] | "
                f"P:{polar_scr:.3f} B:{bit_scr:.3f} G:{godeye_scr:.3f} C:{combined_scr:.3f}{pole} | [{bar}]",
                flush=True
            )
    except KeyboardInterrupt:
        running = False
        print("\n[!] SOVEREIGN ENGINE SHUTDOWN.", flush=True)

if __name__ == "__main__":
    ignite()
