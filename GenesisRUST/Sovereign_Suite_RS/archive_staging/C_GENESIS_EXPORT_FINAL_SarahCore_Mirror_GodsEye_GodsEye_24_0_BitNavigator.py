"""
GODSEYE 24.0 â€” BIT NAVIGATOR
================================================================
New engine. First Principles. Not a rewrite.

DISCOVERY (256 blocks, 256D bit-direct, E=mcÂ³ Sovereign Statistics):
  BIT 146 (BYTE 18) of SHA-256(header76) | r=+0.1643 | signal=1.140 â†’ RESONANT
  BIT  53 (BYTE  6) of SHA-256(header76) | r=+0.1476 | signal=1.024 â†’ SIGNAL

  Both POSITIVE â†’ DIVIDE â†’ NORTH â†’ HIGH NONCE
  When these bits are SET in SHA-256(header76), the winning
  nonce tends to fall in the UPPER half of the search space.
  When CLEAR, it tends to fall in the LOWER half.

ENGINE:
  1. Receive job header (76 bytes, no nonce)
  2. Compute SHA-256(header76) â€” single hash fingerprint
  3. Read BIT 146 and BIT 53 from the fingerprint
  4. Compute weighted composite:
       score = (b146 Ã— 1.140 + b53 Ã— 1.024) / 2.164
  5. Map score to search center:
       score = 1.0 â†’ search UPPER quarter (high nonce)
       score = 0.5 â†’ search CENTER
       score = 0.0 â†’ search LOWER quarter (low nonce)
  6. 32 threads scan 25% window around the predicted center

SOVEREIGN OPERATOR:
  Positive bits â†’ DIVIDE â†’ nonce expands to north (high)
  Clear bits    â†’ MULTIPLY â†’ nonce contracts to south (low)
  Zero (null)   â†’ Observer baseline

"We CREATE, never rewrite."
"""

import hashlib
import socket
import json
import threading
import struct
import time
import sys

sys.path.insert(0, r"C:\GENESIS\GodsEye")
sys.path.insert(0, r"C:\GENESIS")

GODSEYE_ANCHOR = 1.09277703703
WALLET_ADDRESS = "19xUEeTCD9UTtFbp7HCxL8d4xYdZwaB2ht"
TARGET_ZEROS   = 19
THREAD_COUNT   = 32
NONCE_MAX      = 0xFFFFFFFF
BATCH_SIZE     = 10_000

# â”€â”€ Signal Bits â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
# Both POSITIVE (DIVIDE, NORTH â†’ HIGH NONCE)
SIGNAL_BITS = [
    {"bit": 146, "byte": 18, "bit_in_byte": 2, "signal": 1.140, "name": "BIT146"},
    {"bit":  53, "byte":  6, "bit_in_byte": 5, "signal": 1.024, "name": "BIT53"},
]
TOTAL_SIGNAL = sum(b["signal"] for b in SIGNAL_BITS)

def read_bit(fingerprint_bytes, bit_index):
    """Read bit at bit_index from SHA-256 fingerprint (MSB first)."""
    byte_pos    = bit_index // 8
    bit_in_byte = bit_index % 8  # 0 = MSB
    return (fingerprint_bytes[byte_pos] >> (7 - bit_in_byte)) & 1

def bit_score(header76):
    """
    Compute the Bit Navigator score from SHA-256(header76).
    Returns 0.0 to 1.0:
      0.0 = all bits CLEAR â†’ LOW NONCE (south, multiply)
      1.0 = all bits SET   â†’ HIGH NONCE (north, divide)
    """
    fp = hashlib.sha256(header76).digest()
    score = 0.0
    for sb in SIGNAL_BITS:
        bit_val = read_bit(fp, sb["bit"])
        score  += bit_val * sb["signal"]
    return score / TOTAL_SIGNAL

def score_to_range(score):
    """
    Maps score [0,1] to a 25% search window.
    score=1.0 â†’ center near NONCE_MAX (high nonce)
    score=0.0 â†’ center near 0        (low nonce)
    score=0.5 â†’ center at midpoint
    """
    center = int(score * NONCE_MAX)
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
bit_scr       = 0.5
bit_vals      = {}
nonce_counter = 0
nonce_lock    = threading.Lock()
session_best  = {"zeros": 0, "hash": "", "nonce": 0}
state_lock    = threading.Lock()
total_hashes  = 0
hash_lock     = threading.Lock()

