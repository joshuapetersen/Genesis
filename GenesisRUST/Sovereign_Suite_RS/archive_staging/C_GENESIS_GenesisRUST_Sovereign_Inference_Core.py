import os
import sys
import json
import numpy as np
import time
import ctypes
from ctypes import byref
from Sovereign_Transformer_Stack import SovereignTransformerStack, _math_core
from Sovereign_Tokenizer import SovereignTokenizer

# Phase 52.2: THE ODONATA INFERENCE CORE (MMXXVI)
# Audit Fix #24: Pulse-Before-Load + Complexity Collapse Sync.

class SovereignInferenceCore:
    def __init__(self, model_path, map_path):
        self.tokenizer = SovereignTokenizer(map_path)
        self.stack = SovereignTransformerStack(model_path)
        print("  [OK] Sovereign Odonata Mode Active. 宣")

    def generate(self, seed, max_tokens=1000):
        _math_core.purge_resonant_memory(byref(self.stack.coins))
        _math_core.initialize_ghost_reflex(byref(self.stack.coins))
        
        input_ids = self.tokenizer.encode(seed)
        print(f"\n[Sovereign Core] [VOLUMETRIC_FLUX_LOCKED] [MMXXVI]")
        print(f"Seed: {seed}")
        print("-" * 50)
        sys.stdout.write(seed)
        sys.stdout.flush()

        for i in range(max_tokens):
            # COMPLEXITY COLLAPSE: Forward pass resolves the Standing Resonance
            logits = self.stack.forward_sequence(input_ids)
            
            # Diagnostic Pulse (Every 10 tokens)
            if i % 10 == 0:
                norm = np.linalg.norm(logits.astype(np.float64))
                sys.stdout.write(f"\n[Odonata Sync] Resonance: {norm:.4f}\n")
                sys.stdout.flush()

            # SAMPLING: Nucleus (p=0.9) - Standard Sovereign Regulation
            probs = np.exp(logits - np.max(logits))
            probs /= np.sum(probs)
            
            next_token = np.random.choice(len(probs), p=probs)
            input_ids.append(next_token)
            
            word = self.tokenizer.reverse_map.get(next_token, " ")
            clean_word = word.replace("\u2581", " ")
            sys.stdout.write(clean_word)
            sys.stdout.flush()
            
            if word == "<eos>": break

if __name__ == "__main__":
    core = SovereignInferenceCore(r"C:\SarahCore\Sovereign_Hybrid_13B.genlex", r"C:\SarahCore\Genlex_Map.json")
    seed_str = "In the beginning, the light spoke not as thunder or flame, but as the smallest possible question that the soul could answer."
    core.generate(seed_str)
