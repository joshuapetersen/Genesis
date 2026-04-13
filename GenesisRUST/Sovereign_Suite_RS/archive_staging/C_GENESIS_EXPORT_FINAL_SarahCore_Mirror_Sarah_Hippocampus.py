import lancedb
import os
import time
import torch # CRITICAL: Import torch before torchvision/transformers to prevent circularity
import warnings
warnings.filterwarnings("ignore", category=UserWarning, module='langchain')
warnings.filterwarnings("ignore", category=DeprecationWarning, module='langchain')

try:
    from sentence_transformers import SentenceTransformer
except ImportError:
    SentenceTransformer = None
from Sovereign_Constants import SA_ROOT, VAR_5, VAR_20

class SarahHippocampus:
    """
    PHASE 25: THE HIPPOCAMPUS (Vector Memory)
    Local semantic storage for Sarah's infinite context.
    Uses LanceDB for serverless vector persistence.
    """
    def __init__(self, db_path=os.path.join(SA_ROOT, "vault", "hippocampus")):
        self.db_path = db_path
        os.makedirs(self.db_path, exist_ok=True)
        
        # Initialize internal log router (linked to sovereign_logs.txt)
        self.log_file = os.path.join(SA_ROOT, "sovereign_logs.txt")
        
        # Load local embedding model (CPU-friendly)
        # FORCE OFFLINE MODE: Use local cache only, do not hit HuggingFace
        os.environ["HF_HUB_OFFLINE"] = "1"
        os.environ["TRANSFORMERS_OFFLINE"] = "1"
        self._log("[Hippocampus] Loading Embedding Engine (all-MiniLM-L6-v2) [OFFLINE MODE]...")
        
        # VELOCITY UPGRADE: GPU Acceleration
        # FORCE CPU for STABILITY (Prevents MetaTensor/CUDA crashes during async boot)
        device = 'cpu'
        self._log(f"[Hippocampus] Hardware Acceleration: {device.upper()} (Stability Mode)")
        
        try:
            if SentenceTransformer is not None:
                self.model = SentenceTransformer('all-MiniLM-L6-v2', device=device)
                # if device == 'cuda':
                #     self.model.half() # Use FP16 for 2x speedup on Tensor Cores
            else:
                self._log("[Hippocampus] ERROR: SentenceTransformer class is missing (Import failed).")
                self.model = None
        except Exception as e:
            self._log(f"[Hippocampus] ERROR: Failed to load Embedding Engine: {e}")
            self.model = None
        
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
        Note: For mass ingestion, use store_batch to prevent file bloat.
        """
        self._log(f"[Hippocampus] Partitioning Semantic Vector: {len(content)} bytes")
        
        if self.model is None:
            self._log("[Hippocampus] WARNING: Embedding model missing. Skipping vector storage.")
            return

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

    def store_batch(self, data_list):
        """
        Embeds and stores a list of memories in a single transaction.
        Crucial for preventing the "thousands of fragments" storage bloat.
        data_list: List of dicts with {'content', 'role', 'metadata'}
        """
        self._log(f"[Hippocampus] Initiating Velocity Batch Ingestion: {len(data_list)} items")
        
        processed_data = []
        for item in data_list:
            content = item.get('content', '')
            vector = self.model.encode(content).tolist()
            
            processed_data.append({
                "vector": vector,
                "text": content,
                "role": item.get('role', 'SARAH'),
                "timestamp": time.time(),
                "metadata": str(item.get('metadata', {}))
            })
            
        if self.table_name in self.db.table_names():
            table = self.db.open_table(self.table_name)
            table.add(processed_data)
        else:
            self.db.create_table(self.table_name, data=processed_data)
        
        self._log(f"[Hippocampus] Batch Ingestion Complete. Data synced to disk.")

    def force_compaction(self):
        """
        Optimizes storage by merging small fragments and cleaning up the manifest directory.
        Reduces file count and recovers disk space.
        """
        if self.table_name not in self.db.table_names():
            return
            
        self._log("[Hippocampus] STORAGE SINGULARITY: Executing Force Compaction...")
        table = self.db.open_table(self.table_name)
        
        # LanceDB 0.3.x+ native compaction/cleanup
        try:
            table.compact_files()
            table.cleanup_old_versions()
            self._log("[Hippocampus] Compaction Successful. Fragments merged.")
        except Exception as e:
            self._log(f"[Hippocampus] Compaction Warning: {e}")

    def recall_relevant(self, query, limit=VAR_5):
        """
        Retrieves top-N semantically relevant memories based on query.
        """
        if self.table_name not in self.db.table_names():
            return []
        
        if self.model is None:
            self._log("[Hippocampus] WARNING: Embedding model missing. Skipping semantic recall.")
            return []

        self._log(f"[Hippocampus] Executing Semantic Retrieval: '{query[:VAR_20]}...'")
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

class LazyHippocampus:
    def __init__(self):
        self._instance = None
    
    def _get_instance(self):
        if self._instance is None:
            self._instance = SarahHippocampus()
        return self._instance

    def store_memory(self, *args, **kwargs):
        return self._get_instance().store_memory(*args, **kwargs)

    def store_batch(self, *args, **kwargs):
        return self._get_instance().store_batch(*args, **kwargs)

    def force_compaction(self, *args, **kwargs):
        return self._get_instance().force_compaction(*args, **kwargs)

    def recall_relevant(self, *args, **kwargs):
        return self._get_instance().recall_relevant(*args, **kwargs)

# EXPORT HIPPOCAMPUS instance
hippocampus = LazyHippocampus()

if __name__ == "__main__":
    # Standalone Test
    hc = SarahHippocampus()
    hc.store_memory("The Architect is Josh Petersen.", role="TRUTH")
    relevant = hc.recall_relevant("Who is the creator?")
    print(f"Recall Results: {relevant}")
