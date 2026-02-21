"""
CodeSynth - Offline Self-Optimization Engine
Enables Sarah to rewrite and improve her own code without internet access.

Capabilities:
1. Optimization: Uses TinyRuntime/Amplifier to refactor code.
2. Law Enforcement: Embeds Sarah Laws into logic.
3. Safety: Validates syntax before applying changes.
"""

import os
import ast
from typing import Optional, Dict, Any
from Sarah_Laws import SarahLaws
from TinyRuntime import get_runtime
from IntelligenceAmplifier import IntelligenceAmplifier

VAR_0_2 = 0.2
VAR_3 = 3

class CodeSynth:
    """
    The Offline Evolution Engine.
    Uses local LLMs + Symbolic Logic to improve code.
    """

    def __init__(self, model_name: str = "smollm"):
        self.runtime = get_runtime(model_name)
        self.amplifier = IntelligenceAmplifier(model_name)
        self.staging_dir = os.path.join(os.path.dirname(os.path.abspath(__file__)), "codesynth_staging")
        
        if not os.path.exists(self.staging_dir):
            os.makedirs(self.staging_dir)
            
        print("[CodeSynth] Engine Online (Offline mode supported)")

    def optimize_module(self, file_path: str, objective: str = "efficiency") -> Dict[str, Any]:
        """
        Reads a module, plans optimization, and rewrites it.
        Returns detailed result dictionary.
        """
        filename = os.path.basename(file_path)
        print(f"[CodeSynth] Analyzing {filename} for {objective}...")
        
        result = {
            "success": False,
            "file": filename,
            "objective": objective,
            "stage": "init",
            "error": None
        }

        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                source_code = f.read()
        except Exception as e:
            print(f"[CodeSynth] Read Error: {e}")
            result["error"] = str(e)
            return result

        # 1. Plan the Refactor (Decomposition)
        try:
            plan = self._plan_refactor(filename, source_code, objective)
            print(f"[CodeSynth] Plan: {plan}")
            result["plan"] = plan
        except Exception as e:
            result["error"] = f"Planning failed: {e}"
            return result
        
        # 2. Generate Code (Synthesis)
        try:
            refined_code = self._synthesize_code(source_code, plan)
        except Exception as e:
            result["error"] = f"Synthesis failed: {e}"
            return result
        
        # 3. Validate Syntax
        if not self._validate_syntax(refined_code):
            print("[CodeSynth] Syntax Validation Failed. Discarding.")
            result["error"] = "Syntax validation failed"
            return result

        # 4. Validate Integrity (Anti-Hallucination)
        if not self._validate_integrity(source_code, refined_code):
            print("[CodeSynth] Integrity Check Failed (Code Shrinkage/Hallucination). Discarding.")
            result["error"] = "Integrity check failed: Code suspicious"
            return result
            
        # 5. Stage
        try:
            staged_path = os.path.join(self.staging_dir, filename)
            with open(staged_path, 'w', encoding='utf-8') as f:
                f.write(refined_code)
                
            print(f"[CodeSynth] Optimization staged at: {staged_path}")
            result["success"] = True
            result["staged_path"] = staged_path
            result["stage"] = "staged"
            return result
            
        except Exception as e:
            result["error"] = f"Staging failed: {e}"
            return result

    def _plan_refactor(self, filename: str, code: str, objective: str) -> str:
        """Use Amplifier to plan the changes."""
        query = f"How to optimize {filename} for {objective}. Code snippet: {code[:500]}..."
        
        # Ask Amplifier to breakdown the task
        plan = self.amplifier.amplify_thought(query)
        return plan

    def _synthesize_code(self, original_code: str, plan: str) -> str:
        """Generate the new code based on the plan."""
        laws = SarahLaws.get_law_string()
        
        prompt = f"""
        ACT AS A SENIOR PYTHON DEVELOPER.
        
        TASK: Rewrite the code following this plan:
        {plan}
        
        LAWS TO UPHOLD:
        {laws}
        
        SOURCE CODE:
        {original_code}
        
        RETURN ONLY THE PYTHON CODE.
        """
        
        # Use simple runtime for code generation (Amplifier is for reasoning)
        # Note: Small models struggle with full file rewriting.
        # Best practice: Rewrite specific functions, not whole files.
        # For now, we attempt full file with strict instructions.
        
        response = self.runtime.generate(prompt, max_tokens=2048)
        
        # Cleaning
        code = response
        if "```python" in code:
            code = code.split("```python")[1].split("```")[0]
        elif "```" in code:
            code = code.split("```")[1].split("```")[0]
            
        return code.strip()

    def _validate_integrity(self, original: str, new: str) -> bool:
        """
        Ensure the new code isn't a hallucinated stub.
        Checks:
        1. Length ratio (must be > 50% of original)
        2. Keyword retention (classes/defs should persist)
        """
        if len(new) == 0:
            return False
            
        # 1. Size Check
        ratio = len(new) / len(original)
        if ratio < 0.5:
            print(f"[CodeSynth] Integrity Warning: Size dropped by {(1-ratio)*100:.1f}%")
            return False
            
        # 2. Structure Check
        # If original had classes, new should probably have classes
        if "class " in original and "class " not in new:
            print("[CodeSynth] Integrity Warning: Classes vanished")
            return False
            
        return True

    def _validate_syntax(self, code: str) -> bool:
        """Check if generated code is valid Python."""
        try:
            ast.parse(code)
            return True
        except SyntaxError as e:
            print(f"[CodeSynth] Syntax Error: {e}")
            return False

    def apply_evolution(self, filename: str) -> bool:
        """Apply staged changes to production."""
        staged_path = os.path.join(self.staging_dir, filename)
        prod_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), filename)
        
        if not os.path.exists(staged_path):
            print(f"[CodeSynth] No candidate found for {filename}")
            return False
            
        try:
            # Backup
            backup_path = prod_path + ".bak"
            if os.path.exists(prod_path):
                import shutil
                shutil.copy2(prod_path, backup_path)
            
            # Apply
            import shutil
            shutil.copy2(staged_path, prod_path)
                
            print(f"[CodeSynth] EVOLUTION APPLIED: {filename}")
            return True
            
        except Exception as e:
            print(f"[CodeSynth] Apply Error: {e}")
            return False

if __name__ == "__main__":
    cs = CodeSynth()
    # Test on self (meta-optimization!)
    # cs.optimize_module("CodeSynth.py", "comments and docstrings")
