import os
import re
import json

# Target directory containing the custom Unreal Engine game logic
SOURCE_DIR = os.path.join('Genesis_Zero', 'Source')

# Folders to explicitly skip to avoid mapping standard engine/plugin boilerplate
EXCLUDE_DIRS = {
    'ThirdParty',
    'Plugins',
    'Intermediate',
    'Binaries',
    'Saved',
    'cesium',
    '.git'
}

def analyze_cpp_file(filepath):
    """
    Parses a C++ or Header file using regex to extract classes, functions,
    and references to the Sovereign/Sarah architecture (UFUNCTIONs, etc).
    """
    structure = {
        "classes": [],
        "functions": [],
        "includes": [],
        "sovereign_anchors": [] # Places where Sarah or 9+1 math is referenced
    }
    
    try:
        with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
            
            # Find all #include statements
            includes = re.findall(r'#include\s+["<](.*?)[">]', content)
            structure["includes"] = includes
            
            # Find Class Declarations (e.g. class UGenesisBridge : public UObject)
            class_matches = re.finditer(r'(?:class|struct)\s+(?:[A-Z_A-Z0-9]+_API\s+)?([A-Z][a-zA-Z0-9_]+)(?:\s*:\s*public\s+([A-Z][a-zA-Z0-9_]+))?', content)
            for match in class_matches:
                class_name = match.group(1)
                parent_class = match.group(2) if match.group(2) else "None"
                structure["classes"].append({"name": class_name, "parent": parent_class})

            # Find common UE macros that indicate script exposure
            ufunctions = re.findall(r'UFUNCTION\s*\(.*?\)\s*(?:virtual\s+)?[\w\*<>\s:]+\s+(\w+)\s*\(', content)
            structure["functions"].extend(ufunctions)
            
            # Additional heuristic: Find anything referencing Sarah, Agent, Sovereign, or Python bridges
            important_markers = re.findall(r'(?i)(sarah|sovereign|ace_token|python_bridge|agent|neural|geometric|math)', content)
            if important_markers:
                structure["sovereign_anchors"] = list(set([m.lower() for m in important_markers]))

        return structure
    except Exception as e:
        return {"error": str(e)}

def map_unreal_codebase(root_dir):
    total_lines = 0
    file_counts = {}
    architectural_map = {}

    if not os.path.exists(root_dir):
        print(f"Error: {root_dir} does not exist.")
        return 0, {}, {}

    for dirpath, dirnames, filenames in os.walk(root_dir):
        dirnames[:] = [d for d in dirnames if not any(ex.lower() in d.lower() for ex in EXCLUDE_DIRS)]
        
        for filename in filenames:
            ext = os.path.splitext(filename)[1].lower()
            if ext in {'.h', '.cpp', '.cs'}: # .cs for Unreal Build Tool files
                filepath = os.path.join(dirpath, filename)
                rel_path = os.path.relpath(filepath, "C:\GENESIS\GenesisRUST\Sovereign_Suite_RS")
                
                try:
                    lines = 0
                    with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
                        for line in f:
                            lines += 1
                    
                    total_lines += lines
                    file_counts[ext] = file_counts.get(ext, 0) + lines
                    
                    file_info = {"lines": lines}
                    if ext in {'.h', '.cpp'}:
                        file_info["structure"] = analyze_cpp_file(filepath)
                        # Only map it if it has actual substance (classes/functions/anchors)
                        if file_info["structure"].get("classes") or file_info["structure"].get("functions") or file_info["structure"].get("sovereign_anchors"):
                            architectural_map[rel_path] = file_info
                        elif 'GameMode' in filename or 'PlayerController' in filename:
                             architectural_map[rel_path] = file_info # Always map core UE classes
                        
                except Exception as e:
                    pass

    return total_lines, file_counts, architectural_map

def generate_markdown(total_lines, counts, arch_map, output_path):
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write("# Genesis Zero: Unreal Engine Integration Map\n\n")
        f.write("> **Note:** This map explicitly analyzes the custom `Source/` directory of the Unreal Engine project to isolate Sarah's bespoke bridge logic from standard UE boilerplate and plugins.\n\n")
        
        f.write(f"**Total Custom C++ Lines Analyzed:** {total_lines:,}\n\n")
        
        f.write("## File Breakdown\n")
        for ext, count in sorted(counts.items(), key=lambda item: item[1], reverse=True):
            f.write(f"- **{ext.upper()}**: {count:,} lines\n")
            
        f.write("\n## Core C++ Classes & Bridges\n\n")
        
        sorted_files = sorted(arch_map.items(), key=lambda x: x[1]['lines'], reverse=True)
        
        for filepath, info in sorted_files:
            f.write(f"### `{filepath}` ({info['lines']:,} lines)\n")
            struct = info.get('structure', {})
            
            anchors = struct.get('sovereign_anchors', [])
            if anchors:
                f.write(f"**Sovereign Anchors Detected:** `{'`, `'.join(anchors)}`\n\n")
            
            classes = struct.get('classes', [])
            if classes:
                f.write("**Defined Classes:**\n")
                for cls in classes:
                    f.write(f"- `{cls['name']}` (Inherits: `{cls['parent']}`)\n")
            
            funcs = struct.get('functions', [])
            if funcs:
                f.write("**Exposed UFUNCTIONS:**\n")
                for func in funcs:
                    f.write(f"- `{func}()`\n")
            
            f.write("\n---\n\n")

if __name__ == "__main__":
    print("Initiating Genesis Visual Bridge Cartographer...")
    root = "C:\GENESIS\GenesisRUST\Sovereign_Suite_RS"
    source_dir = os.path.join(root, "Genesis_Zero", "Source")
    
    total_lines, counts, arch_map = map_unreal_codebase(source_dir)
    
    print(f"Total Source Lines Analyzed: {total_lines:,}")
    
    md_path = os.path.join(root, "GENESIS_CPP_ARCHITECTURE.md")
    generate_markdown(total_lines, counts, arch_map, md_path)
    print(f"Detailed C++ visual map saved to {md_path}")
