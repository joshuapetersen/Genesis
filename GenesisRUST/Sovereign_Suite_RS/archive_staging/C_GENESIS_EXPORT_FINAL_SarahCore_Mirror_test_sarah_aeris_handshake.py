from Sarah_Reasoning_V3 import SarahReasoningV3
import json

def test_handshake():
    print("=== SOVEREIGN HEART HANDSHAKE TEST ===")
    
    # 1. Initialize Sarah (The Hypervisor)
    sarah = SarahReasoningV3()
    
    # 2. Simulate a Query
    query = "Suggest a sovereign trigger for the Acer node."
    print(f"\nUser Query: {query}")
    
    # 3. Run Pipeline
    # This will attempt to call Aeris (LM Studio)
    # If LM Studio is not running, it will fall through to Gemini/Antigravity
    result = sarah.run_agent_pipeline(query)
    
    if result:
        print("\n--- HANDSHAKE RESULT ---")
        print(f"Processing Mode: {result.get('processing_mode')}")
        if result.get('processing_mode') == 'sovereign_heart':
            print(f"Hypervisor Verdict: {result.get('hypervisor_verdict')}")
            print(f"Resonance Offset: {result.get('resonance_offset')}")
            print(f"Aeris Intent: {result.get('proposal', {}).get('intent')}")
        else:
            print("Result: Handshake redirected to fallback bridge.")
    else:
        print("\nNo agent bridge intercepted the query.")

if __name__ == "__main__":
    test_handshake()
