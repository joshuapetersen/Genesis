import json
import os
from Sovereign_Constants import MEMORY_CONSOLIDATION_LIMIT

class MemoryConsolidator:
    """
    Normalizes memory events from multiple sources into a single chronological stream.
    Handles JSONL input and output.
    """
    def __init__(self, input_file="unified_memory_stream.jsonl", output_file="final_consolidated_memory.jsonl"):
        self.input_file = input_file
        self.output_file = output_file
        self.events = []

    def load_stream(self):
        """Function: load_stream"""
        if not os.path.exists(self.input_file):
            print(f"Error: {self.input_file} not found.")
            return
        
        print(f"[Consolidator] Loading events from {self.input_file}...")
        with open(self.input_file, 'r', encoding='utf-8') as f:
            for line in f:
                if line.strip():
                    self.events.append(json.loads(line))
        print(f"[Consolidator] Loaded {len(self.events)} events.")
 
    def normalize_and_sort(self):
        """Function: normalize_and_sort"""
        print("[Consolidator] Normalizing timestamps and sorting...")
        
        # 1. Deduplication based on content and source
        seen = set()
        unique_events = []
        for event in self.events:
            # Create a fingerprint from source and content
            fingerprint = (event['source'], event['content'][:MEMORY_CONSOLIDATION_LIMIT])
            if fingerprint not in seen:
                seen.add(fingerprint)
                unique_events.append(event)
        
        print(f"[Consolidator] Deduplicated: {len(self.events)} -> {len(unique_events)}")
        
        # 2. Sort by timestamp
        # Phase 13 fix for Break 25: Ensure timestamps are comparable strings/numbers
        unique_events.sort(key=lambda x: str(x.get('timestamp', '0')))
        self.events = unique_events

    def save_consolidated(self):
        """Function: save_consolidated"""
        print(f"[Consolidator] Saving final stream to {self.output_file}...")
        with open(self.output_file, 'w', encoding='utf-8') as f:
            for event in self.events:
                f.write(json.dumps(event) + '\n')
        print("[Consolidator] Final memory consolidation complete.")

if __name__ == "__main__":
    # Phase 13 fix for Break 23: Corrected class name
    consolidator = MemoryConsolidator()
    consolidator.load_stream()
    consolidator.normalize_and_sort()
    consolidator.save_consolidated()
