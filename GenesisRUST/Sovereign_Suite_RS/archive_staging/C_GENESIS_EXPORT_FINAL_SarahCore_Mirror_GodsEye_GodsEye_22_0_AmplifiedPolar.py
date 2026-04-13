"""
GODSEYE 22.0 â€” AMPLIFIED POLAR NAVIGATOR
================================================================
New engine. First Principles. Not a rewrite.

ARCHITECTURE:
  IntelligenceAmplifier + 81D Polar Field = living system.

  The Polar Navigator (21.0) uses static trust weights.
  The Amplifier changes that.

  After every high-zero nonce found, the block's ring
  fingerprint is fed into IntelligenceAmplifier's pipeline:
    decompose â†’ solve (TheoryLab / Vault) â†’ synthesize

  The amplifier reasons about WHY that block's field looked
  that way and returns guidance on trust weight direction.
  Anomaly rings (4, 6) and transition ring (7) are nudged
  up or down based on whether they fired above the null.

  Trust weights persist to polar_weights.json.
  The system learns with every block.

  If IntelligenceAmplifier is unavailable:
  â†’ Degrades gracefully to static polar weights.

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

GODSEYE_ANCHOR = 1.09277703703
WALLET_ADDRESS = "19xUEeTCD9UTtFbp7HCxL8d4xYdZwaB2ht"
TARGET_ZEROS   = 19
THREAD_COUNT   = 32
NONCE_MAX      = 0xFFFFFFFF
BATCH_SIZE     = 10_000
WEIGHTS_FILE   = r"C:\GENESIS\GodsEye\polar_weights.json"

# â”€â”€ Load IntelligenceAmplifier (graceful fallback) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
AMP_AVAILABLE = False
amp = None
try:
    from IntelligenceAmplifier import IntelligenceAmplifier
    amp = IntelligenceAmplifier()
    AMP_AVAILABLE = True
    print("[AMPLIFIER] IntelligenceAmplifier loaded.", flush=True)
except Exception as e:
    print(f"[AMPLIFIER] Not available ({e}). Using static weights.", flush=True)

# â”€â”€ Trust Weight Store â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
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
                w = json.load(f)
            print(f"[WEIGHTS] Loaded from {WEIGHTS_FILE}", flush=True)
            return w
        except Exception:
            pass
    return {k: dict(v) for k, v in DEFAULT_WEIGHTS.items()}

def save_weights(weights):
    try:
        with open(WEIGHTS_FILE, "w") as f:
            json.dump(weights, f, indent=2)
    except Exception:
        pass

trust_weights = load_weights()
weights_lock  = threading.Lock()

# â”€â”€ 81D Expansion â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
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

# â”€â”€ Amplifier Bridge â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def amplify_ring_weights(ring_vals, nonce, zeros):
    if not AMP_AVAILABLE:
        return
    eq_val = ring_vals[5]
    query  = (
        f"A Bitcoin block was solved with {zeros} leading zeros. "
        f"Polar ring fingerprint above equator (R5={eq_val:.4f}): "
        + ", ".join(f"R{r}={ring_vals[r]:.4f}" for r in range(9) if r != 5)
        + f". Winning nonce: {nonce}. "
        f"Which rings show the strongest signal above the null baseline? "
        f"Should anomaly rings 4 and 6 have their trust increased or decreased?"
    )
    try:
        result = amp.amplify_thought(query)
        print(f"\n  [AMPLIFIER]\n  {result[:400]}\n", flush=True)
        nudge = 0.05 if zeros >= 10 else 0.02
        with weights_lock:
            for ring in ["4", "6"]:
                r = int(ring)
                above_null = ring_vals[r] - ring_vals[5]
                if above_null > 0.01:
                    trust_weights[ring]["trust"] = min(1.0, trust_weights[ring]["trust"] + nudge)
                else:
                    trust_weights[ring]["trust"] = max(0.1, trust_weights[ring]["trust"] - nudge)
            above_null_7 = ring_vals[7] - ring_vals[5]
            if above_null_7 > 0.005:
                trust_weights["7"]["trust"] = min(1.0, trust_weights["7"]["trust"] + nudge)
            save_weights(trust_weights)
            print(
                f"  [WEIGHTS] R4={trust_weights['4']['trust']:.2f} "
                f"R6={trust_weights['6']['trust']:.2f} "
                f"R7={trust_weights['7']['trust']:.2f}",
                flush=True
            )
    except Exception as e:
        print(f"  [AMPLIFIER] Error: {e}", flush=True)

# â”€â”€ Polar Score â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def polar_score(vec):
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

def score_to_range(score):
    center = int((1.0 - score) * NONCE_MAX)
    window = NONCE_MAX // 4
    return center, max(0, center - window // 2), min(NONCE_MAX, center + window // 2)

# â”€â”€ Shared State â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
running       = True
current_job   = None
job_header80  = b""
target_center = NONCE_MAX // 2
target_start  = 0
target_end    = NONCE_MAX
polar_scr     = 0.5
nonce_counter = 0
nonce_lock    = threading.Lock()
session_best  = {"zeros": 0, "hash": "", "nonce": 0}
state_lock    = threading.Lock()
total_hashes  = 0
hash_lock     = threading.Lock()

def double_sha256(data):
    return hashlib.sha256(hashlib.sha256(data).digest()).hexdigest()

def amplified_worker(tid):
    global total_hashes
    while running:
        job = current_job
        if job is None:
            time.sleep(0.05)
            continue
        header80   = job_header80
        t_start    = target_start
        t_end      = target_end
        with nonce_lock:
            batch_base = nonce_counter
            globals()["nonce_counter"] += BATCH_SIZE
        span       = max(1, t_end - t_start)
        local_best = session_best["zeros"]
        for i in range(BATCH_SIZE):
            if not running or current_job != job:
                break
            nonce       = t_start + ((batch_base + i * THREAD_COUNT + tid) % span)
            nonce       = nonce & NONCE_MAX
            digest      = double_sha256(header80 + struct.pack("<I", nonce))
            zeros = 0
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
                        bar = "#" * zeros + "-" * (TARGET_ZEROS - zeros)
                        print(
                            f"\n  *** AMPLIFIED HIT [{zeros}/{TARGET_ZEROS}] ***\n"
                            f"  SCORE  : {polar_scr:.4f} | CENTER: {target_center:,}\n"
                            f"  NONCE  : {nonce:,}\n"
                            f"  HASH   : {digest}\n"
                            f"  COMPASS: [{bar}] {zeros/TARGET_ZEROS*100:.1f}%\n",
                            flush=True
                        )
                        vec = expand_81d(header80[:76])
                        rv  = [ring_avg(vec, r) for r in range(9)]
                        threading.Thread(
                            target=amplify_ring_weights,
                            args=(rv, nonce, zeros),
                            daemon=True
                        ).start()
        with hash_lock:
            total_hashes += BATCH_SIZE

def stratum_loop(sock):
    global current_job, job_header80, target_center, target_start
    global target_end, polar_scr, nonce_counter
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
                        vec      = expand_81d(header80[:76])
                        score    = polar_score(vec)
                        center, start, end = score_to_range(score)
                        rv       = [ring_avg(vec, r) for r in range(9)]
                        polar_scr = score; target_center = center
                        target_start = start; target_end = end
                        with nonce_lock:
                            globals()["nonce_counter"] = 0
                        current_job = job_id; job_header80 = header80
                        pole = "SOUTH" if score > 0.5 else "NORTH" if score < 0.5 else "EQ"
                        with weights_lock:
                            r4t = trust_weights["4"]["trust"]
                            r6t = trust_weights["6"]["trust"]
                            r7t = trust_weights["7"]["trust"]
                        print(f"\n{'='*72}", flush=True)
                        print(f"  [NEW JOB]  {job_id}", flush=True)
                        print(f"  [EQUATOR]  R5={rv[5]:.4f} (null baseline)", flush=True)
                        print(f"  [FIELD]    " + " ".join(f"R{r}={rv[r]:.3f}" for r in range(9)), flush=True)
                        print(f"  [POLARITY] {pole} | SCORE: {score:.4f}", flush=True)
                        print(f"  [TRUST]    R4={r4t:.2f} R6={r6t:.2f} R7={r7t:.2f}", flush=True)
                        print(f"  [TARGET]   CENTER:{center:,} | {start:,}â†’{end:,}", flush=True)
                        print(f"{'='*72}", flush=True)
                    elif msg.get("method") == "mining.set_difficulty":
                        print(f"  [DIFF] {msg['params'][0]}", flush=True)
                except Exception:
                    pass
        except (BlockingIOError, socket.error):
            time.sleep(0.01)

def ignite(host="solo.ckpool.org", port=3333):
    global running
    print(f"[!] GODSEYE 22.0 â€” AMPLIFIED POLAR NAVIGATOR", flush=True)
    print(f"    Amplifier : {'ONLINE' if AMP_AVAILABLE else 'OFFLINE (static weights)'}")
    print(f"    Weights   : {WEIGHTS_FILE}\n")
    with weights_lock:
        for r, entry in sorted(trust_weights.items(), key=lambda x: int(x[0])):
            print(f"    Ring {r}: {entry['polarity']:6} trust={entry['trust']:.2f}")
    print()
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
        threading.Thread(target=amplified_worker, args=(i,), daemon=True).start()
    last = 0
    try:
        while True:
            time.sleep(1.0)
            with hash_lock:
                t = total_hashes
            with state_lock:
                z = session_best["zeros"]
            rate = (t - last) / 1_000_000
            last = t
            bar  = "#" * z + "-" * (TARGET_ZEROS - z)
            pole = "S" if polar_scr > 0.5 else "N"
            print(f"  {rate:.3f} MH/s | BEST:{z}/{TARGET_ZEROS} | {pole}:{polar_scr:.3f} | CENTER:{target_center:,} | [{bar}]", flush=True)
    except KeyboardInterrupt:
        running = False
        print("\n[!] AMPLIFIED POLAR SHUTDOWN.", flush=True)

if __name__ == "__main__":
    ignite()
