import ast
import os
import sys

files_to_check = []
target_dir = r"C:\SarahCore"

print(f"--- SCANNING {target_dir} FOR SYNTAX ERRORS ---")

for root, dirs, files in os.walk(target_dir):
    # Prune explicitly to avoid traversing into heavy directories
    dirs[:] = [d for d in dirs if d not in ['.venv', 'node_modules', '.git', '__pycache__', 'site-packages', 'wim_mount', 'sandbox']]
        
    for file in files:
        if file.endswith(".py"):
            files_to_check.append(os.path.join(root, file))

print(f"Found {len(files_to_check)} Python files to verify.")


all_passed = True

for file_path in files_to_check:
    try:
        if not os.path.exists(file_path):
            print(f"[MISSING] {os.path.basename(file_path)}")
            all_passed = False
            continue
            
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
            
        try:
            ast.parse(content)
            print(f"[PASS] {os.path.basename(file_path)}")
        except SyntaxError as e:
            print(f"[FAIL] {os.path.basename(file_path)}: {e}")
            all_passed = False
    except Exception as e:
        print(f"[ERROR] {os.path.basename(file_path)}: {e}")
        all_passed = False

if all_passed:
    print("\nSUCCESS: All files represent valid Python code.")
    sys.exit(0)
else:
    print("\nFAILURE: Some files still have syntax errors.")
    sys.exit(1)
