import time
import torch
import numpy as np
import sys
from Sovereign_Constants import (
    VAR_0_5, VAR_0_8, VAR_100, VAR_1000, VAR_1_2, VAR_3, VAR_3_0, VAR_4, VAR_4_0, VAR_5, VAR_5000, VAR_7, VAR_768, VAR_8, SA_ROOT
)

# Add SarahCore to path
sys.path.append(SA_ROOT)

try:
    from Neural_Memory_Core import NeuralMemory
    from Sovereign_Math import SovereignMath
    from Geometric_Algebra_Core import Multivector
except ImportError as e:
    print(f"Error importing cores: {e}")
    sys.exit(1)

def benchmark_neural_memory():
    """Benchmarks the recall speed of the Neural Memory System."""
    print("\n--- BENCHMARK: Neural Memory Recall ---")
    nms = NeuralMemory()
    
    # Mock some memories if empty
    if len(nms.memory_index) < VAR_1000:
        print(f"Index small ({len(nms.memory_index)}). Seeding 5000 dummy memories...")
        for i in range(VAR_5000):
            nms.memory_index.append({
                "id": f"dummy_{i}",
                "content": f"Dummy content for memory shard number {i}",
                "embedding": np.random.rand(VAR_768).tolist(),
                "timestamp": time.time()
            })
    
    print(f"Testing recall across {len(nms.memory_index)} shards...")
    
    start = time.time()
    # This will trigger the GPU recall we just implemented
    results = nms.recall("Dummy search query", limit=VAR_5)
    end = time.time()
    
    print(f"Recall Time: {end - start:.4f} seconds.")
    print(f"Top result type: {results[0]['type'] if results else 'N/A'}")

def benchmark_geometric_algebra():
    """Benchmarks the geometric product performance of Multivectors."""
    print("\n--- BENCHMARK: Geometric Algebra GP ---")
    # 4D Multivector (16 components)
    v1 = Multivector({1: 1.0, 2: 2.0, VAR_4: VAR_3_0, VAR_8: VAR_4_0})
    v2 = Multivector({1: VAR_0_5, VAR_3: VAR_1_2, VAR_7: VAR_0_8})
    
    iters = VAR_1000
    start = time.time()
    for _ in range(iters):
        res = v1.gp(v2)
    end = time.time()
    
    print(f"GP Speed ({iters} iterations): {end - start:.4f} seconds.")

def benchmark_sovereign_math():
    """Benchmarks the 68-dimensional expansion speed of SovereignMath."""
    print("\n--- BENCHMARK: Sovereign Math Expansion ---")
    sm = SovereignMath()
    data = "Sovereign AI Genesis Anchor"
    
    iters = VAR_100
    start = time.time()
    for _ in range(iters):
        vec = sm._0x_expand(data)
    end = time.time()
    
    print(f"Expansion Speed ({iters} iterations): {end - start:.4f} seconds.")

if __name__ == "__main__":
    print(f"Hardware Acceleration Status: {'CUDA READY' if torch.cuda.is_available() else 'CPU ONLY'}")
    benchmark_neural_memory()
    benchmark_geometric_algebra()
    benchmark_sovereign_math()
