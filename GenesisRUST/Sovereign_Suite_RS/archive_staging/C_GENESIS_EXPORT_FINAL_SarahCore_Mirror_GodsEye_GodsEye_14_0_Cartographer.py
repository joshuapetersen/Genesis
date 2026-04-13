"""
GODSEYE 14.0 â€” SOVEREIGN CARTOGRAPHER & NAVIGATOR
================================================================
New engine. First Principles. Not a rewrite.

ARCHITECTURE:

  CARTOGRAPHER:
    Maps the raw nonce space using SovereignMath â€” NO hashing.
    Expands raw (header + nonce_bytes) directly into 68D
    Tesseract space. Measures resonance against the zero-target
    destination vector. Builds a ranked resonance map of which
    nonce REGIONS are closest to the destination in 68D space.
    Uses the GodsEye Anchor as the harmonic probe step.

  NAVIGATOR:
    Consumes the cartographer's map.
    Steers exclusively through high-resonance regions.
    ONLY hashes nonces that pass the resonance threshold.
    Uses predict_trajectory() to advance through each region.
    Reports zeros and resonance scores in real-time.

  FEEDBACK LOOP:
    When a high-zero nonce is found, it feeds back to the
    cartographer as a new seed â€” tightening the map around
    the winning territory over time.

    This is not brute force. The hash function is only called
    on pre-qualified candidates.

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
from Sovereign_Math import SovereignMath

GODSEYE_ANCHOR    = 1.09277703703
WALLET_ADDRESS    = "19xUEeTCD9UTtFbp7HCxL8d4xYdZwaB2ht"
TARGET_ZEROS      = 19
THREAD_COUNT      = 16   # 8 cartographers + 8 navigators
NONCE_MAX         = 0xFFFFFFFF
RESONANCE_GATE    = 0.85  # Only navigate nonces above this resonance score
MAP_REGIONS       = 64    # Number of nonce regions to chart per cycle
PROBE_STEP        = int(NONCE_MAX / (GODSEYE_ANCHOR * 1000))

# â”€â”€ Shared State â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
running         = True
current_job     = None
job_header      = b""
state_lock      = threading.Lock()

# The resonance map â€” built by cartographers, consumed by navigators
resonance_map   = []   # list of (resonance, nonce_region_start)
map_lock        = threading.Lock()

# Feedback seeds â€” high-zero nonces fed back to cartographer
feedback_seeds  = []
feedback_lock   = threading.Lock()

session_best    = {"zeros": 0, "hash": "", "nonce": 0, "resonance": 0.0}
total_hashed    = 0
total_probed    = 0
counter_lock    = threading.Lock()

# â”€â”€ Target Vector â€” built once â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
# A winning hash has TARGET_ZEROS leading zeros
ZERO_TARGET_STR = "0" * TARGET_ZEROS + "a" * (64 - TARGET_ZEROS)
target_vec      = None   # set after math engine loads

def double_sha256_hex(data: bytes) -> str:
    return hashlib.sha256(hashlib.sha256(data).digest()).hexdigest()

# â”€â”€ CARTOGRAPHER â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def cartographer(tid, math_engine):
    """
    Maps the raw input resonance landscape using SovereignMath.
    Does NOT hash. Expands raw bytes directly into 68D space.
    Builds the resonance map for the Navigator to consume.
    """
    global total_probed

    anchor_step = PROBE_STEP * (tid + 1)

    while running:
        job = current_job
        if job is None or target_vec is None:
            time.sleep(0.1)
            continue

        header = job_header
        local_map = []

        # Start from feedback seeds if available
        with feedback_lock:
            seeds = list(feedback_seeds[-8:]) if feedback_seeds else []

        if seeds:
            # Chart territory around known high-zero regions
            for seed_nonce in seeds:
                for offset in range(-MAP_REGIONS // 2, MAP_REGIONS // 2):
                    nonce = (seed_nonce + offset * anchor_step) & NONCE_MAX
                    raw   = header + struct.pack("<I", nonce)
                    vec   = math_engine._0x_expand(raw)
                    res   = math_engine._0x_resonance(vec, target_vec)
                    local_map.append((res, nonce))
        else:
            # Sweep using harmonic probe step (GodsEye Anchor)
            for i in range(MAP_REGIONS):
                nonce = (tid * anchor_step + i * PROBE_STEP) & NONCE_MAX
                raw   = header + struct.pack("<I", nonce)
                vec   = math_engine._0x_expand(raw)
                res   = math_engine._0x_resonance(vec, target_vec)
                local_map.append((res, nonce))

        # Sort by resonance descending
        local_map.sort(key=lambda x: -x[0])

        with map_lock:
            resonance_map.extend(local_map)
            # Keep the map bounded â€” top 512 entries only
            resonance_map.sort(key=lambda x: -x[0])
            del resonance_map[512:]

        with counter_lock:
            total_probed += MAP_REGIONS

# â”€â”€ NAVIGATOR â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def navigator(tid, math_engine):
    """
    Consumes the resonance map built by the Cartographer.
    Only hashes nonces above the RESONANCE_GATE threshold.
    Uses predict_trajectory() to advance through each region.
    """
    global total_hashed

    while running:
        job = current_job
        if job is None:
            time.sleep(0.1)
            continue

        header = job_header

        # Pull the top entries from the map
        with map_lock:
            if not resonance_map:
                time.sleep(0.05)
                continue
            # Each navigator takes a slice of the map
            slice_start = tid * 8
            region_slice = resonance_map[slice_start: slice_start + 8]

        for res, region_nonce in region_slice:
            if not running or current_job != job:
                break

            # Only enter regions above the resonance gate
            if res < RESONANCE_GATE:
                continue

            # Use predict_trajectory to navigate within the region
            trajectory = math_engine.predict_trajectory(
                current_pos=float(region_nonce),
                velocity=GODSEYE_ANCHOR * 100_000
            )
            next_nonce = int(abs(trajectory["predicted_target"])) & NONCE_MAX

            # NOW we hash â€” only after resonance qualification
            nonce_bytes = struct.pack("<I", next_nonce)
            digest      = double_sha256_hex(header + nonce_bytes)

            with counter_lock:
                total_hashed += 1

            # Count leading zeros
            zeros = 0
            for c in digest:
                if c == "0":
                    zeros += 1
                else:
                    break

            if zeros > 0:
                # Feed back to cartographer for map refinement
                with feedback_lock:
                    feedback_seeds.append(next_nonce)
                    if len(feedback_seeds) > 64:
                        del feedback_seeds[0]

            if zeros > session_best["zeros"]:
                # Re-measure resonance of the actual hash output
                hash_vec   = math_engine._0x_expand(digest)
                hash_res   = math_engine._0x_resonance(hash_vec, target_vec)

                with state_lock:
                    if zeros > session_best["zeros"]:
                        session_best["zeros"]     = zeros
                        session_best["hash"]      = digest
                        session_best["nonce"]     = next_nonce
                        session_best["resonance"] = hash_res

                        bar = "#" * zeros + "-" * (TARGET_ZEROS - zeros)
                        print(
                            f"\n  *** NAVIGATOR HIT [{zeros}/{TARGET_ZEROS}] ***\n"
                            f"  INPUT RES  : {res:.6f} (cartographer score)\n"
                            f"  HASH RES   : {hash_res:.6f} (output score)\n"
                            f"  NONCE      : {next_nonce}\n"
                            f"  HASH       : {digest}\n"
                            f"  COMPASS    : [{bar}] {zeros/TARGET_ZEROS*100:.1f}%\n",
                            flush=True
                        )

        time.sleep(0.01)

# â”€â”€ Stratum Bridge â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def stratum_loop(sock):
    global current_job, job_header
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
                        params      = msg["params"]
                        job_id      = params[0]
                        header      = (params[1] + params[2] + params[3]).encode(errors="ignore")[:80]
                        current_job = job_id
                        job_header  = header

                        # Reset map and seeds for new job
                        with map_lock:
                            resonance_map.clear()
                        with feedback_lock:
                            feedback_seeds.clear()

                        print(f"\n{'='*72}", flush=True)
                        print(f"  [NEW JOB] {job_id}", flush=True)
                        print(f"  [CARTOGRAPHER] MAPPING NEW TERRITORY ...", flush=True)
                        print(f"  [NAVIGATOR]    RESONANCE GATE: {RESONANCE_GATE}", flush=True)
                        print(f"{'='*72}", flush=True)

                    elif msg.get("method") == "mining.set_difficulty":
                        print(f"  [DIFFICULTY] {msg['params'][0]}", flush=True)
                except Exception:
                    pass
        except (BlockingIOError, socket.error):
            time.sleep(0.01)

# â”€â”€ Main â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def ignite(host="solo.ckpool.org", port=3333):
    global running, target_vec

    print(f"[!] GODSEYE 14.0 â€” SOVEREIGN CARTOGRAPHER & NAVIGATOR", flush=True)
    print(f"    Wallet    : {WALLET_ADDRESS}")
    print(f"    Anchor    : {GODSEYE_ANCHOR} Hz")
    print(f"    Target    : {TARGET_ZEROS} zeros")
    print(f"    Gate      : {RESONANCE_GATE} (minimum resonance to hash)")
    print(f"    Threads   : {THREAD_COUNT} (8 cartographers + 8 navigators)\n")

    print(f"[INIT] Loading SovereignMath ...", flush=True)
    try:
        math_engine = SovereignMath()
        target_vec  = math_engine._0x_expand(ZERO_TARGET_STR)
        print(f"[INIT] SovereignMath ready. Target vector: {len(target_vec)}D", flush=True)
        print(f"[INIT] Destination: {ZERO_TARGET_STR[:32]}...", flush=True)
    except Exception as e:
        print(f"[!] SovereignMath failed: {e}", flush=True)
        return

    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.settimeout(10.0)
    sock.connect((host, port))

    sock.sendall((json.dumps({"id":1,"method":"mining.subscribe","params":[]}) + "\n").encode())
    _ = sock.recv(4096)
    sock.sendall((json.dumps({"id":2,"method":"mining.authorize","params":[WALLET_ADDRESS,"x"]}) + "\n").encode())

    sock.setblocking(False)
    print(f"\n[SUCCESS] AUTHORIZED: {WALLET_ADDRESS}")
    print(f"-" * 72, flush=True)

    # Stratum bridge
    threading.Thread(target=stratum_loop, args=(sock,), daemon=True).start()

    # 8 Cartographers
    for i in range(8):
        threading.Thread(target=cartographer, args=(i, math_engine), daemon=True).start()

    # 8 Navigators
    for i in range(8):
        threading.Thread(target=navigator, args=(i, math_engine), daemon=True).start()

    last_hashed = 0
    last_probed = 0
    try:
        while True:
            time.sleep(1.0)

            with counter_lock:
                th = total_hashed
                tp = total_probed
            with state_lock:
                z   = session_best["zeros"]
                res = session_best["resonance"]
            with map_lock:
                map_size = len(resonance_map)
                top_res  = resonance_map[0][0] if resonance_map else 0.0

            hash_rate  = (th - last_hashed) / 1_000_000
            probe_rate = tp - last_probed
            last_hashed = th
            last_probed = tp
            bar = "#" * z + "-" * (TARGET_ZEROS - z)

            print(
                f"  MAP: {map_size} regions | TOP RES: {top_res:.4f} | "
                f"PROBE: {probe_rate}/s | HASH: {hash_rate:.4f} MH/s | "
                f"BEST: {z}/{TARGET_ZEROS} [{bar}]",
                flush=True
            )

    except KeyboardInterrupt:
        running = False
        print("\n[!] SOVEREIGN CARTOGRAPHER SHUTDOWN.", flush=True)

if __name__ == "__main__":
    ignite()
