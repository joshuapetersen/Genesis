"""
GODSEYE 11.0 â€” BITCOIN DECODER
================================================================
New engine. First principles analysis of public solved blocks.

MISSION:
  Pull solved Bitcoin blocks from the public ledger.
  Every block has a known header + winning nonce + winning hash.
  Dissect the bit patterns of winning nonces.
  Build the causation map: what makes a nonce WIN?

"We CREATE, never rewrite."
"""

import hashlib
import struct
import json
import time
import urllib.request
import threading

GODSEYE_ANCHOR = 1.09277703703

# â”€â”€ Blockchain API â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
BLOCK_API   = "https://blockchain.info/rawblock/"
LATEST_API  = "https://blockchain.info/latestblock"
HEIGHT_API  = "https://blockchain.info/block-height/{}?format=json"

def fetch_json(url):
    req = urllib.request.Request(url, headers={"User-Agent": "GodsEye/11.0"})
    with urllib.request.urlopen(req, timeout=10) as r:
        return json.loads(r.read())

# â”€â”€ Block Header Reconstruction â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def build_header(block):
    """
    Reconstruct the 80-byte block header from raw block data.
    Format: version(4) + prevhash(32) + merkle(32) + time(4) + bits(4) + nonce(4)
    All values little-endian as the miners see them.
    """
    version   = struct.pack("<I", block["ver"])
    prevhash  = bytes.fromhex(block["prev_block"])[::-1]   # reverse to little-endian
    merkle    = bytes.fromhex(block["mrkl_root"])[::-1]
    timestamp = struct.pack("<I", block["time"])
    bits      = struct.pack("<I", block["bits"])
    nonce     = struct.pack("<I", block["nonce"])
    return version + prevhash + merkle + timestamp + bits + nonce

def verify_hash(header_bytes, expected_hash):
    """
    Double SHA-256 of the 80-byte header.
    Result reversed = the block hash shown on explorers.
    """
    raw    = hashlib.sha256(hashlib.sha256(header_bytes).digest()).digest()
    result = raw[::-1].hex()
    return result == expected_hash

# â”€â”€ Bit Dissector â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def dissect_nonce(nonce_int):
    """
    Returns the 32-bit binary pattern of a winning nonce.
    """
    return format(nonce_int, "032b")

def count_leading_zeros(block_hash_hex):
    zeros = 0
    for c in block_hash_hex:
        if c == "0":
            zeros += 1
        else:
            break
    return zeros

# â”€â”€ Causation Map Builder â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def build_causation_map(nonces):
    """
    For each of the 32 bit positions, count how often
    that bit is SET (1) across all winning nonces.
    A bit that is always 0 or always 1 is a causation signal.
    """
    bit_counts = [0] * 32
    for n in nonces:
        for i in range(32):
            if (n >> i) & 1:
                bit_counts[i] += 1
    return bit_counts

