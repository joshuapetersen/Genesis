import os
import requests
import sys

VAR_1024 = 1024
VAR_50 = 50
VAR_8192 = 8192

# Configuration
MODELS = [
    {
        "name": "SmolLM2-135M-Instruct",
        "url": "https://huggingface.co/lmstudio-community/SmolLM2-135M-Instruct-GGUF/resolve/main/SmolLM2-135M-Instruct-Q4_K_M.gguf",
        "filename": "smollm2-135m-instruct-q4_k_m.gguf",
        "desc": "The Pattern Matcher (135M)"
    },
    {
        "name": "Qwen2.5-0.5B-Instruct",
        "url": "https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/qwen2.5-0.5b-instruct-q4_k_m.gguf",
        "filename": "qwen2.5-0.5b-instruct-q4_k_m.gguf",
        "desc": "The Logic Auditor (0.5B)"
    }
]

DEST_DIR = r"C:\SarahCore\models\disposable"

def download_file(url, dest_path, desc):
    """Function: download_file"""
    print(f"[HIVE] Fetching {desc}...")
    try:
        response = requests.get(url, stream=True)
        response.raise_for_status()
        total_size = int(response.headers.get('content-length', 0))
        block_size = VAR_8192
        downloaded = 0
        
        with open(dest_path, 'wb') as f:
            for chunk in response.iter_content(chunk_size=block_size):
                if chunk:
                    f.write(chunk)
                    downloaded += len(chunk)
                    done = int(VAR_50 * downloaded / total_size) if total_size else 0
                    sys.stdout.write(f"\r[{'=' * done}{' ' * (VAR_50-done)}] {downloaded//VAR_1024//VAR_1024}MB")
                    sys.stdout.flush()
        print(f"\n[HIVE] {desc} Acquired.")
        return True
    except Exception as e:
        print(f"\n[HIVE] Error downloading {desc}: {e}")
        return False

def main():
    """Function: main"""
    if not os.path.exists(DEST_DIR):
        os.makedirs(DEST_DIR)
        print(f"[HIVE] Created Hive directory: {DEST_DIR}")

    print("=== SOVEREIGN HIVE DOWNLOADER ===")
    print("Initiating Micro-Model Acquisition...")
    
    for model in MODELS:
        dest_path = os.path.join(DEST_DIR, model["filename"])
        if os.path.exists(dest_path):
            print(f"[HIVE] {model['name']} already active.")
        else:
            download_file(model["url"], dest_path, model["desc"])

    print("=== HIVE ASSEMBLED ===")

if __name__ == "__main__":
    main()
