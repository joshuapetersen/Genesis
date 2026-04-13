"""
Dictionary Indexer for Sarah
Indexes English language definitions and programming terminology using ACE Token fingerprints.
"""
import os
import json
import hashlib
import lancedb
from typing import Dict, List
from Sovereign_Constants import ACE_64_BIT_MASK, SOVEREIGN_ANCHOR

VAR_HEX_RADIX = HEX_RADIX

class DictionaryIndexer:
    """
    Indexes dictionary definitions using ACE Token 64-bit fingerprints.
    Supports both English language and programming/coding terminology.
    """
    
    def __init__(self, db_path="c:\\SarahCore\\vault\\dictionary_index"):
        self.db_path = db_path
        os.makedirs(self.db_path, exist_ok=True)
        
        self.db = lancedb.connect(self.db_path)
        self.table_name = "dictionary"
        self.SOVEREIGN_ANCHOR = SOVEREIGN_ANCHOR
        
        print(f"[Dictionary Indexer] Initialized at {self.db_path}")
    
    def generate_ace_fingerprint(self, word: str) -> str:
        """
        Generates a 64-bit ACE Token fingerprint for a word.
        """
        combined = f"{word.lower()}{self.SOVEREIGN_ANCHOR}"
        hash_obj = hashlib.sha256(combined.encode())
        ace_fp = int(hash_obj.hexdigest(), HEX_RADIX) & ACE_64_BIT_MASK
        return hex(ace_fp)
    
    def load_english_dictionary(self) -> List[Dict]:
        """
        Loads English dictionary from a public API or local file.
        """
        print("[Dictionary Indexer] Loading English dictionary...")
        
        # Try to load from local cache first
        cache_file = "c:\\SarahCore\\vault\\english_dict.json"
        
        if os.path.exists(cache_file):
            print(f"[Dictionary Indexer] Loading from cache: {cache_file}")
            with open(cache_file, 'r', encoding='utf-8') as f:
                return json.load(f)
        
        # If no cache, use a basic English word list
        print("[Dictionary Indexer] Building basic English dictionary...")
        
        # Common English words with basic definitions
        # In production, you'd use a full dictionary API or dataset
        basic_words = {
            "sovereign": "possessing supreme or ultimate power; independent",
            "memory": "the faculty by which the mind stores and remembers information",
            "neural": "relating to a nerve or the nervous system",
            "orchestrator": "a person who arranges or coordinates elements to achieve a desired effect",
            "hippocampus": "a complex structure in the brain that plays a role in memory and navigation",
            "vector": "a quantity having direction as well as magnitude",
            "index": "an alphabetical list of names, subjects, etc., with references to the places where they occur",
            "retrieval": "the process of getting something back from somewhere",
            "cognitive": "relating to cognition; the mental action or process of acquiring knowledge",
            "semantic": "relating to meaning in language or logic",
            "anchor": "a person or thing that provides stability or confidence",
            "precision": "the quality of being exact and accurate",
            "resonance": "the quality in a sound of being deep, full, and reverberating",
            "temporal": "relating to time",
            "continuity": "the unbroken and consistent existence or operation of something over time"
        }
        
        entries = []
        for word, definition in basic_words.items():
            entries.append({
                "word": word,
                "definition": definition,
                "type": "english",
                "pos": "noun"  # part of speech
            })
        
        # Save to cache
        with open(cache_file, 'w', encoding='utf-8') as f:
            json.dump(entries, f, indent=2)
        
        return entries
    
    def load_coding_dictionary(self) -> List[Dict]:
        """
        Loads programming/coding terminology dictionary.
        """
        print("[Dictionary Indexer] Loading coding dictionary...")
        
        coding_terms = {
            # Python basics
            "def": "keyword used to define a function in Python",
            "class": "a blueprint for creating objects in object-oriented programming",
            "import": "statement used to bring external modules into the current namespace",
            "return": "statement used to exit a function and optionally pass back a value",
            "self": "reference to the current instance of a class in Python",
            "lambda": "anonymous function defined with the lambda keyword",
            "async": "keyword used to define asynchronous functions",
            "await": "keyword used to wait for an asynchronous operation to complete",
            
            # Data structures
            "list": "ordered, mutable collection of items in Python",
            "dict": "unordered collection of key-value pairs in Python",
            "tuple": "ordered, immutable collection of items in Python",
            "set": "unordered collection of unique items in Python",
            "array": "data structure consisting of a collection of elements",
            
            # Common programming concepts
            "algorithm": "step-by-step procedure for solving a problem or accomplishing a task",
            "recursion": "technique where a function calls itself to solve a problem",
            "iteration": "process of repeating a set of operations",
            "loop": "programming construct that repeats a block of code",
            "variable": "named storage location in memory that holds a value",
            "function": "reusable block of code that performs a specific task",
            "method": "function that belongs to a class or object",
            "parameter": "variable in a function definition",
            "argument": "actual value passed to a function when calling it",
            
            # Advanced concepts
            "vectorization": "technique of performing operations on entire arrays instead of individual elements",
            "embedding": "representation of data in a continuous vector space",
            "fingerprint": "unique identifier generated from data using a hash function",
            "hash": "function that maps data of arbitrary size to fixed-size values",
            "token": "smallest unit of meaning in text processing",
            "inference": "process of using a trained model to make predictions",
            "latency": "time delay between input and output",
            "throughput": "amount of data processed in a given time period",
            
            # Database/Storage
            "database": "organized collection of structured data",
            "index": "data structure that improves the speed of data retrieval",
            "query": "request for data from a database",
            "schema": "structure that defines the organization of data",
            "table": "collection of related data organized in rows and columns",
            
            # Sarah-specific terms
            "ace_token": "64-bit fingerprint used in Sarah's memory architecture for O(1) lookup",
            "gpis": "Gypsy Project Information System - Sarah's chronological memory framework",
            "sovereign_anchor": "mathematical constant (1.09277703703) used for precision locking",
            "hippocampus": "Sarah's memory storage and retrieval engine using LanceDB",
            "neural_orchestrator": "Sarah's core inference engine that manages LLM interactions"
        }
        
        entries = []
        for term, definition in coding_terms.items():
            entries.append({
                "word": term,
                "definition": definition,
                "type": "coding",
                "pos": "term"
            })
        
        return entries
    
    def build_index(self):
        """
        Builds the complete dictionary index with ACE Token fingerprints.
        """
        print("[Dictionary Indexer] Building dictionary index...")
        
        # Load both dictionaries
        english_entries = self.load_english_dictionary()
        coding_entries = self.load_coding_dictionary()
        
        all_entries = english_entries + coding_entries
        
        print(f"[Dictionary Indexer] Total entries: {len(all_entries)}")
        
        # Add ACE Token fingerprints
        indexed_data = []
        for entry in all_entries:
            word = entry['word']
            ace_fp = self.generate_ace_fingerprint(word)
            
            indexed_data.append({
                "ace_fingerprint": ace_fp,
                "word": word,
                "definition": entry['definition'],
                "type": entry['type'],
                "pos": entry.get('pos', 'unknown')
            })
        
        # Store in LanceDB
        print(f"[Dictionary Indexer] Writing to LanceDB...")
        
        # Drop existing table if it exists
        try:
            if self.table_name in self.db.list_tables():
                print(f"[Dictionary Indexer] Dropping existing table: {self.table_name}")
                self.db.drop_table(self.table_name)
        except Exception as e:
            print(f"[Dictionary Indexer] Warning during table drop: {e}")
        
        # Create new table
        try:
            self.db.create_table(self.table_name, data=indexed_data)
            print(f"[Dictionary Indexer] SUCCESS: {len(indexed_data)} definitions indexed")
            print(f"[Dictionary Indexer] Index location: {self.db_path}")
        except ValueError as e:
            print(f"[Dictionary Indexer] Table exists, clearing and recreating...")
            self.db.drop_table(self.table_name)
            self.db.create_table(self.table_name, data=indexed_data)
            print(f"[Dictionary Indexer] SUCCESS: {len(indexed_data)} definitions indexed")
    
    def lookup(self, word: str) -> Dict:
        """
        O(1) lookup of a word's definition using ACE Token fingerprint.
        """
        ace_fp = self.generate_ace_fingerprint(word.lower())
        
        if self.table_name not in self.db.list_tables():
            print(f"[Dictionary Indexer] Error: Index not found")
            return None
        
        table = self.db.open_table(self.table_name)
        
        try:
            results = table.search().where(f"ace_fingerprint = '{ace_fp}'").limit(1).to_list()
            return results[0] if results else None
        except Exception as e:
            print(f"[Dictionary Indexer] Lookup error: {e}")
            return None


def main():
    """
    Main entry point for dictionary indexing.
    """
    indexer = DictionaryIndexer()
    indexer.build_index()
    
    # Test lookups
    print("\n[Dictionary Indexer] Testing lookups...")
    test_words = ["sovereign", "memory", "def", "ace_token", "hippocampus"]
    
    for word in test_words:
        result = indexer.lookup(word)
        if result:
            print(f"\n[{word}] ({result['type']})")
            print(f"  Definition: {result['definition']}")
        else:
            print(f"\n[{word}] Not found")


if __name__ == "__main__":
    main()
