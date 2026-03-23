import os

path = r'C:\SarahCore\Sovereign_Math.py'
if os.path.exists(path):
    with open(path, 'r', encoding='utf-8', errors='ignore') as f:
        lines = f.readlines()
        
    # Search for all occurrences of def _0x_resonance
    for i, line in enumerate(lines):
        if 'def _0x_resonance' in line:
            print(f"L{i+1}: {line.strip()}")
            # Print next 5 lines
            for j in range(1, 6):
                if i+j < len(lines):
                    print(f"  L{i+1+j}: {lines[i+j].rstrip()}")
else:
    print("File not found.")
