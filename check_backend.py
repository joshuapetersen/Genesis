
import os
import sys

# Register CUDA DLLs (similar to Neural_Orchestrator.py)
cuda_bin = os.path.join("C:\\", "Program Files", "NVIDIA GPU Computing Toolkit", "CUDA", "v13.1", "bin", "x64")
if os.path.exists(cuda_bin):
    print(f"Registering DLL directory: {cuda_bin}")
    os.add_dll_directory(cuda_bin)
else:
    print(f"WARNING: CUDA v13.1 bin path not found at {cuda_bin}")

try:
    from llama_cpp import Llama
    print("--- Initializing Llama with verbose=True ---")
    # This will still likely fail because of model_path, but it verifies if the LIB loads
    llm = Llama(model_path="dummy.gguf", n_gpu_layers=-1, verbose=True)
except Exception as e:
    print(f"Result: {e}")
