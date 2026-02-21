"""
Coding Encyclopedia Retrieval Engine
Provides O(1) lookup for programming knowledge.
"""
import lancedb
import hashlib
from Sovereign_Constants import ACE_64_BIT_MASK, SOVEREIGN_ANCHOR, HEX_RADIX, VAR_10, VAR_16
from typing import Dict, Optional, List

class CodingKnowledge:
    """
    O(1) coding knowledge lookup using ACE Token 64-bit fingerprints.
    """
    
    def __init__(self, db_path="c:\\SarahCore\\vault\\coding_encyclopedia"):
        self.db_path = db_path
        self.db = lancedb.connect(self.db_path)
        self.table_name = "coding_knowledge"
        self.SOVEREIGN_ANCHOR = SOVEREIGN_ANCHOR
        
        # Verify table exists
        existing_tables = self.db.list_tables()
        if self.table_name not in existing_tables:
            # Try once more with direct table opening (LanceDB sometimes caches list_tables)
            try:
                self.table = self.db.open_table(self.table_name)
            except Exception:
                raise RuntimeError(f"[Coding Knowledge] Error: Index '{self.table_name}' not found. Run coding_encyclopedia_indexer.py first.")
        else:
            self.table = self.db.open_table(self.table_name)
        print(f"[Coding Knowledge] Loaded {self.table.count_rows()} entries")
    
    def generate_ace_fingerprint(self, term: str) -> str:
        """
        Generates a 64-bit ACE Token fingerprint for a term.
        """
        combined = f"{term.lower()}{self.SOVEREIGN_ANCHOR}"
        hash_obj = hashlib.sha256(combined.encode())
        ace_fp = int(hash_obj.hexdigest(), HEX_RADIX) & ACE_64_BIT_MASK
        return hex(ace_fp)
    
    def lookup(self, term: str) -> Optional[Dict]:
        """
        O(1) lookup of a coding term.
        Returns: {term, description, category, implementation, complexity, use_cases}
        """
        ace_fp = self.generate_ace_fingerprint(term.lower())
        
        try:
            results = self.table.search().where(f"ace_fingerprint = '{ace_fp}'").limit(1).to_list()
            return results[0] if results else None
        except Exception as e:
            print(f"[Coding Knowledge] Lookup error for '{term}': {e}")
            return None
    
    def explain(self, term: str) -> str:
        """
        Returns a formatted explanation of a coding term with implementation.
        """
        result = self.lookup(term)
        
        if not result:
            return f"[Coding Knowledge] '{term}' not found in index."
        
        category = result['category']
        description = result['description']
        complexity = result.get('complexity', 'N/A')
        implementation = result.get('implementation', '')
        
        output = f"[{term}] ({category})\n"
        output += f"Description: {description}\n"
        output += f"Complexity: {complexity}\n"
        
        if implementation:
            output += f"\nImplementation:\n{implementation}\n"
        
        return output
    
    def search_category(self, category: str, limit: int = VAR_10) -> List[Dict]:
        """
        Search for all entries in a specific category.
        """
        try:
            results = self.table.search().where(f"category = '{category}'").limit(limit).to_list()
            return results
        except Exception as e:
            print(f"[Coding Knowledge] Category search error: {e}")
            return []


# Global Instance
try:
    coding_knowledge = CodingKnowledge()
except RuntimeError as e:
    print(f"[Coding Knowledge] {e}")
    coding_knowledge = None
