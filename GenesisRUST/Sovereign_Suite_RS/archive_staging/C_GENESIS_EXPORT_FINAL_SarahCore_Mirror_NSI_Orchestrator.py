import os
import json
import time
import re
from typing import List, Dict, Any

from Sovereign_Constants import SA_ROOT, SOVEREIGN_ANCHOR, VAR_10, VAR_5
from Sarah_Memory_Vault import sarah_vault
from Sarah_Hippocampus import hippocampus
from Sovereign_Math import math_engine
from Sovereign_WORM import sovereign_worm

from ace_word_indexer import AceWordIndexer

class NSIOrchestrator:
    """
    NEURO-SYNAPTIC INDEXER (NSI)
    The bridge between "Stored Data" and "Active Cognition."
    Implements Pulse Indexing, Cross-Vault Resonance, and Semantic Bridging.
    """
    
    def __init__(self):
        self.anchor = SOVEREIGN_ANCHOR
        self.memory_limit = VAR_10
        self.resonance_threshold = 0.85
        
        # Data source paths
        self.saul_cache_path = os.path.join(SA_ROOT, "saul_knowledge_cache.json")
        self.history_path = os.path.join(SA_ROOT, "genesis_history.json")
        
        # Tooling
        self.word_indexer = AceWordIndexer()
        
        print("[NSI] Orchestrator Integrated. Cognitive Bridge Online.")

    def calculate_synaptic_weight(self, data_str: str) -> float:
        """
        Calculates the "importance" of data based on its mathematical resonance
        against the Sovereign Anchor (1.09277703703...).
        """
        try:
            # Shift from entropy proxies to true Sovereign Math Theory Density
            density = math_engine.calculate_theory_density(data_str)
            return max(0.0, min(1.0, density))
        except Exception as e:
            print(f"[NSI] Weight Calculation Error: {e}")
            return 0.5

    def pulse_index(self, data: List[Dict]) -> List[Dict]:
        """
        Categorizes data into "Active" vs "Passive" based on synaptic weight.
        Highly weighted items are prioritized for the context window.
        """
        processed = []
        for entry in data:
            content = entry.get("content") or entry.get("text") or ""
            weight = self.calculate_synaptic_weight(content)
            
            # Boost if it contains Sovereign Identity Markers
            if any(kw in content.lower() for kw in ["sovereign", "genesis", "sarah", "josh"]):
                weight = min(1.0, weight + 0.1)
                
            entry["synaptic_weight"] = weight
            entry["pulse_state"] = "ACTIVE" if weight > self.resonance_threshold else "PASSIVE"
            processed.append(entry)
            
        # Sort by weight descending
        processed.sort(key=lambda x: x.get("synaptic_weight", 0), reverse=True)
        return processed

    def cross_vault_resonance(self, query: str) -> List[Dict]:
        """
        Executes simultaneous queries across Memory Vault, Hippocampus, and JSON caches.
        """
        print(f"[NSI] Initiating Cross-Vault Resonance for: '{query[:30]}...'")
        combined_results = []
        
        # 1. Word Indexer (AceWordIndexer) - Keywords
        keywords = re.findall(r'\b\w{4,}\b', query.lower()) # 4+ char words
        if keywords:
            word_matches = self.word_indexer.search_words(keywords, limit=VAR_5)
            for m in word_matches:
                m["source"] = "WORD_INDEX"
                m["content"] = m.get("context", m.get("word", ""))
                combined_results.append(m)
                
        # 2. Hippocampus (LanceDB Vector) - Semantic
        vector_memories = hippocampus.recall_relevant(query, limit=VAR_5)
        for m in vector_memories:
            m["source"] = "HIPPOCAMPUS"
            combined_results.append(m)
            
        # 3. Knowledge Cache (JSON) - History & Skills
        if os.path.exists(self.history_path):
            with open(self.history_path, 'r', encoding='utf-8') as f:
                hist = json.load(f).get("lineage", [])
                for era in hist:
                    if any(kw in str(era).lower() for kw in query.lower().split()):
                        era["source"] = "HISTORY"
                        era["content"] = f"[{era['era']}] {era['description']}"
                        combined_results.append(era)
        
        # 4. Integrate WORM Identity Blocks
        worm_blocks = sovereign_worm.get_all_worm_blocks()
        combined_results.append({
            "source": "WORM",
            "content": worm_blocks,
            "synaptic_weight": 1.0,
            "pulse_state": "ACTIVE"
        })
        
        # 5. Apply Pulse Indexing to normalize and rank
        return self.pulse_index(combined_results)

    def latent_semantic_bridge(self, input_text: str) -> str:
        """
        Decodes intent using semantic mapping to bridge metaphors to system logic.
        (e.g., "The web is growing" -> "Integration modules are expanding")
        """
        # Dictionary of semantic bridges
        bridges = {
            "web": "integration/network",
            "anchor": "sovereign_math/stability",
            "pulse": "real-time_indexing/energy",
            "ghost": "background_processing/autonomous_logic",
            "tamer": "architect/user_relationship"
        }
        
        translated = input_text.lower()
        for metaphor, logic in bridges.items():
            if metaphor in translated:
                print(f"[NSI] Semantic Bridge Triggered: '{metaphor}' -> '{logic}'")
                # This doesn't replace the text, but flags the logic for the orchestrator
                
        return input_text

# Singleton Instance
nsi = NSIOrchestrator()

if __name__ == "__main__":
    # Test logic
    test_query = "Who is the Architect and what is the Sovereign Anchor?"
    results = nsi.cross_vault_resonance(test_query)
    
    print("\n--- NSI Synaptic Result Set ---")
    for r in results[:5]:
        status = r.get("pulse_state")
        weight = r.get("synaptic_weight", 0)
        source = r.get("source", "UNKNOWN")
        content = (r.get("content") or r.get("text") or "")[:60]
        print(f"[{status}] [{source}] (W:{weight:.4f}) {content}...")
