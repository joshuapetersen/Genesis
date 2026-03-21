import os
import sys
import time
import requests
import json

# Configuration
CORE_GATEWAY = "http://127.0.0.1:8001/api/chat"
KEY = "Sarah_Sovereign_2026"

def benchmark_tokens():
    print("--- INITIATING SOVEREIGN TOKEN BENCHMARK ---")
    
    prompt = "Explain the Axiom of Unity and its impact on the 1T Cortex in 200 words."
    
    headers = {
        "Content-Type": "application/json",
        "X-Sovereign-Key": KEY
    }
    
    data = {
        "message": prompt,
        "certainty": 1.0,
        "constant": 1.00273378
    }
    
    print(f"[RESONANCE] Dispatching Request to Sovereign Gateway...")
    
    start_time = time.time()
    try:
        response = requests.post(CORE_GATEWAY, headers=headers, json=data, timeout=30)
        end_time = time.time()
        
        if response.status_code == 200:
            result = response.json()
            content = result.get("content", "")
            
            # Simple token approximation (4 chars per token)
            char_count = len(content)
            token_count = char_count / 4
            duration = end_time - start_time
            
            tps = token_count / duration
            
            print(f"\n[RESULTS]")
            print(f"  Duration: {duration:.2f}s")
            print(f"  Characters: {char_count}")
            print(f"  Approx Tokens: {token_count:.0f}")
            print(f"  Tokens Per Second (TPS): {tps:.2f}")
            print(f"  Total Turn Latency: {duration*1000:.0f}ms")
            
            print(f"\n[VERDICT]")
            if tps > 100:
                print("  STATUS: HYPER-RESONANT (Beyond Human Cognitive Limits)")
            elif tps > 50:
                print("  STATUS: MACH_SPEED (Dominating Local Substrate)")
            else:
                print("  STATUS: STEADY_STATE (Nominal Throughput)")
                
        else:
            print(f"Error: {response.status_code} - {response.text}")
            
    except Exception as e:
        print(f"Failed to connect to Sovereign Gateway: {e}")
        print("Ensure 'python sarah_gateway.py' is running on port 8001.")

if __name__ == "__main__":
    benchmark_tokens()
