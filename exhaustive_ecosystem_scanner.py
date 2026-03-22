import os
import ast

def scan_directory(root_dir, output_file):
    with open(output_file, 'w', encoding='utf-8') as out:
        out.write(f"# EXHAUSTIVE ECOSYSTEM SCAN: {root_dir}\n")
        out.write("This map includes native Python code, downloaded third-party tool binaries, and all cross-language scripts available to Sarah.\n\n")
        
        py_files = []
        js_ts_files = []
        shell_scripts = []
        executables = []
        
        # We will scan .venv/Scripts to see what pip-installed tools are available to Sarah
        venv_scripts_dir = os.path.join(root_dir, '.venv', 'Scripts')
        if os.path.exists(venv_scripts_dir):
            for f in os.listdir(venv_scripts_dir):
                if f.endswith('.exe') or f.endswith('.cmd') or f.endswith('.ps1'):
                    executables.append(os.path.join('.venv', 'Scripts', f))
        
        for dirpath, _, filenames in os.walk(root_dir):
            if '__pycache__' in dirpath or '.git' in dirpath:
                continue
                
            # Skip the massive site-packages pure library code, but keep the bin/tools
            if '.venv\\Lib' in dirpath:
                continue
                
            for f in filenames:
                file_path = os.path.join(dirpath, f)
                rel_path = os.path.relpath(file_path, root_dir)
                
                if f.endswith('.py') and os.path.getsize(file_path) > 0 and '.venv' not in dirpath:
                    py_files.append(rel_path)
                elif f.endswith(('.js', '.ts', '.tsx', '.jsx')) and 'node_modules' not in dirpath:
                    js_ts_files.append(rel_path)
                elif f.endswith(('.ps1', '.bat', '.cmd', '.sh')):
                    shell_scripts.append(rel_path)
                elif f.endswith('.exe') and '.venv' not in dirpath:
                    executables.append(rel_path)

        out.write(f"## 1. DOWNLOADED TOOLS & EXECUTABLES ({len(executables)})\n")
        out.write("These are the compiled binaries and CLI tools available in Sarah's environment.\n")
        for exe in sorted(executables):
            out.write(f"- `{exe}`\n")
        out.write("\n---\n\n")

        out.write(f"## 2. SHELL & AUTOMATION SCRIPTS ({len(shell_scripts)})\n")
        for sh in sorted(shell_scripts):
            out.write(f"- `{sh}`\n")
        out.write("\n---\n\n")
        
        out.write(f"## 3. JAVASCRIPT / TYPESCRIPT LAYER ({len(js_ts_files)})\n")
        for js in sorted(js_ts_files):
            out.write(f"- `{js}`\n")
        out.write("\n---\n\n")

        out.write(f"## 4. PYTHON NATIVE CORTEX ({len(py_files)})\n")
        for rel_path in sorted(py_files):
            file_path = os.path.join(root_dir, rel_path)
            out.write(f"### `{rel_path}`\n")
            
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                tree = ast.parse(content)
                
                docstring = ast.get_docstring(tree)
                if docstring:
                    out.write(f"**Description:** {docstring.strip().split('\n')[0]}\n")
                    
                classes = [node for node in tree.body if isinstance(node, ast.ClassDef)]
                functions = [node for node in tree.body if isinstance(node, ast.FunctionDef)]
                
                for cls in classes:
                    methods = [n.name for n in cls.body if isinstance(n, ast.FunctionDef)]
                    meth_str = f" methods: {', '.join(methods)}" if methods else ""
                    out.write(f"- **Class:** `{cls.name}`{meth_str}\n")
                    
                if functions:
                    func_names = [f.name for f in functions]
                    out.write(f"- **Functions:** {', '.join(func_names)}\n")
                    
            except Exception as e:
                out.write(f"_(Parse Error or Non-Standard Python)_\n")
                
            out.write("\n")

if __name__ == "__main__":
    scan_directory("C:\\SarahCore", "C:\\SarahCore\\EXHAUSTIVE_ECOSYSTEM_SCAN.md")
    print("Scan complete. Saved to C:\\SarahCore\\EXHAUSTIVE_ECOSYSTEM_SCAN.md")
