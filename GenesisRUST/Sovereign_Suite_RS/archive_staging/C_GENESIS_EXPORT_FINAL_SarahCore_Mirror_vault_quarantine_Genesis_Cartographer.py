import os
import ast
import json

EXCLUDE_DIRS = {
    '.git', 'vault', 'Genesis_Zero', 'node_modules', '__pycache__', 
    'venv', '.venv', 'env', '.env', '.vscode', '.idea', 'build', 'dist', 'Sovereign_Engine_Cpp',
    'lib-blockchain', 'lib-consensus', 'lib-crypto', 'lib-dht', 'lib-dns',
    'lib-economy', 'lib-identity', 'lib-network', 'lib-proofs', 'lib-protocols',
    'lib-storage', 'zhtp', 'Genesis_Zero_Backup', 'data_ingestion', '.gemini', 'tmp',
    'wim_mount', '.dotnet', 'bin', 'obj'
}
ALL_CODE_EXTENSIONS = {'.py', '.js', '.ts', '.tsx', '.jsx', '.cpp', '.hpp', '.h', '.c', '.cs', '.rs', '.bat', '.ps1', '.sh'}

def analyze_python_file(filepath):
    """Parses a Python file and returns its classes and functions."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
            tree = ast.parse(content)
        
        structure = {"classes": [], "functions": []}
        for node in tree.body:
            if isinstance(node, ast.ClassDef):
                class_info = {
                    "name": node.name,
                    "methods": [n.name for n in node.body if isinstance(n, ast.FunctionDef)]
                }
                structure["classes"].append(class_info)
            elif isinstance(node, ast.FunctionDef):
                structure["functions"].append(node.name)
        return structure
    except Exception as e:
        return {"error": str(e)}

def map_codebase(root_dir):
    total_py_lines = 0
    total_all_lines = 0
    file_counts = {}
    architectural_map = {}
    
    file_counter = 0

    for dirpath, dirnames, filenames in os.walk(root_dir):
        # Mutating dirnames in-place to skip excluded directories
        dirnames[:] = [d for d in dirnames if d not in EXCLUDE_DIRS]
        
        for filename in filenames:
            ext = os.path.splitext(filename)[1].lower()
            if ext in ALL_CODE_EXTENSIONS:
                filepath = os.path.join(dirpath, filename)
                rel_path = os.path.relpath(filepath, root_dir)
                
                try:
                    # Skip massive files (e.g. > 1 MB) to prevent hanging
                    if os.path.getsize(filepath) > 1 * 1024 * 1024:
                        continue
                        
                    lines = 0
                    with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
                        for line in f:
                            lines += 1
                    
                    total_all_lines += lines
                    file_counts[ext] = file_counts.get(ext, 0) + lines
                    
                    file_counter += 1
                    if file_counter % 50 == 0:
                        print(f"Parsed {file_counter} files... currently at {rel_path}")
                    
                    if ext == '.py':
                        total_py_lines += lines
                        file_info = {"lines": lines}
                        file_info["structure"] = analyze_python_file(filepath)
                        if "error" not in file_info["structure"]:
                            architectural_map[rel_path] = file_info
                        
                except Exception as e:
                    pass

    return total_all_lines, total_py_lines, file_counts, architectural_map

def generate_markdown_report(total_all_lines, total_py_lines, counts, arch_map, output_path):
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write("# Sovereign Map: Sarah Core Architecture\n\n")
        f.write(f"**Total Lines of Code (Relevant Languages):** {total_all_lines:,}\n")
        f.write(f"**Total Python Lines:** {total_py_lines:,}\n\n")
        
        f.write("## Language Breakdown\n")
        for ext, count in sorted(counts.items(), key=lambda item: item[1], reverse=True):
            f.write(f"- **{ext or 'unknown'}**: {count:,} lines\n")
            
        f.write("\n## Subsystems & Python Engines\n\n")
        
        # Sort files by line count to highlight the biggest engines
        sorted_files = sorted(arch_map.items(), key=lambda x: x[1]['lines'], reverse=True)
        
        for filepath, info in sorted_files:
            f.write(f"### `{filepath}` ({info['lines']:,} lines)\n")
            struct = info.get('structure', {})
            classes = struct.get('classes', [])
            functions = struct.get('functions', [])
            
            if classes:
                f.write("**Classes:**\n")
                for cls in classes:
                    f.write(f"- `{cls['name']}`\n")
                    for method in cls['methods']:
                        f.write(f"  - `{method}()`\n")
            if functions:
                f.write("**Standalone Functions:**\n")
                for func in functions:
                    f.write(f"- `{func}()`\n")
            
            f.write("\n---\n\n")

if __name__ == "__main__":
    print("Starting Sovereign Cartographer...")
    root = "C:\GenesisOS_Core"
    total_all_lines, total_py_lines, counts, arch_map = map_codebase(root)
    
    print(f"Total Lines of Code (Across All Languages): {total_all_lines:,}")
    print(f"Total Python Lines: {total_py_lines:,}")
    
    md_path = os.path.join(root, "SARAH_CORE_ARCHITECTURE.md")
    generate_markdown_report(total_all_lines, total_py_lines, counts, arch_map, md_path)
    print(f"Detailed map saved to {md_path}")