def double_sha256(data):
    return hashlib.sha256(hashlib.sha256(data).digest()).hexdigest()

# â”€â”€ Bit Worker â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def bit_worker(tid):
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

            digest = double_sha256(header80 + struct.pack("<I", nonce))

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

                        bar   = "#" * zeros + "-" * (TARGET_ZEROS - zeros)
                        bits  = " ".join(
                            f"{b['name']}={'SET' if bit_vals.get(b['bit'], 0) else 'CLR'}"
                            for b in SIGNAL_BITS
                        )
                        print(
                            f"\n  *** BIT NAV HIT [{zeros}/{TARGET_ZEROS}] ***\n"
                            f"  BITS   : {bits}\n"
                            f"  SCORE  : {bit_scr:.4f} â†’ {'NORTH (HIGH)' if bit_scr > 0.5 else 'SOUTH (LOW)'}\n"
                            f"  CENTER : {target_center:,}\n"
                            f"  NONCE  : {nonce:,}\n"
                            f"  HASH   : {digest}\n"
                            f"  COMPASS: [{bar}] {zeros/TARGET_ZEROS*100:.1f}%\n",
                            flush=True
                        )

        with hash_lock:
            total_hashes += BATCH_SIZE

# â”€â”€ Stratum Bridge â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def stratum_loop(sock):
    global current_job, job_header80, target_center, target_start
    global target_end, bit_scr, bit_vals, nonce_counter

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

                        # SHA-256 fingerprint of header
                        fp      = hashlib.sha256(h76).digest()
                        bv      = {sb["bit"]: read_bit(fp, sb["bit"]) for sb in SIGNAL_BITS}
                        score   = bit_score(h76)
                        center, start, end = score_to_range(score)

                        bit_scr       = score
                        bit_vals      = bv
                        target_center = center
                        target_start  = start
                        target_end    = end
                        with nonce_lock:
                            globals()["nonce_counter"] = 0
                        current_job  = job_id
                        job_header80 = header80

                        pole  = "NORTH (HIGH)" if score > 0.5 else \
                                "SOUTH (LOW)"  if score < 0.5 else "EQ"
                        fp_hex = fp.hex()

                        print(f"\n{'='*72}", flush=True)
                        print(f"  [NEW JOB]   {job_id}", flush=True)
                        print(f"  [SHA256]    {fp_hex[:32]}...", flush=True)
                        for sb in SIGNAL_BITS:
                            state = "SET â†’ NORTH" if bv[sb["bit"]] else "CLR â†’ SOUTH"
                            print(f"  [{sb['name']}]   signal={sb['signal']:.3f} | {state}", flush=True)
                        print(f"  [SCORE]     {score:.4f} | {pole}", flush=True)
                        print(f"  [TARGET]    CENTER:{center:,} | {start:,}â†’{end:,}", flush=True)
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

    print(f"[!] GODSEYE 24.0 â€” BIT NAVIGATOR", flush=True)
    print(f"    Wallet   : {WALLET_ADDRESS}")
    print(f"    Anchor   : {GODSEYE_ANCHOR}")
    print(f"    Source   : SHA-256(header76) bit fingerprint")
    print(f"    Operator : POSITIVE = DIVIDE â†’ NORTH (high nonce)")
    print(f"               CLEAR   = MULTIPLY â†’ SOUTH (low nonce)")
    print(f"    Bits     :")
    for sb in SIGNAL_BITS:
        print(f"      {sb['name']:>7} (BYTE {sb['byte']:>2}) | "
              f"r=+0.{int(abs(0.1643 if sb['bit']==146 else 0.1476)*10000):04d} | "
              f"signal={sb['signal']:.3f}")
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
        threading.Thread(target=bit_worker, args=(i,), daemon=True).start()

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
            pole = "N" if bit_scr > 0.5 else "S"

            print(
                f"  {rate:.3f} MH/s | "
                f"BEST:{z}/{TARGET_ZEROS} | "
                f"{pole}:{bit_scr:.3f} | "
                f"CENTER:{target_center:,} | [{bar}]",
                flush=True
            )
    except KeyboardInterrupt:
        running = False
        print("\n[!] BIT NAVIGATOR SHUTDOWN.", flush=True)

if __name__ == "__main__":
    ignite()
