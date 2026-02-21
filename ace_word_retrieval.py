"""
ACE Word Retrieval Engine
Integrates ACE Token word-level index with Sarah's Neural Orchestrator.
Provides O(1) lookup speed for instant memory recall.
"""
import lancedb
import hashlib
from typing import List, Dict
from Sovereign_Constants import (
    ACE_64_BIT_MASK, SOVEREIGN_ANCHOR, HEX_RADIX,
    VAR_10, VAR_16, VAR_20, VAR_5
)


class AceWordRetrieval:
    """
    O(1) word lookup engine using ACE Token 64-bit fingerprints.
    Replaces slow semantic embeddings with instant hash-based retrieval.
    """
    
    def __init__(self, db_path="c:\\SarahCore\\vault\\ace_word_index"):
        self.db_path = db_path
        self.db = lancedb.connect(self.db_path)
        self.table_name = "ace_word_index"
        self.SOVEREIGN_ANCHOR = SOVEREIGN_ANCHOR
        
        # Verify table exists
        if self.table_name not in self.db.list_tables():
            raise RuntimeError(f"[ACE Retrieval] Error: Index '{self.table_name}' not found. Run ace_word_indexer.py first.")
        
        self.table = self.db.open_table(self.table_name)
        print(f"[ACE Retrieval] Loaded index with {self.table.count_rows()} words")
    
    def generate_ace_fingerprint(self, word: str, context: str = "") -> str:
        """
        Generates a 64-bit ACE Token fingerprint (as hex string).
        """
        combined = f"{word}{context}{self.SOVEREIGN_ANCHOR}"
        hash_obj = hashlib.sha256(combined.encode())
        ace_fp = int(hash_obj.hexdigest(), HEX_RADIX) & ACE_64_BIT_MASK
        return hex(ace_fp)
    
    def recall_word(self, word: str, limit: int = VAR_10) -> List[Dict]:
        """
        O(1) lookup of a word using its ACE Token fingerprint.
        Returns: List of {word, filename, line_num, context, timestamp}
        """
        ace_fp_hex = self.generate_ace_fingerprint(word.lower())
        
        try:
            results = self.table.search().where(f"ace_fingerprint = '{ace_fp_hex}'").limit(limit).to_list()
            return results
        except Exception as e:
            print(f"[ACE Retrieval] Lookup error for '{word}': {e}")
            return []
    
    def recall_phrase(self, phrase: str, limit: int = VAR_20) -> List[Dict]:
        """
        Recalls all words in a phrase and returns combined results.
        """
        words = phrase.lower().split()
        all_results = []
        
        for word in words:
            results = self.recall_word(word, limit=limit)
            all_results.extend(results)
        
        # Sort by timestamp (most recent first)
        all_results.sort(key=lambda x: x.get('timestamp', ''), reverse=True)
        
        return all_results[:limit]
    
    def recall_relevant(self, query: str, limit: int = VAR_5) -> List[Dict]:
        """
        Drop-in replacement for hippocampus.recall_relevant().
        Uses ACE Token word-level indexing instead of semantic embeddings.
        """
        results = self.recall_phrase(query, limit=limit)
        
        # Format to match hippocampus output
        formatted = []
        for r in results:
            formatted.append({
                'content': r.get('context', ''),
                'role': 'MEMORY',
                'source': r.get('filename', 'unknown'),
                'timestamp': r.get('timestamp', '')
            })
        
        return formatted


# Global Instance (replaces hippocampus for word-level recall)
try:
    ace_retrieval = AceWordRetrieval()
except RuntimeError as e:
    print(f"[ACE Retrieval] {e}")
    ace_retrieval = None
