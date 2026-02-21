import os

target_dir = r"C:\SarahCore"

print(f"--- STARTING MASS REPAIR IN {target_dir} ---")
modified_count = 0

for root, dirs, files in os.walk(target_dir):
    # Skip non-core dirs
    if ".venv" in root or "__pycache__" in root or ".git" in root:
        continue
        
    for file in files:
        if file.endswith(".py"):
            path = os.path.join(root, file)
            try:
                with open(path, 'r', encoding='utf-8') as f:
                    lines = f.readlines()
                
                new_lines = []
                file_modified = False
                
                for line in lines:
                    if marker in line:
                        # Found the artifact, skip it (remove it)
                        file_modified = True
                        continue
                    new_lines.append(line)
                
                if file_modified:
                    with open(path, 'w', encoding='utf-8') as f:
                        f.writelines(new_lines)
                    print(f"[FIXED] {file}")
                    modified_count += 1
                    
            except Exception as e:
                print(f"[ERROR] Could not process {file}: {e}")

print(f"--- REPAIR COMPLETE. Modified {modified_count} files. ---")
