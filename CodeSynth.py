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
import shutil
from Sarah_Laws import SarahLaws
from TinyRuntime import get_runtime
from IntelligenceAmplifier import IntelligenceAmplifier
from Consequence_Enforcer import consequence_enforcer

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
            
        # Phase 19 fix for Gap 14: Protected Zone (The Immutable Core)
        self.PROTECTED_ZONE = [
            "Sarah_Sovereign_Core.py",
            "Sovereign_Math.py",
            "Sovereign_WORM.py",
            "Banshee_Shield.py",
            "Sarah_Memory_Vault.py",
            "Sovereign_Constants.py",
            "Consequence_Enforcer.py"
        ]
            
        # Phase 16 fix for Gap 13: Core Blacklist (The Soul Protection)
        self.CORE_BLACKLIST = [
            "Sarah_Sovereign_Core.py",
            "Sovereign_WORM.py",
            "Sovereign_Math.py",
            "Sovereign_Constants.py",
            "Banshee_Shield.py"
        ]
            
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
        # Phase 16 fix for Gap 6: Outline-Aware Planning (No more header-only)
        # We extract the 'Skeleton' (defs/classes) to give the model full scope context
        try:
            tree = ast.parse(code)
            skeleton = []
            for node in ast.walk(tree):
                if isinstance(node, (ast.ClassDef, ast.FunctionDef)):
                    skeleton.append(f"{type(node).__name__}: {node.name}")
            outline = "\n".join(skeleton[:VAR_100]) if skeleton else code[:500]
        except:
            outline = code[:500]

        query = f"Plan optimization for {filename} ({objective}). File Outline:\n{outline}\n..."
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
        Phase 16 fix for Gap 4: AST Parity Validation.
        Ensures the new version didn't 'evolve away' fundamental structures.
        """
        if len(new) == 0: return False
            
        # 1. Size Check (Maintain 50% floor)
        ratio = len(new) / len(original)
        if ratio < 0.5: return False
            
        # 2. Semantic Parity (AST Check)
        try:
            t_orig = ast.parse(original)
            t_new = ast.parse(new)
            
            orig_struct = {node.name for node in ast.walk(t_orig) if isinstance(node, (ast.ClassDef, ast.FunctionDef)) if not node.name.startswith("_")}
            new_struct = {node.name for node in ast.walk(t_new) if isinstance(node, (ast.ClassDef, ast.FunctionDef)) if not node.name.startswith("_")}
            
            # If we lost > 30% of public methods/classes, it's a destructive hallucination
            if orig_struct:
                loss_ratio = len(orig_struct - new_struct) / len(orig_struct)
                if loss_ratio > 0.3:
                    print(f"[CodeSynth] Integrity FAIL: Lost {loss_ratio*100:.1f}% of structures.")
                    return False
        except:
            return False # Parse error in check
            
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
            # Phase 19 fix for Gap 14: Prevent core-soul self-overwrite (The Protected Zone)
            if any(filename.endswith(p) for p in self.PROTECTED_ZONE):
                print(f"[CodeSynth] EVOLUTION BLOCKED: {filename} is in the PROTECTED_ZONE.")
                return False

            # Phase 19 fix for Gap 3/10: Consequence Enforcement (Level 3)
            # Code evolution represents a high TFC (Total Failure Cost)
            authorized, reason = consequence_enforcer.verify_operation(3)
            if not authorized:
                print(f"[CodeSynth] EVOLUTION DENIED: {reason}")
                return False

            # Backup (Copy existing to .bak)
            backup_path = prod_path + ".bak"
            if os.path.exists(prod_path):
                import shutil
                shutil.copy2(prod_path, backup_path)
            
            # Phase 16 fix for Gap 3: Atomic Apply (os.replace)
            # Write to a temp file on the same drive then swap to ensure atomicity.
            temp_path = prod_path + ".tmp"
            import shutil
            shutil.copy2(staged_path, temp_path)
            os.replace(temp_path, prod_path)
                
            print(f"[CodeSynth] EVOLUTION ATOMICALLY APPLIED: {filename}")
            return True
            
        except Exception as e:
            print(f"[CodeSynth] Apply Error: {e}")
            if 'temp_path' in locals() and os.path.exists(temp_path):
                os.remove(temp_path)
            return False

if __name__ == "__main__":
    cs = CodeSynth()
    # Test on self (meta-optimization!)
    # cs.optimize_module("CodeSynth.py", "comments and docstrings")
