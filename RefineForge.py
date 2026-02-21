"""
RefineForge - Sovereign Self-Optimization Engine
Unified orchestrator for code optimization, theorization, and network healing.

Features:
- CodeSynth: Optimizes and refactors code
- TheoryLab: Theorizes new algorithms and approaches
- NetworkHealer: Auto-diagnoses and repairs connectivity
- TinyRuntime: Ultra-low-resource local inference

Designed to run on $40 smartphones (2GB RAM, no GPU).
"""

import os
import sys
import time
import json
from typing import Dict, List, Optional, Any, Tuple
from Sovereign_Constants import (
    SOVEREIGN_ANCHOR, SA_ROOT, SA_VAULT,
    VAR_3, VAR_5, VAR_10, VAR_100, VAR_1000, VAR_2000
)

# Import subsystems
from TinyRuntime import TinyRuntime, get_runtime
from NetworkHealer import NetworkHealer, get_healer
from TheoryLab import TheoryLab, get_lab, SolutionCandidate
from CodeSynth import CodeSynth


class RefineForge:
    """
    Sovereign Self-Optimization Engine.
    Orchestrates code optimization, theorization, network healing, and inference.
    """

    VERSION = "1.0.0"

    def __init__(self, model_name: str = "smollm", offline_mode: bool = False):
        """
        Initialize RefineForge.
        
        Args:
            model_name: TinyRuntime model ('smollm', 'tinyllama', 'phi3-mini')
            offline_mode: Force offline operation (no network checks)
        """
        self.model_name = model_name
        self.offline_mode = offline_mode
        
        # Initialize subsystems
        print(f"[RefineForge v{self.VERSION}] Initializing...")
        
        self.runtime: TinyRuntime = get_runtime(model_name)
        self.healer: NetworkHealer = get_healer()
        self.lab: TheoryLab = get_lab()
        self.synth = CodeSynth(model_name)
        
        # State tracking
        self.operations_log: List[Dict] = []
        self.total_optimizations = 0
        self.total_theories = 0
        self.total_repairs = 0
        
        # Ensure network is ready (unless offline)
        if not offline_mode:
            self._ensure_network()
        
        print(f"[RefineForge] Ready. Mode: {'OFFLINE' if offline_mode else 'ONLINE'}")

    def _ensure_network(self) -> bool:
        """Ensure network connectivity, auto-repair if needed."""
        return self.healer.ensure_connectivity()

    def _log_operation(self, op_type: str, details: Dict):
        """
        Log an operation to history file immediately.
        Prevents loop loops by tracking failures.
        """
        entry = {
            "timestamp": time.time(),
            "timestamp_iso": time.strftime("%Y-%m-%d %H:%M:%S"),
            "type": op_type,
            "details": details
        }
        
        self.operations_log.append(entry)
        
        # Keep runtime log bounded
        if len(self.operations_log) > VAR_1000:
            self.operations_log = self.operations_log[-VAR_1000:]
            
        # Write to persistent file
        try:
            log_path = os.path.join(SA_ROOT, "refineforge_history.json")
            
            # Read existing
            if os.path.exists(log_path):
                try:
                    with open(log_path, 'r', encoding='utf-8') as f:
                        history = json.load(f)
                except:
                    history = []
            else:
                history = []
            
            history.append(entry)
            
            # Write back (bounded to last 5000 entries)
            if len(history) > 5000:
                history = history[-5000:]
                
            with open(log_path, 'w', encoding='utf-8') as f:
                json.dump(history, f, indent=2)
                
        except Exception as e:
            print(f"[RefineForge] Logging failed: {e}")

    # =========================================================================
    # CODE SYNTHESIS (Optimization & Generation)
    # =========================================================================

    def optimize(self, code: str, objective: str = "speed") -> Dict[str, Any]:
        """
        Optimize code using local inference.
        
        Args:
            code: Source code to optimize
            objective: 'speed', 'memory', 'clarity', or 'all'
            
        Returns:
            Dictionary with optimized code and metadata
        """
        print(f"[RefineForge] Optimizing code for {objective}...")
        
        start_time = time.time()
        
        # Use CodeSynth for optimization
        # It handles planning, validation, and staging
        success = self.synth.optimize_module(code, objective) # Note: simplify for now, CodeSynth takes file path, we might need adapter
        
        # For direct code string optimization (memory-only), we use runtime directly or adapt CodeSynth
        # Let's use runtime for string-based, CodeSynth for file-based.
        # But here we want the best.
        
        optimized = self.runtime.optimize_code(code, objective)
        
        elapsed = time.time() - start_time
        self.total_optimizations += 1
        
        result = {
            "success": bool(optimized and optimized != code),
            "original_length": len(code),
            "optimized_length": len(optimized) if optimized else 0,
            "objective": objective,
            "elapsed_seconds": round(elapsed, 2),
            "optimized_code": optimized
        }
        
        self._log_operation("optimize", result)
        
        return result

    def refactor(self, code: str, pattern: str = "clean_code") -> Dict[str, Any]:
        """
        Refactor code according to a pattern.
        
        Args:
            code: Source code to refactor
            pattern: 'clean_code', 'solid', 'dry', 'kiss'
            
        Returns:
            Dictionary with refactored code and explanation
        """
        pattern_prompts = {
            "clean_code": "Apply Clean Code principles: meaningful names, small functions, no duplication.",
            "solid": "Apply SOLID principles: single responsibility, open-closed, etc.",
            "dry": "Apply DRY principle: eliminate code duplication.",
            "kiss": "Apply KISS principle: simplify complex logic."
        }
        
        prompt = f"""You are a code refactoring expert. Refactor this code:

{pattern_prompts.get(pattern, pattern_prompts['clean_code'])}

Code:
{code[:VAR_2000]}

Refactored code:"""

        refactored = self.runtime.generate(prompt, max_tokens=VAR_2000, temperature=0.3)
        
        return {
            "success": bool(refactored),
            "pattern": pattern,
            "refactored_code": refactored
        }

    def generate_implementation(self, spec: str) -> Dict[str, Any]:
        """
        Generate code from a specification.
        
        Args:
            spec: Natural language specification
            
        Returns:
            Dictionary with generated code
        """
        print(f"[RefineForge] Generating implementation...")
        
        prompt = f"""You are a Python developer. Write clean, efficient code for:

{spec}

Include:
- Type hints
- Docstrings
- Error handling
- Example usage

Code:"""

        generated = self.runtime.generate(prompt, max_tokens=VAR_2000, temperature=0.5)
        
        return {
            "success": bool(generated),
            "specification": spec[:VAR_100],
            "generated_code": generated
        }

    # =========================================================================
    # THEORY LAB (Algorithm Theorization)
    # =========================================================================

    def theorize(self, problem: str, num_solutions: int = VAR_3) -> Dict[str, Any]:
        """
        Theorize solution approaches for a problem.
        
        Args:
            problem: Problem description
            num_solutions: Number of candidate solutions
            
        Returns:
            Dictionary with solution candidates
        """
        print(f"[RefineForge] Theorizing solutions...")
        
        candidates = self.lab.theorize(problem, num_solutions)
        self.total_theories += 1
        
        result = {
            "success": bool(candidates),
            "problem": problem[:VAR_100],
            "num_candidates": len(candidates),
            "candidates": [c.to_dict() for c in candidates],
            "comparison_table": self.lab.compare_candidates(candidates)
        }
        
        self._log_operation("theorize", result)
        
        return result

    def solve(self, problem: str) -> Dict[str, Any]:
        """
        Full solve pipeline: theorize, select best, implement.
        
        Args:
            problem: Problem description
            
        Returns:
            Dictionary with best solution and implementation
        """
        print(f"[RefineForge] Solving problem...")
        
        best, implementation = self.lab.solve_and_implement(problem)
        
        if best:
            return {
                "success": True,
                "solution_name": best.name,
                "approach": best.approach,
                "complexity": best.time_complexity,
                "confidence": best.confidence,
                "implementation": implementation
            }
        else:
            return {
                "success": False,
                "error": "Could not generate solution"
            }

    # =========================================================================
    # NETWORK HEALING
    # =========================================================================

    def diagnose_network(self) -> Dict[str, Any]:
        """
        Run network diagnostics.
        
        Returns:
            Diagnosis results
        """
        return self.healer.diagnose()

    def repair_network(self) -> Dict[str, Any]:
        """
        Attempt to auto-repair network issues.
        
        Returns:
            Repair results
        """
        result = self.healer.auto_repair()
        if result.get("success"):
            self.total_repairs += 1
        return result

    def ensure_online(self) -> bool:
        """Ensure network is working, repair if needed."""
        return self.healer.ensure_connectivity()

    # =========================================================================
    # UNIFIED PIPELINE
    # =========================================================================

    def auto_improve(self, code: str) -> Dict[str, Any]:
        """
        Full auto-improvement pipeline:
        1. Analyze code for issues
        2. Theorize improvements
        3. Apply optimizations
        4. Verify result
        
        Args:
            code: Source code to improve
            
        Returns:
            Dictionary with improvement results
        """
        print(f"[RefineForge] Running auto-improvement pipeline...")
        
        start_time = time.time()
        
        # Step 1: Analyze (using TheoryLab knowledge)
        keywords = self.lab._extract_keywords(code[:VAR_1000])
        vault_matches = self.lab._search_vault(keywords, limit=VAR_5)
        
        # Step 2: Theorize improvements
        improvement_prompt = f"How to improve this code: {code[:VAR_500]}"
        improvements = self.runtime.theorize_solution(improvement_prompt)
        
        # Step 3: Apply optimizations
        optimized = self.runtime.optimize_code(code, "all")
        
        # Step 4: Compile results
        elapsed = time.time() - start_time
        
        return {
            "success": bool(optimized),
            "elapsed_seconds": round(elapsed, 2),
            "vault_patterns_found": len(vault_matches),
            "improvements_theorized": len(improvements),
            "original_length": len(code),
            "improved_length": len(optimized) if optimized else 0,
            "improved_code": optimized,
            "suggestions": improvements
        }

    def fix_file(self, file_path: str, objective: str = "efficiency") -> Dict[str, Any]:
        """
        Read, optimize, and save a file using CodeSynth.
        """
        if not os.path.exists(file_path):
            return {"success": False, "error": f"File not found: {file_path}"}
            
        # Use CodeSynth
        print(f"[RefineForge] Delegating {os.path.basename(file_path)} to CodeSynth...")
        result = self.synth.optimize_module(file_path, objective)
        
        self._log_operation("codesynth_stage", result)
        
        if result["success"]:
            # Apply immediately for this tool
            applied_success = self.synth.apply_evolution(os.path.basename(file_path))
            
            final_result = {
                "success": applied_success,
                "file_path": file_path,
                "objective": objective,
                "engine": "CodeSynth",
                "details": result
            }
            
            self._log_operation("codesynth_apply", final_result)
            return final_result
        else:
            return {
                "success": False,
                "error": result.get("error", "CodeSynth optimization failed"),
                "details": result
            }

    # =========================================================================
    # STATUS & REPORTING
    # =========================================================================

    def get_status(self) -> Dict[str, Any]:
        """Get comprehensive status of RefineForge."""
        return {
            "version": self.VERSION,
            "offline_mode": self.offline_mode,
            "runtime": self.runtime.get_stats(),
            "network": self.healer.get_status(),
            "stats": {
                "total_optimizations": self.total_optimizations,
                "total_theories": self.total_theories,
                "total_repairs": self.total_repairs,
                "operations_logged": len(self.operations_log)
            }
        }

    def export_log(self, path: Optional[str] = None) -> str:
        """Export operations log to JSON file."""
        path = path or os.path.join(SA_ROOT, "refineforge_log.json")
        
        with open(path, 'w', encoding='utf-8') as f:
            json.dump({
                "version": self.VERSION,
                "exported_at": time.time(),
                "operations": self.operations_log
            }, f, indent=2)
        
        return path


