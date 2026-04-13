"""
GODSEYE 29.1 â€” SOVEREIGN JET ENGINE (FIXED)
================================================================
New engine. First Principles. Not a rewrite.

PREVIOUS ISSUE (29.0):
  concurrent.futures executor submitted ALL batches upfront.
  executor.shutdown(wait=True) blocked job changes indefinitely.
  No zeros ever yielded.

FIX:
  Use proven manual threading pattern from 27.0 (achieved 7 zeros).
  Add Turbine concept as a SHARED HOT-ZONE QUEUE that workers
  check before their next batch â€” the JetEngine idea, properly integrated.

ARCHITECTURE:
  32 band workers (manual threads, like 27.0 which worked)
  + Shared turbine_queue (deque) â€” when any worker finds 4+ zeros,
    it pushes (nonce, band_id) to turbine_queue
  + All workers poll turbine_queue between batches â€” if their band
    is hot, they spiral inward on the nearby nonces
  + Seed zones (B14, B25) are pre-loaded into turbine_queue at job start

SCORING STACK:
  Layer 1: Polar (81D ring field)
  Layer 2: 1-3-3 Bit (SHA-256 fingerprint, 3D observer)
  Layer 3: GodsEye Harmonic Probe

"We CREATE, never rewrite."
"""

import hashlib
import socket
import json
import threading
import struct
import time
import sys
import os
from collections import deque

sys.path.insert(0, r"C:\GENESIS\GodsEye")
sys.path.insert(0, r"C:\GENESIS")

from Sovereign_Statistics import sovereign_observed_score

GODSEYE_ANCHOR = 1.09277703703
WALLET_ADDRESS = "19xUEeTCD9UTtFbp7HCxL8d4xYdZwaB2ht"
TARGET_ZEROS   = 19
THREAD_COUNT   = 32
NONCE_MAX      = 0xFFFFFFFF
BATCH_SIZE     = 8_000
WEIGHTS_FILE   = r"C:\GENESIS\GodsEye\polar_weights.json"
ANCHOR_STEP    = int(NONCE_MAX / (GODSEYE_ANCHOR * 1000))  # ~3.93M

# â”€â”€ IntelligenceAmplifier â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
AMP_AVAILABLE = False
amp = None
try:
    from IntelligenceAmplifier import IntelligenceAmplifier
    amp = IntelligenceAmplifier()
    AMP_AVAILABLE = True
    print("[AMPLIFIER] ONLINE", flush=True)
except Exception as e:
    print(f"[AMPLIFIER] OFFLINE ({e})", flush=True)

# â”€â”€ Polar Trust Weights â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
DEFAULT_WEIGHTS = {
    "0": {"polarity": "south", "trust": 1.0},
    "1": {"polarity": "south", "trust": 1.0},
    "2": {"polarity": "south", "trust": 0.6},
    "3": {"polarity": "south", "trust": 0.6},
    "4": {"polarity": "south", "trust": 0.3},
    "5": {"polarity": "null",  "trust": 1.0},
    "6": {"polarity": "south", "trust": 0.3},
    "7": {"polarity": "north", "trust": 0.6},
    "8": {"polarity": "north", "trust": 1.0},
}
def load_weights():
    if os.path.exists(WEIGHTS_FILE):
        try:
            with open(WEIGHTS_FILE) as f: return json.load(f)
        except: pass
    return {k: dict(v) for k, v in DEFAULT_WEIGHTS.items()}
def save_weights(w):
    try:
        with open(WEIGHTS_FILE, "w") as f: json.dump(w, f, indent=2)
    except: pass

trust_weights  = load_weights()
weights_lock   = threading.Lock()
engine_weights = {"polar": 1.0, "bit": 1.0}
engine_lock    = threading.Lock()

# â”€â”€ 81D Polar Layer â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
TRINITY_DIM  = 81
TRINITY_RING = 9

