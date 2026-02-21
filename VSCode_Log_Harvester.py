import os
import json
import re
import glob
from pathlib import Path

# Paths to VS Code log directories
VSCODE_BASE = Path(os.getenv('APPDATA')) / "Code"
LOGS_DIR = VSCODE_BASE / "logs"
GLOBAL_STORAGE = VSCODE_BASE / "User" / "globalStorage"
SARAH_BRIDGE_PATH = Path(r"C:\Users\drago\.vscode\extensions\yourname.sarah-vscode-chat-bridge-1.0.0\sarah_chat_output.txt")

HARVEST_OUTPUT = Path(r"c:\SarahCore\vault\scraped_content\vscode_harvest.json")

def harvest_gemini_logs():
    """
    Optimized crawler to avoid regex hangs on large files.
    """
    print("[Harvester] Searching for Gemini Logs (Optimized)...")
    gemini_data = []
    
    log_files = glob.glob(str(LOGS_DIR / "**" / "*-Gemini Code Assist.log"), recursive=True)
    
    for log_path in log_files:
        print(f"[Harvester] Parsing {log_path}...")
        try:
            with open(log_path, 'r', encoding='utf-8', errors='ignore') as f:
                for line in f:
                    # Look for JSON fragments in lines rather than reading whole file
                    # Gemini logs often have one JSON object per 'line' in some sections
                    if '"content":' in line:
                        # Simple extraction
                        match = re.search(r'"content":\s*"(.*?)"', line)
                        if match:
                            snippet = match.group(1).replace("\\n", "\n").replace("\\\"", "\"")
                            if len(snippet) > 30:
                                gemini_data.append({
                                    "text": snippet,
                                    "source": f"VSCode_Gemini_Log_{Path(log_path).parent.name}",
                                    "type": "chat_fragment"
                                })
        except Exception as e:
            print(f"[Harvester] Error reading {log_path}: {e}")
            
    return gemini_data

def harvest_copilot_cache():
    print("[Harvester] Searching for Copilot Data...")
    copilot_data = []
    api_json = GLOBAL_STORAGE / "github.copilot-chat" / "api.json"
    
    if api_json.exists():
        print(f"[Harvester] Parsing {api_json}...")
        try:
            with open(api_json, 'r', encoding='utf-8') as f:
                data = json.load(f)
                for item in data:
                    if isinstance(item, dict) and "text" in item:
                        copilot_data.append({
                            "text": item["text"],
                            "source": "VSCode_Copilot_Cache",
                            "type": "documentation_or_chat"
                        })
        except Exception as e:
            print(f"[Harvester] Copilot parse error (likely due to file size): {e}")
            # Fallback for massive api.json: line-by-line key extraction
            try:
                with open(api_json, 'r', encoding='utf-8') as f:
                    for line in f:
                        if '"text":' in line:
                             match = re.search(r'"text":\s*"(.*?)"', line)
                             if match:
                                 snippet = match.group(1).replace("\\n", "\n").replace("\\\"", "\"")
                                 copilot_data.append({
                                     "text": snippet,
                                     "source": "VSCode_Copilot_Cache_Stream",
                                     "type": "cache_fragment"
                                 })
            except:
                pass
            
    return copilot_data

def run_harvest():
    all_context = []
    all_context.extend(harvest_gemini_logs())
    all_context.extend(harvest_copilot_cache())
    
    if SARAH_BRIDGE_PATH.exists():
        print("[Harvester] Found Sarah Bridge log.")
        with open(SARAH_BRIDGE_PATH, 'r', encoding='utf-8', errors='ignore') as f:
            all_context.append({
                "text": f.read(),
                "source": "Sarah_VSCode_Bridge",
                "type": "bridge_output"
            })

    # Deduplicate
    unique_contexts = {}
    for c in all_context:
        txt = c["text"].strip()
        if len(txt) > 20 and txt not in unique_contexts:
            unique_contexts[txt] = c
    
    HARVEST_OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    with open(HARVEST_OUTPUT, 'w', encoding='utf-8') as f:
        json.dump(list(unique_contexts.values()), f, indent=2)
        
    print(f"[Harvester] Success. Harvested {len(unique_contexts)} contexts to {HARVEST_OUTPUT}")

if __name__ == "__main__":
    run_harvest()
