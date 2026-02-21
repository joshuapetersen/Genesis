import os
import re

def recover():
    """Function: recover"""
    root_dir = "C:\\SarahCore"
    skip_dirs = {'vault', '__pycache__', '.git', '.venv', 'node_modules', 'dist', 'build', '.vs', '.vscode', 'wim_mount'}
    
    # regex to match the exact pattern inserted by the fixer
    func_pattern = re.compile(r'^\s*"""Function: .*"""\s*$')
    class_pattern = re.compile(r'^\s*"""Class: .*"""\s*$')
    
    count = 0
    files_fixed = 0
    
    for root, dirs, files in os.walk(root_dir):
        dirs[:] = [d for d in dirs if d not in skip_dirs and not d.startswith('.')]
        
        for file in files:
            if file.endswith('.py'):
                file_path = os.path.join(root, file)
                try:
                    with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                        lines = f.readlines()
                    
                    new_lines = []
                    modified = False
                    for line in lines:
                        if func_pattern.match(line) or class_pattern.match(line):
                            modified = True
                            count += 1
                            continue
                        new_lines.append(line)
                    
                    if modified:
                        with open(file_path, 'w', encoding='utf-8') as f:
                            f.writelines(new_lines)
                        files_fixed += 1
                except Exception as e:
                    print(f"Error processing {file_path}: {e}")

    print(f"Recovery complete. Removed {count} bad docstrings from {files_fixed} files.")

if __name__ == "__main__":
    recover()