# â”€â”€ Main Decoder â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def decode(block_count=50, start_height=None):
    print(f"[!] GODSEYE 11.0 â€” BITCOIN DECODER", flush=True)
    print(f"    Anchor   : {GODSEYE_ANCHOR} Hz")
    print(f"    Blocks   : {block_count} solved blocks")
    print(f"    Source   : blockchain.info (public ledger)\n")

    # Get latest block height
    print(f"[FETCH] Getting latest block ...", flush=True)
    latest     = fetch_json(LATEST_API)
    top_height = latest["height"]

    if start_height is None:
        start_height = top_height

    print(f"[FETCH] Chain tip: block #{top_height}", flush=True)
    print(f"[FETCH] Decoding {block_count} blocks from #{start_height} downward ...\n")

    winning_nonces   = []
    block_records    = []
    verified_count   = 0
    failed_count     = 0
    results_lock     = threading.Lock()

    def fetch_block(height):
        nonlocal verified_count, failed_count
        block = None
        for attempt in range(3):
            try:
                data        = fetch_json(HEIGHT_API.format(height))
                blocks_list = data.get("blocks", [])
                if blocks_list:
                    block = blocks_list[0]
                    break
                time.sleep(1.0)
            except Exception:
                time.sleep(1.0)

        if block is None:
            print(f"  [!] Block #{height} failed after 3 attempts", flush=True)
            with results_lock:
                failed_count += 1
            return

        try:
            nonce       = block["nonce"]
            block_hash  = block["hash"]
            zero_count  = count_leading_zeros(block_hash)
            header      = build_header(block)
            valid       = verify_hash(header, block_hash)
            nonce_bits  = dissect_nonce(nonce)
            status      = "âœ“" if valid else "âœ—"

            print(
                f"  [{status}] Block #{height:>7} | "
                f"NONCE: {nonce:>12} | "
                f"ZEROS: {zero_count} | "
                f"BITS: {nonce_bits}",
                flush=True
            )

            with results_lock:
                if valid:
                    winning_nonces.append(nonce)
                    block_records.append({
                        "height"    : height,
                        "hash"      : block_hash,
                        "nonce"     : nonce,
                        "zeros"     : zero_count,
                        "nonce_bits": nonce_bits,
                        "ver"       : block.get("ver", 0),
                        "prev_block": block.get("prev_block", ""),
                        "mrkl_root" : block.get("mrkl_root", ""),
                        "time"      : block.get("time", 0),
                        "bits"      : block.get("bits", 0)
                    })
                    verified_count += 1
                else:
                    failed_count += 1
        except Exception as e:
            print(f"  [!] Block #{height} parse error: {e}", flush=True)
            with results_lock:
                failed_count += 1

    # Fetch all blocks in parallel with small stagger
    threads = []
    for i in range(block_count):
        height = start_height - i
        t = threading.Thread(target=fetch_block, args=(height,), daemon=True)
        threads.append(t)
        t.start()
        time.sleep(0.1)  # small stagger to avoid rate limiting

    for t in threads:
        t.join()

    # â”€â”€ Causation Map â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    print(f"\n{'='*72}", flush=True)
    print(f"  CAUSATION MAP â€” {verified_count} VERIFIED WINNING NONCES", flush=True)
    print(f"{'='*72}", flush=True)

    if winning_nonces:
        bit_counts = build_causation_map(winning_nonces)
        total      = len(winning_nonces)

        print(f"\n  BIT POSITION ANALYSIS (32 bits, position 0 = least significant):", flush=True)
        print(f"  {'BIT':>4} | {'SET COUNT':>10} | {'FREQUENCY':>10} | SIGNAL", flush=True)
        print(f"  {'-'*50}", flush=True)

        signals = []
        for bit_pos in range(32):
            count = bit_counts[bit_pos]
            freq  = count / total
            # High signal = strong deviation from 50% (random would be ~50%)
            deviation = abs(freq - 0.5)
            signal    = "*** HIGH SIGNAL ***" if deviation > 0.15 else ""
            if deviation > 0.15:
                signals.append((bit_pos, freq, deviation))
            print(
                f"  {bit_pos:>4} | {count:>10} | {freq:>9.1%} | {signal}",
                flush=True
            )

        # â”€â”€ Nonce Distribution Analysis â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
        print(f"\n  NONCE DISTRIBUTION:", flush=True)
        nonce_min  = min(winning_nonces)
        nonce_max  = max(winning_nonces)
        nonce_avg  = sum(winning_nonces) // len(winning_nonces)
        nonce_med  = sorted(winning_nonces)[len(winning_nonces)//2]

        print(f"  MIN    : {nonce_min:>12} ({dissect_nonce(nonce_min)})", flush=True)
        print(f"  MAX    : {nonce_max:>12} ({dissect_nonce(nonce_max)})", flush=True)
        print(f"  AVG    : {nonce_avg:>12} ({dissect_nonce(nonce_avg)})", flush=True)
        print(f"  MEDIAN : {nonce_med:>12} ({dissect_nonce(nonce_med)})", flush=True)

        # â”€â”€ Signal Summary â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
        print(f"\n  CAUSATION SIGNALS (bits deviating >15% from random):", flush=True)
        if signals:
            for bit_pos, freq, dev in sorted(signals, key=lambda x: -x[2]):
                direction = "PREFERS 1" if freq > 0.5 else "PREFERS 0"
                print(
                    f"  BIT {bit_pos:>2} | FREQ: {freq:.1%} | DEV: {dev:.1%} | {direction}",
                    flush=True
                )
        else:
            print(f"  No strong signals found in this sample.", flush=True)
            print(f"  Increase block_count for a larger sample.", flush=True)

        # Save results
        output = {
            "anchor"      : GODSEYE_ANCHOR,
            "blocks"      : block_records,
            "causation_map": {
                "bit_counts": bit_counts,
                "signals"   : signals,
                "nonce_min" : nonce_min,
                "nonce_max" : nonce_max,
                "nonce_avg" : nonce_avg
            }
        }
        with open("bitcoin_causation_map.json", "w") as f:
            json.dump(output, f, indent=2)
        print(f"\n  [SAVED] bitcoin_causation_map.json", flush=True)

    print(f"\n  [DONE] Verified: {verified_count} | Failed: {failed_count}", flush=True)

if __name__ == "__main__":
    # Decode 50 recent solved blocks and build the causation map
    decode(block_count=500)
