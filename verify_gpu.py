
import os
import sys
try:
    from llama_cpp import Llama
    print("llama-cpp-python version:", "unknown")
    # Check for GPU backend
    # llama_cpp.llama_supports_gpu_offload() might not exist in all versions, 
    # but we can check if it tries to load CUDA libs.
    # A better way is to try and initialize a tiny model or check the internal state.
    from llama_cpp import llama_backend_init
    llama_backend_init()
    print("Backend initialized.")
    # In newer versions, we can check llama_cpp.GGML_USE_CUDA or similar
    # But let's just try to create a dummy llama object with n_gpu_layers=1
    # if it fails or prints [CPU], we know.
except Exception as e:
    print(f"Error importing llama_cpp: {e}")

try:
    import torch
    print("Torch CUDA available:", torch.cuda.is_available())
    if torch.cuda.is_available():
        print("Torch CUDA device:", torch.cuda.get_device_name(0))
except ImportError:
    print("Torch not installed.")
