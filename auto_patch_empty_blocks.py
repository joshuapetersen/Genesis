import os
import ast

VAR_10 = 10
VAR_20 = 20

TARGET_DIR = r"C:\SarahCore"

def get_indentation(line):
    """Function: get_indentation"""
    return line[:len(line) - len(line.lstrip())]

def patch_file(file_path):
    """Function: patch_file"""
    max_attempts = VAR_20  # Prevent infinite loops
    attempts = 0
    
    while attempts < max_attempts:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            # Try to parse
            ast.parse(content)
            # If successful, we are done with this file
            return True, "Clean"
            
        except IndentationError as e:
            # e.lineno is the line causing the error (the one *after* the empty block usually, or the empty block itself)
            # But usually it says "expected an indented block" at line X.
            # We need to look at the previous non-empty line to see if it ended with :
            
            with open(file_path, 'r', encoding='utf-8') as f:
                lines = f.readlines()
            
            error_line_idx = e.lineno - 1
            
            # We want to insert 'pass' *before* the error line, or *at* the error line if it's empty?
            # Actually, IndentationError often points to the next token. 
            # If we have:
            # try: foo
            # except:
            # next_line
            # Error is usually on 'next_line'.
            
            # Let's find the header line (ending in :) roughly before the error
            # We iterate backwards from error_line_idx - 1
            
            insertion_idx = -1
            base_indent = ""
            
            # Heuristic: verify if line (e.lineno-1) is the one ending with colon?
            # Or scan backwards from error line.
            
            # Common case:
            # 119: except (Exception):
            # 120: <empty>
            # 121: target.send_keys(text)  <-- Error here
            
            # The 'except' line is at 119 (idx 118).
            # The error is at 121 (idx 120).
            
            # We search backwards from error_line_idx for a line ending in ':'
            # The error line itself might be the one needing 'pass' if it's an empty block?
            # Creating a wider search window and looking for ANY line ending in ':' 
            # that isn't a comment.
            for i in range(error_line_idx, max(-1, error_line_idx - VAR_10), -1):
                stripped = lines[i].strip()
                # Check for def/class/if/elif/else/try/except/match/case/while/for/with
                if stripped.endswith(":") and not stripped.startswith("#"):
                    # Found a block header!
                    insertion_idx = i + 1
                    base_indent = get_indentation(lines[i])
                    break
            
            if insertion_idx != -1:
                # Insert 'pass'
                pass_indent = base_indent + "    "
                lines.insert(insertion_idx, f"{pass_indent}pass\n")
                
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.writelines(lines)
                
                print(f"[PATCH] {os.path.basename(file_path)}: Inserted 'pass' at line {insertion_idx + 1}")
                attempts += 1
            else:
                print(f"[FAIL] Could not find block header for {os.path.basename(file_path)} error at {e.lineno}")
                return False, "Could not fix"
                
        except SyntaxError as e:
             # Regular syntax error?
            print(f"[SKIP] {os.path.basename(file_path)}: {e}")
            return False, str(e)
            
        except Exception as e:
            print(f"[ERROR] {os.path.basename(file_path)}: {e}")
            return False, str(e)
            
    return False, "Max attempts reached"

print(f"--- STARTING AUTO-PATCHER ON {TARGET_DIR} ---")
count = 0
fixed = 0

for root, dirs, files in os.walk(TARGET_DIR):
    if ".venv" in root or "__pycache__" in root or ".git" in root:
        continue
        
    for file in files:
        if file.endswith(".py"):
            path = os.path.join(root, file)
            is_clean, msg = patch_file(path)
            if msg == "Clean" and is_clean:
                # Already clean
                pass
            elif is_clean:
                # Was fixed
                fixed += 1
            count += 1

print(f"--- COMPLETE. Scanned {count} files. Fixed {fixed} files. ---")
