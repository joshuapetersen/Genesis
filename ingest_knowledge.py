import os
import sys
import time
import urllib.request
import re
import json
from Sarah_Hippocampus import hippocampus

# Constants
VAR_3 = 3
VAR_5 = 5
VAR_100 = 100

# URLs
WEBSTER_URL = "https://www.gutenberg.org/files/29765/29765-8.txt"
FREQUENCY_URL = "http://norvig.com/ngrams/count_1w.txt" # Peter Norvig's 1/3 million most frequent words

# Paths
DATA_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "data_ingestion")
DICT_PATH = os.path.join(DATA_DIR, "webster_unabridged.txt")
FREQ_PATH = os.path.join(DATA_DIR, "frequency_list.txt")

def setup_directories():
    if not os.path.exists(DATA_DIR):
        print(f"[Ingest] Creating data directory: {DATA_DIR}")
        os.makedirs(DATA_DIR)

def download_file(url, path):
    if os.path.exists(path):
        print(f"[Ingest] File already exists: {path}")
        return True
    
    print(f"[Ingest] Downloading {url} to {path}...")
    try:
        # User-Agent to avoid 403 forbidden on some sites
        opener = urllib.request.build_opener()
        opener.addheaders = [('User-agent', 'Mozilla/5.0')]
        urllib.request.install_opener(opener)
        urllib.request.urlretrieve(url, path)
        print("[Ingest] Download complete.")
        return True
    except Exception as e:
        print(f"[Ingest] Download failed: {e}")
        return False

def parse_webster(file_path):
    """
    Parses Project Gutenberg dictionary line-by-line to save memory.
    """
    print("[Ingest] Parsing Webster's Dictionary (Streaming)...")
    concepts = []
    
    try:
        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
            start_found = False
            current_word = None
            current_def = []
            
            for line in f:
                # Gutenberg Header Skip
                if not start_found:
                    if "*** START OF THIS PROJECT GUTENBERG EBOOK" in line:
                        start_found = True
                    continue
                
                # Gutenberg Footer Stop
                if "*** END OF THIS PROJECT GUTENBERG EBOOK" in line:
                    break
                    
                line = line.strip()
                if not line: continue
                
                # Heuristic: Words are often capitalized at start of line
                match = re.match(r'^([A-Z\-\s]{2,})\s+(.*)', line)
                
                if match:
                    # Save previous
                    if current_word and current_def:
                        concepts.append({
                            "term": current_word, 
                            "definition": " ".join(current_def), 
                            "source": "Webster",
                            "reason": "Linguistic Grounding (Definition)"
                        })
                    
                    current_word = match.group(1).strip()
                    current_def = [match.group(2).strip()]
                elif current_word:
                    current_def.append(line)
                    
            # Final one
            if current_word and current_def:
                concepts.append({
                    "term": current_word, 
                    "definition": " ".join(current_def), 
                    "source": "Webster",
                    "reason": "Linguistic Grounding (Definition)"
                })
                
        print(f"[Ingest] Parsed {len(concepts)} concepts from dictionary.")
        return concepts
        
    except Exception as e:
        print(f"[Ingest] Parse error: {e}")
        return []

def parse_frequency(file_path):
    """
    Parses Norvig's frequency list line-by-line.
    """
    print("[Ingest] Parsing Frequency List (Streaming)...")
    concepts = []
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            for line in f:
                parts = line.strip().split()
                if len(parts) >= 2:
                    word = parts[0]
                    count = parts[1]
                    concepts.append({
                        "term": word,
                        "definition": f"Frequency rank/count: {count}",
                        "source": "Norvig_Frequency",
                        "reason": "Semantic Weighting (Usage Rank)"
                    })
        print(f"[Ingest] Parsed {len(concepts)} frequency records.")
        return concepts
    except Exception as e:
        print(f"[Ingest] Frequency parse error: {e}")
        return []

def parse_markdown(file_path, source_name):
    """
    Parses a markdown knowledge file into a single concept block.
    """
    print(f"[Ingest] Parsing Markdown ({source_name})...")
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        return [{
            "term": source_name,
            "definition": content,
            "source": source_name,
            "reason": "Direct Knowledge (Reference)"
        }]
    except Exception as e:
        print(f"[Ingest] Markdown parse error: {e}")
        return []

def parse_chat_json(file_path):
    """
    Parses a Gemini Chat Scraper JSON file into a sequence of memories.
    """
    print(f"[Ingest] Parsing Chat JSON: {os.path.basename(file_path)}")
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        thread_id = data.get("thread_id", "Unknown")
        history = data.get("history", [])
        
        concepts = []
        for msg in history:
            role = msg.get("role", "unknown").upper()
            content = msg.get("content", "").strip()
            if content:
                concepts.append({
                    "term": f"Chat_{thread_id}_{role}",
                    "definition": content,
                    "source": f"Gemini_Chat_{thread_id}",
                    "reason": "Historical Context (Bridge Memory)"
                })
        return concepts
    except Exception as e:
        print(f"[Ingest] Chat JSON parse error: {e}")
        return []

