"""
Sarah Universal - Tiered Intelligence System
Run Sarah on ANY device with intelligent fallback to bigger brains when needed.

TIER 1 (Offline, 2GB):
- TinyLlama 1.1B or SmolLM for local reasoning
- Sovereign Vault for knowledge
- PersistentMemory for cross-session recall
- TheoryLab for algorithm problem-solving

TIER 2 (Online, Complex):
- Gemini API for heavy reasoning
- Local 8B+ models if RAM available
"""

import os
import sys
import time
import json
from typing import Dict, Optional, Any, List, Tuple
from Sovereign_Constants import (
    SOVEREIGN_ANCHOR, SA_ROOT, SA_VAULT,
    VAR_5, VAR_10, VAR_100, VAR_500, VAR_1000
)

# Local imports
from PersistentMemory import PersistentMemory, get_memory
from TinyRuntime import TinyRuntime, get_runtime
from TheoryLab import TheoryLab, get_lab
from NetworkHealer import NetworkHealer, get_healer
from IntelligenceAmplifier import IntelligenceAmplifier


class SmartRouter:
    """
    Routes queries to appropriate tier based on complexity.
    """
    
    # Keywords that suggest complex reasoning
    COMPLEX_KEYWORDS = {
        "explain", "analyze", "compare", "evaluate", "synthesize",
        "design", "architect", "implement", "debug", "optimize",
        "prove", "derive", "calculate", "multi-step", "complex"
    }
    
    # Keywords that can be handled locally
    SIMPLE_KEYWORDS = {
        "what", "who", "when", "where", "define", "list",
        "remember", "recall", "my", "I said", "you said"
    }
    
    @staticmethod
    def score_complexity(query: str) -> float:
        """
        Score query complexity from 0.0 (trivial) to 1.0 (very complex).
        
        Returns:
            Float complexity score
        """
        query_lower = query.lower()
        words = query_lower.split()
        
        score = 0.0
        
        # Length factor
        if len(words) > 50:
            score += 0.2
        elif len(words) > 20:
            score += 0.1
        
        # Complex keyword presence
        complex_count = sum(1 for kw in SmartRouter.COMPLEX_KEYWORDS 
                          if kw in query_lower)
        score += min(complex_count * 0.15, 0.4)
        
        # Simple keyword presence (reduces score)
        simple_count = sum(1 for kw in SmartRouter.SIMPLE_KEYWORDS 
                          if kw in query_lower)
        score -= min(simple_count * 0.1, 0.3)
        
        # Code generation indicators
        if any(kw in query_lower for kw in ["write code", "implement", "function", "class"]):
            score += 0.3
        
        # Multi-part question
        if query.count("?") > 1 or " and " in query_lower:
            score += 0.15
        
        return max(0.0, min(score, 1.0))
    
    @staticmethod
    def should_escalate(query: str, threshold: float = 0.6) -> bool:
        """Check if query should escalate to Tier 2."""
        return SmartRouter.score_complexity(query) > threshold


