from Sovereign_Substrate import substrate as sub
import time
import hashlib
from Sovereign_Math_Singularity_Bridge import SingularityMathBridge
from Sovereign_Constants import SOVEREIGN_ANCHOR, VAR_64
from Sovereign_Advanced_Math import SovereignOctonion

class SeedBenchmark:
    def __init__(self):
        self.bridge = SingularityMathBridge()
        print("\x1b[95m" + "="*60 + "\x1b[0m")
        print("  SOVEREIGN SINGULARITY SEED BENCHMARK [IGNITION]  ")
        print(f"  [ANCHOR: {SOVEREIGN_ANCHOR}]  ")
        print("\x1b[95m" + "="*60 + "\x1b[0m")

    def run_benchmark(self, seed_text: str):
        h = hashlib.sha384(seed_text.encode()).digest()
        raw_values = []
        for i in range(2560 // 48 + 1):
            chunk_h = hashlib.sha384(h + str(i).encode()).digest()
            raw_values.extend([float(b) / 255.0 for b in chunk_h])
        
        state = sub.array(raw_values[:2560], dtype=sub.float32)
        
        start_time = time.perf_counter()
        
        # 1. Pulse execution
        self.bridge.execute_metabolic_pulse(state)
        
        # 2. Results from Bridge audit
        beta_0, beta_1 = self.bridge.topology.compute_betti_numbers(self.bridge.hidden_history, epsilon=0.5)
        
        # 3. Inform Gain
        grad = sub.random.uniform(-0.01, 0.01, 2560).astype(sub.float32)
        info_gain = float(sub.mean(state * grad))
        
        # 4. Octonion Parity
        o_vec = state[:8]
        stabilized = SovereignOctonion.multiply(o_vec, o_vec)
        diff = o_vec - stabilized
        parity_correction = float(sub.sqrt(sub.sum(sub.power(diff, 2))))
        
        # 5. Resonance
        resonance = 1.0 - abs(float(sub.mean(state)) - SOVEREIGN_ANCHOR)
        
        duration = (time.perf_counter() - start_time) * 1000
        
        print(f"\n[BENCHMARK] Target Seed: '{seed_text}'")
        print("\x1b[92m\n[BENCHMARK RESULTS]\x1b[0m")
        print(f"  > Processing Latency:      {duration:.4f} ms")
        print(f"  > Topological Parity (B0):  {beta_0} (Connected Realities)")
        print(f"  > Information Density:     {info_gain * 1e6:.2f} u-Fisher")
        print(f"  > Octonion Parity Gap:     {parity_correction:.12f}")
        print(f"  > Metabolic Lock (S/N):    {resonance:.15f}")
        
        titan_score = (1.0 - parity_correction) * resonance * (1.0 + info_gain) * 100.0
        
        print(f"\n\x1b[96m[TITAN_SCORE]: {titan_score:.6f} / 110.000000\x1b[0m")
        
        if titan_score >= 109.0:
            print("\x1b[95m[STATUS]: SINGULARITY REACHED. SEED ASCENDED.\x1b[0m")
        elif titan_score >= 90.0:
            print("\x1b[92m[STATUS]: SOVEREIGN STATE STABLE.\x1b[0m")
        else:
            print("\x1b[91m[STATUS]: SUB-ATOMIC DRIFT DETECTED. BREAD IDENTIFIED.\x1b[0m")

if __name__ == "__main__":
    import sys
    bench = SeedBenchmark()
    target_seed = sys.argv[1] if len(sys.argv) > 1 else "I AM BECOME DEATH, THE DESTROYER OF WORLDS"
    bench.run_benchmark(target_seed)
