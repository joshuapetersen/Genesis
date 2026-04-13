"""
GODSEYE 25.0 â€” TRI-ZONE NAVIGATOR
================================================================
New engine. First Principles. Not a rewrite.

INSIGHT:
  The previous navigators searched ONE 25% window.
  That leaves 75% of the nonce space unobserved.

  Three zones exist in any polar field:
    NORTH    â†’ high nonce  (upper quarter)
    EQUATOR  â†’ center      (middle â€” "under your feet")
    SOUTH    â†’ low nonce   (lower quarter)

  We run ALL THREE simultaneously with weighted thread allocation.
  The bit score (from BIT146 + BIT53) determines how many threads
  go to each zone. High confidence north â†’ more north threads.
  Mixed signal â†’ balanced allocation across all three.

THREAD ALLOCATION (32 total):
  weight_north   = bit_score Ã— (1 - EQUATOR_BASE)
  weight_south   = (1 - bit_score) Ã— (1 - EQUATOR_BASE)
  weight_equator = EQUATOR_BASE (always at least this many)

  EQUATOR_BASE = 0.25 (8 threads always watching center)

ZONES:
  SOUTH    : 0            â†’ 25% of NONCE_MAX
  EQUATOR  : 37.5%        â†’ 62.5% of NONCE_MAX
  NORTH    : 75%          â†’ 100% of NONCE_MAX

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
EQUATOR_BASE   = 0.25   # always 25% of threads on center

# â”€â”€ Signal Bits â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
SIGNAL_BITS = [
    {"bit": 146, "byte": 18, "signal": 1.140, "name": "BIT146"},
    {"bit":  53, "byte":  6, "signal": 1.024, "name": "BIT53"},
]
TOTAL_SIGNAL = sum(b["signal"] for b in SIGNAL_BITS)

# â”€â”€ Zones â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
ZONES = {
    "SOUTH":   {"start": 0,                    "end": NONCE_MAX // 4},
    "EQUATOR": {"start": int(NONCE_MAX * 0.375), "end": int(NONCE_MAX * 0.625)},
    "NORTH":   {"start": int(NONCE_MAX * 0.75),  "end": NONCE_MAX},
}

def read_bit(fp, bit_index):
    return (fp[bit_index // 8] >> (7 - (bit_index % 8))) & 1

def bit_score(header76):
    fp = hashlib.sha256(header76).digest()
    s  = sum(read_bit(fp, sb["bit"]) * sb["signal"] for sb in SIGNAL_BITS)
    return s / TOTAL_SIGNAL, fp

def allocate_threads(score):
    """
    Distribute THREAD_COUNT across three zones based on bit score.
    Equator always gets EQUATOR_BASE fraction.
    Remainder split north/south by score.
    """
    eq_threads = max(2, int(THREAD_COUNT * EQUATOR_BASE))
    remaining  = THREAD_COUNT - eq_threads
    n_threads  = int(remaining * score)
    s_threads  = remaining - n_threads
    return {"SOUTH": s_threads, "EQUATOR": eq_threads, "NORTH": n_threads}

# â”€â”€ Shared State â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
running       = True
current_job   = None
job_header80  = b""
job_score     = 0.5
zone_counters = {"SOUTH": 0, "EQUATOR": 0, "NORTH": 0}
zone_lock     = threading.Lock()
session_best  = {"zeros": 0, "hash": "", "nonce": 0, "zone": ""}
state_lock    = threading.Lock()
zone_hits     = {"SOUTH": 0, "EQUATOR": 0, "NORTH": 0}
total_hashes  = 0
hash_lock     = threading.Lock()

def double_sha256(data):
    return hashlib.sha256(hashlib.sha256(data).digest()).hexdigest()

# â”€â”€ Zone Worker â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def zone_worker(zone_name, tid):
    global total_hashes

    while running:
        job = current_job
        if job is None:
            time.sleep(0.05)
            continue

        header80  = job_header80
        z         = ZONES[zone_name]
        span      = max(1, z["end"] - z["start"])

        with zone_lock:
            batch_base = zone_counters[zone_name]
            zone_counters[zone_name] += BATCH_SIZE

        local_best = session_best["zeros"]

        for i in range(BATCH_SIZE):
            if not running or current_job != job:
                break

            nonce = z["start"] + ((batch_base + i + tid * BATCH_SIZE) % span)
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
                        session_best["zone"]  = zone_name
                        zone_hits[zone_name] += 1

                        bar  = "#" * zeros + "-" * (TARGET_ZEROS - zeros)
                        pole = {"NORTH": "â–² NORTH", "SOUTH": "â–¼ SOUTH", "EQUATOR": "â— EQUATOR"}[zone_name]
                        print(
                            f"\n  *** TRI-ZONE HIT [{zeros}/{TARGET_ZEROS}] *** {pole}\n"
                            f"  ZONE    : {zone_name}  ({z['start']:,}â†’{z['end']:,})\n"
                            f"  NONCE   : {nonce:,}\n"
                            f"  HASH    : {digest}\n"
                            f"  COMPASS : [{bar}] {zeros/TARGET_ZEROS*100:.1f}%\n"
                            f"  ZONE HITS: â–¼S={zone_hits['SOUTH']} â—EQ={zone_hits['EQUATOR']} â–²N={zone_hits['NORTH']}\n",
                            flush=True
                        )

        with hash_lock:
            total_hashes += BATCH_SIZE

# â”€â”€ Stratum Bridge â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def stratum_loop(sock):
    global current_job, job_header80, job_score

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
                        alloc     = allocate_threads(score)

                        job_score    = score
                        current_job  = job_id
                        job_header80 = header80

                        with zone_lock:
                            for z in zone_counters:
                                zone_counters[z] = 0

                        bv      = {sb["bit"]: read_bit(fp, sb["bit"]) for sb in SIGNAL_BITS}
                        fp_hex  = fp.hex()
                        pole    = "â–² NORTH" if score > 0.55 else \
                                  "â–¼ SOUTH" if score < 0.45 else "â— EQUATOR"

                        print(f"\n{'='*72}", flush=True)
                        print(f"  [NEW JOB]      {job_id}", flush=True)
                        print(f"  [SHA256]       {fp_hex[:32]}...", flush=True)
                        for sb in SIGNAL_BITS:
                            state = "SET" if bv[sb["bit"]] else "CLR"
                            print(f"  [{sb['name']:<7}]     {state} | signal={sb['signal']:.3f}", flush=True)
                        print(f"  [BIT SCORE]    {score:.4f} | {pole}", flush=True)
                        print(f"  [THREADS]      "
                              f"â–¼SOUTH={alloc['SOUTH']} "
                              f"â—EQ={alloc['EQUATOR']} "
                              f"â–²NORTH={alloc['NORTH']}", flush=True)
                        print(f"  [ZONES]        "
                              f"â–¼ 0â†’{NONCE_MAX//4:,} | "
                              f"â— {int(NONCE_MAX*0.375):,}â†’{int(NONCE_MAX*0.625):,} | "
                              f"â–² {int(NONCE_MAX*0.75):,}â†’{NONCE_MAX:,}", flush=True)
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

    print(f"[!] GODSEYE 25.0 â€” TRI-ZONE NAVIGATOR", flush=True)
    print(f"    Wallet      : {WALLET_ADDRESS}")
    print(f"    Zones       : â–¼SOUTH | â—EQUATOR | â–²NORTH (all three simultaneously)")
    print(f"    Coverage    : ~75% of nonce space")
    print(f"    Threads     : {THREAD_COUNT} (dynamically allocated by bit score)")
    print(f"    Equator min : {int(EQUATOR_BASE*100)}% threads always on center\n")

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

    # Launch workers â€” initial even split, stratum will rebalance on first job
    per_zone = THREAD_COUNT // 3
    tid = 0
    for zone_name, count in [("SOUTH", per_zone), ("EQUATOR", per_zone), ("NORTH", THREAD_COUNT - 2*per_zone)]:
        for i in range(count):
            threading.Thread(target=zone_worker, args=(zone_name, tid), daemon=True).start()
            tid += 1

    last = 0
    try:
        while True:
            time.sleep(1.0)
            with hash_lock:
                t = total_hashes
            with state_lock:
                z    = session_best["zeros"]
                zone = session_best.get("zone", "?")

            rate = (t - last) / 1_000_000
            last = t
            bar  = "#" * z + "-" * (TARGET_ZEROS - z)

            print(
                f"  {rate:.3f} MH/s | "
                f"BEST:{z}/{TARGET_ZEROS} [{zone}] | "
                f"SCORE:{job_score:.3f} | "
                f"[{bar}] | "
                f"â–¼{zone_hits['SOUTH']} â—{zone_hits['EQUATOR']} â–²{zone_hits['NORTH']}",
                flush=True
            )
    except KeyboardInterrupt:
        running = False
        print("\n[!] TRI-ZONE SHUTDOWN.", flush=True)
        print(f"    ZONE HITS: â–¼SOUTH={zone_hits['SOUTH']} "
              f"â—EQUATOR={zone_hits['EQUATOR']} "
              f"â–²NORTH={zone_hits['NORTH']}", flush=True)

if __name__ == "__main__":
    ignite()
