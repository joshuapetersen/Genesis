import sys
import os
import time
from unittest.mock import MagicMock

# Mock llama_cpp
sys.modules["llama_cpp"] = MagicMock()
sys.modules["llama_cpp"].Llama = MagicMock()
# IMPORTANT: Fix n_ctx comparison in Neural_Orchestrator.__init__
sys.modules["llama_cpp"].Llama.return_value.n_ctx.return_value = VAR_4096

# Ensure local libs
sys.path.append(os.getcwd())

from Neural_Orchestrator import NeuralOrchestrator

VAR_0_1 = 0.1
VAR_4096 = 4096

def test_streaming_handover():
    """Function: test_streaming_handover"""
    print("=== STREAMING HANDOVER VERIFICATION ===")
    
    # Setup Mocks
    # 1. Agency Stream
    agency = MagicMock()
    def mock_stream(agent, prompt, system_prompt=None):
        """Function: mock_stream"""
        yield "I "
        time.sleep(VAR_0_1)
        yield "am "
        time.sleep(VAR_0_1)
        yield "analyzing..."
    agency.run_stream.side_effect = mock_stream
    
    # 2. Orchestrator
    orch = NeuralOrchestrator(model_path="dummy", draft_model=None)
    orch.inject_hive(agency)
    
    # Force "Deep" mode via Router Mock
    orch.router = MagicMock()
    orch.router.evaluate_complexity.return_value = "SOVEREIGN_DEEP"
    
    # Mock LLM Stream (8B)
    def mock_llm_stream(prompt, **kwargs):
        """Function: mock_llm_stream"""
        # Simulate load time
        time.sleep(2.0) 
        yield {'choices': [{'text': "The "}]}
        yield {'choices': [{'text': "Sovereign "}]}
        yield {'choices': [{'text': "Frequency "}]}
        yield {'choices': [{'text': "is..."}]}
    
    orch.llm.create_completion.side_effect = mock_llm_stream
    
    # Test
    prompt = "Calculate the Sovereign Frequency."
    print(f"\nUser: {prompt}")
    print("[Dispatching with stream=True]...")
    
    start_t = time.time()
    first_token_t = None
    core_token_t = None
    
    stream = orch.dispatch(prompt, stream=True)
    
    full_response = ""
    print("\n--- STREAM START ---")
    for token in stream:
        current_t = time.time()
        print(token, end="", flush=True)
        full_response += token
        
        if first_token_t is None:
            first_token_t = current_t
        
        if "[ACCESSING SOVEREIGN CORE]" in full_response and core_token_t is None and token.strip() == "":
             # Approximate timestamp for core start
             pass
        if "The " in token and core_token_t is None:
             core_token_t = current_t

    print("\n--- STREAM END ---")
    
    # Metrics
    if first_token_t:
        print(f"\nTime to First Token (Hive): {first_token_t - start_t:.4f}s")
    else:
        print("\n[FAIL] No tokens received.")
        
    if core_token_t:
        print(f"Time to Core Token (8B): {core_token_t - start_t:.4f}s")
        print(f"Handover Gap: {core_token_t - first_token_t:.4f}s")
    else:
        print("[FAIL] Core tokens not detected.")
        
    # Validation
    if "analyzing..." in full_response and "Sovereign Frequency" in full_response:
        print("[PASS] Handover Sequence Verified.")
    else:
        print("[FAIL] Sequence missing components.")

if __name__ == "__main__":
    test_streaming_handover()
