import json
import os

def parse_cluster_topics(file_path):
    """Function: parse_cluster_topics"""
    topics = []
    current_category = "General Coding Knowledge"
    
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
        
    for line in lines:
        line = line.strip()
        if not line:
            continue
            
        # Detect Category Headers (lines that don't look like specific topics)
        # In the provided text, categories often have "Cluster" or are broad titles
        # Simple heuristic: if it has no special chars and looks like a title, or is one of the known headers
        
        known_headers = [
            "Software Engineering & Advanced Architecture (Cluster II)",
            "Advanced Development Paradigms",
            "Self-Healing & Quality Layer",
            "Infrastructure & DevOps",
            "Data Engineering",
            "API & Connectivity Sovereignty",
            "Hardware & Local Optimization (Cluster III)",
            "GPU & Tensor Core Mastery", # Subsection
            "Local Hardware Interface", # Subsection
            "Mobile & Edge Sovereignty", # Subsection
            "Core Systems",
            "Programming Languages & Logic",
            "AI & Data Science",
            "Development Practices & Methodologies",
            "Advanced & Specialized Topics",
            "Logic & Abstract Concepts"
        ]
        
        is_header = False
        for header in known_headers:
            if header.lower() in line.lower():
                current_category = header
                is_header = True
                break
        
        if is_header:
            continue
            
        # Treat as topic
        topics.append({
            "term": line,
            "category": current_category,
            "description": f"Encyclopedia entry for {line} within {current_category}.",
            "language": "python", # Defaulting to python/general
            "source": "Sovereign Cluster Architecture"
        })

    return topics

if __name__ == "__main__":
    raw_path = r"C:\SarahCore\cluster_topics_raw.txt"
    json_path = r"C:\SarahCore\sarah_cluster_topics.json"
    
    if os.path.exists(raw_path):
        data = parse_cluster_topics(raw_path)
        with open(json_path, 'w', encoding='utf-8') as f:
            json.dump(data, f, indent=2)
        print(f"Extracted {len(data)} cluster topics to {json_path}")
    else:
        print(f"File not found: {raw_path}")
