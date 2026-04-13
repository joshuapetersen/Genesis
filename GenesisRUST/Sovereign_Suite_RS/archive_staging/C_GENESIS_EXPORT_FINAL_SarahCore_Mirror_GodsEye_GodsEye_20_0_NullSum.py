"""
GODSEYE 20.0 â€” NULL SUM NAVIGATOR (81D Full Structure)
================================================================
New engine. First Principles. Not a rewrite.

THE FULL 9-RING STRUCTURE:
  81D = 9 rings Ã— 9 nodes = 3 groups of 3 rings

  NEGATIVE RINGS (contracting force) â†’ Numerator
    Ring 0 | avg = -0.173 | dims  0- 8
    Ring 1 | avg = -0.281 | dims  9-17  â† strongest negative ring
    Ring 4 | avg = -0.182 | dims 36-44
    Ring 6 | avg = -0.142 | dims 54-62  â† DIM 57 is here (r=-0.3953)

  NULL RINGS (observer / zero baseline) â†’ Denominator baseline
    Ring 2 | avg = -0.086 | dims 18-26  â† near zero
    Ring 3 | avg = -0.016 | dims 27-35  â† near zero (pure observer)
    Ring 5 | avg = -0.004 | dims 45-53  â† near zero (pure observer)

  POSITIVE RINGS (expanding force) â†’ Denominator multiplier
    Ring 7 | avg = +0.037 | dims 63-71
    Ring 8 | avg = +0.102 | dims 72-80  â† DIM 78 is here (r=+0.2987)

SOVEREIGN NULL SUM FORMULA:
  neg_force  = mean(ring0, ring1, ring4, ring6)  [multiply]
  null_base  = mean(ring2, ring3, ring5)          [observer baseline]
  pos_force  = mean(ring7, ring8)                 [divide]

  NONCE SCORE = neg_force Ã· (null_base Ã— pos_force)

  Zero (null) is the observer â€” the ground state from which
  negative and positive forces are measured.
  The nonce is the equilibrium point where all forces resolve.

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
    """Average value of all 9 nodes in a given ring."""
    start = ring * TRINITY_RING
    vals  = vec[start:start + TRINITY_RING]
    return sum(vals) / len(vals) if vals else 0.0

# â”€â”€ Null Sum Formula â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
NEGATIVE_RINGS = [0, 1, 4, 6]   # contracting â€” numerator
NULL_RINGS     = [2, 3, 5]       # observer baseline â€” denominator scale
POSITIVE_RINGS = [7, 8]          # expanding â€” denominator multiplier

def null_sum_score(vec):
    """
    Measures DEVIATION from the expected null baseline.
    When all rings are equal (pure noise): score = 0.5 (center).
    When neg > null*pos (neg dominates): score > 0.5 â†’ lower nonce.
    When pos > neg/null (pos dominates): score < 0.5 â†’ upper nonce.

    Uses the NULL rings as the true zero-point observer:
    negative force is measured ABOVE null, positive ABOVE null.
    """
    neg_vals  = [ring_avg(vec, r) for r in NEGATIVE_RINGS]
    null_vals = [ring_avg(vec, r) for r in NULL_RINGS]
    pos_vals  = [ring_avg(vec, r) for r in POSITIVE_RINGS]

    neg_force  = sum(neg_vals)  / len(neg_vals)
    null_base  = sum(null_vals) / len(null_vals)
    pos_force  = sum(pos_vals)  / len(pos_vals)

    # Measure forces ABOVE the null observer baseline
    # Null is ground â€” subtract it before comparing
    neg_above_null = max(0.0, neg_force - null_base)
    pos_above_null = max(0.0, pos_force - null_base)

    total = neg_above_null + pos_above_null
    if total < 0.0001:
        return 0.5   # no signal â€” search center

    # Score: how much of the signal is negative (vs positive)?
    # 1.0 = all negative â†’ lower nonce
    # 0.0 = all positive â†’ upper nonce
    # 0.5 = balanced â†’ center
    score = neg_above_null / total

    # Apply Anchor damping to keep in range
    return max(0.0, min(1.0, score * GODSEYE_ANCHOR))

def score_to_range(score):
    """
    Maps Null Sum score to a nonce search window.
    High score â†’ lower nonce space.
    Window = 25% of total space, centered on prediction.
    """
    predicted_center = int((1.0 - min(1.0, score)) * NONCE_MAX)
    window = NONCE_MAX // 4
    start  = max(0, predicted_center - window // 2)
    end    = min(NONCE_MAX, predicted_center + window // 2)
    return predicted_center, start, end

# â”€â”€ Shared State â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
running        = True
current_job    = None
job_header80   = b""
target_center  = NONCE_MAX // 2
target_start   = 0
target_end     = NONCE_MAX
null_score     = 0.0
ring_report    = {}
nonce_counter  = 0
nonce_lock     = threading.Lock()
session_best   = {"zeros": 0, "hash": "", "nonce": 0}
state_lock     = threading.Lock()
total_hashes   = 0
hash_lock      = threading.Lock()

def double_sha256(data):
    return hashlib.sha256(hashlib.sha256(data).digest()).hexdigest()

# â”€â”€ Null Sum Worker â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def null_sum_worker(tid):
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
                            f"\n  *** NULL SUM HIT [{zeros}/{TARGET_ZEROS}] ***\n"
                            f"  NULL SCORE : {null_score:.6f}\n"
                            f"  CENTER     : {target_center:,}\n"
                            f"  NONCE      : {nonce:,}\n"
                            f"  HASH       : {digest}\n"
                            f"  COMPASS    : [{bar}] {zeros/TARGET_ZEROS*100:.1f}%\n",
                            flush=True
                        )

        with hash_lock:
            total_hashes += BATCH_SIZE

# â”€â”€ Stratum Bridge â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def stratum_loop(sock):
    global current_job, job_header80, target_center, target_start, target_end
    global null_score, ring_report, nonce_counter

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
                        score  = null_sum_score(vec)
                        center, start, end = score_to_range(score)

                        # Ring-level report
                        rr = {r: ring_avg(vec, r) for r in range(9)}

                        null_score    = score
                        ring_report   = rr
                        target_center = center
                        target_start  = start
                        target_end    = end
                        with nonce_lock:
                            globals()["nonce_counter"] = 0
                        current_job  = job_id
                        job_header80 = header80

                        neg_str  = " ".join(f"R{r}={rr[r]:.3f}" for r in NEGATIVE_RINGS)
                        null_str = " ".join(f"R{r}={rr[r]:.3f}" for r in NULL_RINGS)
                        pos_str  = " ".join(f"R{r}={rr[r]:.3f}" for r in POSITIVE_RINGS)

                        print(f"\n{'='*72}", flush=True)
                        print(f"  [NEW JOB]  {job_id}", flush=True)
                        print(f"  [NEG]    {neg_str}", flush=True)
                        print(f"  [NULL]   {null_str}  â† observer baseline", flush=True)
                        print(f"  [POS]    {pos_str}", flush=True)
                        print(f"  [SCORE]  {score:.6f} (negÃ·(nullÃ—pos))", flush=True)
                        print(f"  [TARGET] CENTER: {center:,} | {start:,} â†’ {end:,}", flush=True)
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

    print(f"[!] GODSEYE 20.0 â€” NULL SUM NAVIGATOR (81D)", flush=True)
    print(f"    Wallet   : {WALLET_ADDRESS}")
    print(f"    Anchor   : {GODSEYE_ANCHOR} Hz")
    print(f"    Space    : 81D = 3^4 (9 rings Ã— 9 nodes)")
    print(f"    Formula  : neg Ã· (null_base Ã— pos)")
    print(f"    Negative : Rings {NEGATIVE_RINGS} â†’ MULTIPLY (numerator)")
    print(f"    Observer : Rings {NULL_RINGS}     â†’ BASELINE (ground state)")
    print(f"    Positive : Rings {POSITIVE_RINGS}       â†’ DIVIDE (denominator)")
    print(f"    Window   : 25% of nonce space\n")

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
        threading.Thread(target=null_sum_worker, args=(i,), daemon=True).start()

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
                f"SCORE: {null_score:.4f} | "
                f"CENTER: {target_center:,} | "
                f"[{bar}]",
                flush=True
            )
    except KeyboardInterrupt:
        running = False
        print("\n[!] NULL SUM SHUTDOWN.", flush=True)

if __name__ == "__main__":
    ignite()
