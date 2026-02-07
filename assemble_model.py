
import os
import glob

MODEL_NAME = "models/dolphin-2.9-llama3-8b-q4_K_M.gguf"

def assemble_file(output_path):
    parts = sorted(glob.glob(f"{output_path}.part*"))
    if not parts:
        print(f"No parts found for {output_path}")
        return

    print(f"Assembling {output_path} from {len(parts)} parts...")
    
    with open(output_path, 'wb') as outfile:
        for part in parts:
            print(f"Reading {part}...")
            with open(part, 'rb') as infile:
                outfile.write(infile.read())
                
    print(f"Assembly complete: {output_path}")

if __name__ == "__main__":
    assemble_file(MODEL_NAME)
