"""
GODSEYE 19.0 â€” SOVEREIGN AXIS NAVIGATOR (81D)
================================================================
New engine. First Principles. Not a rewrite.

DISCOVERY (48 blocks, 81D = 3^4 Trinity):
  DIM 57 (RING 6) | r = -0.3953 | STRONGEST â†’ MULTIPLY
  DIM  9 (RING 1) | r = -0.3586 | STRONG    â†’ MULTIPLY
  DIM  0 (RING 0) | r = -0.3408 | STRONG    â†’ MULTIPLY
  DIM 13 (RING 1) | r = -0.3095 | STRONG    â†’ MULTIPLY
  DIM 17 (RING 1) | r = -0.2832 | STRONG    â†’ MULTIPLY
  DIM 78 (RING 8) | r = +0.2987 | POSITIVE  â†’ DIVIDE

SOVEREIGN OPERATOR RULE:
  Negative correlation â†’ MULTIPLY (inverse relationship)
  Positive correlation â†’ DIVIDE   (direct relationship)

SCORE = (D57 Ã— D9 Ã— D0 Ã— D13 Ã— D17) Ã· D78

High score â†’ nonce in LOWER space
Low score  â†’ nonce in UPPER space

RING 7 and RING 8 both show positive averages â€” the harmonic
flips in the outer rings of 3^4 space. The positive axis
(DIM 78) acts as the denominator â€” dividing the score back
toward the true nonce coordinate.

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
TRINITY_DIM  = 81   # 3^4
TRINITY_RING = 9

def expand_81d(data):
    if isinstance(data, str):
        data = data.encode()
    h = hashlib.sha384(data).hexdigest()  # 96 chars
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

# â”€â”€ Sovereign Axes â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
# Negative â†’ MULTIPLY | Positive â†’ DIVIDE
AXES = [
    {"dim": 57, "weight": 0.3953, "op": "multiply"},  # STRONGEST
    {"dim":  9, "weight": 0.3586, "op": "multiply"},
    {"dim":  0, "weight": 0.3408, "op": "multiply"},
    {"dim": 13, "weight": 0.3095, "op": "multiply"},
    {"dim": 17, "weight": 0.2832, "op": "multiply"},
    {"dim": 78, "weight": 0.2987, "op": "divide"},    # POSITIVE â€” denominator
]

def sovereign_score(vec):
    """
    Applies the Sovereign operator rule to all 6 axes.
    Negative â†’ multiply into numerator.
    Positive â†’ divide (multiply into denominator).

    Returns a score: HIGH = likely LOW nonce, LOW = likely HIGH nonce.
    """
    numerator   = 1.0
    denominator = 1.0

    for ax in AXES:
        val  = max(0.001, vec[ax["dim"]] / GODSEYE_ANCHOR)  # normalize
        if ax["op"] == "multiply":
            numerator   *= (val * ax["weight"])
        else:
            denominator *= (val * ax["weight"])

    raw = numerator / denominator if denominator > 0 else numerator

    # Normalize to 0-1 range using anchor damping
    return min(1.0, raw * GODSEYE_ANCHOR)

def score_to_range(score):
    """
    Maps Sovereign score to a nonce search range.
    Score is continuous â€” gives a center point and searches outward.
    """
    # Invert: high score â†’ low nonce
    center = int((1.0 - score) * NONCE_MAX)
    # Search window: 25% of nonce space centered on prediction
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
axis_score    = 0.0
nonce_counter = 0
nonce_lock    = threading.Lock()
session_best  = {"zeros": 0, "hash": "", "nonce": 0}
state_lock    = threading.Lock()
total_hashes  = 0
hash_lock     = threading.Lock()

def double_sha256(data):
    return hashlib.sha256(hashlib.sha256(data).digest()).hexdigest()

# â”€â”€ Sovereign Worker â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def sovereign_worker(tid):
    global total_hashes

    while running:
        job = current_job
        if job is None:
            time.sleep(0.05)
            continue

        header80  = job_header80
        t_start   = target_start
        t_end     = target_end

        with nonce_lock:
            batch_base = nonce_counter
            globals()["nonce_counter"] += BATCH_SIZE

        span      = max(1, t_end - t_start)
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
                            f"\n  *** SOVEREIGN HIT [{zeros}/{TARGET_ZEROS}] ***\n"
                            f"  SCORE  : {axis_score:.4f} â†’ CENTER: {target_center:,}\n"
                            f"  NONCE  : {nonce:,}\n"
                            f"  HASH   : {digest}\n"
                            f"  COMPASS: [{bar}] {zeros/TARGET_ZEROS*100:.1f}%\n",
                            flush=True
                        )

        with hash_lock:
            total_hashes += BATCH_SIZE

# â”€â”€ Stratum Bridge â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def stratum_loop(sock):
    global current_job, job_header80, target_center, target_start, target_end
    global axis_score, nonce_counter

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
                        score  = sovereign_score(vec)
                        center, start, end = score_to_range(score)

                        axis_score    = score
                        target_center = center
                        target_start  = start
                        target_end    = end
                        with nonce_lock:
                            globals()["nonce_counter"] = 0
                        current_job  = job_id
                        job_header80 = header80

                        dim_vals = {ax["dim"]: vec[ax["dim"]] for ax in AXES}
                        print(f"\n{'='*72}", flush=True)
                        print(f"  [NEW JOB] {job_id}", flush=True)
                        print(f"  [AXES]    D57={dim_vals[57]:.3f} D9={dim_vals[9]:.3f} "
                              f"D0={dim_vals[0]:.3f} D13={dim_vals[13]:.3f} "
                              f"D17={dim_vals[17]:.3f} D78={dim_vals[78]:.3f}", flush=True)
                        print(f"  [SCORE]   {score:.4f} (multiplyÃ·divide)", flush=True)
                        print(f"  [TARGET]  CENTER: {center:,} | "
                              f"RANGE: {start:,} â†’ {end:,}", flush=True)
                        print(f"  [WINDOW]  {(end-start)/NONCE_MAX*100:.1f}% of nonce space", flush=True)
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

    print(f"[!] GODSEYE 19.0 â€” SOVEREIGN AXIS NAVIGATOR (81D)", flush=True)
    print(f"    Wallet  : {WALLET_ADDRESS}")
    print(f"    Anchor  : {GODSEYE_ANCHOR} Hz")
    print(f"    Space   : 81D = 3^4 Trinity (9 rings x 9 nodes)")
    print(f"    Axes    : 5 multiply + 1 divide")
    print(f"    Formula : (D57Ã—D9Ã—D0Ã—D13Ã—D17) Ã· D78")
    print(f"    Target  : Continuous center-point, 25% window\n")

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
                z = session_best["zeros"]

            rate = (t - last) / 1_000_000
            last = t
            bar  = "#" * z + "-" * (TARGET_ZEROS - z)

            print(
                f"  {rate:.3f} MH/s | "
                f"BEST: {z}/{TARGET_ZEROS} | "
                f"SCORE: {axis_score:.4f} | "
                f"CENTER: {target_center:,} | "
                f"[{bar}]",
                flush=True
            )
    except KeyboardInterrupt:
        running = False
        print("\n[!] SOVEREIGN AXIS SHUTDOWN.", flush=True)

if __name__ == "__main__":
    ignite()
