import os
import sys
import traceback

os.environ["HF_HUB_OFFLINE"] = "1"
os.environ["TRANSFORMERS_OFFLINE"] = "1"

print("--- DIAGNOSTIC: SentenceTransformer Import ---")
try:
    import torch
    print(f"Torch Version: {torch.__version__}")
    
    from sentence_transformers import SentenceTransformer
    print("SentenceTransformer imported successfully.")
    
    print("Attempting to load model 'all-MiniLM-L6-v2' [OFFLINE]...")
    model = SentenceTransformer('all-MiniLM-L6-v2', device='cpu')
    print("Model loaded successfully.")
    
except Exception:
    print("\n[!] IMPORT/LOAD FAILURE DETECTED:")
    traceback.print_exc()

print("\n--- CACHE CHECK ---")
cache_dir = os.path.expanduser("~/.cache/huggingface/hub")
if os.path.exists(cache_dir):
    print(f"Cache Directory found: {cache_dir}")
    # List models
    for item in os.listdir(cache_dir):
        print(f" - {item}")
else:
    print("Cache Directory NOT found.")
