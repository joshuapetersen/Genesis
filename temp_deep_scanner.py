import os
import ast

def scan_directory(root_dir, output_file):
    with open(output_file, 'w', encoding='utf-8') as out:
        out.write(f"# FULL CODEBASE AST SCAN: {root_dir}\n")
        out.write("This file is a live, machine-generated extraction of every class, function, and docstring currently active in the repository.\n\n")
        
        py_files = []
        for dirpath, _, filenames in os.walk(root_dir):
            if '.venv' in dirpath or '__pycache__' in dirpath or '.git' in dirpath:
                continue
            for f in filenames:
                file_path = os.path.join(dirpath, f)
                if f.endswith('.py') and os.path.getsize(file_path) > 0:
                    py_files.append(file_path)
                    
        out.write(f"**Total Python Files Scanned:** {len(py_files)}\n\n")
        out.write("---\n\n")
        
        for file_path in py_files:
            rel_path = os.path.relpath(file_path, root_dir)
            out.write(f"## File: `{rel_path}`\n")
            
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                try:
                    tree = ast.parse(content)
                except Exception as e:
                    out.write(f"_Error parsing AST: {e}_\n\n")
                    continue
                
                docstring = ast.get_docstring(tree)
                if docstring:
                    out.write(f"**Module Docstring:**\n```text\n{docstring.strip()}\n```\n")
                    
                classes = [node for node in tree.body if isinstance(node, ast.ClassDef)]
                functions = [node for node in tree.body if isinstance(node, ast.FunctionDef)]
                
                if not classes and not functions:
                    out.write("*No classes or functions defined. Likely a script or constant definitions.*\n\n")
                
                for cls in classes:
                    out.write(f"### Class: `{cls.name}`\n")
                    cls_doc = ast.get_docstring(cls)
                    if cls_doc:
                        out.write(f"**Docstring:** {cls_doc.strip().split('\n')[0]}\n")
                    
                    methods = [n for n in cls.body if isinstance(n, ast.FunctionDef)]
                    if methods:
                        out.write("**Methods:**\n")
                        for m in methods:
                            out.write(f"- `def {m.name}()`\n")
                    out.write("\n")
                    
                if functions:
                    out.write("### Standalone Functions:\n")
                    for func in functions:
                        out.write(f"- `def {func.name}()`\n")
                    out.write("\n")
                    
            except Exception as e:
                out.write(f"_Error reading file: {e}_\n\n")
                
            out.write("---\n\n")
            
if __name__ == "__main__":
    scan_directory("C:\\SarahCore", "C:\\SarahCore\\FULL_CODEBASE_SCAN.md")
    print("Scan complete. Saved to C:\\SarahCore\\FULL_CODEBASE_SCAN.md")