def expand_81d(data):
    if isinstance(data, str): data = data.encode()
    h = hashlib.sha384(data).hexdigest()
    nodes = []
    for i in range(TRINITY_DIM):
        ring = i // TRINITY_RING; pos = i % TRINITY_RING
        offset = ring * TRINITY_RING
        idx1 = (pos + offset) % 96; idx2 = (pos + offset + TRINITY_RING) % 96
        idx3 = (pos + offset + TRINITY_RING * 2) % 96
        v1 = int(h[idx1], 16)/15.0; v2 = int(h[idx2], 16)/15.0
        v3 = int(h[idx3], 16)/15.0
        scale = (i + 1) / TRINITY_DIM
        nodes.append((v1*v2*v3)*(GODSEYE_ANCHOR**scale) % GODSEYE_ANCHOR)
    return nodes

def ring_avg(vec, ring):
    s = ring * TRINITY_RING
    return sum(vec[s:s+TRINITY_RING]) / TRINITY_RING

def compute_polar_score(vec):
    eq = ring_avg(vec, 5)
    ss = sw = ns = nw = 0.0
    with weights_lock:
        w = {k: dict(v) for k, v in trust_weights.items()}
    for r in range(9):
        e = w.get(str(r), {}); pol = e.get("polarity","null"); tr = e.get("trust",.5)
        if pol == "null": continue
        val = ring_avg(vec, r); above = max(0.0, val - eq)
        if pol == "south": ss += above*tr; sw += tr
        elif pol == "north": ns += above*tr; nw += tr
    sa = ss/max(.001,sw); na = ns/max(.001,nw); tot = sa+na
    if tot < .00001: return 0.5
    return max(0.0, min(1.0, (sa/tot)*GODSEYE_ANCHOR))

