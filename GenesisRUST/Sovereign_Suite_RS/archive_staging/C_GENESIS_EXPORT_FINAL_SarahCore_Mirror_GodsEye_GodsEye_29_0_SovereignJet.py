"""
GODSEYE 29.0 â€” SOVEREIGN JET ENGINE
================================================================
New engine. First Principles. Not a rewrite.

All scoring layers from 28.0 PLUS MiningJetEngine acceleration.

ARCHITECTURE:
  NAVIGATOR (main thread):
    â†’ Polar score (Layer 1)
    â†’ 1-3-3 Bit score (Layer 2)
    â†’ GodsEye Harmonic Probe (Layer 3)
    â†’ Combined score â†’ priority band

  JET ENGINE (MiningJetEngine):
    â†’ INTAKE FAN: nonce bands as job stream
    â†’ COMPRESSOR: ThreadPoolExecutor (cpu_count Ã— 2 workers)
    â†’ COMBUSTION: double_sha256 per batch
    â†’ TURBINE: 4+ zeros â†’ re-queue surrounding with anchor step
    â†’ EXHAUST: yield improvements instantly

  AMPLIFIER (background thread on hits â‰¥ 6 zeros):
    â†’ IntelligenceAmplifier updates polar ring trust weights

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
from MiningJetEngine import MiningJetEngine

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
    {"bit": 146, "signal": 1.140, "operator": +1, "axis": "X", "name": "BIT146"},
    {"bit":  53, "signal": 1.024, "operator": +1, "axis": "Y", "name": "BIT53"},
    {"bit":  74, "signal": 0.998, "operator": -1, "axis": "Z", "name": "BIT74"},
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
    return sovereign_observed_score(signals, GODSEYE_ANCHOR), fp, bv

# â”€â”€ GodsEye Harmonic Probe â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
HARMONIC_STEP = int(NONCE_MAX / (GODSEYE_ANCHOR * 1000))
PROBE_COUNT   = 512

def double_sha256(data):
    return hashlib.sha256(hashlib.sha256(data).digest()).hexdigest()

def probe_landscape(header80):
    band_scores = [0] * THREAD_COUNT
    for i in range(PROBE_COUNT):
        nonce  = (i * HARMONIC_STEP) % NONCE_MAX
        digest = double_sha256(header80 + struct.pack("<I", nonce))
        z      = sum(1 for c in digest if c == "0") if digest[0] == "0" else 0
        band   = min(THREAD_COUNT - 1, int(nonce / NONCE_MAX * THREAD_COUNT))
        band_scores[band] += z
    top_band   = band_scores.index(max(band_scores))
    return (top_band + 0.5) / THREAD_COUNT, top_band, band_scores

# â”€â”€ Combined Score â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def combined_score(p, b, g):
    with engine_lock:
        wp = engine_weights["polar"]
        wb = engine_weights["bit"]
    wg = 1.0
    return (p * wp + b * wb + g * wg) / (wp + wb + wg)

# â”€â”€ Band setup â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
BAND_SIZE  = NONCE_MAX // THREAD_COUNT
BANDS      = [(i * BAND_SIZE, (i+1) * BAND_SIZE - 1 if i < THREAD_COUNT - 1 else NONCE_MAX)
              for i in range(THREAD_COUNT)]

# â”€â”€ Amplifier â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def amplify(ring_vals, nonce, zeros):
    if not AMP_AVAILABLE:
        return
    eq_val = ring_vals[5]
    query  = (
        f"Bitcoin {zeros} zeros. Nonce: {nonce}. "
        f"Rings: " + ", ".join(f"R{r}={ring_vals[r]:.4f}" for r in range(9)) +
        f". Equator R5={eq_val:.4f}. Adjust ring 4, 6, 7 trust?"
    )
    try:
        result = amp.amplify_thought(query)
        print(f"\n  [AMPLIFIER] {result[:300]}\n", flush=True)
        nudge = 0.05 if zeros >= 10 else 0.02
        with weights_lock:
            for ring in ["4", "6"]:
                r = int(ring)
                if ring_vals[r] - eq_val > 0.01:
                    trust_weights[ring]["trust"] = min(1.0, trust_weights[ring]["trust"] + nudge)
                else:
                    trust_weights[ring]["trust"] = max(0.1, trust_weights[ring]["trust"] - nudge)
            if ring_vals[7] - eq_val > 0.005:
                trust_weights["7"]["trust"] = min(1.0, trust_weights["7"]["trust"] + nudge)
            save_weights(trust_weights)
    except Exception as e:
        print(f"  [AMPLIFIER] {e}", flush=True)

# â”€â”€ Shared State â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
running      = True
current_job  = None
job_header80 = b""
combined_scr = 0.5
polar_scr    = 0.5
bit_scr      = 0.5
godeye_scr   = 0.5
pband        = 16
session_best = {"zeros": 0, "hash": "", "nonce": 0, "band": 0, "turbine": 0}
state_lock   = threading.Lock()
job_lock     = threading.Lock()

# â”€â”€ Jet Mining Loop â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def jet_mining_loop():
    """
    Runs the MiningJetEngine for each job.
    JetEngine handles all threading internally via ThreadPoolExecutor.
    Turbine feedback activates on 4+ zeros.
    """
    last_job = None
    jet      = MiningJetEngine()

    while running:
        with job_lock:
            job     = current_job
            header  = job_header80
            pb      = pband

        if job is None or job == last_job:
            time.sleep(0.05)
            continue

        last_job = job
        # Known hot zones from live observation: B14=6zeros, B25=7zeros
        HOT_ZONES = [(14, 6), (25, 7)]

        print(f"  [JET] Igniting on job {job} | priority band {pb} | "
              f"{jet.max_workers} workers | seeding B14+B25", flush=True)

        for band_id, zeros, nonce, digest in jet.stream_ignition(
                header, BANDS, BATCH_SIZE, pb, seed_zones=HOT_ZONES):

            # Stop if job changed
            with job_lock:
                if current_job != job:
                    break

            with state_lock:
                if zeros > session_best["zeros"]:
                    session_best["zeros"]   = zeros
                    session_best["hash"]    = digest
                    session_best["nonce"]   = nonce
                    session_best["band"]    = band_id
                    session_best["turbine"] = jet.turbine_hits

                    pct  = nonce / NONCE_MAX * 100
                    bar  = "#" * zeros + "-" * (TARGET_ZEROS - zeros)
                    turb = " â˜…TURBINE" if jet.turbine_hits > 0 else ""
                    print(
                        f"\n  *** JET HIT [{zeros}/{TARGET_ZEROS}]{turb} ***\n"
                        f"  BAND     : {band_id:>2}/31 ({pct:.1f}%)\n"
                        f"  P:{polar_scr:.3f} B:{bit_scr:.3f} G:{godeye_scr:.3f} C:{combined_scr:.3f}\n"
                        f"  NONCE    : {nonce:,}\n"
                        f"  HASH     : {digest}\n"
                        f"  COMPASS  : [{bar}] {zeros/TARGET_ZEROS*100:.1f}%\n",
                        flush=True
                    )
                    if zeros >= 6:
                        vec = expand_81d(header[:76])
                        rv  = [ring_avg(vec, r) for r in range(9)]
                        threading.Thread(target=amplify,
                                         args=(rv, nonce, zeros),
                                         daemon=True).start()

        b = jet.brief()
        print(f"  [JET] Job {job} complete | "
              f"Combusted: {b['total_combusted']:,} | "
              f"Turbine hits: {b['turbine_hits']}",
              flush=True)

# â”€â”€ Stratum Bridge â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def stratum_loop(sock):
    global current_job, job_header80, combined_scr
    global polar_scr, bit_scr, godeye_scr, pband
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

                        vec   = expand_81d(h76)
                        p_scr = compute_polar_score(vec)
                        rv    = [ring_avg(vec, r) for r in range(9)]
                        b_scr, fp, bv = compute_bit_score(h76)
                        ge_scr, ge_band, bscores = probe_landscape(header80)
                        c_scr = combined_score(p_scr, b_scr, ge_scr)
                        pb_   = int(c_scr * (THREAD_COUNT - 1))

                        with job_lock:
                            polar_scr    = p_scr
                            bit_scr      = b_scr
                            godeye_scr   = ge_scr
                            combined_scr = c_scr
                            pband        = pb_
                            job_header80 = header80
                            current_job  = job_id

                        pole = "â–²N" if c_scr > 0.55 else "â–¼S" if c_scr < 0.45 else "â—EQ"
                        print(f"\n{'='*72}", flush=True)
                        print(f"  [NEW JOB]    {job_id}", flush=True)
                        print(f"  [POLAR]      {p_scr:.4f}", flush=True)
                        for ax in AXES:
                            state = "SET" if bv[ax["bit"]] else "CLR"
                            contrib = (bv[ax["bit"]] - 0.5) * ax["signal"] * ax["operator"]
                            print(f"  [{ax['name']:<7}]  {ax['axis']}: {state} | {contrib:+.3f}", flush=True)
                        print(f"  [BIT]        {b_scr:.4f}", flush=True)
                        print(f"  [GODEYE]     {ge_scr:.4f} | hot band {ge_band}", flush=True)
                        print(f"  [COMBINED]   {c_scr:.4f} | {pole} | PRIORITY â†’ Band {pb_}", flush=True)
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
    workers = os.cpu_count() * 2
    print(f"[!] GODSEYE 29.0 â€” SOVEREIGN JET ENGINE", flush=True)
    print(f"    Layers    : Polar + 1-3-3 Bit + GodsEye Probe")
    print(f"    Jet       : MiningJetEngine | {workers} workers")
    print(f"    Turbine   : fires on 4+ zeros â†’ re-queues surrounding nonces")
    print(f"    Amplifier : {'ONLINE' if AMP_AVAILABLE else 'OFFLINE'}\n")

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
    threading.Thread(target=jet_mining_loop, daemon=True).start()

    try:
        while True:
            time.sleep(2.0)
            with state_lock:
                z    = session_best["zeros"]
                band = session_best["band"]
                turb = session_best["turbine"]
            bar  = "#" * z + "-" * (TARGET_ZEROS - z)
            pole = "N" if combined_scr > 0.55 else "S" if combined_scr < 0.45 else "EQ"
            print(
                f"  BEST:{z}/{TARGET_ZEROS} [B{band:02d}] | "
                f"P:{polar_scr:.3f} B:{bit_scr:.3f} G:{godeye_scr:.3f} "
                f"C:{combined_scr:.3f}{pole} | T:{turb} | [{bar}]",
                flush=True
            )
    except KeyboardInterrupt:
        running = False
        print("\n[!] SOVEREIGN JET ENGINE SHUTDOWN.", flush=True)

if __name__ == "__main__":
    ignite()
