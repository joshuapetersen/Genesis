import os
import time
import psutil
import subprocess
import gc
from Disposable_Agency import DisposableAgency
from Neural_Orchestrator import NeuralOrchestrator

VAR_1024 = 1024
VAR_3 = 3
VAR_50 = 50

# Ensure local libs
if os.name == 'nt':
    lib_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), ".venv", "Lib", "site-packages", "llama_cpp", "lib"))
    if os.path.exists(lib_dir):
        os.add_dll_directory(lib_dir)

def get_memory_usage():
    """Returns dict of current memory usage (RAM + VRAM estimate)."""
    mem = psutil.virtual_memory()
    usage = {
        "ram_used_gb": mem.used / (VAR_1024**VAR_3),
        "ram_percent": mem.percent
    }
    
    # Try getting VRAM via nvidia-smi
    try:
        result = subprocess.run(
            ["nvidia-smi", "--query-gpu=memory.used", "--format=csv,nounits,noheader"],
            capture_output=True, text=True
        )
        if result.returncode == 0:
            usage["vram_used_mb"] = int(result.stdout.strip())
    except (TypeError, ValueError):
        usage["vram_used_mb"] = -1
        
    return usage

def log_metrics(stage, start_mem, end_mem, duration):
    """Function: log_metrics"""
    print(f"\n[{stage}]")
    print(f"  Duration: {duration:.2f}s")
    print(f"  RAM Delta: {end_mem['ram_used_gb'] - start_mem['ram_used_gb']:.2f} GB")
    if end_mem['vram_used_mb'] != -1:
        print(f"  VRAM Delta: {end_mem['vram_used_mb'] - start_mem['vram_used_mb']} MB")
    print(f"  Current RAM: {end_mem['ram_used_gb']:.2f} GB | VRAM: {end_mem['vram_used_mb']} MB")

def test_memory_dynamics():
    """Function: test_memory_dynamics"""
    print("=== SARAH MEMORY DYNAMICS VERIFICATION ===")
    initial_mem = get_memory_usage()
    print(f"Initial State: RAM {initial_mem['ram_used_gb']:.2f} GB | VRAM {initial_mem['vram_used_mb']} MB")
    
    # 1. Test Hive Agency (SmolLM - 135M)
    print("\n--- TEST 1: HIVE (SmolLM 135M) ---")
    start_t = time.time()
    agency = DisposableAgency()
    
    # Run a task
    prompt = "Format this list: apple, banana, cherry"
    print(f"  Prompt: '{prompt}'")
    result = agency.run_mission("smollm", prompt)
    print(f"  Result: {result['result'][:VAR_50]}...")
    
    end_t = time.time()
    mid_mem = get_memory_usage()
    log_metrics("Hive Execution", initial_mem, mid_mem, end_t - start_t)
    
    # Verification: Did it Annihilate?
    # We check if memory returned close to initial
    # Note: Python GC might be lazy, so exact return isn't guaranteed, but large chunks should go.
    
    # 2. Test Hive Agency (Qwen - 0.5B)
    print("\n--- TEST 2: HIVE (Qwen 0.5B) ---")
    start_t = time.time()
    prompt = "Summarize the concept of gravity in one sentence."
    print(f"  Prompt: '{prompt}'")
    result = agency.run_mission("qwen", prompt)
    print(f"  Result: {result['result'][:VAR_50]}...")
    
    end_t = time.time()
    end_mem = get_memory_usage()
    log_metrics("Hive Execution (Qwen)", mid_mem, end_mem, end_t - start_t)
    
    # 3. Test Neural Orchestrator (8B Loading) takes time
    # This is the "heavy" test.
    print("\n--- TEST 3: ORCHESTRATOR (8B Load) ---")
    print("  Loading 8B Model... (Expect VRAM Spike)")
    start_t = time.time()
    
    # We load it but don't run inference to save time/risk, or run one simple prompt
    # Actually, let's run a simple prompts via dispatch to test the audit too.
    orch = NeuralOrchestrator(draft_model=None) # No speculative decoding
    orch.inject_hive(agency)
    
    load_end_t = time.time()
    load_mem = get_memory_usage()
    log_metrics("8B Load", end_mem, load_mem, load_end_t - start_t)
    
    # Run 8B Inference (Precision Audit)
    print("\n--- TEST 4: 8B INFERENCE (Precision Audit) ---")
    prompt = "What is the Sovereign Frequency?"
    response, latency = orch.dispatch(prompt)
    print(f"  Result: {response}")
    
    infer_end_t = time.time()
    infer_mem = get_memory_usage()
    log_metrics("8B Inference", load_mem, infer_mem, infer_end_t - load_end_t)
    
    # Cleanup
    print("\n--- CLEANUP ---")
    del orch
    del agency
    gc.collect()
    final_mem = get_memory_usage()
    print(f"Final State: RAM {final_mem['ram_used_gb']:.2f} GB | VRAM {final_mem['vram_used_mb']} MB")
    
if __name__ == "__main__":
    test_memory_dynamics()
