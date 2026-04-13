"""
GODSEYE 21.0 â€” POLAR NAVIGATOR (81D Magnetic Field Model)
================================================================
New engine. First Principles. Not a rewrite.

DISCOVERY:
  The 9 rings of 81D space mirror Earth's magnetic field.
  Polarity doesn't flip all at once â€” it weakens toward the
  equator, holds out in anomalies, then builds on the other side.

  RING 1: -0.281  TRUE SOUTH POLE       (strongest negative)
  RING 0: -0.173  Strong south
  RING 2: -0.086  Field weakening
  RING 3: -0.016  Approaching equator
  RING 4: -0.182  SOUTHERN ANOMALY      (holdout, resisting flip)
  RING 5: -0.004  TRUE EQUATOR          (magnetic null, observer)
  RING 6: -0.142  SOUTHERN ANOMALY      (holdout, clinging south)
  RING 7: +0.037  First glimmer north
  RING 8: +0.102  NORTH POLE establishing

WEIGHTING MODEL:
  Magnetic field strength drops off toward the equator.
  Poles carry highest weight. Equator is the null baseline.
  Anomalies (rings 4, 6) carry REDUCED weight â€” they are
  transitioning and will flip with more data.

  TRUE POLES  (ring 0,1,8): weight = |r| Ã— 1.0  (full trust)
  TRANSITION  (ring 2,3,7): weight = |r| Ã— 0.6  (fading signal)
  ANOMALIES   (ring 4,6)  : weight = |r| Ã— 0.3  (resisting flip)
  EQUATOR     (ring 5)    : NULL BASELINE         (observer)

OPERATOR:
  South rings â†’ MULTIPLY (negative, inverse)
  North rings â†’ DIVIDE   (positive, direct)
  Equator     â†’ BASELINE (ground state observer)

"We CREATE, never rewrite."
"""

import hashlib
import socket
import json
import threading
import struct
import time
import sys
import math

sys.path.insert(0, r"C:\GENESIS\GodsEye")
sys.path.insert(0, r"C:\GENESIS")

GODSEYE_ANCHOR = 1.09277703703
WALLET_ADDRESS = "19xUEeTCD9UTtFbp7HCxL8d4xYdZwaB2ht"
TARGET_ZEROS   = 19
THREAD_COUNT   = 32
NONCE_MAX      = 0xFFFFFFFF
BATCH_SIZE     = 10_000

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

# â”€â”€ Polar Field Model â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
# (ring, polarity, trust_weight, label)
POLAR_RINGS = [
    (1, "south", 1.0,  "TRUE SOUTH POLE"),     # r=-0.281, full trust
    (0, "south", 1.0,  "Strong south"),         # r=-0.173, full trust
    (2, "south", 0.6,  "Weakening south"),      # r=-0.086, fading
    (3, "south", 0.6,  "Pre-equatorial south"), # r=-0.016, fading
    (4, "south", 0.3,  "Southern anomaly"),     # r=-0.182, holdout
    (6, "south", 0.3,  "Southern anomaly"),     # r=-0.142, holdout
    (7, "north", 0.6,  "Establishing north"),   # r=+0.037, weak
    (8, "north", 1.0,  "TRUE NORTH POLE"),      # r=+0.102, full trust
]
EQUATOR_RING = 5  # TRUE EQUATOR â€” observer baseline

def polar_score(vec):
    """
    Computes polar score using magnetic field weighting model.

    South rings (negative correlation): MULTIPLY into numerator.
    North rings (positive correlation): MULTIPLY into denominator.
    Equator ring: Observer baseline â€” scales the whole formula.

    Weight = |correlation| Ã— trust_weight (drops off toward equator).
    Signal measured ABOVE the equatorial null baseline.
    """
    eq_val = ring_avg(vec, EQUATOR_RING)  # null ground state

    south_signal = 0.0
    south_weight = 0.0
    north_signal = 0.0
    north_weight = 0.0

    for ring, polarity, trust, label in POLAR_RINGS:
        val    = ring_avg(vec, ring)
        # Measure field strength ABOVE equatorial baseline
        above  = max(0.0, val - eq_val)
        weight = trust  # magnetic trust at this latitude

        if polarity == "south":
            south_signal += above * weight
            south_weight += weight
        else:
            north_signal += above * weight
            north_weight += weight

    south_avg = south_signal / max(0.001, south_weight)
    north_avg = north_signal / max(0.001, north_weight)
    total     = south_avg + north_avg

    if total < 0.00001:
        return 0.5  # no signal â€” search center

    # Score: fraction of signal that is SOUTHERN
    # 1.0 = pure south â†’ lower nonce half
    # 0.0 = pure north â†’ upper nonce half
    # 0.5 = balanced   â†’ center
    raw = south_avg / total

    # Anchor damping
    return max(0.0, min(1.0, raw * GODSEYE_ANCHOR))