# â”€â”€ 1-3-3 Bit Layer â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
AXES = [
    {"bit": 146, "signal": 1.140, "operator": +1, "axis": "X", "name": "BIT146"},
    {"bit":  53, "signal": 1.024, "operator": +1, "axis": "Y", "name": "BIT53"},
    {"bit":  74, "signal": 0.998, "operator": -1, "axis": "Z", "name": "BIT74"},
]
def read_bit(fp, bi): return (fp[bi//8] >> (7-(bi%8))) & 1
def compute_bit_score(h76):
    fp  = hashlib.sha256(h76).digest()
    bv  = {ax["bit"]: read_bit(fp, ax["bit"]) for ax in AXES}
    sig = [((bv[ax["bit"]]-0.5)*ax["operator"]+0.5, ax["signal"]) for ax in AXES]
    return sovereign_observed_score(sig, GODSEYE_ANCHOR), fp, bv

# â”€â”€ GodsEye Harmonic Probe â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
PROBE_COUNT = 512
def double_sha256(data):
    return hashlib.sha256(hashlib.sha256(data).digest()).hexdigest()
def probe_landscape(header80):
    bs = [0]*THREAD_COUNT
    for i in range(PROBE_COUNT):
        n = (i*ANCHOR_STEP) % NONCE_MAX
        d = double_sha256(header80 + struct.pack("<I", n))
        z = 0
        for c in d:
            if c=="0": z+=1
            else: break
        b = min(THREAD_COUNT-1, int(n/NONCE_MAX*THREAD_COUNT))
        bs[b] += z
    top = bs.index(max(bs))
    return (top+0.5)/THREAD_COUNT, top, bs

def combined_score(p, b, g):
    with engine_lock: wp=engine_weights["polar"]; wb=engine_weights["bit"]
    return (p*wp + b*wb + g*1.0) / (wp+wb+1.0)

# â”€â”€ Band Setup â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
BAND_SIZE  = NONCE_MAX // THREAD_COUNT
BAND_START = [i*BAND_SIZE for i in range(THREAD_COUNT)]
BAND_END   = [(i+1)*BAND_SIZE-1 for i in range(THREAD_COUNT-1)] + [NONCE_MAX]

# â”€â”€ Turbine Queue â€” shared hot zones â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
turbine_queue = deque(maxlen=64)   # (nonce_center, band_id, zeros_hint)
turbine_lock  = threading.Lock()
turbine_hits  = 0

def turbine_push(nonce, band_id, zeros):
    global turbine_hits
    with turbine_lock:
        turbine_queue.append((nonce, band_id, zeros))
        turbine_hits += 1

def turbine_pop():
    with turbine_lock:
        return turbine_queue.popleft() if turbine_queue else None

# â”€â”€ Shared State â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
running      = True
current_job  = None
job_header80 = b""
combined_scr = 0.5; polar_scr = 0.5; bit_scr = 0.5; godeye_scr = 0.5
priority_band = 16
session_best  = {"zeros":0,"hash":"","nonce":0,"band":0}
state_lock    = threading.Lock()
total_hashes  = 0
hash_lock     = threading.Lock()

# â”€â”€ Amplifier â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def amplify(rv, nonce, zeros):
    if not AMP_AVAILABLE: return
    eq = rv[5]
    try:
        r = amp.amplify_thought(
            f"Bitcoin {zeros} zeros. Nonce:{nonce}. "
            f"Rings: "+", ".join(f"R{i}={rv[i]:.4f}" for i in range(9))+
            f". Adjust ring 4,6,7 trust?"
        )
        print(f"\n  [AMPLIFIER] {r[:250]}\n", flush=True)
        nudge = 0.05 if zeros >= 10 else 0.02
        with weights_lock:
            for ring in ["4","6"]:
                ri = int(ring)
                if rv[ri]-eq > 0.01: trust_weights[ring]["trust"] = min(1.0,trust_weights[ring]["trust"]+nudge)
                else: trust_weights[ring]["trust"] = max(0.1,trust_weights[ring]["trust"]-nudge)
            if rv[7]-eq > 0.005: trust_weights["7"]["trust"] = min(1.0,trust_weights["7"]["trust"]+nudge)
            save_weights(trust_weights)
    except Exception as e:
        print(f"  [AMPLIFIER] {e}", flush=True)

def nonce_zone(n):
    p = n/NONCE_MAX
    if p<.25: return "â–¼S"
    if p<.5: return "â–¼SEQ"
    if p<.75: return "â–²NEQ"
    return "â–²N"

# â”€â”€ Band Worker (proven pattern from 27.0) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def band_worker(band_id):
    global total_hashes
    while running:
        job = current_job
        if job is None:
            time.sleep(0.05); continue

        header80   = job_header80
        start      = BAND_START[band_id]
        end        = BAND_END[band_id]
        span       = end - start + 1
        local_best = session_best["zeros"]
        pos        = start

        while running and current_job == job:

            # â”€â”€ TURBINE CHECK: drain hot zone queue â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
            hot = turbine_pop()
            if hot is not None:
                hot_nonce, hot_band, hint_z = hot
                # All workers spiral inward on hot nonce using anchor step
                radius = ANCHOR_STEP
                lo = max(0, hot_nonce - radius)
                hi = min(NONCE_MAX, hot_nonce + radius)
                # Only work on portion in this band
                lo = max(lo, start); hi = min(hi, end)
                if hi > lo:
                    for nn in range(lo, hi, BATCH_SIZE):
                        if current_job != job: break
                        count = min(BATCH_SIZE, hi-nn)
                        for i in range(count):
                            nonce  = (nn+i) & NONCE_MAX
                            digest = double_sha256(header80+struct.pack("<I",nonce))
                            z = 0
                            for c in digest:
                                if c=="0": z+=1
                                else: break
                            if z > local_best:
                                local_best = z
                                with state_lock:
                                    if z > session_best["zeros"]:
                                        session_best.update({"zeros":z,"hash":digest,"nonce":nonce,"band":band_id})
                                        bar  = "#"*z+"-"*(TARGET_ZEROS-z)
                                        zone = nonce_zone(nonce)
                                        print(
                                            f"\n  â˜… TURBINE HIT [{z}/{TARGET_ZEROS}] (band {band_id}) {zone}\n"
                                            f"  NONCE  : {nonce:,}\n  HASH   : {digest}\n"
                                            f"  [{bar}] {z/TARGET_ZEROS*100:.1f}%\n",flush=True)
                                        if z>=6:
                                            vec=expand_81d(header80[:76])
                                            rv=[ring_avg(vec,r) for r in range(9)]
                                            threading.Thread(target=amplify,args=(rv,nonce,z),daemon=True).start()
                        with hash_lock: total_hashes += count

            # â”€â”€ NORMAL BATCH BURN â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
            for i in range(BATCH_SIZE):
                nonce  = start + ((pos-start+i) % span)
                nonce  = nonce & NONCE_MAX
                digest = double_sha256(header80+struct.pack("<I",nonce))
                z = 0
                for c in digest:
                    if c=="0": z+=1
                    else: break

                if z > local_best:
                    local_best = z
                    with state_lock:
                        if z > session_best["zeros"]:
                            session_best.update({"zeros":z,"hash":digest,"nonce":nonce,"band":band_id})
                            bar  = "#"*z+"-"*(TARGET_ZONES:=TARGET_ZEROS-z and TARGET_ZEROS-z)
                            bar  = "#"*z+"-"*(TARGET_ZEROS-z)
                            zone = nonce_zone(nonce)
                            print(
                                f"\n  *** JET HIT [{z}/{TARGET_ZEROS}] Band {band_id} {zone}\n"
                                f"  P:{polar_scr:.3f} B:{bit_scr:.3f} G:{godeye_scr:.3f} C:{combined_scr:.3f}\n"
                                f"  NONCE  : {nonce:,}\n  HASH   : {digest}\n"
                                f"  [{bar}] {z/TARGET_ZEROS*100:.1f}%\n",flush=True)
                            # Push to turbine â€” other workers will spiral in
                            if z >= 4:
                                turbine_push(nonce, band_id, z)
                            if z >= 6:
                                vec=expand_81d(header80[:76])
                                rv=[ring_avg(vec,r) for r in range(9)]
                                threading.Thread(target=amplify,args=(rv,nonce,z),daemon=True).start()

            pos = start + ((pos-start+BATCH_SIZE) % span)
            with hash_lock: total_hashes += BATCH_SIZE

# â”€â”€ Stratum Bridge â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def stratum_loop(sock):
    global current_job, job_header80, combined_scr
    global polar_scr, bit_scr, godeye_scr, priority_band
    buf = ""
    while running:
        try:
            chunk = sock.recv(4096).decode(errors="ignore")
            if not chunk: break
            buf += chunk
            while "\n" in buf:
                line, buf = buf.split("\n",1)
                if not line.strip(): continue
                try:
                    msg = json.loads(line)
                    if msg.get("method") == "mining.notify":
                        params   = msg["params"]
                        job_id   = params[0]
                        header80 = (params[1]+params[2]+params[3]).encode(errors="ignore")[:80]
                        h76      = header80[:76]
                        vec      = expand_81d(h76)
                        p_scr    = compute_polar_score(vec)
                        rv       = [ring_avg(vec,r) for r in range(9)]
                        b_scr,fp,bv = compute_bit_score(h76)
                        ge_scr,ge_band,bscores = probe_landscape(header80)
                        c_scr    = combined_score(p_scr, b_scr, ge_scr)
                        pb_      = int(c_scr*(THREAD_COUNT-1))

                        polar_scr=p_scr; bit_scr=b_scr; godeye_scr=ge_scr
                        combined_scr=c_scr; priority_band=pb_
                        current_job=job_id; job_header80=header80

                        # Seed known hot zones into turbine for new job
                        with turbine_lock:
                            turbine_queue.clear()
                        center_14 = BAND_START[14] + BAND_SIZE//2
                        center_25 = BAND_START[25] + BAND_SIZE//2
                        turbine_push(center_14, 14, 6)
                        turbine_push(center_25, 25, 7)

                        pole = "â–²N" if c_scr>.55 else "â–¼S" if c_scr<.45 else "â—EQ"
                        print(f"\n{'='*72}",flush=True)
                        print(f"  [NEW JOB]    {job_id}",flush=True)
                        print(f"  [POLAR]      {p_scr:.4f}",flush=True)
                        for ax in AXES:
                            st="SET" if bv[ax["bit"]] else "CLR"
                            co=(bv[ax["bit"]]-.5)*ax["signal"]*ax["operator"]
                            print(f"  [{ax['name']:<7}]  {ax['axis']}: {st} | {co:+.3f}",flush=True)
                        print(f"  [BIT]        {b_scr:.4f}",flush=True)
                        print(f"  [GODEYE]     {ge_scr:.4f} | hot band {ge_band} | score={bscores[ge_band]}",flush=True)
                        print(f"  [COMBINED]   {c_scr:.4f} | {pole} | PRIORITY â†’ Band {pb_}",flush=True)
                        print(f"  [TURBINE]    Seeded B14 + B25 (live hot zones)",flush=True)
                        print(f"{'='*72}",flush=True)
                    elif msg.get("method") == "mining.set_difficulty":
                        print(f"  [DIFF] {msg['params'][0]}",flush=True)
                except Exception: pass
        except (BlockingIOError, socket.error): time.sleep(0.01)

# â”€â”€ Main â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def ignite(host="solo.ckpool.org", port=3333):
    global running
    print(f"[!] GODSEYE 29.1 â€” SOVEREIGN JET ENGINE (FIXED)",flush=True)
    print(f"    Architecture : 32 workers + shared Turbine queue")
    print(f"    Layers       : Polar + 1-3-3 Bit + GodsEye Probe")
    print(f"    Turbine      : 4+ zeros â†’ push to hot-zone queue â†’ all workers spiral in")
    print(f"    Seeds        : B14 (6z) + B25 (7z) â€” known hot from live data")
    print(f"    Amplifier    : {'ONLINE' if AMP_AVAILABLE else 'OFFLINE'}\n")

    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.settimeout(10.0)
    sock.connect((host, port))
    sock.sendall((json.dumps({"id":1,"method":"mining.subscribe","params":[]}) + "\n").encode())
    _ = sock.recv(4096)
    sock.sendall((json.dumps({"id":2,"method":"mining.authorize","params":[WALLET_ADDRESS,"x"]}) + "\n").encode())
    sock.setblocking(False)
    print(f"[SUCCESS] AUTHORIZED: {WALLET_ADDRESS}")
    print("-"*72, flush=True)

    threading.Thread(target=stratum_loop, args=(sock,), daemon=True).start()
    for i in range(THREAD_COUNT):
        threading.Thread(target=band_worker, args=(i,), daemon=True).start()

    last = 0
    try:
        while True:
            time.sleep(1.0)
            with hash_lock: t = total_hashes
            with state_lock: z = session_best["zeros"]; band = session_best["band"]
            rate = (t-last)/1_000_000; last = t
            bar  = "#"*z+"-"*(TARGET_ZEROS-z)
            pole = "N" if combined_scr>.55 else "S" if combined_scr<.45 else "EQ"
            print(
                f"  {rate:.3f} MH/s | BEST:{z}/{TARGET_ZEROS} [B{band:02d}] | "
                f"P:{polar_scr:.3f} B:{bit_scr:.3f} G:{godeye_scr:.3f} "
                f"C:{combined_scr:.3f}{pole} | T:{turbine_hits} | [{bar}]",flush=True)
    except KeyboardInterrupt:
        running = False
        print("\n[!] SOVEREIGN JET SHUTDOWN.", flush=True)

if __name__ == "__main__":
    ignite()
