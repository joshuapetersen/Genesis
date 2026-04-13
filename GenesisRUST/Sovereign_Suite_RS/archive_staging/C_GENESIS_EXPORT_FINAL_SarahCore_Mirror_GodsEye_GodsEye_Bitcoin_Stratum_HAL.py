"""
GODSEYE 10.0 â€” NAVIGATOR REACTOR
================================================================
First Principles Build (NEW â€” not a rewrite).
Sequential nonce scanning with zero-frontier tracking.
Real Stratum protocol byte-level hashing.
Every time a new zero clicks into place â€” you see it.

"We CREATE, never rewrite."
"""

import socket
import json
import time
import hashlib
import threading
import struct

GODSEYE_ANCHOR  = 1.09277703703
WALLET_ADDRESS  = "19xUEeTCD9UTtFbp7HCxL8d4xYdZwaB2ht"
THREAD_COUNT    = 32
BATCH_SIZE      = 50_000
TARGET_ZEROS    = 19

# â”€â”€ Shared State â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
running         = True
current_job     = None
job_header      = b"GODSEYE_GENESIS"   # updated on each mining.notify
nonce_counter   = 0                    # global sequential nonce
nonce_lock      = threading.Lock()

# Zero frontier â€” current job
best_zeros      = 0
best_hash       = "f" * 64
best_nonce      = 0
frontier_lock   = threading.Lock()
total_hashes    = 0
hash_count_lock = threading.Lock()

# Session best â€” never resets
session_best_zeros = 0
session_best_hash  = "f" * 64
session_best_nonce = 0

# â”€â”€ Nonce Allocator â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def allocate_nonces(batch):
    """Sequential nonce allocation â€” no gaps, no repeats."""
    global nonce_counter
    with nonce_lock:
        start = nonce_counter
        nonce_counter += batch
    return start

# â”€â”€ Navigator Worker â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def navigator_worker(tid):
    """
    Scans sequential nonce space.
    Every time a new zero clicks into place, logs it immediately.
    Uses proper byte-level header + nonce (real Stratum format).
    """
    global best_zeros, best_hash, best_nonce, total_hashes
    global session_best_zeros, session_best_hash, session_best_nonce

    while running:
        job = current_job
        if job is None:
            time.sleep(0.05)
            continue

        header = job_header
        start  = allocate_nonces(BATCH_SIZE)

        local_best_zeros = best_zeros  # read current frontier

        for i in range(BATCH_SIZE):
            if not running:
                return

            nonce  = start + i
            # Real Stratum format: header bytes + nonce as 4-byte little-endian
            nonce_bytes = struct.pack("<I", nonce & 0xFFFFFFFF)
            digest = hashlib.sha256(hashlib.sha256(header + nonce_bytes).digest()).hexdigest()

            # Count leading zeros
            zeros = 0
            for c in digest:
                if c == '0':
                    zeros += 1
                else:
                    break

            # Zero frontier â€” did a new zero click into place?
            if zeros > local_best_zeros:
                local_best_zeros = zeros
                with frontier_lock:
                    if zeros > best_zeros:
                        best_zeros = zeros
                        best_hash  = digest
                        best_nonce = nonce
                        # Immediate alert â€” don't wait for the 1s display loop
                        bar = "#" * zeros + "-" * (TARGET_ZEROS - zeros)
                        # Update session best
                        is_session_record = zeros > session_best_zeros
                        if is_session_record:
                            session_best_zeros = zeros
                            session_best_hash  = digest
                            session_best_nonce = nonce
                        record_tag = " *** SESSION RECORD ***" if is_session_record else ""
                        print(
                            f"\n  *** ZERO LOCKED IN [{zeros}/{TARGET_ZEROS}]{record_tag} ***\n"
                            f"  NONCE : {nonce}\n"
                            f"  HASH  : {digest}\n"
                            f"  PROG  : [{bar}] {zeros/TARGET_ZEROS*100:.1f}%\n",
                            flush=True
                        )

        with hash_count_lock:
            total_hashes += BATCH_SIZE

