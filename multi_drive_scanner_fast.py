import os
import ast

def scan_directories(root_dirs, output_file):
    with open(output_file, 'w', encoding='utf-8') as out:
        out.write(f"# EXHAUSTIVE MULTI-DRIVE ECOSYSTEM SCAN\n")
        out.write("This map spans the TRUE breadth of the Sovereign architectures across the disk.\n\n")
        
        py_files = []
        js_ts_files = []
        shell_scripts = []
        executables = []
        
        exclude_dirs = {'.git', '__pycache__', '$WINDOWS.~BT', '.vs', 'node_modules', '.venv', 'AppData', 'build', 'out', 'dist', 'target', 'Temp'}
        
        for root_dir in root_dirs:
            if not os.path.exists(root_dir):
                continue
                
            out.write(f"## SCANNING VOLUME: `{root_dir}`\n---\n")
            
            # Check venv scripts if any for Python tools
            venv_scripts_dir = os.path.join(root_dir, '.venv', 'Scripts')
            if os.path.exists(venv_scripts_dir):
                for f in os.listdir(venv_scripts_dir):
                    if f.endswith(('.exe', '.cmd', '.ps1')):
                        executables.append(os.path.join(root_dir, '.venv', 'Scripts', f))
            
            for dirpath, dirnames, filenames in os.walk(root_dir):
                # Prune tree in-place so we don't recurse into massive caches
                dirnames[:] = [d for d in dirnames if d not in exclude_dirs]
                    
                for f in filenames:
                    file_path = os.path.join(dirpath, f)
                    
                    if f.endswith('.py') and os.path.getsize(file_path) > 0:
                        py_files.append(file_path)
                    elif f.endswith(('.js', '.ts', '.tsx', '.jsx')):
                        js_ts_files.append(file_path)
                    elif f.endswith(('.ps1', '.bat', '.cmd', '.sh')):
                        shell_scripts.append(file_path)
                    elif f.endswith('.exe'):
                        executables.append(file_path)

        out.write(f"\n\n# GLOBAL TOTALS\n")
        out.write(f"**Compiled Tools & Binaries:** {len(executables)}\n")
        out.write(f"**Shell & Automation Scripts:** {len(shell_scripts)}\n")
        out.write(f"**JavaScript/TypeScript Nodes:** {len(js_ts_files)}\n")
        out.write(f"**Python Native Cortex Cells:** {len(py_files)}\n\n")
        out.write("---\n")
        
        out.write(f"## 1. DOWNLOADED TOOLS & EXECUTABLES\n")
        for exe in sorted(executables):
            out.write(f"- `{exe}`\n")
        out.write("\n---\n\n")

        out.write(f"## 2. SHELL & AUTOMATION SCRIPTS\n")
        for sh in sorted(shell_scripts):
            out.write(f"- `{sh}`\n")
        out.write("\n---\n\n")
        
        out.write(f"## 3. JAVASCRIPT / TYPESCRIPT LAYER\n")
        for js in sorted(js_ts_files):
            out.write(f"- `{js}`\n")
        out.write("\n---\n\n")

        out.write(f"## 4. PYTHON NATIVE CORTEX\n")
        for file_path in sorted(py_files):
            out.write(f"### `{file_path}`\n")
            
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                try:
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
                except SyntaxError:
                    out.write("_(Parsing error due to non-standard syntax or Genlex linear mapping)_\n")
                    
            except Exception as e:
                pass
            out.write("\n")

if __name__ == "__main__":
    targets = [
        r"C:\Sarah_Sidecars",
        r"C:\SarahCore.worktrees",
        r"C:\archive_memories",
        r"C:\04_THE_MEMORY",
        r"C:\05_THE_CORE",
        r"C:\SarahCore",
        r"C:\tmp",
        r"C:\OpenSource",
        r"C:\Genlex_Frequency",
        r"C:\Aethelgard",
        r"C:\Genlex_Linear",
        r"C:\Genlex_Core",
        r"C:\genlex_repo",
        r"C:\S-OS_Build",
        r"C:\Sumerian_Grid",
        r"C:\Sovereign",
        r"C:\PrimordialEarth",
        r"C:\Genesis_Bridge",
        r"C:\Sovereign_Native"
    ]
    scan_directories(targets, r"C:\SarahCore\GLOBAL_ECOSYSTEM_SCAN.md")
    print("Massive Multi-Drive Scan complete. Saved to C:\\SarahCore\\GLOBAL_ECOSYSTEM_SCAN.md")
