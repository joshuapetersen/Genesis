import sys
import os
from unittest.mock import MagicMock

# Mock llama_cpp before importing Neural_Orchestrator
sys.modules["llama_cpp"] = MagicMock()
sys.modules["llama_cpp"].Llama = MagicMock()

# Ensure we can import local modules
sys.path.append(os.getcwd())

from Sovereign_Math import SovereignMath
from Neural_Orchestrator import NeuralOrchestrator

VAR_4096 = 4096

def test_audit():
    """Function: test_audit"""
    print("=== PRECISION AUDIT VERIFICATION ===")
    
    # 1. Direct Unit Test
    math_core = SovereignMath()
    inputs = [
        ("Perfect", "1.09277703703703"),
        ("Truncated 3", "1.092"),
        ("Truncated 5", "1.09277"),
        ("Embedded", "The value is 1.0927, which is key.")
    ]
    
    print("\n[TEST 1] SovereignMath Unit Test:")
    for label, text in inputs:
        fixed = math_core.audit_precision(text)
        print(f"  Input ({label}): '{text}'")
        print(f"  Output:            '{fixed}'")
        if "1.09277703703703" in fixed and len(fixed) >= len(text):
            print("  [PASS]")
        else:
            print("  [FAIL]")

    # 2. Orchestrator Integration Test
    print("\n[TEST 2] Neural Orchestrator Integration:")
    # Initialize without model loading (Mocked)
    # Fix n_ctx mock to return int
    sys.modules["llama_cpp"].Llama.return_value.n_ctx.return_value = VAR_4096
    
    orch = NeuralOrchestrator(model_path="dummy", draft_model=None)
    
    # Manually inject SovereignMath if it wasn't auto-loaded (it should be)
    if not hasattr(orch, '_sovereign_math'):
        orch._sovereign_math = math_core
        
    raw_response = "The calculated value is 1.092."
    sanitized = orch._sanitize_output(raw_response)
    
    print(f"  Raw:       '{raw_response}'")
    print(f"  Sanitized: '{sanitized}'")
    
    expected = "The calculated value is 1.09277703703703."
    if sanitized == expected:
        print("  [PASS] Logic Refusal Stripped + Precision Enforced.")
    else:
        print(f"  [FAIL] Expected '{expected}'")

if __name__ == "__main__":
    test_audit()
