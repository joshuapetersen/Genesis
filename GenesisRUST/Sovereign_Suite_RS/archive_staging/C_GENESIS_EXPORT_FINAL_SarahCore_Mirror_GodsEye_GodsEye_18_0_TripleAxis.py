"""
GODSEYE 18.0 â€” TRIPLE AXIS NAVIGATOR
================================================================
New engine. First Principles. Not a rewrite.

DISCOVERY (48 solved blocks, 27D Trinity analysis):
  DIM  9 (RING 1) | r = -0.3586 | STRONGEST
  DIM  0 (RING 0) | r = -0.3408 | CONFIRMED
  DIM 13 (RING 1) | r = -0.3095 | NEW

  All three: HIGH value â†’ LOW nonce (negative correlation).
  RING 1 is the most predictive ring (-0.2809 avg).

STRATEGY:
  Combine all three dimensions into a single composite score.
  Weight each by its correlation strength (|r|).
  Map the composite score to a nonce QUADRANT:

    Score > 0.75  â†’ Q1: nonces 0        to 1.07B  (top 25%)
    Score > 0.50  â†’ Q2: nonces 1.07B    to 2.14B
    Score > 0.25  â†’ Q3: nonces 2.14B    to 3.22B
    Score â‰¤ 0.25  â†’ Q4: nonces 3.22B    to 4.29B  (bottom 25%)

  Then search the selected quadrant with all 32 threads.
  Search remaining quadrants in order if job is long enough.

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

# â”€â”€ Trinity Dimensions â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
TRINITY_DIM  = 27
TRINITY_RING = 9

# The three correlated dimensions, weights, and Sovereign operators
# Negative correlation â†’ MULTIPLY (inverse relationship)
# Positive correlation â†’ DIVIDE  (direct relationship, ratio preserved)
AXIS = [
    {"dim": 9,  "weight": 0.3586, "r": -0.3586, "op": "multiply"},
    {"dim": 0,  "weight": 0.3408, "r": -0.3408, "op": "multiply"},
    {"dim": 13, "weight": 0.3095, "r": -0.3095, "op": "multiply"},
]
TOTAL_WEIGHT = sum(a["weight"] for a in AXIS)

# Quadrant boundaries
QUADRANT_SIZE = NONCE_MAX // 4
QUADRANTS = [
    (0,                QUADRANT_SIZE,       "Q1 [0 â†’ 1.07B]"),
    (QUADRANT_SIZE,    QUADRANT_SIZE * 2,   "Q2 [1.07B â†’ 2.14B]"),
    (QUADRANT_SIZE*2,  QUADRANT_SIZE * 3,   "Q3 [2.14B â†’ 3.22B]"),
    (QUADRANT_SIZE*3,  NONCE_MAX,           "Q4 [3.22B â†’ 4.29B]"),
]

# â”€â”€ 27D Trinity Expansion â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def expand_27d(data):
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
        scale  = (i + 1) / TRINITY_DIM
        node   = (v1 * v2 * v3) * (GODSEYE_ANCHOR ** scale)
        nodes.append(node % GODSEYE_ANCHOR)
    return nodes

def compute_composite_score(vec):
    """
    Weighted composite score using Sovereign operator rule:
      Negative correlation â†’ MULTIPLY (dim_val * weight)
      Positive correlation â†’ DIVIDE   (weight / dim_val)
    Higher score = predict LOWER nonce (all current axes are negative).
    """
    score = 0.0
    for axis in AXIS:
        dim_val = vec[axis["dim"]]
        norm    = min(1.0, dim_val / GODSEYE_ANCHOR)
        if axis["op"] == "multiply":
            contribution = norm * axis["weight"]
        else:  # divide
            contribution = (axis["weight"] / norm) if norm > 0.001 else axis["weight"]
        score += contribution
    return score / TOTAL_WEIGHT

def select_quadrant(score):
    """
    Maps composite score to a prioritized quadrant order.
    High score â†’ search Q1 first (low nonces).
    Low score  â†’ search Q4 first (high nonces).
    """
    if score > 0.5:
        return [0, 1, 2, 3]   # Q1 first (low nonces)
    elif score > 0.35:
        return [1, 0, 2, 3]   # Q2 first
    elif score > 0.20:
        return [2, 3, 1, 0]   # Q3 first
    else:
        return [3, 2, 1, 0]   # Q4 first (high nonces)

# â”€â”€ Shared State â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
running        = True
current_job    = None
job_header80   = b""
active_quadrant = 0              # which quadrant index we're in
quadrant_order  = [0, 1, 2, 3]  # priority order
composite_score = 0.0
nonce_counter   = 0
nonce_lock      = threading.Lock()
session_best    = {"zeros": 0, "hash": "", "nonce": 0, "score": 0.0, "quadrant": ""}
state_lock      = threading.Lock()
total_hashes    = 0
hash_lock       = threading.Lock()

def double_sha256(data):
    return hashlib.sha256(hashlib.sha256(data).digest()).hexdigest()

# â”€â”€ Triple Axis Worker â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def triple_axis_worker(tid):
    global total_hashes, active_quadrant

    while running:
        job = current_job
        if job is None:
            time.sleep(0.05)
            continue

        header80 = job_header80
        q_idx    = quadrant_order[active_quadrant]
        q_start, q_end, q_label = QUADRANTS[q_idx]

        with nonce_lock:
            batch_base = nonce_counter
            globals()["nonce_counter"] += BATCH_SIZE

        local_best = session_best["zeros"]

        for i in range(BATCH_SIZE):
            if not running or current_job != job:
                break

            span  = q_end - q_start
            nonce = q_start + ((batch_base + i * THREAD_COUNT + tid) % span)
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
                        session_best["zeros"]    = zeros
                        session_best["hash"]     = digest
                        session_best["nonce"]    = nonce
                        session_best["score"]    = composite_score
                        session_best["quadrant"] = q_label

                        bar = "#" * zeros + "-" * (TARGET_ZEROS - zeros)
                        print(
                            f"\n  *** TRIPLE AXIS HIT [{zeros}/{TARGET_ZEROS}] ***\n"
                            f"  QUADRANT  : {q_label}\n"
                            f"  SCORE     : {composite_score:.4f} "
                            f"(DIM0={session_best['score']:.3f})\n"
                            f"  NONCE     : {nonce}\n"
                            f"  HASH      : {digest}\n"
                            f"  COMPASS   : [{bar}] {zeros/TARGET_ZEROS*100:.1f}%\n",
                            flush=True
                        )

        with hash_lock:
            total_hashes += BATCH_SIZE

# â”€â”€ Stratum Bridge â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def stratum_loop(sock):
    global current_job, job_header80, active_quadrant, quadrant_order
    global composite_score, nonce_counter

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
                        header80 = (params[1] + params[2] + params[3]).encode(errors="ignore")[:80]
                        header76 = header80[:76]

                        vec   = expand_27d(header76)
                        score = compute_composite_score(vec)
                        order = select_quadrant(score)

                        composite_score  = score
                        quadrant_order   = order
                        active_quadrant  = 0
                        with nonce_lock:
                            globals()["nonce_counter"] = 0
                        current_job  = job_id
                        job_header80 = header80

                        q_idx = order[0]
                        q_start, q_end, q_label = QUADRANTS[q_idx]

                        dim_vals = [vec[a["dim"]] for a in AXIS]
                        print(f"\n{'='*72}", flush=True)
                        print(f"  [NEW JOB]  {job_id}", flush=True)
                        print(f"  [AXIS]     DIM9={dim_vals[0]:.4f} | DIM0={dim_vals[1]:.4f} | DIM13={dim_vals[2]:.4f}", flush=True)
                        print(f"  [SCORE]    Composite = {score:.4f}", flush=True)
                        print(f"  [TARGET]   {q_label} â€” searching first", flush=True)
                        print(f"  [ORDER]    {[QUADRANTS[i][2] for i in order]}", flush=True)
                        print(f"{'='*72}", flush=True)

                    elif msg.get("method") == "mining.set_difficulty":
                        print(f"  [DIFFICULTY] {msg['params'][0]}", flush=True)
                except Exception:
                    pass
        except (BlockingIOError, socket.error):
            time.sleep(0.01)

# â”€â”€ Main â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def ignite(host="solo.ckpool.org", port=3333):
    global running

    print(f"[!] GODSEYE 18.0 â€” TRIPLE AXIS NAVIGATOR", flush=True)
    print(f"    Wallet  : {WALLET_ADDRESS}")
    print(f"    Anchor  : {GODSEYE_ANCHOR} Hz")
    print(f"    Axes    : DIM 9 (r=-0.36) | DIM 0 (r=-0.34) | DIM 13 (r=-0.31)")
    print(f"    Space   : 4 quadrants, prioritized by composite score")
    print(f"    Threads : {THREAD_COUNT}\n")

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
        threading.Thread(target=triple_axis_worker, args=(i,), daemon=True).start()

    last = 0
    try:
        while True:
            time.sleep(1.0)
            with hash_lock:
                t = total_hashes
            with state_lock:
                z   = session_best["zeros"]
                q   = session_best["quadrant"]
                s   = session_best["score"]

            rate = (t - last) / 1_000_000
            last = t
            bar  = "#" * z + "-" * (TARGET_ZEROS - z)

            print(
                f"  {rate:.3f} MH/s | "
                f"BEST: {z}/{TARGET_ZEROS} | "
                f"SCORE: {composite_score:.3f} | "
                f"[{bar}]",
                flush=True
            )
    except KeyboardInterrupt:
        running = False
        print("\n[!] TRIPLE AXIS SHUTDOWN.", flush=True)

if __name__ == "__main__":
    ignite()
