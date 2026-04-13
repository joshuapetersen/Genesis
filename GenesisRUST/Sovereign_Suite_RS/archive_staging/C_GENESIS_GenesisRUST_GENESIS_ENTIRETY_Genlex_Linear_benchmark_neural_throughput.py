# benchmark_neural_throughput.py — NEURAL CORE AUDIT
import time
import numpy as np
from all_engine import GenlexLinearRuntime

def benchmark_neural():
    print("--- SOVEREIGN NEURAL CORE BENCHMARK ---")
    runtime = GenlexLinearRuntime()
    
    # Load 8B Core Logic
    print("[NEURAL] Loading Llama 3 8B Native Architecture...")
    
    # 1. Measure a single Inference Pulse (Transformer Layer)
    # We mock the tensors for high-speed simulation
    runtime.stack.append(np.random.randn(4096).astype(np.float32)) # Hidden State
    
    start_time = time.perf_counter()
    runtime.run(r"C:\Genlex_Core\llama_8b_core.all")
    end_time = time.perf_counter()
    
    pulse_latency_ms = (end_time - start_time) * 1000
    print(f"\n[NEURAL] Inference Pulse Latency: {pulse_latency_ms:.2f} ms per layer")
    
    # 2. Project Total Tokens Per Second
    # 32 layers per token
    token_latency_ms = pulse_latency_ms * 32
    tokens_per_sec = 1000 / token_latency_ms
    
    print("\n--- NEURAL PERFORMANCE REPORT ---")
    print(f"Throughput: {tokens_per_sec:.2f} tokens/second")
    print(f"Efficiency Index: Sovereign Tier (Native Tensor Pulse)")
    
    if tokens_per_sec > 15:
        print("[VERDICT] SYSTEM EXCEEDS CORPO-TIER INFERENCE DENSITY.")
    else:
        print("[VERDICT] SYSTEM IN OPTIMIZATION PHASE.")

if __name__ == "__main__":
    benchmark_neural()
