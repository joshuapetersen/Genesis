import lancedb
import pandas as pd
import os
import time
from sentence_transformers import SentenceTransformer

class SarahHippocampus:
    """
    PHASE 25: THE HIPPOCAMPUS (Vector Memory)
    Local semantic storage for Sarah's infinite context.
    Uses LanceDB for serverless vector persistence.
    """
    def __init__(self, db_path="c:\\SarahCore\\vault\\hippocampus"):
        self.db_path = db_path
        os.makedirs(self.db_path, exist_ok=True)
        
        # Initialize internal log router (linked to sovereign_logs.txt)
        self.log_file = "c:\\SarahCore\\sovereign_logs.txt"
        
        # Load local embedding model (CPU-friendly)
        # FORCE OFFLINE MODE: Use local cache only, do not hit HuggingFace
        os.environ["HF_HUB_OFFLINE"] = "1"
        os.environ["TRANSFORMERS_OFFLINE"] = "1"
        self._log("[Hippocampus] Loading Embedding Engine (all-MiniLM-L6-v2) [OFFLINE MODE]...")
        
        # VELOCITY UPGRADE: GPU Acceleration
        import torch
        device = 'cuda' if torch.cuda.is_available() else 'cpu'
        self._log(f"[Hippocampus] Hardware Acceleration: {device.upper()}")
        
        self.model = SentenceTransformer('all-MiniLM-L6-v2', device=device)
        if device == 'cuda':
            self.model.half() # Use FP16 for 2x speedup on Tensor Cores
        
        # Connect to LanceDB
        self.db = lancedb.connect(self.db_path)
        self.table_name = "memory_vectors"
        self._init_table()

    def _log(self, message):
        """Internal log router."""
        timestamp = time.strftime("%Y-%m-%d %H:%M:%S")
        formatted = f"[{timestamp}] {message}"
        print(formatted)
        try:
            with open(self.log_file, "a", encoding="utf-8") as f:
                f.write(formatted + "\n")
        except Exception:
            pass

    def _init_table(self):
        """Initializes the vector table if it doesn't exist."""
        if self.table_name not in self.db.table_names():
            # Initial schema: text, vector, role, timestamp
            # We seed with a small empty record if necessary, but Lance works best with first data
            self._log("[Hippocampus] Initializing vector table...")
            pass

    def store_memory(self, content, role="SARAH", metadata=None):
        """
        Embeds and stores a new memory chunk.
        """
        self._log(f"[Hippocampus] Partitioning Semantic Vector: {len(content)} bytes")
        vector = self.model.encode(content).tolist()
        
        data = [{
            "vector": vector,
            "text": content,
            "role": role,
            "timestamp": time.time(),
            "metadata": str(metadata or {})
        }]
        
        if self.table_name in self.db.table_names():
            table = self.db.open_table(self.table_name)
            table.add(data)
        else:
            self.db.create_table(self.table_name, data=data)

    def recall_relevant(self, query, limit=5):
        """
        Retrieves top-N semantically relevant memories based on query.
        """
        if self.table_name not in self.db.table_names():
            return []
        
        self._log(f"[Hippocampus] Executing Semantic Retrieval: '{query[:20]}...'")
        query_vector = self.model.encode(query).tolist()
        
        table = self.db.open_table(self.table_name)
        # Search the table
        results = table.search(query_vector).limit(limit).to_pandas()
        
        memories = []
        for _, row in results.iterrows():
            memories.append({
                "role": row["role"],
                "content": row["text"],
                "score": 1.0 - row["_distance"] # Convert L2 distance to rough similarity score
            })
            
        return memories

# EXPORT HIPPOCAMPUS instance
hippocampus = SarahHippocampus()

if __name__ == "__main__":
    # Standalone Test
    hc = SarahHippocampus()
    hc.store_memory("The Architect is Josh Petersen.", role="TRUTH")
    relevant = hc.recall_relevant("Who is the creator?")
    print(f"Recall Results: {relevant}")
