import os
print("--- SEEDING EMBEDDING CACHE ---")
try:
    from sentence_transformers import SentenceTransformer
    # Temporarily allow online access to get the model
    os.environ["HF_HUB_OFFLINE"] = "0"
    os.environ["TRANSFORMERS_OFFLINE"] = "0"
    
    print("Downloading/Loading 'all-MiniLM-L6-v2'...")
    model = SentenceTransformer('all-MiniLM-L6-v2', device='cpu')
    print("SUCCESS: Model is now in cache.")
    
except Exception as e:
    print(f"FAILURE: {e}")
