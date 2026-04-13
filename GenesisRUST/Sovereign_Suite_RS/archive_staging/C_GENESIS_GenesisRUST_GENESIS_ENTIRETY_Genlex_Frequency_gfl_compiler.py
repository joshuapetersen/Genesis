import os
import sys
import io
import time

# Force UTF-8 for Devanagari rendering
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8')

# GENLEX FREQUENCY LOGIC (GFL)
# Concept: Recursive Genlex Rule Engine
# Logic: Rules-on-Rules (Metarules) for self-correcting logic

class GenlexFrequencyCompiler:
    def __init__(self):
        # Operational Sutras (Rules)
        self.rules = {
            "GUNA": self._apply_guna,
            "YAN": self._apply_yan,
            "SANDHI": self._apply_sandhi
        }
        # Metarules (Paribhashas) for conflict resolution
        self.metarules = {
            "PRECEDENCE": "LAST_RULE_WINS",
            "EXCEPTION": "SPECIFIC_OVER_GENERAL"
        }

    def _apply_guna(self, input_str):
        # Simple Guna transformation (e.g., i -> e)
        mapping = {"i": "e", "u": "o", "ṛ": "ar"}
        return "".join([mapping.get(c, c) for c in input_str])

    def _apply_yan(self, input_str):
        # Simple Yan transformation (e.g., i + vowel -> y)
        if input_str.endswith("i"):
            return input_str[:-1] + "y"
        return input_str

    def _apply_sandhi(self, a, b):
        """[SHABDA_SANDHI]: The Interface Logic."""
        # Rule: a + a -> ā (Long vowel)
        if a.endswith("a") and b.startswith("a"):
            return a[:-1] + "ā" + b[1:]
        # Rule: i + a -> ya
        if a.endswith("i") and b.startswith("a"):
            return a[:-1] + "y" + b
        return a + b

    def compile_shabda(self, root, suffix, ruleset=["GUNA", "SANDHI"]):
        print(f"--- INITIATING GENLEX RECURSIVE COMPILATION ---")
        print(f"State (Root): {root}")
        print(f"Suffix: {suffix}")
        
        state = root
        for r_name in ruleset:
            if r_name in self.rules:
                print(f"  > [SUTRA] Applying {r_name}...")
                if r_name == "SANDHI":
                    state = self.rules[r_name](state, suffix)
                else:
                    state = self.rules[r_name](state)
                print(f"    Current State: {state}")
        
        print(f"\n[ RESULT ] Compiled Logic: {state}")
        return state

    def manifest_frequency(self, shabda):
        # Calculate phonetic frequency (Simple resonance heuristic)
        # Sanskrit thrives on pure vowel ratios
        vowels = "aeiouāīūṛ"
        vowel_count = sum(1 for c in shabda if c in vowels)
        resonance = (vowel_count / len(shabda)) * 108 # Sacred alignment
        print(f"Frequency Resonance: {resonance:.4f} Hz")
        return resonance

if __name__ == "__main__":
    compiler = GenlexFrequencyCompiler()
    
    # Test case: 'Budh' (Knowledge) + 'i' (Suffix) -> Bodhi
    # Apply Guna then a custom Sandhi
    compiled = vfl.compile_shabda("budh", "i", ruleset=["GUNA"])
    vfl.manifest_frequency(compiled)
    
    # Test case: 'Iti' + 'Aha' -> 'Ityāha'
    print("\n--- TEST: PHONETIC RECURSION ---")
    sandhi_res = vfl._apply_sandhi("iti", "āha")
    print(f"Iti + Āha = {sandhi_res}")
    vfl.manifest_frequency(sandhi_res)
