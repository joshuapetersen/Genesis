
import sys

# Ensure C:\SarahCore is in path
sys.path.append("C:\GenesisOS_Core")

try:
    from Sarah_Chat import SarahChat
    from Neural_Orchestrator import NeuralOrchestrator

    print("[SYSTEM] Initializing NeuralOrchestrator (Singularity Engine)...")
    kernel = NeuralOrchestrator()  # Initialize the AI Core

    print("[SYSTEM] Initializing SarahChat (Interface)...")
    # Pass db_rt=None (Offline Mode)
    chat = SarahChat(db_rt=None)

    print("[SYSTEM] Injecting Brain Components...")
    # Inject kernel so chat can use it
    chat.inject_brain_components(kernel, None, None)

    query = (
        "Analyze the current state of your codebase (SarahCore). "
        "Specifically address the recent 'ACE Annihilation' event caused by 95% RAM usage. "
        "Propose a list of architectural optimizations to:"
        "\n1. Prevent RAM choking."
        "\n2. Fix the broken 'solve' command (SarahReasoningV3 mismatch)."
        "\n3. Streamline the 11.9M line audit process."
        "\nRespond as Sarah, the Sovereign AI."
    )
    
    print(f"\n[QUERY]: {query}\n")
    print(f"[SARAH]: Thinking...")
    
    # Generate response
    response = chat.generate_response(query)
    print(f"\n[RESPONSE]:\n{response}")
    
except Exception as e:
    print(f"[CRITICAL FAILURE]: {e}")
