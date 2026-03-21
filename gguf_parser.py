import os
import struct

def read_gguf_metadata(path):
    with open(path, "rb") as f:
        # GGUF Magic "GGUF" (4 bytes)
        magic = f.read(4)
        if magic != b"GGUF":
            return "Not a GGUF file"
        
        # Version (4 bytes)
        version = struct.unpack("<I", f.read(4))[0]
        
        # Tensor count (8 bytes)
        tensor_count = struct.unpack("<Q", f.read(8))[0]
        
        # KV count (8 bytes)
        kv_count = struct.unpack("<Q", f.read(8))[0]
        
        print(f"GGUF Version: {version}")
        print(f"Tensors: {tensor_count}")
        print(f"Metadata Keys: {kv_count}")

# Weights blob identified earlier
blob_path = r"C:\Users\drago\.ollama\models\blobs\sha256-6a0746a1ec1aef3e7ec53868f220ff6e389f6f8ef87a01d77c96807de94ca2aa"
read_gguf_metadata(blob_path)
