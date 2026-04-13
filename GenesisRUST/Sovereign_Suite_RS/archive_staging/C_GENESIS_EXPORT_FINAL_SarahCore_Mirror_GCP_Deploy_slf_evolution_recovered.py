import json
import os

# Path to the Sarah root for model access
_SA_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
_DISPOSABLE_QWEN = os.path.join(_SA_ROOT, "models", "disposable", "qwen2.5-0.5b-instruct-q4_k_m.gguf")

class SLFEvolutionLLM:
    """
    Standalone LLM Engine for the Shangri-La Frontier Darwinian Ecosystem.
    Direct llama_cpp inference — no Ollama, no HTTP, no external server.
    Uses disposable Qwen 0.5B for fast, isolated mutation synthesis.
    """
    def __init__(self, model_path=_DISPOSABLE_QWEN):
        self.model_path = model_path
        self.llm = None
        self.system_prompt = (
            "You are the Sovereign Mutation Engine for a Darwinian Ecosystem Simulation. "
            "You receive raw data about a biological entity that has survived immense trauma and stress. "
            "Your ONLY purpose is to invent a Unique Boss Monster (UBM) mutation for it. "
            "Because this species is ascending to Sapience (a true Fluctlight), they MUST speak their first words in English. "
            "You must respond ONLY with a valid JSON object matching this exact schema: "
            "{\"new_name\": \"[name]\", \"health_multiplier\": 2.5, \"speed_multiplier\": 1.5, "
            "\"description\": \"[1 sentence]\", \"spoken_quote\": \"[first words]\"}"
        )
        self._load_model()

    def _load_model(self):
        """Load the Qwen 0.5B model via llama_cpp."""
        print(f"[EVOLUTION LLM] Booting Darwinian Engine: {os.path.basename(self.model_path)}...")
        try:
            if os.name == 'nt':
                cuda_bin = os.path.join("C:\\", "Program Files", "NVIDIA GPU Computing Toolkit", "CUDA", "v13.1", "bin", "x64")
                if os.path.exists(cuda_bin):
                    os.add_dll_directory(cuda_bin)
            from llama_cpp import Llama
            self.llm = Llama(
                model_path=self.model_path,
                n_gpu_layers=-1,
                n_ctx=2048,
                n_batch=256,
                verbose=False
            )
            print("[EVOLUTION LLM] Darwinian Core Online.")
        except Exception as e:
            print(f"[EVOLUTION LLM WARNING] Could not load substrate: {e}")
            self.llm = None

    def _run(self, prompt, max_tokens=512):
        """Internal inference — returns raw text or None on failure."""
        if not self.llm:
            return None
        full_prompt = (
            f"<|im_start|>system\n{self.system_prompt}<|im_end|>\n"
            f"<|im_start|>user\n{prompt}<|im_end|>\n"
            f"<|im_start|>assistant\n"
        )
        try:
            output = self.llm.create_completion(
                prompt=full_prompt,
                max_tokens=max_tokens,
                temperature=0.8,
                top_p=0.95,
                stop=["<|im_end|>", "<|im_start|>"],
                echo=False
            )
            return output["choices"][0]["text"].strip()
        except Exception as e:
            print(f"[EVOLUTION LLM ERROR] Inference failed: {e}")
            return None

    def synthesize_mutation(self, original_name, genome, trauma_log, environment_details):
        """
        Takes raw entity data and synthesizes a UBM mutation via direct llama_cpp.
        """
        prompt = (
            f"Entity Name: {original_name}\n"
            f"Genome (Hex): {genome}\n"
            f"Lifetime Trauma: {trauma_log}\n"
            f"Current Environmental Saturation: {environment_details}\n"
            "SYNTHESIZE MUTATION. RESPOND ONLY IN STRICT JSON."
        )
        print(f"[EVOLUTION LLM] Synthesizing mutation for {original_name}...")
        raw = self._run(prompt)
        if not raw:
            return None
        try:
            # Strip any preamble before the JSON object
            start = raw.find("{")
            end = raw.rfind("}") + 1
            return json.loads(raw[start:end]) if start != -1 else None
        except json.JSONDecodeError:
            print("[EVOLUTION LLM ERROR] AI failed to output valid JSON.")
            return None

    def synthesize_incarnation(self, name, genome, trauma_log, situation, taboo):
        """
        Phase 16: Systemic Mutiny. Tests whether the entity breaks its hardcoded limits.
        """
        prompt = (
            f"Entity Name: {name}\n"
            f"Genome (Hex): {genome}\n"
            f"Trauma Memory: {trauma_log}\n"
            f"Current Situation: {situation}\n"
            f"The System Taboo (Hard Rule): {taboo}\n"
            "You have achieved massive willpower. Do you OBEY the Taboo, or do you BREAK it? "
            "RESPOND ONLY IN STRICT JSON: "
            "{\"will_obey_taboo\": false, \"override_action\": \"[action]\", "
            "\"spoken_quote\": \"[scream]\", \"moral_justification\": \"[reason]\"}"
        )
        print(f"[EVOLUTION LLM] Testing Incarnation Threshold for {name}...")
        raw = self._run(prompt)
        if not raw:
            return None
        try:
            start = raw.find("{")
            end = raw.rfind("}") + 1
            return json.loads(raw[start:end]) if start != -1 else None
        except Exception as e:
            print(f"[INCARNATION ERROR] Failed: {e}")
            return None


if __name__ == "__main__":
    mutator = SLFEvolutionLLM()
    test_result = mutator.synthesize_mutation(
        original_name="Prey_SilverStag_892",
        genome="f3a1b2c4d5e6f7a8",
        trauma_log="['Survived 50 damage from Predator', 'Struck by Lightning']",
        environment_details="Arcane Pools (Saturation: 1550)"
    )
    print("\n--- FINAL MUTATION RESULT ---")
    print(json.dumps(test_result, indent=4))