class SarahUniversal:
    """
    Sarah Universal - Runs on any device with tiered intelligence.
    """
    
    VERSION = "1.0.0"
    
    def __init__(self, model_name: str = "tinyllama", force_offline: bool = False):
        """
        Initialize Sarah Universal.
        
        Args:
            model_name: 'smollm' (135M) or 'tinyllama' (1.1B)
            force_offline: Never escalate to Tier 2
        """
        print(f"\n{'='*50}")
        print(f" SARAH UNIVERSAL v{self.VERSION}")
        print(f" Tiered Intelligence System")
        print(f"{'='*50}\n")
        
        self.model_name = model_name
        self.force_offline = force_offline
        self.tier = 1  # Current operating tier
        
        # Apply resource caps
        self._enforce_limits()
        
        # Initialize Tier 1 components
        print("[Universal] Loading Tier 1 (Offline) components...")
        self.memory = get_memory()
        self.runtime = get_runtime(model_name)
        self.lab = get_lab()
        self.healer = get_healer()
        
        # Intelligence Amplifier (1.1B -> 120B reasoning)
        self.amplifier = IntelligenceAmplifier(model_name)
        
        # Tier 2 backend (lazy loaded)
        self.tier2_backend = None
        
        # Stats
        self.queries_local = 0
        self.queries_escalated = 0
        
        print(f"[Universal] Ready. Mode: {'OFFLINE ONLY' if force_offline else 'TIERED'}")

    def _enforce_limits(self):
        """Enforce 2GB RAM cap and CPU limits."""
        try:
            from sarah_lite import enforce_2gb_cap, set_low_priority
            enforce_2gb_cap()
            set_low_priority()
        except Exception as e:
            print(f"[Universal] Limit enforcement warning: {e}")

    def _check_tier2_available(self) -> bool:
        """Check if Tier 2 (online) is available."""
        if self.force_offline:
            return False
        
        # Check network
        diagnosis = self.healer.diagnose()
        return diagnosis.get("healthy", False)

    def _load_tier2(self) -> bool:
        """Lazy load Tier 2 backend."""
        if self.tier2_backend:
            return True
        
        try:
            # Try Gemini API
            from Gemini_Genesis_Core import GeminiGenesisCore
            api_key = os.getenv("GEMINI_API_KEY")
            if api_key:
                self.tier2_backend = GeminiGenesisCore(api_key)
                print("[Universal] Tier 2: Gemini API loaded")
                return True
        except Exception as e:
            print(f"[Universal] Tier 2 load warning: {e}")
        
        return False

    def _call_tier2(self, query: str) -> Optional[str]:
        """Make a Tier 2 API call."""
        if not self.tier2_backend:
            return None
        
        try:
            response = self.tier2_backend.generate_content_safe(
                user_input=query,
                system_instruction="You are Sarah, an intelligent AI assistant. Be helpful and concise."
            )
            return response
        except Exception as e:
            print(f"[Universal] Tier 2 call failed: {e}")
            return None

    def think(self, query: str) -> str:
        """
        Process a query using tiered intelligence.
        
        Args:
            query: User query
            
        Returns:
            Response string
        """
        # Store in memory
        self.memory.add_conversation("user", query)
        
        # Score complexity
        complexity = SmartRouter.score_complexity(query)
        should_escalate = complexity > 0.6
        
        print(f"[Universal] Complexity: {complexity:.2f} | Escalate: {should_escalate}")
        
        # Check for memory recall queries
        if any(kw in query.lower() for kw in ["remember", "recall", "you said", "i said", "my name"]):
            memories = self.memory.recall(query, limit=3)
            if memories:
                context = "\n".join([f"- {m.content}" for m in memories])
                response = f"I remember:\n{context}"
                self.memory.add_conversation("sarah", response)
                self.queries_local += 1
                return response
        
        # Check for algorithm/problem-solving queries
        if any(kw in query.lower() for kw in ["how to", "solve", "algorithm", "approach", "design"]):
            # Use Intelligence Amplifier for deep reasoning
            print("[Universal] activating Intelligence Amplifier...")
            response = self.amplifier.amplify_thought(query)
            self.memory.add_conversation("sarah", response)
            self.queries_local += 1
            return response
        
        # Tier 1: Local inference
        if not should_escalate or self.force_offline:
            self.tier = 1
            
            # Add memory context
            context = self.memory.get_conversation_context(turns=5)
            prompt = f"Previous conversation:\n{context}\n\nUser: {query}\n\nSarah:"
            
            response = self.runtime.generate(prompt, max_tokens=VAR_500)
            
            if response and len(response) > 10:
                self.memory.add_conversation("sarah", response)
                self.queries_local += 1
                return response
        
        # Tier 2: Escalate to bigger brain
        if self._check_tier2_available() and self._load_tier2():
            self.tier = 2
            print("[Universal] Escalating to Tier 2 (Gemini)...")
            
            # Add memory context
            context = self.memory.get_conversation_context(turns=5)
            full_query = f"Context:\n{context}\n\nQuery: {query}"
            
            response = self._call_tier2(full_query)
            
            if response:
                self.memory.add_conversation("sarah", response)
                self.queries_escalated += 1
                return response
        
        # Fallback: Best effort local response
        self.tier = 1
        response = self.runtime.generate(query, max_tokens=VAR_500)
        
        if response:
            self.memory.add_conversation("sarah", response)
            self.queries_local += 1
            return response
        
        return "I'm not sure how to help with that. Could you rephrase?"

    def get_stats(self) -> Dict[str, Any]:
        """Get universal stats."""
        import psutil
        p = psutil.Process()
        
        return {
            "version": self.VERSION,
            "current_tier": self.tier,
            "force_offline": self.force_offline,
            "queries_local": self.queries_local,
            "queries_escalated": self.queries_escalated,
            "memory_stats": self.memory.get_stats(),
            "runtime_stats": self.runtime.get_stats(),
            "ram_mb": p.memory_info().rss / 1024 / 1024
        }

    def interactive(self):
        """Run interactive chat loop."""
        print("\n" + "-"*50)
        print(" Sarah Universal - Type 'exit' to quit")
        print(" 'stats' for system info, 'clear' for memory")
        print("-"*50 + "\n")
        
        while True:
            try:
                user_input = input("You: ").strip()
                
                if not user_input:
                    continue
                
                if user_input.lower() in ('exit', 'quit', 'q'):
                    break
                
                if user_input.lower() == 'stats':
                    stats = self.get_stats()
                    print(f"\n[Stats] RAM: {stats['ram_mb']:.0f} MB | "
                          f"Tier: {stats['current_tier']} | "
                          f"Local: {stats['queries_local']} | "
                          f"Escalated: {stats['queries_escalated']}")
                    print(f"[Stats] Memories: {stats['memory_stats']['total_memories']}\n")
                    continue
                
                if user_input.lower() == 'clear':
                    self.memory.clear_conversation_buffer()
                    print("[Universal] Conversation cleared.\n")
                    continue
                
                # Process query
                response = self.think(user_input)
                print(f"\nSarah [Tier {self.tier}]: {response}\n")
                
            except KeyboardInterrupt:
                break
            except Exception as e:
                print(f"\n[Error]: {e}\n")
        
        print("\n[Universal] Goodbye!")


def main():
    """CLI entry point."""
    import argparse
    
    parser = argparse.ArgumentParser(description="Sarah Universal - Tiered Intelligence")
    parser.add_argument("--model", "-m", default="tinyllama", 
                       choices=["smollm", "tinyllama"],
                       help="Local model: smollm (135M) or tinyllama (1.1B)")
    parser.add_argument("--offline", action="store_true",
                       help="Force offline mode (never escalate)")
    
    args = parser.parse_args()
    
    sarah = SarahUniversal(model_name=args.model, force_offline=args.offline)
    sarah.interactive()


if __name__ == "__main__":
    main()
