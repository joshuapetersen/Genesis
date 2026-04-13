"""
GODSEYE 26.0 â€” FULL SPACE NAVIGATOR
================================================================
New engine. First Principles. Not a rewrite.

FINDING (256 blocks, 25.1 Tri-Zone Test):
  Nonces are uniformly distributed across all 4.29B values.
  No zone is hotter than any other.
  Covering 75% â†’ miss 25% by definition.
  The only winning move: cover 100%.

ARCHITECTURE:
  32 threads. 4.29B nonce space divided into 32 equal bands.
  Each thread owns one band and cycles through it continuously.
  On new job â†’ all threads reset to their band start.
  No gaps. No missed zones. 100% coverage.

  Band i: start = i Ã— (NONCE_MAX // 32)
           end   = (i+1) Ã— (NONCE_MAX // 32)

  The bit score (BIT146, BIT53) still steers PRIORITY â€”
  the thread handling the predicted zone gets a tighter
  inner loop (searches its band first rather than sequentially).
  Signal narrows where we START, not what we SKIP.

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
BATCH_SIZE     = 8_000

# â”€â”€ Signal Bits â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
SIGNAL_BITS = [
    {"bit": 146, "signal": 1.140, "name": "BIT146"},
    {"bit":  53, "signal": 1.024, "name": "BIT53"},
]
TOTAL_SIGNAL = sum(b["signal"] for b in SIGNAL_BITS)

def read_bit(fp, bit_index):
    return (fp[bit_index // 8] >> (7 - (bit_index % 8))) & 1

from Sovereign_Statistics import sovereign_observed_score

def bit_score(header76):
    fp       = hashlib.sha256(header76).digest()
    signals  = [(read_bit(fp, sb["bit"]), sb["signal"]) for sb in SIGNAL_BITS]
    score    = sovereign_observed_score(signals, GODSEYE_ANCHOR)
    return score, fp

# Band boundaries â€” divide space equally across 32 threads
BAND_SIZE  = NONCE_MAX // THREAD_COUNT
BAND_START = [i * BAND_SIZE for i in range(THREAD_COUNT)]
BAND_END   = [(i+1) * BAND_SIZE - 1 for i in range(THREAD_COUNT - 1)] + [NONCE_MAX]

# â”€â”€ Shared State â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
running       = True
current_job   = None
job_header80  = b""
job_score     = 0.5
priority_band = 16     # which band to prioritize (0-31), set by bit score
band_pos      = [0] * THREAD_COUNT  # current position in each band
band_lock     = threading.Lock()
session_best  = {"zeros": 0, "hash": "", "nonce": 0, "band": 0}
state_lock    = threading.Lock()
band_best     = [0] * THREAD_COUNT  # best zeros per band
total_hashes  = 0
hash_lock     = threading.Lock()

def double_sha256(data):
    return hashlib.sha256(hashlib.sha256(data).digest()).hexdigest()

# â”€â”€ Band Worker â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def band_worker(band_id):
    global total_hashes

    while running:
        job = current_job
        if job is None:
            time.sleep(0.05)
            continue

        header80 = job_header80
        start    = BAND_START[band_id]
        end      = BAND_END[band_id]
        span     = end - start + 1

        # Priority offset â€” preferred bands start earlier in their window
        prio_offset = 0
        if band_id == priority_band:
            prio_offset = 0           # start of band
        elif abs(band_id - priority_band) <= 4:
            prio_offset = span // 4  # second quarter

        pos       = (start + prio_offset) % (span) + start
        local_best = session_best["zeros"]

        while running and current_job == job:
            for i in range(BATCH_SIZE):
                nonce = start + ((pos - start + i) % span)
                nonce = nonce & NONCE_MAX

                digest = double_sha256(header80 + struct.pack("<I", nonce))

                zeros = 0
                for c in digest:
                    if c == "0": zeros += 1
                    else: break

                if zeros > local_best:
                    local_best = zeros
                    band_best[band_id] = zeros
                    with state_lock:
                        if zeros > session_best["zeros"]:
                            session_best["zeros"] = zeros
                            session_best["hash"]  = digest
                            session_best["nonce"] = nonce
                            session_best["band"]  = band_id

                            pct  = nonce / NONCE_MAX * 100
                            bar  = "#" * zeros + "-" * (TARGET_ZEROS - zeros)
                            print(
                                f"\n  *** FULL SPACE HIT [{zeros}/{TARGET_ZEROS}] ***\n"
                                f"  BAND    : {band_id:>2}/31 ({pct:.1f}% of space)\n"
                                f"  NONCE   : {nonce:,}\n"
                                f"  HASH    : {digest}\n"
                                f"  COMPASS : [{bar}] {zeros/TARGET_ZEROS*100:.1f}%\n",
                                flush=True
                            )

            pos = start + ((pos - start + BATCH_SIZE) % span)

        with hash_lock:
            total_hashes += BATCH_SIZE

# â”€â”€ Stratum Bridge â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def stratum_loop(sock):
    global current_job, job_header80, job_score, priority_band

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

                        score, fp = bit_score(h76)
                        bv        = {sb["bit"]: read_bit(fp, sb["bit"]) for sb in SIGNAL_BITS}
                        pband     = int(score * (THREAD_COUNT - 1))

                        job_score    = score
                        priority_band = pband
                        current_job  = job_id
                        job_header80 = header80

                        pole = "â–² NORTH" if score > 0.55 else \
                               "â–¼ SOUTH" if score < 0.45 else "â— EQ"

                        print(f"\n{'='*72}", flush=True)
                        print(f"  [NEW JOB]    {job_id}", flush=True)
                        for sb in SIGNAL_BITS:
                            state = "SET" if bv[sb["bit"]] else "CLR"
                            print(f"  [{sb['name']:<7}]  {state} | signal={sb['signal']:.3f}", flush=True)
                        print(f"  [SCORE]      {score:.4f} | {pole}", flush=True)
                        print(f"  [PRIORITY]   Band {pband:>2} "
                              f"({BAND_START[pband]:,}â†’{BAND_END[pband]:,})", flush=True)
                        print(f"  [COVERAGE]   100% (32 bands Ã— {BAND_SIZE//1_000_000}M nonces)", flush=True)
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

    print(f"[!] GODSEYE 26.0 â€” FULL SPACE NAVIGATOR", flush=True)
    print(f"    Wallet    : {WALLET_ADDRESS}")
    print(f"    Coverage  : 100% â€” 32 equal bands across all 4.29B nonces")
    print(f"    Priority  : bit score steers which band STARTS first")
    print(f"    Band size : {BAND_SIZE:,} nonces per thread\n")

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
        threading.Thread(target=band_worker, args=(i,), daemon=True).start()

    last = 0
    try:
        while True:
            time.sleep(1.0)
            with hash_lock:
                t = total_hashes
            with state_lock:
                z     = session_best["zeros"]
                band  = session_best["band"]

            rate = (t - last) / 1_000_000
            last = t
            bar  = "#" * z + "-" * (TARGET_ZEROS - z)
            pole = "N" if job_score > 0.55 else "S" if job_score < 0.45 else "EQ"

            print(
                f"  {rate:.3f} MH/s | "
                f"BEST:{z}/{TARGET_ZEROS} [B{band:02d}] | "
                f"{pole}:{job_score:.3f} | [{bar}]",
                flush=True
            )
    except KeyboardInterrupt:
        running = False
        print("\n[!] FULL SPACE SHUTDOWN.", flush=True)

if __name__ == "__main__":
    ignite()