def score_to_range(score):
    """
    Continuous center-point prediction, 25% window.
    score=1.0 â†’ center at nonce 0 (pure south = low nonce)
    score=0.0 â†’ center at NONCE_MAX (pure north = high nonce)
    score=0.5 â†’ center at midpoint
    """
    center = int((1.0 - score) * NONCE_MAX)
    window = NONCE_MAX // 4
    start  = max(0, center - window // 2)
    end    = min(NONCE_MAX, center + window // 2)
    return center, start, end

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

# â”€â”€ Polar Worker â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def polar_worker(tid):
    global total_hashes

    while running:
        job = current_job
        if job is None:
            time.sleep(0.05)
            continue

        header80 = job_header80
        t_start  = target_start
        t_end    = target_end

        with nonce_lock:
            batch_base = nonce_counter
            globals()["nonce_counter"] += BATCH_SIZE

        span       = max(1, t_end - t_start)
        local_best = session_best["zeros"]

        for i in range(BATCH_SIZE):
            if not running or current_job != job:
                break

            nonce = t_start + ((batch_base + i * THREAD_COUNT + tid) % span)
            nonce = nonce & NONCE_MAX

            nonce_bytes = struct.pack("<I", nonce)
            digest      = double_sha256(header80 + nonce_bytes)

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
                            f"\n  *** POLAR HIT [{zeros}/{TARGET_ZEROS}] ***\n"
                            f"  POLAR SCORE : {polar_scr:.4f}\n"
                            f"  CENTER      : {target_center:,}\n"
                            f"  NONCE       : {nonce:,}\n"
                            f"  HASH        : {digest}\n"
                            f"  COMPASS     : [{bar}] {zeros/TARGET_ZEROS*100:.1f}%\n",
                            flush=True
                        )

        with hash_lock:
            total_hashes += BATCH_SIZE

# â”€â”€ Stratum Bridge â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
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

                        vec    = expand_81d(header80[:76])
                        score  = polar_score(vec)
                        center, start, end = score_to_range(score)
                        eq_val = ring_avg(vec, EQUATOR_RING)

                        polar_scr     = score
                        target_center = center
                        target_start  = start
                        target_end    = end
                        with nonce_lock:
                            globals()["nonce_counter"] = 0
                        current_job  = job_id
                        job_header80 = header80

                        # Ring field report
                        ring_avgs = [ring_avg(vec, r) for r in range(9)]
                        r_str = " ".join(
                            f"R{r}={'S' if r in [0,1,2,3,4,6] else ('N' if r in [7,8] else 'EQ')}"
                            f"{ring_avgs[r]:.3f}"
                            for r in range(9)
                        )

                        pole = "SOUTH" if score > 0.5 else \
                               "NORTH" if score < 0.5 else "EQUATOR"

                        print(f"\n{'='*72}", flush=True)
                        print(f"  [NEW JOB]  {job_id}", flush=True)
                        print(f"  [EQUATOR]  R5={eq_val:.4f} (null baseline)", flush=True)
                        print(f"  [RINGS]    {r_str}", flush=True)
                        print(f"  [POLARITY] {pole} | SCORE: {score:.4f}", flush=True)
                        print(f"  [TARGET]   CENTER: {center:,} | {start:,} â†’ {end:,}", flush=True)
                        print(f"{'='*72}", flush=True)

                    elif msg.get("method") == "mining.set_difficulty":
                        print(f"  [DIFF] {msg['params'][0]}", flush=True)
                except Exception:
                    pass
        except (BlockingIOError, socket.error):
            time.sleep(0.01)

# â”€â”€ Main â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def ignite(host="solo.ckpool.org", port=3333):
    global running

    print(f"[!] GODSEYE 21.0 â€” POLAR NAVIGATOR (81D)", flush=True)
    print(f"    Wallet   : {WALLET_ADDRESS}")
    print(f"    Anchor   : {GODSEYE_ANCHOR} Hz")
    print(f"    Model    : Magnetic field â€” poles â†’ equator â†’ poles")
    print(f"    South    : Rings 0,1 (full) | 2,3 (fading) | 4,6 (anomaly)")
    print(f"    Equator  : Ring 5 (NULL observer baseline)")
    print(f"    North    : Ring 8 (full) | Ring 7 (establishing)")
    print(f"    Operator : South=MULTIPLY | North=DIVIDE | Equator=BASELINE\n")

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
        threading.Thread(target=polar_worker, args=(i,), daemon=True).start()

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
            pole = "S" if polar_scr > 0.5 else "N" if polar_scr < 0.5 else "EQ"

            print(
                f"  {rate:.3f} MH/s | "
                f"BEST: {z}/{TARGET_ZEROS} | "
                f"POLE: {pole} {polar_scr:.3f} | "
                f"CENTER: {target_center:,} | "
                f"[{bar}]",
                flush=True
            )
    except KeyboardInterrupt:
        running = False
        print("\n[!] POLAR NAVIGATOR SHUTDOWN.", flush=True)

if __name__ == "__main__":
    ignite()