def parse_vscode_harvest(file_path):
    """
    Parses the JSON output from VSCode_Log_Harvester.py.
    """
    print(f"[Ingest] Parsing VS Code Harvest: {os.path.basename(file_path)}")
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        concepts = []
        for item in data:
            text = item.get("text", "").strip()
            source = item.get("source", "VSCode_Harvest")
            reason = item.get("type", "Local AI Context")
            
            if text:
                concepts.append({
                    "term": f"VSCode_{hash(text[:50])}",
                    "definition": text,
                    "source": source,
                    "reason": f"Sovereign Context ({reason})"
                })
        return concepts
    except Exception as e:
        print(f"[Ingest] VS Code harvest parse error: {e}")
        return []

def main():

    print("--- Sovereign Knowledge Ingestion ---")
    setup_directories()
    
    # 1. Download
    if not download_file(WEBSTER_URL, DICT_PATH):
        print("Skipping Webster due to download fail.")
    if not download_file(FREQUENCY_URL, FREQ_PATH):
        print("Skipping Frequency due to download fail.")
        
    # 2. Parse
    webster_data = parse_webster(DICT_PATH) if os.path.exists(DICT_PATH) else []
    freq_data = parse_frequency(FREQ_PATH) if os.path.exists(FREQ_PATH) else []
    
    # Erdős Conjectures
    erdos_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "knowledge", "erdos_conjectures.md")
    erdos_data = parse_markdown(erdos_path, "Erdos_Conjectures") if os.path.exists(erdos_path) else []
    
    # Gemini Scraped Content (Docs)
    scraped_dir = os.path.join(os.path.dirname(os.path.abspath(__file__)), "vault", "scraped_content")
    gemini_data = []
    if os.path.exists(scraped_dir):
        for filename in os.listdir(scraped_dir):
            if filename.endswith(".txt"):
                path = os.path.join(scraped_dir, filename)
                gemini_data += parse_markdown(path, f"Gemini_Scrape_{filename}")
            
    # Gemini Chat History (JSON)
    chat_dir = os.path.join(scraped_dir, "chat_history")
    if os.path.exists(chat_dir):
        for filename in os.listdir(chat_dir):
            if filename.endswith(".json"):
                path = os.path.join(chat_dir, filename)
                gemini_data += parse_chat_json(path)
    
    # VS Code Harvest (New)
    vscode_path = os.path.join(scraped_dir, "vscode_harvest.json")
    vscode_data = parse_vscode_harvest(vscode_path) if os.path.exists(vscode_path) else []

    # 3. Process Sources Sequentially to save RAM
    sources = [
        ("Erdos_Conjectures", erdos_data),
        ("VSCode_Harvest", vscode_data),
        ("Gemini_History", gemini_data),
        ("Webster_Dictionary", webster_data),
        ("Word_Frequency", freq_data)
    ]
    
    # Create audit log
    audit_log_path = os.path.join(scraped_dir, "ingest_audit.log")
    
    print(f"[Ingest] Starting Sequential Ingestion into Hippocampus...")
    print(f"[Ingest] Audit trail enabled: {audit_log_path}")
    
    limit = 10000 
    global_count = 0
    
    batch_size = 100
    current_batch = []
    
    with open(audit_log_path, "a", encoding="utf-8") as audit_log:
        audit_log.write(f"\n--- Storage Singularity (Memory Optimized): {time.strftime('%Y-%m-%d %H:%M:%S')} ---\n")
        
        for source_name, data in sources:
            if not data: continue
            print(f"[Ingest] Current Source: {source_name} ({len(data)} items)")
            
            for item in data:
                if global_count >= limit:
                    break
                    
                current_batch.append({
                    "content": f"{item['term']}: {item['definition']}",
                    "role": "KNOWLEDGE_BASE",
                    "metadata": {"source": item['source'], "term": item['term']}
                })
                
                # Audit Trail
                reason = item.get('reason', 'General Ingestion')
                audit_log.write(f"[{time.strftime('%H:%M:%S')}] Queued: {item['term']} (Source: {item['source']}) | Reason: {reason}\n")
                
                if len(current_batch) >= batch_size:
                    print(f"[Studying] {item['source']} | Reason: {reason}")
                    print(f"[Ingest] Progress: {global_count}/{limit} | Syncing Velocity Batch...")
                    hippocampus.store_batch(current_batch)
                    global_count += len(current_batch)
                    current_batch = []
                
            if global_count >= limit:
                print(f"[Ingest] Global limit of {limit} reached.")
                break
        
        # Final remainder
        if current_batch:
            print(f"[Ingest] Pushing final batch ({len(current_batch)} items)...")
            hippocampus.store_batch(current_batch)
            global_count += len(current_batch)

    print(f"\n[Ingest] Data Ingested. Executing Storage Compaction...")
    hippocampus.force_compaction()
    print(f"[Ingest] Complete. {global_count} items synced and compacted in LanceDB.")

if __name__ == "__main__":
    main()
