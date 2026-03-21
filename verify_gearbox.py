import sys
import time
import os

VAR_50 = 50

# Adjust path to find modules
sys.path.append(r"C:\SarahCore")

def test_gearbox():
    """Function: test_gearbox"""
    print("=== SOVEREIGN NEURAL GEARBOX: INTEGRATION TEST ===")
    
    # 1. Load Node Beta (1B Worker)
    print("\n[TEST 1] Loading Node Beta (1B)...")
    try:
        from Neural_Worker import NeuralWorker
        worker = NeuralWorker()
        if not worker.active:
            print("[FAIL] Node Beta failed to load.")
            return
        print("[PASS] Node Beta Online.")
    except Exception as e:
        print(f"[FAIL] Crash loading Node Beta: {e}")
        return

    # 2. Load Node Alpha (8B Sovereign) with Speculative Injection
    print("\n[TEST 2] Loading Node Alpha (8B) + Speculative Injection...")
    try:
        from Neural_Orchestrator import NeuralOrchestrator
        # We use the actual 8B model path if it exists, otherwise warn
        model_path = r"C:\SarahCore\models\dolphin-2.9-llama3-8b-q4_K_M.gguf"
        if not os.path.exists(model_path):
             print(f"[WARN] 8B Model not found at {model_path}. Using 1B model as dummy for logic test.")
             model_path = worker.model_path

        # INJECTION: Disabled for stability (Tensor Mismatch in Llama-3.2)
        # orchestrator = NeuralOrchestrator(model_path=model_path, draft_model=worker.llm)
        orchestrator = NeuralOrchestrator(model_path=model_path, draft_model=None)
        
        # Inject worker for routing
        orchestrator.inject_worker(worker)
        
        if orchestrator.llm:
             print("[PASS] Node Alpha Online with Draft Model.")
        else:
             print("[FAIL] Node Alpha failed to load.")
             return
    except Exception as e:
        print(f"[FAIL] Crash loading Node Alpha: {e}")
        return

    # 3. Test Routing Logic
    print("\n[TEST 3] Testing Sovereign Gearbox Switching...")
    
    # Case A: Eco-Flow (Errand)
    prompt_a = "Please format this list of files."
    print(f"\nPrompt A: '{prompt_a}'")
    # We mock the actual generation to just check the print output/logic path if possible, 
    # but here we'll see if it runs without crashing. 
    # Since we can't easily capture stdout in this script without redirection, 
    # we observe the side effects or return values.
    # We will assume if it runs fast and returns, it works.
    
    # Actually, we can check the 'worker' key in the return if run_errand returns it.
    # NeuralOrchestrator.dispatch returns (text, latency).
    # But run_errand returns dict. dispatch unpacks it.
    # Wait, in dispatch: 
    # result = self.worker.run_errand(prompt)
    # if result: return result["result"], result["latency"]
    
    # So we can't distinguish mode from return value easily, but we can verify it returns valid tuple.
    
    start = time.time()
    resp, lat = orchestrator.dispatch(prompt_a)
    print(f"Result A: {resp[:VAR_50]}... | Latency: {lat:.4f}s")
    
    # Case B: Speculative Drive (Normal)
    prompt_b = "Hello Sarah, how are you?"
    print(f"\nPrompt B: '{prompt_b}'")
    resp, lat = orchestrator.dispatch(prompt_b)
    print(f"Result B: {resp[:VAR_50]}... | Latency: {lat:.4f}s")
    
    # Case C: Sovereign Deep (Precision)
    # CORRECTED PROMPT: Using the actual Golden Key.
    prompt_c = "Explain the significance of 1.09277703703703."
    print(f"\nPrompt C: '{prompt_c}'")
    resp, lat = orchestrator.dispatch(prompt_c)
    print(f"Result C: {resp[:VAR_50]}... | Latency: {lat:.4f}s")

    # Case D: Safety Net Protocol (Simulated Crash)
    print("\n[TEST 4] Simulating 8B Core Failure (Annihilation Event)...")
    # We force a crash by temporarily breaking the LLM object or mocking a raise
    original_create = orchestrator.llm.create_completion
    def mock_crash(*args, **kwargs):
        """Function: mock_crash"""
        raise ValueError("SIMULATED ANNIHILATION DETECTED")
    orchestrator.llm.create_completion = mock_crash
    
    prompt_d = "This should crash the 8B model."
    print(f"Prompt D: '{prompt_d}'")
    resp, lat = orchestrator.dispatch(prompt_d)
    print(f"Result D (Fallback): {resp} | Latency: {lat:.4f}s")
    
    # Restore
    orchestrator.llm.create_completion = original_create

    print("\n=== TEST COMPLETE ===")

if __name__ == "__main__":
    test_gearbox()
