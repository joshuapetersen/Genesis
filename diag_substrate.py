import ctypes
import os

# SOVEREIGN SUBSTRATE DIAGNOSTIC
# Objective: Identify the missing links in the CUDA / Llama-CPP bridge.

def diag_substrate():
    print("--- [SUBSTRATE DIAGNOSTIC] ---")
    cuda_bin = r"C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.1\bin"
    llama_lib = r"C:\Users\drago\AppData\Local\Programs\Python\Python312\Lib\site-packages\llama_cpp\lib"
    
    # 1. Check Paths
    print(f"[ PATH ] CUDA_BIN: {os.path.exists(cuda_bin)}")
    print(f"[ PATH ] LLAMA_LIB: {os.path.exists(llama_lib)}")

    if os.path.exists(llama_lib):
        print(f"[ CONTENTS ] LLAMA_LIB: {os.listdir(llama_lib)}")

    # 2. Test DLL Load
    dll_to_test = os.path.join(llama_lib, "llama.dll")
    if os.path.exists(dll_to_test):
        print(f"[ STATUS ] Found {dll_to_test}. Attempting manual load...")
        # Add CUDA to DLL search path (Python 3.8+)
        os.add_dll_directory(cuda_bin)
        try:
            ctypes.CDLL(dll_to_test)
            print("[ SUCCESS ] Llama.dll loaded with CUDA bin context.")
        except Exception as e:
            print(f"[ FAILURE ] Could not load llama.dll: {e}")
    else:
        print(f"[ CRITICAL ] llama.dll is MISSING from the library folder.")

if __name__ == "__main__":
    diag_substrate()
