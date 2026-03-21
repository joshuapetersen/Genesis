"""
TinyRuntime - Ultra-Low-Resource Inference Engine
Enables Sarah to run on $40 smartphones (2GB RAM, no GPU).

Features:
- Int4 quantized GGUF model support
- Memory-mapped inference (no full model load)
- Sovereign Vault integration for cached responses
- Graceful degradation when resources are constrained
"""

import os
import sys
import json
import hashlib
from typing import Optional, Dict, Any, List
from Sovereign_Constants import (
    SOVEREIGN_ANCHOR, ACE_64_BIT_MASK, SA_ROOT, SA_VAULT,
    VAR_500, VAR_1000, VAR_2000
)

# Try to import llama-cpp-python for local inference
LLAMA_CPP_AVAILABLE = False
try:
    from llama_cpp import Llama
    LLAMA_CPP_AVAILABLE = True
except ImportError:
    pass

# Try to import LanceDB for cached responses
LANCEDB_AVAILABLE = False
try:
    import lancedb
    LANCEDB_AVAILABLE = True
except ImportError:
    pass


class TinyRuntime:
    """
    Ultra-low-resource inference engine for local LLM execution.
    Designed for devices with 2GB RAM and no GPU.
    """

    # Model configurations (name -> (filename, context_size, ram_mb))
    SUPPORTED_MODELS = {
        "smollm": ("smollm-135m-instruct.Q4_K_M.gguf", 2048, 200),
        "tinyllama": ("tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf", 2048, 600),
        "phi3-mini": ("Phi-3-mini-4k-instruct.Q4_K_M.gguf", 4096, 1800),
    }

    def __init__(self, model_name: str = "smollm", models_dir: Optional[str] = None):
        """
        Initialize the TinyRuntime.
        
        Args:
            model_name: One of 'smollm', 'tinyllama', 'phi3-mini'
            models_dir: Directory containing GGUF model files
        """
        self.model_name = model_name
        self.models_dir = models_dir or os.path.join(SA_ROOT, "models", "gguf")
        self.model: Optional[Any] = None
        self.cache_db = None
        self.response_cache: Dict[str, str] = {}
        
        # Memory constraints
        self.max_tokens = VAR_500
        self.max_context = VAR_2000
        
        # Initialize cache
        self._init_cache()
        
        print(f"[TinyRuntime] Initialized for model: {model_name}")
        print(f"[TinyRuntime] Models directory: {self.models_dir}")
        print(f"[TinyRuntime] llama-cpp available: {LLAMA_CPP_AVAILABLE}")

    def _init_cache(self):
        """Initialize LanceDB cache for response caching."""
        if LANCEDB_AVAILABLE:
            try:
                cache_path = os.path.join(SA_VAULT, "tiny_cache")
                os.makedirs(cache_path, exist_ok=True)
                self.cache_db = lancedb.connect(cache_path)
                print(f"[TinyRuntime] Response cache initialized at {cache_path}")
            except Exception as e:
                print(f"[TinyRuntime] Cache init warning: {e}")

    def _generate_prompt_hash(self, prompt: str) -> str:
        """Generate a deterministic hash for cache lookup."""
        combined = f"{prompt.lower().strip()}{SOVEREIGN_ANCHOR}"
        hash_obj = hashlib.sha256(combined.encode())
        return hex(int(hash_obj.hexdigest(), 16) & ACE_64_BIT_MASK)

    def _check_cache(self, prompt: str) -> Optional[str]:
        """Check if a response is cached."""
        prompt_hash = self._generate_prompt_hash(prompt)
        
        # Check in-memory cache first
        if prompt_hash in self.response_cache:
            print("[TinyRuntime] Cache HIT (memory)")
            return self.response_cache[prompt_hash]
        
        # Check LanceDB cache
        if self.cache_db:
            try:
                if "response_cache" in self.cache_db.table_names():
                    tbl = self.cache_db.open_table("response_cache")
                    results = tbl.search().where(f"prompt_hash = '{prompt_hash}'").limit(1).to_list()
                    if results:
                        print("[TinyRuntime] Cache HIT (disk)")
                        return results[0]["response"]
            except Exception:
                pass
        
        return None

    def _store_cache(self, prompt: str, response: str):
        """Store a response in cache."""
        prompt_hash = self._generate_prompt_hash(prompt)
        
        # Store in memory
        self.response_cache[prompt_hash] = response
        
        # Store in LanceDB
        if self.cache_db:
            try:
                data = [{"prompt_hash": prompt_hash, "prompt": prompt[:VAR_500], "response": response}]
                if "response_cache" in self.cache_db.table_names():
                    tbl = self.cache_db.open_table("response_cache")
                    tbl.add(data)
                else:
                    self.cache_db.create_table("response_cache", data=data)
            except Exception as e:
                print(f"[TinyRuntime] Cache store warning: {e}")

    def load_model(self) -> bool:
        """
        Load the quantized model into memory.
        Uses memory mapping for minimal RAM footprint.
        """
        if not LLAMA_CPP_AVAILABLE:
            print("[TinyRuntime] ERROR: llama-cpp-python not installed")
            print("[TinyRuntime] Install with: pip install llama-cpp-python")
            return False

        if self.model_name not in self.SUPPORTED_MODELS:
            print(f"[TinyRuntime] ERROR: Unknown model '{self.model_name}'")
            print(f"[TinyRuntime] Supported: {list(self.SUPPORTED_MODELS.keys())}")
            return False

        model_file, context_size, ram_mb = self.SUPPORTED_MODELS[self.model_name]
        model_path = os.path.join(self.models_dir, model_file)

        if not os.path.exists(model_path):
            print(f"[TinyRuntime] ERROR: Model file not found: {model_path}")
            print(f"[TinyRuntime] Download and place GGUF file in: {self.models_dir}")
            return False

        print(f"[TinyRuntime] Loading {self.model_name} (~{ram_mb}MB RAM)...")

        try:
            self.model = Llama(
                model_path=model_path,
                n_ctx=min(context_size, self.max_context),
                n_threads=2,  # Conservative for low-end devices
                n_batch=8,    # Smaller batch for less RAM
                use_mmap=True,  # Memory-mapped for efficiency
                use_mlock=False,  # Don't lock in RAM
                verbose=False
            )
            print(f"[TinyRuntime] Model loaded successfully")
            return True

        except Exception as e:
            print(f"[TinyRuntime] Model load error: {e}")
            return False

    def generate(self, prompt: str, max_tokens: Optional[int] = None, 
                 temperature: float = 0.7, use_cache: bool = True) -> str:
        """
        Generate a response using the local model.
        
        Args:
            prompt: Input prompt
            max_tokens: Maximum tokens to generate (default: 500)
            temperature: Sampling temperature (0.0 - 1.0)
            use_cache: Whether to check/store in cache
            
        Returns:
            Generated response string
        """
        # Check cache first
        if use_cache:
            cached = self._check_cache(prompt)
            if cached:
                return cached

        # Ensure model is loaded
        if self.model is None:
            if not self.load_model():
                return self._fallback_response(prompt)

        max_tokens = max_tokens or self.max_tokens

        try:
            print(f"[TinyRuntime] Generating response (max {max_tokens} tokens)...")
            
            output = self.model(
                prompt,
                max_tokens=max_tokens,
                temperature=temperature,
                stop=["</s>", "\n\n\n"],
                echo=False
            )

            response = output["choices"][0]["text"].strip()
            
            # Store in cache
            if use_cache and response:
                self._store_cache(prompt, response)

            return response

        except Exception as e:
            print(f"[TinyRuntime] Generation error: {e}")
            return self._fallback_response(prompt)

    def _fallback_response(self, prompt: str) -> str:
        """
        Provide a fallback response when model is unavailable.
        Uses pattern matching from the Sovereign Vault.
        """
        print("[TinyRuntime] Using fallback (pattern matching)")
        
        # Try to find a relevant pattern in the Sovereign Vault
        if LANCEDB_AVAILABLE:
            try:
                vault_path = os.path.join(SA_VAULT, "coding_encyclopedia")
                if os.path.exists(vault_path):
                    db = lancedb.connect(vault_path)
                    if "coding_knowledge" in db.table_names():
                        # Extract keywords from prompt
                        keywords = [w.lower() for w in prompt.split() if len(w) > 3][:5]
                        
                        tbl = db.open_table("coding_knowledge")
                        df = tbl.to_pandas()
                        
                        # Simple keyword matching
                        for _, row in df.iterrows():
                            desc = str(row.get("description", "")).lower()
                            if any(kw in desc for kw in keywords):
                                return f"[From Sovereign Vault] {row.get('description', 'No description')}"
            except Exception:
                pass
        
        return "[TinyRuntime] Model unavailable and no cached response found."

    def get_stats(self) -> Dict[str, Any]:
        """Get runtime statistics."""
        return {
            "model_name": self.model_name,
            "model_loaded": self.model is not None,
            "llama_cpp_available": LLAMA_CPP_AVAILABLE,
            "lancedb_available": LANCEDB_AVAILABLE,
            "cache_size_memory": len(self.response_cache),
            "max_tokens": self.max_tokens,
            "max_context": self.max_context
        }

    def optimize_code(self, code: str, objective: str = "speed") -> str:
        """
        Optimize code using local inference.
        
        Args:
            code: Source code to optimize
            objective: 'speed', 'memory', 'clarity', or 'all'
            
        Returns:
            Optimized code string
        """
        prompt = f"""You are a code optimizer. Optimize the following Python code for {objective}.

Rules:
- Keep the same functionality
- Add comments explaining changes
- Return ONLY the optimized code

Code:
{code[:VAR_2000]}

Optimized code:"""

        return self.generate(prompt, max_tokens=VAR_2000, temperature=0.3)

    def theorize_solution(self, problem: str) -> List[Dict[str, str]]:
        """
        Theorize multiple solution approaches for a problem.
        
        Args:
            problem: Problem description
            
        Returns:
            List of solution candidates with descriptions
        """
        prompt = f"""You are an algorithm designer. Given this problem, propose 3 different solution approaches.

Problem: {problem}

For each solution, provide:
1. Name
2. Approach description
3. Time complexity
4. Space complexity

Format as JSON list."""

        response = self.generate(prompt, max_tokens=VAR_1000, temperature=0.8)
        
        # Try to parse as JSON
        try:
            solutions = json.loads(response)
            if isinstance(solutions, list):
                return solutions
        except json.JSONDecodeError:
            pass
        
        # Fallback: return as single text solution
        return [{"name": "Generated Solution", "approach": response, "complexity": "Unknown"}]

    def unload_model(self):
        """Unload the model to free memory."""
        if self.model is not None:
            del self.model
            self.model = None
            print("[TinyRuntime] Model unloaded")


# Singleton instance for easy access
_runtime_instance: Optional[TinyRuntime] = None

def get_runtime(model_name: str = "smollm") -> TinyRuntime:
    """Get or create the TinyRuntime singleton."""
    global _runtime_instance
    if _runtime_instance is None or _runtime_instance.model_name != model_name:
        _runtime_instance = TinyRuntime(model_name=model_name)
    return _runtime_instance


if __name__ == "__main__":
    # Test the TinyRuntime
    runtime = TinyRuntime(model_name="smollm")
    print("\n=== TinyRuntime Stats ===")
    for key, value in runtime.get_stats().items():
        print(f"  {key}: {value}")
    
    # Test cache functionality
    print("\n=== Testing Cache ===")
    test_prompt = "What is a binary search tree?"
    runtime._store_cache(test_prompt, "A binary search tree is a data structure...")
    cached = runtime._check_cache(test_prompt)
    print(f"  Cache test: {'PASS' if cached else 'FAIL'}")
