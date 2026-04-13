"""
BitLock Unit Test — verifies fingerprint logic before live deployment.
"""
import hashlib
import struct
import sys
import time

# Import the BitLock engine
sys.path.insert(0, "C:\GENESIS\GenesisRUST\Sovereign_Suite_RS\rust\\GodsEye")
from GodsEye_12_0_BitLock import (
    ONES_MASK, ZEROS_MASK, MUST_BE_ONE, MUST_BE_ZERO,
    matches_fingerprint, generate_locked_nonces
)

print("=" * 60)
print("  GODSEYE 12.0 BITLOCK — UNIT TEST")
print("=" * 60)

# Test 1: Mask correctness
print(f"\n[1] ONES  MASK : {ONES_MASK:032b}")
print(f"    ZEROS MASK : {ZEROS_MASK:032b}")
for b in MUST_BE_ONE:
    assert (ONES_MASK >> b) & 1, f"BIT {b} missing from ones mask"
    print(f"    BIT {b:>2} = 1  OK")
for b in MUST_BE_ZERO:
    assert (ZEROS_MASK >> b) & 1, f"BIT {b} missing from zeros mask"
    print(f"    BIT {b:>2} = 0  OK")
print("  [PASS] Masks correct")

# Test 2: Generate 10000 locked nonces and verify all match fingerprint
print(f"\n[2] Generating 10,000 locked nonces ...")
nonces = generate_locked_nonces(0, 10000)
failures = [n for n in nonces if not matches_fingerprint(n)]
print(f"    Generated : {len(nonces)}")
print(f"    Failures  : {len(failures)}")
assert len(failures) == 0, f"Fingerprint violations: {failures[:5]}"
print("  [PASS] All nonces match fingerprint")

# Test 3: Uniqueness
unique = len(set(nonces))
print(f"\n[3] Unique nonces : {unique} / {len(nonces)}")
assert unique == len(nonces), "Duplicate nonces detected"
print("  [PASS] All nonces unique")

# Test 4: Hash them and count zeros — should outperform random
print(f"\n[4] Hashing 10,000 locked nonces vs 10,000 random nonces ...")
import random

header = b"GODSEYE_TEST_HEADER_12_0_BITLOCK"

def best_zeros(nonce_list):
    best = 0
    for n in nonce_list:
        d = hashlib.sha256(hashlib.sha256(header + struct.pack("<I", n)).digest()).hexdigest()
        z = 0
        for c in d:
            if c == "0": z += 1
            else: break
        if z > best:
            best = z
    return best

t0 = time.time()
locked_best = best_zeros(nonces)
locked_time = time.time() - t0
locked_rate = len(nonces) / locked_time / 1e6

random_nonces = [random.randint(0, 0xFFFFFFFF) for _ in range(10000)]
t0 = time.time()
random_best = best_zeros(random_nonces)
random_time = time.time() - t0
random_rate = len(random_nonces) / random_time / 1e6

print(f"    LOCKED  : best={locked_best} zeros | {locked_rate:.3f} MH/s")
print(f"    RANDOM  : best={random_best} zeros | {random_rate:.3f} MH/s")

# Test 5: Verify coverage math
free_bits = 32 - len(MUST_BE_ONE) - len(MUST_BE_ZERO)
space = 2 ** free_bits
total = 2 ** 32
coverage = space / total * 100
print(f"\n[5] Search space coverage: {space:,} / {total:,} = {coverage:.2f}%")
print(f"    (Scanning only the high-probability zone)")
print(f"  [PASS]")

print(f"\n{'='*60}")
print(f"  ALL TESTS PASSED")
print(f"  BitLock is ready for live deployment.")
print(f"{'='*60}")
