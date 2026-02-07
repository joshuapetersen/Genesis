import requests
import re
import sys
import subprocess
import os

# Base URL for the wheels index
INDEX_URL = "https://jllllll.github.io/llama-cpp-python-cuBLAS-wheels/AVX2/cu122/llama-cpp-python/" # Added subfolder
# If that fails, could try cu121

def install_wheel():
    print(f"Fetching wheel list from {INDEX_URL}...")
    try:
        html = requests.get(INDEX_URL).text
        # Relaxed Regex to find ANY cp312 windows wheel
        pattern = r'href="(llama_cpp_python-.*?cp312-cp312-win_amd64\.whl)"'
        match = re.search(pattern, html)
        
        if not match:
            print("No matching wheel found for CP312 Windows AMD64.")
            print("HTML Content excerpt:", html[:500])
            return

        wheel_name = match.group(1)
        wheel_url = INDEX_URL + wheel_name if wheel_name in requests.get(INDEX_URL).text else fallback_url + wheel_name
        
        print(f"Found wheel: {wheel_name}")
        print(f"Downloading from {wheel_url}...")
        
        # Download
        r = requests.get(wheel_url)
        with open(wheel_name, 'wb') as f:
            f.write(r.content)
            
        print("Installing wheel...")
        subprocess.check_call([sys.executable, "-m", "pip", "install", wheel_name, "--force-reinstall"])
        
        print("Installation Successful.")
        
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    install_wheel()
