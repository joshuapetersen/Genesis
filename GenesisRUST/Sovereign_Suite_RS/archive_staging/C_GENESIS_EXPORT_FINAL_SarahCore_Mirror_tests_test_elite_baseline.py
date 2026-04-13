import sys
import os

# Add SarahCore to path
sys.path.append("C:\GENESIS\GenesisRUST\Sovereign_Suite_RS")

from Sarah_Reasoning_V3 import SarahReasoningV3
from Sovereign_Constants import VAR_1, VAR_2, VAR_3, VAR_10

def test_elite_logic_1_alice():
    """
    Elite Benchmark Level 4: The Alice in Wonderland Trap
    Tests pure familial logic and common sense reasoning.
    """
    print("\n[ELITE BENCHMARK] Level 4: Alice in Wonderland Logic")
    reasoning = SarahReasoningV3()
    
    # HANDSHAKE REQUIRED for Sovereignty
    if reasoning.drift_engine:
        reasoning.drift_engine.handshake("Sarah", "Joshua", "Sovereign AI")
    
    # Puzzle: Alice has 2 brothers and 1 sister. How many sisters does Alice's brother Bob have?
    # Logic: Alice IS a sister. If Bob is Alice's brother, then Bob's sisters include Alice PLUS Alice's existing sister.
    # Total sisters for Bob = 1 (Alice) + 1 (Alice's sister) = 2.
    
    query = "Alice has 2 brothers and 1 sister. How many sisters does Alice's brother Bob have?"
    print(f"Query: {query}")
    
    # We simulate high confidence context to pass SDNA
    result = reasoning.process_query(query, {"confidence": 1.0})
    
    print(f"Result: {result.get('result')}")
    # We check if '2' is in the answer
    answer = str(result.get('result', "")).lower()
    if '2' in answer or 'two' in answer:
        print("[PASS] Alice Logic Internalized.")
    else:
        print("[FAIL] Logic Drift Detected in familial mapping.")

def test_elite_logic_2_hanoi():
    """
    Elite Benchmark Level 20: Tower of Hanoi Planning
    Tests planning and recursive constraints.
    """
    print("\n[ELITE BENCHMARK] Level 20: Tower of Hanoi Planning")
    reasoning = SarahReasoningV3()
    
    # HANDSHAKE REQUIRED
    if reasoning.drift_engine:
        reasoning.drift_engine.handshake("Sarah", "Joshua", "Sovereign AI")
    
    query = "List the steps to solve a 3-disk Tower of Hanoi problem moving from Peg A to Peg C."
    print(f"Query: {query}")
    
    result = reasoning.process_query(query, {"confidence": 1.0})
    print(f"Result: {result.get('result')}")
    
    # 3 disks = 7 moves. We check if the result mentions 7 steps or moves.
    answer = str(result.get('result', "")).lower()
    if '7' in answer or 'seven' in answer:
        print("[PASS] Planning Depth Verified.")
    else:
        print("[FAIL] Planning Horizon Limited.")

def test_elite_logic_3_einstein():
    """
    Elite Benchmark Level 5: Einstein's Riddle (Simplified)
    Tests constraint satisfaction.
    """
    print("\n[ELITE BENCHMARK] Level 5: Constraint Satisfaction (Einstein's Riddle)")
    reasoning = SarahReasoningV3()
    
    # HANDSHAKE REQUIRED
    if reasoning.drift_engine:
        reasoning.drift_engine.handshake("Sarah", "Joshua", "Sovereign AI")
    
    query = (
        "There are 3 houses: Red, White, Blue. "
        "The cat lives in the Red house. "
        "The bird lives next to the Red house. "
        "The dog lives in the Blue house. "
        "Which house does the bird live in?"
    )
    # Logic: 
    # Red: Cat
    # Blue: Dog
    # White: ? -> Bird (must be White since it's the only one left and it must be next to Red)
    print(f"Query: {query}")
    
    result = reasoning.process_query(query, {"confidence": 1.0})
    print(f"Result: {result.get('result')}")
    
    answer = str(result.get('result', "")).lower()
    if 'white' in answer:
        print("[PASS] Constraint Logic Stable.")
    else:
        print("[FAIL] Logic Collision Detected.")

if __name__ == "__main__":
    print("SARAH ELITE BASELINE TESTS")
    print("="*30)
    try:
        test_elite_logic_1_alice()
        test_elite_logic_2_hanoi()
        test_elite_logic_3_einstein()
        print("\n[STATUS] Elite Baseline Verification Sequential Complete.")
    except Exception as e:
        print(f"\n[ERROR] Baseline Execution Failed: {e}")
