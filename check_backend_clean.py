
import os
import sys

# Register ONLY Internal libs for Windows (avoiding 13.1 conflicts)
lib_dir = os.path.abspath(os.path.join("c:\\SarahCore", ".venv", "Lib", "site-packages", "llama_cpp", "lib"))
if os.path.exists(lib_dir):
    print(f"Registering library directory: {lib_dir}")
    os.add_dll_directory(lib_dir)
else:
    print(f"WARNING: Library directory not found at {lib_dir}")

try:
    from llama_cpp import Llama
    print("--- Initializing Llama (Library Load Check) ---")
    # This just needs to not throw an OSError on import/init
    # We use a non-existent path to trigger the backend log before failure
    llm = Llama(model_path="dummy.gguf", n_gpu_layers=-1, verbose=True)
except Exception as e:
    print(f"Result: {e}")
