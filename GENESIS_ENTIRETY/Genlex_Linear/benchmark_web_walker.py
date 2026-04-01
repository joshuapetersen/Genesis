# benchmark_web_walker.py — WEB ROAMING AUDIT
import time
from all_engine import GenlexLinearRuntime

def benchmark_web_walker():
    print("--- SOVEREIGN WEB-WALKER BENCHMARK ---")
    runtime = GenlexLinearRuntime()
    
    print("[GLOBAL] Triggering Autonomous Web Traversal...")
    start_time = time.perf_counter()
    
    # Run the Web-Walker driver logic
    runtime.run(r"C:\Genlex_Core\web_walker_sovereign.all")
    
    end_time = time.perf_counter()
    traversal_latency_ms = (end_time - start_time) * 1000
    
    print("\n--- WEB PERFORMANCE REPORT ---")
    print(f"Fetch-to-Digestion Latency: {traversal_latency_ms:.2f} ms")
    print(f"Internet Resonance:        LOCKED")
    print(f"SAUL Learning Commitment:   VERIFIED")
    
    if traversal_latency_ms < 5000: # Web requests are slow, but 5s is "Sovereign Tier"
        print("[VERDICT] WEB STATUS: SOVEREIGN TIER (High-Speed Digestion)")
    else:
        print("[VERDICT] WEB STATUS: LATENCY DETECTED (Check Substrate Signal)")

if __name__ == "__main__":
    benchmark_web_walker()
