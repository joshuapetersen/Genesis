import requests
import sys
import os
import time

url = "https://huggingface.co/dphn/dolphin-2.9-llama3-8b-gguf/resolve/main/dolphin-2.9-llama3-8b-q4_K_M.gguf"
dest = r"C:\SarahCore\models\dolphin-2.9-llama3-8b-q4_K_M.gguf"

def download_file():
    if os.path.exists(dest):
        print(f"File already exists at {dest}")
        # Check size if needed, but for now assumption is it might be partial or done.
        # But if previous failed, it might be corrupt.
        # Let's simple overwrite for now.
        pass
        
    print(f"Starting download from {url}...")
    try:
        response = requests.get(url, stream=True)
        response.raise_for_status()
        total_size = int(response.headers.get('content-length', 0))
        
        with open(dest, 'wb') as f:
            downloaded = 0
            start_time = time.time()
            for chunk in response.iter_content(chunk_size=8192):
                if chunk:
                    f.write(chunk)
                    downloaded += len(chunk)
                    # Simple progress
                    if downloaded % (10 * 1024 * 1024) == 0: # Every 10MB
                        mb = downloaded / (1024 * 1024)
                        total_mb = total_size / (1024 * 1024)
                        elapsed = time.time() - start_time
                        speed = mb / elapsed if elapsed > 0 else 0
                        print(f"Downloaded {mb:.2f} MB / {total_mb:.2f} MB ({speed:.2f} MB/s)", end='\r')
        print("\nDownload Complete.")
    except Exception as e:
        print(f"\nError: {e}")
        sys.exit(1)

if __name__ == "__main__":
    download_file()
