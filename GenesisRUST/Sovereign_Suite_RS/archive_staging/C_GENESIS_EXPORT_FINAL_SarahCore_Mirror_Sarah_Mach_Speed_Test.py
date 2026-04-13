import time
import sys
sys.path.append("C:\GenesisOS_Core")
from Sarah_Fast_Brain import SarahFastBrain

print("=" * 80)
print("SARAH MACH SPEED TEST - 500MS CHALLENGE")
print("=" * 80)

# Initialize Mach Kernel only (ignore LLM for speed test)
from Sovereign_Math import SovereignMath
math = SovereignMath()

brain = SarahFastBrain()
# Note: Since Brain singleton is already initialized in memory if running from same process, 
# but here we just want to test the mach_solve method speed.

prompt = "[MACH] Solve the 11GB singularity paradox relative to volumetric C3."

print(f"\n🚀 STARTING MACH CHALLENGE...")
start = time.time()

response = brain.mach_solve(prompt)

total_ms = (time.time() - start) * 1000
print(f"\nSarah: {response}")

print("-" * 80)
if total_ms < 500:
    print(f"✅ SUCCESS: Total Cycle {total_ms:.2f}ms (< 500ms)")
elif total_ms < 1000:
    print(f"⚠️  PARTIAL: Total Cycle {total_ms:.2f}ms (< 1000ms)")
else:
    print(f"❌ FAIL: Total Cycle {total_ms:.2f}ms (> 1000ms)")
print("=" * 80)
