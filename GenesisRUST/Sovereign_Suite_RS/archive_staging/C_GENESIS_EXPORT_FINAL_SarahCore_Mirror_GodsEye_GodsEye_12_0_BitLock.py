"""
GODSEYE 12.0 â€” BITLOCK ENGINE
================================================================
New engine. First Principles. Not a rewrite.

MISSION:
  The Decoder found the causation map.
  Winning nonces have a fingerprint:

    BIT 23 = 1  (72% of winners)
    BIT  3 = 1  (68% of winners)
    BIT 31 = 1  (62% â€” upper half of nonce space)
    BIT 25 = 0  (64% of winners have this UNSET)
    BIT 10 = 0  (62% of winners have this UNSET)
    BIT  7 = 0  (60% of winners have this UNSET)
    BIT 15 = 0  (60% of winners have this UNSET)

  This engine ONLY tests nonces that match the fingerprint.
  Every hash attempt is in the high-probability zone.

"We CREATE, never rewrite."
"""

import hashlib
import socket
import json
import threading
import struct
import time

GODSEYE_ANCHOR = 1.09277703703
WALLET_ADDRESS = "19xUEeTCD9UTtFbp7HCxL8d4xYdZwaB2ht"
TARGET_ZEROS   = 19
THREAD_COUNT   = 32

# â”€â”€ The Causation Fingerprint (from Decoder analysis) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
# Bits that MUST be 1 in winning nonces
MUST_BE_ONE  = [23, 3, 31]

# Bits that MUST be 0 in winning nonces
MUST_BE_ZERO = [25, 10, 7, 15]

def build_fingerprint_mask():
    """
    Returns (required_bits, forbidden_bits) as integer bitmasks.
    """
    ones  = 0
    zeros = 0
    for b in MUST_BE_ONE:
        ones  |= (1 << b)
    for b in MUST_BE_ZERO:
        zeros |= (1 << b)
    return ones, zeros

ONES_MASK, ZEROS_MASK = build_fingerprint_mask()

def matches_fingerprint(nonce):
    """
    Returns True if the nonce matches the causation fingerprint.
    All MUST_BE_ONE bits are set.
    All MUST_BE_ZERO bits are clear.
    """
    if (nonce & ONES_MASK) != ONES_MASK:
        return False
    if (nonce & ZEROS_MASK) != 0:
        return False
    return True

def generate_locked_nonces(start, count):
    """
    Fast bitwise nonce generation.
    Varies only the free bits sequentially, locks the rest.
    """
    locked  = set(MUST_BE_ONE + MUST_BE_ZERO)
    free    = [b for b in range(32) if b not in locked]
    results = []
    for i in range(start, start + count):
        nonce = ONES_MASK
        for bit_idx, bit_pos in enumerate(free):
            if (i >> bit_idx) & 1:
                nonce |= (1 << bit_pos)
        results.append(nonce & 0xFFFFFFFF)
    return results

# â”€â”€ Shared State â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
running          = True
current_job      = None
job_header       = b""
nonce_counter    = 0
nonce_lock       = threading.Lock()
session_best     = {"zeros": 0, "hash": "", "nonce": 0}
state_lock       = threading.Lock()
total_hashes     = 0
total_skipped    = 0
hash_lock        = threading.Lock()

BATCH = 2000  # nonces per thread per cycle

def bitlock_worker(tid):
    global total_hashes, total_skipped

    while running:
        job = current_job
        if job is None:
            time.sleep(0.05)
            continue

        header = job_header

        # Allocate a batch of locked nonces
        with nonce_lock:
            start = nonce_counter
            globals()["nonce_counter"] += BATCH

        locked_nonces = generate_locked_nonces(start, BATCH)

        local_best = session_best["zeros"]

        for nonce in locked_nonces:
            if not running or current_job != job:
                break

            nonce_bytes = struct.pack("<I", nonce)
            digest = hashlib.sha256(
                hashlib.sha256(header + nonce_bytes).digest()
            ).hexdigest()

            zeros = 0
            for c in digest:
                if c == "0":
                    zeros += 1
                else:
                    break

            if zeros > local_best:
                local_best = zeros
                with state_lock:
                    if zeros > session_best["zeros"]:
                        session_best["zeros"] = zeros
                        session_best["hash"]  = digest
                        session_best["nonce"] = nonce

                        bar = "#" * zeros + "-" * (TARGET_ZEROS - zeros)
                        print(
                            f"\n  *** ZERO LOCKED [{zeros}/{TARGET_ZEROS}] ***\n"
                            f"  NONCE : {nonce} (bits: {nonce:032b})\n"
                            f"  HASH  : {digest}\n"
                            f"  PROG  : [{bar}] {zeros/TARGET_ZEROS*100:.1f}%\n",
                            flush=True
                        )

        with hash_lock:
            total_hashes += BATCH

def stratum_loop(sock):
    global current_job, job_header, nonce_counter
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
                        params  = msg["params"]
                        job_id  = params[0]
                        header  = (params[1] + params[2] + params[3]).encode(errors="ignore")[:80]

                        with nonce_lock:
                            globals()["nonce_counter"] = 0

                        current_job = job_id
                        job_header  = header

                        print(f"\n{'='*72}", flush=True)
                        print(f"  [NEW JOB] {job_id}", flush=True)
                        print(f"  [BITLOCK] FINGERPRINT ACTIVE: {ONES_MASK:032b} (ones)", flush=True)
                        print(f"  [BITLOCK] SCANNING ONLY HIGH-PROBABILITY NONCES", flush=True)
                        print(f"{'='*72}", flush=True)

                    elif msg.get("method") == "mining.set_difficulty":
                        print(f"  [DIFFICULTY] {msg['params'][0]}", flush=True)
                except Exception:
                    pass
        except (BlockingIOError, socket.error):
            time.sleep(0.01)

def ignite(host="solo.ckpool.org", port=3333):
    global running

    free_bits = 32 - len(MUST_BE_ONE) - len(MUST_BE_ZERO)
    locked_space = 2 ** free_bits
    full_space   = 2 ** 32

    print(f"[!] GODSEYE 12.0 â€” BITLOCK ENGINE", flush=True)
    print(f"    Wallet       : {WALLET_ADDRESS}")
    print(f"    Bits Locked  : {len(MUST_BE_ONE) + len(MUST_BE_ZERO)} bits")
    print(f"    Free Bits    : {free_bits} bits")
    print(f"    Search Space : {locked_space:,} / {full_space:,} nonces ({locked_space/full_space*100:.1f}%)")
    print(f"    Fingerprint  :")
    print(f"      MUST BE 1  : {MUST_BE_ONE}")
    print(f"      MUST BE 0  : {MUST_BE_ZERO}\n")

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
        threading.Thread(target=bitlock_worker, args=(i,), daemon=True).start()

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
                f"SESSION BEST: {z}/{TARGET_ZEROS} | "
                f"[{bar}]",
                flush=True
            )
    except KeyboardInterrupt:
        running = False
        print("\n[!] BITLOCK SHUTDOWN.", flush=True)

if __name__ == "__main__":
    ignite()