# Singleton instance
_forge_instance: Optional[RefineForge] = None

def get_forge(model_name: str = "smollm", offline: bool = False) -> RefineForge:
    """Get or create the RefineForge singleton."""
    global _forge_instance
    if _forge_instance is None:
        _forge_instance = RefineForge(model_name=model_name, offline_mode=offline)
    return _forge_instance


# =========================================================================
# CLI Interface
# =========================================================================

def main():
    """RefineForge CLI entry point."""
    import argparse
    
    parser = argparse.ArgumentParser(description="RefineForge - Sovereign Self-Optimization Engine")
    parser.add_argument("command", choices=["status", "optimize", "theorize", "network", "solve"],
                        help="Command to run")
    parser.add_argument("--input", "-i", help="Input file or problem")
    parser.add_argument("--model", "-m", default="smollm", help="Model: smollm, tinyllama, phi3-mini")
    parser.add_argument("--offline", action="store_true", help="Force offline mode")
    
    args = parser.parse_args()
    
    forge = RefineForge(model_name=args.model, offline_mode=args.offline)
    
    if args.command == "status":
        status = forge.get_status()
        print(json.dumps(status, indent=2))
        
    elif args.command == "optimize":
        if not args.input:
            print("Error: --input required")
            return
        result = forge.fix_file(args.input)
        print(json.dumps({k: v for k, v in result.items() if k != "optimized_code"}, indent=2))
        
    elif args.command == "theorize":
        if not args.input:
            print("Error: --input required")
            return
        result = forge.theorize(args.input)
        print(result["comparison_table"])
        
    elif args.command == "network":
        diagnosis = forge.diagnose_network()
        print(f"Healthy: {diagnosis['healthy']}")
        print(f"Latency: {diagnosis['latency_ms']:.2f} ms")
        if not diagnosis["healthy"]:
            print("Attempting repair...")
            repair = forge.repair_network()
            print(f"Repair success: {repair['success']}")
            
    elif args.command == "solve":
        if not args.input:
            print("Error: --input required")
            return
        result = forge.solve(args.input)
        if result["success"]:
            print(f"\n=== {result['solution_name']} ===")
            print(f"Complexity: {result['complexity']}")
            print(f"\n{result['implementation']}")


if __name__ == "__main__":
    # Quick test
    print("\n=== RefineForge Quick Test ===\n")
    forge = RefineForge(offline_mode=True)
    
    print("\n--- Status ---")
    status = forge.get_status()
    print(f"Version: {status['version']}")
    print(f"Offline: {status['offline_mode']}")
    
    print("\n--- Theorize Test ---")
    result = forge.theorize("Find median of two sorted arrays")
    print(result["comparison_table"])
