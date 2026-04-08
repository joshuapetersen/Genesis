import os
import time
import psutil
import json
import numpy as np
import sys

# Phase 110: THE BENCHMARK AUDIT (Lattice-Scale)
# Goal: Measure absolute peak throughput of the 1,450-agent hive.

LOG_PATH = r"C:\SarahCore\sovereign_logs.txt"
TARGET_HZ = 1.092777037037037
TARGET_PERIOD = 1.0 / TARGET_HZ

def run_stability_benchmark(pulses=50):
    print(f"[ BENCHMARK ] Measuring Heartbeat Jitter across {pulses} pulses...")
    
    deltas = []
    
    # Simulation of the actual heartbeat gap to measure OS bus latency
    for i in range(pulses):
        start = time.perf_counter()
        # High-precision sleep
        time.sleep(TARGET_PERIOD)
        end = time.perf_counter()
        
        actual_period = end - start
        jitter = abs(actual_period - TARGET_PERIOD) * 1000 # ms
        deltas.append(jitter)
        if i % 10 == 0:
            print(f"  Pulse {i}: Jitter={jitter:.4f}ms")
            
    avg_jitter = np.mean(deltas)
    max_jitter = np.max(deltas)
    
    print("\n" + "-"*50)
    print(f"[ RESULT ] Target Period: {TARGET_PERIOD*1000:.4f} ms")
    print(f"[ RESULT ] Average Jitter: {avg_jitter:.4f} ms")
    print(f"[ RESULT ] Maximum Jitter: {max_jitter:.4f} ms")
    print("-"*50)
    
    return avg_jitter

def run_resource_density_benchmark():
    print("[ BENCHMARK ] Measuring Resource Density (V-110 Absolute)...")
    
    process_names = ["universality_strike.exe", "sovereign_agent.exe", "python.exe"]
    
    total_mem_mb = 0
    total_cpu_pct = 0
    agent_count = 0
    
    for proc in psutil.process_iter(['name', 'memory_info', 'cpu_percent']):
        try:
            if proc.info['name'] in process_names:
                total_mem_mb += proc.info['memory_info'].rss / (1024 * 1024)
                total_cpu_pct += proc.info['cpu_percent']
                if "sovereign_agent" in proc.info['name']:
                    agent_count += 1
        except (psutil.NoSuchProcess, psutil.AccessDenied):
            pass
            
    print("\n" + "-"*50)
    print(f"[ RESULT ] Active Agents: {agent_count}")
    print(f"[ RESULT ] Memory Usage: {total_mem_mb:.2f} MB")
    print(f"[ RESULT ] CPU Utilization: {total_cpu_pct:.1f}%")
    if agent_count > 0:
        print(f"[ RESULT ] Density: {total_mem_mb / agent_count:.3f} MB/Agent")
    print("-"*50)
    
    return total_mem_mb, total_cpu_pct

if __name__ == "__main__":
    mode = sys.argv[sys.argv.index("--mode") + 1] if "--mode" in sys.argv else "all"
    
    if mode == "stability" or mode == "all":
        run_stability_benchmark()
        
    if mode == "density" or mode == "all":
        run_resource_density_benchmark()
