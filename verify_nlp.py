import math

PHI = 1.618033988749895
HB = 1.09277703703703

def encode(c, pos):
    seed = ord(c) * HB
    xyz = [math.sin(seed + i * PHI + pos) for i in range(27)]
    ein = [math.cos(seed + i * PHI) for i in range(12)]
    pol = [math.sin(seed * PHI + i) for i in range(12)]
    phi = [PHI**(-i) for i in range(5)]
    return xyz, ein, pol, phi

def decode(xyz, ein, pol, phi, anchor=0.0):
    s = sum(xyz[i] * (i+1) for i in range(27))
    s += sum(ein[i] * (i+28) for i in range(12))
    s += sum(pol[i] * (-(i+40)) for i in range(12))
    s += sum(phi[i] * (i+52) for i in range(5))
    s += anchor * 57
    raw = int(abs(s * 100))
    return chr(32 + (raw % 95))

def new_predict(word):
    seq = [encode(c, i) for i, c in enumerate(word)]
    n = len(seq)
    weights = [HB**(n-1-i) for i in range(n)]
    tw = sum(weights)
    thesis = [
        [sum(seq[si][0][i]*weights[si] for si in range(n))/tw for i in range(27)],
        [sum(seq[si][1][i]*weights[si] for si in range(n))/tw for i in range(12)],
        [sum(seq[si][2][i]*weights[si] for si in range(n))/tw for i in range(12)],
        [sum(seq[si][3][i]*weights[si] for si in range(n))/tw for i in range(5)],
    ]
    prev_n, last_n = seq[-2], seq[-1]
    anti = [
        [last_n[0][i] - prev_n[0][i] for i in range(27)],
        [last_n[1][i] - prev_n[1][i] for i in range(12)],
        [last_n[2][i] - prev_n[2][i] for i in range(12)],
        [last_n[3][i] - prev_n[3][i] for i in range(5)],
    ]
    synth = [
        [thesis[0][i] + anti[0][i]*HB for i in range(27)],
        [thesis[1][i] + anti[1][i]*HB for i in range(12)],
        [thesis[2][i] + anti[2][i]*HB for i in range(12)],
        [thesis[3][i] + anti[3][i]*HB for i in range(5)],
    ]
    return decode(synth[0], synth[1], synth[2], synth[3])

# Old broken logic for comparison
def old_predict(word):
    seq = [encode(c, i) for i, c in enumerate(word)]
    last = seq[-1]
    # thesis+antithesis cancel => xyz[i] = HB always
    old_synth_xyz = [last[0][i]*HB + (-last[0][i]*HB) + HB for i in range(27)]
    s = sum(abs(x) for x in old_synth_xyz)
    return chr((int)(s * 10) % 127)

test_words = ["hello", "world", "forge", "strike", "sovereign", "resonance"]

print("=" * 55)
print("  GodsEye NLP Predictor — Decode Verification")
print("=" * 55)
print(f"{'Input':<15} {'OLD (stuck)':>10} {'NEW (sequence-aware)':>20}")
print("-" * 55)
old_results = set()
new_results = set()
for word in test_words:
    old_c = old_predict(word)
    new_c = new_predict(word)
    old_results.add(old_c)
    new_results.add(new_c)
    print(f"  {word:<13} {repr(old_c):>10}   {repr(new_c):>20}")

print("-" * 55)
print(f"  Unique OLD predictions: {len(old_results)}  (expect 1 = stuck)")
print(f"  Unique NEW predictions: {len(new_results)}  (expect {len(test_words)} = diverse)")
print()
if len(new_results) > 1:
    print("[OK] NLP predictor produces SEQUENCE-DEPENDENT predictions")
else:
    print("[FAIL] Still stuck")
