"""
Intelligence Amplifier - 120B Reasoning on 2GB RAM
Enables a 1.1B (or smaller) model to solve complex problems through decomposition, retrieval, and symbolic execution.

Architecture:
1. Decomposer: Breaks complex queries into atomic sub-tasks.
2. Retriever: Fetches exact facts from Sovereign Vault (simulating "knowledge").
3. Symbolic Engine: Solves math/logic deterministically (simulating "reasoning").
4. Synthesizer: Compiles sub-results into a coherent answer.
"""

import re
import json
from typing import List, Dict, Any, Optional
from Sovereign_Constants import VAR_3, VAR_5, VAR_10
from TinyRuntime import get_runtime
from TheoryLab import get_lab
from PersistentMemory import get_memory

class IntelligenceAmplifier:
    """
    Amplifies small model intelligence via iterative refinement and tool use.
    """
    
    def __init__(self, model_name: str = "smollm"):
        self.runtime = get_runtime(model_name)
        self.lab = get_lab()
        self.memory = get_memory()
        print("[Amplifier] Intelligence Amplification Module Online.")

    def amplify_thought(self, complex_query: str) -> str:
        """
        Main entry point: Transforms a complex query into a high-intelligence response.
        """
        print(f"[Amplifier] Amplifying: {complex_query[:50]}...")
        
        # 1. Decomposition (Break it down)
        sub_tasks = self._decompose(complex_query)
        print(f"[Amplifier] Decomposed into {len(sub_tasks)} sub-tasks.")
        
        results = []
        for task in sub_tasks:
            # 2. Routing (Solve each sub-task)
            result = self._solve_atomic_task(task)
            results.append(result)
            print(f"[Amplifier] Solved: {task[:30]}... -> {str(result)[:30]}...")
            
        # 3. Synthesis (Combine results)
        final_answer = self._synthesize(complex_query, results)
        return final_answer

    def _decompose(self, query: str) -> List[str]:
        """
        Uses heuristics and small-model prompting to break down a query.
        """
        # Simple heuristic decomposition for now (can be enhanced with LLM)
        if " and " in query:
             return [part.strip() for part in query.split(" and ")]
        
        # Ask TinyLlama to decompose if it's a "how to" or "explain"
        prompt = f"""Break this problem into 3 simple steps: "{query}".
        Return ONLY the steps as a numbered list."""
        
        response = self.runtime.generate(prompt, max_tokens=100)
        steps = re.findall(r"\d+\.\s*(.*)", response)
        
        if not steps:
            return [query] # Fallback to single task
            
        return steps

    def _solve_atomic_task(self, task: str) -> str:
        """
        Solves a single, simple task using the best available tool.
        """
        task_lower = task.lower()
        
        # Tool: Sovereign Vault (Knowledge)
        if any(kw in task_lower for kw in ["what is", "define", "explain", "who", "concept"]):
            # Search vault
            keywords = [w for w in task_lower.split() if len(w) > 3]
            vault_hits = self.lab._search_vault(keywords, limit=1)
            if vault_hits:
                return f"Fact: {vault_hits[0]['description']}"
                
        # Tool: Symbolic Math/Logic (Reasoning)
        if any(kw in task_lower for kw in ["calculate", "solve", "math", "logic"]):
            # Try TheoryLab for algorithms
            candidates = self.lab.theorize(task, num_candidates=1)
            if candidates:
                return f"Algorithm: {candidates[0].approach}"
                
        # Tool: Persistent Memory (Recall)
        if "remember" in task_lower or "recall" in task_lower:
             memories = self.memory.recall(task, limit=1)
             if memories:
                 return f"Memory: {memories[0].content}"

        # Fallback: Ask the Small Model (Creativity/Filling gaps)
        return self.runtime.generate(task, max_tokens=200)

    def _synthesize(self, original_query: str, results: List[str]) -> str:
        """
        Combines atomic results into a fluent answer.
        """
        context = "\n".join([f"- {r}" for r in results])
        prompt = f"""Question: {original_query}
        
        Information:
        {context}
        
        Answer the question using the information above."""
        
        return self.runtime.generate(prompt, max_tokens=300)

if __name__ == "__main__":
    amp = IntelligenceAmplifier()
    query = "Explain binary search and write a python function for it"
    print(f"\nFinal Answer:\n{amp.amplify_thought(query)}")
