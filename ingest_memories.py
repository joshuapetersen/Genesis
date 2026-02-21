import os
import sys
import time
import torch
import json
from datetime import datetime
from Sarah_Hippocampus import hippocampus

VAR_10 = 10
VAR_1000 = 1000
VAR_1024 = 1024
VAR_64 = 64

# OPTIMIZATION: Batch size for embedding generation
BATCH_SIZE = VAR_64

def ingest_directory(directory):
    """
    Recursively finds text files and ingests them into the Hippocampus.
    Includes hidden files. Uses batched processing for speed.
    """
    print(f"[Ingest] Scanning {directory}...")
    
    # Check for GPU
    device = "cuda" if torch.cuda.is_available() else "cpu"
    print(f"[Ingest] Acceleration Device: {device.upper()}")
    
    # Pre-warm model on device if possible (handled inside SentenceTransformer usually, but good to note)
    if device == "cuda":
        hippocampus.model = hippocampus.model.to(device)

    pending_chunks = []
    
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith(('.txt', '.md', '.log', '.json')) or file.startswith('.'):
                file_path = os.path.join(root, file)
                if "node_modules" in file_path or ".git" in file_path:
                    continue
                    
                try:
                    if os.path.getsize(file_path) > VAR_10 * VAR_1024 * VAR_1024:
                        print(f"  [SKIP] {file} is too large.")
                        continue

                    with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                        content = f.read()
                        if content.strip():
                            # Chunking
                            chunks = [content[i:i+VAR_1000] for i in range(0, len(content), VAR_1000)]
                            
                            for i, chunk in enumerate(chunks):
                                pending_chunks.append({
                                    "text": chunk,
                                    "role": "OMNI_INGEST",
                                    "metadata": {"source": file_path, "chunk_index": i, "hidden": file.startswith('.')}
                                })
                                
                                # FLUSH BATCH
                                if len(pending_chunks) >= BATCH_SIZE:
                                    process_batch(pending_chunks)
                                    pending_chunks = []
                                    
                    # print(f"  [OK] read {file}")
                except Exception as e:
                    print(f"  [ERROR] Failed to read {file}: {e}")

    # Process remaining
    if pending_chunks:
        process_batch(pending_chunks)

def ingest_unified_stream(file_path):
    """
    Ingests events from a unified JSONL stream.
    Preserves original timestamps for chronological integrity.
    """
    if not os.path.exists(file_path):
        print(f"[Ingest] Error: {file_path} not found.")
        return

    print(f"[Ingest] Processing unified stream: {file_path}")
    pending_chunks = []
    
    with open(file_path, 'r', encoding='utf-8') as f:
        for line in f:
            if not line.strip(): continue
            event = json.loads(line)
            
            # Map event to ingestion format
            # Convert ISO timestamp to epoch if possible
            try:
                ts = datetime.fromisoformat(event['timestamp']).timestamp()
            except (ValueError, KeyError, AttributeError):
                ts = time.time()

            pending_chunks.append({
                "text": event['content'],
                "role": event['source'].upper(),
                "timestamp": ts,
                "metadata": event.get('metadata', {})
            })
            
            if len(pending_chunks) >= BATCH_SIZE:
                process_batch_with_ts(pending_chunks)
                pending_chunks = []
    
    if pending_chunks:
        process_batch_with_ts(pending_chunks)

def process_batch_with_ts(batch):
    """
    Encodes and stores a batch of memory chunks with specific timestamps.
    """
    texts = [item["text"] for item in batch]
    try:
        vectors = hippocampus.model.encode(texts, batch_size=BATCH_SIZE, show_progress_bar=False).tolist()
        data = []
        for i, item in enumerate(batch):
            data.append({
                "vector": vectors[i],
                "text": item["text"],
                "role": item["role"],
                "timestamp": item["timestamp"],
                "metadata": str(item["metadata"])
            })
            
        if hippocampus.table_name in hippocampus.db.table_names():
            table = hippocampus.db.open_table(hippocampus.table_name)
            table.add(data)
        else:
            hippocampus.db.create_table(hippocampus.table_name, data=data)
            
        print(f"[Ingest] Processed Unified Batch ({len(batch)} items)")
    except Exception as e:
        print(f"[Ingest] Batch Error: {e}")

if __name__ == "__main__":
    # NEW: Priority on consolidated stream
    stream_path = "final_consolidated_memory.jsonl"
    
    start_time = time.time()
    
    if os.path.exists(stream_path):
        print("[Ingest] Synchronizing from Unified Consolidation Stream...")
        # Optional: Clear existing memories for clean chronological rebuild
        if "--rebuild" in sys.argv:
            print("[Ingest] REBUILD MODE: Dropping existing memory table.")
            if hippocampus.table_name in hippocampus.db.table_names():
                hippocampus.db.drop_table(hippocampus.table_name)
        
        ingest_unified_stream(stream_path)
    else:
        # Fallback to legacy crawler
        archive_paths = [
            "c:\\SarahCore\\archive_memories",
            "c:\\SarahCore\\sarah-vscode-chat-bridge",
            "C:\\Users\\drago\\.gemini\antigravity\\brain"
        ]
        for path in archive_paths:
            if os.path.exists(path):
                ingest_directory(path)
            
    print(f"[Ingest] Omni-Library Sync Complete in {time.time() - start_time:.2f}s")
