import re
import json

VAR_3 = 3

def parse_sarah_architecture(file_path):
    """Function: parse_sarah_architecture"""
    topics = []
    
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
        
    # Slice the relevant section (approximate based on previous view)
    # We look for "Hierarchical Architecture (The 3-12-72 Logic)"
    start_index = -1
    for i, line in enumerate(lines):
        if "Hierarchical Architecture (The 3-12-72 Logic)" in line:
            start_index = i
            break
            
    if start_index == -1:
        print("Could not find start of architecture section")
        return

    # Layer 1: Input Modalities (3)
    # Look for "Layer 1:"
    idx = start_index
    while idx < len(lines):
        line = lines[idx].strip()
        
        # Layer 1
        if line.startswith("Layer 1:"):
            # Next 3 lines usually
            # But the text has format: "Voice Processing: (Orion Layer 2 / VPA)"
            # parsing until Layer 2
            idx += 1
            while idx < len(lines) and not lines[idx].strip().startswith("Layer 2:"):
                l = lines[idx].strip()
                if l and ":" in l:
                    # e.g. "Voice Processing: (Orion Layer 2 / VPA)"
                    topic_name = l.split(":")[0].strip()
                    desc = l.split(":")[1].strip(" ()")
                    topics.append({
                        "category": "Layer 1: Input Modalities",
                        "term": topic_name,
                        "description": desc,
                        "layer": 1
                    })
                idx += 1
            continue
            
        # Layer 2
        if line.startswith("Layer 2:"):
            idx += 1
            while idx < len(lines) and not lines[idx].strip().startswith("Layer 3:"):
                l = lines[idx].strip()
                if l and ":" in l:
                    topic_name = l.split(":")[0].strip()
                    desc = l.split(":")[1].strip(" ()")
                    topics.append({
                        "category": "Layer 2: Persona & Identity Orchestration",
                        "term": topic_name,
                        "description": desc,
                        "layer": 2
                    })
                idx += 1
            continue
            
        # Layer 3
        if line.startswith("Layer 3:"):
            # "Layer 3: Core Gemini Engine (72)"
            # Skip introduction lines until categories
            idx += 1
            current_category = ""
            
            while idx < len(lines):
                l = lines[idx].strip()
                
                # Stop if we hit end of section (e.g. "That 3-12-72 structure...")
                if l.startswith("That 3-12-72 structure") or l.startswith("Here is how we implement"):
                    break
                
                # Check for Category Headers e.g. "Linguistic Parsing Modules (9)"
                # Regex for "Name (Number)"
                cat_match = re.match(r"^([A-Za-z\s&]+)\s\((\d+)\)$", l)
                if cat_match:
                    current_category = cat_match.group(1).strip()
                    print(f"DEBUG: Found Category: {current_category}")
                    idx += 1
                    continue
                
                # Check for Module lines e.g. "Syntactic (Grammar, Structure, Clause Boundary)"
                if current_category and "(" in l and ")" in l:
                    print(f"DEBUG: Processing Line: {l}")
                    # Allow for lines that don't have a colon
                    # Split by first parenthesis
                    parts = l.split("(", 1)
                    module_name = parts[0].strip()
                    sub_items_str = parts[1].rsplit(")", 1)[0]
                    
                    # Generate entries for the Module itself (21 Modules * 3 = 63 topics)
                    # 1. Module Core
                    topics.append({
                        "category": f"Layer 3: {current_category}",
                        "module": module_name,
                        "term": f"{module_name} Module: Core Concepts",
                        "description": f"Overview and role of the {module_name} module within {current_category}.",
                        "layer": VAR_3
                    })
                    # 2. Module Implementation
                    topics.append({
                        "category": f"Layer 3: {current_category}",
                        "module": module_name,
                        "term": f"{module_name} Module: Architecture",
                        "description": f"Architectural patterns and data flow for the {module_name} module.",
                        "layer": VAR_3
                    })
                    # 3. Module Debugging
                    topics.append({
                        "category": f"Layer 3: {current_category}",
                        "module": module_name,
                        "term": f"{module_name} Module: Diagnostics",
                        "description": f"Auditing and troubleshooting the {module_name} module.",
                        "layer": VAR_3
                    })

                    sub_items = [x.strip() for x in sub_items_str.split(",")]
                    
                    # Add Module as a topic? Or Sub-items as topics?
                    # Recommendation: Add Sub-items as the "216" topics.
                    # Also add the module itself?
                    # Let's add the sub-items as the main entries, with context.
                    
                    for item in sub_items:
                        # Expand each component into 3 distinct encyclopedia entries to meet the 216-topic target
                        # 1. Core Concepts
                        topics.append({
                            "category": f"Layer 3: {current_category} -> {module_name}",
                            "module": module_name,
                            "term": f"{item}: Core Concepts",
                            "description": f"Fundamental principles and definitions of {item} within the {module_name} module.",
                            "layer": VAR_3
                        })
                        # 2. Implementation
                        topics.append({
                            "category": f"Layer 3: {current_category} -> {module_name}",
                            "module": module_name,
                            "term": f"{item}: Implementation Strategies",
                            "description": f"Code patterns, algorithms, and implementation details for {item}.",
                            "layer": VAR_3
                        })
                        # 3. Optimization
                        topics.append({
                            "category": f"Layer 3: {current_category} -> {module_name}",
                            "module": module_name,
                            "term": f"{item}: Optimization & Debugging",
                            "description": f"Performance tuning, error handling, and debugging techniques for {item}.",
                            "layer": VAR_3
                        })

                else:
                    print(f"DEBUG: Skipped Line: {l}")
                
                idx += 1
            break
            
        idx += 1

    # Save to JSON
    out_path = "C:\\SarahCore\\sarah_encyclopedia_topics.json"
    with open(out_path, 'w', encoding='utf-8') as f:
        json.dump(topics, f, indent=2)
        
    print(f"Extracted and expanded {len(topics)} architecture topics to {out_path}")

if __name__ == "__main__":
    parse_sarah_architecture(r"C:\SarahCore\extracted_topics.txt")
