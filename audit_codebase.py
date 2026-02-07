import os
import ast
import sys

def check_file(filepath):
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            source = f.read()
        ast.parse(source)
        return True, None
    except SyntaxError as e:
        return False, f"SyntaxError in {filepath}: {e}"
    except Exception as e:
        return False, f"Error reading {filepath}: {e}"

def audit_directory(root_dir):
    print(f"Auditing Python files in {root_dir}...")
    issues = []
    count = 0
    for root, dirs, files in os.walk(root_dir):
        if ".venv" in root or "node_modules" in root or "build" in root or "dist" in root or "tests" in root:
            continue
        for file in files:
            if file.endswith(".py"):
                count += 1
                filepath = os.path.join(root, file)
                success, error = check_file(filepath)
                if not success:
                    issues.append(error)
                    print(f"[FAIL] {file}: {error}")
                else:
                    # Optional: Print success specifically for core files to reassure user
                    if file in ["Neural_Orchestrator.py", "Sarah_Chat.py", "Sarah_Memory_Vault.py"]:
                        print(f"[PASS] {file}")
    
    print(f"\nAudit Complete. Scanned {count} files.")
    if issues:
        print(f"Found {len(issues)} issues:")
        for i in issues:
            print(i)
    else:
        print("No syntax errors found.")

if __name__ == "__main__":
    audit_directory(r"C:\SarahCore")
