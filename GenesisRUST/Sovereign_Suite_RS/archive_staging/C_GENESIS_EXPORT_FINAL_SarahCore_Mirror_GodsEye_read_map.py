import json

with open("bitcoin_causation_map.json") as f:
    d = json.load(f)

m = d["causation_map"]
blocks = d["blocks"]

print(f"BLOCKS DECODED : {len(blocks)}")
print(f"NONCE MIN      : {m['nonce_min']}")
print(f"NONCE MAX      : {m['nonce_max']}")
print(f"NONCE AVG      : {m['nonce_avg']}")
print()
print("ALL BIT DEVIATIONS (ranked by magnitude):")
all_devs = [(i, c/len(blocks), abs(c/len(blocks)-0.5)) for i, c in enumerate(m["bit_counts"])]
for bit, freq, dev in sorted(all_devs, key=lambda x: -x[2]):
    direction = "PREFERS 1" if freq > 0.5 else "PREFERS 0"
    flag = " ***" if dev > 0.05 else ""
    print(f"  BIT {bit:>2} | FREQ: {freq:.1%} | DEV: {dev:.1%} | {direction}{flag}")

print()
print("BIT FREQUENCY MAP:")
for i, c in enumerate(m["bit_counts"]):
    freq = c / len(blocks)
    bar = "#" * int(freq * 20)
    print(f"  BIT {i:>2} | {freq:.1%} | {bar}")
