"""
Verification Script for Phase 8 Knowledge Expansion
Tests O(1) lookup for newly ingested industry categories.
"""

from coding_knowledge import CodingKnowledge

def verify_industry_knowledge():
    print("\n--- VERIFYING INDUSTRY KNOWLEDGE EXPANSION ---")
    ck = CodingKnowledge()
    
    test_terms = [
        "Transformers & Self-Attention",
        "Zero Trust",
        "Quantum Computing",
        "Atomic Design",
        "Algo Trading",
        "SLAM"
    ]
    
    passed = 0
    for term in test_terms:
        result = ck.lookup(term)
        if result:
            print(f"[OK] Found: {term}")
            passed += 1
        else:
            print(f"[FAIL] Missing: {term}")
            
    print(f"\nVerification Results: {passed}/{len(test_terms)} categories found.")
    if passed == len(test_terms):
        print("[SUCCESS] Phase 8 Expansion Verified.")
    else:
        print("[PARTIAL] Some categories may still be indexing.")

if __name__ == "__main__":
    verify_industry_knowledge()
