import time

print("Profiling Sovereign Math...")
start = time.time()
from Sovereign_Math import SovereignMath
print(f"Import time: {(time.time() - start)*1000:.2f}ms")

start = time.time()
math_engine = SovereignMath()
print(f"Init time: {(time.time() - start)*1000:.2f}ms")

prompt = "Solve the 11GB singularity paradox relative to volumetric C3."
start = time.time()
vec = math_engine._0x_expand(prompt)
print(f"Expand time: {(time.time() - start)*1000:.2f}ms")

start = time.time()
density = math_engine.calculate_theory_density(prompt)
print(f"Density time: {(time.time() - start)*1000:.2f}ms")
