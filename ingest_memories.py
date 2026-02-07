import os
import sys
import time
import torch
from Sarah_Hippocampus import hippocampus

# OPTIMIZATION: Batch size for embedding generation
BATCH_SIZE = 64

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
                    if os.path.getsize(file_path) > 10 * 1024 * 1024:
                        print(f"  [SKIP] {file} is too large.")
                        continue

                    with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                        content = f.read()
                        if content.strip():
                            # Chunking
                            chunks = [content[i:i+1000] for i in range(0, len(content), 1000)]
                            
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

def process_batch(batch):
    """
    Encodes and stores a batch of memory chunks.
    """
    texts = [item["text"] for item in batch]
    try:
        # Encode batch
        vectors = hippocampus.model.encode(texts, batch_size=BATCH_SIZE, show_progress_bar=False).tolist()
        
        # Add to DB
        data = []
        for i, item in enumerate(batch):
            data.append({
                "vector": vectors[i],
                "text": item["text"],
                "role": item["role"],
                "timestamp": time.time(),
                "metadata": str(item["metadata"])
            })
            
        if hippocampus.table_name in hippocampus.db.table_names():
            table = hippocampus.db.open_table(hippocampus.table_name)
            table.add(data)
        else:
            hippocampus.db.create_table(hippocampus.table_name, data=data)
            
        print(f"[Ingest] Processed Batch ({len(batch)} items) | Device: {'GPU' if torch.cuda.is_available() else 'CPU'}")
    except Exception as e:
        print(f"[Ingest] Batch Error: {e}")

if __name__ == "__main__":
    archive_paths = [
        "c:\\SarahCore\\archive_memories",
        "c:\\SarahCore\\sarah-vscode-chat-bridge",
        "c:\\SarahCore\\sarah_chat_output.txt",
        "C:\\Users\\drago\\.gemini\\antigravity\\brain",
        "C:\\Users\\drago\\.antigravity\\extensions",
        "C:\\Users\\drago\\AppData\\Roaming\\Code\\User\\globalStorage\\google.geminicodeassist"
    ]
    
    start_time = time.time()
    for path in archive_paths:
        if os.path.exists(path):
            ingest_directory(path)
        else:
            print(f"[ERROR] Path not found: {path}")
            
    print(f"[Ingest] Omni-Library Sync Complete in {time.time() - start_time:.2f}s")
