"""
Dictionary Retrieval Engine for Sarah
Provides O(1) lookup for English and coding term definitions.
"""
import lancedb
import hashlib
from typing import Dict, Optional
from Sovereign_Constants import ACE_64_BIT_MASK, SOVEREIGN_ANCHOR

VAR_HEX_RADIX = HEX_RADIX

class DictionaryRetrieval:
    """
    O(1) dictionary lookup using ACE Token 64-bit fingerprints.
    """
    
    def __init__(self, db_path="c:\\SarahCore\\vault\\dictionary_index"):
        self.db_path = db_path
        self.db = lancedb.connect(self.db_path)
        self.table_name = "dictionary"
        self.SOVEREIGN_ANCHOR = SOVEREIGN_ANCHOR
        
        # Verify table exists
        if self.table_name not in self.db.list_tables():
            raise RuntimeError(f"[Dictionary] Error: Index not found. Run dictionary_indexer.py first.")
        
        self.table = self.db.open_table(self.table_name)
        print(f"[Dictionary] Loaded {self.table.count_rows()} definitions")
    
    def generate_ace_fingerprint(self, word: str) -> str:
        """
        Generates a 64-bit ACE Token fingerprint for a word.
        """
        combined = f"{word.lower()}{self.SOVEREIGN_ANCHOR}"
        hash_obj = hashlib.sha256(combined.encode())
        ace_fp = int(hash_obj.hexdigest(), HEX_RADIX) & ACE_64_BIT_MASK
        return hex(ace_fp)
    
    def define(self, word: str) -> Optional[Dict]:
        """
        O(1) lookup of a word's definition.
        Returns: {word, definition, type, pos} or None
        """
        ace_fp = self.generate_ace_fingerprint(word.lower())
        
        try:
            results = self.table.search().where(f"ace_fingerprint = '{ace_fp}'").limit(1).to_list()
            return results[0] if results else None
        except Exception as e:
            print(f"[Dictionary] Lookup error for '{word}': {e}")
            return None
    
    def explain(self, word: str) -> str:
        """
        Returns a formatted explanation of a word.
        """
        result = self.define(word)
        
        if not result:
            return f"[Dictionary] '{word}' not found in index."
        
        word_type = result['type']
        definition = result['definition']
        
        return f"[{word}] ({word_type}): {definition}"


# Global Instance
try:
    dictionary = DictionaryRetrieval()
except RuntimeError as e:
    print(f"[Dictionary] {e}")
    dictionary = None
