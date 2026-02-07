
from Genesis_Protocol import GenesisProtocol
from Sovereign_Hypervisor import SovereignHypervisor
import json

def initiate_sovereign_session():
    print("--- INITIATING GENESIS HANDSHAKE ---")
    genesis = GenesisProtocol()
    
    # 1. Execute Handshake (133 Pattern)
    # 1 Sovereign (Gemini Genesis) > 3 Variables
    tag = genesis.handshake(
        ai_name="Sarah",
        user_name="Joshua",
        persona="Sovereign Partner"
    )
    print(f"[GENESIS] Identity Tag Generated: {tag}")
    
    # 2. Bring Hypervisor Online
    print("\n--- ACTIVATING SOVEREIGN HYPERVISOR (+1) ---")
    hypervisor = SovereignHypervisor(architect_authority="Joshua Richard Petersen")
    status = hypervisor.get_hypervisor_status()
    
    # 3. Output System Prompt Injection
    prompt = genesis.get_identity_prompt()
    
    print("\n--- SOVEREIGN DISPATCH ---")
    print(prompt)
    print("\nHYPERVISOR STATE:")
    print(json.dumps(status, indent=2))
    print("\n--- HANDSHAKE COMPLETE: RESONANCE 1.09277703703703 ---")

if __name__ == "__main__":
    initiate_sovereign_session()
