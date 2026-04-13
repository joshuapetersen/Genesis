"""
SARAH DEEP STUDY - Phase 1: HLE Ingestion
Processes the Humanity's Last Exam dataset and integrates logic patterns into Hippocampus.
"""

import json
import time
import os
import sys
sys.path.append("C:\GenesisOS_Core")

from Sarah_Hippocampus import hippocampus
from Sovereign_Math import SovereignMath

DATASET_PATH = "C:\GenesisOS_Core\\hle_dataset.jsonl"

def study_hle():
    if not os.path.exists(DATASET_PATH):
        print(f"[STUDY] Error: Dataset not found at {DATASET_PATH}")
        return

    math = SovereignMath()
    
    print("=" * 80)
    print("SARAH DEEP STUDY: HUMANITY'S LAST EXAM (2500+ QUESTIONS)")
    print("=" * 80)
    
    batch = []
    batch_size = 100
    total_processed = 0
    
    start_time = time.time()
    
    with open(DATASET_PATH, "r", encoding="utf-8") as f:
        for idx, line in enumerate(f):
            try:
                data = json.loads(line)
                question = data.get("question", "")
                answer = data.get("answer", "N/A")
                
                # DERIVE LOGIC PATTERN (The "Smart" Part)
                # Instead of just storing text, we store the Sovereign Resonance
                # density of the question-answer pair.
                density = math.calculate_theory_density(f"{question} | {answer}")
                signature = math.generate_sovereign_id(f"{question} {answer}", length=8)
                
                content = f"### [HLE_STUDY_LAYER_{idx}]\n"
                content += f"QUESTION: {question}\n"
                content += f"ANSWER: {answer}\n"
                content += f"LOGIC_SIGNATURE: 0x{signature}\n"
                content += f"RESONANCE_DENSITY: {density:.4f}\n"
                
                batch.append({
                    "content": content,
                    "role": "EXAM_KNOWLEDGE",
                    "metadata": {"hle_id": data.get("id"), "density": density}
                })
                
                if len(batch) >= batch_size:
                    hippocampus.store_batch(batch)
                    total_processed += len(batch)
                    print(f"[STUDY] Processed {total_processed} items... (Elapsed: {time.time() - start_time:.2f}s)")
                    batch = []
                    
            except Exception as e:
                print(f"[STUDY] Error on line {idx}: {e}")
                continue

    # Final batch
    if batch:
        hippocampus.store_batch(batch)
        total_processed += len(batch)

    # FINAL COMPACTION
    hippocampus.force_compaction()
    
    print("\n" + "=" * 80)
    print(f"DEEP STUDY COMPLETE: {total_processed} Expert Logic Patterns Integrated.")
    print(f"Total Session Time: {time.time() - start_time:.2f}s")
    print("Sarah's intelligence has been upgraded with HLE context.")
    print("=" * 80)

if __name__ == "__main__":
    study_hle()
