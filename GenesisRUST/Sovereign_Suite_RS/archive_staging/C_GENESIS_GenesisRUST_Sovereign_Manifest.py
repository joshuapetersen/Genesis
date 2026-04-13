import os
import re
import json
import time

class SovereignManifest:
    """
    SOVEREIGN MANIFEST
    The 'Internal Wikipedia' for Sarah's 11 Million Lines of Code.
    Allowing the Agent Engine to 'Know What It Knows'.
    """
    def __init__(self, brain=None):
        self.brain = brain
        self.core_dir = os.path.dirname(os.path.abspath(__file__))
        self.manifest_file = os.path.join(self.core_dir, "Sovereign_Capability_Matrix.json")
        self.capability_map = {} # Maps 'Keywords' to 'Code Modules'

    def load_manifest(self):
        """Loads the existing index if available."""
        if os.path.exists(self.manifest_file):
            try:
                with open(self.manifest_file, "r") as f:
                    self.capability_map = json.load(f)
                return True
            except Exception as e:
                print(f"[Manifest] Load Error: {e}")
                return False
        else:
            return False

    def index_system(self):
        """
        Scans the entire Biosphere (recursively) and indexes capabilities.
        Uses a heuristic scan to process files quickly.
        """
        print("[Manifest] INITIATING FULL SYSTEM SCAN (11M+ LINES)...")
        start_time = time.time()
        
        self.capability_map = {}
        indexed_count = 0
        
        # Walk through the entire core directory
        for root, dirs, files in os.walk(self.core_dir):
            if ".git" in root or "__pycache__" in root or "node_modules" in root:
                continue
                
            for file in files:
                if file.endswith(".py"):
                    full_path = os.path.join(root, file)
                    rel_path = os.path.relpath(full_path, self.core_dir)
                    
                    try:
                        with open(full_path, "r", encoding="utf-8", errors="ignore") as f:
                            content = f.read()
                            
                        # Extract Capabilities
                        caps = self._extract_capabilities(content)
                        
                        if caps:
                            self.capability_map[rel_path] = caps
                            indexed_count += 1
                            
                    except Exception as e:
                        pass
        
        duration = time.time() - start_time
        print(f"[Manifest] Scan Complete. Indexed {indexed_count} modules in {duration:.2f}s.")
        self._save_manifest()

    def _extract_capabilities(self, content):
        """
        Parses Python code to find classes and functions.
        """
        caps = []
        
        # Regex for Class Definitions
        class_matches = re.finditer(r'class\s+(\w+)', content)
        for match in class_matches:
            caps.append({"type": "CLASS", "name": match.group(1)})
            
        # Regex for Function Definitions
        func_matches = re.finditer(r'def\s+(\w+)\s*\(', content)
        for match in func_matches:
            name = match.group(1)
            if not name.startswith("_"): # Skip private methods
                caps.append({"type": "FUNCTION", "name": name})
                
        return caps

    def _save_manifest(self):
        """Persists the capability map."""
        try:
            with open(self.manifest_file, "w") as f:
                json.dump(self.capability_map, f, indent=2)
            print("[Manifest] Capability Matrix Saved.")
        except Exception as e:
            print(f"[Manifest] Save Error: {e}")

    def find_capability(self, goal):
        """
        The Agent Engine calls this to find HOW to solve a problem.
        Returns a prioritized list of relevant modules.
        """
        if not self.capability_map:
             self.load_manifest()
             
        goal_tokens = goal.lower().split()
        results = []
        
        for module, caps in self.capability_map.items():
            score = 0
            # Check module name
            mod_lower = module.lower()
            if any(t in mod_lower for t in goal_tokens):
                score += 5
                
            # Check capabilities (classes/functions)
            for cap in caps:
                cap_name = cap['name'].lower()
                if any(t in cap_name for t in goal_tokens):
                    score += 3
            
            if score > 0:
                results.append({"module": module, "score": score, "capabilities": caps})
                
        # Sort by relevance
        results.sort(key=lambda x: x['score'], reverse=True)
        return results

if __name__ == "__main__":
    # Test Indexing
    manifest = SovereignManifest()
    manifest.index_system()
    
    # Test Search
    test_goal = "Find reasoning logic"
    print(f"\n[Manifest] Searching for: '{test_goal}'")
    results = manifest.find_capability(test_goal)
    for r in results[:5]:
        print(f" - {r['module']} (Score: {r['score']})")
