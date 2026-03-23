import os
import math

path = r'C:\SarahCore\Sovereign_Math.py'
with open(path, 'r', encoding='utf-8', errors='ignore') as f:
    lines = f.readlines()

print(f"Read {len(lines)} lines.")

# 1. Fix L282 (Primary Resonance)
found_v1 = False
for i, line in enumerate(lines):
    if 'def _0x_resonance(self, _0x_v1: list, _0x_v2: list) -> float:' in line and i < 500:
        found_v1 = True
        print(f"Found V1 at line {i+1}. Applying L2 normalization...")
        # We'll replace the block from line i to whenever it returns
        # Actually, let's just find the specific lines to replace
        for j in range(i, i+50):
            if 'score = (similarity / self._0x_dim) * self._0x_sigma' in lines[j]:
                lines[j] = "            # Phase 27 Fix: High-precision Euclidean (L2) Resonance\n"
                lines[j] += "            diff_sq = sub.square(a1 - a2)\n"
                lines[j] += "            dist = sub.sqrt(sub.sum(diff_sq))\n"
                lines[j] += "            score = (1.0 - (dist / math.sqrt(self._0x_dim))) * self._0x_sigma\n"
                print(f"  Fixed math at line {j+1}")
                break

# 2. Find and DELETE V2 (The Fragmented one)
found_v2_at = -1
for i, line in enumerate(lines):
    if 'def _0x_resonance(self, _0x_v1: list, _0x_v2: list) -> float:' in line and i > 500:
        found_v2_at = i
        print(f"Found fragmented V2 at line {i+1}. Identifying boundaries for deletion...")
        break

if found_v2_at != -1:
    # Delete from the def until the next return or a line that clearly isn't part of it
    end_v2 = found_v2_at
    for j in range(found_v2_at, found_v2_at + 30):
        if 'return score' in lines[j]:
            end_v2 = j + 1
            break
    
    print(f"  Deleting lines {found_v2_at+1} to {end_v2}...")
    del lines[found_v2_at:end_v2]

# 3. Write back
with open(path, 'w', encoding='utf-8') as f:
    f.writelines(lines)
print("Repair Successful.")