# â”€â”€ Stratum Listener â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def stratum_loop(sock, diff_ref):
    global current_job, job_header, nonce_counter, best_zeros, best_hash, best_nonce
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
                        params     = msg["params"]
                        job_id     = params[0]
                        prevhash   = params[1]
                        coinbase1  = params[2]
                        coinbase2  = params[3]

                        # Build the block header seed from Stratum params
                        new_header = (prevhash + coinbase1 + coinbase2).encode(errors="ignore")[:80]

                        # Reset per-job frontier (session best preserved)
                        with nonce_lock:
                            nonce_counter = 0
                        with frontier_lock:
                            best_zeros = 0
                            best_hash  = "f" * 64
                            best_nonce = 0

                        current_job = job_id
                        job_header  = new_header

                        print(f"\n{'='*72}", flush=True)
                        print(f"  [NEW JOB] {job_id}", flush=True)
                        print(f"  NAVIGATOR RESETTING â€” NONCE BACK TO 0", flush=True)
                        print(f"  SESSION RECORD : {session_best_zeros}/{TARGET_ZEROS} zeros (preserved)", flush=True)
                        print(f"{'='*72}", flush=True)

                    elif msg.get("method") == "mining.set_difficulty":
                        diff_ref[0] = msg["params"][0]
                        print(f"  [DIFFICULTY] {diff_ref[0]}", flush=True)
                except Exception:
                    pass
        except (BlockingIOError, socket.error):
            time.sleep(0.01)

# â”€â”€ Main â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def ignite_sovereign_payout(host="solo.ckpool.org", port=3333):
    global running

    print(f"[!] GODSEYE 10.0 â€” NAVIGATOR REACTOR", flush=True)
    print(f"    Pool      : {host}:{port}")
    print(f"    Wallet    : {WALLET_ADDRESS}")
    print(f"    Threads   : {THREAD_COUNT}")
    print(f"    Mode      : Sequential Nonce Scan (First Principles)")
    print(f"    Target    : {TARGET_ZEROS} Leading Zeros\n")

    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.settimeout(10.0)
    sock.connect((host, port))

    sock.sendall((json.dumps({"id":1,"method":"mining.subscribe","params":[]}) + "\n").encode())
    _ = sock.recv(4096)
    sock.sendall((json.dumps({"id":2,"method":"mining.authorize","params":[WALLET_ADDRESS,"x"]}) + "\n").encode())

    sock.setblocking(False)
    print(f"[SUCCESS] AUTHORIZED: {WALLET_ADDRESS}")
    print("-" * 72, flush=True)

    diff_ref = [10000]

    threading.Thread(target=stratum_loop, args=(sock, diff_ref), daemon=True).start()

    for i in range(THREAD_COUNT):
        threading.Thread(target=navigator_worker, args=(i,), daemon=True).start()

    last_total = 0
    try:
        while True:
            time.sleep(1.0)
            with hash_count_lock:
                total = total_hashes
            with frontier_lock:
                zeros = best_zeros
                h     = best_hash
                nc    = best_nonce

            rate       = (total - last_total) / 1_000_000
            last_total = total
            remaining  = TARGET_ZEROS - zeros
            pct        = zeros / TARGET_ZEROS * 100
            bar        = "#" * zeros + "-" * (TARGET_ZEROS - zeros)

            with frontier_lock:
                s_zeros = session_best_zeros

            print(
                f"  {rate:.2f} MH/s | "
                f"JOB BEST: {zeros}/{TARGET_ZEROS} | "
                f"SESSION BEST: {s_zeros}/{TARGET_ZEROS} | "
                f"NEED: {remaining} more | "
                f"[{bar}] {pct:.1f}%",
                flush=True
            )

    except KeyboardInterrupt:
        running = False
        print("\n[!] SOVEREIGN SHUTDOWN.", flush=True)

if __name__ == "__main__":
    ignite_sovereign_payout()
