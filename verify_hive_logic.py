import sys
import os
from unittest.mock import MagicMock

# Mock llama_cpp
sys.modules["llama_cpp"] = MagicMock()
sys.modules["llama_cpp"].Llama = MagicMock()

# Ensure we can import local modules
sys.path.append(os.getcwd())

from Hive_Router import HiveRouter
from Disposable_Agency import DisposableAgency
from Neural_Orchestrator import NeuralOrchestrator

VAR_0_1 = 0.1
VAR_4096 = 4096

def test_hive():
    """Function: test_hive"""
    print("=== SOVEREIGN HIVE VERIFICATION ===")

    # 1. Test Router Logic
    print("\n[TEST 1] Hive Router Logic:")
    router = HiveRouter()
    test_cases = [
        ("Clean up this regex pattern", "smollm"),
        ("Format this JSON list", "smollm"),
        ("Summarize the history of AI", "qwen"),
        ("Explain the logic behind this", "qwen"),
        ("Just a random thought", "qwen") # Fallback
    ]
    
    passed = 0
    for prompt, expected in test_cases:
        agent = router.select_agent(prompt)
        print(f"  Current: '{prompt}' -> {agent}")
        if agent == expected:
            passed += 1
        else:
            print(f"  [FAIL] Expected {expected}, got {agent}")
            
    if passed == len(test_cases):
        print("  [PASS] Router Logic Correct.")
    else:
        print("  [FAIL] Router Logic Errors.")

    # 2. Test Agency Configuration
    print("\n[TEST 2] Agency Model Availability:")
    agency = DisposableAgency()
    
    # Mock os.path.exists to be true only if we want to fake it, 
    # but strictly we want to see if they are actually there on disk.
    # We won't mock os.path.exists here, we want REAL verification of files.
    
    for name, config in agency.AGENTS.items():
        exists = os.path.exists(config['path'])
        print(f"  Agent '{name}': {config['path']} -> {'FOUND' if exists else 'MISSING'}")

    # 3. Test Integration
    print("\n[TEST 3] Orchestrator Integration:")
    
    # Mock Llama for Orchestrator
    sys.modules["llama_cpp"].Llama.return_value.n_ctx.return_value = VAR_4096
    orch = NeuralOrchestrator(model_path="dummy", draft_model=None)
    
    # Inject Agency
    orch.inject_hive(agency)
    
    # Force Router to return ECO_FLOW
    orch.router = MagicMock()
    orch.router.evaluate_complexity.return_value = "ECO_FLOW"
    
    # Mock Agency.run_mission
    agency.run_mission = MagicMock(return_value={"result": "HIVE_OUTPUT", "latency": VAR_0_1})
    
    # Prompt triggering SmolLM via HiveRouter
    prompt = "Format this JSON for me."
    
    response, latency = orch.dispatch(prompt)
    
    print(f"  Prompt: '{prompt}'")
    print(f"  Router Mode: ECO_FLOW")
    print(f"  Response: {response}")
    
    # Verify agency was called
    if agency.run_mission.called:
        args = agency.run_mission.call_args
        print(f"  Agency Called With: {args}")
        if args[0][0] == "smollm": 
             print("  [PASS] Correctly routed to SmolLM agent.")
        else:
             print(f"  [FAIL] Routed to {args[0][0]}, expected smollm.")
    else:
        print("  [FAIL] Agency not called.")

if __name__ == "__main__":
    test_hive()
