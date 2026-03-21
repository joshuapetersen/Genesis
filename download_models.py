"""
Sovereign Model Downloader
Downloads GGUF models for Sarah's TinyRuntime.

Target: TinyLlama-1.1B-Chat-v1.0.Q4_K_M.gguf
Size: ~638 MB
"""

import os
import sys
import time
import urllib.request
from typing import Optional

# Configuration
MODEL_URL = "https://huggingface.co/TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF/resolve/main/tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf"
MODEL_DIR = r"C:\SarahCore\models\gguf"
MODEL_FILENAME = "tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf"

def download_file(url: str, dest_path: str):
    """Download a file with progress bar."""
    print(f"Downloading {url}...")
    print(f"Destination: {dest_path}")
    
    start_time = time.time()
    
    try:
        def report(block_num, block_size, total_size):
            downloaded = block_num * block_size
            if total_size > 0:
                percent = (downloaded / total_size) * 100
                # Simple progress bar
                bar_len = 50
                filled = int(bar_len * percent / 100)
                bar = '=' * filled + '-' * (bar_len - filled)
                
                sys.stdout.write(f"\r[{bar}] {percent:.1f}% ({downloaded / 1024 / 1024:.1f} MB)")
                sys.stdout.flush()
        
        urllib.request.urlretrieve(url, dest_path, reporthook=report)
        print("\nDownload complete!")
        
        elapsed = time.time() - start_time
        size_mb = os.path.getsize(dest_path) / (1024 * 1024)
        print(f"Stats: {size_mb:.1f} MB in {elapsed:.1f}s ({size_mb/elapsed:.1f} MB/s)")
        return True
        
    except Exception as e:
        print(f"\nError downloading: {e}")
        if os.path.exists(dest_path):
            os.remove(dest_path) # Cleanup partial
        return False

def main():
    # Ensure directory exists
    if not os.path.exists(MODEL_DIR):
        print(f"Creating directory: {MODEL_DIR}")
        os.makedirs(MODEL_DIR)
        
    dest_path = os.path.join(MODEL_DIR, MODEL_FILENAME)
    
    if os.path.exists(dest_path):
        print(f"Model already exists at {dest_path}")
        size_mb = os.path.getsize(dest_path) / (1024 * 1024)
        print(f"Size: {size_mb:.1f} MB")
        
        # Optional: check if it's too small (failed download)
        if size_mb < 100:
            print("File seems too small (corrupt?). Re-downloading...")
            os.remove(dest_path)
            download_file(MODEL_URL, dest_path)
    else:
        download_file(MODEL_URL, dest_path)
        
    # Verify
    if os.path.exists(dest_path):
        print("\n[SUCCESS] Sarah's Brain is ready.")
        print(f"Path: {dest_path}")
    else:
        print("\n[FAILURE] Download failed.")

if __name__ == "__main__":
    main()
