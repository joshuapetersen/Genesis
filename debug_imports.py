import sys
import os
import TinyRuntime

print(f"Python Executable: {sys.executable}")
print(f"TinyRuntime File: {TinyRuntime.__file__}")
print(f"CWD: {os.getcwd()}")

with open(TinyRuntime.__file__, 'r') as f:
    content = f.read()
    if "[DEBUG]" in content:
        print("File on disk HAS debug prints.")
    else:
        print("File on disk DOES NOT have debug prints.")
