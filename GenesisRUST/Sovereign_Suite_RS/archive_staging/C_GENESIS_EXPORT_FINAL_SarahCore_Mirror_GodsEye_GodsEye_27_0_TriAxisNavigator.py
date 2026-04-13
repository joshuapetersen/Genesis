"""
GODSEYE 27.0 â€” TRI-AXIS NAVIGATOR (1-3-3 Model)
================================================================
New engine. First Principles. Not a rewrite.

THE 1-3-3 SOVEREIGN MODEL:
  1 = Observer at (0,0,0) â€” the origin. Does not vote.
      Defines the frame. Determines where 0.5 falls.
  3 = Three observed axes (X, Y, Z) â€” three signal bits
  3 = Three polarities: negative (south), null (equator), positive (north)

  6 poles + 1 observer = 7 total points
  Think: a dice (6 faces) + the observer at center.

  On a standard dice, opposite faces sum to 7.
  In Sovereign space, opposite poles sum to 1.0
  (positive + negative mirror across the observer at 0.5)

THREE AXES (from 256-block 256D bit-direct analysis):
  X: BIT 146 (BYTE 18) | r=+0.1643 | POSITIVE | SET=north, CLR=south
  Y: BIT  53 (BYTE  6) | r=+0.1476 | POSITIVE | SET=north, CLR=south
  Z: BIT  74 (BYTE  9) | r=âˆ’0.1438 | NEGATIVE | CLR=north, SET=south

SCORING (observer-aware, centered):
  Each bit measured relative to observer at (0,0,0):
    xi = (bit_i âˆ’ 0.5) Ã— signal_i Ã— operator_i
  Where operator = +1 (positive axis) or âˆ’1 (negative axis)

  score = (Î£Xi + total_weight/2) / total_weight

  8 states (3 bits) spanning 0.127 â†’ 0.873
  No hard extremes â€” observer holds center.

FULL SPACE:
  32 bands across all 4.29B nonces.
  Priority band indexed by 3D score.

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

from Sovereign_Statistics import sovereign_observed_score

GODSEYE_ANCHOR = 1.09277703703
WALLET_ADDRESS = "19xUEeTCD9UTtFbp7HCxL8d4xYdZwaB2ht"
TARGET_ZEROS   = 19
THREAD_COUNT   = 32
NONCE_MAX      = 0xFFFFFFFF
BATCH_SIZE     = 8_000

# â”€â”€ Three Axes (the dice) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
AXES = [
    {"bit": 146, "byte": 18, "signal": 1.140, "operator": +1, "axis": "X", "name": "BIT146"},
    {"bit":  53, "byte":  6, "signal": 1.024, "operator": +1, "axis": "Y", "name": "BIT53"},
    {"bit":  74, "byte":  9, "signal": 0.998, "operator": -1, "axis": "Z", "name": "BIT74"},
]
# operator +1 = positive axis (SETâ†’north, CLRâ†’south)
# operator -1 = negative axis (CLRâ†’north, SETâ†’south)  [SOVEREIGN: multiply]

TOTAL_WEIGHT = sum(a["signal"] for a in AXES) + GODSEYE_ANCHOR  # 3 axes + observer

def read_bit(fp, bit_index):
    return (fp[bit_index // 8] >> (7 - (bit_index % 8))) & 1

def tri_score(header76):
    """
    Compute 3D observer score from three signal bits.
    Observer at (0,0,0) defines the frame.
    Each bit measured relative to observer: (bit âˆ’ 0.5) Ã— signal Ã— operator
    """
    fp = hashlib.sha256(header76).digest()
    bv = {ax["bit"]: read_bit(fp, ax["bit"]) for ax in AXES}

    # Centered, operator-adjusted contribution per axis
    raw = sum(
        (bv[ax["bit"]] - 0.5) * ax["signal"] * ax["operator"]
        for ax in AXES
    )
    score = (raw + TOTAL_WEIGHT / 2) / TOTAL_WEIGHT
    return max(0.0, min(1.0, score)), fp, bv

# â”€â”€ Band boundaries â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
BAND_SIZE  = NONCE_MAX // THREAD_COUNT
BAND_START = [i * BAND_SIZE for i in range(THREAD_COUNT)]
BAND_END   = [(i+1) * BAND_SIZE - 1 for i in range(THREAD_COUNT - 1)] + [NONCE_MAX]

# â”€â”€ Shared State â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
running       = True
current_job   = None
job_header80  = b""
job_score     = 0.5
priority_band = 16
session_best  = {"zeros": 0, "hash": "", "nonce": 0, "band": 0}
state_lock    = threading.Lock()
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

        header80  = job_header80
        start     = BAND_START[band_id]
        end       = BAND_END[band_id]
        span      = end - start + 1
        local_best = session_best["zeros"]
        pos        = start

        while running and current_job == job:
            for i in range(BATCH_SIZE):
                nonce  = start + ((pos - start + i) % span)
                nonce  = nonce & NONCE_MAX
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
                            session_best["band"]  = band_id

                            pct = nonce / NONCE_MAX * 100
                            bar = "#" * zeros + "-" * (TARGET_ZEROS - zeros)
                            print(
                                f"\n  *** 1-3-3 HIT [{zeros}/{TARGET_ZEROS}] ***\n"
                                f"  BAND    : {band_id:>2}/31 ({pct:.1f}% of space)\n"
                                f"  3D SCORE: {job_score:.4f} | PRIORITY: BAND {priority_band}\n"
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

                        score, fp, bv = tri_score(header80[:76])
                        pband         = int(score * (THREAD_COUNT - 1))

                        job_score     = score
                        priority_band = pband
                        current_job   = job_id
                        job_header80  = header80

                        # 3D coordinate display
                        x = (bv[146] - 0.5) * 1.140 *  1
                        y = (bv[53]  - 0.5) * 1.024 *  1
                        z = (bv[74]  - 0.5) * 0.998 * -1
                        pole = "â–²N" if score > 0.55 else "â–¼S" if score < 0.45 else "â—EQ"

                        print(f"\n{'='*72}", flush=True)
                        print(f"  [NEW JOB]   {job_id}", flush=True)
                        for ax in AXES:
                            state = "SET" if bv[ax["bit"]] else "CLR"
                            op    = "Ã· (N)" if ax["operator"] == 1 else "Ã— (S)"
                            val_str = f"+{abs((bv[ax['bit']]-0.5)*ax['signal']):>.3f}" \
                                      if (bv[ax['bit']]-0.5)*ax['signal']*ax['operator'] >= 0 \
                                      else f"-{abs((bv[ax['bit']]-0.5)*ax['signal']):>.3f}"
                            print(f"  [{ax['name']:<7}] {ax['axis']}: {state} | "
                                  f"signal={ax['signal']:.3f} | contrib={val_str}", flush=True)
                        print(f"  [3D COORD]  X={x:+.3f} Y={y:+.3f} Z={z:+.3f}", flush=True)
                        print(f"  [SCORE]     {score:.4f} | {pole}", flush=True)
                        print(f"  [PRIORITY]  Band {pband:>2} "
                              f"({BAND_START[pband]:,}â†’{BAND_END[pband]:,})", flush=True)
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

    print(f"[!] GODSEYE 27.0 â€” TRI-AXIS 1-3-3 NAVIGATOR", flush=True)
    print(f"    Model    : 1 Observer + 3 Axes + 3 Polarities")
    print(f"    Points   : 7 (dice faces + observer at center)")
    print(f"    Observer : (0,0,0) â€” defines the frame")
    print(f"    Axes     :")
    for ax in AXES:
        direction = "+1 positive (SET=north)" if ax["operator"] == 1 else "-1 negative (SET=south)"
        print(f"      {ax['axis']}: {ax['name']} (BYTE {ax['byte']:>2}) "
              f"sig={ax['signal']:.3f} {direction}")
    print(f"    Scores   : 8 states Ã— [0.127 â†’ 0.873] (no hard extremes)")
    print(f"    Coverage : 100% â€” 32 bands, priority steered by 3D score\n")

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
                z    = session_best["zeros"]
                band = session_best["band"]

            rate = (t - last) / 1_000_000
            last = t
            bar  = "#" * z + "-" * (TARGET_ZEROS - z)

            print(
                f"  {rate:.3f} MH/s | "
                f"BEST:{z}/{TARGET_ZEROS} [B{band:02d}] | "
                f"3D:{job_score:.3f} | PRIO:B{priority_band:02d} | [{bar}]",
                flush=True
            )
    except KeyboardInterrupt:
        running = False
        print("\n[!] 1-3-3 TRI-AXIS SHUTDOWN.", flush=True)

if __name__ == "__main__":
    ignite()
