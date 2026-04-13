import requests
import sys
import os
import time

VAR_100 = 100
VAR_1024 = 1024
VAR_8192 = 8192

# Target: 1B Instruct Model (Quantized Q4_K_M) - ~800MB
# Source: bartowski (Reliable Quantizer)
URL = "https://huggingface.co/bartowski/Llama-3.2-1B-Instruct-GGUF/resolve/main/Llama-3.2-1B-Instruct-Q4_K_M.gguf"
DEST = r"C:\SarahCore\models\Llama-3.2-1B-Instruct-Q4_K_M.gguf"

def download_worker():
    """Function: download_worker"""
    if os.path.exists(DEST):
        print(f"[Worker] Model already exists at {DEST}")
        return

    print(f"[Worker] Initiating download of 1B Unit...")
    print(f"Target: {URL}")
    print(f"Destination: {DEST}")
    
    try:
        response = requests.get(URL, stream=True)
        response.raise_for_status()
        total_size = int(response.headers.get('content-length', 0))
        
        with open(DEST, 'wb') as f:
            downloaded = 0
            start_time = time.time()
            for chunk in response.iter_content(chunk_size=VAR_8192):
                if chunk:
                    f.write(chunk)
                    downloaded += len(chunk)
                    
                    # Progress Bar
                    if downloaded % (VAR_1024 * VAR_1024) == 0: # Every 1MB for smooth update
                        mb = downloaded / (VAR_1024 * VAR_1024)
                        total_mb = total_size / (VAR_1024 * VAR_1024)
                        elapsed = time.time() - start_time
                        speed = mb / elapsed if elapsed > 0 else 0
                        percent = (downloaded / total_size) * VAR_100 if total_size > 0 else 0
                        sys.stdout.write(f"\r[Downloading] {percent:.1f}% | {mb:.1f} MB / {total_mb:.1f} MB ({speed:.2f} MB/s)")
                        sys.stdout.flush()
                        
        print("\n[Worker] Download Complete. 1B Unit Online.")
    except Exception as e:
        print(f"\n[Worker] FATAL: Download failed. Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    download_worker()
